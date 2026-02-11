use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::sqlx;
use serde_json::json;

#[derive(Debug, thiserror::Error, Clone)]
pub enum ErrorBlog {
    #[error("Error in database: {0}")]
    Database(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Unauthorized error: {0}")]
    Unauthorized(String),
    #[error("Argument error: {0}")]
    Argument(String),
    #[error("Forbidden error: {0}")]
    Forbidden(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<axum::http::Error> for ErrorBlog {
    fn from(err: axum::http::Error) -> Self {
        ErrorBlog::Internal(err.to_string())
    }
}

impl From<sqlx::Error> for ErrorBlog {
    fn from(err: sqlx::Error) -> Self {
        ErrorBlog::Database(err.to_string())
    }
}

impl From<sea_orm::DbErr> for ErrorBlog {
    fn from(err: sea_orm::DbErr) -> Self {
        ErrorBlog::Database(err.to_string())
    }
}

impl From<uuid::Error> for ErrorBlog {
    fn from(err: uuid::Error) -> Self {
        ErrorBlog::Validation(format!("UUID error: {}", err.to_string()))
    }
}

impl From<chrono::ParseError> for ErrorBlog {
    fn from(err: chrono::ParseError) -> Self {
        ErrorBlog::Validation(format!("Chrono error: {}", err.to_string()))
    }
}

impl From<serde_json::Error> for ErrorBlog {
    fn from(err: serde_json::Error) -> Self {
        ErrorBlog::Validation(format!("JSON error: {}", err.to_string()))
    }
}

impl IntoResponse for ErrorBlog {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ErrorBlog::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ErrorBlog::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ErrorBlog::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            ErrorBlog::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ErrorBlog::Argument(msg) => (StatusCode::BAD_REQUEST, msg),
            ErrorBlog::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ErrorBlog::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
        };
        let body = Json(json!({
            "status": status.as_str(),
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

impl From<ErrorBlog> for tonic::Status {
    fn from(err: ErrorBlog) -> Self {
        let (code, message) = match err {
            ErrorBlog::Database(msg) => (tonic::Code::Internal, msg),
            ErrorBlog::NotFound(msg) => (tonic::Code::NotFound, msg),
            ErrorBlog::Validation(msg) => (tonic::Code::InvalidArgument, msg),
            ErrorBlog::Unauthorized(msg) => (tonic::Code::Unauthenticated, msg),
            ErrorBlog::Argument(msg) => (tonic::Code::InvalidArgument, msg),
            ErrorBlog::Internal(msg) => (tonic::Code::Internal, msg),
            ErrorBlog::Forbidden(msg) => (tonic::Code::PermissionDenied, msg),
        };
        tonic::Status::new(code, message)
    }
}
