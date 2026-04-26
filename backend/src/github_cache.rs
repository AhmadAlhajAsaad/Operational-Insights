//! GitHub Cache Repository (FR-012 / TR-012)
//!
//! Database operations for the three GitHub cache tables:
//!   - `github_users_cache`
//!   - `github_licenses_cache`
//!   - `github_copilot_cache`
//!
//! All queries use the runtime (`sqlx::query`) API to avoid requiring
//! compile-time query cache updates in SQLX_OFFLINE mode.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};

// ============================================================================
// Domain Types
// ============================================================================

/// A GitHub Enterprise user as stored in the local cache.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubCachedUser {
    pub id: String,
    pub login: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub enterprise_role: Option<String>,
    pub organization_name: Option<String>,
    pub team_names: Option<Vec<String>>,
    pub verified_domain_emails: Option<Vec<String>>,
    pub is_active: bool,
    pub synced_at: DateTime<Utc>,
}

/// A snapshot of GitHub Enterprise license consumption.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubCachedLicense {
    pub enterprise_slug: String,
    pub synced_at: DateTime<Utc>,
    pub total_seats_purchased: Option<i32>,
    pub total_seats_consumed: Option<i32>,
    pub ghas_seats_consumed: Option<i32>,
}

/// A Copilot seat assignment as stored in the local cache.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubCachedCopilotSeat {
    pub github_login: String,
    pub seat_type: Option<String>,
    pub is_active: bool,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub last_activity_editor: Option<String>,
    pub assigning_team: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// A GitHub user who is in the cache but not linked to any person.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedGitHubUser {
    pub login: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub enterprise_role: Option<String>,
    pub is_active: bool,
    pub synced_at: DateTime<Utc>,
}

/// Linked person data retrieved from the persons table by github_login.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonInfo {
    pub github_login: String,
    pub person_id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub org_id: Option<String>,
    pub country: Option<String>,
}

// ============================================================================
// Repository
// ============================================================================

/// Handles all database operations for the GitHub cache tables.
pub struct GitHubCacheRepository {
    pool: PgPool,
}

