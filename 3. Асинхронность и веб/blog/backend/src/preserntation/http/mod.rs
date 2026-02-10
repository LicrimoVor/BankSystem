mod api;
mod consts;
pub mod dto;
pub(self) mod extractor;
pub(self) mod middleware;
use self::consts::{HEADER_CSRF_TOKEN, HEADER_X_ID_REQUEST, MAX_AGE_CORS};
use self::middleware::{csrf::CsrfLayer, req_id::RequestIdLayer, time::TimeLayer};
use crate::{
    data::Database, infrastructure::config::Config, preserntation::http::middleware::jwt::JwtLayer,
};
use anyhow::Result;
use axum::http::{HeaderValue, Method, header};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub fn router_init(config: &Config, database: Arc<Database>) -> Result<axum::Router> {
    let origin = config.cors_origin.parse::<HeaderValue>()?;
    let cors_layer = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([Method::DELETE, Method::POST, Method::GET, Method::PUT])
        .allow_origin(origin)
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::HeaderName::from_static(HEADER_X_ID_REQUEST),
            header::HeaderName::from_static(HEADER_CSRF_TOKEN),
        ])
        .max_age(MAX_AGE_CORS);

    Ok(axum::Router::new()
        .layer(TimeLayer)
        .layer(RequestIdLayer)
        .layer(cors_layer)
        .layer(CsrfLayer)
        // Уместен ли он тут? Или все в UserIdExtracor делать
        .layer(JwtLayer)
        .with_state(database))
    // .merge(post::router())
    // .merge(user::router())
}
