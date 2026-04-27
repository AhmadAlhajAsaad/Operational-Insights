//! Organization data types and DTOs (FR-006)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ============================================================================
// Database Models
// ============================================================================

/// Organization database model - using f64 for budget since it maps to NUMERIC
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Organization {
    pub id: i32,
    pub org_id: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_org_id: Option<String>,
    pub cost_center: Option<String>,
    pub manager: Option<String>,
    #[sqlx(default)]
    pub budget: Option<f64>,
    pub org_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// API Response Types
// ============================================================================

/// Organization summary for list view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationSummary {
    pub org_id: String,
    pub name: String,
    pub primary_country: Option<String>,
    pub person_count: i64,
    pub country_count: i64,
    pub status: String,
}

/// Organization detail response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationDetail {
    pub org_id: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_org_id: Option<String>,
    pub cost_center: Option<String>,
    pub manager: Option<String>,
    pub budget: Option<f64>,
    pub org_type: String,
    pub status: String,
    pub person_count: i64,
    pub children: Vec<OrganizationChild>,
    pub country_distribution: Vec<CountryDistribution>,
    pub billing_location_distribution: Vec<BillingLocationDistribution>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Child organization reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationChild {
    pub org_id: String,
    pub name: String,
    pub person_count: i64,
}

/// Country distribution in organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryDistribution {
    pub country: String,
    pub count: i64,
    pub percentage: f64,
}

/// Billing location distribution in organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingLocationDistribution {
    pub billing_location: String,
    pub count: i64,
    pub percentage: f64,
}

/// Business unit (org_type) distribution across organizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessUnitDistribution {
    pub business_unit: String,
    pub org_count: i64,
    pub person_count: i64,
    pub percentage: f64,
}

/// Organization tree node (for hierarchy view)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationTreeNode {
    pub org_id: String,
    pub name: String,
    pub person_count: i64,
    pub children: Vec<OrganizationTreeNode>,
}

// ============================================================================
// Request Types
// ============================================================================

/// Query parameters for organization list
#[derive(Debug, Deserialize, Default)]
pub struct OrganizationListParams {
    pub search: Option<String>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// Create/Update organization request
#[derive(Debug, Deserialize)]
pub struct CreateOrganizationRequest {
    pub org_id: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_org_id: Option<String>,
    pub cost_center: Option<String>,
    pub manager: Option<String>,
    pub budget: Option<f64>,
    pub org_type: Option<String>,
    pub status: Option<String>,
}

/// Organization statistics
#[derive(Debug, Serialize)]
pub struct OrganizationStats {
    pub total: i64,
    pub active: i64,
    pub total_persons: i64,
    pub countries: i64,
}

/// Linking statistics for data quality indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkingStats {
    pub total_persons: i64,
    pub persons_with_org: i64,
    pub persons_with_atlassian: i64,
    pub persons_with_github: i64,
    pub total_atlassian_cached: i64,
    pub total_github_cached: i64,
}
