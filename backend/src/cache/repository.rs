//! Cache repository for PostgreSQL storage
//!
//! Handles all database operations for the Atlassian data cache.

use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;

use crate::atlassian::error::CacheError;
use crate::atlassian::types::{
    CachedGroup, CachedUser, Group, ProductStatEntry, ProductStats, SyncStatus, User,
};

/// Cache repository for Atlassian data
#[derive(Clone)]
pub struct CacheRepository {
    pool: PgPool,
    ttl_hours: i64,
}

impl CacheRepository {
    /// Create a new cache repository
    pub fn new(pool: PgPool, ttl_hours: i64) -> Self {
        Self { pool, ttl_hours }
    }

    /// Get the database pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    // ========================================================================
    // Users Cache
    // ========================================================================

    /// Get cached users if cache is valid
    pub async fn get_cached_users(
        &self,
    ) -> Result<Option<(Vec<User>, DateTime<Utc>, DateTime<Utc>)>, CacheError> {
        let now = Utc::now();

        // Check if we have valid cache
        let cached: Vec<CachedUser> = sqlx::query_as(
            r#"
            SELECT account_id, account_type, email, display_name, active, raw_data, cached_at, expires_at
            FROM atlassian_users_cache
            WHERE expires_at > $1
            ORDER BY display_name
            "#,
        )
        .bind(now)
        .fetch_all(&self.pool)
        .await?;

        if cached.is_empty() {
            return Ok(None);
        }

        // Get cache timestamps from first record
        let cached_at = cached.first().map(|u| u.cached_at).unwrap_or(now);
        let expires_at = cached.first().map(|u| u.expires_at).unwrap_or(now);

        // Convert cached users back to User structs
        let users: Vec<User> = cached
            .into_iter()
            .filter_map(|cu| serde_json::from_value(cu.raw_data).ok())
            .collect();

        Ok(Some((users, cached_at, expires_at)))
    }

    /// Get cached users even if expired (for fallback)
    pub async fn get_stale_users(
        &self,
    ) -> Result<Option<(Vec<User>, DateTime<Utc>, DateTime<Utc>)>, CacheError> {
        let cached: Vec<CachedUser> = sqlx::query_as(
            r#"
            SELECT account_id, account_type, email, display_name, active, raw_data, cached_at, expires_at
            FROM atlassian_users_cache
            ORDER BY display_name
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        if cached.is_empty() {
            return Ok(None);
        }

        let cached_at = cached.first().map(|u| u.cached_at).unwrap_or(Utc::now());
        let expires_at = cached.first().map(|u| u.expires_at).unwrap_or(Utc::now());

        let users: Vec<User> = cached
            .into_iter()
            .filter_map(|cu| serde_json::from_value(cu.raw_data).ok())
            .collect();

        Ok(Some((users, cached_at, expires_at)))
    }

    /// Query users from cache with SQL-level filtering, name/email search and pagination.
    ///
    /// Returns `None` only when the cache is **completely empty** (zero rows), signalling
    /// the caller to populate it first.  Stale (expired) rows are served as-is -- the TTL
    /// is only used by background sync jobs to decide when to refresh, not to block reads.
    /// Otherwise returns `(page_of_users, total_count, cached_at, expires_at)`.
    ///
    /// * `name`         -- searched against **both** `display_name` and `email` (ILIKE)
    /// * `status`       -- exact `account_status` match (e.g. `"active"`)
    /// * `email_filter` -- additional email-only ILIKE filter
    /// * `product`      -- exact product key that must exist in the `product_access` JSONB array
    pub async fn query_users_filtered(
        &self,
        page: usize,
        per_page: usize,
        product: Option<&str>,
        name: Option<&str>,
        status: Option<&str>,
        email_filter: Option<&str>,
    ) -> Result<Option<(Vec<User>, i64, DateTime<Utc>, DateTime<Utc>)>, CacheError> {
        // Fast check: is there at least one row (fresh or stale)?
        // We intentionally do NOT filter by expires_at here -- stale data is still useful
        // and must be served immediately rather than waiting on an API call.
        let cache_check: Option<(DateTime<Utc>, DateTime<Utc>)> =
            sqlx::query_as("SELECT cached_at, expires_at FROM atlassian_users_cache LIMIT 1")
                .fetch_optional(&self.pool)
                .await?;

        let (cached_at, expires_at) = match cache_check {
            Some((ca, ea)) => (ca, ea),
            None => return Ok(None), // genuinely empty -- caller must populate the cache
        };

        // Build lowercased LIKE patterns so they hit the lower(column) expression indexes.
        // NULL means "no filter".
        let name_pattern: Option<String> = name.map(|n| format!("%{}%", n.to_lowercase()));
        let email_pattern: Option<String> = email_filter.map(|e| format!("%{}%", e.to_lowercase()));

        // For the product filter we use the JSONB containment operator (@>) which is
        // fully supported by the jsonb_path_ops GIN index.
        let product_filter: Option<serde_json::Value> =
            product.map(|p| serde_json::json!([{"key": p}]));

        let limit = per_page as i64;
        let offset = ((page - 1) * per_page) as i64;

        // Total matching count (needed for pagination)
        let total: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM atlassian_users_cache
            WHERE ($1::text  IS NULL OR lower(display_name) LIKE $1 OR lower(email) LIKE $1)
              AND ($2::text  IS NULL OR account_status = $2)
              AND ($3::text  IS NULL OR lower(email) LIKE $3)
              AND ($4::jsonb IS NULL OR product_access @> $4)
            "#,
        )
        .bind(name_pattern.as_deref())
        .bind(status)
        .bind(email_pattern.as_deref())
        .bind(&product_filter)
        .fetch_one(&self.pool)
        .await?;

        // Fetch only the requested page
        let rows: Vec<(serde_json::Value,)> = sqlx::query_as(
            r#"
            SELECT raw_data
            FROM atlassian_users_cache
            WHERE ($1::text  IS NULL OR lower(display_name) LIKE $1 OR lower(email) LIKE $1)
              AND ($2::text  IS NULL OR account_status = $2)
              AND ($3::text  IS NULL OR lower(email) LIKE $3)
              AND ($4::jsonb IS NULL OR product_access @> $4)
            ORDER BY display_name
            LIMIT $5 OFFSET $6
            "#,
        )
        .bind(name_pattern.as_deref())
        .bind(status)
        .bind(email_pattern.as_deref())
        .bind(&product_filter)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let users: Vec<User> = rows
            .into_iter()
            .filter_map(|(raw,)| serde_json::from_value(raw).ok())
            .collect();

        Ok(Some((users, total, cached_at, expires_at)))
    }

