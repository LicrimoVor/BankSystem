mod api;
mod consts;
pub(self) mod extractor;
pub(self) mod middleware;
use std::sync::Arc;

// use super::middleware;
use crate::{data::Database, infrastructure::config::Config};
use anyhow::Result;
use axum::http::{HeaderValue, Method, header};
use tower_http::cors::CorsLayer;

use self::consts::{HEADER_CSRF_TOKEN, HEADER_X_ID_REQUEST, MAX_AGE_CORS};

pub fn router_init(config: &Config, database: Arc<Database>) -> Result<axum::Router> {
    let origin = config.cors_origin.parse::<HeaderValue>()?;
    Ok(axum::Router::new()
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_methods([Method::DELETE, Method::POST, Method::GET, Method::PUT])
                .allow_origin(origin)
                .allow_headers([
                    header::AUTHORIZATION,
                    header::CONTENT_TYPE,
                    header::HeaderName::from_static(HEADER_X_ID_REQUEST),
                    header::HeaderName::from_static(HEADER_CSRF_TOKEN),
                ])
                .max_age(MAX_AGE_CORS),
        )
        .with_state(database))
    // .merge(post::router())
    // .merge(user::router())
}
