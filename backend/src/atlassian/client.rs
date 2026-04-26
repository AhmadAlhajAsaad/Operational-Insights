//! Atlassian API HTTP client
//!
//! Handles all direct communication with the Atlassian Admin API.

use reqwest::Client;

use super::error::AtlassianError;
use super::types::{
    Group, GroupsResponse, InviteUserRequest, LicenseCount, LicenseCountDetailed, Organization,
    OrganizationsResponse, User, UserSummary, UsersResponse,
};

const API_BASE: &str = "https://api.atlassian.com";

/// Atlassian API client
#[derive(Clone)]
pub struct AtlassianClient {
    client: Client,
    api_token: String,
}

impl AtlassianClient {
    /// Create a new API client with the given token
    pub fn new(api_token: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .connect_timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            api_token,
        }
    }

    /// Fetch all organizations
    pub async fn fetch_organizations(&self) -> Result<Vec<Organization>, AtlassianError> {
        let url = format!("{}/admin/v1/orgs", API_BASE);

        tracing::info!("Fetching Atlassian organizations from: {}", url);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .header("Accept", "application/json")
            .header("User-Agent", "Equans-Operational-Insights/1.0")
            .send()
            .await?;

        self.handle_response_status(&response)?;

        let orgs_response: OrganizationsResponse = response
            .json()
            .await
            .map_err(|e| AtlassianError::InvalidResponse(e.to_string()))?;

        tracing::info!("Retrieved {} organizations", orgs_response.data.len());

        Ok(orgs_response.data)
    }

    /// Fetch all users for an organization (with pagination)
    pub async fn fetch_users(&self, org_id: &str) -> Result<Vec<User>, AtlassianError> {
        let mut all_users = Vec::new();
        let mut url = format!("{}/admin/v1/orgs/{}/users?limit=100", API_BASE, org_id);
        let mut page = 1;

        tracing::info!(
            "Fetching users for organization: {} (with pagination)",
            org_id
        );

        loop {
            tracing::debug!("Fetching page {} from: {}", page, url);

            let response = self
                .client
                .get(&url)
                .bearer_auth(&self.api_token)
                .header("Accept", "application/json")
                .header("User-Agent", "Equans-Operational-Insights/1.0")
                .send()
                .await?;

            self.handle_response_status(&response)?;

            let users_response: UsersResponse = response
                .json()
                .await
                .map_err(|e| AtlassianError::InvalidResponse(e.to_string()))?;

            let page_count = users_response.data.len();
            all_users.extend(users_response.data);

            tracing::debug!(
                "Page {}: retrieved {} users (total so far: {})",
                page,
                page_count,
                all_users.len()
            );

            if let Some(links) = users_response.links {
                if let Some(next_url) = links.next {
                    url = next_url;
                    page += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        tracing::info!(
            "Retrieved total of {} users across {} pages",
            all_users.len(),
            page
        );

        Ok(all_users)
    }

    /// Fetch all groups for an organization
    pub async fn fetch_groups(&self, org_id: &str) -> Result<Vec<Group>, AtlassianError> {
        let url = format!("{}/admin/v2/orgs/{}/directories/-/groups", API_BASE, org_id);

        tracing::info!("Fetching groups for organization: {}", org_id);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .header("Accept", "application/json")
            .header("User-Agent", "Equans-Operational-Insights/1.0")
            .send()
            .await?;

        self.handle_response_status(&response)?;

        let groups_response: GroupsResponse = response
            .json()
            .await
            .map_err(|e| AtlassianError::InvalidResponse(e.to_string()))?;

        tracing::info!("Retrieved {} groups", groups_response.data.len());

        Ok(groups_response.data)
    }

    /// Fetch groups with member counts
    pub async fn fetch_groups_with_counts(
        &self,
        org_id: &str,
    ) -> Result<Vec<Group>, AtlassianError> {
        let mut groups = self.fetch_groups(org_id).await?;

        tracing::info!("Fetching member counts for {} groups...", groups.len());

        for group in &mut groups {
            match self
                .fetch_users_in_groups(org_id, std::slice::from_ref(&group.id))
                .await
            {
                Ok(users) => {
                    group.member_count = Some(users.len() as u32);
                    tracing::debug!(
                        "Group '{}' ({}): {} members",
                        group.name,
                        group.id,
                        users.len()
                    );
                }
                Err(e) => {
                    tracing::warn!("Failed to fetch members for group '{}': {}", group.name, e);
                    group.member_count = None;
                }
            }
        }

        tracing::info!("Retrieved {} groups with member counts", groups.len());
        Ok(groups)
    }

    /// Fetch users belonging to specific groups
    pub async fn fetch_users_in_groups(
        &self,
        org_id: &str,
        group_ids: &[String],
    ) -> Result<Vec<User>, AtlassianError> {
        let group_ids_str = group_ids.join(",");
        let url = format!(
            "{}/admin/v2/orgs/{}/directories/-/users?group_ids={}",
            API_BASE, org_id, group_ids_str
        );

        tracing::info!("Fetching users for {} groups", group_ids.len());

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .header("Accept", "application/json")
            .header("User-Agent", "Equans-Operational-Insights/1.0")
            .send()
            .await?;

        self.handle_response_status(&response)?;

        let users_response: UsersResponse = response
            .json()
            .await
            .map_err(|e| AtlassianError::InvalidResponse(e.to_string()))?;

        tracing::info!("Retrieved {} users", users_response.data.len());

        Ok(users_response.data)
    }

    /// Calculate license counts for a product
    pub async fn calculate_licenses(
        &self,
        org_id: &str,
        product_name: &str,
    ) -> Result<LicenseCount, AtlassianError> {
        tracing::info!("Calculating license count for product: {}", product_name);

        let users = self.fetch_users(org_id).await?;

        let total_users: Vec<&User> = users
            .iter()
            .filter(|u| {
                u.product_access
                    .as_ref()
                    .is_some_and(|pa| pa.iter().any(|p| p.key == product_name))
            })
            .collect();

        let active_users = total_users
            .iter()
            .filter(|u| u.account_status == "active")
            .count();

        Ok(LicenseCount {
            product: product_name.to_string(),
            total_users: total_users.len(),
            active_users,
        })
    }

    /// Calculate detailed license information
    pub async fn calculate_licenses_detailed(
        &self,
        org_id: &str,
        product_name: &str,
    ) -> Result<LicenseCountDetailed, AtlassianError> {
        tracing::info!(
            "Calculating detailed license info for product: {}",
            product_name
        );

        let users = self.fetch_users(org_id).await?;

        let total_users: Vec<UserSummary> = users
            .iter()
            .filter(|u| {
                u.product_access
                    .as_ref()
                    .is_some_and(|pa| pa.iter().any(|p| p.key == product_name))
            })
            .map(|u| UserSummary {
                account_id: u.account_id.clone(),
                name: u.name.clone(),
                email: u.email.clone(),
                account_status: u.account_status.clone(),
            })
            .collect();

        let active_users: Vec<UserSummary> = total_users
            .iter()
            .filter(|u| u.account_status == "active")
            .cloned()
            .collect();

        Ok(LicenseCountDetailed {
            product: product_name.to_string(),
            total_users_count: total_users.len(),
            active_users_count: active_users.len(),
            total_users,
            active_users,
        })
    }

    // ========================================================================
    // User Management API (FR-008)
    // ========================================================================

    /// Get details for a specific user by account ID
    pub async fn get_user_by_id(
        &self,
        org_id: &str,
        account_id: &str,
    ) -> Result<User, AtlassianError> {
        let url = format!("{}/admin/v1/orgs/{}/users/{}", API_BASE, org_id, account_id);

        tracing::info!("Fetching user details for account_id: {}", account_id);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .header("Accept", "application/json")
            .header("User-Agent", "Equans-Operational-Insights/1.0")
            .send()
            .await?;

        self.handle_response_status(&response)?;

        let user: User = response
            .json()
            .await
            .map_err(|e| AtlassianError::InvalidResponse(e.to_string()))?;

        tracing::info!("Retrieved user: {}", user.email);

        Ok(user)
    }

    /// Invite a new user to the organization
    pub async fn invite_user(
        &self,
        org_id: &str,
        request: &InviteUserRequest,
    ) -> Result<String, AtlassianError> {
        let url = format!("{}/admin/v1/orgs/{}/invitations", API_BASE, org_id);

        tracing::info!("Inviting user: {}", request.email);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_token)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("User-Agent", "Equans-Operational-Insights/1.0")
            .json(request)
            .send()
            .await?;

        self.handle_response_status(&response)?;

        tracing::info!("User invitation sent: {}", request.email);

        Ok(format!("Invitation sent to {}", request.email))
    }

    /// Suspend a user (remove access but keep account)
    pub async fn suspend_user(
        &self,
        org_id: &str,
        account_id: &str,
    ) -> Result<String, AtlassianError> {
        let url = format!(
            "{}/admin/v1/orgs/{}/users/{}/manage/lifecycle/disable",
            API_BASE, org_id, account_id
        );

        tracing::info!("Suspending user: {}", account_id);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_token)
            .header("Accept", "application/json")
            .header("User-Agent", "Equans-Operational-Insights/1.0")
            .send()
            .await?;

        self.handle_response_status(&response)?;

        tracing::info!("User suspended: {}", account_id);

        Ok(format!("User {} suspended successfully", account_id))
    }

    /// Remove a user from the organization
    pub async fn remove_user(
        &self,
        org_id: &str,
        account_id: &str,
    ) -> Result<String, AtlassianError> {
        let url = format!("{}/admin/v1/orgs/{}/users/{}", API_BASE, org_id, account_id);

        tracing::info!("Removing user: {}", account_id);

        let response = self
            .client
            .delete(&url)
            .bearer_auth(&self.api_token)
            .header("Accept", "application/json")
            .header("User-Agent", "Equans-Operational-Insights/1.0")
            .send()
            .await?;

        self.handle_response_status(&response)?;

        tracing::info!("User removed: {}", account_id);

        Ok(format!("User {} removed from organization", account_id))
    }

    /// Handle HTTP response status codes
    fn handle_response_status(&self, response: &reqwest::Response) -> Result<(), AtlassianError> {
        let status = response.status();

        if status.is_success() {
            return Ok(());
        }

        match status.as_u16() {
            401 => Err(AtlassianError::Unauthorized),
            403 => Err(AtlassianError::Forbidden),
            404 => Err(AtlassianError::NotFound("Resource not found".to_string())),
            429 => {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse().ok());
                Err(AtlassianError::RateLimited { retry_after })
            }
            _ => Err(AtlassianError::ApiError {
                status: status.as_u16(),
                message: format!("HTTP {}", status),
            }),
        }
    }
}
