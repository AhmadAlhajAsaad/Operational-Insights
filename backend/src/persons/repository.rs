//! Person repository for database operations (FR-005)

use crate::persons::types::{CreatePersonRequest, Person, PersonDetail, PersonListParams};
use sqlx::{PgPool, Postgres, QueryBuilder};

/// Person repository
pub struct PersonRepository {
    pool: PgPool,
}

impl PersonRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get all persons with pagination and filtering
    pub async fn list(&self, params: &PersonListParams) -> Result<(Vec<Person>, i64), sqlx::Error> {
        let page = params.page.unwrap_or(1).max(1);
        let per_page = params.per_page.unwrap_or(25).min(100);
        let offset = (page - 1) * per_page;

        // Build dynamic query
        let mut conditions = vec!["1=1".to_string()];

        if let Some(ref org_id) = params.org_id {
            conditions.push(format!("org_id = '{}'", org_id.replace('\'', "''")));
        }
        if let Some(ref country) = params.country {
            conditions.push(format!("country = '{}'", country.replace('\'', "''")));
        }
        if let Some(ref billing_location) = params.billing_location {
            conditions.push(format!(
                "billing_location = '{}'",
                billing_location.replace('\'', "''")
            ));
        }
        if let Some(ref status) = params.status {
            conditions.push(format!("status = '{}'", status.replace('\'', "''")));
        }
        if let Some(ref gid_status) = params.gid_status {
            match gid_status.as_str() {
                "matched" => conditions.push("gid_confidence >= 100".to_string()),
                "pending" => {
                    conditions.push("gid_confidence >= 30 AND gid_confidence < 100".to_string())
                }
                "unmatched" => {
                    conditions.push("(gid_confidence < 30 OR gid_confidence IS NULL)".to_string())
                }
                _ => {}
            }
        }
        if let Some(ref search) = params.search {
            if search.len() >= 2 {
                // Use GIN full-text search index (idx_persons_search) instead of ILIKE
                let tsquery_parts: Vec<String> = search
                    .split_whitespace()
                    .map(|s| {
                        s.chars()
                            .filter(|c| c.is_alphanumeric())
                            .collect::<String>()
                    })
                    .filter(|s| !s.is_empty())
                    .map(|s| format!("{}:*", s))
                    .collect();
                if !tsquery_parts.is_empty() {
                    let tsquery = tsquery_parts.join(" & ");
                    conditions.push(format!(
                        "to_tsvector('simple', coalesce(first_name,'') || ' ' || coalesce(last_name,'') || ' ' || coalesce(email,'') || ' ' || coalesce(person_id,'')) @@ to_tsquery('simple', '{}')"
                        ,tsquery
                    ));
                }
            }
        }
        if let Some(ref atlassian_status) = params.atlassian_status {
            match atlassian_status.as_str() {
                "linked" => conditions.push("atlassian_link_status LIKE 'linked%'".to_string()),
                "unlinked" => conditions.push("atlassian_link_status = 'unlinked'".to_string()),
                "no_account" => {
                    conditions.push("atlassian_link_status = 'no_atlassian_account'".to_string())
                }
                _ => {}
            }
        }

        if let Some(ref github_status) = params.github_status {
            match github_status.as_str() {
                "linked" => conditions.push("github_link_status LIKE 'linked%'".to_string()),
                "unlinked" => conditions.push("github_link_status = 'unlinked'".to_string()),
                "no_account" => {
                    conditions.push("github_link_status = 'no_github_account'".to_string())
                }
                _ => {}
            }
        }

        let where_clause = conditions.join(" AND ");

        let sort_by = params.sort_by.as_deref().unwrap_or("last_name");
        let sort_order = params.sort_order.as_deref().unwrap_or("ASC");
        let sort_column = match sort_by {
            "person_id"
            | "first_name"
            | "last_name"
            | "email"
            | "country"
            | "status"
            | "created_at"
            | "atlassian_link_status"
            | "github_link_status" => sort_by,
            _ => "last_name",
        };
        let order = if sort_order.to_uppercase() == "DESC" {
            "DESC"
        } else {
            "ASC"
        };

        // Count query
        let count_query = format!(
            "SELECT COUNT(*) as count FROM persons WHERE {}",
            where_clause
        );
        let count: (i64,) = sqlx::query_as(&count_query).fetch_one(&self.pool).await?;

        // Data query
        let data_query = format!(
            "SELECT * FROM persons WHERE {} ORDER BY {} {} LIMIT {} OFFSET {}",
            where_clause, sort_column, order, per_page, offset
        );
        let persons: Vec<Person> = sqlx::query_as(&data_query).fetch_all(&self.pool).await?;

        Ok((persons, count.0))
    }

    /// Get person by ID
    pub async fn get_by_id(&self, person_id: &str) -> Result<Option<Person>, sqlx::Error> {
        sqlx::query_as::<_, Person>("SELECT * FROM persons WHERE person_id = $1")
            .bind(person_id)
            .fetch_optional(&self.pool)
            .await
    }

    /// Get person by email
    pub async fn get_by_email(&self, email: &str) -> Result<Option<Person>, sqlx::Error> {
        sqlx::query_as::<_, Person>("SELECT * FROM persons WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
    }

    /// Create a new person
    pub async fn create(&self, req: &CreatePersonRequest) -> Result<Person, sqlx::Error> {
        sqlx::query_as::<_, Person>(
            r#"
            INSERT INTO persons (
                person_id, first_name, last_name, email, local_id,
                language, billing_location, country, job_title, department,
                manager, start_date, org_id, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING *
        "#,
        )
        .bind(&req.person_id)
        .bind(&req.first_name)
        .bind(&req.last_name)
        .bind(&req.email)
        .bind(&req.local_id)
        .bind(&req.language)
        .bind(&req.billing_location)
        .bind(&req.country)
        .bind(&req.job_title)
        .bind(&req.department)
        .bind(&req.manager)
        .bind(req.start_date)
        .bind(&req.org_id)
        .bind(req.status.as_deref().unwrap_or("Active"))
        .fetch_one(&self.pool)
        .await
    }

    /// Update a person
    pub async fn update(
        &self,
        person_id: &str,
        req: &CreatePersonRequest,
    ) -> Result<Option<Person>, sqlx::Error> {
        sqlx::query_as::<_, Person>(
            r#"
            UPDATE persons SET
                first_name = $2,
                last_name = $3,
                email = $4,
                local_id = $5,
                language = $6,
                billing_location = $7,
                country = $8,
                job_title = $9,
                department = $10,
                manager = $11,
                start_date = $12,
                org_id = $13,
                status = COALESCE($14, status)
            WHERE person_id = $1
            RETURNING *
        "#,
        )
        .bind(person_id)
        .bind(&req.first_name)
        .bind(&req.last_name)
        .bind(&req.email)
        .bind(&req.local_id)
        .bind(&req.language)
        .bind(&req.billing_location)
        .bind(&req.country)
        .bind(&req.job_title)
        .bind(&req.department)
        .bind(&req.manager)
        .bind(req.start_date)
        .bind(&req.org_id)
        .bind(&req.status)
        .fetch_optional(&self.pool)
        .await
    }

    /// Get persons by organization with optional search and atlassian filter
    pub async fn get_by_org(
        &self,
        org_id: &str,
        page: i64,
        per_page: i64,
        search: Option<&str>,
        atlassian_filter: Option<&str>,
    ) -> Result<(Vec<Person>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;

        // Helper closure to append shared filter conditions to a QueryBuilder
        let push_filters = |qb: &mut QueryBuilder<Postgres>,
                            org: &str,
                            srch: Option<&str>,
                            af: Option<&str>| {
            qb.push("org_id = ");
            qb.push_bind(org.to_owned());
            if let Some(s) = srch {
                if s.len() >= 2 {
                    let pattern = format!("%{}%", s);
                    qb.push(" AND (last_name ILIKE ");
                    qb.push_bind(pattern.clone());
                    qb.push(" OR first_name ILIKE ");
                    qb.push_bind(pattern.clone());
                    qb.push(" OR email ILIKE ");
                    qb.push_bind(pattern.clone());
                    qb.push(" OR person_id ILIKE ");
                    qb.push_bind(pattern);
                    qb.push(")");
                }
            }
            match af {
                Some("linked") => {
                    qb.push(" AND atlassian_link_status LIKE 'linked%'");
                }
                Some("unlinked") => {
                    qb.push(" AND (atlassian_link_status IS NULL OR atlassian_link_status NOT LIKE 'linked%')");
                }
                _ => {}
            }
        };

        // Count query
        let mut count_qb: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT COUNT(*) FROM persons WHERE ");
        push_filters(&mut count_qb, org_id, search, atlassian_filter);
        let count: (i64,) = count_qb.build_query_as().fetch_one(&self.pool).await?;

        // Data query
        let mut data_qb: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM persons WHERE ");
        push_filters(&mut data_qb, org_id, search, atlassian_filter);
        data_qb
            .push(" ORDER BY last_name ASC LIMIT ")
            .push_bind(per_page)
            .push(" OFFSET ")
            .push_bind(offset);
        let persons: Vec<Person> = data_qb.build_query_as().fetch_all(&self.pool).await?;

        Ok((persons, count.0))
    }

    /// Get atlassian linked count and org total for a single organization
    pub async fn get_atlassian_linked_count(
        &self,
        org_id: &str,
    ) -> Result<(i64, i64), sqlx::Error> {
        let linked: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM persons WHERE org_id = $1 AND atlassian_link_status LIKE 'linked%'",
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM persons WHERE org_id = $1")
            .bind(org_id)
            .fetch_one(&self.pool)
            .await?;

        Ok((linked.0, total.0))
    }

    /// Get inactive persons
    pub async fn get_inactive(&self) -> Result<Vec<Person>, sqlx::Error> {
        sqlx::query_as::<_, Person>(
            r#"
            SELECT * FROM persons
            WHERE status = 'Inactive'
            OR updated_at < NOW() - INTERVAL '90 days'
            ORDER BY updated_at ASC
        "#,
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Get statistics
    pub async fn get_stats(&self) -> Result<PersonStats, sqlx::Error> {
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM persons")
            .fetch_one(&self.pool)
            .await?;

        let active: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM persons WHERE status = 'Active'")
            .fetch_one(&self.pool)
            .await?;

        let matched: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM persons WHERE gid_confidence >= 80")
                .fetch_one(&self.pool)
                .await?;

        let atlassian_linked: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(DISTINCT p.person_id)
            FROM persons p
            INNER JOIN atlassian_users_cache a ON p.atlassian_account_id = a.account_id
            WHERE p.atlassian_link_status LIKE 'linked%'
              AND p.atlassian_account_id IS NOT NULL
              AND a.product_access IS NOT NULL
              AND jsonb_array_length(a.product_access) > 0
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(PersonStats {
            total: total.0,
            active: active.0,
            inactive: total.0 - active.0,
            gid_matched: matched.0,
            atlassian_linked: atlassian_linked.0,
        })
    }

    /// Get all persons for GID matching (paginated)
    pub async fn get_all_for_gid_matching(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Person>, sqlx::Error> {
        sqlx::query_as::<_, Person>(
            r#"
            SELECT * FROM persons
            ORDER BY person_id
            LIMIT $1 OFFSET $2
        "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Update GID match for a person
    pub async fn update_gid_match(
        &self,
        person_id: &str,
        gid: &str,
        confidence: i32,
        extraction_method: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
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
        .bind(extraction_method)
        .bind(person_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Batch update GID matches
    pub async fn batch_update_gid_matches(
        &self,
        updates: &[(String, String, i32, String)], // (person_id, gid, confidence, method)
    ) -> Result<u64, sqlx::Error> {
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

    /// Get all persons (for batch re-matching)
    /// Fetches in batches to avoid memory issues
    pub async fn get_all_persons_paginated(
        &self,
        batch_size: i64,
        offset: i64,
    ) -> Result<Vec<Person>, sqlx::Error> {
        sqlx::query_as::<_, Person>(
            r#"
            SELECT * FROM persons
            ORDER BY id
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(batch_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Count total persons
    pub async fn count_persons(&self) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM persons")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0)
    }

    /// Get person by ID with Atlassian details (FR-009)
    /// Returns PersonDetail with Atlassian user information if linked
    pub async fn get_by_id_with_atlassian(
        &self,
        person_id: &str,
    ) -> Result<Option<PersonDetail>, sqlx::Error> {
        // Use runtime query (not sqlx::query! macro) to avoid stale offline cache
        // when schema evolves (FR-012 added github_* columns).
        let row = sqlx::query(
            r#"
            SELECT
                p.id, p.person_id, p.first_name, p.last_name, p.email, p.local_id,
                p.language, p.billing_location, p.country, p.job_title, p.department,
                p.manager, p.start_date, p.org_id, p.status, p.source,
                p.gid, p.gid_confidence, p.gid_extraction_method, p.last_matched_at,
                p.matching_metadata, p.vendor_identifiers,
                p.atlassian_account_id, p.atlassian_link_status, p.atlassian_linked_at,
                p.atlassian_link_method,
                p.github_login, p.github_account_id, p.github_username,
                p.github_link_status, p.github_linked_at, p.github_linked_by,
                p.created_at, p.updated_at,
                a.display_name as atlassian_display_name,
                a.email as atlassian_email,
                a.account_status as atlassian_account_status,
                a.active as atlassian_active,
                a.last_active as atlassian_last_active,
                a.access_billable as atlassian_access_billable,
                a.product_access as atlassian_product_access
            FROM persons p
            LEFT JOIN atlassian_users_cache a ON p.atlassian_account_id = a.account_id
            WHERE p.person_id = $1
            "#,
        )
        .bind(person_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            use crate::persons::types::GidStatus;
            use sqlx::Row;

            let gid_confidence: Option<i32> = r.get("gid_confidence");
            PersonDetail {
                person_id: r.get("person_id"),
                first_name: r.get("first_name"),
                last_name: r.get("last_name"),
                email: r.get("email"),
                local_id: r.get("local_id"),
                language: r.get("language"),
                billing_location: r.get("billing_location"),
                country: r.get("country"),
                job_title: r.get("job_title"),
                department: r.get("department"),
                manager: r.get("manager"),
                start_date: r.get("start_date"),
                org_id: r.get("org_id"),
                status: r.get("status"),
                source: r.get("source"),
                gid: r.get("gid"),
                gid_confidence,
                gid_extraction_method: r.get("gid_extraction_method"),
                gid_status: GidStatus::from(gid_confidence),
                last_matched_at: r.get("last_matched_at"),
                matching_metadata: r.get("matching_metadata"),
                vendor_identifiers: r.get("vendor_identifiers"),
                atlassian_account_id: r.get("atlassian_account_id"),
                atlassian_link_status: r.get("atlassian_link_status"),
                atlassian_linked_at: r.get("atlassian_linked_at"),
                atlassian_link_method: r.get("atlassian_link_method"),
                atlassian_display_name: r.get("atlassian_display_name"),
                atlassian_email: r.get("atlassian_email"),
                atlassian_account_status: r.get("atlassian_account_status"),
                atlassian_active: r.get("atlassian_active"),
                atlassian_last_active: r.get("atlassian_last_active"),
                atlassian_access_billable: r.get("atlassian_access_billable"),
                atlassian_product_access: r.get("atlassian_product_access"),
                github_login: r.get("github_login"),
                github_account_id: r.get("github_account_id"),
                github_username: r.get("github_username"),
                github_link_status: r.get("github_link_status"),
                github_linked_at: r.get("github_linked_at"),
                github_linked_by: r.get("github_linked_by"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            }
        }))
    }

    /// Delete a person and all related PII (TS-06, GDPR Art. 17)
    ///
    /// Performs a hard delete of the person record and clears related data
    /// from linked tables. Used for GDPR right-to-erasure requests.
    pub async fn delete_person(&self, person_id: &str) -> Result<bool, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Clear GitHub link audit trail
        sqlx::query("DELETE FROM github_link_audit WHERE person_id = ")
            .bind(person_id)
            .execute(&mut *tx)
            .await?;

        // Clear Atlassian link audit trail
        sqlx::query("DELETE FROM atlassian_link_audit WHERE person_id = ")
            .bind(person_id)
            .execute(&mut *tx)
            .await?;

        // Delete the person record itself (hard delete)
        let result = sqlx::query("DELETE FROM persons WHERE person_id = ")
            .bind(person_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(result.rows_affected() > 0)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct PersonStats {
    pub total: i64,
    pub active: i64,
    pub inactive: i64,
    pub gid_matched: i64,
    pub atlassian_linked: i64,
}
