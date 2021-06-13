use actix_web::{get, web, HttpResponse, Responder};

use crate::AuthState;

pub mod auth;
pub mod leaderboards;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"Welcome to Leaderboards API Index.
Available routes:
GET /users -> list of all users
GET /categories -> list of all categories
GET /leaderboards -> list of all leaderboards
GET /scores -> list of all scores
"#,
    )
}

pub fn init(cfg: &mut web::ServiceConfig, auth_state: web::Data<AuthState>) {
    cfg.service(index)
        .service(web::scope("/auth").configure(|cfg| auth::init(cfg, auth_state.clone())))
        .service(
            web::scope("/leaderboards")
                .configure(|cfg| leaderboards::init(cfg, auth_state.clone())),
        );
}
