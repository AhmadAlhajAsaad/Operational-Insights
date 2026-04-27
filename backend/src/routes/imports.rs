//! Import API route handlers (FR-007)

use axum::{
    extract::{DefaultBodyLimit, Multipart, Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::auth::middleware::AuthenticatedUser;
use crate::imports::error::ImportError;
use crate::imports::{ImportService, ValidationResult};

/// Shared state for import routes
#[derive(Clone)]
pub struct ImportState {
    pub service: Arc<ImportService>,
}

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub upload_id: String,
    pub file_name: String,
    pub file_size: usize,
    pub total_rows: usize,
    pub parsed_organizations: usize,
    pub parsed_persons: usize,
    pub validation: ValidationResult,
}

#[derive(Debug, Deserialize)]
pub struct PreviewRequest {
    pub upload_id: String,
    #[serde(default)]
    pub import_valid_only: bool,
}

#[derive(Debug, Serialize)]
pub struct PreviewResponse {
    pub preview_id: String,
    pub organizations: OrganizationPreviewSummary,
    pub persons: PersonPreviewSummary,
    pub changes: Vec<ChangeDetailResponse>,
}

#[derive(Debug, Serialize)]
pub struct OrganizationPreviewSummary {
    pub new: usize,
    pub updated: usize,
    pub unchanged: usize,
    pub new_ids: Vec<String>,
    pub updated_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PersonPreviewSummary {
    pub new: usize,
    pub updated: usize,
    pub soft_deleted: usize,
    pub reactivated: usize,
    pub unchanged: usize,
    pub new_ids: Vec<String>,
    pub updated_ids: Vec<String>,
    pub soft_deleted_ids: Vec<String>,
    pub reactivated_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ChangeDetailResponse {
    pub record_type: String,
    pub record_id: String,
    pub change_type: String,
    pub field_changes: Vec<crate::imports::types::FieldChange>,
}

#[derive(Debug, Deserialize)]
pub struct ExecuteRequest {
    pub preview_id: String,
    #[serde(default)]
    pub confirmed: bool,
    #[serde(default)]
    pub async_mode: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ExecuteResponse {
    pub import_id: String,
    pub status: String,
    pub organizations: OrgStatsResponse,
    pub persons: PersonStatsResponse,
    pub duration_ms: u64,
    pub completed_at: String,
}

#[derive(Debug, Serialize)]
pub struct OrgStatsResponse {
    pub added: i32,
    pub updated: i32,
    pub deleted: i32,
}

#[derive(Debug, Serialize)]
pub struct PersonStatsResponse {
    pub added: i32,
    pub updated: i32,
    pub soft_deleted: i32,
    pub reactivated: i32,
}

#[derive(Debug, Deserialize)]
pub struct ListImportsParams {
    pub status: Option<String>,
    pub record_type: Option<String>,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

fn default_page() -> i64 {
    1
}
fn default_per_page() -> i64 {
    25
}

#[derive(Debug, Serialize)]
pub struct ListImportsResponse {
    pub data: Vec<crate::imports::types::ImportSummary>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

// ============================================================================
// Route Handlers
// ============================================================================

/// POST /api/imports/upload - Upload and parse file
pub async fn upload_file(
    State(state): State<ImportState>,
    user: Option<AuthenticatedUser>, // Optional authentication
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ImportError> {
    let _user_id = user.as_ref().map(|u| u.user_id()).unwrap_or("anonymous");
    // Extract file from multipart
    let mut file_name = String::new();
    let mut file_data = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        ImportError::InvalidRequest(format!("Failed to read multipart field: {}", e))
    })? {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            file_name = field.file_name().unwrap_or("upload.csv").to_string();
            file_data = field
                .bytes()
                .await
                .map_err(|e| {
                    ImportError::InvalidRequest(format!("Failed to read file data: {}", e))
                })?
                .to_vec();
        }
    }

    if file_data.is_empty() {
        return Err(ImportError::InvalidRequest("No file provided".to_string()));
    }

    // Parse file
    let upload = state
        .service
        .upload_and_parse(file_name.clone(), file_data)
        .await?;

    let response = UploadResponse {
        upload_id: upload.upload_id,
        file_name: upload.file_name,
        file_size: upload.file_size,
        total_rows: upload.organizations.len() + upload.persons.len(),
        parsed_organizations: upload.organizations.len(),
        parsed_persons: upload.persons.len(),
        validation: upload.validation,
    };

    Ok(Json(response))
}

/// POST /api/imports/preview - Generate preview of import
pub async fn generate_preview(
    State(state): State<ImportState>,
    user: Option<AuthenticatedUser>,
    Json(req): Json<PreviewRequest>,
) -> Result<impl IntoResponse, ImportError> {
    let _user_id = user.as_ref().map(|u| u.user_id()).unwrap_or("anonymous");
    let preview = state
        .service
        .generate_preview(&req.upload_id, req.import_valid_only)
        .await?;

    // Limit ID arrays to max 5 items to avoid huge HTTP responses on large imports
    const MAX_PREVIEW_IDS: usize = 5;
    let response = PreviewResponse {
        preview_id: preview.preview_id,
        organizations: OrganizationPreviewSummary {
            new: preview.organizations_preview.new_count,
            updated: preview.organizations_preview.updated_count,
            unchanged: preview.organizations_preview.unchanged_count,
            new_ids: preview
                .organizations_preview
                .new_ids
                .into_iter()
                .take(MAX_PREVIEW_IDS)
                .collect(),
            updated_ids: preview
                .organizations_preview
                .updated_ids
                .into_iter()
                .take(MAX_PREVIEW_IDS)
                .collect(),
        },
        persons: PersonPreviewSummary {
            new: preview.persons_preview.new_count,
            updated: preview.persons_preview.updated_count,
            soft_deleted: preview.persons_preview.soft_deleted_count,
            reactivated: preview.persons_preview.reactivated_count,
            unchanged: preview.persons_preview.unchanged_count,
            new_ids: preview
                .persons_preview
                .new_ids
                .into_iter()
                .take(MAX_PREVIEW_IDS)
                .collect(),
            updated_ids: preview
                .persons_preview
                .updated_ids
                .into_iter()
                .take(MAX_PREVIEW_IDS)
                .collect(),
            soft_deleted_ids: preview
                .persons_preview
                .soft_deleted_ids
                .into_iter()
                .take(MAX_PREVIEW_IDS)
                .collect(),
            reactivated_ids: preview
                .persons_preview
                .reactivated_ids
                .into_iter()
                .take(MAX_PREVIEW_IDS)
                .collect(),
        },
        changes: preview
            .changes
            .into_iter()
            .map(|c| ChangeDetailResponse {
                record_type: c.record_type,
                record_id: c.record_id,
                change_type: c.change_type,
                field_changes: c.field_changes,
            })
            .collect(),
    };

    Ok(Json(response))
}

/// POST /api/imports/execute - Execute import
pub async fn execute_import(
    State(state): State<ImportState>,
    user: Option<AuthenticatedUser>,
    Json(req): Json<ExecuteRequest>,
) -> Result<impl IntoResponse, ImportError> {
    let user_id = user.as_ref().map(|u| u.user_id()).unwrap_or("anonymous");

    if !req.confirmed {
        return Err(ImportError::InvalidRequest(
            "Import not confirmed".to_string(),
        ));
    }

    // Check if async mode is requested (for large imports)
    if req.async_mode.unwrap_or(true) {
        // Default to async for better UX
        tracing::info!("Starting async import for preview: {}", req.preview_id);

        let import_id = state
            .service
            .start_import_async(&req.preview_id, user_id)
            .await?;

        // Return immediately with import_id
        return Ok(Json(serde_json::json!({
            "import_id": import_id,
            "status": "Running",
            "message": "Import started in background. Use GET /api/imports/{import_id} to check status."
        })));
    }

    // Synchronous execution: wait for import to complete before responding
    tracing::info!("Starting sync import for preview: {}", req.preview_id);

    let (import_id, stats) = state
        .service
        .start_import_sync(&req.preview_id, user_id)
        .await?;

    Ok(Json(serde_json::json!({
        "import_id": import_id,
        "status": "Completed",
        "organizations": {
            "added": stats.organizations_added,
            "updated": stats.organizations_updated,
            "deleted": stats.organizations_deleted,
        },
        "persons": {
            "added": stats.persons_added,
            "updated": stats.persons_updated,
            "soft_deleted": stats.persons_soft_deleted,
            "reactivated": stats.persons_reactivated,
        },
    })))
}

/// GET /api/imports - List imports
pub async fn list_imports(
    State(state): State<ImportState>,
    _user: Option<AuthenticatedUser>,
    Query(params): Query<ListImportsParams>,
) -> Result<impl IntoResponse, ImportError> {
    let (imports, total) = state
        .service
        .list_imports(
            params.status.as_deref(),
            params.record_type.as_deref(),
            params.page,
            params.per_page,
        )
        .await?;

    let total_pages = (total as f64 / params.per_page as f64).ceil() as i64;

    let response = ListImportsResponse {
        data: imports,
        pagination: PaginationInfo {
            page: params.page,
            per_page: params.per_page,
            total,
            total_pages,
        },
    };

    Ok(Json(response))
}

/// GET /api/imports/:import_id - Get import details
pub async fn get_import(
    State(state): State<ImportState>,
    _user: Option<AuthenticatedUser>,
    Path(import_id): Path<String>,
) -> Result<impl IntoResponse, ImportError> {
    let import = state.service.get_import(&import_id).await?;
    Ok(Json(import))
}

/// POST /api/imports/quick-import - One-step import: upload, preview, and execute
/// Accepts multipart file upload with optional `import_valid_only` field
pub async fn quick_import(
    State(state): State<ImportState>,
    user: Option<AuthenticatedUser>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ImportError> {
    let user_id = user.as_ref().map(|u| u.user_id()).unwrap_or("anonymous");

    // Extract file and parameters from multipart
    let mut file_name = String::new();
    let mut file_data = Vec::new();
    let mut import_valid_only = false;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        ImportError::InvalidRequest(format!("Failed to read multipart field: {}", e))
    })? {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "file" => {
                file_name = field.file_name().unwrap_or("upload.csv").to_string();
                file_data = field
                    .bytes()
                    .await
                    .map_err(|e| {
                        ImportError::InvalidRequest(format!("Failed to read file data: {}", e))
                    })?
                    .to_vec();
            }
            "import_valid_only" => {
                let value = field.text().await.map_err(|e| {
                    ImportError::InvalidRequest(format!("Failed to read import_valid_only: {}", e))
                })?;
                import_valid_only = value == "true";
            }
            _ => {
                // Ignore unknown fields
            }
        }
    }

    if file_data.is_empty() {
        return Err(ImportError::InvalidRequest("No file provided".to_string()));
    }

    tracing::info!(
        "Quick import request from user {}: {} ({} bytes)",
        user_id,
        file_name,
        file_data.len()
    );

    // Execute quick import (async)
    let import_id = state
        .service
        .quick_import_async(file_name, file_data, user_id, import_valid_only)
        .await?;

    Ok(Json(serde_json::json!({
        "import_id": import_id,
        "status": "Running",
        "message": "Import started. Use GET /api/imports/{import_id} to check progress."
    })))
}

// ============================================================================
// Router Setup
// ============================================================================

/// Create import routes router
pub fn routes(service: Arc<ImportService>) -> Router {
    let state = ImportState { service };

    Router::new()
        .route("/upload", post(upload_file))
        .route("/preview", post(generate_preview))
        .route("/execute", post(execute_import))
        .route("/quick-import", post(quick_import))
        .route("/", get(list_imports))
        .route("/:import_id", get(get_import))
        .layer(DefaultBodyLimit::max(52_428_800)) // 50MB limit
        .with_state(state)
}
