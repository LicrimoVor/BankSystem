mod api;
mod consts;
pub mod dto;
pub(self) mod extractor;
pub(self) mod middleware;
use self::consts::{HEADER_CSRF_TOKEN, HEADER_X_ID_REQUEST, MAX_AGE_CORS};
use self::middleware::{csrf::CsrfLayer, req_id::RequestIdLayer, time::TimeLayer};
use crate::preserntation::http::api::{auth, general, post, user};
use crate::{
    data::Database, infrastructure::config::Config, preserntation::http::middleware::jwt::JwtLayer,
};
use anyhow::Result;
use axum::http::{HeaderValue, Method, header};
use std::sync::Arc;
use std::vec;
use tower_http::cors::CorsLayer;
use tracing::info;
use utoipa::openapi::SecurityRequirement;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{OpenApi, openapi};
use utoipa_redoc::{Redoc, Servable};

#[derive(Clone)]
pub(self) struct AppState {
    pub database: Arc<Database>,
    pub config: Arc<Config>,
}

/// Еще я без понятия как сделать security_schema (она просто не работает)
/// Поэтому буду очень любезен если подскажите как сделать ;)
#[derive(utoipa::OpenApi)]
#[openapi()]
pub struct ApiDoc;

impl ApiDoc {
    pub fn openapi() -> openapi::OpenApi {
        let mut api = openapi::OpenApi::builder()
            .security(Some(vec![SecurityRequirement::new(
                "jwt",
                ["edit:items", "read:items"],
            )]))
            .build();
        if let Some(schema) = api.components.as_mut() {
            schema.add_security_scheme(
                "jwt",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }

        api.merge(auth::Doc::openapi());
        api.merge(user::Doc::openapi());
        api.merge(post::Doc::openapi());
        api.merge(general::Doc::openapi());

        api
    }
}

pub fn http_init(config: Arc<Config>, database: Arc<Database>) -> Result<axum::Router> {
    let origin = config
        .cors_origin
        .iter()
        .map(|cors| cors.parse::<HeaderValue>())
        .collect::<Result<Vec<HeaderValue>, _>>()?;

    info!("CORS origin: {:?}", origin);
    let cors_layer = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([
            Method::DELETE,
            Method::POST,
            Method::GET,
            Method::PATCH,
            Method::OPTIONS,
        ])
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
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
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
