//! Unified application error type for consistent JSON error responses.
//!
//! Replaces ad-hoc `(StatusCode, String)` tuples in route handlers with a
//! single type that always returns `{ "error": "...", "message": "..." }`.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Application-wide error type for route handlers.
///
/// Every variant maps to an HTTP status code and produces a JSON body:
/// ```json
/// { "error": "ERROR_CODE", "message": "Human-readable explanation" }
/// ```
#[derive(Debug)]
pub enum AppError {
    /// 404 – resource not found
    NotFound(String),
    /// 409 – duplicate / conflict
    Conflict(String),
    /// 400 – invalid input
    BadRequest(String),
    /// 500 – internal failure (details are logged, NOT exposed to client)
    Internal(String),
}

impl AppError {
    /// Wrap a database or other internal error. The raw message is logged
    /// at `error` level but a generic message is returned to the caller.
    pub fn internal(source: impl std::fmt::Display) -> Self {
        tracing::error!(error = %source, "Internal error");
        Self::Internal(source.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone()),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "An internal error occurred. Please try again later.".to_string(),
            ),
        };

        (status, Json(json!({ "error": code, "message": message }))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::internal(err)
    }
}
