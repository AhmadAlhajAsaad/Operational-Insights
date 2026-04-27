//! Import data types and DTOs (FR-007)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ============================================================================
// Database Models
// ============================================================================

/// Import record database model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Import {
    pub id: i32,
    pub import_id: String,
    pub file_name: String,
    pub file_size: i32,
    pub record_type: String,
    pub status: String,
    pub user_id: String,
    pub total_rows: Option<i32>,
    pub imported: Option<i32>,
    pub updated: Option<i32>,
    pub skipped: Option<i32>,
    pub errors: Option<i32>,
    pub rollback_available: Option<bool>,
    pub rollback_deadline: Option<DateTime<Utc>>,
    pub rollback_data: Option<serde_json::Value>,
    pub error_details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Import error record (database model)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ImportErrorRecord {
    pub id: i32,
    pub import_id: String,
    pub row_number: i32,
    pub field: Option<String>,
    pub value: Option<String>,
    pub error_type: String,
    pub message: String,
    pub severity: String,
    pub created_at: DateTime<Utc>,
}

/// Column mapping template
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ColumnMapping {
    pub id: i32,
    pub mapping_id: String,
    pub name: String,
    pub record_type: String,
    pub mappings: serde_json::Value,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// API Types
// ============================================================================

/// Import summary for list view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSummary {
    pub import_id: String,
    pub file_name: String,
    pub record_type: String,
    pub status: String,
    pub total_rows: i32,
    pub imported: i32,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub rollback_available: bool,
}

impl From<Import> for ImportSummary {
    fn from(i: Import) -> Self {
        ImportSummary {
            import_id: i.import_id,
            file_name: i.file_name,
            record_type: i.record_type,
            status: i.status,
            total_rows: i.total_rows.unwrap_or(0),
            imported: i.imported.unwrap_or(0),
            user_id: i.user_id,
            created_at: i.created_at,
            rollback_available: i.rollback_available.unwrap_or(false),
        }
    }
}

/// Import validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub total_rows: usize,
    pub valid_rows: usize,
    pub error_rows: usize,
    pub warning_rows: usize,
    pub errors: Vec<ValidationError>,
}

/// Single validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub row: usize,
    pub field: String,
    pub value: Option<String>,
    pub error_type: ValidationErrorType,
    pub message: String,
    pub severity: ErrorSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValidationErrorType {
    MissingField,
    FormatError,
    Duplicate,
    ReferenceError,
    InvalidValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ErrorSeverity {
    Error,
    Warning,
}

/// Import preview - what will change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportPreview {
    pub new_records: usize,
    pub updates: usize,
    pub unchanged: usize,
    pub skipped: usize,
    pub preview_data: Vec<PreviewRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewRow {
    pub row_number: usize,
    pub action: PreviewAction,
    pub data: serde_json::Value,
    pub changes: Option<Vec<FieldChange>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PreviewAction {
    Insert,
    Update,
    Skip,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldChange {
    pub field: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

/// Column mapping definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnMappingDefinition {
    pub source_column: String,
    pub target_field: String,
}

/// Create mapping request
#[derive(Debug, Deserialize)]
pub struct CreateMappingRequest {
    pub name: String,
    pub record_type: String,
    pub mappings: Vec<ColumnMappingDefinition>,
}

/// Import request (for starting an import)
#[derive(Debug, Deserialize)]
pub struct StartImportRequest {
    pub mapping_id: Option<String>,
    pub mappings: Option<Vec<ColumnMappingDefinition>>,
    pub skip_rows_with_errors: bool,
    pub start_vendor_matching: bool,
}

/// CSV/Excel row for person import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonImportRow {
    /// CSV column `person_id` — internal HR identifier (e.g. `GH5745`).
    /// Used as the unique key for the person, NOT as a link to Atlassian.
    pub id: Option<String>,
    /// CSV column `person_local_id` — login identity used by Atlassian as email
    /// (e.g. `CCJ183@equans.com`). Stored as `persons.local_id` and used as
    /// step-1 matching key against `atlassian_users_cache.email`.
    pub local_id: Option<String>,
    pub full_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    /// CSV column `person_email` — personal work email (e.g. `jan.devries@equans.com`).
    /// Used as step-2 fallback matching key against `atlassian_users_cache.email`.
    pub email: Option<String>,
    pub department: Option<String>,
    pub job_title: Option<String>,
    pub manager: Option<String>,
    pub start_date: Option<String>,
    pub status: Option<String>,
    pub cost_center: Option<String>,
    pub country: Option<String>,
    pub billing_location: Option<String>,
    pub org_id: Option<String>,
}

/// CSV/Excel row for organization import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgImportRow {
    pub org_id: Option<String>,
    pub org_name: Option<String>,
    pub parent_org: Option<String>,
    pub cost_center: Option<String>,
    pub manager: Option<String>,
    pub budget: Option<String>,
    pub org_type: Option<String>,
}
