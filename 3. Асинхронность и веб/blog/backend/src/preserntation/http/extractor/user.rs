use crate::{domain::auth::JwtToken, preserntation::http::AppState};
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use uuid::Uuid;

pub struct UserIdExtracor(pub Uuid);

impl FromRequestParts<AppState> for UserIdExtracor {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        if let Some(jwt_token) = parts.extensions.get::<JwtToken>() {
            let Ok(claims) = jwt_token.verify(state.config.jwt_secret.as_ref()) else {
                return Err((StatusCode::UNAUTHORIZED, "Invalid token"));
            };
            Ok(UserIdExtracor(claims.sub))
        } else {
            Err((StatusCode::UNAUTHORIZED, "Missing jwt token"))
        }
    }
}