impl GitHubCacheRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // -------------------------------------------------------------------------
    // github_users_cache
    // -------------------------------------------------------------------------

    /// Insert or update a GitHub user in the cache.
    pub async fn upsert_user(&self, user: &GitHubCachedUser) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO github_users_cache
                (id, login, email, name, enterprise_role, organization_name,
                 team_names, verified_domain_emails, is_active, synced_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW())
            ON CONFLICT (id) DO UPDATE SET
                login             = EXCLUDED.login,
                email             = EXCLUDED.email,
                name              = EXCLUDED.name,
                enterprise_role   = EXCLUDED.enterprise_role,
                organization_name = EXCLUDED.organization_name,
                team_names        = EXCLUDED.team_names,
                verified_domain_emails = EXCLUDED.verified_domain_emails,
                is_active         = EXCLUDED.is_active,
                synced_at         = NOW()
            "#,
        )
        .bind(&user.id)
        .bind(&user.login)
        .bind(&user.email)
        .bind(&user.name)
        .bind(&user.enterprise_role)
        .bind(&user.organization_name)
        .bind(&user.team_names)
        .bind(&user.verified_domain_emails)
        .bind(user.is_active)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Mark users whose logins are absent from this sync run as inactive.
    ///
    /// Safety guard: if `active_logins` is empty the function returns 0 without
    /// touching the database to prevent accidental mass-deactivation.
    pub async fn deactivate_missing_users(
        &self,
        active_logins: &[String],
    ) -> Result<u64, sqlx::Error> {
        if active_logins.is_empty() {
            return Ok(0);
        }

        let result = sqlx::query(
            r#"
            UPDATE github_users_cache
            SET is_active = FALSE, synced_at = NOW()
            WHERE login != ALL($1)
              AND is_active = TRUE
            "#,
        )
        .bind(active_logins)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Return `true` when the `github_users_cache` table contains no rows.
    pub async fn is_users_cache_empty(&self) -> Result<bool, sqlx::Error> {
        let row = sqlx::query("SELECT COUNT(*) AS cnt FROM github_users_cache")
            .fetch_one(&self.pool)
            .await?;

        let count: i64 = row.get("cnt");
        Ok(count == 0)
    }

    /// Return the timestamp of the most recent sync across all users.
    pub async fn last_sync_at(&self) -> Result<Option<DateTime<Utc>>, sqlx::Error> {
        let row = sqlx::query("SELECT MAX(synced_at) AS last_sync FROM github_users_cache")
            .fetch_one(&self.pool)
            .await?;

        Ok(row.get("last_sync"))
    }

    /// Return all GitHub logins that are not yet linked to a `persons` record.
    pub async fn get_unlinked_accounts(&self) -> Result<Vec<UnlinkedGitHubUser>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT g.login, g.email, g.name, g.enterprise_role, g.is_active, g.synced_at
            FROM github_users_cache g
            WHERE g.login NOT IN (
                SELECT github_login
                FROM persons
                WHERE github_login IS NOT NULL
            )
            ORDER BY g.login
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| UnlinkedGitHubUser {
                login: r.get("login"),
                email: r.get("email"),
                name: r.get("name"),
                enterprise_role: r.get("enterprise_role"),
                is_active: r.get("is_active"),
                synced_at: r.get("synced_at"),
            })
            .collect())
    }

    // -------------------------------------------------------------------------
    // github_licenses_cache
    // -------------------------------------------------------------------------

    /// Insert a new license snapshot (append-only time-series).
    pub async fn save_license_snapshot(
        &self,
        enterprise_slug: &str,
        total_seats_purchased: Option<i32>,
        total_seats_consumed: Option<i32>,
        ghas_seats_consumed: Option<i32>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO github_licenses_cache
                (enterprise_slug, synced_at, total_seats_purchased,
                 total_seats_consumed, ghas_seats_consumed)
            VALUES ($1, NOW(), $2, $3, $4)
            "#,
        )
        .bind(enterprise_slug)
        .bind(total_seats_purchased)
        .bind(total_seats_consumed)
        .bind(ghas_seats_consumed)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Return the most recent license snapshot for the given enterprise.
    pub async fn get_latest_license_snapshot(
        &self,
        enterprise_slug: &str,
    ) -> Result<Option<GitHubCachedLicense>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT enterprise_slug, synced_at,
                   total_seats_purchased, total_seats_consumed, ghas_seats_consumed
            FROM github_licenses_cache
            WHERE enterprise_slug = $1
            ORDER BY synced_at DESC
            LIMIT 1
            "#,
        )
        .bind(enterprise_slug)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| GitHubCachedLicense {
            enterprise_slug: r.get("enterprise_slug"),
            synced_at: r.get("synced_at"),
            total_seats_purchased: r.get("total_seats_purchased"),
            total_seats_consumed: r.get("total_seats_consumed"),
            ghas_seats_consumed: r.get("ghas_seats_consumed"),
        }))
    }

    // -------------------------------------------------------------------------
    // github_copilot_cache
    // -------------------------------------------------------------------------

    /// Insert or update a Copilot seat assignment.
    pub async fn upsert_copilot_seat(
        &self,
        seat: &GitHubCachedCopilotSeat,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO github_copilot_cache
                (github_login, seat_type, is_active, last_activity_at,
                 last_activity_editor, assigning_team, created_at, updated_at, synced_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())
            ON CONFLICT (github_login) DO UPDATE SET
                seat_type            = EXCLUDED.seat_type,
                is_active            = EXCLUDED.is_active,
                last_activity_at     = EXCLUDED.last_activity_at,
                last_activity_editor = EXCLUDED.last_activity_editor,
                assigning_team       = EXCLUDED.assigning_team,
                updated_at           = EXCLUDED.updated_at,
                synced_at            = NOW()
            "#,
        )
        .bind(&seat.github_login)
        .bind(&seat.seat_type)
        .bind(seat.is_active)
        .bind(seat.last_activity_at)
        .bind(&seat.last_activity_editor)
        .bind(&seat.assigning_team)
        .bind(seat.created_at)
        .bind(seat.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Mark Copilot seats as inactive for logins not present in this sync run.
    pub async fn deactivate_missing_copilot_seats(
        &self,
        active_logins: &[String],
    ) -> Result<u64, sqlx::Error> {
        if active_logins.is_empty() {
            return Ok(0);
        }

        let result = sqlx::query(
            r#"
            UPDATE github_copilot_cache
            SET is_active = FALSE, synced_at = NOW()
            WHERE github_login != ALL($1)
              AND is_active = TRUE
            "#,
        )
        .bind(active_logins)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Retrieve the Copilot seat record for the given GitHub login.
    pub async fn get_copilot_seat(
        &self,
        login: &str,
    ) -> Result<Option<GitHubCachedCopilotSeat>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT github_login, seat_type, is_active, last_activity_at,
                   last_activity_editor, assigning_team, created_at, updated_at
            FROM github_copilot_cache
            WHERE github_login = $1
            "#,
        )
        .bind(login)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| GitHubCachedCopilotSeat {
            github_login: r.get("github_login"),
            seat_type: r.get("seat_type"),
            is_active: r.get("is_active"),
            last_activity_at: r.get("last_activity_at"),
            last_activity_editor: r.get("last_activity_editor"),
            assigning_team: r.get("assigning_team"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    /// Return GitHub user details from the cache for a given login.
    pub async fn get_user_by_login(
        &self,
        login: &str,
    ) -> Result<Option<GitHubCachedUser>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, login, email, name, enterprise_role,
                   organization_name, team_names, verified_domain_emails, is_active, synced_at
            FROM github_users_cache
            WHERE login = $1
            "#,
        )
        .bind(login)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| GitHubCachedUser {
            id: r.get("id"),
            login: r.get("login"),
            email: r.get("email"),
            name: r.get("name"),
            enterprise_role: r.get("enterprise_role"),
            organization_name: r.get("organization_name"),
            team_names: r.get("team_names"),
            verified_domain_emails: r.get("verified_domain_emails"),
            is_active: r.get("is_active"),
            synced_at: r.get("synced_at"),
        }))
    }

    /// Bulk-fetch GitHub users by a list of logins.
    ///
    /// Returns one `GitHubCachedUser` per login found in the cache.
    /// Missing logins are silently omitted (no error).
    /// Used to enrich Copilot seat items with email and name for search.
    pub async fn get_users_by_logins(
        &self,
        logins: &[String],
    ) -> Result<Vec<GitHubCachedUser>, sqlx::Error> {
        if logins.is_empty() {
            return Ok(vec![]);
        }

        // Build a parameterised ANY($1) query.
        let rows = sqlx::query(
            r#"
            SELECT id, login, email, name, enterprise_role,
                   organization_name, team_names, verified_domain_emails, is_active, synced_at
            FROM github_users_cache
            WHERE login = ANY($1)
            "#,
        )
        .bind(logins)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| GitHubCachedUser {
                id: r.get("id"),
                login: r.get("login"),
                email: r.get("email"),
                name: r.get("name"),
                enterprise_role: r.get("enterprise_role"),
                organization_name: r.get("organization_name"),
                team_names: r.get("team_names"),
                verified_domain_emails: r.get("verified_domain_emails"),
                is_active: r.get("is_active"),
                synced_at: r.get("synced_at"),
            })
            .collect())
    }

    /// Look up person data for a batch of GitHub logins using FR-012's
    /// multi-step matching strategy (real-time, no pre-linking required):
    ///
    /// 1. Pre-linked persons (github_login already set by the link service)
    /// 2. person_id via _equans suffix stripping (FR-012 step 1)
    /// 3. local_id matches GitHub verified email (FR-012 step 2)
    /// 4. persons.email matches GitHub verified email (FR-012 step 3)
    /// 5. Manually set github_username field (FR-012 step 4)
    ///
    /// DISTINCT ON ensures one result per login, preferring the
    /// highest-priority match.
    pub async fn get_persons_by_github_logins(
        &self,
        logins: &[String],
    ) -> Result<Vec<PersonInfo>, sqlx::Error> {
        if logins.is_empty() {
            return Ok(vec![]);
        }

        let rows = sqlx::query(
            r#"
            SELECT DISTINCT ON (matched_login)
                matched_login AS github_login,
                person_id, first_name, last_name, org_id, country
            FROM (
                -- Priority 1: Already linked by the link service
                SELECT p.github_login AS matched_login,
                       p.person_id, p.first_name, p.last_name, p.org_id, p.country,
                       1 AS prio
                FROM persons p
                WHERE p.github_login = ANY($1)

                UNION ALL

                -- Priority 2: Real-time person_id match via _equans suffix (FR-012 step 1)
                SELECT l.login AS matched_login,
                       p.person_id, p.first_name, p.last_name, p.org_id, p.country,
                       2 AS prio
                FROM unnest($1::text[]) AS l(login)
                INNER JOIN persons p
                    ON LOWER(p.person_id) = LOWER(LEFT(l.login, LENGTH(l.login) - 7))
                WHERE LENGTH(l.login) > 7
                  AND LOWER(RIGHT(l.login, 7)) = '_equans'
                  AND p.github_login IS NULL
                  AND p.status = 'Active'

                UNION ALL

                -- Priority 3: local_id matches GitHub email (FR-012 step 2)
                SELECT g.login AS matched_login,
                       p.person_id, p.first_name, p.last_name, p.org_id, p.country,
                       3 AS prio
                FROM github_users_cache g
                JOIN persons p ON (
                    LOWER(p.local_id) = LOWER(g.email)
                    OR LOWER(p.local_id) = ANY(
                        SELECT LOWER(unnest(g.verified_domain_emails))
                    )
                )
                WHERE g.login = ANY($1)
                  AND p.github_login IS NULL
                  AND p.status = 'Active'
                  AND g.is_active = TRUE

                UNION ALL

                -- Priority 4: persons.email matches GitHub email (FR-012 step 3)
                SELECT g.login AS matched_login,
                       p.person_id, p.first_name, p.last_name, p.org_id, p.country,
                       4 AS prio
                FROM github_users_cache g
                JOIN persons p ON (
                    LOWER(p.email) = LOWER(g.email)
                    OR LOWER(p.email) = ANY(
                        SELECT LOWER(unnest(g.verified_domain_emails))
                    )
                )
                WHERE g.login = ANY($1)
                  AND p.github_login IS NULL
                  AND p.status = 'Active'
                  AND g.is_active = TRUE

                UNION ALL

                -- Priority 5: Manually set github_username (FR-012 step 4)
                SELECT l.login AS matched_login,
                       p.person_id, p.first_name, p.last_name, p.org_id, p.country,
                       5 AS prio
                FROM unnest($1::text[]) AS l(login)
                INNER JOIN persons p
                    ON LOWER(p.github_username) = LOWER(l.login)
                WHERE p.github_login IS NULL
                  AND p.github_username IS NOT NULL
                  AND p.status = 'Active'
            ) sub
            ORDER BY matched_login, prio
            "#,
        )
        .bind(logins)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| PersonInfo {
                github_login: r.get("github_login"),
                person_id: r.get("person_id"),
                first_name: r.get("first_name"),
                last_name: r.get("last_name"),
                org_id: r.get("org_id"),
                country: r.get("country"),
            })
            .collect())
    }
}
