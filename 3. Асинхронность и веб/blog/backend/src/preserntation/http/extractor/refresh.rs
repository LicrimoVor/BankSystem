use super::super::consts::COOKIE_REFRESH;
use crate::{domain::auth::RefreshToken, utils::cookie::extract_cookie};
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};

pub struct RefreshExtracor(pub RefreshToken);

impl<S> FromRequestParts<S> for RefreshExtracor
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(refresh_token) = extract_cookie(&parts.headers, COOKIE_REFRESH) {
            Ok(Self(RefreshToken::from(refresh_token)))
        } else {
            Err((StatusCode::UNAUTHORIZED, "Missing refresh token"))
        }
    }
}
