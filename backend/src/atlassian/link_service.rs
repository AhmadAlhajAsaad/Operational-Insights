//! Atlassian Link Service
//!
//! FR-009: Atlassian Gegevens Opslaan in Persons en Organizations
//! TR-009: Atlassian-naar-DB Synchronisatie
//!
//! This module handles linking persons to Atlassian accounts and
//! organizations to Atlassian groups.
//!
//! Linking strategy (two-step, in priority order):
//!   1. persons.local_id (= person_local_id from CSV, e.g. CCJ183@equans.com)
//!      is matched against atlassian_users_cache.email
//!      --> link_method = 'auto_local_id'
//!
//!   2. persons.email (= person_email from CSV, e.g. jan.devries@equans.com)
//!      is matched against atlassian_users_cache.email (fallback)
//!      --> link_method = 'auto_email'
//!
//!   3. Manual link via API
//!      --> link_method = 'manual'
//!
//! NOTE: person_id (e.g. GH5745) != account_id (e.g. 557058:4598ea15-...).
//!       These fields are NEVER used for matching.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::atlassian::error::ServiceError;

// ============================================================================
// Types
// ============================================================================

/// Result of a link operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkResult {
    pub person_id: String,
    pub account_id: String,
    pub link_method: LinkMethod,
    pub linked_at: DateTime<Utc>,
}

/// Link method used
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkMethod {
    AutoLocalId, // Matched via persons.local_id = atlassian.email (step 1)
    AutoEmail,   // Matched via persons.email = atlassian.email (step 2)
    AutoName,    // Matched via persons name = atlassian.display_name (step 3)
    Manual,      // Manually linked by administrator
}

impl LinkMethod {
    pub fn as_str(&self) -> &str {
        match self {
            LinkMethod::AutoLocalId => "auto_local_id",
            LinkMethod::AutoEmail => "auto_email",
            LinkMethod::AutoName => "auto_name",
            LinkMethod::Manual => "manual",
        }
    }

    pub fn as_status(&self) -> &str {
        match self {
            LinkMethod::AutoLocalId => "linked_auto_local_id",
            LinkMethod::AutoEmail => "linked_auto_email",
            LinkMethod::AutoName => "linked_auto_name",
            LinkMethod::Manual => "linked_manual",
        }
    }
}

/// Statistics from a link_all operation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkStats {
    pub linked_by_local_id: u32, // Matched via persons.local_id → atlassian.email (step 1)
    pub linked_by_email: u32,    // Matched via persons.email → atlassian.email (step 2)
    pub linked_by_name: u32,     // Matched via persons name → atlassian.display_name (step 3)
    pub already_linked: u32,     // Skipped: already had a link
    pub no_match: u32,           // No Atlassian account found via any step
    pub ambiguous: u32,          // Multiple Atlassian accounts matched
    pub errors: u32,
}

/// Unlinked Atlassian user (in cache but not linked to any person)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UnlinkedAtlassianUser {
    pub account_id: String,
    pub display_name: String,
    pub email: Option<String>,
    pub account_status: Option<String>,
    pub active: bool,
    pub product_access: Option<serde_json::Value>,
}

/// Person's Atlassian link status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonAtlassianLink {
    pub person_id: String,
    pub atlassian_account_id: Option<String>,
    pub atlassian_link_status: String,
    pub atlassian_linked_at: Option<DateTime<Utc>>,
    pub atlassian_link_method: Option<String>,

    // Atlassian user details if linked
    pub atlassian_profile: Option<AtlassianUserProfile>,
}

/// Atlassian user profile details
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AtlassianUserProfile {
    pub account_id: String,
    pub display_name: String,
    pub email: Option<String>,
    pub account_status: Option<String>,
    pub active: bool,
    pub last_active: Option<DateTime<Utc>>,
    pub access_billable: Option<bool>,
    pub product_access: Option<serde_json::Value>,
}

// ============================================================================
// Service
// ============================================================================

/// Service responsible for linking persons to Atlassian accounts
/// and organizations to Atlassian groups
pub struct AtlassianLinkService {
    pool: PgPool,
}

