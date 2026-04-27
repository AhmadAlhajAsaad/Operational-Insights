//! Merge logic for import data with existing database records (FR-007)

use crate::imports::types::{OrgImportRow, PersonImportRow};
use crate::organizations::types::Organization;
use crate::persons::types::Person;
use chrono::NaiveDate;

/// Merge engine for combining import data with existing database records
#[derive(Clone, Copy)]
pub struct MergeEngine;

impl MergeEngine {
    /// Merge import person with database person
    ///
    /// Rule: Import priority EXCEPT when import field empty and DB field filled
    ///
    /// This means:
    /// - If import has a value, use it (even if it differs from DB)
    /// - If import is empty/None but DB has a value, keep DB value
    /// - If both are empty/None, keep empty/None
    pub fn merge_person(db_person: &Person, import_person: &PersonImportRow) -> Person {
        Person {
            id: db_person.id,
            person_id: db_person.person_id.clone(),

            // Merge name fields (use placeholders if both import and DB are empty)
            first_name: Self::merge_string_with_placeholder(
                import_person.first_name.as_deref(),
                &db_person.first_name,
                "[To Be Determined]",
            ),

            last_name: Self::merge_string_with_placeholder(
                import_person.last_name.as_deref(),
                &db_person.last_name,
                "[To Be Determined]",
            ),

            email: Self::merge_string_with_placeholder(
                import_person.email.as_deref(),
                &db_person.email,
                "unknown@placeholder.local",
            ),

            // Merge optional fields
            // local_id comes from CSV column `person_local_id` (e.g. CCJ183@equans.com),
            // which is the login identity Atlassian stores as the user's email.
            local_id: Self::merge_optional_field(
                import_person.local_id.as_ref(),
                db_person.local_id.as_ref(),
            ),

            language: db_person.language.clone(), // Not in import

            billing_location: Self::merge_optional_field(
                import_person.billing_location.as_ref(),
                db_person.billing_location.as_ref(),
            ),

            country: Self::merge_optional_field(
                import_person.country.as_ref(),
                db_person.country.as_ref(),
            ),

            job_title: Self::merge_optional_field(
                import_person.job_title.as_ref(),
                db_person.job_title.as_ref(),
            ),

            department: Self::merge_optional_field(
                import_person.department.as_ref(),
                db_person.department.as_ref(),
            ),

            manager: Self::merge_optional_field(
                import_person.manager.as_ref(),
                db_person.manager.as_ref(),
            ),

            start_date: Self::merge_date_field(
                import_person.start_date.as_deref(),
                db_person.start_date,
            ),

            org_id: Self::merge_optional_field(
                import_person.org_id.as_ref(),
                db_person.org_id.as_ref(),
            ),

            // Preserve existing fields not in import
            status: db_person.status.clone(),
            source: db_person.source.clone(),
            gid: db_person.gid.clone(),
            gid_confidence: db_person.gid_confidence,
            gid_extraction_method: db_person.gid_extraction_method.clone(),
            last_matched_at: db_person.last_matched_at,
            matching_metadata: db_person.matching_metadata.clone(),
            vendor_identifiers: db_person.vendor_identifiers.clone(),

            // Atlassian link fields (FR-009) - preserve existing linking
            atlassian_account_id: db_person.atlassian_account_id.clone(),
            atlassian_link_status: db_person.atlassian_link_status.clone(),
            atlassian_linked_at: db_person.atlassian_linked_at,
            atlassian_link_method: db_person.atlassian_link_method.clone(),

            // GitHub link fields (FR-012) - preserve existing linking
            github_login: db_person.github_login.clone(),
            github_account_id: db_person.github_account_id.clone(),
            github_username: db_person.github_username.clone(),
            github_link_status: db_person.github_link_status.clone(),
            github_linked_at: db_person.github_linked_at,
            github_linked_by: db_person.github_linked_by.clone(),

            // Timestamps
            created_at: db_person.created_at,
            updated_at: chrono::Utc::now(),
        }
    }

    /// Merge import organization with database organization
    pub fn merge_organization(db_org: &Organization, import_org: &OrgImportRow) -> Organization {
        Organization {
            id: db_org.id,
            org_id: db_org.org_id.clone(),

            // Merge name (use placeholder if both import and DB are empty)
            name: Self::merge_string_with_placeholder(
                import_org.org_name.as_deref(),
                &db_org.name,
                "[Organization Name To Be Determined]",
            ),

            // Merge optional fields
            description: db_org.description.clone(), // Not in import

            parent_org_id: Self::merge_optional_field(
                import_org.parent_org.as_ref(),
                db_org.parent_org_id.as_ref(),
            ),

            cost_center: Self::merge_optional_field(
                import_org.cost_center.as_ref(),
                db_org.cost_center.as_ref(),
            ),

            manager: Self::merge_optional_field(
                import_org.manager.as_ref(),
                db_org.manager.as_ref(),
            ),

            budget: Self::merge_budget_field(import_org.budget.as_deref(), db_org.budget),

            org_type: Self::merge_org_type(import_org.org_type.as_deref(), &db_org.org_type),

            // Preserve existing fields not in import
            status: db_org.status.clone(),

            // Timestamps
            created_at: db_org.created_at,
            updated_at: chrono::Utc::now(),
        }
    }

    /// Merge required field (String)
    /// Import priority except when import is empty and DB has value
    fn merge_required_field(import_value: Option<&str>, db_value: &str) -> String {
        match import_value {
            Some(imp) if !imp.trim().is_empty() => {
                // Import has non-empty value, use it
                imp.trim().to_string()
            }
            _ => {
                // Import empty or None, keep database value
                db_value.to_string()
            }
        }
    }

