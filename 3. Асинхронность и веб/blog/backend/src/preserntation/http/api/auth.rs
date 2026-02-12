use crate::{
    application::auth::AuthService,
    infrastructure::errors::ErrorBlog,
    preserntation::http::{
        AppState,
        consts::COOKIE_REFRESH,
        dto::auth::{AuthLoginRequest, AuthLoginResponse, AuthRegister},
        extractor::{refresh::RefreshExtracor, user::UserIdExtracor},
    },
    utils::cookie::set_cookie,
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use cookie::{Cookie, SameSite};
use serde_json::json;
use utoipa::OpenApi;

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = AuthRegister,
    responses((status = 200, body = AuthLoginResponse))
)]
async fn register(
    State(state): State<AppState>,
    Json(data): Json<AuthRegister>,
) -> Result<impl IntoResponse, ErrorBlog> {
    let AppState { database, config } = state;
    let AuthRegister {
        username,
        email,
        password,
    } = data;
    let service = AuthService(database);
    let (user, refresh, jwt) = service.register(config, username, email, password).await?;

    let mut res = (
        StatusCode::OK,
        Json(json!(AuthLoginResponse::new(user, jwt))),
    )
        .into_response();

    let cookie = Cookie::build((COOKIE_REFRESH, refresh.0))
        .path("/api/auth/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .build();
    set_cookie(&mut res, cookie);

    Ok(res)
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = AuthLoginRequest,
    responses((status = 200, body = AuthLoginResponse))
)]
async fn login(
    State(state): State<AppState>,
    Json(data): Json<AuthLoginRequest>,
) -> Result<impl IntoResponse, ErrorBlog> {
    let AppState { database, config } = state;
    let AuthLoginRequest { email, password } = data;
    let service = AuthService(database);
    let (user, refresh, jwt) = service.login(config, email, password).await?;
    let mut res = (
        StatusCode::OK,
        Json(json!(AuthLoginResponse::new(user, jwt))),
    )
        .into_response();

    let cookie = Cookie::build((COOKIE_REFRESH, refresh.0))
        .path("/api/auth/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .build();
    set_cookie(&mut res, cookie);

    Ok(res)
}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    request_body = AuthLoginRequest,
    security(("jwt" = [])),
    responses((status = 204))
)]
async fn logout(
    State(state): State<AppState>,
    RefreshExtracor(refresh): RefreshExtracor,
    UserIdExtracor(user_id): UserIdExtracor,
) -> Result<impl IntoResponse, ErrorBlog> {
    let AppState { database, .. } = state;
    let service = AuthService(database);
    service.logout(user_id, refresh).await?;
    Ok((StatusCode::NO_CONTENT, ()).into_response())
}

#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    request_body = AuthLoginRequest,
    security(("bearerAuth" = [])),
    responses((status = 200, body = String))
)]
async fn refresh(
    State(state): State<AppState>,
    RefreshExtracor(refresh): RefreshExtracor,
) -> Result<impl IntoResponse, ErrorBlog> {
    let AppState { database, config } = state;
    let service = AuthService(database);
    let jwt = service.refresh(config, refresh).await?;
    Ok((StatusCode::OK, Json(json!(jwt))))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        register,
        login,
        logout,
        refresh,
    ),
    components(
        schemas(AuthLoginResponse, AuthLoginRequest, AuthRegister),
    ),
    tags((name = "auth", description = "Auth API"))
)]
pub struct Doc;
