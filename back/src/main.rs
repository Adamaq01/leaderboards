use std::{collections::HashSet, env, sync::RwLock};

use actix_cors::Cors;
use actix_http::{error::InternalError, http::header::ContentType};
use actix_web::{
    dev::{Service, ServiceRequest},
    http::{header, HeaderValue},
    middleware::Logger,
    web::Data,
    App, HttpResponse, HttpServer,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use anyhow::{Context, Result};
use dotenv::dotenv;
use listenfd::ListenFd;
use log::info;
use sqlx::MssqlPool;

mod models;
mod routes;

#[derive(Debug, Default)]
pub struct AuthState {
    sessions: RwLock<HashSet<String>>,
}

pub async fn auth_middleware(
    req: ServiceRequest,
    b: BearerAuth,
) -> Result<ServiceRequest, actix_http::Error> {
    if req
        .app_data::<Data<AuthState>>()
        .unwrap()
        .sessions
        .read()
        .unwrap()
        .contains(b.token())
    {
        Ok(req)
    } else {
        let mut err = HttpResponse::Ok()
            .set(ContentType::json())
            .body("{\"success\": false}");
        err.headers_mut()
            .append(header::WWW_AUTHENTICATE, HeaderValue::from_static("Bearer"));
        let err = InternalError::from_response("Failed to authenticate user", err);
        Err(err.into())
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let database_url =
        env::var("DATABASE_URL").context("DATABASE_URL environment variable is not set")?;
    let db_pool = MssqlPool::connect(&database_url).await?;

    let auth_state = actix_web::web::Data::new(AuthState::default());

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .wrap(Logger::default())
            .data(db_pool.clone())
            .configure(|cfg| routes::init(cfg, auth_state.clone()))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").context("HOST environment variable is not set")?;
            let port = env::var("PORT").context("PORT environment variable is not set")?;
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await?;

    Ok(())
}
