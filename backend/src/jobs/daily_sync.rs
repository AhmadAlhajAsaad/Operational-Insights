//! Daily sync background job
//!
//! Periodically syncs Atlassian data to the local cache and links persons to Atlassian accounts.

use std::sync::Arc;
use std::time::Duration;
use tokio::time::{interval, sleep};

use crate::atlassian::{AtlassianLinkService, AtlassianService};

/// Configuration for the sync job
pub struct SyncJobConfig {
    /// Initial delay before first sync (seconds)
    pub initial_delay_secs: u64,
    /// Interval between syncs (hours)
    pub interval_hours: u64,
    /// Organization ID to sync
    pub org_id: Option<String>,
}

impl Default for SyncJobConfig {
    fn default() -> Self {
        Self {
            initial_delay_secs: 60,
            interval_hours: 24,
            org_id: None,
        }
    }
}

/// Start the background sync job
pub fn start_sync_job(
    service: Arc<AtlassianService>,
    link_service: Arc<AtlassianLinkService>,
    config: SyncJobConfig,
) {
    tokio::spawn(async move {
        // Initial delay
        tracing::info!(
            "Sync job will start in {} seconds",
            config.initial_delay_secs
        );
        sleep(Duration::from_secs(config.initial_delay_secs)).await;

        // Check if we have an org_id configured
        let org_id = match &config.org_id {
            Some(id) => id.clone(),
            None => {
                tracing::info!("No org_id configured, sync job will fetch org on each run");
                String::new()
            }
        };

        // Create interval timer
        let mut interval_timer = interval(Duration::from_secs(config.interval_hours * 3600));

        loop {
            interval_timer.tick().await;

            tracing::info!("Starting scheduled sync...");

            // Get org_id if not configured
            let sync_org_id = if org_id.is_empty() {
                match service.get_organizations().await {
                    Ok(response) => {
                        if let Some(org) = response.data.first() {
                            org.id.clone()
                        } else {
                            tracing::warn!("No organizations found, skipping sync");
                            continue;
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to fetch organizations: {}", e);
                        continue;
                    }
                }
            } else {
                org_id.clone()
            };

            // Sync users
            match service.sync_users(&sync_org_id).await {
                Ok(count) => tracing::info!("Synced {} users", count),
                Err(e) => {
                    tracing::error!("Failed to sync users: {}", e);
                    if let Err(cache_err) = service
                        .cache()
                        .update_sync_error("users", &e.to_string())
                        .await
                    {
                        tracing::error!("Failed to update sync status: {}", cache_err);
                    }
                }
            }

            // Sync groups
            match service.sync_groups(&sync_org_id).await {
                Ok(count) => tracing::info!("Synced {} groups", count),
                Err(e) => {
                    tracing::error!("Failed to sync groups: {}", e);
                    if let Err(cache_err) = service
                        .cache()
                        .update_sync_error("groups", &e.to_string())
                        .await
                    {
                        tracing::error!("Failed to update sync status: {}", cache_err);
                    }
                }
            }

            // Link persons to Atlassian accounts after sync
            tracing::info!("Starting automatic person-to-Atlassian linking...");
            match link_service.link_all_unlinked().await {
                Ok(stats) => {
                    let total_linked =
                        stats.linked_by_local_id + stats.linked_by_email + stats.linked_by_name;
                    tracing::info!(
                        "Linking completed: {} linked ({} by local_id, {} by email, {} by name), {} no match, {} errors",
                        total_linked,
                        stats.linked_by_local_id,
                        stats.linked_by_email,
                        stats.linked_by_name,
                        stats.no_match,
                        stats.errors
                    );
                }
                Err(e) => {
                    tracing::error!("Failed to link persons to Atlassian accounts: {}", e);
                }
            }

            tracing::info!("Scheduled sync completed");
        }
    });
}

/// Run initial sync if cache is empty (blocking)
pub async fn run_initial_sync_if_empty(
    service: &AtlassianService,
    link_service: &AtlassianLinkService,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if cache is empty
    if !service.cache().is_cache_empty().await? {
        tracing::info!("Cache is not empty, skipping initial sync");
        return Ok(());
    }

    tracing::info!("Cache is empty, running initial sync...");

    // Get first org
    let orgs = service.get_organizations().await?;
    let org_id = orgs
        .data
        .first()
        .map(|o| o.id.clone())
        .ok_or("No organizations found")?;

    // Sync users
    match service.sync_users(&org_id).await {
        Ok(count) => tracing::info!("Initial sync: {} users", count),
        Err(e) => tracing::warn!("Initial user sync failed: {}", e),
    }

    // Sync groups
    match service.sync_groups(&org_id).await {
        Ok(count) => tracing::info!("Initial sync: {} groups", count),
        Err(e) => tracing::warn!("Initial group sync failed: {}", e),
    }

    // Link persons to Atlassian accounts
    tracing::info!("Running initial person-to-Atlassian linking...");
    match link_service.link_all_unlinked().await {
        Ok(stats) => {
            let total_linked =
                stats.linked_by_local_id + stats.linked_by_email + stats.linked_by_name;
            tracing::info!(
                "Initial linking: {} linked ({} by local_id, {} by email, {} by name), {} no match",
                total_linked,
                stats.linked_by_local_id,
                stats.linked_by_email,
                stats.linked_by_name,
                stats.no_match
            );
        }
        Err(e) => tracing::warn!("Initial linking failed: {}", e),
    }

    tracing::info!("Initial sync completed");
    Ok(())
}
