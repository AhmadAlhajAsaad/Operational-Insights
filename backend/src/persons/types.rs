//! Person data types and DTOs (FR-005)

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ============================================================================
// Database Models
// ============================================================================

/// Person database model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub person_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub local_id: Option<String>,
    pub language: Option<String>,
    pub billing_location: Option<String>,
    pub country: Option<String>,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub manager: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub org_id: Option<String>,
    pub status: String,
    pub source: Option<String>,
    pub gid: Option<String>,
    pub gid_confidence: Option<i32>,
    pub gid_extraction_method: Option<String>,
    pub last_matched_at: Option<DateTime<Utc>>,
    pub matching_metadata: Option<serde_json::Value>,
    pub vendor_identifiers: Option<serde_json::Value>,

    // Atlassian link fields (FR-009)
    pub atlassian_account_id: Option<String>,
    pub atlassian_link_status: Option<String>,
    pub atlassian_linked_at: Option<DateTime<Utc>>,
    pub atlassian_link_method: Option<String>,

    // GitHub link fields (FR-012)
    pub github_login: Option<String>,
    pub github_account_id: Option<String>,
    pub github_username: Option<String>,
    pub github_link_status: Option<String>,
    pub github_linked_at: Option<DateTime<Utc>>,
    pub github_linked_by: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// API Response Types
// ============================================================================

/// Person summary for list view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonSummary {
    pub person_id: String,
    pub name: String,
    pub email: String,
    /// Login identity used by Atlassian (person_local_id from CSV, e.g. CCJ183@equans.com)
    pub local_id: Option<String>,
    pub org_id: Option<String>,
    pub country: Option<String>,
    pub billing_location: Option<String>,
    pub status: String,
    /// Raw GID value (e.g. "GID12345"), null when not yet matched
    pub gid: Option<String>,
    pub gid_status: GidStatus,

    // Atlassian link status (FR-009)
    pub atlassian_status: Option<String>,

    // GitHub link status (FR-012)
    pub github_status: Option<String>,
}

/// GID matching status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GidStatus {
    Matched,
    Pending,
    Unmatched,
}

impl From<Option<i32>> for GidStatus {
    fn from(confidence: Option<i32>) -> Self {
        match confidence {
            Some(c) if c >= 80 => GidStatus::Matched,
            Some(c) if c >= 50 => GidStatus::Pending,
            _ => GidStatus::Unmatched,
        }
    }
}

/// Person detail response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonDetail {
    pub person_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub local_id: Option<String>,
    pub language: Option<String>,
    pub billing_location: Option<String>,
    pub country: Option<String>,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub manager: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub org_id: Option<String>,
    pub status: String,
    pub source: Option<String>,

    // GID info
    pub gid: Option<String>,
    pub gid_confidence: Option<i32>,
    pub gid_extraction_method: Option<String>,
    pub gid_status: GidStatus,
    pub last_matched_at: Option<DateTime<Utc>>,
    pub matching_metadata: Option<serde_json::Value>,

    // Vendor info
    pub vendor_identifiers: Option<serde_json::Value>,

    // Atlassian link info (FR-009)
    pub atlassian_account_id: Option<String>,
    pub atlassian_link_status: Option<String>,
    pub atlassian_linked_at: Option<DateTime<Utc>>,
    pub atlassian_link_method: Option<String>,

    // Atlassian user details (from cache, if linked)
    pub atlassian_display_name: Option<String>,
    pub atlassian_email: Option<String>,
    pub atlassian_account_status: Option<String>,
    pub atlassian_active: Option<bool>,
    pub atlassian_last_active: Option<DateTime<Utc>>,
    pub atlassian_access_billable: Option<bool>,
    pub atlassian_product_access: Option<serde_json::Value>,

    // GitHub link info (FR-012)
    pub github_login: Option<String>,
    pub github_account_id: Option<String>,
    pub github_username: Option<String>,
    pub github_link_status: Option<String>,
    pub github_linked_at: Option<DateTime<Utc>>,
    pub github_linked_by: Option<String>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Person> for PersonDetail {
    fn from(p: Person) -> Self {
        PersonDetail {
            person_id: p.person_id,
            first_name: p.first_name,
            last_name: p.last_name,
            email: p.email,
            local_id: p.local_id,
            language: p.language,
            billing_location: p.billing_location,
            country: p.country,
            job_title: p.job_title,
            department: p.department,
            manager: p.manager,
            start_date: p.start_date,
            org_id: p.org_id,
            status: p.status,
            source: p.source,
            gid: p.gid,
            gid_confidence: p.gid_confidence,
            gid_extraction_method: p.gid_extraction_method,
            gid_status: GidStatus::from(p.gid_confidence),
            last_matched_at: p.last_matched_at,
            matching_metadata: p.matching_metadata,
            vendor_identifiers: p.vendor_identifiers,
            atlassian_account_id: p.atlassian_account_id,
            atlassian_link_status: p.atlassian_link_status,
            atlassian_linked_at: p.atlassian_linked_at,
            atlassian_link_method: p.atlassian_link_method,
            // Atlassian user details will be populated separately via JOIN or separate query
            atlassian_display_name: None,
            atlassian_email: None,
            atlassian_account_status: None,
            atlassian_active: None,
            atlassian_last_active: None,
            atlassian_access_billable: None,
            atlassian_product_access: None,
            github_login: p.github_login,
            github_account_id: p.github_account_id,
            github_username: p.github_username,
            github_link_status: p.github_link_status.clone(),
            github_linked_at: p.github_linked_at,
            github_linked_by: p.github_linked_by,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}

impl From<Person> for PersonSummary {
    fn from(p: Person) -> Self {
        PersonSummary {
            person_id: p.person_id,
            name: format!("{}, {}", p.last_name, p.first_name),
            email: p.email,
            local_id: p.local_id,
            org_id: p.org_id,
            country: p.country,
            billing_location: p.billing_location,
            status: p.status,
            gid: p.gid,
            gid_status: GidStatus::from(p.gid_confidence),
            atlassian_status: p.atlassian_link_status,
            github_status: p.github_link_status,
        }
    }
}

// ============================================================================
// Request Types
// ============================================================================

/// Query parameters for person list
#[derive(Debug, Deserialize, Default)]
pub struct PersonListParams {
    pub search: Option<String>,
    pub org_id: Option<String>,
    pub country: Option<String>,
    pub billing_location: Option<String>,
    pub status: Option<String>,
    pub gid_status: Option<String>,
    pub atlassian_status: Option<String>,
    pub github_status: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// Paginated response
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, page: i64, per_page: i64) -> Self {
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        PaginatedResponse {
            data,
            total,
            page,
            per_page,
            total_pages,
        }
    }
}

/// Create/Update person request
#[derive(Debug, Deserialize)]
pub struct CreatePersonRequest {
    pub person_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub local_id: Option<String>,
    pub language: Option<String>,
    pub billing_location: Option<String>,
    pub country: Option<String>,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub manager: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub org_id: Option<String>,
    pub status: Option<String>,
}
