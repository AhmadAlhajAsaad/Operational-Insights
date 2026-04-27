//! Import service layer with business logic (FR-007)

use chrono::Utc;
use serde::Serialize;
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

use crate::atlassian::link_service::AtlassianLinkService;
use crate::imports::error::{ImportError, ImportResult};
use crate::imports::merger::MergeEngine;
use crate::imports::parser::{FileFormat, FileParser};
use crate::imports::repository::ImportRepository;
use crate::imports::repository::PersonBulkRecord;
use crate::imports::types::{FieldChange, OrgImportRow, PersonImportRow, ValidationResult};
use crate::imports::validator::Validator;
use crate::persons::gid_matcher::GidMatcher;
use crate::persons::types::Person;

const MAX_FILE_SIZE: usize = 50 * 1024 * 1024; // 50MB

/// Temporary data structure for preview and execute
#[derive(Debug, Clone)]
pub struct UploadData {
    pub upload_id: String,
    pub file_name: String,
    pub file_size: usize,
    pub organizations: Vec<OrgImportRow>,
    pub persons: Vec<PersonImportRow>,
    pub validation: ValidationResult,
}

/// Preview data structure - stores only metadata, NOT the 85K records
/// The actual data stays in `uploads` HashMap until execution time
#[derive(Debug, Clone, Serialize)]
pub struct PreviewData {
    pub preview_id: String,
    pub upload_id: String,
    pub organizations_preview: OrganizationPreview,
    pub persons_preview: PersonPreview,
    pub changes: Vec<ChangeDetail>,
    pub total_changes: usize,    // Total number of changes (before sampling)
    pub changes_truncated: bool, // True if changes were truncated
    /// Whether to filter invalid rows at execution time
    #[serde(skip)]
    pub import_valid_only: bool,
    /// Cached person/org counts for async start (avoids reading upload data)
    #[serde(skip)]
    pub total_persons: usize,
    #[serde(skip)]
    pub total_orgs: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationPreview {
    pub new_count: usize,
    pub updated_count: usize,
    pub deleted_count: usize,
    pub unchanged_count: usize,
    pub new_ids: Vec<String>,
    pub updated_ids: Vec<String>,
    pub deleted_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PersonPreview {
    pub new_count: usize,
    pub updated_count: usize,
    pub soft_deleted_count: usize,
    pub reactivated_count: usize,
    pub unchanged_count: usize,
    pub new_ids: Vec<String>,
    pub updated_ids: Vec<String>,
    pub soft_deleted_ids: Vec<String>,
    pub reactivated_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChangeDetail {
    pub record_type: String,
    pub record_id: String,
    pub change_type: String,
    pub field_changes: Vec<FieldChange>,
}

#[derive(Debug, Clone)]
pub struct ImportStats {
    pub organizations_added: i32,
    pub organizations_updated: i32,
    pub organizations_deleted: i32,
    pub persons_added: i32,
    pub persons_updated: i32,
    pub persons_soft_deleted: i32,
    pub persons_reactivated: i32,
    pub persons_skipped: i32,
}

enum PersonAction {
    Added,
    Updated,
    Reactivated,
}

/// Import service orchestrates the import process
pub struct ImportService {
    pub repository: Arc<ImportRepository>,
    merger: MergeEngine,
    pool: PgPool,
    // In-memory cache for upload and preview data (in production, use Redis)
    uploads: Arc<tokio::sync::RwLock<HashMap<String, UploadData>>>,
    previews: Arc<tokio::sync::RwLock<HashMap<String, PreviewData>>>,
}

impl Clone for ImportService {
    fn clone(&self) -> Self {
        Self {
            repository: Arc::clone(&self.repository),
            merger: self.merger,
            pool: self.pool.clone(),
            uploads: Arc::clone(&self.uploads),
            previews: Arc::clone(&self.previews),
        }
    }
}

impl ImportService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: Arc::new(ImportRepository::new(pool.clone())),
            merger: MergeEngine,
            pool,
            uploads: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            previews: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    // ========================================================================
    // Upload & Parse
    // ========================================================================

    /// Upload and parse file
    pub async fn upload_and_parse(
        &self,
        file_name: String,
        file_data: Vec<u8>,
    ) -> ImportResult<UploadData> {
        // Check file size
        if file_data.len() > MAX_FILE_SIZE {
            return Err(ImportError::FileTooLarge(file_data.len()));
        }

        // Detect format and capture size before we move file_data
        let format = FileParser::detect_format(&file_data, &file_name)?;
        let file_size = file_data.len();

        // Parse file - run the CPU-bound work on the blocking thread pool so
        // we never starve the async executor with a 20s CSV parse.
        let (persons_raw, orgs_raw) = match format {
            FileFormat::Csv => tokio::task::spawn_blocking(move || {
                let (p, o, _) = FileParser::parse_csv_fast(&file_data)?;
                Ok::<_, ImportError>((p, o))
            })
            .await
            .map_err(|e| ImportError::ParseError(format!("Parse task panicked: {}", e)))??,
            FileFormat::Excel => {
                // Excel falls back to HashMap path (rare, small files)
                let raw = FileParser::parse_excel(&file_data)?;
                let first = raw.first().cloned().unwrap_or_default();
                if Self::detect_person_import(&first) {
                    (FileParser::parse_person_records(raw)?, vec![])
                } else {
                    (vec![], FileParser::parse_org_records(raw)?)
                }
            }
        };

        if persons_raw.is_empty() && orgs_raw.is_empty() {
            return Err(ImportError::ParseError(
                "No data rows found in file".to_string(),
            ));
        }

        let (organizations, persons, validation) = if !persons_raw.is_empty() || orgs_raw.is_empty()
        {
            let validation = Validator::validate_persons(&persons_raw);
            (vec![], persons_raw, validation)
        } else {
            let validation = Validator::validate_organizations(&orgs_raw);
            (orgs_raw, vec![], validation)
        };

        // Generate upload ID
        let upload_id = format!("UPL-{}", Utc::now().format("%Y%m%d-%H%M%S"));

        let upload_data = UploadData {
            upload_id: upload_id.clone(),
            file_name,
            file_size,
            organizations,
            persons,
            validation,
        };

        // Store in cache
        let mut uploads = self.uploads.write().await;
        uploads.insert(upload_id.clone(), upload_data.clone());

        Ok(upload_data)
    }

    fn detect_person_import(record: &HashMap<String, String>) -> bool {
        // Check for person-specific columns
        let person_columns = [
            "person_id",
            "person_email",
            "email",
            "first_name",
            "last_name",
        ];
        let org_columns = ["org_id", "org_name", "organization_id", "organization_name"];

        let has_person_column = person_columns
            .iter()
            .any(|col| record.keys().any(|k| k.to_lowercase().contains(col)));

        let has_org_column = org_columns
            .iter()
            .any(|col| record.keys().any(|k| k.to_lowercase().contains(col)));

        // If both, prefer person import
        has_person_column || !has_org_column
    }

    // ========================================================================
    // Preview Generation
    // ========================================================================

    /// Generate preview of import changes
    pub async fn generate_preview(
        &self,
        upload_id: &str,
        import_valid_only: bool,
    ) -> ImportResult<PreviewData> {
        // Get upload data
        let uploads = self.uploads.read().await;
        let upload = uploads
            .get(upload_id)
            .ok_or_else(|| ImportError::UploadNotFound(upload_id.to_string()))?;

        // Filter out invalid rows if requested
        let (orgs, persons) = if import_valid_only && !upload.validation.valid {
            // Build set of row numbers with errors (severity = Error)
            let error_rows: HashSet<usize> = upload
                .validation
                .errors
                .iter()
                .filter(|e| matches!(e.severity, crate::imports::types::ErrorSeverity::Error))
                .map(|e| e.row)
                .collect();

            // Filter persons: keep only rows not in error_rows AND not corrupt AND not duplicates
            // Note: row numbers in validation are 1-based, but Vec indices are 0-based
            // The validation row number corresponds to the CSV row (1-based index)
            // But we need to map this to the persons Vec index (0-based)

            // Track seen person_ids and emails for deduplication
            let mut seen_person_ids = HashSet::new();
            let mut seen_emails = HashSet::new();

            let filtered_persons: Vec<PersonImportRow> = upload
                .persons
                .iter()
                .enumerate()
                .filter(|(idx, person)| {
                    // Skip validation error rows
                    if error_rows.contains(&(idx + 2)) {
                        return false;
                    }

                    // Skip corrupt data: person_id with commas or very long
                    if let Some(id) = &person.id {
                        if id.contains(',') || id.len() > 50 {
                            tracing::warn!(
                                "Filtering out corrupt person_id at row {}: {}",
                                idx + 2,
                                &id[..id.len().min(100)]
                            );
                            return false;
                        }

                        // Deduplicate by person_id: keep first, skip rest
                        if seen_person_ids.contains(id) {
                            tracing::debug!(
                                "Skipping duplicate person_id '{}' at row {} (first occurrence kept)",
                                id, idx + 2
                            );
                            return false;
                        }
                        seen_person_ids.insert(id.clone());
                    }

                    // Deduplicate by email: keep first, skip rest
                    if let Some(email) = &person.email {
                        if !email.is_empty() {
                            if seen_emails.contains(email) {
                                tracing::debug!(
                                    "Skipping duplicate email '{}' at row {} (first occurrence kept)",
                                    email, idx + 2
                                );
                                return false;
                            }
                            seen_emails.insert(email.clone());
                        }
                    }

                    true
                })
                .map(|(_, p)| p.clone())
                .collect();

            // Filter organizations similarly
            let filtered_orgs: Vec<OrgImportRow> = upload
                .organizations
                .iter()
                .enumerate()
                .filter(|(idx, _)| !error_rows.contains(&(idx + 2)))
                .map(|(_, o)| o.clone())
                .collect();

            (filtered_orgs, filtered_persons)
        } else if !upload.validation.valid {
            return Err(ImportError::ValidationError(upload.validation.error_rows));
        } else {
            (upload.organizations.clone(), upload.persons.clone())
        };

        // Generate preview for organizations
        let orgs_preview = self.preview_organizations(&orgs).await?;

        // Generate preview for persons
        let persons_preview = self.preview_persons(&persons).await?;

        // Generate change details (with sampling for large imports)
        let (changes, total_changes, changes_truncated) =
            self.generate_change_details(&orgs, &persons).await?;

        // Generate preview ID
        let preview_id = format!("PRV-{}", Utc::now().format("%Y%m%d-%H%M%S"));

        let total_persons = persons.len();
        let total_orgs = orgs.len();

        // Drop local filtered vars - data stays in uploads HashMap, NOT duplicated here!
        drop(orgs);
        drop(persons);
        drop(uploads);

        let preview_data = PreviewData {
            preview_id: preview_id.clone(),
            upload_id: upload_id.to_string(),
            organizations_preview: orgs_preview,
            persons_preview,
            changes,
            total_changes,
            changes_truncated,
            import_valid_only,
            total_persons,
            total_orgs,
        };

        // Store in cache (only metadata, NO data duplication!)
        let mut previews = self.previews.write().await;
        previews.insert(preview_id.clone(), preview_data.clone());

        Ok(preview_data)
    }

    async fn preview_organizations(
        &self,
        orgs: &[OrgImportRow],
    ) -> ImportResult<OrganizationPreview> {
        // Use optimized method to get only IDs
        let existing_org_ids: HashSet<String> = self
            .repository
            .get_organization_ids()
            .await?
            .into_iter()
            .collect();

        let import_org_ids: HashSet<String> = orgs
            .iter()
            .filter_map(|o| o.org_id.clone())
            .filter(|id| !id.trim().is_empty())
            .collect();

        let mut new_ids = Vec::new();
        let mut updated_ids = Vec::new();

        for org in orgs {
            if let Some(org_id) = &org.org_id {
                if existing_org_ids.contains(org_id) {
                    updated_ids.push(org_id.clone());
                } else {
                    new_ids.push(org_id.clone());
                }
            }
        }

        // Find organizations in DB but not in import (will be deleted)
        let deleted_ids: Vec<String> = existing_org_ids
            .iter()
            .filter(|id| !import_org_ids.contains(*id))
            .cloned()
            .collect();

        Ok(OrganizationPreview {
            new_count: new_ids.len(),
            updated_count: updated_ids.len(),
            deleted_count: deleted_ids.len(),
            unchanged_count: 0,
            new_ids,
            updated_ids,
            deleted_ids,
        })
    }

    async fn preview_persons(&self, persons: &[PersonImportRow]) -> ImportResult<PersonPreview> {
        // Use optimized method to get only IDs with status
        let existing_persons_with_status: Vec<(String, String)> =
            self.repository.get_person_ids_with_status().await?;

        let existing_person_map: std::collections::HashMap<String, String> =
            existing_persons_with_status.into_iter().collect();

        let import_person_ids: HashSet<String> =
            persons.iter().filter_map(|p| p.id.clone()).collect();

        let mut new_ids = Vec::new();
        let mut updated_ids = Vec::new();
        let mut reactivated_ids = Vec::new();

        for person in persons {
            if let Some(person_id) = &person.id {
                if let Some(status) = existing_person_map.get(person_id) {
                    if status == "Inactive" {
                        reactivated_ids.push(person_id.clone());
                    } else {
                        updated_ids.push(person_id.clone());
                    }
                } else {
                    new_ids.push(person_id.clone());
                }
            }
        }

        // Find soft-deleted persons (in DB but not in import, and currently Active)
        let mut soft_deleted_ids = Vec::new();
        for (person_id, status) in existing_person_map.iter() {
            if !import_person_ids.contains(person_id) && status == "Active" {
                soft_deleted_ids.push(person_id.clone());
            }
        }

        Ok(PersonPreview {
            new_count: new_ids.len(),
            updated_count: updated_ids.len(),
            soft_deleted_count: soft_deleted_ids.len(),
            reactivated_count: reactivated_ids.len(),
            unchanged_count: 0,
            new_ids,
            updated_ids,
            soft_deleted_ids,
            reactivated_ids,
        })
    }

    async fn generate_change_details(
        &self,
        _orgs: &[OrgImportRow],
        persons: &[PersonImportRow],
    ) -> ImportResult<(Vec<ChangeDetail>, usize, bool)> {
        let total_records = persons.len();

        // For large imports skip DB comparison entirely - loading 85K persons just
        // to show 50 sample changes is too slow and blocks the HTTP response.
        if total_records > 1_000 {
            return Ok((vec![], total_records, true));
        }

        let mut changes = Vec::new();
        const MAX_PREVIEW_CHANGES: usize = 50;

        let existing_persons = self.repository.get_all_persons().await?;
        let existing_map: HashMap<String, &crate::persons::types::Person> = existing_persons
            .iter()
            .map(|p| (p.person_id.clone(), p))
            .collect();

        let mut changes_generated = 0;
        for person in persons.iter().take(MAX_PREVIEW_CHANGES) {
            if let Some(person_id) = &person.id {
                if let Some(db_person) = existing_map.get(person_id) {
                    let field_changes = self.compare_person_fields(db_person, person);
                    if !field_changes.is_empty() {
                        changes.push(ChangeDetail {
                            record_type: "Person".to_string(),
                            record_id: person_id.clone(),
                            change_type: if db_person.status == "Inactive" {
                                "Reactivate"
                            } else {
                                "Update"
                            }
                            .to_string(),
                            field_changes,
                        });
                        changes_generated += 1;
                    }
                }
            }
        }
        let _ = changes_generated;

        Ok((changes, total_records, false))
    }

    fn compare_person_fields(
        &self,
        db_person: &crate::persons::types::Person,
        import_person: &PersonImportRow,
    ) -> Vec<FieldChange> {
        let mut changes = Vec::new();

        // Compare first_name
        if let Some(import_first) = &import_person.first_name {
            if import_first != &db_person.first_name {
                changes.push(FieldChange {
                    field: "first_name".to_string(),
                    old_value: Some(db_person.first_name.clone()),
                    new_value: Some(import_first.clone()),
                });
            }
        }

        // Compare last_name
        if let Some(import_last) = &import_person.last_name {
            if import_last != &db_person.last_name {
                changes.push(FieldChange {
                    field: "last_name".to_string(),
                    old_value: Some(db_person.last_name.clone()),
                    new_value: Some(import_last.clone()),
                });
            }
        }

        // Compare country (showing merge logic)
        match (&import_person.country, &db_person.country) {
            (Some(imp), Some(db)) if imp != db && !imp.is_empty() => {
                changes.push(FieldChange {
                    field: "country".to_string(),
                    old_value: Some(db.clone()),
                    new_value: Some(imp.clone()),
                });
            }
            (Some(imp), Some(db)) if imp.is_empty() => {
                // Import empty, DB has value - will keep DB (no change)
                changes.push(FieldChange {
                    field: "country".to_string(),
                    old_value: Some(db.clone()),
                    new_value: None,
                });
            }
            _ => {}
        }

        changes
    }

    // ========================================================================
    // Execute Import
    // ========================================================================

    /// Execute the import based on preview (optimized for large datasets)
    /// import_id must be pre-created by the caller (start_import_async or direct call)
    pub async fn execute_import(
        &self,
        preview_id: &str,
        _user_id: &str,
        import_id: &str,
    ) -> ImportResult<ImportStats> {
        tracing::info!(
            "Starting import execution for preview: {} (import_id: {})",
            preview_id,
            import_id
        );

        // Get preview metadata only (no large data stored here anymore)
        let mut previews = self.previews.write().await;
        let preview = previews
            .remove(preview_id)
            .ok_or_else(|| ImportError::PreviewNotFound(preview_id.to_string()))?;
        drop(previews);

        let upload_id = preview.upload_id.clone();
        let import_valid_only = preview.import_valid_only;
        drop(preview);

        // Take OWNERSHIP of upload data (remove = move, no clone!) - this is the single copy
        let mut uploads = self.uploads.write().await;
        let upload = uploads
            .remove(&upload_id)
            .ok_or_else(|| ImportError::UploadNotFound(upload_id.clone()))?;
        drop(uploads);

        // Derive filtered data from upload (same logic as generate_preview)
        // This is the ONLY copy in RAM - no duplication!
        let (orgs, persons) = if import_valid_only && !upload.validation.valid {
            let error_rows: HashSet<usize> = upload
                .validation
                .errors
                .iter()
                .filter(|e| matches!(e.severity, crate::imports::types::ErrorSeverity::Error))
                .map(|e| e.row)
                .collect();

            let mut seen_person_ids = HashSet::new();
            let mut seen_emails = HashSet::new();

            let filtered_persons: Vec<PersonImportRow> = upload
                .persons
                .into_iter()
                .enumerate()
                .filter(|(idx, person)| {
                    if error_rows.contains(&(idx + 2)) {
                        return false;
                    }
                    if let Some(id) = &person.id {
                        if id.contains(',') || id.len() > 50 {
                            return false;
                        }
                        if seen_person_ids.contains(id) {
                            return false;
                        }
                        seen_person_ids.insert(id.clone());
                    }
                    if let Some(email) = &person.email {
                        if !email.is_empty() {
                            if seen_emails.contains(email) {
                                return false;
                            }
                            seen_emails.insert(email.clone());
                        }
                    }
                    true
                })
                .map(|(_, p)| p)
                .collect();

            let filtered_orgs: Vec<OrgImportRow> = upload
                .organizations
                .into_iter()
                .enumerate()
                .filter(|(idx, _)| !error_rows.contains(&(idx + 2)))
                .map(|(_, o)| o)
                .collect();

            (filtered_orgs, filtered_persons)
        } else {
            // No filtering - move directly (no clone!)
            (upload.organizations, upload.persons)
        };

        tracing::info!(
            "Import will process {} organizations and {} persons",
            orgs.len(),
            persons.len()
        );

        let mut stats = ImportStats {
            organizations_added: 0,
            organizations_updated: 0,
            organizations_deleted: 0,
            persons_added: 0,
            persons_updated: 0,
            persons_soft_deleted: 0,
            persons_reactivated: 0,
            persons_skipped: 0,
        };

        // Import organizations: insert new, update existing, delete missing
        if !orgs.is_empty() {
            let mut tx = self.pool.begin().await?;

            // Collect all org_ids from the import for the delete step
            let mut import_org_ids: Vec<String> = Vec::with_capacity(orgs.len());

            for org in &orgs {
                // Generate org_id if missing
                let org_id = if let Some(id) = &org.org_id {
                    if !id.trim().is_empty() {
                        id.clone()
                    } else {
                        if let Some(name) = &org.org_name {
                            format!("ORG_{}", name.replace(" ", "_").to_uppercase())
                        } else {
                            format!("ORG_{}", uuid::Uuid::new_v4())
                        }
                    }
                } else {
                    if let Some(name) = &org.org_name {
                        format!("ORG_{}", name.replace(" ", "_").to_uppercase())
                    } else {
                        format!("ORG_{}", uuid::Uuid::new_v4())
                    }
                };

                import_org_ids.push(org_id.clone());

                let exists = self
                    .repository
                    .get_organization_by_id(&org_id)
                    .await?
                    .is_some();

                let org_name = org.org_name.as_deref().unwrap_or("[Organization Name To Be Determined]");
                let org_type = org.org_type.as_deref().unwrap_or("Team");
                let budget = org.budget.as_ref().and_then(|b| b.parse::<f64>().ok());

                if exists {
                    self.repository
                        .update_organization_from_import(
                            &mut tx,
                            &org_id,
                            org_name,
                            org.parent_org.as_deref(),
                            org.cost_center.as_deref(),
                            org.manager.as_deref(),
                            budget,
                            org_type,
                        )
                        .await?;
                    stats.organizations_updated += 1;
                } else {
                    self.repository
                        .insert_organization(
                            &mut tx,
                            &org_id,
                            org_name,
                            org.parent_org.as_deref(),
                            org.cost_center.as_deref(),
                            org.manager.as_deref(),
                            budget,
                            org_type,
                        )
                        .await?;
                    stats.organizations_added += 1;
                }
            }

            // Delete organizations not present in the CSV import
            let deleted = self
                .repository
                .delete_organizations_not_in(&mut tx, &import_org_ids)
                .await?;
            stats.organizations_deleted = deleted as i32;

            tx.commit().await.map_err(|e| {
                tracing::error!("Organization import transaction failed: {}", e);
                ImportError::DatabaseError(format!("Organization import failed: {}", e))
            })?;

            tracing::info!(
                "Organizations imported: {} added, {} updated, {} deleted",
                stats.organizations_added,
                stats.organizations_updated,
                stats.organizations_deleted
            );
        }

        // Import persons in batches (optimized for large datasets)
        if !persons.is_empty() {
            // Extract person_ids and emails from import for targeted query
            let import_person_ids: Vec<String> =
                persons.iter().filter_map(|p| p.id.clone()).collect();
            let import_emails: Vec<String> =
                persons.iter().filter_map(|p| p.email.clone()).collect();

            // Only load existing persons that match import IDs or emails (memory efficient!)
            let existing_persons_with_status = self
                .repository
                .get_persons_by_ids_or_emails(&import_person_ids, &import_emails)
                .await?;
            drop(import_person_ids);
            drop(import_emails);
            let existing_person_map: std::collections::HashMap<String, String> =
                existing_persons_with_status.into_iter().collect();

            // Classify persons by index only (NO cloning - saves ~80MB for large imports!)
            // Store (index_into_persons, generated_person_id, generated_email)
            let mut new_insert_indices: Vec<(usize, String, String)> = Vec::new();
            let mut update_indices: Vec<(usize, String, String)> = Vec::new();

            for (idx, person) in persons.iter().enumerate() {
                // Validate and generate person_id
                let person_id = if let Some(id) = &person.id {
                    if id.contains(',') || id.len() > 50 {
                        tracing::warn!(
                            "Skipping person with corrupt person_id: {}",
                            &id[..id.len().min(100)]
                        );
                        stats.persons_skipped += 1;
                        continue;
                    }
                    if !id.trim().is_empty() {
                        let trimmed = id.trim();
                        if trimmed.len() > 20 {
                            trimmed[..20].to_string()
                        } else {
                            trimmed.to_string()
                        }
                    } else if let Some(email) = &person.email {
                        let prefix = email.split('@').next().unwrap_or("unk");
                        let safe_prefix = if prefix.len() > 15 {
                            &prefix[..15]
                        } else {
                            prefix
                        };
                        format!("AUTO_{}", safe_prefix)
                    } else {
                        format!("AUTO_{}", &uuid::Uuid::new_v4().to_string()[..8])
                    }
                } else if let Some(email) = &person.email {
                    let prefix = email.split('@').next().unwrap_or("unk");
                    let safe_prefix = if prefix.len() > 15 {
                        &prefix[..15]
                    } else {
                        prefix
                    };
                    format!("AUTO_{}", safe_prefix)
                } else {
                    format!("AUTO_{}", &uuid::Uuid::new_v4().to_string()[..8])
                };

                let email = if let Some(e) = &person.email {
                    if !e.trim().is_empty() {
                        let trimmed = e.trim();
                        if trimmed.len() > 255 {
                            trimmed[..255].to_string()
                        } else {
                            trimmed.to_string()
                        }
                    } else {
                        format!("unknown_{}@placeholder.local", person_id)
                    }
                } else {
                    format!("unknown_{}@placeholder.local", person_id)
                };

                if person_id.is_empty() {
                    stats.persons_skipped += 1;
                    continue;
                }

                if existing_person_map.contains_key(&person_id) {
                    update_indices.push((idx, person_id, email));
                } else {
                    new_insert_indices.push((idx, person_id, email));
                }
            }

            tracing::info!(
                "Classified persons: {} new inserts, {} updates/conflicts",
                new_insert_indices.len(),
                update_indices.len()
            );

            // Pre-fetch valid org_ids to avoid FK constraint violations.
            // Any org_id not in this set will be stored as NULL.
            let valid_org_ids = self.repository.get_valid_org_id_set().await?;
            let resolve_org_id = |org_id: Option<&String>| -> Option<String> {
                org_id.and_then(|id| {
                    if valid_org_ids.contains(id.as_str()) {
                        Some(id.clone())
                    } else {
                        None // FK does not exist - store NULL
                    }
                })
            };

            // ----------------------------------------------------------------
            // BULK INSERT (UNNEST) - single SQL per batch, no savepoints
            // ----------------------------------------------------------------
            const BULK_INSERT_SIZE: usize = 10_000;
            let insert_batches =
                (new_insert_indices.len() + BULK_INSERT_SIZE - 1).max(1) / BULK_INSERT_SIZE;

            for (batch_idx, insert_batch) in new_insert_indices.chunks(BULK_INSERT_SIZE).enumerate()
            {
                tracing::info!(
                    "Bulk inserting batch {}/{} ({} records via UNNEST)",
                    batch_idx + 1,
                    insert_batches,
                    insert_batch.len()
                );

                let mut records: Vec<PersonBulkRecord> = Vec::with_capacity(insert_batch.len());

                for (idx, person_id, email) in insert_batch {
                    let person = &persons[*idx];
                    let first_name = person
                        .first_name
                        .as_deref()
                        .filter(|s| !s.trim().is_empty())
                        .unwrap_or("[To Be Determined]")
                        .to_string();
                    let last_name = person
                        .last_name
                        .as_deref()
                        .filter(|s| !s.trim().is_empty())
                        .unwrap_or("[To Be Determined]")
                        .to_string();

                    records.push(PersonBulkRecord {
                        person_id: person_id.clone(),
                        first_name,
                        last_name,
                        email: email.clone(),
                        local_id: person.local_id.clone(),
                        billing_location: person.billing_location.clone(),
                        country: person.country.clone(),
                        job_title: person.job_title.clone(),
                        department: person.department.clone(),
                        manager: person.manager.clone(),
                        org_id: resolve_org_id(person.org_id.as_ref()),
                    });
                }

                let inserted = self.repository.bulk_insert_persons(&records).await?;
                stats.persons_added += inserted as i32;

                // Report progress after each batch
                self.repository
                    .update_import_progress(import_id, stats.persons_added, stats.persons_updated)
                    .await
                    .ok();
            }

            // Build set of all import person IDs for soft-delete check
            let all_import_person_ids: HashSet<&str> = new_insert_indices
                .iter()
                .chain(update_indices.iter())
                .map(|(_, pid, _)| pid.as_str())
                .collect();

            // ----------------------------------------------------------------
            // BULK UPDATE (UNNEST) - replace per-row savepoint loop
            // ----------------------------------------------------------------
            const UPDATE_BATCH_SIZE: usize = 5_000;
            let update_batches =
                (update_indices.len() + UPDATE_BATCH_SIZE - 1).max(1) / UPDATE_BATCH_SIZE;

            for (batch_idx, update_batch) in update_indices.chunks(UPDATE_BATCH_SIZE).enumerate() {
                tracing::info!(
                    "Processing update batch {}/{} ({} records via UNNEST UPDATE)",
                    batch_idx + 1,
                    update_batches,
                    update_batch.len()
                );

                // Pre-fetch existing persons for the MergeEngine
                let batch_person_ids: Vec<String> = update_batch
                    .iter()
                    .map(|(_, id, _)| id.clone())
                    .filter(|id| existing_person_map.contains_key(id))
                    .collect();

                let batch_db_persons_list = self
                    .repository
                    .get_persons_by_ids(&batch_person_ids)
                    .await?;
                let batch_db_persons: std::collections::HashMap<String, Person> =
                    batch_db_persons_list
                        .into_iter()
                        .map(|p| (p.person_id.clone(), p))
                        .collect();

                let mut update_records: Vec<PersonBulkRecord> = Vec::new();
                let mut reactivate_ids: Vec<String> = Vec::new();

                for (idx, person_id, email) in update_batch {
                    let person = &persons[*idx];

                    if let Some(status) = existing_person_map.get(person_id) {
                        if let Some(db_person) = batch_db_persons.get(person_id) {
                            let merged = MergeEngine::merge_person(db_person, person);

                            update_records.push(PersonBulkRecord {
                                person_id: merged.person_id.clone(),
                                first_name: merged.first_name.clone(),
                                last_name: merged.last_name.clone(),
                                email: merged.email.clone(),
                                local_id: merged.local_id.clone(),
                                billing_location: merged.billing_location.clone(),
                                country: merged.country.clone(),
                                job_title: merged.job_title.clone(),
                                department: merged.department.clone(),
                                manager: merged.manager.clone(),
                                org_id: resolve_org_id(merged.org_id.as_ref()),
                            });

                            if status == "Inactive" {
                                reactivate_ids.push(person_id.clone());
                                stats.persons_reactivated += 1;
                            } else {
                                stats.persons_updated += 1;
                            }
                        }
                    } else {
                        // Not found by person_id - fall back to email-based update (one-off)
                        tracing::debug!(
                            "Person {} not in existing map, skipping update",
                            person_id
                        );
                        let _ = email; // suppress unused warning
                        stats.persons_skipped += 1;
                    }
                }

                // Single UNNEST UPDATE for the whole batch
                let updated = self.repository.bulk_update_persons(&update_records).await?;
                tracing::debug!("Batch updated {} persons", updated);

                // Reactivate any inactive persons in one shot
                if !reactivate_ids.is_empty() {
                    self.repository
                        .bulk_reactivate_persons(&reactivate_ids)
                        .await?;
                }

                // Progress report
                self.repository
                    .update_import_progress(
                        import_id,
                        stats.persons_added,
                        stats.persons_updated + stats.persons_reactivated,
                    )
                    .await
                    .ok();
            }

            tracing::info!(
                "Persons imported: {} added, {} updated, {} reactivated, {} skipped",
                stats.persons_added,
                stats.persons_updated,
                stats.persons_reactivated,
                stats.persons_skipped
            );

            // ----------------------------------------------------------------
            // BULK SOFT-DELETE - single query replaces 1-by-1 loop
            // ----------------------------------------------------------------
            let to_soft_delete: Vec<String> = existing_person_map
                .iter()
                .filter(|(person_id, status)| {
                    !all_import_person_ids.contains(person_id.as_str()) && *status == "Active"
                })
                .map(|(person_id, _)| person_id.clone())
                .collect();

            if !to_soft_delete.is_empty() {
                tracing::info!(
                    "Soft-deleting {} persons in one query",
                    to_soft_delete.len()
                );
                let deleted = self
                    .repository
                    .bulk_soft_delete_persons(&to_soft_delete)
                    .await?;
                stats.persons_soft_deleted = deleted as i32;
                tracing::info!("Soft-deleted {} persons", stats.persons_soft_deleted);
            }

            // Run GID matching for imported persons
            tracing::info!("Starting GID matching for imported persons...");
            let gid_match_result = self.run_gid_matching_for_import(&persons).await;
            match gid_match_result {
                Ok(matched_count) => {
                    tracing::info!("GID matching completed: {} persons matched", matched_count);
                }
                Err(e) => {
                    tracing::warn!("GID matching failed (non-critical): {}", e);
                }
            }

            // Run Atlassian auto-linking for imported persons only
            tracing::info!(
                "Starting Atlassian auto-linking for {} imported persons...",
                all_import_person_ids.len()
            );
            let atlassian_link_service = AtlassianLinkService::new(self.pool.clone());
            let mut atlassian_linked = 0u32;
            for pid in &all_import_person_ids {
                match atlassian_link_service.link_person_by_matching(pid).await {
                    Ok(Some(_)) => atlassian_linked += 1,
                    Ok(None) => {}
                    Err(e) => {
                        tracing::debug!("Atlassian link failed for {}: {}", pid, e);
                    }
                }
            }
            tracing::info!(
                "Atlassian auto-linking completed: {}/{} linked",
                atlassian_linked,
                all_import_person_ids.len()
            );

            // Run GitHub auto-linking for imported persons only
            tracing::info!(
                "Starting GitHub auto-linking for {} imported persons...",
                all_import_person_ids.len()
            );
            let github_link_service = crate::github_link::GitHubLinkService::new(self.pool.clone());
            let mut github_linked = 0u32;
            for pid in &all_import_person_ids {
                match github_link_service.link_person_by_matching(pid).await {
                    Ok(Some(_)) => github_linked += 1,
                    Ok(None) => {}
                    Err(e) => {
                        tracing::debug!("GitHub link failed for {}: {}", pid, e);
                    }
                }
            }
            tracing::info!(
                "GitHub auto-linking completed: {}/{} linked",
                github_linked,
                all_import_person_ids.len()
            );
        }

        // Update import statistics
        let total_rows = (orgs.len() + persons.len()) as i32;
        self.repository
            .update_import_stats(
                import_id,
                "Completed",
                total_rows,
                stats.persons_added + stats.organizations_added,
                stats.persons_updated + stats.organizations_updated,
                0,
                0,
            )
            .await?;

        Ok(stats)
    }

    /// Start import execution asynchronously (non-blocking)
    /// Returns import_id immediately and runs import in background
    pub async fn start_import_async(
        &self,
        preview_id: &str,
        user_id: &str,
    ) -> ImportResult<String> {
        tracing::info!(
            "Starting async import execution for preview: {}",
            preview_id
        );

        // Get preview data - only read lengths, avoid cloning 85K+ records
        let previews = self.previews.read().await;
        let preview = previews
            .get(preview_id)
            .ok_or_else(|| ImportError::PreviewNotFound(preview_id.to_string()))?;

        // Only extract metadata (counts stored in preview, no large Vecs anymore!)
        let persons_len = preview.total_persons;
        let orgs_len = preview.total_orgs;
        let upload_id = preview.upload_id.clone();
        drop(previews);

        // Get upload metadata
        let uploads = self.uploads.read().await;
        let upload = uploads
            .get(&upload_id)
            .ok_or_else(|| ImportError::UploadNotFound(upload_id.clone()))?;

        let file_name = upload.file_name.clone();
        let file_size = upload.file_size;
        drop(uploads);

        // Generate import ID with timestamp + UUID suffix for uniqueness
        let now = Utc::now();
        let uuid_suffix = Uuid::new_v4().to_string()[..8].to_string();
        let import_id = format!("IMP-{}-{}", now.format("%Y%m%d-%H%M%S"), uuid_suffix);

        // Create import record with "Running" status
        let record_type = if persons_len > 0 {
            "Person"
        } else {
            "Organization"
        };
        self.repository
            .create_import(
                &import_id,
                &file_name,
                file_size as i32,
                record_type,
                user_id,
            )
            .await?;

        // Immediately update status to "Running"
        self.repository
            .update_import_stats(
                &import_id,
                "Running",
                (orgs_len + persons_len) as i32,
                0,
                0,
                0,
                0,
            )
            .await?;

        // Clone self for background task
        let service = self.clone();
        let import_id_clone = import_id.clone();
        let preview_id_clone = preview_id.to_string();
        let user_id_clone = user_id.to_string();

        // Spawn background task - pass the import_id so execute_import updates the right record
        tokio::spawn(async move {
            tracing::info!("Background import task started for: {}", import_id_clone);

            match service
                .execute_import(&preview_id_clone, &user_id_clone, &import_id_clone)
                .await
            {
                Ok(stats) => {
                    tracing::info!(
                        "Background import completed successfully: {} - {} added, {} updated",
                        import_id_clone,
                        stats.persons_added,
                        stats.persons_updated
                    );
                }
                Err(e) => {
                    tracing::error!("Background import failed: {} - {}", import_id_clone, e);

                    // Update status to Failed with error details
                    let error_message = format!("{}", e);
                    if let Err(update_err) = service
                        .repository
                        .update_import_error(&import_id_clone, &error_message)
                        .await
                    {
                        tracing::error!("Failed to update import error: {}", update_err);
                    }
                }
            }
        });

        tracing::info!("Import {} started in background", import_id);
        Ok(import_id)
    }

    /// Start import execution synchronously (blocking until complete)
    /// Returns import_id after the import has finished
    pub async fn start_import_sync(
        &self,
        preview_id: &str,
        user_id: &str,
    ) -> ImportResult<(String, ImportStats)> {
        tracing::info!("Starting sync import execution for preview: {}", preview_id);

        // Get preview data
        let previews = self.previews.read().await;
        let preview = previews
            .get(preview_id)
            .ok_or_else(|| ImportError::PreviewNotFound(preview_id.to_string()))?;

        let persons_len = preview.total_persons;
        let orgs_len = preview.total_orgs;
        let upload_id = preview.upload_id.clone();
        drop(previews);

        // Get upload metadata
        let uploads = self.uploads.read().await;
        let upload = uploads
            .get(&upload_id)
            .ok_or_else(|| ImportError::UploadNotFound(upload_id.clone()))?;

        let file_name = upload.file_name.clone();
        let file_size = upload.file_size;
        drop(uploads);

        // Generate import ID
        let now = Utc::now();
        let uuid_suffix = Uuid::new_v4().to_string()[..8].to_string();
        let import_id = format!("IMP-{}-{}", now.format("%Y%m%d-%H%M%S"), uuid_suffix);

        let record_type = if persons_len > 0 {
            "Person"
        } else {
            "Organization"
        };
        self.repository
            .create_import(
                &import_id,
                &file_name,
                file_size as i32,
                record_type,
                user_id,
            )
            .await?;

        self.repository
            .update_import_stats(
                &import_id,
                "Running",
                (orgs_len + persons_len) as i32,
                0,
                0,
                0,
                0,
            )
            .await?;

        // Execute synchronously (inline, not spawned)
        let stats = self.execute_import(preview_id, user_id, &import_id).await?;

        tracing::info!(
            "Sync import completed: {} - {} added, {} updated",
            import_id,
            stats.persons_added,
            stats.persons_updated
        );

        Ok((import_id, stats))
    }

    // ========================================================================
    // Query Operations
    // ========================================================================

    /// Get import by ID
    pub async fn get_import(&self, import_id: &str) -> ImportResult<crate::imports::types::Import> {
        self.repository.get_import(import_id).await
    }

    /// List imports with pagination
    pub async fn list_imports(
        &self,
        status: Option<&str>,
        record_type: Option<&str>,
        page: i64,
        per_page: i64,
    ) -> ImportResult<(Vec<crate::imports::types::ImportSummary>, i64)> {
        self.repository
            .list_imports(status, record_type, page, per_page)
            .await
    }

    // ========================================================================
    // GID Matching for Import
    // ========================================================================

    /// Run GID matching for persons in the import
    async fn run_gid_matching_for_import(
        &self,
        import_persons: &[PersonImportRow],
    ) -> ImportResult<u64> {
        // Extract person IDs from import
        let person_ids: Vec<String> = import_persons.iter().filter_map(|p| p.id.clone()).collect();

        if person_ids.is_empty() {
            tracing::debug!("No person IDs to match");
            return Ok(0);
        }

        tracing::info!(
            "Running GID matching for {} imported persons",
            person_ids.len()
        );

        // Initialize GID matcher
        let matcher = GidMatcher::new();

        // Fetch persons from database in batches
        const BATCH_SIZE: usize = 1000;
        let mut total_matched = 0u64;

        for (batch_idx, id_batch) in person_ids.chunks(BATCH_SIZE).enumerate() {
            // Fetch batch of persons from database
            let persons = self.repository.get_persons_by_ids(id_batch).await?;

            if persons.is_empty() {
                continue;
            }

            tracing::debug!(
                "GID matching batch {}: {} persons",
                batch_idx + 1,
                persons.len()
            );

            // Match GIDs for this batch
            let matches = matcher.match_batch(&persons);

            // Prepare updates (only for successful matches)
            let updates: Vec<(String, String, i32, String)> = matches
                .into_iter()
                .filter_map(|(person_id, match_opt)| {
                    match_opt.map(|m| (person_id, m.gid, m.confidence, m.extraction_method))
                })
                .collect();

            let matched_count = updates.len();

            // Batch update database
            if !updates.is_empty() {
                self.repository
                    .batch_update_gid_matches(&updates)
                    .await
                    .map_err(|e| {
                        tracing::error!("GID batch update failed: {}", e);
                        ImportError::DatabaseError(e.to_string())
                    })?;

                total_matched += matched_count as u64;
                tracing::debug!("Updated {} persons with GID matches", matched_count);
            }
        }

        tracing::info!(
            "GID matching completed: {}/{} persons matched",
            total_matched,
            person_ids.len()
        );
        Ok(total_matched)
    }

    // ========================================================================
    // Quick Import (One-Step Import)
    // ========================================================================

    /// Quick import: parse file synchronously, then run preview + execute in background.
    /// Returns import_id IMMEDIATELY after parsing so the frontend can start polling.
    pub async fn quick_import_async(
        &self,
        file_name: String,
        file_data: Vec<u8>,
        user_id: &str,
        import_valid_only: bool,
    ) -> ImportResult<String> {
        tracing::info!(
            "Starting quick import for file: {} (size: {} bytes)",
            file_name,
            file_data.len()
        );

        // Step 1: Create the import DB record IMMEDIATELY - before any parsing.
        //         This lets the frontend start polling within ~50 ms.
        let now = Utc::now();
        let uuid_suffix = Uuid::new_v4().to_string()[..8].to_string();
        let import_id = format!("IMP-{}-{}", now.format("%Y%m%d-%H%M%S"), uuid_suffix);

        self.repository
            .create_import(
                &import_id,
                &file_name,
                file_data.len() as i32,
                "Person", // will be corrected after parse if it's an org file
                user_id,
            )
            .await?;
        self.repository
            .update_import_stats(&import_id, "Running", 0, 0, 0, 0, 0)
            .await?;

        // Step 2: Spawn a background task that does ALL the heavy work:
        //         parse (on blocking pool) -> preview -> execute.
        //         The HTTP handler returns import_id immediately after this.
        let service = self.clone();
        let import_id_bg = import_id.clone();
        let user_id_owned = user_id.to_string();

        tokio::spawn(async move {
            // parse + validate runs on a blocking thread (CPU-bound, not async I/O)
            let upload = match service.upload_and_parse(file_name.clone(), file_data).await {
                Ok(u) => u,
                Err(e) => {
                    tracing::error!("Quick-import {} parse failed: {}", import_id_bg, e);
                    let _ = service
                        .repository
                        .update_import_error(&import_id_bg, &e.to_string())
                        .await;
                    return;
                }
            };

            let total_rows = (upload.persons.len() + upload.organizations.len()) as i32;
            let upload_id = upload.upload_id.clone();

            tracing::info!(
                "Quick import {} - parse done: {} rows, upload_id={}, generating preview",
                import_id_bg,
                total_rows,
                upload_id
            );

            // Update total row count now that we know it
            let _ = service
                .repository
                .update_import_stats(&import_id_bg, "Running", total_rows, 0, 0, 0, 0)
                .await;

            // Generate preview (fast: skips DB query for large imports)
            let preview_id = match service
                .generate_preview(&upload_id, import_valid_only)
                .await
            {
                Ok(p) => p.preview_id.clone(),
                Err(e) => {
                    tracing::error!("Quick-import preview failed ({}): {}", import_id_bg, e);
                    let _ = service
                        .repository
                        .update_import_error(&import_id_bg, &e.to_string())
                        .await;
                    return;
                }
            };

            // Execute import
            match service
                .execute_import(&preview_id, &user_id_owned, &import_id_bg)
                .await
            {
                Ok(stats) => {
                    tracing::info!(
                        "Quick-import {} completed: {} added, {} updated",
                        import_id_bg,
                        stats.persons_added,
                        stats.persons_updated
                    );
                }
                Err(e) => {
                    tracing::error!("Quick-import {} failed: {}", import_id_bg, e);
                    let _ = service
                        .repository
                        .update_import_error(&import_id_bg, &e.to_string())
                        .await;
                }
            }
        });

        // Step 3: Return import_id to the caller - this completes within ~50 ms
        //         and lets the frontend show the progress bar immediately.
        tracing::info!(
            "Quick import {} registered - background task running",
            import_id
        );
        Ok(import_id)
    }
}
