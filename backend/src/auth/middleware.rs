//! Axum middleware for JWT authentication

use axum::{
    extract::{FromRequestParts, Request, State},
    http::{header, request::Parts},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::auth::{claims::AzureAdClaims, error::AuthError, jwt::JwtValidator};

/// Extension type for accessing authenticated user in handlers
#[derive(Clone, Debug)]
pub struct AuthenticatedUser(pub AzureAdClaims);

impl AuthenticatedUser {
    /// Get the user's claims
    pub fn claims(&self) -> &AzureAdClaims {
        &self.0
    }

    /// Get the user's ID for logging
    pub fn user_id(&self) -> &str {
        self.0.user_id()
    }

    /// Check if user has admin privileges
    pub fn is_admin(&self, admin_group_id: Option<&str>) -> bool {
        self.0.is_admin(admin_group_id)
    }
}

/// Extract AuthenticatedUser from request extensions
#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthenticatedUser>()
            .cloned()
            .ok_or(AuthError::MissingToken)
    }
}

/// Authentication middleware - validates JWT token
///
/// This middleware extracts the Bearer token from the Authorization header,
/// validates it against Azure AD's public keys, and adds the authenticated
/// user to the request extensions for use in handlers.
pub async fn auth_middleware(
    State(validator): State<Arc<JwtValidator>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    // Extract Bearer token from Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::MissingToken)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthError::InvalidAuthHeader)?;

    // Validate token
    let claims = validator.validate(token).await?;

    // Log authenticated request (without sensitive data)
    tracing::info!(
        user_id = claims.user_id(),
        method = %request.method(),
        path = %request.uri().path(),
        "Authenticated API request"
    );

    // Add claims to request extensions for use in handlers
    request.extensions_mut().insert(AuthenticatedUser(claims));

    Ok(next.run(request).await)
}

/// Middleware that requires admin role
///
/// Must be used after auth_middleware to ensure user is authenticated.
pub async fn require_admin_middleware(
    State(validator): State<Arc<JwtValidator>>,
    user: AuthenticatedUser,
    request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let admin_group_id = validator.admin_group_id();

    if user.is_admin(admin_group_id) {
        tracing::info!(
            user_id = user.user_id(),
            path = %request.uri().path(),
            "Admin access granted"
        );
        Ok(next.run(request).await)
    } else {
        tracing::warn!(
            user_id = user.user_id(),
            path = %request.uri().path(),
            "Authorization denied - insufficient permissions"
        );
        Err(AuthError::InsufficientPermissions)
    }
}

/// Optional authentication middleware
///
/// Similar to auth_middleware but doesn't fail if no token is present.
/// Useful for endpoints that work both with and without authentication.
pub async fn optional_auth_middleware(
    State(validator): State<Arc<JwtValidator>>,
    mut request: Request,
    next: Next,
) -> Response {
    // Try to extract and validate token
    if let Some(auth_header) = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
    {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            if let Ok(claims) = validator.validate(token).await {
                tracing::debug!(
                    user_id = claims.user_id(),
                    "Optional auth: user authenticated"
                );
                request.extensions_mut().insert(AuthenticatedUser(claims));
            }
        }
    }

    next.run(request).await
}
