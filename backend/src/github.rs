//! GitHub Enterprise API integration (FR-011 / TR-011 / FR-012 / TR-012)
//!
//! Authentication: Personal Access Token (PAT) via Authorization: Bearer header.
//! Rate limiting: exponential backoff on 429, respects Retry-After header.
//! Security: PAT is NEVER logged, never sent to frontend.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc};
use thiserror::Error;

use crate::github_cache::GitHubCacheRepository;
use crate::github_link::GitHubLinkService;

// ============================================================================
// State (Axum dependency injection)
// ============================================================================

/// Shared state for all GitHub route handlers (FR-011 + FR-012).
#[derive(Clone)]
pub struct GitHubState {
    pub client: Arc<GitHubApiClient>,
    /// Database pool <-> used by FR-012 link and sync handlers.
    pub pool: sqlx::PgPool,
    /// Cache repository for the github_*_cache tables.
    pub cache_repo: Arc<GitHubCacheRepository>,
    /// Link service for person <-> GitHub account matching.
    pub link_service: Arc<GitHubLinkService>,
}

// ============================================================================
// Error Types (TR-011 s.3.1)
// ============================================================================

#[derive(Debug, Error)]
pub enum GitHubApiError {
    #[error("HTTP request failed: {0}")]
    HttpRequest(reqwest::Error),

    #[error("API error {status}: {body}")]
    ApiError {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("Failed to parse response: {0}")]
    ResponseParse(reqwest::Error),

    #[error("Unauthorized  check PAT token validity and scopes")]
    Unauthorized,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Max retries exceeded")]
    MaxRetriesExceeded,
}

// ============================================================================
// GitHub API Data Structures
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseConsumption {
    pub total_seats_consumed: u32,
    pub total_seats_purchased: u32,
    #[serde(default)]
    pub users: Vec<GitHubUser>,
}

// ============================================================================
// FR-012 / TR-012: Consumed-licenses response (actual GitHub API field names)
// ============================================================================

/// One user entry from `GET /enterprises/{enterprise}/consumed-licenses`.
/// Field names match the actual GitHub Enterprise API JSON keys.
#[derive(Debug, Deserialize, Clone)]
pub struct ConsumedLicenseEntry {
    pub github_com_login: Option<String>,
    pub github_com_name: Option<String>,
    /// Primary email used by Atlassian; equals the persons.local_id in most cases.
    pub github_com_saml_name_id: Option<String>,
    /// Company-domain emails verified by GitHub.
    #[serde(default)]
    pub github_com_verified_domain_emails: Vec<String>,
    pub github_com_enterprise_role: Option<String>,
    #[serde(default)]
    pub github_com_enterprise_teams: Vec<String>,
}

/// Full response from `GET /enterprises/{enterprise}/consumed-licenses`.
#[derive(Debug, Deserialize)]
pub struct ConsumedLicensesResponse {
    pub total_seats_consumed: u32,
    pub total_seats_purchased: u32,
    #[serde(default)]
    pub users: Vec<ConsumedLicenseEntry>,
}

/// Sync status response type (GET /admin/sync/github/status).
#[derive(Debug, Serialize)]
pub struct GitHubSyncStatus {
    pub enterprise_slug: String,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub github_api_reachable: bool,
}

