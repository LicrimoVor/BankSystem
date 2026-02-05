mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;
mod utils;

use std::{sync::Arc, time::Duration};

use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use tracing::info;

use infrastructure::{config::Config, logging::init_logging, migrate};
use presentation::middleware::{RequestIdMiddleware, TimingMiddleware};

use crate::{
    data::Database, infrastructure::state::State, presentation::middleware::csrf::CsrfMiddleware,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    init_logging();

    let cfg = Config::from_env().expect("invalid config");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&cfg.database_url)
        .await
        .expect("failed to connect to database");

    let client = reqwest::Client::builder()
        .user_agent("bank-api/1.0")
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(15))
        .build()
        .expect("Failed to create HTTP client");

    info!("Running migrations");
    migrate::run(&pool)
        .await
        .map_err(|e| format!("Migration error: {}", e.to_string()))
        .unwrap();

    let state = Arc::new(State::new());

    // let db = Database::STATE(state);
    let db = Database::PgSQL(Arc::new(pool));
    let addr = format!("{}:{}", cfg.host, cfg.port);
    info!("Listening on http://{}", addr);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cfg.cors_origin)
            .allowed_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .supports_credentials()
            .max_age(3600);

        info!("Start app");
        App::new()
            .wrap(TimingMiddleware)
            .wrap(RequestIdMiddleware)
            .wrap(Logger::default())
            .wrap(CsrfMiddleware)
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .build(),
            )
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(cfg.clone()))
            .app_data(web::Data::new(client.clone()))
            .configure(presentation::api::configure)
    })
    .bind(addr)?
    .run()
    .await
}
