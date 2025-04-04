use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sqlx::Error as SqlxError;
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    // Authentication errors
    Unauthorized(String),
    
    // Client errors
    BadRequest(String),
    NotFound(String),
    Conflict(String),
    
    // Server errors
    Internal(String),
    Database(SqlxError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            Self::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::Conflict(msg) => write!(f, "Conflict: {}", msg),
            Self::Internal(msg) => write!(f, "Internal server error: {}", msg),
            Self::Database(err) => write!(f, "Database error: {}", err),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Self::Conflict(msg) => (StatusCode::CONFLICT, msg),
            Self::Internal(msg) => {
                eprintln!("Internal server error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            Self::Database(err) => {
                eprintln!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
            }
        }));

        (status, body).into_response()
    }
}

impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        match err {
            SqlxError::RowNotFound => Self::NotFound("Resource not found".to_string()),
            _ => Self::Database(err),
        }
    }
}

// Implement From for common error types
impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        Self::Unauthorized(format!("Password error: {}", err))
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Self::Unauthorized(format!("JWT error: {}", err))
    }
}

// Helper result type
pub type AppResult<T> = Result<T, AppError>; 