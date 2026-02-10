use axum::http::Method;
use std::time::Duration;
use uuid::Uuid;

pub const HEADER_X_ID_REQUEST: &str = "X-Id-Request";
pub const MAX_AGE_CORS: Duration = Duration::from_secs(3600);

pub const COOKIE_REFRESH: &str = "refresh-token";

pub const COOKIE_CSRF_TOKEN: &str = "csrf-token";
pub const HEADER_CSRF_TOKEN: &str = "x-csrf-token";
pub const EXCLUDE_CSRF_PATHS: &[&str] =
    &["/api/auth/login", "/api/auth/register", "/api/auth/refresh"];
pub const METHODS_CSRF: &[Method] = &[Method::POST, Method::PUT, Method::DELETE];

#[derive(Debug, Clone)]
pub struct RequestId(pub Uuid);