/// Response type for a manual sync trigger (POST /admin/sync/github).
#[derive(Debug, Serialize)]
pub struct GitHubSyncResponse {
    pub success: bool,
    pub message: String,
    pub users_synced: usize,
    pub copilot_seats_synced: usize,
    pub persons_linked: u32,
    pub duration_ms: u128,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CopilotSeatAssignee {
    pub login: Option<String>,
    pub id: Option<u64>,
    pub email: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CopilotSeat {
    pub assignee: Option<CopilotSeatAssignee>,
    pub plan_type: Option<String>,
    pub pending_cancellation_date: Option<String>,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub last_activity_editor: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopilotBillingSeatsResponse {
    pub total_seats: u32,
    pub seats: Option<Vec<CopilotSeat>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GHASCommitter {
    pub user_login: Option<String>,
    pub last_pushed_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GHASRepository {
    pub name: String,
    pub advanced_security_committers: Option<u32>,
    #[serde(default)]
    pub advanced_security_committers_breakdown: Option<Vec<GHASCommitter>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GHASBillingResponse {
    pub total_advanced_security_committers: Option<u32>,
    pub total_count: Option<u32>,
    #[serde(default)]
    pub repositories: Vec<GHASRepository>,
}

// ============================================================================
// Frontend Response Types
// ============================================================================

/// Aggregated overview for GitHub vendor dashboard (FR-011)
#[derive(Debug, Serialize)]
pub struct GitHubOverview {
    pub enterprise_slug: String,
    pub connected: bool,
    pub copilot_active_seats: u32,
    pub copilot_total_seats: u32,
    pub ghas_total_committers: u32,
    pub license_seats_consumed: u32,
    pub license_seats_purchased: u32,
    pub license_seats_available: u32,
}

#[derive(Debug, Serialize)]
pub struct CopilotSeatItem {
    pub login: String,
    pub email: Option<String>,
    pub name: Option<String>,
    /// "active" = seat assigned, no pending cancellation.
    /// "pending_cancellation" = seat cancelling (still billable).
    /// "inactive" = no activity in 90 days, seat still assigned.
    pub status: String,
    pub has_pending_cancellation: bool,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub plan_type: Option<String>,
    pub person_id: Option<String>,
    pub person_name: Option<String>,
    pub org_id: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedCopilotSeats {
    pub data: Vec<CopilotSeatItem>,
    pub total: usize,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Serialize)]
pub struct GHASUserItem {
    pub login: String,
    pub repository: String,
    pub last_pushed_date: Option<String>,
    pub status: String,
    pub person_id: Option<String>,
    pub person_name: Option<String>,
    pub org_id: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedGHASUsers {
    pub data: Vec<GHASUserItem>,
    pub total: usize,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Serialize)]
pub struct LicenseUserItem {
    pub login: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub status: String,
    pub person_id: Option<String>,
    pub person_name: Option<String>,
    pub org_id: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedLicenseUsers {
    pub data: Vec<LicenseUserItem>,
    pub total: usize,
    pub page: u32,
    pub per_page: u32,
}

// ============================================================================
// Query Params
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct UserListQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search: Option<String>,
    pub status: Option<String>,
}

// ============================================================================
// Internal error response
// ============================================================================

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn make_error(status: StatusCode, message: impl Into<String>) -> Response {
    (
        status,
        Json(ErrorResponse {
            error: message.into(),
        }),
    )
        .into_response()
}

// ============================================================================
// API Client (TR-011 s.3.1)
// ============================================================================

const API_BASE: &str = "https://api.github.com";
const MAX_RETRIES: u32 = 3;

pub struct GitHubApiClient {
    client: Client,
    pat_token: String,
    pub enterprise_slug: String,
}

impl GitHubApiClient {
    #[allow(clippy::expect_used)]
    pub fn new(pat_token: String, enterprise_slug: String) -> Self {
        let client = reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .min_tls_version(reqwest::tls::Version::TLS_1_2)
            .https_only(true)
            .user_agent("Equans-Operational-Insights/1.0")
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            pat_token,
            enterprise_slug,
        }
    }

    /// GET with exponential backoff on rate limits (TR-011 s.4.3)
    /// NEVER logs the PAT token (TR-011 s.3.2 / s.7.2)
    async fn request_with_retry<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T, GitHubApiError> {
        let mut retries = 0u32;

        loop {
            let response = self
                .client
                .get(url)
                .bearer_auth(&self.pat_token)
                .header("Accept", "application/vnd.github+json")
                .header("X-GitHub-Api-Version", "2022-11-28")
                .send()
                .await
                .map_err(GitHubApiError::HttpRequest)?;

            // Log rate-limit status (TR-011 s.4.2 / s.7.1)  no token in log
            if let Some(remaining) = response
                .headers()
                .get("X-RateLimit-Remaining")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u32>().ok())
            {
                if remaining < 10 {
                    tracing::warn!(
                        "GitHub API rate limit low: {} requests remaining",
                        remaining
                    );
                } else {
                    tracing::debug!("GitHub API rate limit remaining: {}", remaining);
                }
            }

            match response.status().as_u16() {
                200..=299 => {
                    tracing::debug!("GitHub API success for endpoint");
                    return response.json().await.map_err(GitHubApiError::ResponseParse);
                }

                401 => {
                    // NEVER log token (TR-011 s.7.2)
                    tracing::error!(
                        "GitHub API 401 Unauthorized  check PAT token validity and scopes"
                    );
                    return Err(GitHubApiError::Unauthorized);
                }
                429 => {
                    let retry_after = response
                        .headers()
                        .get("Retry-After")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse::<u64>().ok())
                        .unwrap_or_else(|| 2u64.pow(retries));

                    tracing::warn!(
                        "GitHub API rate limit hit, retrying in {}s ({}/{})",
                        retry_after,
                        retries + 1,
                        MAX_RETRIES
                    );

                    if retries >= MAX_RETRIES {
                        tracing::error!(
                            "GitHub API max retries exceeded after {} attempts",
                            MAX_RETRIES
                        );
                        return Err(GitHubApiError::MaxRetriesExceeded);
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(retry_after)).await;
                }

                403 | 404 => {
                    let body = response.text().await.unwrap_or_default();
                    tracing::warn!("GitHub API 404/403: {} (not retrying)", body);
                    return Err(GitHubApiError::NotFound(body));
                }

                status => {
                    let body = response.text().await.unwrap_or_default();
                    // Log status + sanitised body, never the PAT (TR-011 s.7.1)
                    tracing::error!("GitHub API error {}: {}", status, body);

                    if retries >= MAX_RETRIES {
                        tracing::error!(
                            "GitHub API max retries exceeded after {} attempts",
                            MAX_RETRIES
                        );
                        return Err(GitHubApiError::ApiError {
                            status: reqwest::StatusCode::from_u16(status)
                                .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
                            body,
                        });
                    }
                    let backoff = std::time::Duration::from_secs(2u64.pow(retries));
                    tokio::time::sleep(backoff).await;
                }
            }

            retries += 1;
        }
    }

    pub async fn validate_token(&self) -> Result<GitHubUser, GitHubApiError> {
        let url = format!("{}/user", API_BASE);
        tracing::debug!("Validating GitHub PAT token");
        self.request_with_retry(&url).await
    }

    // -------------------------------------------------------------------------
    // Pagination helpers
    // -------------------------------------------------------------------------

    /// Parse the `Link` response header and return the URL of the `next` page,
    /// or `None` when the last page has been reached.
    ///
    /// GitHub format:
    ///   `Link: <https://api.github.com/...?page=2>; rel="next", <...>; rel="last"`
    fn parse_next_link(headers: &reqwest::header::HeaderMap) -> Option<String> {
        let link_header = headers.get("Link")?.to_str().ok()?;
        // Split on `,` to get individual link entries, then find rel="next"
        for part in link_header.split(',') {
            let mut url: Option<&str> = None;
            let mut is_next = false;
            for segment in part.trim().split(';') {
                let segment = segment.trim();
                if segment.starts_with('<') && segment.ends_with('>') {
                    url = Some(&segment[1..segment.len() - 1]);
                } else if segment
                    .trim_matches('"')
                    .eq_ignore_ascii_case("rel=\"next\"")
                    || segment == "rel=\"next\""
                {
                    is_next = true;
                }
            }
            if is_next {
                if let Some(u) = url {
                    return Some(u.to_string());
                }
            }
        }
        None
    }

    /// Like `request_with_retry` but returns both the parsed body **and** the
    /// raw response headers so callers can read the `Link` header for pagination.
    async fn request_page_with_retry<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<(T, reqwest::header::HeaderMap), GitHubApiError> {
        let mut retries = 0u32;

        loop {
            let response = self
                .client
                .get(url)
                .bearer_auth(&self.pat_token)
                .header("Accept", "application/vnd.github+json")
                .header("X-GitHub-Api-Version", "2022-11-28")
                .send()
                .await
                .map_err(GitHubApiError::HttpRequest)?;

            // Log rate-limit status
            if let Some(remaining) = response
                .headers()
                .get("X-RateLimit-Remaining")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u32>().ok())
            {
                if remaining < 10 {
                    tracing::warn!(
                        "GitHub API rate limit low: {} requests remaining",
                        remaining
                    );
                }
            }

            match response.status().as_u16() {
                200..=299 => {
                    let headers = response.headers().clone();
                    let body: T = response
                        .json()
                        .await
                        .map_err(GitHubApiError::ResponseParse)?;
                    return Ok((body, headers));
                }
                401 => {
                    tracing::error!(
                        "GitHub API 401 Unauthorized <-> check PAT token validity and scopes"
                    );
                    return Err(GitHubApiError::Unauthorized);
                }
                429 => {
                    let retry_after = response
                        .headers()
                        .get("Retry-After")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse::<u64>().ok())
                        .unwrap_or_else(|| 2u64.pow(retries));

                    if retries >= MAX_RETRIES {
                        return Err(GitHubApiError::MaxRetriesExceeded);
                    }
                    tracing::warn!(
                        "GitHub API rate limit, retrying in {}s ({}/{})",
                        retry_after,
                        retries + 1,
                        MAX_RETRIES
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(retry_after)).await;
                }

                status => {
                    let body = response.text().await.unwrap_or_default();
                    tracing::error!("GitHub API error {}: {}", status, body);
                    if retries >= MAX_RETRIES {
                        return Err(GitHubApiError::ApiError {
                            status: reqwest::StatusCode::from_u16(status)
                                .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
                            body,
                        });
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(2u64.pow(retries))).await;
                }
            }

            retries += 1;
        }
    }

    // -------------------------------------------------------------------------
    // Paginated fetch methods
    // -------------------------------------------------------------------------

    pub async fn fetch_license_consumption(&self) -> Result<LicenseConsumption, GitHubApiError> {
        tracing::debug!("Fetching license consumption (all pages) for enterprise");

        let mut all_users: Vec<GitHubUser> = Vec::new();
        let mut total_consumed = 0u32;
        let mut total_purchased = 0u32;
        let mut page = 1u32;

        loop {
            let url = format!(
                "{}/enterprises/{}/consumed-licenses?per_page=100&page={}",
                API_BASE, self.enterprise_slug, page
            );
            tracing::debug!("Fetching license consumption page {}", page);

            let (page_data, _headers): (LicenseConsumption, _) =
                self.request_page_with_retry(&url).await?;

            if page == 1 {
                total_consumed = page_data.total_seats_consumed;
                total_purchased = page_data.total_seats_purchased;
            }

            let count = page_data.users.len();
            tracing::debug!("License page {}: {} users", page, count);
            all_users.extend(page_data.users);

            // Stop when we have all users the API reported, or on empty page.
            if all_users.len() as u32 >= total_consumed || count == 0 {
                break;
            }
            page += 1;
            if page > 100 {
                tracing::warn!("License pagination safety cap reached at page 100");
                break;
            }
        }

        tracing::info!(
            pages = page,
            total_users = all_users.len(),
            total_consumed,
            total_purchased,
            "License consumption fetch complete"
        );

        Ok(LicenseConsumption {
            total_seats_consumed: total_consumed,
            total_seats_purchased: total_purchased,
            users: all_users,
        })
    }

    /// Fetch **all pages** of `GET /enterprises/{enterprise}/copilot/billing/seats`.
    ///
    /// Uses the `total_seats` field from the first page as the authoritative
    /// stopping condition so we never under-fetch even when the API silently
    /// caps `per_page` to a smaller value than what we requested.
    pub async fn fetch_copilot_seats(&self) -> Result<CopilotBillingSeatsResponse, GitHubApiError> {
        tracing::debug!("Fetching Copilot seats (all pages) for enterprise");

        let mut all_seats: Vec<CopilotSeat> = Vec::new();
        let mut total_seats = 0u32;
        let mut page = 1u32;

        loop {
            let url = format!(
                "{}/enterprises/{}/copilot/billing/seats?per_page=100&page={}",
                API_BASE, self.enterprise_slug, page
            );
            tracing::debug!("Fetching Copilot seats page {}", page);

            let (page_data, _headers): (CopilotBillingSeatsResponse, _) =
                self.request_page_with_retry(&url).await?;

            if page == 1 {
                total_seats = page_data.total_seats;
            }

            let seats = page_data.seats.unwrap_or_default();
            let count = seats.len();
            tracing::debug!("Copilot page {}: {} seats", page, count);
            all_seats.extend(seats);

            // Stop when we have collected all seats the API reports,
            // or when a page returns 0 items (safety net).
            if all_seats.len() as u32 >= total_seats || count == 0 {
                break;
            }
            page += 1;
            if page > 100 {
                tracing::warn!("Copilot seats pagination safety cap reached at page 100");
                break;
            }
        }
        tracing::info!(
            pages = page,
            total_seats,
            actual_seats = all_seats.len(),
            "Copilot seats fetch complete"
        );

        Ok(CopilotBillingSeatsResponse {
            total_seats,
            seats: Some(all_seats),
        })
    }

    pub async fn fetch_ghas_usage(&self) -> Result<GHASBillingResponse, GitHubApiError> {
        let url = format!(
            "{}/enterprises/{}/settings/billing/advanced-security",
            API_BASE, self.enterprise_slug
        );
        tracing::debug!("Fetching GHAS billing usage for enterprise");
        self.request_with_retry(&url).await
    }

    /// Fetch **all pages** of `GET /enterprises/{enterprise}/consumed-licenses`
    /// using the detailed field names from the real GitHub Enterprise API.
    /// Used by the sync-to-cache job (FR-012 / TR-012).
    pub async fn fetch_consumed_licenses_detailed(
        &self,
    ) -> Result<ConsumedLicensesResponse, GitHubApiError> {
        tracing::debug!(
            "Fetching detailed consumed-licenses (all pages) for enterprise cache sync"
        );

        let mut all_users: Vec<ConsumedLicenseEntry> = Vec::new();
        let mut total_consumed = 0u32;
        let mut total_purchased = 0u32;
        let mut page = 1u32;

        loop {
            let url = format!(
                "{}/enterprises/{}/consumed-licenses?per_page=100&page={}",
                API_BASE, self.enterprise_slug, page
            );
            tracing::debug!("Fetching detailed licenses page {}", page);

            let (page_data, _headers): (ConsumedLicensesResponse, _) =
                self.request_page_with_retry(&url).await?;

            if page == 1 {
                total_consumed = page_data.total_seats_consumed;
                total_purchased = page_data.total_seats_purchased;
            }

            let count = page_data.users.len();
            tracing::debug!("Detailed licenses page {}: {} users", page, count);
            all_users.extend(page_data.users);

            // Stop when we have all users the API reported, or on empty page.
            if all_users.len() as u32 >= total_consumed || count == 0 {
                break;
            }
            page += 1;
            if page > 100 {
                tracing::warn!("License detailed pagination safety cap reached at page 100");
                break;
            }
        }

        tracing::info!(
            pages = page,
            total_users = all_users.len(),
            total_consumed,
            total_purchased,
            "Detailed license consumption fetch complete"
        );

        Ok(ConsumedLicensesResponse {
            total_seats_consumed: total_consumed,
            total_seats_purchased: total_purchased,
            users: all_users,
        })
    }
}

// ============================================================================
// FR-012 / TR-012: Sync Service helpers
// ============================================================================

/// Synchronise Enterprise users and license counts from the GitHub API into the
/// local cache tables.  Returns the number of user rows upserted.
///
/// Order: users first, then copilot seats (due to the FK dependency).
pub async fn sync_github_to_cache(
    client: &GitHubApiClient,
    cache_repo: &GitHubCacheRepository,
) -> Result<(usize, usize), String> {
    use crate::github_cache::GitHubCachedCopilotSeat;
    use crate::github_cache::GitHubCachedUser;

    // Users + License snapshot
    let license_data = client
        .fetch_consumed_licenses_detailed()
        .await
        .map_err(|e| format!("Failed to fetch consumed-licenses: {}", e))?;

    // Save the aggregate license snapshot
    cache_repo
        .save_license_snapshot(
            &client.enterprise_slug,
            Some(license_data.total_seats_purchased as i32),
            Some(license_data.total_seats_consumed as i32),
            None, // GHAS seats: from a separate endpoint (see GHAS billing)
        )
        .await
        .map_err(|e| format!("Failed to save license snapshot: {}", e))?;

    // Upsert each user
    let mut active_logins: Vec<String> = Vec::new();
    let mut users_synced = 0usize;

    for entry in &license_data.users {
        let login = match entry.github_com_login.as_deref() {
            Some(l) if !l.is_empty() => l.to_string(),
            _ => continue,
        };

        // Use the login as the stable ID since the endpoint doesn't return a numeric ID
        let id = login.clone();

        // Prefer verified domain email first, fall back to SAML name ID
        let email = entry
            .github_com_verified_domain_emails
            .first()
            .cloned()
            .or_else(|| entry.github_com_saml_name_id.clone());

        // Store all verified domain emails for fallback matching (FR-012 steps 2-3)
        let verified_domain_emails = if entry.github_com_verified_domain_emails.is_empty() {
            None
        } else {
            Some(entry.github_com_verified_domain_emails.clone())
        };

        let user = GitHubCachedUser {
            id,
            login: login.clone(),
            email,
            name: entry.github_com_name.clone(),
            enterprise_role: entry.github_com_enterprise_role.clone(),
            organization_name: None, // not available from this endpoint
            team_names: if entry.github_com_enterprise_teams.is_empty() {
                None
            } else {
                Some(entry.github_com_enterprise_teams.clone())
            },
            verified_domain_emails,
            is_active: true,
            synced_at: Utc::now(),
        };

        cache_repo
            .upsert_user(&user)
            .await
            .map_err(|e| format!("Failed to upsert user {}: {}", login, e))?;

        active_logins.push(login);
        users_synced += 1;
    }

    // Deactivate users who disappeared from this sync run
    let deactivated = cache_repo
        .deactivate_missing_users(&active_logins)
        .await
        .map_err(|e| format!("Failed to deactivate missing users: {}", e))?;

    if deactivated > 0 {
        tracing::info!(deactivated, "Marked GitHub users as inactive after sync");
    }

    // Copilot seats
    let copilot_data = client
        .fetch_copilot_seats()
        .await
        .map_err(|e| format!("Failed to fetch Copilot seats: {}", e))?;

    let seats = copilot_data.seats.unwrap_or_default();
    let mut active_copilot_logins: Vec<String> = Vec::new();
    let mut copilot_synced = 0usize;

    for seat in &seats {
        let login = match seat.assignee.as_ref().and_then(|a| a.login.as_deref()) {
            Some(l) if !l.is_empty() => l.to_string(),
            _ => continue,
        };

        let cached_seat = GitHubCachedCopilotSeat {
            github_login: login.clone(),
            seat_type: seat.plan_type.clone(),
            is_active: seat.pending_cancellation_date.is_none(),
            last_activity_at: seat.last_activity_at,
            last_activity_editor: seat.last_activity_editor.clone(),
            assigning_team: None,
            created_at: seat.created_at,
            updated_at: seat.updated_at,
        };

        cache_repo
            .upsert_copilot_seat(&cached_seat)
            .await
            .map_err(|e| format!("Failed to upsert Copilot seat for {}: {}", login, e))?;

        active_copilot_logins.push(login);
        copilot_synced += 1;
    }

    // Deactivate seats that were not in this sync run
    cache_repo
        .deactivate_missing_copilot_seats(&active_copilot_logins)
        .await
        .map_err(|e| format!("Failed to deactivate missing Copilot seats: {}", e))?;

    tracing::info!(users_synced, copilot_synced, "GitHub cache sync completed");

    Ok((users_synced, copilot_synced))
}

// ============================================================================
// HTTP Handlers
// ============================================================================

/// GET /api/github/validate  Validate PAT token (TR-011 acceptatiecriteria)
pub async fn validate_token(State(state): State<GitHubState>) -> Response {
    match state.client.validate_token().await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(GitHubApiError::Unauthorized) => make_error(
            StatusCode::UNAUTHORIZED,
            "GitHub PAT token is invalid or expired  update GITHUB_PAT_TOKEN",
        ),
        Err(e) => {
            tracing::error!("GitHub token validation failed: {}", e);
            make_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}

/// GET /api/github/overview  Aggregated stats for vendor dashboard (FR-011)
pub async fn get_overview(State(state): State<GitHubState>) -> Response {
    // Fetch all three products in parallel
    let (licenses_res, copilot_res, ghas_res) = tokio::join!(
        state.client.fetch_license_consumption(),
        state.client.fetch_copilot_seats(),
        state.client.fetch_ghas_usage(),
    );

    // A 401 on any endpoint means the PAT is invalid  return immediately
    if matches!(&licenses_res, Err(GitHubApiError::Unauthorized))
        || matches!(&copilot_res, Err(GitHubApiError::Unauthorized))
        || matches!(&ghas_res, Err(GitHubApiError::Unauthorized))
    {
        return make_error(
            StatusCode::UNAUTHORIZED,
            "GitHub PAT token is invalid  update GITHUB_PAT_TOKEN",
        );
    }

    let license_consumed = licenses_res
        .as_ref()
        .map(|l| l.total_seats_consumed)
        .unwrap_or(0);
    let license_purchased = licenses_res
        .as_ref()
        .map(|l| l.total_seats_purchased)
        .unwrap_or(0);

    let (copilot_total, copilot_active) = if let Ok(c) = &copilot_res {
        let active = c
            .seats
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter(|s| s.pending_cancellation_date.is_none())
            .count() as u32;
        (c.total_seats, active)
    } else {
        (0, 0)
    };

    let ghas_committers = ghas_res
        .as_ref()
        .map(|g| g.total_advanced_security_committers.unwrap_or(0))
        .unwrap_or(0);

    let connected = licenses_res.is_ok() || copilot_res.is_ok() || ghas_res.is_ok();

    (
        StatusCode::OK,
        Json(GitHubOverview {
            enterprise_slug: state.client.enterprise_slug.clone(),
            connected,
            copilot_active_seats: copilot_active,
            copilot_total_seats: copilot_total,
            ghas_total_committers: ghas_committers,
            license_seats_consumed: license_consumed,
            license_seats_purchased: license_purchased,
            license_seats_available: license_purchased.saturating_sub(license_consumed),
        }),
    )
        .into_response()
}

/// GET /api/github/copilot/seats  Paginated Copilot seat list (FR-011 US-2)
/// GET /api/github/copilot/seats  Paginated Copilot seat list (FR-011 US-2)
///
/// Status logic (corrected):
///   - "pending_cancellation" when GitHub has set a pending_cancellation_date
///   - "inactive"             when no activity in the last 90 days but seat is assigned
///   - "active"               otherwise (includes newly-assigned seats with no activity yet)
///
/// Email/name enrichment:
///   The Copilot seats API returns only login/id/node_id in the assignee object.
///   We enrich from github_users_cache to enable full-text search on name and email.
pub async fn get_copilot_seats_list(
    State(state): State<GitHubState>,
    Query(params): Query<UserListQuery>,
) -> Response {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(25).min(100);
    let search = params.search.as_deref().unwrap_or("").to_lowercase();
    let status_filter = params.status.as_deref().map(|s| s.to_lowercase());

    // 90-day inactivity threshold used only for the "inactive" sub-status.
    let inactive_threshold = Utc::now() - chrono::Duration::days(90);

    //  1. Fetch all Copilot seats (all pages)
    let response = match state.client.fetch_copilot_seats().await {
        Ok(r) => r,
        Err(GitHubApiError::Unauthorized) => {
            return make_error(
                StatusCode::UNAUTHORIZED,
                "GitHub PAT token is invalid  update GITHUB_PAT_TOKEN",
            );
        }
        Err(GitHubApiError::NotFound(_)) => {
            return make_error(
                StatusCode::NOT_FOUND,
                "Copilot is not available for this enterprise. Check your GitHub Enterprise Copilot subscription and PAT token scopes.",
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch Copilot seats: {}", e);
            return make_error(StatusCode::BAD_GATEWAY, format!("GitHub API error: {}", e));
        }
    };

    let seats = response.seats.unwrap_or_default();

    //  2. Collect all logins and bulk-fetch email/name from cache
    let all_logins: Vec<String> = seats
        .iter()
        .filter_map(|s| s.assignee.as_ref()?.login.clone())
        .collect();

    // Build a HashMap<login_lowercase, (email, name)> from the cache.
    // Falls back to empty map on DB error so the handler never fails because of cache.
    let cache_map = match state.cache_repo.get_users_by_logins(&all_logins).await {
        Ok(users) => {
            let mut m = std::collections::HashMap::new();
            for u in users {
                m.insert(u.login.to_lowercase(), (u.email, u.name));
            }
            m
        }
        Err(e) => {
            tracing::warn!("Cache lookup for Copilot enrichment failed: {}", e);
            std::collections::HashMap::new()
        }
    };

    // Bulk-fetch linked person data (person_id, person_name, org_id, country) from DB.
    let person_map = match state
        .cache_repo
        .get_persons_by_github_logins(&all_logins)
        .await
    {
        Ok(persons) => {
            let mut m = std::collections::HashMap::new();
            for p in persons {
                let person_name = match (p.first_name.as_deref(), p.last_name.as_deref()) {
                    (Some(f), Some(l)) => Some(format!("{} {}", f, l)),
                    (Some(f), None) => Some(f.to_string()),
                    (None, Some(l)) => Some(l.to_string()),
                    (None, None) => None,
                };
                m.insert(
                    p.github_login.to_lowercase(),
                    (p.person_id, person_name, p.org_id, p.country),
                );
            }
            m
        }
        Err(e) => {
            tracing::warn!("Person lookup for Copilot enrichment failed: {}", e);
            std::collections::HashMap::new()
        }
    };

    //  3. Build enriched item list
    let mut items: Vec<CopilotSeatItem> = seats
        .iter()
        .map(|s| {
            let assignee = s.assignee.as_ref();
            let login = assignee
                .and_then(|a| a.login.as_deref())
                .unwrap_or("unknown")
                .to_string();

            // Prefer cache data; fall back to whatever the API returned.
            let cache_entry = cache_map.get(&login.to_lowercase());
            let email = cache_entry
                .and_then(|(e, _)| e.clone())
                .or_else(|| assignee.and_then(|a| a.email.clone()));
            let name = cache_entry
                .and_then(|(_, n)| n.clone())
                .or_else(|| assignee.and_then(|a| a.name.clone()));

            // Determine seat status:
            //   pending_cancellation > inactive (no activity) > active
            let has_pending = s.pending_cancellation_date.is_some();
            let status = if has_pending {
                "pending_cancellation"
            } else if s
                .last_activity_at
                .map(|t| t <= inactive_threshold)
                .unwrap_or(false)
            {
                // last_activity_at is set AND older than threshold  inactive seat
                "inactive"
            } else {
                // Either recently active OR newly assigned (null last_activity_at)
                "active"
            }
            .to_string();

            let person_entry = person_map.get(&login.to_lowercase());
            CopilotSeatItem {
                login,
                email,
                name,
                status,
                has_pending_cancellation: has_pending,
                last_activity_at: s.last_activity_at,
                plan_type: s.plan_type.clone(),
                person_id: person_entry.map(|(pid, _, _, _)| pid.clone()),
                person_name: person_entry.and_then(|(_, pn, _, _)| pn.clone()),
                org_id: person_entry.and_then(|(_, _, oid, _)| oid.clone()),
                country: person_entry.and_then(|(_, _, _, c)| c.clone()),
            }
        })
        .collect();

    //  4. Search filter  applied against enriched data
    if !search.is_empty() {
        items.retain(|item| {
            item.login.to_lowercase().contains(&search)
                || item
                    .email
                    .as_deref()
                    .map(|e| e.to_lowercase().contains(&search))
                    .unwrap_or(false)
                || item
                    .name
                    .as_deref()
                    .map(|n| n.to_lowercase().contains(&search))
                    .unwrap_or(false)
        });
    }

    //  5. Status filter
    if let Some(ref s) = status_filter {
        let s = s.as_str();
        if matches!(s, "active" | "inactive" | "pending_cancellation") {
            items.retain(|item| item.status == s);
        }
    }

    //  6. Paginate
    let total = items.len();
    let start = ((page - 1) * per_page) as usize;
    let data = items
        .into_iter()
        .skip(start)
        .take(per_page as usize)
        .collect();

    (
        StatusCode::OK,
        Json(PaginatedCopilotSeats {
            data,
            total,
            page,
            per_page,
        }),
    )
        .into_response()
}

/// GET /api/github/ghas/users  Paginated GHAS committer list (FR-011 US-3)
pub async fn get_ghas_users(
    State(state): State<GitHubState>,
    Query(params): Query<UserListQuery>,
) -> Response {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(25).min(100);
    let search = params.search.as_deref().unwrap_or("").to_lowercase();

    match state.client.fetch_ghas_usage().await {
        Ok(response) => {
            // Collect all GHAS logins for person lookup
            let ghas_logins: Vec<String> = response
                .repositories
                .iter()
                .flat_map(|repo| {
                    repo.advanced_security_committers_breakdown
                        .as_deref()
                        .unwrap_or(&[])
                        .iter()
                        .filter_map(|c| c.user_login.clone())
                })
                .collect();

            let person_map = match state
                .cache_repo
                .get_persons_by_github_logins(&ghas_logins)
                .await
            {
                Ok(persons) => {
                    let mut m = std::collections::HashMap::new();
                    for p in persons {
                        let person_name = match (p.first_name.as_deref(), p.last_name.as_deref()) {
                            (Some(f), Some(l)) => Some(format!("{} {}", f, l)),
                            (Some(f), None) => Some(f.to_string()),
                            (None, Some(l)) => Some(l.to_string()),
                            (None, None) => None,
                        };
                        m.insert(
                            p.github_login.to_lowercase(),
                            (p.person_id, person_name, p.org_id, p.country),
                        );
                    }
                    m
                }
                Err(e) => {
                    tracing::warn!("Person lookup for GHAS enrichment failed: {}", e);
                    std::collections::HashMap::new()
                }
            };

            // Flatten repositories to a list of unique committers
            let mut all_items: Vec<GHASUserItem> = response
                .repositories
                .iter()
                .flat_map(|repo| {
                    let repo_name = repo.name.clone();
                    let pm = &person_map;
                    repo.advanced_security_committers_breakdown
                        .as_deref()
                        .unwrap_or(&[])
                        .iter()
                        .map(move |c| {
                            let login = c
                                .user_login
                                .clone()
                                .unwrap_or_else(|| "unknown".to_string());
                            let person_entry = pm.get(&login.to_lowercase());
                            GHASUserItem {
                                login,
                                repository: repo_name.clone(),
                                last_pushed_date: c.last_pushed_date.clone(),
                                status: "active".to_string(),
                                person_id: person_entry.map(|(pid, _, _, _)| pid.clone()),
                                person_name: person_entry.and_then(|(_, pn, _, _)| pn.clone()),
                                org_id: person_entry.and_then(|(_, _, oid, _)| oid.clone()),
                                country: person_entry.and_then(|(_, _, _, c)| c.clone()),
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect();

            // Deduplicate: a committer may appear across multiple repos
            let mut seen: HashSet<String> = HashSet::new();
            all_items.retain(|item| seen.insert(item.login.clone()));

            // Search filter
            if !search.is_empty() {
                all_items.retain(|item| item.login.to_lowercase().contains(&search));
            }

            let total = all_items.len();
            let start = ((page - 1) * per_page) as usize;
            let data = all_items
                .into_iter()
                .skip(start)
                .take(per_page as usize)
                .collect();

            (
                StatusCode::OK,
                Json(PaginatedGHASUsers {
                    data,
                    total,
                    page,
                    per_page,
                }),
            )
                .into_response()
        }
        Err(GitHubApiError::Unauthorized) => make_error(
            StatusCode::UNAUTHORIZED,
            "GitHub PAT token is invalid  update GITHUB_PAT_TOKEN",
        ),
        Err(GitHubApiError::NotFound(_)) => make_error(
            StatusCode::NOT_FOUND,
            "GitHub Advanced Security is not available for this enterprise.",
        ),
        Err(e) => {
            tracing::error!("Failed to fetch GHAS usage: {}", e);
            make_error(StatusCode::BAD_GATEWAY, format!("GitHub API error: {}", e))
        }
    }
}

/// GET /api/github/license/users  Paginated license user list (FR-011 US-4)
pub async fn get_license_users(
    State(state): State<GitHubState>,
    Query(params): Query<UserListQuery>,
) -> Response {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(25).min(100);
    let search = params.search.as_deref().unwrap_or("").to_lowercase();

    // Fetch all license users using detailed field names (github_com_login etc.)
    let response = match state.client.fetch_consumed_licenses_detailed().await {
        Ok(r) => r,
        Err(GitHubApiError::Unauthorized) => {
            return make_error(
                StatusCode::UNAUTHORIZED,
                "GitHub PAT token is invalid  update GITHUB_PAT_TOKEN",
            );
        }
        Err(GitHubApiError::NotFound(_)) => {
            return make_error(
                StatusCode::NOT_FOUND,
                "GitHub Enterprise licensing data is not available.",
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch license data: {}", e);
            return make_error(StatusCode::BAD_GATEWAY, format!("GitHub API error: {}", e));
        }
    };

    // Collect logins for person lookup
    let all_logins: Vec<String> = response
        .users
        .iter()
        .filter_map(|u| u.github_com_login.clone())
        .collect();

    let person_map = match state
        .cache_repo
        .get_persons_by_github_logins(&all_logins)
        .await
    {
        Ok(persons) => {
            let mut m = std::collections::HashMap::new();
            for p in persons {
                let person_name = match (p.first_name.as_deref(), p.last_name.as_deref()) {
                    (Some(f), Some(l)) => Some(format!("{} {}", f, l)),
                    (Some(f), None) => Some(f.to_string()),
                    (None, Some(l)) => Some(l.to_string()),
                    (None, None) => None,
                };
                m.insert(
                    p.github_login.to_lowercase(),
                    (p.person_id, person_name, p.org_id, p.country),
                );
            }
            m
        }
        Err(e) => {
            tracing::warn!("Person lookup for license enrichment failed: {}", e);
            std::collections::HashMap::new()
        }
    };

    {
        let mut items: Vec<LicenseUserItem> = response
            .users
            .iter()
            .map(|u| {
                let login = u
                    .github_com_login
                    .clone()
                    .unwrap_or_else(|| "unknown".to_string());
                let email = u
                    .github_com_saml_name_id
                    .clone()
                    .or_else(|| u.github_com_verified_domain_emails.first().cloned());
                let person_entry = person_map.get(&login.to_lowercase());
                LicenseUserItem {
                    login,
                    email,
                    name: u.github_com_name.clone(),
                    status: "active".to_string(),
                    person_id: person_entry.map(|(pid, _, _, _)| pid.clone()),
                    person_name: person_entry.and_then(|(_, pn, _, _)| pn.clone()),
                    org_id: person_entry.and_then(|(_, _, oid, _)| oid.clone()),
                    country: person_entry.and_then(|(_, _, _, c)| c.clone()),
                }
            })
            .collect();

        // Search filter: login, email, name
        if !search.is_empty() {
            items.retain(|item| {
                item.login.to_lowercase().contains(&search)
                    || item
                        .email
                        .as_deref()
                        .map(|e| e.to_lowercase().contains(&search))
                        .unwrap_or(false)
                    || item
                        .name
                        .as_deref()
                        .map(|n| n.to_lowercase().contains(&search))
                        .unwrap_or(false)
            });
        }

        let total = items.len();
        let start = ((page - 1) * per_page) as usize;
        let data = items
            .into_iter()
            .skip(start)
            .take(per_page as usize)
            .collect();

        (
            StatusCode::OK,
            Json(PaginatedLicenseUsers {
                data,
                total,
                page,
                per_page,
            }),
        )
            .into_response()
    }
}

// ============================================================================
// Backward-compatible handlers (original URL scheme)
// ============================================================================

/// GET /api/github/enterprises/:enterprise/licenses
pub async fn get_licenses_compat(
    State(state): State<GitHubState>,
    Path(_enterprise): Path<String>,
) -> Response {
    match state.client.fetch_license_consumption().await {
        Ok(licenses) => {
            let seats_available = licenses
                .total_seats_purchased
                .saturating_sub(licenses.total_seats_consumed);
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "total_seats_consumed": licenses.total_seats_consumed,
                    "total_seats_purchased": licenses.total_seats_purchased,
                    "seats_available": seats_available,
                    "user_count": licenses.users.len(),
                })),
            )
                .into_response()
        }
        Err(GitHubApiError::NotFound(_)) => make_error(
            StatusCode::NOT_FOUND,
            "GitHub Enterprise licensing data is not available.",
        ),
        Err(e) => {
            tracing::error!("Failed to fetch licenses: {}", e);
            make_error(StatusCode::BAD_GATEWAY, e.to_string())
        }
    }
}

/// GET /api/github/enterprises/:enterprise/copilot
pub async fn get_copilot_seats_compat(
    State(state): State<GitHubState>,
    Path(_enterprise): Path<String>,
) -> Response {
    match state.client.fetch_copilot_seats().await {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(GitHubApiError::NotFound(_)) => {
            make_error(
                StatusCode::NOT_FOUND,
                "Copilot is not available for this enterprise. Check your GitHub Enterprise Copilot subscription and PAT token scopes.",
            )
        }
        Err(e) => {
            tracing::error!("Failed to fetch Copilot seats: {}", e);
            make_error(StatusCode::BAD_GATEWAY, e.to_string())
        }
    }
}

/// GET /api/github/enterprises/:enterprise/ghas
pub async fn get_ghas_usage_compat(
    State(state): State<GitHubState>,
    Path(_enterprise): Path<String>,
) -> Response {
    match state.client.fetch_ghas_usage().await {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(GitHubApiError::NotFound(_)) => make_error(
            StatusCode::NOT_FOUND,
            "GitHub Advanced Security is not available for this enterprise.",
        ),
        Err(e) => {
            tracing::error!("Failed to fetch GHAS usage: {}", e);
            make_error(StatusCode::BAD_GATEWAY, e.to_string())
        }
    }
}

// ============================================================================
// FR-012: Person <-> GitHub Link Handlers
// ============================================================================

/// GET /api/persons/:person_id/github
/// Return the GitHub link status and cached profile for a person.
pub async fn get_person_github_link(
    State(state): State<GitHubState>,
    Path(person_id): Path<String>,
) -> Response {
    match state.link_service.get_person_github_link(&person_id).await {
        Ok(link) => (StatusCode::OK, Json(link)).into_response(),
        Err(crate::github_link::GitHubLinkError::NotFound(msg)) => {
            make_error(StatusCode::NOT_FOUND, msg)
        }
        Err(e) => {
            tracing::error!("Failed to get GitHub link for person {}: {}", person_id, e);
            make_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}

/// POST /api/persons/:person_id/github/link
/// Manually link a person to a specific GitHub account.
pub async fn link_person_github(
    State(state): State<GitHubState>,
    Path(person_id): Path<String>,
    Json(req): Json<crate::github_link::ManualLinkRequest>,
) -> Response {
    match state
        .link_service
        .link_person_manual(&person_id, &req.github_login, "admin")
        .await
    {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(crate::github_link::GitHubLinkError::NotFound(msg)) => {
            make_error(StatusCode::NOT_FOUND, msg)
        }
        Err(crate::github_link::GitHubLinkError::Conflict(msg)) => {
            make_error(StatusCode::CONFLICT, msg)
        }
        Err(e) => {
            tracing::error!("Failed to link person {} to GitHub: {}", person_id, e);
            make_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}

/// DELETE /api/persons/:person_id/github/link
/// Remove the GitHub link from a person.
pub async fn unlink_person_github(
    State(state): State<GitHubState>,
    Path(person_id): Path<String>,
) -> Response {
    match state.link_service.unlink_person(&person_id, "admin").await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({ "success": true }))).into_response(),
        Err(crate::github_link::GitHubLinkError::NotFound(msg)) => {
            make_error(StatusCode::NOT_FOUND, msg)
        }
        Err(e) => {
            tracing::error!("Failed to unlink person {} from GitHub: {}", person_id, e);
            make_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}

/// POST /api/persons/:person_id/github/username
/// Set the `github_username` hint used for step-4 matching.
pub async fn set_person_github_username(
    State(state): State<GitHubState>,
    Path(person_id): Path<String>,
    Json(req): Json<crate::github_link::SetUsernameRequest>,
) -> Response {
    match state
        .link_service
        .set_github_username(&person_id, &req.username)
        .await
    {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({ "success": true }))).into_response(),
        Err(crate::github_link::GitHubLinkError::NotFound(msg)) => {
            make_error(StatusCode::NOT_FOUND, msg)
        }
        Err(e) => {
            tracing::error!(
                "Failed to set github_username for person {}: {}",
                person_id,
                e
            );
            make_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}

// ============================================================================
// FR-012: Organization <-> GitHub Link Handlers
// ============================================================================

/// GET /api/organizations/:org_id/github
/// Return GitHub license aggregates for an organization.
pub async fn get_org_github_info(
    State(state): State<GitHubState>,
    Path(org_id): Path<String>,
) -> Response {
    match state.link_service.get_org_github_info(&org_id).await {
        Ok(info) => (StatusCode::OK, Json(info)).into_response(),
        Err(crate::github_link::GitHubLinkError::NotFound(msg)) => {
            make_error(StatusCode::NOT_FOUND, msg)
        }
        Err(e) => {
            tracing::error!("Failed to get GitHub info for org {}: {}", org_id, e);
            make_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}

/// PUT /api/organizations/:org_id/github
/// Set GitHub Organization and Team names linked to this org.
pub async fn set_org_github_links(
    State(state): State<GitHubState>,
    Path(org_id): Path<String>,
    Json(req): Json<crate::github_link::SetOrgGitHubLinksRequest>,
) -> Response {
    match state
        .link_service
        .set_org_github_links(&org_id, req.org_names, req.team_names)
        .await
    {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({ "success": true }))).into_response(),
        Err(crate::github_link::GitHubLinkError::NotFound(msg)) => {
            make_error(StatusCode::NOT_FOUND, msg)
        }
        Err(e) => {
            tracing::error!("Failed to set GitHub links for org {}: {}", org_id, e);
            make_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}

// ============================================================================
// FR-012: Admin Sync Handlers
// ============================================================================

/// POST /api/admin/sync/github
/// Trigger a manual GitHub sync (users, licenses, Copilot, then person linking).
pub async fn trigger_github_sync(State(state): State<GitHubState>) -> Response {
    tracing::info!("Manual GitHub sync triggered  spawning background task");

    let client = state.client.clone();
    let cache_repo = state.cache_repo.clone();
    let link_service = state.link_service.clone();

    // Spawn the sync in a background task so it survives HTTP disconnects
    tokio::spawn(async move {
        let start = std::time::Instant::now();

        // Sync users + licenses + copilot to cache
        match sync_github_to_cache(&client, &cache_repo).await {
            Ok((users_synced, copilot_synced)) => {
                tracing::info!(users_synced, copilot_synced, "GitHub cache sync completed");
            }
            Err(e) => {
                tracing::error!("GitHub sync failed: {}", e);
                return;
            }
        }

        // Re-link unlinked persons (batch SQL  fast)
        match link_service.link_all_unlinked().await {
            Ok(stats) => {
                let linked = stats.linked_by_person_id
                    + stats.linked_by_local_id
                    + stats.linked_by_email
                    + stats.linked_by_username;
                tracing::info!(
                    linked,
                    no_match = stats.no_match,
                    duration_ms = start.elapsed().as_millis() as u64,
                    "GitHub sync + link completed"
                );
            }
            Err(e) => {
                tracing::error!("Person linking failed after sync: {}", e);
            }
        }
    });

    // Return immediately  sync runs in background
    (
        StatusCode::ACCEPTED,
        Json(serde_json::json!({
            "success": true,
            "message": "Sync started in background. Check /admin/sync/status for progress."
        })),
    )
        .into_response()
}

/// GET /api/admin/sync/github/status
/// Return the timestamp of the last sync and API reachability.
pub async fn get_github_sync_status(State(state): State<GitHubState>) -> Response {
    let last_sync_at = state.cache_repo.last_sync_at().await.unwrap_or(None);

    // Quick reachability check: validate the PAT token
    let github_api_reachable = state.client.validate_token().await.is_ok();

    (
        StatusCode::OK,
        Json(GitHubSyncStatus {
            enterprise_slug: state.client.enterprise_slug.clone(),
            last_sync_at,
            github_api_reachable,
        }),
    )
        .into_response()
}

/// GET /api/admin/github/unlinked
/// Return all GitHub accounts in the cache that are not linked to any person.
pub async fn get_unlinked_github_accounts(State(state): State<GitHubState>) -> Response {
    match state.cache_repo.get_unlinked_accounts().await {
        Ok(accounts) => (StatusCode::OK, Json(accounts)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch unlinked GitHub accounts: {}", e);
            make_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}
