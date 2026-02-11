use crate::{
    application::{post::PostService, user::UserService},
    infrastructure::errors::ErrorBlog,
    preserntation::http::{
        AppState,
        dto::post::{PostCreate, PostResponse, PostUpdate},
        extractor::user::UserIdExtracor,
    },
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
};
use serde_json::json;
use uuid::Uuid;

async fn create_post(
    State(state): State<AppState>,
    UserIdExtracor(user_id): UserIdExtracor,
    Json(post): Json<PostCreate>,
) -> Result<impl IntoResponse, ErrorBlog> {
    let AppState { database, config } = state;
    let PostCreate {
        title,
        content,
        img_base64,
    } = post;
    let post_service = PostService(database.clone());
    let user_service = UserService(database);
    let user = user_service.get_by_id(user_id).await?;
    let post = post_service
        .create(config, title, content, user_id, img_base64)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(json!(PostResponse::new(user, post))),
    )
        .into_response())
}

async fn update_post(
    State(state): State<AppState>,
    UserIdExtracor(user_id): UserIdExtracor,
    Path(post_id): Path<Uuid>,
    Json(post): Json<PostUpdate>,
) -> Result<impl IntoResponse, ErrorBlog> {
    let AppState { database, config } = state;
    let PostUpdate {
        title,
        content,
        img_base64,
    } = post;
    let post_service = PostService(database.clone());
    let user_service = UserService(database);
    let user = user_service.get_by_id(user_id).await?;
    let post = post_service
        .update(config, post_id, user_id, title, content, img_base64)
        .await?;

    Ok((StatusCode::OK, Json(json!(PostResponse::new(user, post)))).into_response())
}

async fn delete_post(
    State(state): State<AppState>,
    UserIdExtracor(user_id): UserIdExtracor,
    Path(post_id): Path<Uuid>,
) -> Result<impl IntoResponse, ErrorBlog> {
    let AppState { database, .. } = state;
    let service = PostService(database);
    service.delete(user_id, post_id).await?;
    Ok((StatusCode::NO_CONTENT, ()).into_response())
}

async fn get_by_id(
    State(state): State<AppState>,
    Path(post_id): Path<Uuid>,
) -> Result<impl IntoResponse, ErrorBlog> {
    let AppState { database, .. } = state;
    let post_service = PostService(database.clone());
    let user_service = UserService(database);
    let post = post_service.get_by_id(post_id).await?;
    let user = user_service.get_by_id(*post.author_id()).await?;

    Ok((StatusCode::OK, Json(json!(PostResponse::new(user, post)))).into_response())
}

/// В идеале конечно сделать пагинацию...
async fn gets_by_user_id(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, ErrorBlog> {
    let AppState { database, .. } = state;
    let post_service = PostService(database.clone());
    let user_service = UserService(database);
    let user = user_service.get_by_id(user_id).await?;
    let posts = post_service.gets_by_author(user_id).await?;
    let data: Vec<PostResponse> = posts
        .into_iter()
        .map(|post| PostResponse::new(user.clone(), post))
        .collect();

    Ok((StatusCode::OK, Json(json!(data))).into_response())
}

/// В идеале конечно сделать пагинацию...
async fn gets_me(
    State(state): State<AppState>,
    UserIdExtracor(user_id): UserIdExtracor,
) -> Result<impl IntoResponse, ErrorBlog> {
    let AppState { database, .. } = state;
    let post_service = PostService(database.clone());
    let user_service = UserService(database);
    let user = user_service.get_by_id(user_id).await?;
    let posts = post_service.gets_by_author(user_id).await?;
    let data: Vec<PostResponse> = posts
        .into_iter()
        .map(|post| PostResponse::new(user.clone(), post))
        .collect();

    Ok((StatusCode::OK, Json(json!(data))).into_response())
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_post))
        .route("/author/{user_id}", get(gets_by_user_id))
        .route("/me", get(gets_me))
        .route("/{post_id}", patch(update_post))
        .route("/{post_id}", delete(delete_post))
        .route("/{post_id}", get(get_by_id))
}
