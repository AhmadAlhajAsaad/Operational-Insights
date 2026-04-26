//! Atlassian service with caching support
//!
//! Business logic layer that combines API client with cache repository.

use std::sync::Arc;
use tokio::sync::OnceCell;

use super::client::AtlassianClient;
use super::error::{ServiceError, ServiceResult};
use super::types::{
    CacheMetadata, CachedResponse, Group, InviteUserRequest, LicenseCount, LicenseCountDetailed,
    Organization, ProductStats, User, UserListParams, UserListResponse,
};
use crate::cache::CacheRepository;

/// Atlassian service with cache-first strategy
#[derive(Clone)]
pub struct AtlassianService {
    client: AtlassianClient,
    cache: CacheRepository,
    /// In-memory cache for the primary org_id.
    /// Organisation IDs are stable for the lifetime of the process, so a
    /// `OnceCell` (initialise-once) is sufficient and adds zero overhead after
    /// the first request.
    org_id_cache: Arc<OnceCell<String>>,
}

impl AtlassianService {
    /// Create a new service instance
    pub fn new(client: AtlassianClient, cache: CacheRepository) -> Self {
        Self {
            client,
            cache,
            org_id_cache: Arc::new(OnceCell::new()),
        }
    }

    /// Resolve the primary organization ID, using an in-memory cache.
    ///
    /// The first call fetches from the Atlassian API; every subsequent call
    /// returns the value immediately without any I/O.
    pub async fn resolve_org_id(&self) -> ServiceResult<String> {
        self.org_id_cache
            .get_or_try_init(|| async {
                let orgs = self.client.fetch_organizations().await?;
                orgs.into_iter().next().map(|o| o.id).ok_or_else(|| {
                    crate::atlassian::error::AtlassianError::InvalidResponse(
                        "No organizations returned by Atlassian API".to_string(),
                    )
                })
            })
            .await
            .cloned()
            .map_err(ServiceError::Atlassian)
    }

    /// Get the cache repository (for background jobs)
    pub fn cache(&self) -> &CacheRepository {
        &self.cache
    }

    /// Get the API client (for background jobs)
    pub fn client(&self) -> &AtlassianClient {
        &self.client
    }

    // ========================================================================
    // Organizations (not cached - lightweight call)
    // ========================================================================

    /// Fetch organizations (always from API, not cached)
    pub async fn get_organizations(&self) -> ServiceResult<CachedResponse<Vec<Organization>>> {
        let orgs = self.client.fetch_organizations().await?;
        Ok(CachedResponse::fresh(orgs))
    }

    // ========================================================================
    // Users (cached)
    // ========================================================================

    /// Get users with cache-first strategy
    pub async fn get_users(
        &self,
        org_id: &str,
        force_refresh: bool,
    ) -> ServiceResult<CachedResponse<Vec<User>>> {
        // Check cache first (unless force_refresh)
        if !force_refresh {
            if let Ok(Some((users, cached_at, expires_at))) = self.cache.get_cached_users().await {
                tracing::info!("Returning {} users from cache", users.len());
                return Ok(CachedResponse::cached(users, cached_at, expires_at, false));
            }
        }

        // Fetch from API
        match self.client.fetch_users(org_id).await {
            Ok(users) => {
                // Store in cache
                if let Err(e) = self.cache.store_users(&users).await {
                    tracing::warn!("Failed to cache users: {}", e);
                }
                Ok(CachedResponse::fresh(users))
            }
            Err(api_error) => {
                // API failed - try stale cache as fallback
                tracing::warn!("API error, trying stale cache: {}", api_error);

                if let Ok(Some((users, cached_at, expires_at))) = self.cache.get_stale_users().await
                {
                    tracing::info!("Returning {} stale users from cache", users.len());
                    return Ok(CachedResponse::cached(users, cached_at, expires_at, true));
                }

                // No cache available
                Err(ServiceError::Atlassian(api_error))
            }
        }
    }

    // ========================================================================
    // Groups (cached)
    // ========================================================================

    /// Get groups with cache-first strategy
    pub async fn get_groups(
        &self,
        org_id: &str,
        force_refresh: bool,
    ) -> ServiceResult<CachedResponse<Vec<Group>>> {
        // Check cache first (unless force_refresh)
        if !force_refresh {
            if let Ok(Some((groups, cached_at, expires_at))) = self.cache.get_cached_groups().await
            {
                tracing::info!("Returning {} groups from cache", groups.len());
                return Ok(CachedResponse::cached(groups, cached_at, expires_at, false));
            }
        }

        // Fetch from API (with member counts)
        match self.client.fetch_groups_with_counts(org_id).await {
            Ok(groups) => {
                // Store in cache
                if let Err(e) = self.cache.store_groups(&groups).await {
                    tracing::warn!("Failed to cache groups: {}", e);
                }
                Ok(CachedResponse::fresh(groups))
            }
            Err(api_error) => {
                // API failed - try stale cache as fallback
                tracing::warn!("API error, trying stale cache: {}", api_error);

                if let Ok(Some((groups, cached_at, expires_at))) =
                    self.cache.get_stale_groups().await
                {
                    tracing::info!("Returning {} stale groups from cache", groups.len());
                    return Ok(CachedResponse::cached(groups, cached_at, expires_at, true));
                }

                // No cache available
                Err(ServiceError::Atlassian(api_error))
            }
        }
    }

