//! GitHub background sync job (FR-012 / TR-012)
//!
//! Runs a scheduled task to:
//!   1. Sync GitHub Enterprise users and licenses to the local cache.
//!   2. Match unlinked persons to their GitHub accounts.

use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

use crate::github::{sync_github_to_cache, GitHubApiClient};
use crate::github_cache::GitHubCacheRepository;
use crate::github_link::GitHubLinkService;

/// Configuration for the GitHub background sync job.
pub struct GitHubSyncJobConfig {
    /// Initial delay before the first sync attempt (seconds).
    pub initial_delay_secs: u64,
    /// Interval between sync runs (hours).
    pub interval_hours: u64,
}

impl Default for GitHubSyncJobConfig {
    fn default() -> Self {
        Self {
            initial_delay_secs: 120,
            interval_hours: 24,
        }
    }
}

/// Spawn the GitHub sync background task.
///
/// The task runs on the given `interval_hours` schedule.  After each
/// successful cache sync it attempts to link any newly synced users to
/// existing persons.
pub fn start_github_sync_job(
    client: Arc<GitHubApiClient>,
    cache_repo: Arc<GitHubCacheRepository>,
    link_service: Arc<GitHubLinkService>,
    config: GitHubSyncJobConfig,
) {
    tokio::spawn(async move {
        // Wait before the first run so startup I/O doesn't interfere.
        tracing::info!(
            delay_secs = config.initial_delay_secs,
            "GitHub sync job scheduled"
        );
        sleep(Duration::from_secs(config.initial_delay_secs)).await;

        let interval = Duration::from_secs(config.interval_hours * 3600);

        loop {
            tracing::info!("Starting scheduled GitHub sync...");

            match sync_github_to_cache(&client, &cache_repo).await {
                Ok((users, copilot)) => {
                    tracing::info!(
                        users_synced = users,
                        copilot_synced = copilot,
                        "GitHub cache sync completed"
                    );
                }
                Err(e) => {
                    tracing::error!("GitHub cache sync failed: {}", e);
                    continue; // skip linking if sync failed
                }
            }

            // Link newly synced users to existing persons
            tracing::info!("Starting automatic GitHub person-linking after sync...");
            match link_service.link_all_unlinked().await {
                Ok(stats) => {
                    let total = stats.linked_by_person_id
                        + stats.linked_by_local_id
                        + stats.linked_by_email
                        + stats.linked_by_username;
                    tracing::info!(
                        total_linked = total,
                        by_person_id = stats.linked_by_person_id,
                        by_local_id = stats.linked_by_local_id,
                        by_email = stats.linked_by_email,
                        by_username = stats.linked_by_username,
                        no_match = stats.no_match,
                        errors = stats.errors,
                        "GitHub person linking completed"
                    );
                }
                Err(e) => {
                    tracing::error!("GitHub person linking failed: {}", e);
                }
            }

            tracing::info!("Scheduled GitHub sync completed");

            // Wait for the next scheduled run
            sleep(interval).await;
        }
    });
}
