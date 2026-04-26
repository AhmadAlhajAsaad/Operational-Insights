//! Atlassian data types and DTOs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// Core Domain Types
// ============================================================================

/// Atlassian organization
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Organization {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Atlassian group
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

/// Product access information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductAccess {
    pub name: String,
    pub key: String,
}

/// Atlassian user
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub account_id: String,
    pub name: String,
    pub email: String,
    pub account_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_access: Option<Vec<ProductAccess>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
}

/// User summary for license details
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSummary {
    pub account_id: String,
    pub name: String,
    pub email: String,
    pub account_status: String,
}

// ============================================================================
// License Types
// ============================================================================

/// License count summary
#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseCount {
    pub product: String,
    pub total_users: usize,
    pub active_users: usize,
}

/// Detailed license information with user lists
#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseCountDetailed {
    pub product: String,
    pub total_users_count: usize,
    pub active_users_count: usize,
    pub total_users: Vec<UserSummary>,
    pub active_users: Vec<UserSummary>,
}

// ============================================================================
// API Response Types
// ============================================================================

/// Organizations API response
#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationsResponse {
    pub data: Vec<Organization>,
}

/// Groups API response
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupsResponse {
    pub data: Vec<Group>,
}

/// Users API response with pagination
#[derive(Debug, Serialize, Deserialize)]
pub struct UsersResponse {
    pub data: Vec<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
}

/// Pagination links
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Links {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

// ============================================================================
// Cache Response Types
// ============================================================================

/// Cache metadata for API responses
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheMetadata {
    pub cached: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale: Option<bool>,
}

impl CacheMetadata {
    /// Create metadata for fresh API response (not cached)
    pub fn fresh() -> Self {
        Self {
            cached: false,
            cached_at: None,
            expires_at: None,
            stale: None,
        }
    }

    /// Create metadata for cached response
    pub fn from_cache(cached_at: DateTime<Utc>, expires_at: DateTime<Utc>, stale: bool) -> Self {
        Self {
            cached: true,
            cached_at: Some(cached_at),
            expires_at: Some(expires_at),
            stale: if stale { Some(true) } else { None },
        }
    }
}

/// Generic cached response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct CachedResponse<T> {
    pub data: T,
    pub cache: CacheMetadata,
}

impl<T> CachedResponse<T> {
    pub fn fresh(data: T) -> Self {
        Self {
            data,
            cache: CacheMetadata::fresh(),
        }
    }

    pub fn cached(
        data: T,
        cached_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
        stale: bool,
    ) -> Self {
        Self {
            data,
            cache: CacheMetadata::from_cache(cached_at, expires_at, stale),
        }
    }
}

// ============================================================================
// Cache DB Models
// ============================================================================

/// Cached user record from database
#[allow(dead_code)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CachedUser {
    pub account_id: String,
    pub account_type: String,
    pub email: Option<String>,
    pub display_name: String,
    pub active: bool,
    pub raw_data: serde_json::Value,
    pub cached_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// Cached group record from database
#[allow(dead_code)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CachedGroup {
    pub group_id: String,
    pub name: String,
    pub raw_data: serde_json::Value,
    pub cached_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// Sync status record
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct SyncStatus {
    pub sync_type: String,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub last_success_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub items_synced: Option<i32>,
}

// ============================================================================
// Query Parameters
// ============================================================================

/// Query parameters for cache refresh
#[derive(Debug, Deserialize, Default)]
pub struct RefreshParams {
    #[serde(default)]
    pub force_refresh: bool,
}

/// Query parameters for user list filtering
#[derive(Debug, Deserialize, Default)]
pub struct UserListParams {
    /// Filter by account status (active, inactive, closed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Search by email (partial match)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Search by name (partial match)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter by product access (e.g., "jira-software", "confluence")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// Force refresh from API
    #[serde(default)]
    pub force_refresh: bool,
    /// Page number (1-based)
    pub page: Option<i64>,
    /// Items per page (max 500)
    pub per_page: Option<i64>,
}

/// Paginated user list response (from cache, with filtering)
#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub data: Vec<User>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub cache: CacheMetadata,
}

/// Per-product statistics entry
#[derive(Debug, Serialize)]
pub struct ProductStatEntry {
    pub product_key: String,
    pub total_count: i64,
    pub active_count: i64,
}

/// Aggregated product statistics (fast SQL aggregation from cache)
#[derive(Debug, Serialize)]
pub struct ProductStats {
    pub total_users: i64,
    pub active_users: i64,
    pub per_product: Vec<ProductStatEntry>,
}

// ============================================================================
// User Management DTOs
// ============================================================================

/// Request body for inviting a new user
#[derive(Debug, Deserialize, Serialize)]
pub struct InviteUserRequest {
    pub email: String,
    pub product_access: Vec<String>,
}

/// Response after inviting a user
#[derive(Debug, Serialize)]
pub struct InviteUserResponse {
    pub success: bool,
    pub message: String,
    pub account_id: Option<String>,
}

/// Request body for suspending a user
#[derive(Debug, Deserialize, Serialize)]
pub struct SuspendUserRequest {
    pub message: Option<String>,
}

/// Response after suspending a user
#[derive(Debug, Serialize)]
pub struct SuspendUserResponse {
    pub success: bool,
    pub message: String,
}

/// Response after deleting a user
#[derive(Debug, Serialize)]
pub struct DeleteUserResponse {
    pub success: bool,
    pub message: String,
}

/// Manual sync trigger request
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SyncRequest {
    /// Optionally specify organization ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
}

/// Sync result response
#[derive(Debug, Serialize)]
pub struct SyncResponse {
    pub success: bool,
    pub message: String,
    pub users_synced: usize,
    pub duration_ms: u128,
}

/// Link Atlassian accounts result response
#[derive(Debug, Serialize)]
pub struct LinkResponse {
    pub success: bool,
    pub message: String,
    pub linked_by_local_id: u32,
    pub linked_by_email: u32,
    pub linked_by_name: u32,
    pub no_match: u32,
    pub errors: u32,
    pub duration_ms: u128,
}

// ============================================================================
// Error Response
// ============================================================================

/// Standard error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl ErrorResponse {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            details: None,
        }
    }

    pub fn with_details(error: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            details: Some(details.into()),
        }
    }
}
