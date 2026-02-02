use actix_web::{dev::Payload, http::header, web::Data, Error, FromRequest, HttpRequest};
use serde::Serialize;
use std::future::{ready, Ready};
use uuid::Uuid;

use crate::infrastructure::{config::Config, error::ErrorApi, security::verify_jwt};

#[derive(Debug, Clone, Serialize)]
pub struct UserExtractor {
    pub id: Uuid,
}

impl FromRequest for UserExtractor {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        let token = match auth_header {
            Some(header) if header.starts_with("Bearer ") => &header[7..],
            _ => {
                return ready(Err(ErrorApi::Unauthorized(
                    "Missing or invalid Authorization header".to_string(),
                )
                .into()))
            }
        };

        let config = match req.app_data::<Data<Config>>() {
            Some(cfg) => cfg,
            None => {
                return ready(Err(ErrorApi::Unauthorized(
                    "Configuration not found".to_string(),
                )
                .into()))
            }
        };

        match verify_jwt(&config.jwt_secret, token) {
            Ok(id) => ready(Ok(UserExtractor { id })),
            Err(_) => ready(Err(ErrorApi::Unauthorized(
                "Invalid or expired token".to_string(),
            )
            .into())),
        }
    }
}
