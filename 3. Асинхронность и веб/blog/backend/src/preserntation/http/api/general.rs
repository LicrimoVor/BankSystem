use axum::{Router, routing::get};

async fn health() -> &'static str {
    "ok"
}

async fn ping() -> &'static str {
    "pong"
}

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ping", get(ping))
}
