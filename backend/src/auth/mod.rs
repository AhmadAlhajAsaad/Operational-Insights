//! Authentication and Authorization module
//!
//! This module provides JWT-based authentication using Azure AD (Entra ID).
//! It validates tokens against Azure AD's public keys and supports role-based
//! access control via Azure AD groups and app roles.
//!
//! # Components
//!
//! - [`claims`] - JWT claims types for Azure AD tokens
//! - [`error`] - Authentication error types
//! - [`jwt`] - JWT validation logic
//! - [`middleware`] - Axum middleware for authentication
//!
//! # Usage
//!
//! ```rust,ignore
//! use crate::auth::{AuthConfig, JwtValidator, auth_middleware};
//! use std::sync::Arc;
//!
//! // Create validator
//! let config = AuthConfig::from_env()?;
//! let validator = Arc::new(JwtValidator::new(&config));
//!
//! // Add middleware to protected routes
//! let protected = Router::new()
//!     .route("/api/data", get(handler))
//!     .layer(middleware::from_fn_with_state(validator, auth_middleware));
//! ```

pub mod claims;
pub mod error;
pub mod jwt;
pub mod middleware;

// Re-exports for convenience
pub use jwt::{AuthConfig, JwtValidator};
pub use middleware::auth_middleware;
