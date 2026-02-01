use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use serde::Serialize;
use std::future::{ready, Ready};

use crate::{infrastructure::error::ErrorApi, presentation::consts::REFRESH_COOKIE};

#[derive(Debug, Clone, Serialize)]
pub struct RefreshTokenExtractor(pub String);

impl FromRequest for RefreshTokenExtractor {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let Some(refresh_token) = req.cookie(REFRESH_COOKIE).map(|c| c.value().to_string()) else {
            return ready(Err(ErrorApi::Forbidden(
                "Missing refresh token cookie".to_string(),
            )
            .into()));
        };
        return ready(Ok(RefreshTokenExtractor(refresh_token)));
    }
}