    // ========================================================================
    // Licenses (uses users cache)
    // ========================================================================

    /// Get license count for a product (SQL aggregation, no full user load)
    pub async fn get_license_count(
        &self,
        org_id: &str,
        product: &str,
        force_refresh: bool,
    ) -> ServiceResult<CachedResponse<LicenseCount>> {
        // Force refresh: repopulate cache first
        if force_refresh {
            if let Ok(users) = self.client.fetch_users(org_id).await {
                if let Err(e) = self.cache.store_users(&users).await {
                    tracing::warn!("Failed to cache users after force_refresh: {}", e);
                }
            }
        }

        // Use SQL COUNT instead of loading all users into memory
        if let Some((total, active, cached_at, expires_at)) = self
            .cache
            .count_product_licenses(product)
            .await
            .map_err(ServiceError::Cache)?
        {
            let license_count = LicenseCount {
                product: product.to_string(),
                total_users: total,
                active_users: active,
            };
            return Ok(CachedResponse::cached(
                license_count,
                cached_at,
                expires_at,
                false,
            ));
        }

        // Cache empty - fetch from API and retry
        match self.client.fetch_users(org_id).await {
            Ok(users) => {
                if let Err(e) = self.cache.store_users(&users).await {
                    tracing::warn!("Failed to cache users: {}", e);
                }
            }
            Err(e) => {
                tracing::warn!("Atlassian API unavailable: {}", e);
            }
        }

        // Retry SQL count
        let (total, active, cached_at, expires_at) = self
            .cache
            .count_product_licenses(product)
            .await
            .map_err(ServiceError::Cache)?
            .unwrap_or((0, 0, chrono::Utc::now(), chrono::Utc::now()));

        let license_count = LicenseCount {
            product: product.to_string(),
            total_users: total,
            active_users: active,
        };
        Ok(CachedResponse::cached(
            license_count,
            cached_at,
            expires_at,
            false,
        ))
    }

    /// Get detailed license information for a product
    pub async fn get_license_count_detailed(
        &self,
        org_id: &str,
        product: &str,
        force_refresh: bool,
    ) -> ServiceResult<CachedResponse<LicenseCountDetailed>> {
        // Get users (uses cache)
        let users_response = self.get_users(org_id, force_refresh).await?;
        let users = users_response.data;

        // Calculate detailed license info
        let total_users: Vec<_> = users
            .iter()
            .filter(|u| {
                u.product_access
                    .as_ref()
                    .is_some_and(|pa| pa.iter().any(|p| p.key == product))
            })
            .map(|u| super::types::UserSummary {
                account_id: u.account_id.clone(),
                name: u.name.clone(),
                email: u.email.clone(),
                account_status: u.account_status.clone(),
            })
            .collect();

        let active_users: Vec<_> = total_users
            .iter()
            .filter(|u| u.account_status == "active")
            .cloned()
            .collect();

        let license_details = LicenseCountDetailed {
            product: product.to_string(),
            total_users_count: total_users.len(),
            active_users_count: active_users.len(),
            total_users,
            active_users,
        };

        // Return with same cache metadata as users
        Ok(CachedResponse {
            data: license_details,
            cache: users_response.cache,
        })
    }

    // ========================================================================
    // Sync Operations (for background jobs)
    // ========================================================================

    /// Sync users from API to cache
    pub async fn sync_users(&self, org_id: &str) -> ServiceResult<usize> {
        tracing::info!("Starting user sync for org: {}", org_id);

        let users = self.client.fetch_users(org_id).await?;
        let count = users.len();

        self.cache
            .store_users(&users)
            .await
            .map_err(ServiceError::Cache)?;

        tracing::info!("Successfully synced {} users", count);
        Ok(count)
    }

    /// Sync groups from API to cache
    pub async fn sync_groups(&self, org_id: &str) -> ServiceResult<usize> {
        tracing::info!("Starting group sync for org: {}", org_id);

        let groups = self.client.fetch_groups_with_counts(org_id).await?;
        let count = groups.len();

        self.cache
            .store_groups(&groups)
            .await
            .map_err(ServiceError::Cache)?;

        tracing::info!("Successfully synced {} groups", count);
        Ok(count)
    }

    /// Get sync status for a given sync type (e.g. "users", "groups")
    pub async fn get_sync_status(
        &self,
        sync_type: &str,
    ) -> ServiceResult<Option<super::types::SyncStatus>> {
        self.cache
            .get_sync_status(sync_type)
            .await
            .map_err(ServiceError::Cache)
    }
    // ========================================================================
    // User Management (FR-008)
    // ========================================================================