    /// Extend the TTL on all existing (possibly stale) cached users.
    ///
    /// Called when the Atlassian API is temporarily unavailable so that the
    /// filter queries (which require `expires_at > NOW()`) can still serve the
    /// existing data instead of returning empty pages.
    pub async fn extend_stale_users_ttl(&self) -> Result<(), CacheError> {
        let expires_at = Utc::now() + Duration::hours(self.ttl_hours);
        sqlx::query("UPDATE atlassian_users_cache SET expires_at = $1")
            .bind(expires_at)
            .execute(&self.pool)
            .await?;
        tracing::info!("Extended stale user cache TTL to {}", expires_at);
        Ok(())
    }

    /// Store users in cache
    pub async fn store_users(&self, users: &[User]) -> Result<(), CacheError> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(self.ttl_hours);

        // Start a transaction
        let mut tx = self.pool.begin().await?;

        // Reset linked persons before clearing cache to avoid constraint violation.
        // The ON DELETE SET NULL cascade would null account_id while link_status
        // remains 'linked_*', violating atlassian_link_consistency.
        sqlx::query(
            r#"
            UPDATE persons
            SET atlassian_link_status = 'unlinked',
                atlassian_account_id = NULL,
                atlassian_linked_at = NULL,
                atlassian_link_method = NULL
            WHERE atlassian_account_id IS NOT NULL
            "#,
        )
        .execute(&mut *tx)
        .await?;

        // Clear existing cache
        sqlx::query("DELETE FROM atlassian_group_members_cache")
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM atlassian_users_cache")
            .execute(&mut *tx)
            .await?;

