use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to load enviroment variables from .env file : {0}")]
    Dotenv(#[from] dotenv::Error),
    #[error("Encryption error : {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("Database error : {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("Status code : {0}")]
    StatusCode(StatusCode),
    #[error("Conflict")]
    ConflictError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        log::error!("Error occured : {}", self);

        match self {
            Self::StatusCode(code) => code.into_response(),
            Self::ConflictError(str) => (StatusCode::CONFLICT, str).into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }
}

impl From<StatusCode> for AppError {
    fn from(value: StatusCode) -> Self {
        Self::StatusCode(value)
    }
}