    /// Get aggregated product statistics from cache (fast, no data download)
    pub async fn get_product_stats(&self) -> ServiceResult<ProductStats> {
        self.cache
            .get_product_stats()
            .await
            .map_err(ServiceError::Cache)
    }

    /// Get filtered and paginated users using SQL-level filtering.
    ///
    /// All heavy lifting (search, product filter, status filter, pagination) is
    /// done inside PostgreSQL so the application never loads the entire user
    /// dataset into memory.  This makes every page-flip and search instant
    /// regardless of how many users are cached.
    pub async fn get_users_filtered(
        &self,
        org_id: &str,
        params: &UserListParams,
    ) -> ServiceResult<UserListResponse> {
        let page = params.page.unwrap_or(1).max(1) as usize;
        let per_page = params.per_page.unwrap_or(50).min(500) as usize;

        // If force_refresh is requested, repopulate the cache first.
        if params.force_refresh {
            match self.client.fetch_users(org_id).await {
                Ok(users) => {
                    if let Err(e) = self.cache.store_users(&users).await {
                        tracing::warn!("Failed to store users after force_refresh: {}", e);
                    }
                }
                Err(e) => {
                    tracing::warn!("force_refresh failed, will use existing cache: {}", e);
                }
            }
        }

        // Fast path: SQL-level filtering + pagination (only fetches the needed page).
        // The `name` parameter is searched against both display_name AND email so that
        // users found by email address or partial name variant are also returned.
        if let Some((users, total, cached_at, expires_at)) = self
            .cache
            .query_users_filtered(
                page,
                per_page,
                params.product.as_deref(),
                params.name.as_deref(),
                params.status.as_deref(),
                params.email.as_deref(),
            )
            .await
            .map_err(ServiceError::Cache)?
        {
            tracing::info!(
                "DB-filtered users: {} total (page {}) per_page={}",
                total,
                page,
                per_page
            );
            return Ok(UserListResponse {
                data: users,
                total: total as usize,
                page,
                per_page,
                cache: CacheMetadata::from_cache(cached_at, expires_at, false),
            });
        }

        // Slow path: cache is genuinely empty (no rows at all) – try a fresh API fetch.
        // NOTE: stale (expired) rows are now served directly by `query_users_filtered`
        // without hitting this path, so this only runs on the very first request after
        // the database is wiped.
        tracing::info!("Cache is empty – fetching users from Atlassian API");
        match self.client.fetch_users(org_id).await {
            Ok(users) => {
                if let Err(e) = self.cache.store_users(&users).await {
                    tracing::warn!("Failed to store fetched users: {}", e);
                }
            }
            Err(e) => {
                tracing::warn!("Atlassian API unavailable, cache remains empty: {}", e);
            }
        }

        // Re-run the DB query now that the cache is populated.
        let (users, total, cached_at, expires_at) = self
            .cache
            .query_users_filtered(
                page,
                per_page,
                params.product.as_deref(),
                params.name.as_deref(),
                params.status.as_deref(),
                params.email.as_deref(),
            )
            .await
            .map_err(ServiceError::Cache)?
            .unwrap_or_else(|| {
                tracing::warn!("Cache still empty after API fetch – returning empty page");
                (vec![], 0, chrono::Utc::now(), chrono::Utc::now())
            });

        Ok(UserListResponse {
            data: users,
            total: total as usize,
            page,
            per_page,
            cache: CacheMetadata::from_cache(cached_at, expires_at, false),
        })
    }

    /// Get details for a specific user
    pub async fn get_user_details(&self, org_id: &str, account_id: &str) -> ServiceResult<User> {
        self.client
            .get_user_by_id(org_id, account_id)
            .await
            .map_err(ServiceError::Atlassian)
    }

    /// Invite a new user to the organization
    pub async fn invite_user(
        &self,
        org_id: &str,
        request: &InviteUserRequest,
    ) -> ServiceResult<String> {
        self.client
            .invite_user(org_id, request)
            .await
            .map_err(ServiceError::Atlassian)
    }

    /// Suspend a user (disable account)
    pub async fn suspend_user(&self, org_id: &str, account_id: &str) -> ServiceResult<String> {
        // Suspend via API
        let result = self
            .client
            .suspend_user(org_id, account_id)
            .await
            .map_err(ServiceError::Atlassian)?;

        // Invalidate cache to force refresh on next request
        if let Err(e) = self.cache.clear_users_cache().await {
            tracing::warn!("Failed to clear cache after suspend: {}", e);
        }

        Ok(result)
    }

    /// Remove a user from the organization
    pub async fn remove_user(&self, org_id: &str, account_id: &str) -> ServiceResult<String> {
        // Remove via API
        let result = self
            .client
            .remove_user(org_id, account_id)
            .await
            .map_err(ServiceError::Atlassian)?;

        // Invalidate cache to force refresh on next request
        if let Err(e) = self.cache.clear_users_cache().await {
            tracing::warn!("Failed to clear cache after removal: {}", e);
        }

        Ok(result)
    }
}