        // Insert new users
        for user in users {
            let account_type = if user.email.contains("@") {
                "atlassian"
            } else {
                "app"
            };

            // Deduplicate product_access by key before storing (both in the dedicated
            // column and in raw_data, so reads always get clean data).
            let unique_products: Vec<_> = {
                use std::collections::HashSet;
                let mut seen = HashSet::new();
                user.product_access
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(|p| seen.insert(p.key.clone()))
                    .collect()
            };

            // Build a cleaned user for raw_data storage
            let clean_user = crate::atlassian::types::User {
                product_access: if unique_products.is_empty() {
                    None
                } else {
                    Some(unique_products.iter().map(|p| (*p).clone()).collect())
                },
                ..user.clone()
            };
            let raw_data = serde_json::to_value(&clean_user)?;
            let product_access =
                serde_json::to_value(&unique_products).unwrap_or(serde_json::json!([]));

            sqlx::query(
                r#"
                INSERT INTO atlassian_users_cache
                    (account_id, account_type, email, display_name, active,
                     account_status, last_active, access_billable, product_access,
                     raw_data, cached_at, expires_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                ON CONFLICT (account_id) DO UPDATE SET
                    account_type = EXCLUDED.account_type,
                    email = EXCLUDED.email,
                    display_name = EXCLUDED.display_name,
                    active = EXCLUDED.active,
                    account_status = EXCLUDED.account_status,
                    last_active = EXCLUDED.last_active,
                    access_billable = EXCLUDED.access_billable,
                    product_access = EXCLUDED.product_access,
                    raw_data = EXCLUDED.raw_data,
                    cached_at = EXCLUDED.cached_at,
                    expires_at = EXCLUDED.expires_at
                "#,
            )
            .bind(&user.account_id)
            .bind(account_type)
            .bind(&user.email)
            .bind(&user.name)
            .bind(user.account_status == "active")
            .bind(&user.account_status)
            .bind(None::<DateTime<Utc>>) // last_active - not provided by API yet
            .bind(user.account_status == "active") // access_billable - infer from active status
            .bind(&product_access)
            .bind(&raw_data)
            .bind(now)
            .bind(expires_at)
            .execute(&mut *tx)
            .await?;
        }

        // Update sync status
        sqlx::query(
            r#"
            UPDATE atlassian_sync_status
            SET last_sync_at = $1, last_success_at = $1, last_error = NULL, items_synced = $2
            WHERE sync_type = 'users'
            "#,
        )
        .bind(now)
        .bind(users.len() as i32)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        tracing::info!("Cached {} users, expires at {}", users.len(), expires_at);
        Ok(())
    }

    // ========================================================================
    // Groups Cache
    // ========================================================================

    /// Get cached groups if cache is valid
    pub async fn get_cached_groups(
        &self,
    ) -> Result<Option<(Vec<Group>, DateTime<Utc>, DateTime<Utc>)>, CacheError> {
        let now = Utc::now();

        let cached: Vec<CachedGroup> = sqlx::query_as(
            r#"
            SELECT group_id, name, raw_data, cached_at, expires_at
            FROM atlassian_groups_cache
            WHERE expires_at > $1
            ORDER BY name
            "#,
        )
        .bind(now)
        .fetch_all(&self.pool)
        .await?;

        if cached.is_empty() {
            return Ok(None);
        }

        let cached_at = cached.first().map(|g| g.cached_at).unwrap_or(now);
        let expires_at = cached.first().map(|g| g.expires_at).unwrap_or(now);

        let groups: Vec<Group> = cached
            .into_iter()
            .filter_map(|cg| serde_json::from_value(cg.raw_data).ok())
            .collect();

        Ok(Some((groups, cached_at, expires_at)))
    }

    /// Get cached groups even if expired (for fallback)
    pub async fn get_stale_groups(
        &self,
    ) -> Result<Option<(Vec<Group>, DateTime<Utc>, DateTime<Utc>)>, CacheError> {
        let cached: Vec<CachedGroup> = sqlx::query_as(
            r#"
            SELECT group_id, name, raw_data, cached_at, expires_at
            FROM atlassian_groups_cache
            ORDER BY name
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        if cached.is_empty() {
            return Ok(None);
        }

        let cached_at = cached.first().map(|g| g.cached_at).unwrap_or(Utc::now());
        let expires_at = cached.first().map(|g| g.expires_at).unwrap_or(Utc::now());

        let groups: Vec<Group> = cached
            .into_iter()
            .filter_map(|cg| serde_json::from_value(cg.raw_data).ok())
            .collect();

        Ok(Some((groups, cached_at, expires_at)))
    }

    /// Store groups in cache
    pub async fn store_groups(&self, groups: &[Group]) -> Result<(), CacheError> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(self.ttl_hours);

        let mut tx = self.pool.begin().await?;

        // Clear existing groups (members are cascade deleted)
        sqlx::query("DELETE FROM atlassian_groups_cache")
            .execute(&mut *tx)
            .await?;

        // Insert new groups
        for group in groups {
            let raw_data = serde_json::to_value(group)?;

            sqlx::query(
                r#"
                INSERT INTO atlassian_groups_cache
                    (group_id, name, raw_data, cached_at, expires_at)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (group_id) DO UPDATE SET
                    name = EXCLUDED.name,
                    raw_data = EXCLUDED.raw_data,
                    cached_at = EXCLUDED.cached_at,
                    expires_at = EXCLUDED.expires_at
                "#,
            )
            .bind(&group.id)
            .bind(&group.name)
            .bind(&raw_data)
            .bind(now)
            .bind(expires_at)
            .execute(&mut *tx)
            .await?;
        }

        // Update sync status
        sqlx::query(
            r#"
            UPDATE atlassian_sync_status
            SET last_sync_at = $1, last_success_at = $1, last_error = NULL, items_synced = $2
            WHERE sync_type = 'groups'
            "#,
        )
        .bind(now)
        .bind(groups.len() as i32)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        tracing::info!("Cached {} groups, expires at {}", groups.len(), expires_at);
        Ok(())
    }

