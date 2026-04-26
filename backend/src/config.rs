//! Application configuration module
//!
//! Centralizes all environment variable handling and configuration.

use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// Database connection URL
    pub database_url: String,
    /// Atlassian API token (from Codespaces secret ATLASSIAN_API_TOKEN)
    pub atlassian_api_token: String,
    /// Optional Atlassian email for Basic auth (not needed for Admin API)
    pub atlassian_email: Option<String>,
    /// GitHub Enterprise Personal Access Token (TR-011)  None if not configured
    pub github_pat_token: Option<String>,
    /// GitHub Enterprise slug, e.g. "equans" (TR-011)  None if not configured
    pub github_enterprise_slug: Option<String>,
    /// Cache TTL in hours (default: 25)
    pub cache_ttl_hours: i64,
    /// Sync interval in hours (default: 24)
    pub sync_interval_hours: u64,
    /// Backend server port
    pub backend_port: u16,
    /// Authentication configuration (optional for local development)
    pub auth: Option<AuthConfig>,
}

/// Authentication configuration for Azure AD
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// Azure AD tenant ID
    pub tenant_id: String,
    /// Azure AD client ID / application ID
    pub client_id: String,
    /// Expected audience (usually api://...)
    pub audience: String,
    /// Azure AD group ID for admin role (optional)
    pub admin_group_id: Option<String>,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        let database_url =
            env::var("DATABASE_URL").map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL"))?;

        let atlassian_api_token = env::var("ATLASSIAN_API_TOKEN")
            .map_err(|_| ConfigError::MissingEnvVar("ATLASSIAN_API_TOKEN"))?;

        let atlassian_email = env::var("ATLASSIAN_EMAIL").ok();

        // GitHub Enterprise PAT (TR-011 §2.1)  optional; GitHub endpoints disabled if absent
        let github_pat_token = env::var("GITHUB_PAT_TOKEN").ok().filter(|v| !v.is_empty());
        let github_enterprise_slug = env::var("GITHUB_ENTERPRISE_SLUG")
            .ok()
            .filter(|v| !v.is_empty());

        let cache_ttl_hours = env::var("CACHE_TTL_HOURS")
            .unwrap_or_else(|_| "25".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidValue("CACHE_TTL_HOURS"))?;

        let sync_interval_hours = env::var("SYNC_INTERVAL_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidValue("SYNC_INTERVAL_HOURS"))?;

        let backend_port = env::var("BACKEND_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidValue("BACKEND_PORT"))?;

        // Auth config is optional - if not set, auth middleware will not be used
        let auth = AuthConfig::from_env().ok();

        Ok(Self {
            database_url,
            atlassian_api_token,
            atlassian_email,
            github_pat_token,
            github_enterprise_slug,
            cache_ttl_hours,
            sync_interval_hours,
            backend_port,
            auth,
        })
    }

    /// Check if authentication is enabled
    pub fn auth_enabled(&self) -> bool {
        self.auth.is_some()
    }
}

impl AuthConfig {
    /// Load auth configuration from environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        let tenant_id = env::var("AZURE_AD_TENANT_ID")
            .map_err(|_| ConfigError::MissingEnvVar("AZURE_AD_TENANT_ID"))?;

        let client_id = env::var("AZURE_AD_CLIENT_ID")
            .map_err(|_| ConfigError::MissingEnvVar("AZURE_AD_CLIENT_ID"))?;

        let audience = env::var("AZURE_AD_AUDIENCE")
            .map_err(|_| ConfigError::MissingEnvVar("AZURE_AD_AUDIENCE"))?;

        let admin_group_id = env::var("ADMIN_GROUP_ID").ok();

        Ok(Self {
            tenant_id,
            client_id,
            audience,
            admin_group_id,
        })
    }

    /// Convert to auth module's AuthConfig
    pub fn to_auth_config(&self) -> crate::auth::AuthConfig {
        crate::auth::AuthConfig {
            tenant_id: self.tenant_id.clone(),
            client_id: self.client_id.clone(),
            audience: self.audience.clone(),
            admin_group_id: self.admin_group_id.clone(),
        }
    }
}

/// Configuration errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(&'static str),

    #[error("Invalid value for environment variable: {0}")]
    InvalidValue(&'static str),
}
