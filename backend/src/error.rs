use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

use crate::domain::errors::DomainError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Database error: {0}")]
    Database(#[from] surrealdb::Error),
}

impl From<DomainError> for AppError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::RequiredFieldMissing { field } => {
                AppError::Validation(format!("{} is required", field))
            }
            DomainError::InvalidField { field, reason } => {
                AppError::Validation(format!("{}: {}", field, reason))
            }
            DomainError::InvalidStateTransition { from, to, reason } => {
                AppError::BadRequest(format!(
                    "Cannot transition from {} to {}: {}",
                    from, to, reason
                ))
            }
            DomainError::BusinessRuleViolation { rule, details } => {
                AppError::BadRequest(format!("{}: {}", rule, details))
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
