//! Authentication error types

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Authentication and authorization errors
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Missing authentication token")]
    MissingToken,

    #[error("Invalid Authorization header format")]
    InvalidAuthHeader,

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Token has expired")]
    TokenExpired,

    #[error("Invalid token audience")]
    InvalidAudience,

    #[error("Invalid token issuer")]
    InvalidIssuer,

    #[error("Key not found in JWKS: {0}")]
    KeyNotFound(String),

    #[error("Invalid key: {0}")]
    InvalidKey(String),

    #[error("Failed to fetch JWKS: {0}")]
    JwksError(String),

    #[error("Insufficient permissions")]
    InsufficientPermissions,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Authentication required"),
            AuthError::InvalidAuthHeader => {
                (StatusCode::UNAUTHORIZED, "Invalid Authorization header")
            }
            AuthError::InvalidToken(_) => {
                (StatusCode::UNAUTHORIZED, "Invalid authentication token")
            }
            AuthError::TokenExpired => (
                StatusCode::UNAUTHORIZED,
                "Session expired, please login again",
            ),
            AuthError::InvalidAudience | AuthError::InvalidIssuer => {
                (StatusCode::UNAUTHORIZED, "Invalid authentication token")
            }
            AuthError::KeyNotFound(_) | AuthError::InvalidKey(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Authentication service error",
            ),
            AuthError::JwksError(_) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Authentication service unavailable",
            ),
            AuthError::InsufficientPermissions => (
                StatusCode::FORBIDDEN,
                "You don't have permission to perform this action",
            ),
        };

        // Log actual error for debugging
        tracing::warn!(error = %self, "Authentication error");

        (status, Json(json!({ "error": message }))).into_response()
    }
}