    // ========================================================================
    // Sync Status
    // ========================================================================

    /// Get sync status for a sync type
    pub async fn get_sync_status(&self, sync_type: &str) -> Result<Option<SyncStatus>, CacheError> {
        let status: Option<SyncStatus> = sqlx::query_as(
            r#"
            SELECT sync_type, last_sync_at, last_success_at, last_error, items_synced
            FROM atlassian_sync_status
            WHERE sync_type = $1
            "#,
        )
        .bind(sync_type)
        .fetch_optional(&self.pool)
        .await?;

        Ok(status)
    }

    /// Update sync status on error
    pub async fn update_sync_error(&self, sync_type: &str, error: &str) -> Result<(), CacheError> {
        sqlx::query(
            r#"
            UPDATE atlassian_sync_status
            SET last_sync_at = $1, last_error = $2
            WHERE sync_type = $3
            "#,
        )
        .bind(Utc::now())
        .bind(error)
        .bind(sync_type)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get aggregated product statistics directly from the cache (fast SQL, no data transfer)
    pub async fn get_product_stats(&self) -> Result<ProductStats, CacheError> {
        // Overall user totals
        let totals: (i64, i64) = sqlx::query_as(
            r#"
            SELECT
                COUNT(*) AS total_users,
                COUNT(*) FILTER (WHERE account_status = 'active') AS active_users
            FROM atlassian_users_cache
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        // Per-product counts via jsonb_array_elements
        let per_product: Vec<(String, i64, i64)> = sqlx::query_as(
            r#"
            SELECT
                product_elem->>'key' AS product_key,
                COUNT(*) AS total_count,
                COUNT(*) FILTER (WHERE account_status = 'active') AS active_count
            FROM atlassian_users_cache
            CROSS JOIN LATERAL jsonb_array_elements(
                COALESCE(product_access, '[]'::jsonb)
            ) AS product_elem
            WHERE product_elem->>'key' IS NOT NULL
            GROUP BY product_elem->>'key'
            ORDER BY total_count DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(ProductStats {
            total_users: totals.0,
            active_users: totals.1,
            per_product: per_product
                .into_iter()
                .map(|(key, total, active)| ProductStatEntry {
                    product_key: key,
                    total_count: total,
                    active_count: active,
                })
                .collect(),
        })
    }

    /// Check if cache is empty (for initial sync)
    pub async fn is_cache_empty(&self) -> Result<bool, CacheError> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM atlassian_users_cache")
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0 == 0)
    }

    /// Count licenses for a product using SQL aggregation (avoids loading all users)
    pub async fn count_product_licenses(
        &self,
        product: &str,
    ) -> Result<Option<(usize, usize, DateTime<Utc>, DateTime<Utc>)>, CacheError> {
        let cache_check: Option<(DateTime<Utc>, DateTime<Utc>)> =
            sqlx::query_as("SELECT cached_at, expires_at FROM atlassian_users_cache LIMIT 1")
                .fetch_optional(&self.pool)
                .await?;

        let (cached_at, expires_at) = match cache_check {
            Some(row) => row,
            None => return Ok(None),
        };

        let product_filter = serde_json::json!([{"key": product}]);
        let counts: (i64, i64) = sqlx::query_as(
            r#"
            SELECT
                COUNT(*) FILTER (WHERE product_access @> $1) AS total,
                COUNT(*) FILTER (WHERE product_access @> $1 AND account_status = 'active') AS active
            FROM atlassian_users_cache
            "#,
        )
        .bind(&product_filter)
        .fetch_one(&self.pool)
        .await?;

        Ok(Some((
            counts.0 as usize,
            counts.1 as usize,
            cached_at,
            expires_at,
        )))
    }

    // ========================================================================
    // Cache Invalidation (for User Management)
    // ========================================================================

    /// Clear users cache to force refresh on next request
    pub async fn clear_users_cache(&self) -> Result<(), CacheError> {
        tracing::info!("Clearing users cache");

        sqlx::query("DELETE FROM atlassian_users_cache")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Clear groups cache to force refresh on next request
    pub async fn clear_groups_cache(&self) -> Result<(), CacheError> {
        tracing::info!("Clearing groups cache");

        sqlx::query("DELETE FROM atlassian_groups_cache")
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
