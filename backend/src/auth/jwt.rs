//! JWT validation for Azure AD tokens

use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

use crate::auth::claims::AzureAdClaims;
use crate::auth::error::AuthError;

/// JWKS (JSON Web Key Set) response from Azure AD
#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<Jwk>,
}

/// Individual JSON Web Key
#[derive(Debug, Deserialize)]
struct Jwk {
    kid: String,
    #[allow(dead_code)]
    kty: String,
    n: String,
    e: String,
}

/// Configuration for JWT validation
#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub tenant_id: String,
    pub client_id: String,
    pub audience: String,
    pub admin_group_id: Option<String>,
}

impl AuthConfig {
    /// Create config from environment variables
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            tenant_id: std::env::var("AZURE_AD_TENANT_ID")
                .map_err(|_| "AZURE_AD_TENANT_ID not set")?,
            client_id: std::env::var("AZURE_AD_CLIENT_ID")
                .map_err(|_| "AZURE_AD_CLIENT_ID not set")?,
            audience: std::env::var("AZURE_AD_AUDIENCE")
                .map_err(|_| "AZURE_AD_AUDIENCE not set")?,
            admin_group_id: std::env::var("ADMIN_GROUP_ID").ok(),
        })
    }
}

/// JWT Validator for Azure AD tokens
///
/// Validates JWT tokens against Azure AD's public keys (JWKS).
/// Keys are cached in memory for 1 hour to avoid excessive JWKS requests.
pub struct JwtValidator {
    client: Client,
    jwks_uri: String,
    tenant_id: String,
    audience: String,
    admin_group_id: Option<String>,
    // Simple in-memory cache for decoding keys
    // Key: kid (key ID), Value: (DecodingKey, timestamp)
    key_cache: Arc<RwLock<HashMap<String, (DecodingKey, std::time::Instant)>>>,
    cache_ttl: Duration,
}

impl JwtValidator {
    /// Create a new JWT validator
    #[allow(clippy::expect_used)]
    pub fn new(config: &AuthConfig) -> Self {
        let jwks_uri = format!(
            "https://login.microsoftonline.com/{}/discovery/v2.0/keys",
            config.tenant_id
        );

        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
            jwks_uri,
            tenant_id: config.tenant_id.clone(),
            audience: config.audience.clone(),
            admin_group_id: config.admin_group_id.clone(),
            key_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl: Duration::from_secs(3600), // 1 hour
        }
    }

    /// Get the admin group ID
    pub fn admin_group_id(&self) -> Option<&str> {
        self.admin_group_id.as_deref()
    }

    /// Validate JWT token and extract claims
    pub async fn validate(&self, token: &str) -> Result<AzureAdClaims, AuthError> {
        // Decode header to get key ID (kid)
        let header = decode_header(token)
            .map_err(|e| AuthError::InvalidToken(format!("Invalid JWT header: {}", e)))?;

        let kid = header
            .kid
            .ok_or_else(|| AuthError::InvalidToken("Missing kid in header".into()))?;

        // Get decoding key (from cache or fetch)
        let decoding_key = self.get_decoding_key(&kid).await?;

        // Configure validation
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&self.audience]);
        validation.set_issuer(&[
            format!("https://login.microsoftonline.com/{}/v2.0", self.tenant_id),
            format!("https://sts.windows.net/{}/", self.tenant_id),
        ]);
        validation.validate_exp = true;
        validation.validate_nbf = true;
        // Explicitly require both exp and nbf so that a malformed claim
        // (FailedToParse) is rejected rather than silently skipped.
        // This is the mitigation for CVE / GHSA jsonwebtoken type-confusion.
        validation.set_required_spec_claims(&["exp", "nbf"]);

        // Decode and validate token
        let token_data =
            decode::<AzureAdClaims>(token, &decoding_key, &validation).map_err(|e| {
                match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                    jsonwebtoken::errors::ErrorKind::InvalidAudience => AuthError::InvalidAudience,
                    jsonwebtoken::errors::ErrorKind::InvalidIssuer => AuthError::InvalidIssuer,
                    _ => AuthError::InvalidToken(e.to_string()),
                }
            })?;

        Ok(token_data.claims)
    }

    /// Get or fetch the decoding key for the given key ID
    async fn get_decoding_key(&self, kid: &str) -> Result<DecodingKey, AuthError> {
        // Check cache first
        {
            let cache = self.key_cache.read().await;
            if let Some((key, timestamp)) = cache.get(kid) {
                if timestamp.elapsed() < self.cache_ttl {
                    return Ok(key.clone());
                }
            }
        }

        // Fetch JWKS from Azure AD
        tracing::debug!("Fetching JWKS from Azure AD");
        let jwks: JwksResponse = self
            .client
            .get(&self.jwks_uri)
            .send()
            .await
            .map_err(|e| AuthError::JwksError(format!("Failed to fetch JWKS: {}", e)))?
            .json()
            .await
            .map_err(|e| AuthError::JwksError(format!("Failed to parse JWKS: {}", e)))?;

        // Find matching key
        let jwk = jwks
            .keys
            .iter()
            .find(|k| k.kid == kid)
            .ok_or_else(|| AuthError::KeyNotFound(kid.to_string()))?;

        // Create decoding key from RSA components
        let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
            .map_err(|e| AuthError::InvalidKey(e.to_string()))?;

        // Cache the key
        {
            let mut cache = self.key_cache.write().await;
            cache.insert(
                kid.to_string(),
                (decoding_key.clone(), std::time::Instant::now()),
            );
        }

        Ok(decoding_key)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_config_from_env() {
        std::env::set_var("AZURE_AD_TENANT_ID", "test-tenant");
        std::env::set_var("AZURE_AD_CLIENT_ID", "test-client");
        std::env::set_var("AZURE_AD_AUDIENCE", "api://test");
        std::env::set_var("ADMIN_GROUP_ID", "admin-group");

        let config = AuthConfig::from_env().unwrap();
        assert_eq!(config.tenant_id, "test-tenant");
        assert_eq!(config.client_id, "test-client");
        assert_eq!(config.audience, "api://test");
        assert_eq!(config.admin_group_id, Some("admin-group".to_string()));
    }

    #[test]
    fn test_jwks_uri_construction() {
        let config = AuthConfig {
            tenant_id: "my-tenant-id".to_string(),
            client_id: "my-client-id".to_string(),
            audience: "api://my-app".to_string(),
            admin_group_id: None,
        };

        let validator = JwtValidator::new(&config);
        assert_eq!(
            validator.jwks_uri,
            "https://login.microsoftonline.com/my-tenant-id/discovery/v2.0/keys"
        );
    }
}
