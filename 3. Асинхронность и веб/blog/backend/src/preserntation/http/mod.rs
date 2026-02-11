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

#[derive(Clone)]
pub(self) struct AppState {
    pub database: Arc<Database>,
    pub config: Arc<Config>,
}

pub fn http_init(config: Arc<Config>, database: Arc<Database>) -> Result<axum::Router> {
    let origin = config.cors_origin.parse::<HeaderValue>()?;
    let cors_layer = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([Method::DELETE, Method::POST, Method::GET, Method::PATCH])
        .allow_origin(origin)
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::HeaderName::from_static(HEADER_X_ID_REQUEST),
            header::HeaderName::from_static(HEADER_CSRF_TOKEN),
        ])
        .max_age(MAX_AGE_CORS);
    let app_state = AppState { database, config };
    let api_router = axum::Router::new()
        .merge(api::general::router())
        .nest("/auth", api::auth::router())
        .nest("/user", api::user::router())
        .nest("/post", api::post::router());

    Ok(axum::Router::new()
        .nest("/api", api_router)
        // Уместен ли он тут? Или все в UserIdExtracor делать
        .layer(JwtLayer)
        .layer(CsrfLayer)
        .layer(cors_layer)
        .layer(TimeLayer)
        .layer(RequestIdLayer)
        .with_state(app_state))
}
