//! Organization repository for database operations (FR-006)

use crate::organizations::types::*;
use sqlx::PgPool;
use std::collections::HashMap;

/// Organization repository
pub struct OrganizationRepository {
    pool: PgPool,
}

impl OrganizationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get all organizations with pagination
    pub async fn list(
        &self,
        params: &OrganizationListParams,
    ) -> Result<(Vec<OrganizationSummary>, i64), sqlx::Error> {
        let page = params.page.unwrap_or(1).max(1);
        let per_page = params.per_page.unwrap_or(25).min(100);
        let offset = (page - 1) * per_page;

        let mut conditions = vec!["1=1".to_string()];

        if let Some(ref status) = params.status {
            conditions.push(format!("o.status = '{}'", status.replace('\'', "''")));
        }
        if let Some(ref search) = params.search {
            if search.len() >= 2 {
                let s = search.replace('\'', "''");
                conditions.push(format!(
                    "(o.org_id ILIKE '%{}%' OR o.name ILIKE '%{}%')",
                    s, s
                ));
            }
        }

        let where_clause = conditions.join(" AND ");
        let sort_by = params.sort_by.as_deref().unwrap_or("org_id");
        let sort_order = if params.sort_order.as_deref() == Some("DESC") {
            "DESC"
        } else {
            "ASC"
        };

        let count_query = format!(
            "SELECT COUNT(*) FROM organizations o WHERE {}",
            where_clause
        );
        let count: (i64,) = sqlx::query_as(&count_query).fetch_one(&self.pool).await?;

        let data_query = format!(
            r#"
            SELECT
                o.org_id,
                o.name,
                o.status,
                COALESCE(
                    (SELECT country FROM persons WHERE org_id = o.org_id GROUP BY country ORDER BY COUNT(*) DESC LIMIT 1),
                    NULL
                ) as primary_country,
                COALESCE((SELECT COUNT(*) FROM persons WHERE org_id = o.org_id), 0) as person_count,
                COALESCE((SELECT COUNT(DISTINCT country) FROM persons WHERE org_id = o.org_id), 0) as country_count
            FROM organizations o
            WHERE {}
            ORDER BY {} {}
            LIMIT {} OFFSET {}
        "#,
            where_clause, sort_by, sort_order, per_page, offset
        );

        let rows: Vec<(String, String, String, Option<String>, i64, i64)> =
            sqlx::query_as(&data_query).fetch_all(&self.pool).await?;

        let summaries: Vec<OrganizationSummary> = rows
            .into_iter()
            .map(|r| OrganizationSummary {
                org_id: r.0,
                name: r.1,
                status: r.2,
                primary_country: r.3,
                person_count: r.4,
                country_count: r.5,
            })
            .collect();

        Ok((summaries, count.0))
    }

    /// Get organization by ID with full details
    pub async fn get_by_id(&self, org_id: &str) -> Result<Option<Organization>, sqlx::Error> {
        sqlx::query_as::<_, Organization>("SELECT * FROM organizations WHERE org_id = $1")
            .bind(org_id)
            .fetch_optional(&self.pool)
            .await
    }

    /// Get organization detail with related data
    pub async fn get_detail(
        &self,
        org_id: &str,
    ) -> Result<Option<OrganizationDetail>, sqlx::Error> {
        let org = self.get_by_id(org_id).await?;

        let Some(org) = org else {
            return Ok(None);
        };

        // Get person count
        let person_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM persons WHERE org_id = $1")
            .bind(org_id)
            .fetch_one(&self.pool)
            .await?;

        // Get children
        let children_rows: Vec<(String, String, i64)> = sqlx::query_as(
            r#"
            SELECT
                o.org_id,
                o.name,
                COALESCE((SELECT COUNT(*) FROM persons WHERE org_id = o.org_id), 0) as person_count
            FROM organizations o
            WHERE o.parent_org_id = $1
            ORDER BY o.name
        "#,
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await?;

        let children: Vec<OrganizationChild> = children_rows
            .into_iter()
            .map(|r| OrganizationChild {
                org_id: r.0,
                name: r.1,
                person_count: r.2,
            })
            .collect();

        // Get country distribution
        let country_rows: Vec<(String, i64)> = sqlx::query_as(
            r#"
            SELECT country, COUNT(*) as count
            FROM persons
            WHERE org_id = $1 AND country IS NOT NULL
            GROUP BY country
            ORDER BY count DESC
        "#,
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await?;

        let total_persons = person_count.0 as f64;
        let country_distribution: Vec<CountryDistribution> = country_rows
            .into_iter()
            .map(|(country, count)| CountryDistribution {
                country,
                count,
                percentage: if total_persons > 0.0 {
                    (count as f64 / total_persons) * 100.0
                } else {
                    0.0
                },
            })
            .collect();

        // Get billing location distribution
        let billing_rows: Vec<(String, i64)> = sqlx::query_as(
            r#"
            SELECT billing_location, COUNT(*) as count
            FROM persons
            WHERE org_id = $1 AND billing_location IS NOT NULL
            GROUP BY billing_location
            ORDER BY count DESC
        "#,
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await?;

        let billing_location_distribution: Vec<BillingLocationDistribution> = billing_rows
            .into_iter()
            .map(|(billing_location, count)| BillingLocationDistribution {
                billing_location,
                count,
                percentage: if total_persons > 0.0 {
                    (count as f64 / total_persons) * 100.0
                } else {
                    0.0
                },
            })
            .collect();
        Ok(Some(OrganizationDetail {
            org_id: org.org_id,
            name: org.name,
            description: org.description,
            parent_org_id: org.parent_org_id,
            cost_center: org.cost_center,
            manager: org.manager,
            budget: org.budget.map(|b| b.to_string().parse().unwrap_or(0.0)),
            org_type: org.org_type,
            status: org.status,
            person_count: person_count.0,
            children,
            country_distribution,
            billing_location_distribution,
            created_at: org.created_at,
            updated_at: org.updated_at,
        }))
    }

    /// Create a new organization
    pub async fn create(
        &self,
        req: &CreateOrganizationRequest,
    ) -> Result<Organization, sqlx::Error> {
        sqlx::query_as::<_, Organization>(r#"
            INSERT INTO organizations (org_id, name, description, parent_org_id, cost_center, manager, budget, org_type, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7::numeric, $8, $9)
            RETURNING *
        "#)
        .bind(&req.org_id)
        .bind(&req.name)
        .bind(&req.description)
        .bind(&req.parent_org_id)
        .bind(&req.cost_center)
        .bind(&req.manager)
        .bind(req.budget)
        .bind(req.org_type.as_deref().unwrap_or("Business Unit"))
        .bind(req.status.as_deref().unwrap_or("Active"))
        .fetch_one(&self.pool)
        .await
    }

    /// Get organization tree structure
    pub async fn get_tree(&self) -> Result<Vec<OrganizationTreeNode>, sqlx::Error> {
        let rows: Vec<(String, String, Option<String>, i64)> = sqlx::query_as(
            r#"
            SELECT
                o.org_id,
                o.name,
                o.parent_org_id,
                COALESCE((SELECT COUNT(*) FROM persons WHERE org_id = o.org_id), 0) as person_count
            FROM organizations o
            ORDER BY o.name
        "#,
        )
        .fetch_all(&self.pool)
        .await?;

        // Build tree structure
        let mut nodes: HashMap<String, OrganizationTreeNode> = HashMap::new();
        let mut children_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut root_ids: Vec<String> = Vec::new();

        for (org_id, name, parent_org_id, person_count) in rows {
            nodes.insert(
                org_id.clone(),
                OrganizationTreeNode {
                    org_id: org_id.clone(),
                    name,
                    person_count,
                    children: vec![],
                },
            );

            if let Some(parent) = parent_org_id {
                children_map.entry(parent).or_default().push(org_id);
            } else {
                root_ids.push(org_id);
            }
        }

        fn build_subtree(
            org_id: &str,
            nodes: &HashMap<String, OrganizationTreeNode>,
            children_map: &HashMap<String, Vec<String>>,
        ) -> Option<OrganizationTreeNode> {
            let node = nodes.get(org_id)?;
            let children: Vec<OrganizationTreeNode> = children_map
                .get(org_id)
                .map(|child_ids| {
                    child_ids
                        .iter()
                        .filter_map(|id| build_subtree(id, nodes, children_map))
                        .collect()
                })
                .unwrap_or_default();

            Some(OrganizationTreeNode {
                org_id: node.org_id.clone(),
                name: node.name.clone(),
                person_count: node.person_count,
                children,
            })
        }

        let tree: Vec<OrganizationTreeNode> = root_ids
            .iter()
            .filter_map(|id| build_subtree(id, &nodes, &children_map))
            .collect();

        Ok(tree)
    }

    /// Get Atlassian product user counts for an organization
    /// Returns how many persons in the org have access to each Atlassian product
    pub async fn get_atlassian_product_counts(
        &self,
        org_id: &str,
    ) -> Result<Vec<(String, i64)>, sqlx::Error> {
        let rows: Vec<(String, i64)> = sqlx::query_as(
            r#"
            SELECT
                product_elem->>'key' AS product_key,
                COUNT(DISTINCT pers.person_id) AS user_count
            FROM persons pers
            JOIN atlassian_users_cache a ON pers.atlassian_account_id = a.account_id
            CROSS JOIN LATERAL jsonb_array_elements(
                COALESCE(a.product_access, '[]'::jsonb)
            ) AS product_elem
            WHERE pers.org_id = $1
              AND a.product_access IS NOT NULL
              AND jsonb_array_length(a.product_access) > 0
              AND product_elem->>'key' IS NOT NULL
            GROUP BY product_elem->>'key'
            ORDER BY user_count DESC
        "#,
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    /// Get statistics
    pub async fn get_stats(&self) -> Result<OrganizationStats, sqlx::Error> {
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM organizations")
            .fetch_one(&self.pool)
            .await?;

        let active: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM organizations WHERE status = 'Active'")
                .fetch_one(&self.pool)
                .await?;

        let persons: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM persons")
            .fetch_one(&self.pool)
            .await?;

        let countries: (i64,) =
            sqlx::query_as("SELECT COUNT(DISTINCT country) FROM persons WHERE country IS NOT NULL")
                .fetch_one(&self.pool)
                .await?;

        Ok(OrganizationStats {
            total: total.0,
            active: active.0,
            total_persons: persons.0,
            countries: countries.0,
        })
    }

    /// Get GitHub product user counts for an organization
    pub async fn get_github_product_counts(
        &self,
        org_id: &str,
    ) -> Result<Vec<(String, i64)>, sqlx::Error> {
        let license_count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(DISTINCT p.person_id)
            FROM persons p
            JOIN github_users_cache g ON p.github_login = g.login
            WHERE p.org_id = $1
              AND p.github_login IS NOT NULL
              AND g.is_active = true
            "#,
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        let copilot_count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(DISTINCT p.person_id)
            FROM persons p
            JOIN github_copilot_cache c ON p.github_login = c.github_login
            WHERE p.org_id = $1
              AND p.github_login IS NOT NULL
              AND c.is_active = true
            "#,
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        // GHAS: proportional share of enterprise-wide GHAS committers for this org
        let ghas_count: (i64,) = sqlx::query_as(
            r#"
            SELECT COALESCE(
                (SELECT ghas_seats_consumed FROM github_licenses_cache
                 ORDER BY synced_at DESC LIMIT 1), 0
            )::bigint *
            GREATEST(
                (SELECT COUNT(DISTINCT p2.person_id)
                 FROM persons p2
                 JOIN github_users_cache g2 ON p2.github_login = g2.login
                 WHERE p2.org_id = $1
                   AND p2.github_login IS NOT NULL
                   AND g2.is_active = true), 0
            ) /
            GREATEST(
                (SELECT COUNT(DISTINCT p3.person_id)
                 FROM persons p3
                 JOIN github_users_cache g3 ON p3.github_login = g3.login
                 WHERE p3.github_login IS NOT NULL
                   AND g3.is_active = true), 1
            )
            "#,
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        let mut results = Vec::new();
        if license_count.0 > 0 {
            results.push(("license".to_string(), license_count.0));
        }
        if copilot_count.0 > 0 {
            results.push(("copilot".to_string(), copilot_count.0));
        }
        if ghas_count.0 > 0 {
            results.push(("ghas".to_string(), ghas_count.0));
        }

        Ok(results)
    }

    /// Get billing location distribution across all persons (global aggregation)
    pub async fn get_global_billing_location_distribution(
        &self,
    ) -> Result<Vec<BillingLocationDistribution>, sqlx::Error> {
        let total: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM persons WHERE billing_location IS NOT NULL")
                .fetch_one(&self.pool)
                .await?;

        let rows: Vec<(String, i64)> = sqlx::query_as(
            r#"
            SELECT billing_location, COUNT(*) as count
            FROM persons
            WHERE billing_location IS NOT NULL
            GROUP BY billing_location
            ORDER BY count DESC
        "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let total_persons = total.0 as f64;
        Ok(rows
            .into_iter()
            .map(|(billing_location, count)| BillingLocationDistribution {
                billing_location,
                count,
                percentage: if total_persons > 0.0 {
                    (count as f64 / total_persons) * 100.0
                } else {
                    0.0
                },
            })
            .collect())
    }

    /// Get global Atlassian product user counts (total across all users, not org-scoped)
    /// Used as fallback when person-org linking is incomplete
    pub async fn get_global_atlassian_product_counts(
        &self,
    ) -> Result<Vec<(String, i64)>, sqlx::Error> {
        let rows: Vec<(String, i64)> = sqlx::query_as(
            r#"
            SELECT
                product_elem->>'key' AS product_key,
                COUNT(DISTINCT a.account_id) AS user_count
            FROM atlassian_users_cache a
            CROSS JOIN LATERAL jsonb_array_elements(
                COALESCE(a.product_access, '[]'::jsonb)
            ) AS product_elem
            WHERE a.product_access IS NOT NULL
              AND jsonb_array_length(a.product_access) > 0
              AND product_elem->>'key' IS NOT NULL
            GROUP BY product_elem->>'key'
            ORDER BY user_count DESC
        "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    /// Get global GitHub product user counts (total across all users)
    pub async fn get_global_github_product_counts(
        &self,
    ) -> Result<Vec<(String, i64)>, sqlx::Error> {
        let license_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(DISTINCT login) FROM github_users_cache WHERE is_active = true",
        )
        .fetch_one(&self.pool)
        .await?;

        let copilot_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(DISTINCT github_login) FROM github_copilot_cache WHERE is_active = true",
        )
        .fetch_one(&self.pool)
        .await?;

        let ghas_count: (i64,) = sqlx::query_as(
            "SELECT COALESCE((SELECT ghas_seats_consumed FROM github_licenses_cache ORDER BY synced_at DESC LIMIT 1), 0)::bigint",
        )
        .fetch_one(&self.pool)
        .await?;

        let mut results = Vec::new();
        if license_count.0 > 0 {
            results.push(("license".to_string(), license_count.0));
        }
        if copilot_count.0 > 0 {
            results.push(("copilot".to_string(), copilot_count.0));
        }
        if ghas_count.0 > 0 {
            results.push(("ghas".to_string(), ghas_count.0));
        }

        Ok(results)
    }

    /// Get linking statistics for data quality indicator
    pub async fn get_linking_stats(&self) -> Result<LinkingStats, sqlx::Error> {
        let total_persons: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM persons")
            .fetch_one(&self.pool)
            .await?;
        let persons_with_org: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM persons WHERE org_id IS NOT NULL")
                .fetch_one(&self.pool)
                .await?;
        let persons_with_atlassian: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM persons WHERE atlassian_account_id IS NOT NULL")
                .fetch_one(&self.pool)
                .await?;
        let persons_with_github: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM persons WHERE github_login IS NOT NULL")
                .fetch_one(&self.pool)
                .await?;
        let total_atlassian_cached: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM atlassian_users_cache")
                .fetch_one(&self.pool)
                .await?;
        let total_github_cached: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM github_users_cache")
            .fetch_one(&self.pool)
            .await?;

        Ok(LinkingStats {
            total_persons: total_persons.0,
            persons_with_org: persons_with_org.0,
            persons_with_atlassian: persons_with_atlassian.0,
            persons_with_github: persons_with_github.0,
            total_atlassian_cached: total_atlassian_cached.0,
            total_github_cached: total_github_cached.0,
        })
    }

    /// Get business unit (org_type) distribution across all organizations
    pub async fn get_business_unit_distribution(
        &self,
    ) -> Result<Vec<BusinessUnitDistribution>, sqlx::Error> {
        let total_persons: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM persons")
            .fetch_one(&self.pool)
            .await?;

        let rows: Vec<(String, i64, i64)> = sqlx::query_as(
            r#"
            SELECT
                o.org_type AS business_unit,
                COUNT(DISTINCT o.org_id) AS org_count,
                COALESCE(SUM((SELECT COUNT(*) FROM persons p WHERE p.org_id = o.org_id)), 0)::BIGINT AS person_count
            FROM organizations o
            GROUP BY o.org_type
            ORDER BY person_count DESC
        "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let total = total_persons.0 as f64;
        Ok(rows
            .into_iter()
            .map(
                |(business_unit, org_count, person_count)| BusinessUnitDistribution {
                    business_unit,
                    org_count,
                    person_count,
                    percentage: if total > 0.0 {
                        (person_count as f64 / total) * 100.0
                    } else {
                        0.0
                    },
                },
            )
            .collect())
    }
}
