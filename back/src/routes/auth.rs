use actix_http::http::header::ContentType;
use actix_web::{
    post,
    web::{self, Data, Json, Payload},
    FromRequest, HttpRequest, HttpResponse, Responder,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, MssqlPool};
use uuid::Uuid;

use crate::AuthState;

#[derive(Debug, FromRow, Serialize, Deserialize, PartialEq, Eq)]
struct RegisterInformations {
    username: String,
    display_name: String,
    password: String,
}

#[post("/register")]
async fn register(
    req: Json<RegisterInformations>,
    db: Data<MssqlPool>,
    auth_state: Data<AuthState>,
) -> impl Responder {
    let mut config = argon2::Config::default();
    config.variant = argon2::Variant::Argon2id;
    let salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();
    let hash = argon2::hash_encoded(req.password.as_bytes(), salt.as_bytes(), &config).unwrap();

    match query(&format!(
        "INSERT INTO users (username, display_name, salt, password) VALUES ('{}', '{}', '{}', '{}')",
        req.username, req.display_name, salt, hash
    ))
    .execute(db.as_ref())
    .await
    {
        Ok(_) => {
            let token = Uuid::new_v4().to_hyphenated().to_string();
            auth_state.sessions.write().unwrap().insert(token.clone());
            HttpResponse::Ok()
                .set(ContentType::json())
                .body(format!("{{\"success\": true, \"token\": \"{}\"}}", token))
        }
        Err(err) => {
            println!("{:?}", err);
            HttpResponse::Ok()
                .set(ContentType::json())
                .body("{\"success\": false}")
        }
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize, PartialEq, Eq)]
struct Credentials {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(
    req: Json<Credentials>,
    db: Data<MssqlPool>,
    auth_state: Data<AuthState>,
) -> impl Responder {
    #[derive(FromRow)]
    struct User {
        salt: String,
        password: String,
    }

    let user: User = if let Ok(user) = query_as(&format!(
        "SELECT salt, password FROM users WHERE username = '{}'",
        req.username
    ))
    .fetch_one(db.as_ref())
    .await
    {
        user
    } else {
        return HttpResponse::Ok()
            .set(ContentType::json())
            .body("{\"success\": false}");
    };

    println!("{:?}", user.salt);

    let mut config = argon2::Config::default();
    config.variant = argon2::Variant::Argon2id;
    let hash =
        argon2::hash_encoded(req.password.as_bytes(), user.salt.as_bytes(), &config).unwrap();

    if hash == user.password {
        let token = Uuid::new_v4().to_hyphenated().to_string();
        auth_state.sessions.write().unwrap().insert(token.clone());
        HttpResponse::Ok()
            .set(ContentType::json())
            .body(format!("{{\"success\": true, \"token\": \"{}\"}}", token))
    } else {
        HttpResponse::Ok()
            .set(ContentType::json())
            .body("{\"success\": false}")
    }
}

#[post("/logout")]
async fn logout(req: HttpRequest, payload: Payload, auth_state: Data<AuthState>) -> impl Responder {
    if let Ok(auth) = BearerAuth::from_request(&req, &mut payload.into_inner()).await {
        if auth_state.sessions.write().unwrap().remove(auth.token()) {
            HttpResponse::Ok()
                .set(ContentType::json())
                .body("{\"success\": true}")
        } else {
            HttpResponse::Ok()
                .set(ContentType::json())
                .body("{\"success\": false}")
        }
    } else {
        HttpResponse::Ok()
            .set(ContentType::json())
            .body("{\"success\": false}")
    }
}

pub fn init(cfg: &mut web::ServiceConfig, auth_state: web::Data<AuthState>) {
    cfg.app_data(auth_state)
        .service(register)
        .service(login)
        .service(logout);
}
