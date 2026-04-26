//! GitHub Link Service (FR-012 / TR-012)
//!
//! Handles automatic and manual linking of `persons` records to GitHub accounts
//! stored in `github_users_cache`.
//!
//! # Matching strategy (priority order)
//!
//! 1. **`linked_auto_person_id`** — strip `_equans` suffix from the cache
//!    `login` field → the result must equal `persons.person_id`
//!    (e.g. `ABG409_equans` → `ABG409`).
//!
//! 2. **`linked_auto_local_id`** — `persons.local_id` matches
//!    `github_users_cache.email` (case-insensitive).
//!
//! 3. **`linked_auto_email`** — `persons.email` matches
//!    `github_users_cache.email` (case-insensitive, fallback).
//!
//! 4. **`linked_manual_username`** — administrator has pre-filled
//!    `persons.github_username`; this value is looked up in the cache.
//!
//! 5. **`linked_manual`** — administrator picks the account explicitly via UI.
//!
//! # Business rules (BR from FR-012)
//!
//! * One person ↔ one GitHub account (unique constraints on both sides).
//! * A `linked_manual` / `linked_manual_username` link is never overwritten
//!   by the automatic job.
//! * The link is preserved when a person becomes inactive.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};

// ============================================================================
// Error
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum GitHubLinkError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}

impl From<sqlx::Error> for GitHubLinkError {
    fn from(e: sqlx::Error) -> Self {
        GitHubLinkError::Database(e.to_string())
    }
}

// ============================================================================
// Types
// ============================================================================

/// Outcome of a single link operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubLinkResult {
    pub person_id: String,
    pub github_login: String,
    pub link_status: String,
    pub linked_at: DateTime<Utc>,
}

/// Aggregated statistics from a batch link run.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GitHubLinkStats {
    pub linked_by_person_id: u32,
    pub linked_by_local_id: u32,
    pub linked_by_email: u32,
    pub linked_by_username: u32,
    pub already_linked: u32,
    pub no_match: u32,
    pub errors: u32,
}

/// Full GitHub link information for a person (used by the API response).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonGitHubLink {
    pub person_id: String,
    pub github_login: Option<String>,
    pub github_account_id: Option<String>,
    pub github_link_status: String,
    pub github_linked_at: Option<DateTime<Utc>>,
    pub github_linked_by: Option<String>,
    pub github_profile: Option<GitHubUserProfile>,
}

/// GitHub account details surfaced on the person page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUserProfile {
    pub login: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub enterprise_role: Option<String>,
    pub is_active: bool,
    pub has_copilot: bool,
    pub copilot_last_activity_at: Option<DateTime<Utc>>,
    pub synced_at: DateTime<Utc>,
}

// ============================================================================
// Service
// ============================================================================

/// Service responsible for linking persons to GitHub accounts.
pub struct GitHubLinkService {
    pool: PgPool,
}

