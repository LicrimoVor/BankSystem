mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use tracing::info;

use infrastructure::{config::Config, logging::init_logging, migrate};
use presentation::middleware::{RequestIdMiddleware, TimingMiddleware};

use crate::{data::Database, infrastructure::state::State};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    init_logging();

    let cfg = Config::from_env().expect("invalid config");
    // let pool = PgPoolOptions::new()
    //     .max_connections(10)
    //     .connect(&cfg.database_url)
    //     .await
    //     .expect("failed to connect to database");

    // info!("Running migrations");
    // migrate::run(&pool).await.expect("migrations failed");

    let state = Arc::new(State::new());
    let db = Database::STATE(state);
    let addr = format!("{}:{}", cfg.host, cfg.port);
    info!("Listening on http://{}", addr);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cfg.cors_origin)
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .supports_credentials()
            .max_age(600);

        info!("Start app");
        App::new()
            .wrap(TimingMiddleware)
            .wrap(RequestIdMiddleware)
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(cfg.clone()))
            .configure(presentation::api::configure)
    })
    .bind(addr)?
    .run()
    .await
}
