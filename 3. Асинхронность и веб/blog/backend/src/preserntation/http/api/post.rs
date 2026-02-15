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
use utoipa::OpenApi;
use uuid::Uuid;

#[utoipa::path(
    post,
    tag = "post",
    path = "/api/post/",
    request_body = PostCreate,
    responses((status = 201, body = PostResponse)),
    security(("jwt" = []))
)]
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

#[utoipa::path(
    patch,
    tag = "post",
    path = "/api/post/{post_id}",
    request_body = PostUpdate,
    responses((status = 200, body = PostResponse)),
    security(("jwt" = []))
)]
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

#[utoipa::path(
    delete,
    tag = "post",
    path = "/api/post/{post_id}",
    responses((status = 204)),
    security(("jwt" = []))
)]
async fn delete_post(
    State(state): State<AppState>,
    UserIdExtracor(user_id): UserIdExtracor,
    Path(post_id): Path<Uuid>,
) -> Result<impl IntoResponse, ErrorBlog> {
    let AppState { database, .. } = state;
    let post_service = PostService(database);
    post_service.delete(user_id, post_id).await?;
    Ok((StatusCode::NO_CONTENT, ()).into_response())
}

#[utoipa::path(
    get,
    tag = "post",
    path = "/api/post/{post_id}",
    responses((status = 200, body = PostResponse))
)]
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

#[utoipa::path(
    get,
    tag = "post",
    path = "/api/post/author/{email}",
    responses((status = 200, body = Vec<PostResponse>))
)]
async fn gets_by_author(
    State(state): State<AppState>,
    Path(email): Path<String>,
) -> Result<impl IntoResponse, ErrorBlog> {
    // В идеале конечно сделать пагинацию...
    let AppState { database, .. } = state;
    let post_service = PostService(database.clone());
    let user_service = UserService(database);
    let user = user_service.get_by_email(email).await?;
    let posts = post_service.gets_by_author(*user.id()).await?;
    let data: Vec<PostResponse> = posts
        .into_iter()
        .map(|post| PostResponse::new(user.clone(), post))
        .collect();

    Ok((StatusCode::OK, Json(json!(data))).into_response())
}

#[utoipa::path(
    get,
    tag = "post",
    path = "/api/post/me",
    responses((status = 200, body = Vec<PostResponse>)),
    security(("jwt" = []))
)]
async fn gets_me(
    State(state): State<AppState>,
    UserIdExtracor(user_id): UserIdExtracor,
) -> Result<impl IntoResponse, ErrorBlog> {
    // В идеале конечно сделать пагинацию...
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
        .route("/author/{email}", get(gets_by_author))
        .route("/me", get(gets_me))
        .route("/{post_id}", patch(update_post))
        .route("/{post_id}", delete(delete_post))
        .route("/{post_id}", get(get_by_id))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        create_post,
        update_post,
        delete_post,
        get_by_id,
        gets_by_author,
        gets_me,
    ),
    components(
        schemas(PostCreate, PostResponse, PostUpdate),
    ),
    tags((name = "post", description = "Post API"))
)]
pub struct Doc;