impl GitHubLinkService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // -------------------------------------------------------------------------
    // Batch linking (called after every sync and after CSV import)
    // -------------------------------------------------------------------------

    /// Attempt to link all persons whose `github_link_status` is `unlinked` or
    /// `no_github_account` using efficient batch SQL.
    ///
    /// Returns aggregated statistics.
    pub async fn link_all_unlinked(&self) -> Result<GitHubLinkStats, GitHubLinkError> {
        let mut stats = GitHubLinkStats::default();

        tracing::info!("Starting batch GitHub link matching for unlinked persons");

        // Step 1: person_id match - person_id + '_equans' = cache.login
        let step1 = sqlx::query(
            r#"
            UPDATE persons p
            SET github_login       = g.login,
                github_account_id  = g.id,
                github_link_status = 'linked_auto_person_id',
                github_linked_at   = NOW(),
                github_linked_by   = 'system',
                updated_at         = NOW()
            FROM github_users_cache g
            WHERE LOWER(g.login) = LOWER(p.person_id || '_equans')
              AND p.github_login IS NULL
              AND p.github_link_status IN ('unlinked', 'no_github_account')
              AND p.status = 'Active'
              AND g.is_active = TRUE
            "#,
        )
        .execute(&self.pool)
        .await?;
        stats.linked_by_person_id = step1.rows_affected() as u32;
        tracing::info!(
            count = stats.linked_by_person_id,
            "Step 1 (person_id) matched"
        );

        // Step 2: local_id match - persons.local_id = cache.email (unique only)
        let step2 = sqlx::query(
            r#"
            UPDATE persons p
            SET github_login       = sub.login,
                github_account_id  = sub.id,
                github_link_status = 'linked_auto_local_id',
                github_linked_at   = NOW(),
                github_linked_by   = 'system',
                updated_at         = NOW()
            FROM (
                SELECT g.id, g.login, g.email, g.verified_domain_emails
                FROM github_users_cache g
                WHERE (g.email IS NOT NULL OR g.verified_domain_emails IS NOT NULL)
                  AND g.is_active = TRUE
            ) sub
            WHERE (LOWER(p.local_id) = LOWER(sub.email)
                   OR LOWER(p.local_id) = ANY(SELECT LOWER(unnest(sub.verified_domain_emails))))
              AND p.github_login IS NULL
              AND p.github_link_status IN ('unlinked', 'no_github_account')
              AND p.status = 'Active'
              AND NOT EXISTS (
                  SELECT 1 FROM github_users_cache g2
                  WHERE (LOWER(g2.email) = LOWER(p.local_id)
                         OR LOWER(p.local_id) = ANY(SELECT LOWER(unnest(g2.verified_domain_emails))))
                    AND g2.id != sub.id
              )
            "#,
        )
        .execute(&self.pool)
        .await?;
        stats.linked_by_local_id = step2.rows_affected() as u32;
        tracing::info!(
            count = stats.linked_by_local_id,
            "Step 2 (local_id) matched"
        );

        // Step 3: email match - persons.email = cache.email (unique only)
        let step3 = sqlx::query(
            r#"
            UPDATE persons p
            SET github_login       = sub.login,
                github_account_id  = sub.id,
                github_link_status = 'linked_auto_email',
                github_linked_at   = NOW(),
                github_linked_by   = 'system',
                updated_at         = NOW()
            FROM (
                SELECT g.id, g.login, g.email, g.verified_domain_emails
                FROM github_users_cache g
                WHERE (g.email IS NOT NULL OR g.verified_domain_emails IS NOT NULL)
                  AND g.is_active = TRUE
            ) sub
            WHERE (LOWER(p.email) = LOWER(sub.email)
                   OR LOWER(p.email) = ANY(SELECT LOWER(unnest(sub.verified_domain_emails))))
              AND p.github_login IS NULL
              AND p.github_link_status IN ('unlinked', 'no_github_account')
              AND p.status = 'Active'
              AND NOT EXISTS (
                  SELECT 1 FROM github_users_cache g2
                  WHERE (LOWER(g2.email) = LOWER(p.email)
                         OR LOWER(p.email) = ANY(SELECT LOWER(unnest(g2.verified_domain_emails))))
                    AND g2.id != sub.id
              )
            "#,
        )
        .execute(&self.pool)
        .await?;
        stats.linked_by_email = step3.rows_affected() as u32;
        tracing::info!(count = stats.linked_by_email, "Step 3 (email) matched");

        // Step 4: github_username match - persons.github_username = cache.login
        let step4 = sqlx::query(
            r#"
            UPDATE persons p
            SET github_login       = g.login,
                github_account_id  = g.id,
                github_link_status = 'linked_manual_username',
                github_linked_at   = NOW(),
                github_linked_by   = 'system',
                updated_at         = NOW()
            FROM github_users_cache g
            WHERE LOWER(g.login) = LOWER(p.github_username)
              AND p.github_login IS NULL
              AND p.github_link_status IN ('unlinked', 'no_github_account')
              AND p.status = 'Active'
              AND p.github_username IS NOT NULL
              AND g.is_active = TRUE
            "#,
        )
        .execute(&self.pool)
        .await?;
        stats.linked_by_username = step4.rows_affected() as u32;
        tracing::info!(
            count = stats.linked_by_username,
            "Step 4 (username) matched"
        );

        // Mark remaining unlinked persons as no_github_account
        let remaining = sqlx::query(
            r#"
            UPDATE persons
            SET github_link_status = 'no_github_account', updated_at = NOW()
            WHERE github_link_status = 'unlinked'
              AND github_login IS NULL
              AND status = 'Active'
            "#,
        )
        .execute(&self.pool)
        .await?;
        stats.no_match = remaining.rows_affected() as u32;

        tracing::info!(
            linked_person_id = stats.linked_by_person_id,
            linked_local_id = stats.linked_by_local_id,
            linked_email = stats.linked_by_email,
            linked_username = stats.linked_by_username,
            no_match = stats.no_match,
            errors = stats.errors,
            "GitHub batch link matching completed"
        );

        Ok(stats)
    }

    // -------------------------------------------------------------------------
    // Single-person matching
    // -------------------------------------------------------------------------

    /// Run the full 4-step automatic matching strategy for one person.
    /// Returns `None` when no GitHub account was found via any automatic step.
    pub async fn link_person_by_matching(
        &self,
        person_id: &str,
    ) -> Result<Option<GitHubLinkResult>, GitHubLinkError> {
        // Step 1: person_id → github_com_login minus _equans
        if let Some(r) = self.link_by_person_id(person_id).await? {
            return Ok(Some(r));
        }

        // Step 2: persons.local_id → cache.email
        if let Some(r) = self.link_by_local_id(person_id).await? {
            return Ok(Some(r));
        }

        // Step 3: persons.email → cache.email
        if let Some(r) = self.link_by_email(person_id).await? {
            return Ok(Some(r));
        }

        // Step 4: manually set persons.github_username → cache.login
        if let Some(r) = self.link_by_username_field(person_id).await? {
            return Ok(Some(r));
        }

        // Mark the person as having no GitHub account in the cache at all
        self.mark_no_github_account(person_id).await?;
        Ok(None)
    }

    // -------------------------------------------------------------------------
    // Step 1: person_id → login-minus-_equans
    // -------------------------------------------------------------------------

    async fn link_by_person_id(
        &self,
        person_id: &str,
    ) -> Result<Option<GitHubLinkResult>, GitHubLinkError> {
        // Build the expected login: person_id + "_equans" (case-insensitive)
        let expected_login = format!("{}_equans", person_id.to_lowercase());

        let row = sqlx::query(
            r#"
            SELECT id, login
            FROM github_users_cache
            WHERE LOWER(login) = $1
            LIMIT 1
            "#,
        )
        .bind(&expected_login)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => {
                let login: String = r.get("login");
                let id: String = r.get("id");
                self.apply_link(person_id, &login, &id, "linked_auto_person_id", "system")
                    .await?;
                Ok(Some(GitHubLinkResult {
                    person_id: person_id.to_string(),
                    github_login: login,
                    link_status: "linked_auto_person_id".to_string(),
                    linked_at: Utc::now(),
                }))
            }
            None => Ok(None),
        }
    }

    // -------------------------------------------------------------------------
    // Step 2: local_id → cache.email
    // -------------------------------------------------------------------------

    async fn link_by_local_id(
        &self,
        person_id: &str,
    ) -> Result<Option<GitHubLinkResult>, GitHubLinkError> {
        let person = sqlx::query(
            r#"
            SELECT local_id
            FROM persons
            WHERE person_id = $1
              AND local_id IS NOT NULL
              AND github_login IS NULL
            "#,
        )
        .bind(person_id)
        .fetch_optional(&self.pool)
        .await?;

        let local_id: String = match person.and_then(|r| r.get("local_id")) {
            Some(v) => v,
            None => return Ok(None),
        };

        let matches = sqlx::query(
            r#"
            SELECT id, login
            FROM github_users_cache
            WHERE LOWER(email) = LOWER($1)
               OR LOWER($1) = ANY(SELECT LOWER(unnest(verified_domain_emails)))
            "#,
        )
        .bind(&local_id)
        .fetch_all(&self.pool)
        .await?;

        match matches.len() {
            0 => Ok(None),
            1 => {
                let login: String = matches[0].get("login");
                let id: String = matches[0].get("id");
                self.apply_link(person_id, &login, &id, "linked_auto_local_id", "system")
                    .await?;
                Ok(Some(GitHubLinkResult {
                    person_id: person_id.to_string(),
                    github_login: login,
                    link_status: "linked_auto_local_id".to_string(),
                    linked_at: Utc::now(),
                }))
            }
            _ => {
                tracing::warn!(
                    person_id = %person_id,
                    local_id = %local_id,
                    matches = matches.len(),
                    "Ambiguous GitHub email match on local_id, skipping auto-link"
                );
                Ok(None)
            }
        }
    }

    // -------------------------------------------------------------------------
    // Step 3: email → cache.email
    // -------------------------------------------------------------------------

    async fn link_by_email(
        &self,
        person_id: &str,
    ) -> Result<Option<GitHubLinkResult>, GitHubLinkError> {
        let person = sqlx::query(
            r#"
            SELECT email
            FROM persons
            WHERE person_id = $1
              AND email IS NOT NULL
              AND github_login IS NULL
            "#,
        )
        .bind(person_id)
        .fetch_optional(&self.pool)
        .await?;

        let email: String = match person.and_then(|r| r.get("email")) {
            Some(v) => v,
            None => return Ok(None),
        };

        let matches = sqlx::query(
            r#"
            SELECT id, login
            FROM github_users_cache
            WHERE LOWER(email) = LOWER($1)
               OR LOWER($1) = ANY(SELECT LOWER(unnest(verified_domain_emails)))
            "#,
        )
        .bind(&email)
        .fetch_all(&self.pool)
        .await?;

        match matches.len() {
            0 => Ok(None),
            1 => {
                let login: String = matches[0].get("login");
                let id: String = matches[0].get("id");
                self.apply_link(person_id, &login, &id, "linked_auto_email", "system")
                    .await?;
                Ok(Some(GitHubLinkResult {
                    person_id: person_id.to_string(),
                    github_login: login,
                    link_status: "linked_auto_email".to_string(),
                    linked_at: Utc::now(),
                }))
            }
            _ => {
                tracing::warn!(
                    person_id = %person_id,
                    email = %email,
                    matches = matches.len(),
                    "Ambiguous GitHub email match, skipping auto-link"
                );
                Ok(None)
            }
        }
    }

    // -------------------------------------------------------------------------
    // Step 4: manually set github_username → cache.login
    // -------------------------------------------------------------------------

    async fn link_by_username_field(
        &self,
        person_id: &str,
    ) -> Result<Option<GitHubLinkResult>, GitHubLinkError> {
        let person = sqlx::query(
            r#"
            SELECT github_username
            FROM persons
            WHERE person_id = $1
              AND github_username IS NOT NULL
              AND github_login IS NULL
            "#,
        )
        .bind(person_id)
        .fetch_optional(&self.pool)
        .await?;

        let username: String = match person.and_then(|r| r.get("github_username")) {
            Some(v) => v,
            None => return Ok(None),
        };

        let row = sqlx::query(
            r#"
            SELECT id, login
            FROM github_users_cache
            WHERE LOWER(login) = LOWER($1)
            LIMIT 1
            "#,
        )
        .bind(&username)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => {
                let login: String = r.get("login");
                let id: String = r.get("id");
                self.apply_link(person_id, &login, &id, "linked_manual_username", "system")
                    .await?;
                Ok(Some(GitHubLinkResult {
                    person_id: person_id.to_string(),
                    github_login: login,
                    link_status: "linked_manual_username".to_string(),
                    linked_at: Utc::now(),
                }))
            }
            None => Ok(None),
        }
    }

    // -------------------------------------------------------------------------
    // Step 5 (API entry point): fully manual link by administrator
    // -------------------------------------------------------------------------

    /// Manually link a person to a specific GitHub login.
    /// Validates uniqueness on both sides before writing.
    pub async fn link_person_manual(
        &self,
        person_id: &str,
        github_login: &str,
        linked_by: &str,
    ) -> Result<GitHubLinkResult, GitHubLinkError> {
        // Validate that the login exists in cache
        let cache_row = sqlx::query("SELECT id, login FROM github_users_cache WHERE login = $1")
            .bind(github_login)
            .fetch_optional(&self.pool)
            .await?;

        let row = cache_row.ok_or_else(|| {
            GitHubLinkError::NotFound(format!(
                "GitHub account '{}' not found in cache",
                github_login
            ))
        })?;

        let cached_id: String = row.get("id");

        // Check that this login is not already linked to another person
        let conflict = sqlx::query(
            r#"
            SELECT person_id FROM persons
            WHERE github_login = $1 AND person_id != $2
            "#,
        )
        .bind(github_login)
        .bind(person_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(other) = conflict {
            let other_id: String = other.get("person_id");
            return Err(GitHubLinkError::Conflict(format!(
                "GitHub account '{}' is already linked to person '{}'",
                github_login, other_id
            )));
        }

        self.apply_link(
            person_id,
            github_login,
            &cached_id,
            "linked_manual",
            linked_by,
        )
        .await?;

        Ok(GitHubLinkResult {
            person_id: person_id.to_string(),
            github_login: github_login.to_string(),
            link_status: "linked_manual".to_string(),
            linked_at: Utc::now(),
        })
    }

    // -------------------------------------------------------------------------
    // Unlink
    // -------------------------------------------------------------------------

    /// Remove the GitHub link from a person.
    pub async fn unlink_person(
        &self,
        person_id: &str,
        unlinked_by: &str,
    ) -> Result<(), GitHubLinkError> {
        let row = sqlx::query("SELECT github_login FROM persons WHERE person_id = $1")
            .bind(person_id)
            .fetch_optional(&self.pool)
            .await?;

        let github_login: Option<String> = row.as_ref().and_then(|r| r.get("github_login"));

        if github_login.is_none() {
            return Err(GitHubLinkError::NotFound(format!(
                "Person '{}' has no GitHub link to remove",
                person_id
            )));
        }

        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE persons
            SET github_login       = NULL,
                github_account_id  = NULL,
                github_link_status = 'unlinked',
                github_linked_at   = NULL,
                github_linked_by   = NULL,
                updated_at         = $1
            WHERE person_id = $2
            "#,
        )
        .bind(now)
        .bind(person_id)
        .execute(&self.pool)
        .await?;

        // Audit log
        self.write_audit(
            person_id,
            github_login.as_deref(),
            "unlinked",
            None,
            unlinked_by,
            None,
        )
        .await?;

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Set github_username field (enables step-4 matching)
    // -------------------------------------------------------------------------

    /// Persist an administrator-provided GitHub username hint on the person
    /// record. This is used in the next link run as step 4.
    pub async fn set_github_username(
        &self,
        person_id: &str,
        github_username: &str,
    ) -> Result<(), GitHubLinkError> {
        // Validate person exists
        let exists = sqlx::query("SELECT 1 FROM persons WHERE person_id = $1")
            .bind(person_id)
            .fetch_optional(&self.pool)
            .await?;

        if exists.is_none() {
            return Err(GitHubLinkError::NotFound(format!(
                "Person '{}' not found",
                person_id
            )));
        }

        sqlx::query(
            r#"
            UPDATE persons
            SET github_username = $1, updated_at = NOW()
            WHERE person_id = $2
            "#,
        )
        .bind(github_username)
        .bind(person_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Query
    // -------------------------------------------------------------------------

    /// Return the full GitHub link information for a person, including the
    /// cached GitHub profile if one is linked.
    pub async fn get_person_github_link(
        &self,
        person_id: &str,
    ) -> Result<PersonGitHubLink, GitHubLinkError> {
        let person = sqlx::query(
            r#"
            SELECT person_id, github_login, github_account_id,
                   github_link_status, github_linked_at, github_linked_by
            FROM persons
            WHERE person_id = $1
            "#,
        )
        .bind(person_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| GitHubLinkError::NotFound(format!("Person '{}' not found", person_id)))?;

        let github_login: Option<String> = person.get("github_login");

        let github_profile = if let Some(ref login) = github_login {
            self.build_github_profile(login).await?
        } else {
            None
        };

        Ok(PersonGitHubLink {
            person_id: person.get("person_id"),
            github_login,
            github_account_id: person.get("github_account_id"),
            github_link_status: person.get("github_link_status"),
            github_linked_at: person.get("github_linked_at"),
            github_linked_by: person.get("github_linked_by"),
            github_profile,
        })
    }

    // -------------------------------------------------------------------------
    // Organization GitHub info
    // -------------------------------------------------------------------------

    /// Return aggregate GitHub license info for an organization, computed
    /// from the persons linked to that org and the cache tables.
    pub async fn get_org_github_info(
        &self,
        org_id: &str,
    ) -> Result<OrgGitHubInfo, GitHubLinkError> {
        // Count linked persons in this org
        let counts = sqlx::query(
            r#"
            SELECT
                COUNT(*) FILTER (WHERE github_login IS NOT NULL) AS linked_count,
                COUNT(*) FILTER (WHERE github_login IS NULL)     AS unlinked_count
            FROM persons
            WHERE org_id = $1
            "#,
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        let linked_count: i64 = counts.get("linked_count");
        let unlinked_count: i64 = counts.get("unlinked_count");

        // Count Copilot seats for persons in this org
        let copilot_count: i64 = sqlx::query(
            r#"
            SELECT COUNT(*) AS cnt
            FROM github_copilot_cache cc
            JOIN persons p ON p.github_login = cc.github_login
            WHERE p.org_id = $1 AND cc.is_active = TRUE
            "#,
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await
        .map(|r| r.get("cnt"))
        .unwrap_or(0);

        // Fetch configured org names from organizations table
        let org_row = sqlx::query(
            "SELECT github_org_names, github_team_names FROM organizations WHERE org_id = $1",
        )
        .bind(org_id)
        .fetch_optional(&self.pool)
        .await?;

        let github_org_names: Option<Vec<String>> =
            org_row.as_ref().and_then(|r| r.get("github_org_names"));
        let github_team_names: Option<Vec<String>> =
            org_row.as_ref().and_then(|r| r.get("github_team_names"));

        Ok(OrgGitHubInfo {
            org_id: org_id.to_string(),
            github_org_names: github_org_names.unwrap_or_default(),
            github_team_names: github_team_names.unwrap_or_default(),
            linked_persons: linked_count,
            unlinked_persons: unlinked_count,
            copilot_seats: copilot_count,
        })
    }

    /// Set GitHub org/team names on an organization.
    pub async fn set_org_github_links(
        &self,
        org_id: &str,
        org_names: Vec<String>,
        team_names: Vec<String>,
    ) -> Result<(), GitHubLinkError> {
        let exists = sqlx::query("SELECT 1 FROM organizations WHERE org_id = $1")
            .bind(org_id)
            .fetch_optional(&self.pool)
            .await?;

        if exists.is_none() {
            return Err(GitHubLinkError::NotFound(format!(
                "Organization '{}' not found",
                org_id
            )));
        }

        sqlx::query(
            r#"
            UPDATE organizations
            SET github_org_names  = $1,
                github_team_names = $2,
                updated_at        = NOW()
            WHERE org_id = $3
            "#,
        )
        .bind(&org_names)
        .bind(&team_names)
        .bind(org_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Internal helpers
    // -------------------------------------------------------------------------

    /// Write the link fields to `persons` and record in `github_link_audit`.
    async fn apply_link(
        &self,
        person_id: &str,
        github_login: &str,
        github_account_id: &str,
        link_status: &str,
        linked_by: &str,
    ) -> Result<(), GitHubLinkError> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE persons
            SET github_login       = $1,
                github_account_id  = $2,
                github_link_status = $3,
                github_linked_at   = $4,
                github_linked_by   = $5,
                updated_at         = $4
            WHERE person_id = $6
            "#,
        )
        .bind(github_login)
        .bind(github_account_id)
        .bind(link_status)
        .bind(now)
        .bind(linked_by)
        .bind(person_id)
        .execute(&self.pool)
        .await?;

        self.write_audit(
            person_id,
            Some(github_login),
            "linked",
            Some(link_status),
            linked_by,
            None,
        )
        .await?;

        Ok(())
    }

    /// Mark a person as having no matching GitHub account in the cache.
    async fn mark_no_github_account(&self, person_id: &str) -> Result<(), GitHubLinkError> {
        sqlx::query(
            r#"
            UPDATE persons
            SET github_link_status = 'no_github_account', updated_at = NOW()
            WHERE person_id = $1
              AND github_login IS NULL
            "#,
        )
        .bind(person_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Build an in-memory `GitHubUserProfile` from the cache tables.
    async fn build_github_profile(
        &self,
        login: &str,
    ) -> Result<Option<GitHubUserProfile>, GitHubLinkError> {
        let user = sqlx::query(
            r#"
            SELECT login, email, name, enterprise_role, is_active, synced_at
            FROM github_users_cache
            WHERE login = $1
            "#,
        )
        .bind(login)
        .fetch_optional(&self.pool)
        .await?;

        let user = match user {
            Some(r) => r,
            None => return Ok(None),
        };

        let copilot = sqlx::query(
            r#"
            SELECT is_active, last_activity_at
            FROM github_copilot_cache
            WHERE github_login = $1
            "#,
        )
        .bind(login)
        .fetch_optional(&self.pool)
        .await?;

        let (has_copilot, copilot_last_activity_at) = match copilot {
            Some(r) => (r.get::<bool, _>("is_active"), r.get("last_activity_at")),
            None => (false, None),
        };

        Ok(Some(GitHubUserProfile {
            login: user.get("login"),
            email: user.get("email"),
            name: user.get("name"),
            enterprise_role: user.get("enterprise_role"),
            is_active: user.get("is_active"),
            has_copilot,
            copilot_last_activity_at,
            synced_at: user.get("synced_at"),
        }))
    }

    /// Insert one row into `github_link_audit`.
    async fn write_audit(
        &self,
        person_id: &str,
        github_login: Option<&str>,
        action: &str,
        method: Option<&str>,
        performed_by: &str,
        details: Option<serde_json::Value>,
    ) -> Result<(), GitHubLinkError> {
        sqlx::query(
            r#"
            INSERT INTO github_link_audit
                (person_id, github_login, action, method, performed_by, details, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW())
            "#,
        )
        .bind(person_id)
        .bind(github_login)
        .bind(action)
        .bind(method)
        .bind(performed_by)
        .bind(details)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

// ============================================================================
// Additional response types for the organization endpoint
// ============================================================================

/// Aggregated GitHub data for an organization page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgGitHubInfo {
    pub org_id: String,
    pub github_org_names: Vec<String>,
    pub github_team_names: Vec<String>,
    pub linked_persons: i64,
    pub unlinked_persons: i64,
    pub copilot_seats: i64,
}

/// Request body for the organization GitHub PUT endpoint.
#[derive(Debug, Deserialize)]
pub struct SetOrgGitHubLinksRequest {
    pub org_names: Vec<String>,
    pub team_names: Vec<String>,
}

/// Request body for the manual person link endpoint.
#[derive(Debug, Deserialize)]
pub struct ManualLinkRequest {
    pub github_login: String,
}

/// Request body for setting the github_username hint.
#[derive(Debug, Deserialize)]
pub struct SetUsernameRequest {
    pub username: String,
}