impl AtlassianLinkService {
    /// Create a new link service
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Attempt to link all unlinked persons to Atlassian accounts.
    /// Called after CSV import and after Atlassian sync.
    ///
    /// Step 1: match persons.local_id against atlassian_users_cache.email
    /// Step 2: match persons.email against atlassian_users_cache.email (fallback)
    pub async fn link_all_unlinked(&self) -> Result<LinkStats, ServiceError> {
        let mut stats = LinkStats::default();

        // Get all unlinked persons
        let unlinked_persons = sqlx::query!(
            r#"
            SELECT person_id, local_id, email
            FROM persons
            WHERE atlassian_account_id IS NULL
              AND atlassian_link_status = 'unlinked'
              AND status = 'Active'
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        tracing::info!(
            count = unlinked_persons.len(),
            "Starting Atlassian link matching for unlinked persons"
        );

        // Try to link each person
        for person in unlinked_persons {
            match self.link_person_by_matching(&person.person_id).await {
                Ok(Some(result)) => {
                    match result.link_method {
                        LinkMethod::AutoLocalId => stats.linked_by_local_id += 1,
                        LinkMethod::AutoEmail => stats.linked_by_email += 1,
                        LinkMethod::AutoName => stats.linked_by_name += 1,
                        _ => {}
                    }
                    tracing::info!(
                        person_id = %person.person_id,
                        account_id = %result.account_id,
                        method = %result.link_method.as_str(),
                        "Successfully linked person to Atlassian account"
                    );
                }
                Ok(None) => {
                    stats.no_match += 1;
                    tracing::debug!(
                        person_id = %person.person_id,
                        "No Atlassian account match found for person"
                    );
                }
                Err(e) => {
                    stats.errors += 1;
                    tracing::warn!(
                        person_id = %person.person_id,
                        error = %e,
                        "Error linking person to Atlassian account"
                    );
                }
            }
        }

        // Record stats in database
        self.record_link_stats(&stats, "sync_job").await?;

        tracing::info!(
            linked_local_id = stats.linked_by_local_id,
            linked_email = stats.linked_by_email,
            linked_name = stats.linked_by_name,
            no_match = stats.no_match,
            errors = stats.errors,
            "Atlassian link matching completed"
        );

        Ok(stats)
    }

    /// Link a single person using the three-step matching strategy.
    /// Returns None if no Atlassian account could be found.
    pub async fn link_person_by_matching(
        &self,
        person_id: &str,
    ) -> Result<Option<LinkResult>, ServiceError> {
        // Step 1: Try matching by local_id
        if let Some(result) = self.link_person_by_local_id(person_id).await? {
            return Ok(Some(result));
        }

        // Step 2: Try matching by email (fallback)
        if let Some(result) = self.link_person_by_email(person_id).await? {
            return Ok(Some(result));
        }

        // Step 3: Try matching by name (final fallback)
        if let Some(result) = self.link_person_by_name(person_id).await? {
            return Ok(Some(result));
        }

        // No match found
        Ok(None)
    }

    /// Step 1: match persons.local_id against atlassian_users_cache.email
    pub async fn link_person_by_local_id(
        &self,
        person_id: &str,
    ) -> Result<Option<LinkResult>, ServiceError> {
        // Get person's local_id
        let person = sqlx::query!(
            r#"
            SELECT local_id
            FROM persons
            WHERE person_id = $1
              AND local_id IS NOT NULL
              AND atlassian_account_id IS NULL
            "#,
            person_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        let local_id = match person {
            Some(p) => p.local_id.ok_or_else(|| {
                ServiceError::NotFound(format!("Person {} has no local_id", person_id))
            })?,
            None => return Ok(None),
        };

        // Find matching Atlassian account
        let matches = sqlx::query!(
            r#"
            SELECT account_id
            FROM atlassian_users_cache
            WHERE LOWER(email) = LOWER($1)
              AND active = true
            "#,
            local_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        match matches.len() {
            0 => Ok(None),
            1 => {
                let account_id = &matches[0].account_id;
                self.link_person_internal(person_id, account_id, LinkMethod::AutoLocalId, "system")
                    .await?;
                Ok(Some(LinkResult {
                    person_id: person_id.to_string(),
                    account_id: account_id.clone(),
                    link_method: LinkMethod::AutoLocalId,
                    linked_at: Utc::now(),
                }))
            }
            _ => {
                tracing::warn!(
                    person_id = %person_id,
                    local_id = %local_id,
                    matches = matches.len(),
                    "Ambiguous Atlassian email match on local_id, skipping auto-link"
                );
                Ok(None)
            }
        }
    }

    /// Step 2: match persons.email against atlassian_users_cache.email (fallback)
    pub async fn link_person_by_email(
        &self,
        person_id: &str,
    ) -> Result<Option<LinkResult>, ServiceError> {
        // Get person's email
        let person = sqlx::query!(
            r#"
            SELECT email
            FROM persons
            WHERE person_id = $1
              AND atlassian_account_id IS NULL
            "#,
            person_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        let email = match person {
            Some(p) => p.email,
            None => return Ok(None),
        };

        // Find matching Atlassian account
        let matches = sqlx::query!(
            r#"
            SELECT account_id
            FROM atlassian_users_cache
            WHERE LOWER(email) = LOWER($1)
              AND active = true
            "#,
            email
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        match matches.len() {
            0 => Ok(None),
            1 => {
                let account_id = &matches[0].account_id;
                self.link_person_internal(person_id, account_id, LinkMethod::AutoEmail, "system")
                    .await?;
                Ok(Some(LinkResult {
                    person_id: person_id.to_string(),
                    account_id: account_id.clone(),
                    link_method: LinkMethod::AutoEmail,
                    linked_at: Utc::now(),
                }))
            }
            _ => {
                tracing::warn!(
                    person_id = %person_id,
                    email = %email,
                    matches = matches.len(),
                    "Ambiguous Atlassian email match, skipping auto-link"
                );
                Ok(None)
            }
        }
    }

    /// Step 3: match persons name against atlassian_users_cache.display_name (final fallback)
    pub async fn link_person_by_name(
        &self,
        person_id: &str,
    ) -> Result<Option<LinkResult>, ServiceError> {
        // Get person's first and last name
        let person = sqlx::query!(
            r#"
            SELECT first_name, last_name
            FROM persons
            WHERE person_id = $1
              AND atlassian_account_id IS NULL
              AND first_name IS NOT NULL
              AND last_name IS NOT NULL
            "#,
            person_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        let (first_name, last_name) = match person {
            Some(p) => (p.first_name, p.last_name),
            None => return Ok(None),
        };

        // Try multiple name formats for matching:
        // 1. "FirstName LastName" (e.g., "John Smith")
        // 2. "LastName, FirstName" (e.g., "Smith, John")
        // 3. "FirstName.LastName" (e.g., "John.Smith")
        // 4. "LastName FirstName" (reversed, e.g., "Smith John")

        let name_format1 = format!("{} {}", first_name, last_name);
        let name_format2 = format!("{}, {}", last_name, first_name);
        let name_format3 = format!("{}.{}", first_name, last_name);
        let name_format4 = format!("{} {}", last_name, first_name);

        // Try to find matching Atlassian account using any of the name formats
        let matches = sqlx::query!(
            r#"
            SELECT account_id, display_name
            FROM atlassian_users_cache
            WHERE active = true
              AND (
                  LOWER(display_name) = LOWER($1)
                  OR LOWER(display_name) = LOWER($2)
                  OR LOWER(display_name) = LOWER($3)
                  OR LOWER(display_name) = LOWER($4)
              )
            "#,
            name_format1,
            name_format2,
            name_format3,
            name_format4
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        match matches.len() {
            0 => Ok(None),
            1 => {
                let account_id = &matches[0].account_id;
                let display_name = &matches[0].display_name;

                self.link_person_internal(person_id, account_id, LinkMethod::AutoName, "system")
                    .await?;

                tracing::info!(
                    person_id = %person_id,
                    account_id = %account_id,
                    display_name = %display_name,
                    "Matched person to Atlassian account by name"
                );

                Ok(Some(LinkResult {
                    person_id: person_id.to_string(),
                    account_id: account_id.clone(),
                    link_method: LinkMethod::AutoName,
                    linked_at: Utc::now(),
                }))
            }
            _ => {
                tracing::warn!(
                    person_id = %person_id,
                    first_name = %first_name,
                    last_name = %last_name,
                    matches = matches.len(),
                    "Ambiguous Atlassian name match, skipping auto-link"
                );
                Ok(None)
            }
        }
    }

    /// Manually link a person to a specific Atlassian account_id
    pub async fn link_person_manual(
        &self,
        person_id: &str,
        account_id: &str,
        linked_by: &str,
    ) -> Result<LinkResult, ServiceError> {
        // Validate that account_id exists in cache
        let account_exists = sqlx::query!(
            r#"
            SELECT account_id
            FROM atlassian_users_cache
            WHERE account_id = $1
            "#,
            account_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        if account_exists.is_none() {
            return Err(ServiceError::NotFound(format!(
                "Atlassian account {} not found in cache",
                account_id
            )));
        }

        // Check if account is already linked to another person
        let existing_link = sqlx::query!(
            r#"
            SELECT person_id
            FROM persons
            WHERE atlassian_account_id = $1
              AND person_id != $2
            "#,
            account_id,
            person_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        if let Some(existing) = existing_link {
            return Err(ServiceError::Conflict(format!(
                "Atlassian account {} is already linked to person {}",
                account_id, existing.person_id
            )));
        }

        // Link the person
        self.link_person_internal(person_id, account_id, LinkMethod::Manual, linked_by)
            .await?;

        Ok(LinkResult {
            person_id: person_id.to_string(),
            account_id: account_id.to_string(),
            link_method: LinkMethod::Manual,
            linked_at: Utc::now(),
        })
    }

    /// Internal method to update the link in the database
    async fn link_person_internal(
        &self,
        person_id: &str,
        account_id: &str,
        link_method: LinkMethod,
        linked_by: &str,
    ) -> Result<(), ServiceError> {
        let link_status = link_method.as_status();
        let method_str = link_method.as_str();
        let linked_at = Utc::now();

        // Update persons table
        sqlx::query!(
            r#"
            UPDATE persons
            SET
                atlassian_account_id = $1,
                atlassian_link_status = $2,
                atlassian_linked_at = $3,
                atlassian_link_method = $4,
                updated_at = NOW()
            WHERE person_id = $5
            "#,
            account_id,
            link_status,
            linked_at,
            method_str,
            person_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        // Record in audit log
        sqlx::query!(
            r#"
            INSERT INTO atlassian_link_audit (
                person_id, account_id, action, link_method, performed_by, performed_at
            )
            VALUES ($1, $2, 'linked', $3, $4, $5)
            "#,
            person_id,
            account_id,
            method_str,
            linked_by,
            linked_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Remove the Atlassian link from a person
    pub async fn unlink_person(&self, person_id: &str) -> Result<(), ServiceError> {
        let person = sqlx::query!(
            r#"
            SELECT atlassian_account_id
            FROM persons
            WHERE person_id = $1
            "#,
            person_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        if let Some(p) = person {
            let account_id = p.atlassian_account_id;

            // Update persons table
            sqlx::query!(
                r#"
                UPDATE persons
                SET
                    atlassian_account_id = NULL,
                    atlassian_link_status = 'unlinked',
                    atlassian_linked_at = NULL,
                    atlassian_link_method = NULL,
                    updated_at = NOW()
                WHERE person_id = $1
                "#,
                person_id
            )
            .execute(&self.pool)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

            // Record in audit log
            sqlx::query!(
                r#"
                INSERT INTO atlassian_link_audit (
                    person_id, account_id, action, performed_by, performed_at
                )
                VALUES ($1, $2, 'unlinked', 'system', NOW())
                "#,
                person_id,
                account_id
            )
            .execute(&self.pool)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

            tracing::info!(
                person_id = %person_id,
                account_id = %account_id.unwrap_or_default(),
                "Unlinked person from Atlassian account"
            );
        }

        Ok(())
    }

    /// Get person's Atlassian link status including profile details
    pub async fn get_person_atlassian_link(
        &self,
        person_id: &str,
    ) -> Result<PersonAtlassianLink, ServiceError> {
        let person = sqlx::query!(
            r#"
            SELECT
                p.person_id,
                p.atlassian_account_id,
                p.atlassian_link_status,
                p.atlassian_linked_at,
                p.atlassian_link_method,
                a.account_id as "account_id?",
                a.display_name as "display_name?",
                a.email,
                a.account_status,
                a.active as "active?",
                a.last_active,
                a.access_billable as "access_billable?",
                a.product_access
            FROM persons p
            LEFT JOIN atlassian_users_cache a ON p.atlassian_account_id = a.account_id
            WHERE p.person_id = $1
            "#,
            person_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        match person {
            Some(p) => {
                let profile = if let Some(account_id) = p.account_id {
                    Some(AtlassianUserProfile {
                        account_id,
                        display_name: p.display_name.unwrap_or_default(),
                        email: p.email,
                        account_status: p.account_status,
                        active: p.active.unwrap_or(false),
                        last_active: p.last_active,
                        access_billable: p.access_billable,
                        product_access: p.product_access,
                    })
                } else {
                    None
                };

                Ok(PersonAtlassianLink {
                    person_id: p.person_id,
                    atlassian_account_id: p.atlassian_account_id,
                    atlassian_link_status: p.atlassian_link_status,
                    atlassian_linked_at: p.atlassian_linked_at,
                    atlassian_link_method: p.atlassian_link_method,
                    atlassian_profile: profile,
                })
            }
            None => Err(ServiceError::NotFound(format!(
                "Person {} not found",
                person_id
            ))),
        }
    }

    /// Retrieve unlinked Atlassian accounts (in cache but not linked to any person)
    pub async fn get_unlinked_atlassian_users(
        &self,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<UnlinkedAtlassianUser>, i64), ServiceError> {
        let offset = (page - 1) * per_page;

        let users = sqlx::query_as!(
            UnlinkedAtlassianUser,
            r#"
            SELECT
                a.account_id,
                a.display_name,
                a.email,
                a.account_status,
                a.active,
                a.product_access
            FROM atlassian_users_cache a
            LEFT JOIN persons p ON a.account_id = p.atlassian_account_id
            WHERE p.person_id IS NULL
              AND a.active = true
            ORDER BY a.display_name
            LIMIT $1 OFFSET $2
            "#,
            per_page,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        let total = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM atlassian_users_cache a
            LEFT JOIN persons p ON a.account_id = p.atlassian_account_id
            WHERE p.person_id IS NULL
              AND a.active = true
            "#
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        Ok((users, total.count.unwrap_or(0)))
    }

    /// Link an organization to an Atlassian group
    pub async fn link_org_to_group(
        &self,
        org_id: &str,
        group_id: &str,
        linked_by: Option<&str>,
    ) -> Result<(), ServiceError> {
        // Validate that group_id exists in cache
        let group_exists = sqlx::query!(
            r#"
            SELECT group_id
            FROM atlassian_groups_cache
            WHERE group_id = $1
            "#,
            group_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        if group_exists.is_none() {
            return Err(ServiceError::NotFound(format!(
                "Atlassian group {} not found in cache",
                group_id
            )));
        }

        // Insert link
        sqlx::query!(
            r#"
            INSERT INTO organization_atlassian_groups (
                org_id, group_id, link_method, linked_at, linked_by
            )
            VALUES ($1, $2, 'manual', NOW(), $3)
            ON CONFLICT (org_id, group_id) DO NOTHING
            "#,
            org_id,
            group_id,
            linked_by.unwrap_or("system")
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        tracing::info!(
            org_id = %org_id,
            group_id = %group_id,
            "Linked organization to Atlassian group"
        );

        Ok(())
    }

    /// Record link statistics in the database
    async fn record_link_stats(
        &self,
        stats: &LinkStats,
        trigger: &str,
    ) -> Result<(), ServiceError> {
        let total_linked = stats.linked_by_local_id + stats.linked_by_email + stats.linked_by_name;

        sqlx::query!(
            r#"
            INSERT INTO atlassian_link_sync_status (
                run_at, trigger, linked, already_linked, no_match, ambiguous, errors
            )
            VALUES (
                NOW(), $1,
                $2, $3, $4, $5, $6
            )
            "#,
            trigger,
            total_linked as i32,
            stats.already_linked as i32,
            stats.no_match as i32,
            stats.ambiguous as i32,
            stats.errors as i32
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
