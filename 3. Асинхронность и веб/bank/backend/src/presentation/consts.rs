pub const USER_ID_HEADER: &str = "x-user-id";
pub const REFRESH_COOKIE: &str = "refresh-token";
pub const CSRF_TOKEN_COOKIE: &str = "csrf-token";
pub const CSRF_TOKEN_HEADER: &str = "x-csrf-token";

pub(super) const EXCLUDE_PATHS: &[&str] =
    &["/api/auth/login", "/api/auth/register", "/api/auth/refresh"];
