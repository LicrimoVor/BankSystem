use crate::domain::auth::JwtToken;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use uuid::Uuid;

pub struct UserIdExtracor(pub Uuid);

impl<S> FromRequestParts<S> for UserIdExtracor
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(jwt_token) = parts.extensions.get::<JwtToken>() {
            let Ok(claims) = jwt_token.verify() else {
                return Err((StatusCode::UNAUTHORIZED, "Invalid token"));
            };
            Ok(UserIdExtracor(claims.sub))
        } else {
            Err((StatusCode::UNAUTHORIZED, "Missing jwt token"))
        }
    }
}
