//! Atlassian error types with proper HTTP status mapping

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Atlassian API errors
#[derive(Debug, Error)]
pub enum AtlassianError {
    #[error("Unauthorized: Invalid or missing API token")]
    Unauthorized,

    #[error("Forbidden: Insufficient permissions")]
    Forbidden,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Rate limited: Please retry after {retry_after:?} seconds")]
    RateLimited { retry_after: Option<u64> },

    #[error("API error ({status}): {message}")]
    ApiError { status: u16, message: String },

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

impl From<reqwest::Error> for AtlassianError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            AtlassianError::NetworkError("Request timed out".to_string())
        } else if err.is_connect() {
            AtlassianError::NetworkError("Failed to connect to Atlassian API".to_string())
        } else {
            AtlassianError::NetworkError(err.to_string())
        }
    }
}

/// Cache/database errors
#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Cache not available")]
    NotAvailable,

    #[error("Cache expired")]
    Expired,
}

impl From<sqlx::Error> for CacheError {
    fn from(err: sqlx::Error) -> Self {
        CacheError::DatabaseError(err.to_string())
    }
}

impl From<serde_json::Error> for CacheError {
    fn from(err: serde_json::Error) -> Self {
        CacheError::SerializationError(err.to_string())
    }
}

/// Combined service error
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Atlassian API error: {0}")]
    Atlassian(#[from] AtlassianError),

    #[error("Cache error: {0}")]
    Cache(#[from] CacheError),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Service unavailable: {0}")]
    Unavailable(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            ServiceError::Atlassian(AtlassianError::Unauthorized) => (
                StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED",
                "Invalid or missing API token".to_string(),
            ),
            ServiceError::Atlassian(AtlassianError::Forbidden) => (
                StatusCode::FORBIDDEN,
                "FORBIDDEN",
                "Insufficient permissions".to_string(),
            ),
            ServiceError::Atlassian(AtlassianError::NotFound(msg)) => {
                (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone())
            }
            ServiceError::Atlassian(AtlassianError::RateLimited { retry_after }) => {
                let msg = match retry_after {
                    Some(secs) => format!("Rate limited. Please retry after {} seconds.", secs),
                    None => "Rate limited. Please try again later.".to_string(),
                };
                (StatusCode::TOO_MANY_REQUESTS, "RATE_LIMITED", msg)
            }
            ServiceError::Cache(CacheError::NotAvailable) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "SERVICE_UNAVAILABLE",
                "Cache service is temporarily unavailable".to_string(),
            ),
            ServiceError::Unavailable(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "SERVICE_UNAVAILABLE",
                msg.clone(),
            ),
            ServiceError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone()),
            ServiceError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg.clone()),
            ServiceError::DatabaseError(_) | ServiceError::Config(_) => {
                tracing::error!(error = %self, "Internal service error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "An internal error occurred. Please try again later.".to_string(),
                )
            }
            _ => {
                tracing::error!(error = %self, "Internal service error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "An internal error occurred. Please try again later.".to_string(),
                )
            }
        };

        let body = Json(json!({ "error": code, "message": message }));
        (status, body).into_response()
    }
}

/// Result type alias for service operations
pub type ServiceResult<T> = Result<T, ServiceError>;