    /// Merge string field with placeholder fallback
    /// Used when both import and existing DB value might be empty
    /// Import priority > DB value > placeholder
    fn merge_string_with_placeholder(
        import_value: Option<&str>,
        db_value: &str,
        placeholder: &str,
    ) -> String {
        match import_value {
            Some(imp) if !imp.trim().is_empty() => {
                // Import has non-empty value, use it
                imp.trim().to_string()
            }
            _ => {
                // Import empty or None
                if !db_value.is_empty()
                    && !db_value.starts_with("[To Be Determined]")
                    && !db_value.starts_with("[Organization")
                    && db_value != "unknown@placeholder.local"
                {
                    // DB has real value, keep it
                    db_value.to_string()
                } else {
                    // DB also empty or is placeholder, use provided placeholder
                    placeholder.to_string()
                }
            }
        }
    }

    /// Merge optional field (Option<String>)
    /// Import priority except when import is empty and DB has value
    fn merge_optional_field(
        import_value: Option<&String>,
        db_value: Option<&String>,
    ) -> Option<String> {
        match (import_value, db_value) {
            (Some(imp), _) if !imp.trim().is_empty() => {
                // Import has non-empty value, use it
                Some(imp.trim().to_string())
            }
            (Some(_), Some(db)) => {
                // Import empty but DB has value, keep DB
                Some(db.clone())
            }
            (None, Some(db)) => {
                // Import not provided, keep DB
                Some(db.clone())
            }
            _ => {
                // All other cases: both None or imp with None db
                None
            }
        }
    }

    /// Merge date field
    fn merge_date_field(
        import_value: Option<&str>,
        db_value: Option<NaiveDate>,
    ) -> Option<NaiveDate> {
        match import_value {
            Some(date_str) if !date_str.trim().is_empty() => {
                // Try to parse import date
                Self::parse_date(date_str).or(db_value)
            }
            _ => {
                // Import empty or None, keep database value
                db_value
            }
        }
    }

    /// Parse date string (supports multiple formats)
    fn parse_date(date_str: &str) -> Option<NaiveDate> {
        let trimmed = date_str.trim();

        // Try different date formats
        let formats = [
            "%Y-%m-%d", // 2024-12-31
            "%d/%m/%Y", // 31/12/2024
            "%d-%m-%Y", // 31-12-2024
            "%m/%d/%Y", // 12/31/2024
            "%d.%m.%Y", // 31.12.2024
        ];

        for format in &formats {
            if let Ok(date) = NaiveDate::parse_from_str(trimmed, format) {
                return Some(date);
            }
        }

        None
    }

    /// Merge budget field
    fn merge_budget_field(import_value: Option<&str>, db_value: Option<f64>) -> Option<f64> {
        match import_value {
            Some(budget_str) if !budget_str.trim().is_empty() => {
                // Try to parse import budget
                budget_str.trim().parse::<f64>().ok().or(db_value)
            }
            _ => {
                // Import empty or None, keep database value
                db_value
            }
        }
    }

    /// Merge organization type
    fn merge_org_type(import_value: Option<&str>, db_value: &str) -> String {
        match import_value {
            Some(typ) if !typ.trim().is_empty() => typ.trim().to_string(),
            _ => db_value.to_string(),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_required_field_import_priority() {
        let result = MergeEngine::merge_required_field(Some("New Value"), "Old Value");
        assert_eq!(result, "New Value");
    }

    #[test]
    fn test_merge_required_field_keep_db_when_import_empty() {
        let result = MergeEngine::merge_required_field(Some(""), "Old Value");
        assert_eq!(result, "Old Value");
    }

    #[test]
    fn test_merge_required_field_keep_db_when_import_none() {
        let result = MergeEngine::merge_required_field(None, "Old Value");
        assert_eq!(result, "Old Value");
    }

    #[test]
    fn test_merge_optional_field_import_priority() {
        let import = "New Value".to_string();
        let db = "Old Value".to_string();
        let result = MergeEngine::merge_optional_field(Some(&import), Some(&db));
        assert_eq!(result, Some("New Value".to_string()));
    }

    #[test]
    fn test_merge_optional_field_keep_db_when_import_empty() {
        let import = "".to_string();
        let db = "Old Value".to_string();
        let result = MergeEngine::merge_optional_field(Some(&import), Some(&db));
        assert_eq!(result, Some("Old Value".to_string()));
    }

    #[test]
    fn test_merge_optional_field_keep_db_when_import_none() {
        let db = "Old Value".to_string();
        let result = MergeEngine::merge_optional_field(None, Some(&db));
        assert_eq!(result, Some("Old Value".to_string()));
    }

    #[test]
    fn test_merge_optional_field_both_none() {
        let result = MergeEngine::merge_optional_field(None, None);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_date_iso_format() {
        let date = MergeEngine::parse_date("2024-12-31");
        assert!(date.is_some());
        assert_eq!(date.unwrap().to_string(), "2024-12-31");
    }

    #[test]
    fn test_parse_date_european_format() {
        let date = MergeEngine::parse_date("31/12/2024");
        assert!(date.is_some());
        assert_eq!(date.unwrap().to_string(), "2024-12-31");
    }

    #[test]
    fn test_parse_date_invalid() {
        let date = MergeEngine::parse_date("invalid");
        assert!(date.is_none());
    }
}
