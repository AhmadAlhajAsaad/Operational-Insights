//! Database repository for import operations (FR-007)

use sqlx::{PgPool, Postgres, Transaction};
use std::collections::HashSet;

use crate::imports::error::{ImportError, ImportResult};
use crate::imports::types::{Import, ImportSummary, ValidationError};
use crate::organizations::types::Organization;
use crate::persons::types::Person;

/// Repository for import database operations
pub struct ImportRepository {
    pool: PgPool,
}

/// Data record used for bulk person insert and update operations.
#[derive(Debug)]
pub struct PersonBulkRecord {
    pub person_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub local_id: Option<String>,
    pub billing_location: Option<String>,
    pub country: Option<String>,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub manager: Option<String>,
    pub org_id: Option<String>,
}

impl ImportRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // Import CRUD Operations
    // ========================================================================

    /// Create a new import record
    pub async fn create_import(
        &self,
        import_id: &str,
        file_name: &str,
        file_size: i32,
        record_type: &str,
        user_id: &str,
    ) -> ImportResult<Import> {
        let import = sqlx::query_as::<_, Import>(
            r#"
            INSERT INTO imports (
                import_id, file_name, file_size, record_type,
                user_id, status, created_at
            )
            VALUES ($1, $2, $3, $4, $5, 'Pending', NOW())
            RETURNING *
            "#,
        )
        .bind(import_id)
        .bind(file_name)
        .bind(file_size)
        .bind(record_type)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(import)
    }

    /// Update import statistics
    #[allow(clippy::too_many_arguments)]
    pub async fn update_import_stats(
        &self,
        import_id: &str,
        status: &str,
        total_rows: i32,
        imported: i32,
        updated: i32,
        skipped: i32,
        errors: i32,
    ) -> ImportResult<()> {
        sqlx::query(
            r#"
            UPDATE imports
            SET status = $2, total_rows = $3, imported = $4,
                updated = $5, skipped = $6, errors = $7,
                completed_at = CASE WHEN $2 IN ('Completed', 'Failed') THEN NOW() ELSE completed_at END
            WHERE import_id = $1
            "#
        )
        .bind(import_id)
        .bind(status)
        .bind(total_rows)
        .bind(imported)
        .bind(updated)
        .bind(skipped)
        .bind(errors)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Update import error details
    pub async fn update_import_error(
        &self,
        import_id: &str,
        error_message: &str,
    ) -> ImportResult<()> {
        let error_json = serde_json::json!({
            "error": error_message,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        sqlx::query(
            r#"
            UPDATE imports
            SET status = 'Failed',
                error_details = $2,
                completed_at = NOW()
            WHERE import_id = $1
            "#,
        )
        .bind(import_id)
        .bind(error_json)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get import by ID
    pub async fn get_import(&self, import_id: &str) -> ImportResult<Import> {
        let import = sqlx::query_as::<_, Import>("SELECT * FROM imports WHERE import_id = $1")
            .bind(import_id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| ImportError::NotFound(import_id.to_string()))?;

        Ok(import)
    }

    /// List imports with pagination
    pub async fn list_imports(
        &self,
        status: Option<&str>,
        record_type: Option<&str>,
        page: i64,
        per_page: i64,
    ) -> ImportResult<(Vec<ImportSummary>, i64)> {
        let offset = (page - 1) * per_page;

        // Build query with optional filters
        let mut query = String::from("SELECT * FROM imports WHERE 1=1");

        if status.is_some() {
            query.push_str(" AND status = $1");
        }
        if record_type.is_some() {
            query.push_str(if status.is_some() {
                " AND record_type = $2"
            } else {
                " AND record_type = $1"
            });
        }

        query.push_str(" ORDER BY created_at DESC LIMIT $");
        query.push_str(&format!(
            "{}",
            if status.is_some() && record_type.is_some() {
                3
            } else if status.is_some() || record_type.is_some() {
                2
            } else {
                1
            }
        ));
        query.push_str(" OFFSET $");
        query.push_str(&format!(
            "{}",
            if status.is_some() && record_type.is_some() {
                4
            } else if status.is_some() || record_type.is_some() {
                3
            } else {
                2
            }
        ));

        let mut sql_query = sqlx::query_as::<_, Import>(&query);

        if let Some(s) = status {
            sql_query = sql_query.bind(s);
        }
        if let Some(rt) = record_type {
            sql_query = sql_query.bind(rt);
        }

        sql_query = sql_query.bind(per_page).bind(offset);

        let imports = sql_query.fetch_all(&self.pool).await?;

        // Count total
        let mut count_query = String::from("SELECT COUNT(*) FROM imports WHERE 1=1");
        if status.is_some() {
            count_query.push_str(" AND status = $1");
        }
        if record_type.is_some() {
            count_query.push_str(if status.is_some() {
                " AND record_type = $2"
            } else {
                " AND record_type = $1"
            });
        }

        let mut sql_count = sqlx::query_scalar::<_, i64>(&count_query);
        if let Some(s) = status {
            sql_count = sql_count.bind(s);
        }
        if let Some(rt) = record_type {
            sql_count = sql_count.bind(rt);
        }

        let total = sql_count.fetch_one(&self.pool).await?;

        let summaries = imports.into_iter().map(ImportSummary::from).collect();

        Ok((summaries, total))
    }

    // ========================================================================
    // Import Errors
    // ========================================================================

    /// Save import errors to database
    pub async fn save_import_errors(
        &self,
        import_id: &str,
        errors: &[ValidationError],
    ) -> ImportResult<()> {
        if errors.is_empty() {
            return Ok(());
        }

        for error in errors {
            sqlx::query(
                r#"
                INSERT INTO import_errors (
                    import_id, row_number, field, value,
                    error_type, message, severity
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
            )
            .bind(import_id)
            .bind(error.row as i32)
            .bind(&error.field)
            .bind(&error.value)
            .bind(format!("{:?}", error.error_type))
            .bind(&error.message)
            .bind(format!("{:?}", error.severity))
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    // ========================================================================
    // Person Operations
    // ========================================================================

    /// Get all active persons
    pub async fn get_all_persons(&self) -> ImportResult<Vec<Person>> {
        let persons = sqlx::query_as::<_, Person>("SELECT * FROM persons ORDER BY person_id")
            .fetch_all(&self.pool)
            .await?;

        Ok(persons)
    }

    /// Get person by person_id
    pub async fn get_person_by_id(&self, person_id: &str) -> ImportResult<Option<Person>> {
        let person = sqlx::query_as::<_, Person>("SELECT * FROM persons WHERE person_id = $1")
            .bind(person_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(person)
    }

    /// Get multiple persons by person_ids (bulk query optimization)
    pub async fn get_persons_by_ids(&self, person_ids: &[String]) -> ImportResult<Vec<Person>> {
        if person_ids.is_empty() {
            return Ok(Vec::new());
        }

        let persons =
            sqlx::query_as::<_, Person>("SELECT * FROM persons WHERE person_id = ANY($1)")
                .bind(person_ids)
                .fetch_all(&self.pool)
                .await?;

        Ok(persons)
    }

    /// Get multiple persons by emails (for duplicate detection)
    pub async fn get_persons_by_emails(&self, emails: &[String]) -> ImportResult<Vec<Person>> {
        if emails.is_empty() {
            return Ok(Vec::new());
        }

        let persons = sqlx::query_as::<_, Person>("SELECT * FROM persons WHERE email = ANY($1)")
            .bind(emails)
            .fetch_all(&self.pool)
            .await?;

        Ok(persons)
    }

    /// Insert new person (in transaction)
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_person(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        person_id: &str,
        first_name: &str,
        last_name: &str,
        email: &str,
        local_id: Option<&str>,
        billing_location: Option<&str>,
        country: Option<&str>,
        job_title: Option<&str>,
        department: Option<&str>,
        manager: Option<&str>,
        org_id: Option<&str>,
    ) -> ImportResult<()> {
        sqlx::query(
            r#"
            INSERT INTO persons (
                person_id, first_name, last_name, email,
                local_id, billing_location, country, job_title,
                department, manager, org_id, status, source
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 'Active', 'Import')
            "#,
        )
        .bind(person_id)
        .bind(first_name)
        .bind(last_name)
        .bind(email)
        .bind(local_id)
        .bind(billing_location)
        .bind(country)
        .bind(job_title)
        .bind(department)
        .bind(manager)
        .bind(org_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// Update existing person (in transaction)
    pub async fn update_person(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        person: &Person,
    ) -> ImportResult<()> {
        sqlx::query(
            r#"
            UPDATE persons
            SET first_name = $2, last_name = $3, email = $4,
                local_id = $5, billing_location = $6, country = $7,
                job_title = $8, department = $9, manager = $10,
                org_id = $11, updated_at = NOW()
            WHERE person_id = $1
            "#,
        )
        .bind(&person.person_id)
        .bind(&person.first_name)
        .bind(&person.last_name)
        .bind(&person.email)
        .bind(&person.local_id)
        .bind(&person.billing_location)
        .bind(&person.country)
        .bind(&person.job_title)
        .bind(&person.department)
        .bind(&person.manager)
        .bind(&person.org_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// Update existing person including person_id (for person_id changes)
    pub async fn update_person_by_email(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        email: &str,
        new_person_id: &str,
        person: &Person,
    ) -> ImportResult<()> {
        sqlx::query(
            r#"
            UPDATE persons
            SET person_id = $2, first_name = $3, last_name = $4,
                local_id = $5, billing_location = $6, country = $7,
                job_title = $8, department = $9, manager = $10,
                org_id = $11, updated_at = NOW()
            WHERE email = $1
            "#,
        )
        .bind(email)
        .bind(new_person_id)
        .bind(&person.first_name)
        .bind(&person.last_name)
        .bind(&person.local_id)
        .bind(&person.billing_location)
        .bind(&person.country)
        .bind(&person.job_title)
        .bind(&person.department)
        .bind(&person.manager)
        .bind(&person.org_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// Update person status (for soft-delete and reactivation)
    pub async fn update_person_status(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        person_id: &str,
        status: &str,
    ) -> ImportResult<()> {
        sqlx::query("UPDATE persons SET status = $2, updated_at = NOW() WHERE person_id = $1")
            .bind(person_id)
            .bind(status)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    // ========================================================================
    // Organization Operations
    // ========================================================================

    /// Get all organizations
    pub async fn get_all_organizations(&self) -> ImportResult<Vec<Organization>> {
        let orgs = sqlx::query_as::<_, Organization>("SELECT * FROM organizations ORDER BY org_id")
            .fetch_all(&self.pool)
            .await?;

        Ok(orgs)
    }

    /// Get organization by org_id
    pub async fn get_organization_by_id(&self, org_id: &str) -> ImportResult<Option<Organization>> {
        let org =
            sqlx::query_as::<_, Organization>("SELECT * FROM organizations WHERE org_id = $1")
                .bind(org_id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(org)
    }

    /// Insert new organization (in transaction)
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_organization(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        org_id: &str,
        name: &str,
        parent_org_id: Option<&str>,
        cost_center: Option<&str>,
        manager: Option<&str>,
        budget: Option<f64>,
        org_type: &str,
    ) -> ImportResult<()> {
        sqlx::query(
            r#"
            INSERT INTO organizations (
                org_id, name, parent_org_id, cost_center,
                manager, budget, org_type, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, 'Active')
            "#,
        )
        .bind(org_id)
        .bind(name)
        .bind(parent_org_id)
        .bind(cost_center)
        .bind(manager)
        .bind(budget)
        .bind(org_type)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// Update existing organization (in transaction)
    pub async fn update_organization(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        org: &Organization,
    ) -> ImportResult<()> {
        sqlx::query(
            r#"
            UPDATE organizations
            SET name = $2, parent_org_id = $3, cost_center = $4,
                manager = $5, budget = $6, org_type = $7, updated_at = NOW()
            WHERE org_id = $1
            "#,
        )
        .bind(&org.org_id)
        .bind(&org.name)
        .bind(&org.parent_org_id)
        .bind(&org.cost_center)
        .bind(&org.manager)
        .bind(org.budget)
        .bind(&org.org_type)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    // ========================================================================
    // Optimized Preview Operations
    // ========================================================================

    /// Get only person IDs (optimized for preview)
    pub async fn get_person_ids(&self) -> ImportResult<Vec<String>> {
        let ids: Vec<(String,)> =
            sqlx::query_as("SELECT person_id FROM persons ORDER BY person_id")
                .fetch_all(&self.pool)
                .await?;

        Ok(ids.into_iter().map(|(id,)| id).collect())
    }

    /// Get person IDs with status (optimized for preview)
    pub async fn get_person_ids_with_status(&self) -> ImportResult<Vec<(String, String)>> {
        let ids: Vec<(String, String)> =
            sqlx::query_as("SELECT person_id, status FROM persons ORDER BY person_id")
                .fetch_all(&self.pool)
                .await?;

        Ok(ids)
    }

    /// Get persons matching import IDs or emails (memory-efficient for large imports)
    pub async fn get_persons_by_ids_or_emails(
        &self,
        person_ids: &[String],
        emails: &[String],
    ) -> ImportResult<Vec<(String, String)>> {
        // For large imports, use chunked queries to avoid SQL parameter limits
        const CHUNK_SIZE: usize = 5000;

        let mut all_results = Vec::new();

        // Process person_ids in chunks
        for chunk in person_ids.chunks(CHUNK_SIZE) {
            let placeholders = (1..=chunk.len())
                .map(|i| format!("${}", i))
                .collect::<Vec<_>>()
                .join(",");

            let query_str = format!(
                "SELECT person_id, status FROM persons WHERE person_id IN ({})",
                placeholders
            );

            let mut query = sqlx::query_as(&query_str);
            for id in chunk {
                query = query.bind(id);
            }

            let results: Vec<(String, String)> = query.fetch_all(&self.pool).await?;
            all_results.extend(results);
        }

        // Process emails in chunks
        for chunk in emails.chunks(CHUNK_SIZE) {
            let placeholders = (1..=chunk.len())
                .map(|i| format!("${}", i))
                .collect::<Vec<_>>()
                .join(",");

            let query_str = format!(
                "SELECT person_id, status FROM persons WHERE email IN ({})",
                placeholders
            );

            let mut query = sqlx::query_as(&query_str);
            for email in chunk {
                query = query.bind(email);
            }

            let results: Vec<(String, String)> = query.fetch_all(&self.pool).await?;
            all_results.extend(results);
        }

        //Deduplicate results
        let mut seen = HashSet::new();
        let unique_results: Vec<(String, String)> = all_results
            .into_iter()
            .filter(|(id, _)| seen.insert(id.clone()))
            .collect();

        Ok(unique_results)
    }

    /// Get only organization IDs (optimized for preview)
    pub async fn get_organization_ids(&self) -> ImportResult<Vec<String>> {
        let ids: Vec<(String,)> =
            sqlx::query_as("SELECT org_id FROM organizations ORDER BY org_id")
                .fetch_all(&self.pool)
                .await?;

        Ok(ids.into_iter().map(|(id,)| id).collect())
    }

    /// Count active persons matching criteria
    pub async fn count_active_persons_not_in_list(
        &self,
        person_ids: &[String],
    ) -> ImportResult<i64> {
        if person_ids.is_empty() {
            let count: (i64,) =
                sqlx::query_as("SELECT COUNT(*) FROM persons WHERE status = 'Active'")
                    .fetch_one(&self.pool)
                    .await?;
            return Ok(count.0);
        }

        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM persons WHERE status = 'Active' AND person_id != ALL($1)",
        )
        .bind(person_ids)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    // ========================================================================
    // GID Matching Operations
    // ========================================================================

    // ========================================================================
    // High-Performance Bulk Operations
    // ========================================================================

    /// Returns the set of org_ids that currently exist in the organizations table.
    /// Used to null-out invalid org_id references before bulk inserting persons.
    pub async fn get_valid_org_id_set(&self) -> ImportResult<std::collections::HashSet<String>> {
        let ids: Vec<(String,)> = sqlx::query_as("SELECT org_id FROM organizations")
            .fetch_all(&self.pool)
            .await?;
        Ok(ids.into_iter().map(|(id,)| id).collect())
    }

    /// True bulk INSERT using UNNEST: inserts thousands of persons in a single SQL round-trip.
    /// Uses ON CONFLICT DO NOTHING (no conflict target = covers ALL unique constraints
    /// including person_id AND email) so the whole batch never aborts.
    pub async fn bulk_insert_persons(&self, records: &[PersonBulkRecord]) -> ImportResult<u64> {
        if records.is_empty() {
            return Ok(0);
        }

        let person_ids: Vec<String> = records.iter().map(|r| r.person_id.clone()).collect();
        let first_names: Vec<String> = records.iter().map(|r| r.first_name.clone()).collect();
        let last_names: Vec<String> = records.iter().map(|r| r.last_name.clone()).collect();
        let emails: Vec<String> = records.iter().map(|r| r.email.clone()).collect();
        let local_ids: Vec<Option<String>> = records.iter().map(|r| r.local_id.clone()).collect();
        let billing_locations: Vec<Option<String>> =
            records.iter().map(|r| r.billing_location.clone()).collect();
        let countries: Vec<Option<String>> = records.iter().map(|r| r.country.clone()).collect();
        let job_titles: Vec<Option<String>> = records.iter().map(|r| r.job_title.clone()).collect();
        let departments: Vec<Option<String>> =
            records.iter().map(|r| r.department.clone()).collect();
        let managers: Vec<Option<String>> = records.iter().map(|r| r.manager.clone()).collect();
        let org_ids: Vec<Option<String>> = records.iter().map(|r| r.org_id.clone()).collect();

        let result = sqlx::query(
            r#"
            INSERT INTO persons (
                person_id, first_name, last_name, email,
                local_id, billing_location, country, job_title,
                department, manager, org_id, status, source
            )
            SELECT
                UNNEST($1::text[]),
                UNNEST($2::text[]),
                UNNEST($3::text[]),
                UNNEST($4::text[]),
                UNNEST($5::text[]),
                UNNEST($6::text[]),
                UNNEST($7::text[]),
                UNNEST($8::text[]),
                UNNEST($9::text[]),
                UNNEST($10::text[]),
                UNNEST($11::text[]),
                'Active',
                'Import'
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(&person_ids)
        .bind(&first_names)
        .bind(&last_names)
        .bind(&emails)
        .bind(&local_ids)
        .bind(&billing_locations)
        .bind(&countries)
        .bind(&job_titles)
        .bind(&departments)
        .bind(&managers)
        .bind(&org_ids)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// True bulk UPDATE using UNNEST: updates thousands of persons in a single SQL round-trip.
    pub async fn bulk_update_persons(&self, records: &[PersonBulkRecord]) -> ImportResult<u64> {
        if records.is_empty() {
            return Ok(0);
        }

        let person_ids: Vec<String> = records.iter().map(|r| r.person_id.clone()).collect();
        let first_names: Vec<String> = records.iter().map(|r| r.first_name.clone()).collect();
        let last_names: Vec<String> = records.iter().map(|r| r.last_name.clone()).collect();
        let emails: Vec<String> = records.iter().map(|r| r.email.clone()).collect();
        let local_ids: Vec<Option<String>> = records.iter().map(|r| r.local_id.clone()).collect();
        let billing_locations: Vec<Option<String>> =
            records.iter().map(|r| r.billing_location.clone()).collect();
        let countries: Vec<Option<String>> = records.iter().map(|r| r.country.clone()).collect();
        let job_titles: Vec<Option<String>> = records.iter().map(|r| r.job_title.clone()).collect();
        let departments: Vec<Option<String>> =
            records.iter().map(|r| r.department.clone()).collect();
        let managers: Vec<Option<String>> = records.iter().map(|r| r.manager.clone()).collect();
        let org_ids: Vec<Option<String>> = records.iter().map(|r| r.org_id.clone()).collect();

        let result = sqlx::query(
            r#"
            UPDATE persons
            SET
                first_name = data.first_name,
                last_name  = data.last_name,
                email      = data.email,
                local_id   = data.local_id,
                billing_location = data.billing_location,
                country    = data.country,
                job_title  = data.job_title,
                department = data.department,
                manager    = data.manager,
                org_id     = data.org_id,
                updated_at = NOW()
            FROM (
                SELECT
                    UNNEST($1::text[])        AS person_id,
                    UNNEST($2::text[])        AS first_name,
                    UNNEST($3::text[])        AS last_name,
                    UNNEST($4::text[])        AS email,
                    UNNEST($5::text[])        AS local_id,
                    UNNEST($6::text[])        AS billing_location,
                    UNNEST($7::text[])        AS country,
                    UNNEST($8::text[])        AS job_title,
                    UNNEST($9::text[])        AS department,
                    UNNEST($10::text[])       AS manager,
                    UNNEST($11::text[])       AS org_id
            ) AS data
            WHERE persons.person_id = data.person_id
            "#,
        )
        .bind(&person_ids)
        .bind(&first_names)
        .bind(&last_names)
        .bind(&emails)
        .bind(&local_ids)
        .bind(&billing_locations)
        .bind(&countries)
        .bind(&job_titles)
        .bind(&departments)
        .bind(&managers)
        .bind(&org_ids)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Bulk soft-delete: sets status='Inactive' for all persons not in the import,
    /// in a single UPDATE ... WHERE person_id = ANY($1).
    pub async fn bulk_soft_delete_persons(&self, person_ids: &[String]) -> ImportResult<u64> {
        if person_ids.is_empty() {
            return Ok(0);
        }

        let result = sqlx::query(
            "UPDATE persons SET status = 'Inactive', updated_at = NOW() WHERE person_id = ANY($1)",
        )
        .bind(person_ids)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Bulk reactivate persons: set status back to 'Active'
    pub async fn bulk_reactivate_persons(&self, person_ids: &[String]) -> ImportResult<u64> {
        if person_ids.is_empty() {
            return Ok(0);
        }

        let result = sqlx::query(
            "UPDATE persons SET status = 'Active', updated_at = NOW() WHERE person_id = ANY($1)",
        )
        .bind(person_ids)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Update import progress counters during execution (called after each batch).
    pub async fn update_import_progress(
        &self,
        import_id: &str,
        imported: i32,
        updated: i32,
    ) -> ImportResult<()> {
        sqlx::query("UPDATE imports SET imported = $2, updated = $3 WHERE import_id = $1")
            .bind(import_id)
            .bind(imported)
            .bind(updated)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Batch update GID matches for imported persons
    pub async fn batch_update_gid_matches(
        &self,
        updates: &[(String, String, i32, String)], // (person_id, gid, confidence, method)
    ) -> ImportResult<u64> {
        let mut tx = self.pool.begin().await?;
        let mut total_updated = 0u64;

        for (person_id, gid, confidence, method) in updates {
            let result = sqlx::query(
                r#"
                UPDATE persons
                SET gid = $1,
                    gid_confidence = $2,
                    gid_extraction_method = $3,
                    last_matched_at = NOW()
                WHERE person_id = $4
                "#,
            )
            .bind(gid)
            .bind(confidence)
            .bind(method)
            .bind(person_id)
            .execute(&mut *tx)
            .await?;

            total_updated += result.rows_affected();
        }

        tx.commit().await?;
        Ok(total_updated)
    }

    /// Update an existing organization from import data (in transaction)
    #[allow(clippy::too_many_arguments)]
    pub async fn update_organization_from_import(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        org_id: &str,
        name: &str,
        parent_org_id: Option<&str>,
        cost_center: Option<&str>,
        manager: Option<&str>,
        budget: Option<f64>,
        org_type: &str,
    ) -> ImportResult<()> {
        sqlx::query(
            r#"
            UPDATE organizations
            SET name = $2, parent_org_id = $3, cost_center = $4,
                manager = $5, budget = $6, org_type = $7, updated_at = NOW()
            WHERE org_id = $1
            "#,
        )
        .bind(org_id)
        .bind(name)
        .bind(parent_org_id)
        .bind(cost_center)
        .bind(manager)
        .bind(budget)
        .bind(org_type)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// Delete all organizations whose org_id is NOT in the provided list
    pub async fn delete_organizations_not_in(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        keep_org_ids: &[String],
    ) -> ImportResult<u64> {
        let result = sqlx::query(
            r#"DELETE FROM organizations WHERE org_id != ALL($1)"#,
        )
        .bind(keep_org_ids)
        .execute(&mut **tx)
        .await?;

        Ok(result.rows_affected())
    }
}
