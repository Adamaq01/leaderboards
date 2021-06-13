use actix_http::http::header::ContentType;
use actix_web::{get, web, HttpResponse, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::MssqlPool;

use crate::{models::Leaderboard, AuthState};

#[get("")]
async fn find_all(db_pool: web::Data<MssqlPool>) -> impl Responder {
    let leaderboards: Vec<Leaderboard> = sqlx::query_as("SELECT * FROM leaderboards ORDER BY id")
        .fetch_all(db_pool.as_ref())
        .await
        .unwrap();
    HttpResponse::Ok()
        .set(ContentType::json())
        .body(serde_json::to_string_pretty(&leaderboards).unwrap())
}

pub fn init(cfg: &mut web::ServiceConfig, auth_state: web::Data<AuthState>) {
    cfg.app_data(auth_state.clone()).service(
        web::scope("")
            .wrap(HttpAuthentication::bearer(crate::auth_middleware))
            .configure(|cfg| {
                cfg.service(find_all);
            }),
    );
}
