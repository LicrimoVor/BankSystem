use crate::{
    application::user::UserService,
    infrastructure::errors::ErrorBlog,
    preserntation::http::{
        AppState,
        dto::user::{UserResponse, UserUpdate},
        extractor::user::UserIdExtracor,
    },
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch},
};
use serde_json::json;

async fn me_user(
    State(state): State<AppState>,
    UserIdExtracor(user_id): UserIdExtracor,
) -> Result<impl IntoResponse, ErrorBlog> {
    let AppState { database, .. } = state;
    let service = UserService(database);
    let user = service.get_by_id(user_id).await?;

    Ok((StatusCode::OK, Json(json!(UserResponse::new(user)))).into_response())
}

async fn get_user_by_email(
    State(state): State<AppState>,
    Path(user_email): Path<String>,
) -> Result<impl IntoResponse, ErrorBlog> {
    let AppState { database, .. } = state;
    let service = UserService(database);
    let user = service.get_by_email(user_email).await?;

    Ok((StatusCode::OK, Json(json!(UserResponse::new(user)))).into_response())
}

async fn update_user(
    State(state): State<AppState>,
    UserIdExtracor(user_id): UserIdExtracor,
    Json(user): Json<UserUpdate>,
) -> Result<impl IntoResponse, ErrorBlog> {
    let AppState { database, .. } = state;
    let UserUpdate {
        username,
        email,
        password,
    } = user;
    if let (true, true, true) = (username.is_none(), email.is_none(), password.is_none()) {
        return Ok((StatusCode::BAD_REQUEST, ()).into_response());
    }
    let service = UserService(database);
    let user = service.update(user_id, username, email, password).await?;
    Ok((StatusCode::OK, Json(json!(UserResponse::new(user)))).into_response())
}

async fn delete_user(
    State(state): State<AppState>,
    UserIdExtracor(user_id): UserIdExtracor,
) -> Result<impl IntoResponse, ErrorBlog> {
    let AppState { database, .. } = state;
    let service = UserService(database);
    service.delete(user_id).await?;

    Ok((StatusCode::NO_CONTENT, ()))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{user_email}", get(get_user_by_email))
        .route("/me", get(me_user))
        .route("/me", patch(update_user))
        .route("/me", delete(delete_user))
}
