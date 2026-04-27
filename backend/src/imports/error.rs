//! Error types for import module (FR-007)

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Import-related errors
#[derive(Debug, Error)]
pub enum ImportError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Validation failed: {0} errors found")]
    ValidationError(usize),

    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),

    #[error("File too large: {0} bytes (maximum 50MB)")]
    FileTooLarge(usize),

    #[error("Import not found: {0}")]
    NotFound(String),

    #[error("Preview not found: {0}")]
    PreviewNotFound(String),

    #[error("Upload not found: {0}")]
    UploadNotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Transaction failed, rollback completed")]
    TransactionFailed,

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl From<sqlx::Error> for ImportError {
    fn from(err: sqlx::Error) -> Self {
        ImportError::DatabaseError(err.to_string())
    }
}

impl From<csv::Error> for ImportError {
    fn from(err: csv::Error) -> Self {
        ImportError::ParseError(err.to_string())
    }
}

impl IntoResponse for ImportError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            ImportError::ParseError(msg) => (StatusCode::BAD_REQUEST, "PARSE_ERROR", msg),
            ImportError::ValidationError(count) => (
                StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR",
                format!("Validation failed: {} errors found", count),
            ),
            ImportError::UnsupportedFormat(msg) => {
                (StatusCode::BAD_REQUEST, "UNSUPPORTED_FORMAT", msg)
            }
            ImportError::FileTooLarge(size) => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "FILE_TOO_LARGE",
                format!("File too large: {} bytes (maximum 50MB)", size),
            ),
            ImportError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                format!("Import not found: {}", msg),
            ),
            ImportError::PreviewNotFound(msg) => (
                StatusCode::NOT_FOUND,
                "PREVIEW_NOT_FOUND",
                format!("Preview not found: {}", msg),
            ),
            ImportError::UploadNotFound(msg) => (
                StatusCode::NOT_FOUND,
                "UPLOAD_NOT_FOUND",
                format!("Upload not found: {}", msg),
            ),
            ImportError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DATABASE_ERROR",
                format!("Database error: {}", msg),
            ),
            ImportError::TransactionFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "TRANSACTION_FAILED",
                "Transaction failed, rollback completed".to_string(),
            ),
            ImportError::InvalidRequest(msg) => (StatusCode::BAD_REQUEST, "INVALID_REQUEST", msg),
            ImportError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg),
            ImportError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", msg)
            }
        };

        let body = Json(json!({
            "error": error_type,
            "message": message,
        }));

        (status, body).into_response()
    }
}

/// Result type for import operations
pub type ImportResult<T> = Result<T, ImportError>;
