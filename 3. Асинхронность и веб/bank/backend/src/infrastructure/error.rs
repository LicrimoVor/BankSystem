use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorApi {
    #[error("Forbidden")]
    Forbidden,

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("State error: {0}")]
    DataBase(String),

    #[error("Inner error")]
    Inner(String),
}

impl ResponseError for ErrorApi {
    fn error_response(&self) -> HttpResponse {
        let status = match self {
            ErrorApi::Forbidden => StatusCode::FORBIDDEN,
            ErrorApi::Validation(_) => StatusCode::BAD_REQUEST,
            ErrorApi::NotFound(_) => StatusCode::NOT_FOUND,
            ErrorApi::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorApi::DataBase(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorApi::Inner(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": self.to_string(),
            "status": status.as_u16(),
        }))
    }
}
