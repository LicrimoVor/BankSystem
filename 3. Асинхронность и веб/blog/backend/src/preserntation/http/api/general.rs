use crate::{infrastructure::errors::ErrorBlog, preserntation::http::AppState};
use axum::{
    Router,
    extract::{Path, State},
    http::{StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use tokio::{fs::File, io::AsyncReadExt};

async fn health() -> &'static str {
    "ok"
}

async fn ping() -> &'static str {
    "pong"
}

async fn media(
    State(state): State<AppState>,
    Path(filename): Path<String>,
) -> Result<impl IntoResponse, ErrorBlog> {
    let AppState { config, .. } = state;
    let file_path = format!("{}/{}", config.media_path, filename.clone());
    let mut f = File::open(file_path)
        .await
        .map_err(|_| ErrorBlog::NotFound("Image not found".to_string()))?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)
        .await
        .map_err(|_| ErrorBlog::Internal("Failed to read file".to_string()))?;

    let headers = [
        (header::CONTENT_TYPE, "image/png"),
        (
            header::CONTENT_DISPOSITION,
            &format!("attachment; filename=\"{:?}\"", filename),
        ),
    ];

    Ok((StatusCode::OK, headers, buf).into_response())
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/ping", get(ping))
        .route("/media/{filename}", get(media))
}
