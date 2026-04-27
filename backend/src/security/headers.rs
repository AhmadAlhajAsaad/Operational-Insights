//! Security headers middleware (TM-01, OWASP)
//!
//! Adds security headers to all HTTP responses as specified in the
//! Privacy & Security Plan section 7.

use axum::{extract::Request, middleware::Next, response::Response};
use http::header::HeaderValue;

/// Middleware that adds security headers to every response.
///
/// Headers added:
/// - Strict-Transport-Security: Enforces HTTPS (TM-01)
/// - X-Content-Type-Options: Prevents MIME-sniffing
/// - X-Frame-Options: Prevents clickjacking
/// - Content-Security-Policy: Mitigates XSS attacks
/// - Referrer-Policy: Controls referrer information leakage
/// - X-XSS-Protection: Legacy XSS protection for older browsers
/// - Permissions-Policy: Restricts browser feature access
pub async fn security_headers_middleware(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    headers.insert(
        "Strict-Transport-Security",
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );
    headers.insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff"),
    );
    headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
    headers.insert(
        "Content-Security-Policy",
        HeaderValue::from_static(
            "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self'; connect-src 'self'; frame-ancestors 'none'"
        ),
    );
    headers.insert(
        "Referrer-Policy",
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        "X-XSS-Protection",
        HeaderValue::from_static("1; mode=block"),
    );
    headers.insert(
        "Permissions-Policy",
        HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );

    response
}
