//! Atlassian API route handlers

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::atlassian::{
    types::{
        CachedResponse, DeleteUserResponse, InviteUserRequest, InviteUserResponse, RefreshParams,
        SuspendUserRequest, SuspendUserResponse, SyncRequest, SyncResponse, UserListParams,
    },
    AtlassianService, ServiceError,
};

use crate::atlassian::AtlassianLinkService;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub atlassian_service: Arc<AtlassianService>,
    pub link_service: Arc<AtlassianLinkService>,
}

// ============================================================================
// Organizations
// ============================================================================

/// GET /api/atlassian/organizations
pub async fn get_organizations(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ServiceError> {
    let response = state.atlassian_service.get_organizations().await?;
    Ok((cache_headers(&response), Json(response)))
}

// ============================================================================
// Users
// ============================================================================

/// GET /api/atlassian/organizations/:org_id/users
pub async fn get_users(
    State(state): State<AppState>,
    Path(org_id): Path<String>,
    Query(params): Query<RefreshParams>,
) -> Result<impl IntoResponse, ServiceError> {
    let response = state
        .atlassian_service
        .get_users(&org_id, params.force_refresh)
        .await?;
    Ok((cache_headers(&response), Json(response)))
}

// ============================================================================
// Groups
// ============================================================================

/// GET /api/atlassian/organizations/:org_id/groups
pub async fn get_groups(
    State(state): State<AppState>,
    Path(org_id): Path<String>,
    Query(params): Query<RefreshParams>,
) -> Result<impl IntoResponse, ServiceError> {
    let response = state
        .atlassian_service
        .get_groups(&org_id, params.force_refresh)
        .await?;
    Ok((cache_headers(&response), Json(response)))
}

// ============================================================================
// Licenses
// ============================================================================

/// GET /api/atlassian/organizations/:org_id/licenses/:product
pub async fn get_license_count(
    State(state): State<AppState>,
    Path((org_id, product)): Path<(String, String)>,
    Query(params): Query<RefreshParams>,
) -> Result<impl IntoResponse, ServiceError> {
    let response = state
        .atlassian_service
        .get_license_count(&org_id, &product, params.force_refresh)
        .await?;
    Ok((cache_headers(&response), Json(response)))
}

/// GET /api/atlassian/organizations/:org_id/licenses/:product/details
pub async fn get_license_count_detailed(
    State(state): State<AppState>,
    Path((org_id, product)): Path<(String, String)>,
    Query(params): Query<RefreshParams>,
) -> Result<impl IntoResponse, ServiceError> {
    let response = state
        .atlassian_service
        .get_license_count_detailed(&org_id, &product, params.force_refresh)
        .await?;
    Ok((cache_headers(&response), Json(response)))
}

// ============================================================================
// Product Statistics (fast aggregation from cache, no API call)
// ============================================================================

/// GET /api/atlassian/product-stats
pub async fn get_product_stats(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ServiceError> {
    let stats = state.atlassian_service.get_product_stats().await?;
    Ok(Json(stats))
}

// ============================================================================
// User Management (FR-008)
// ============================================================================

/// GET /api/atlassian/users?status=active&email=...&name=...&product=...
pub async fn get_users_list(
    State(state): State<AppState>,
    Query(params): Query<UserListParams>,
) -> Result<impl IntoResponse, ServiceError> {
    // Use cached org_id to avoid an HTTP round-trip on every request.
    let org_id = state.atlassian_service.resolve_org_id().await?;

    let response = state
        .atlassian_service
        .get_users_filtered(&org_id, &params)
        .await?;

    Ok(Json(response))
}

/// GET /api/atlassian/users/:account_id
pub async fn get_user_detail(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
) -> Result<impl IntoResponse, ServiceError> {
    let org_id = state.atlassian_service.resolve_org_id().await?;

    let user = state
        .atlassian_service
        .get_user_details(&org_id, &account_id)
        .await?;

    Ok(Json(user))
}

/// POST /api/atlassian/users (invite new user)
pub async fn invite_user(
    State(state): State<AppState>,
    Json(request): Json<InviteUserRequest>,
) -> Result<impl IntoResponse, ServiceError> {
    let org_id = state.atlassian_service.resolve_org_id().await?;

    let message = state
        .atlassian_service
        .invite_user(&org_id, &request)
        .await?;

    let response = InviteUserResponse {
        success: true,
        message,
        account_id: None,
    };

    Ok(Json(response))
}

/// PUT /api/atlassian/users/:account_id/suspend
pub async fn suspend_user(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
    Json(_request): Json<SuspendUserRequest>,
) -> Result<impl IntoResponse, ServiceError> {
    let org_id = state.atlassian_service.resolve_org_id().await?;

    let message = state
        .atlassian_service
        .suspend_user(&org_id, &account_id)
        .await?;

    let response = SuspendUserResponse {
        success: true,
        message,
    };

    Ok(Json(response))
}

/// DELETE /api/atlassian/users/:account_id
pub async fn delete_user(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
) -> Result<impl IntoResponse, ServiceError> {
    let org_id = state.atlassian_service.resolve_org_id().await?;

    let message = state
        .atlassian_service
        .remove_user(&org_id, &account_id)
        .await?;

    let response = DeleteUserResponse {
        success: true,
        message,
    };

    Ok(Json(response))
}

/// POST /api/atlassian/users/sync (manual sync trigger)
pub async fn sync_users_manual(
    State(state): State<AppState>,
    Json(request): Json<SyncRequest>,
) -> Result<impl IntoResponse, ServiceError> {
    let start = std::time::Instant::now();

    // Get organization ID (from request or auto-detect)
    let org_id = if let Some(org_id) = request.org_id {
        org_id
    } else {
        state.atlassian_service.resolve_org_id().await?
    };

    // Perform sync
    let users_synced = state.atlassian_service.sync_users(&org_id).await?;

    // Re-link persons after cache refresh (store_users resets all links)
    let link_stats = state.link_service.link_all_unlinked().await?;
    let relinked =
        link_stats.linked_by_local_id + link_stats.linked_by_email + link_stats.linked_by_name;
    tracing::info!("Re-linked {} persons after user sync", relinked);

    let duration_ms = start.elapsed().as_millis();

    let response = SyncResponse {
        success: true,
        message: format!("Successfully synced {} users", users_synced),
        users_synced,
        duration_ms,
    };

    Ok(Json(response))
}

/// POST /api/atlassian/link-persons (manually trigger person-to-Atlassian linking)
pub async fn link_persons_manual(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ServiceError> {
    use crate::atlassian::types::LinkResponse;

    let start = std::time::Instant::now();

    tracing::info!("Starting manual Atlassian person linking...");

    // Perform linking
    let stats = state.link_service.link_all_unlinked().await?;

    let duration_ms = start.elapsed().as_millis();
    let total_linked = stats.linked_by_local_id + stats.linked_by_email + stats.linked_by_name;

    let message = format!(
        "Successfully linked {} persons ({} by local_id, {} by email, {} by name)",
        total_linked, stats.linked_by_local_id, stats.linked_by_email, stats.linked_by_name
    );

    let response = LinkResponse {
        success: true,
        message,
        linked_by_local_id: stats.linked_by_local_id,
        linked_by_email: stats.linked_by_email,
        linked_by_name: stats.linked_by_name,
        no_match: stats.no_match,
        errors: stats.errors,
        duration_ms,
    };

    tracing::info!(
        "Linking completed: {} linked, {} no match, {} errors",
        total_linked,
        stats.no_match,
        stats.errors
    );

    Ok(Json(response))
}

/// GET /api/atlassian/sync-status/:sync_type
pub async fn get_sync_status(
    State(state): State<AppState>,
    Path(sync_type): Path<String>,
) -> Result<impl IntoResponse, ServiceError> {
    let status = state.atlassian_service.get_sync_status(&sync_type).await?;

    match status {
        Some(s) => Ok(Json(s)),
        None => Err(ServiceError::NotFound(format!(
            "No sync status found for type: {}",
            sync_type
        ))),
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Generate cache-related response headers
fn cache_headers<T>(response: &CachedResponse<T>) -> HeaderMap {
    let mut headers = HeaderMap::new();

    if response.cache.cached {
        headers.insert("X-Cache-Status", HeaderValue::from_static("HIT"));

        if response.cache.stale.unwrap_or(false) {
            headers.insert("X-Cache-Stale", HeaderValue::from_static("true"));
        }

        if let Some(cached_at) = &response.cache.cached_at {
            if let Ok(val) = HeaderValue::from_str(&cached_at.to_rfc3339()) {
                headers.insert("X-Cache-Cached-At", val);
            }
        }

        if let Some(expires_at) = &response.cache.expires_at {
            if let Ok(val) = HeaderValue::from_str(&expires_at.to_rfc3339()) {
                headers.insert("X-Cache-Expires-At", val);
            }
        }
    } else {
        headers.insert("X-Cache-Status", HeaderValue::from_static("MISS"));
    }

    headers
}
