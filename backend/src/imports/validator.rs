//! Data validation module for import records (FR-007)

use crate::imports::types::{
    ErrorSeverity, OrgImportRow, PersonImportRow, ValidationError, ValidationErrorType,
    ValidationResult,
};
use std::collections::{HashMap, HashSet};

/// Data validator for import records
pub struct Validator;

impl Validator {
    /// Validate organization import records
    pub fn validate_organizations(records: &[OrgImportRow]) -> ValidationResult {
        let mut errors = Vec::new();
        let mut seen_org_ids = HashMap::new();

        for (idx, org) in records.iter().enumerate() {
            let row = idx + 2; // +2 for header and 0-indexing

            // Skip validation if org_id is missing (auto-generation will handle it)
            let Some(org_id) = org.org_id.as_ref() else {
                continue;
            };

            // Check unique org_id
            if let Some(first_row) = seen_org_ids.get(org_id) {
                errors.push(ValidationError {
                    row,
                    field: "org_id".to_string(),
                    value: Some(org_id.clone()),
                    error_type: ValidationErrorType::Duplicate,
                    message: format!(
                        "org_id '{}' komt meerdere keren voor (rij {}, {})",
                        org_id, first_row, row
                    ),
                    severity: ErrorSeverity::Error,
                });
            } else {
                seen_org_ids.insert(org_id.clone(), row);
            }

            // Removed all format and missing field validations - accept any input
        }

        // Count unique rows that contain errors (one row may have multiple error entries)
        let rows_with_errors: HashSet<_> = errors
            .iter()
            .filter(|e| matches!(e.severity, ErrorSeverity::Error))
            .map(|e| e.row)
            .collect();
        let unique_error_rows = rows_with_errors.len();
        let warning_count = errors
            .iter()
            .filter(|e| matches!(e.severity, ErrorSeverity::Warning))
            .count();

        ValidationResult {
            valid: unique_error_rows == 0,
            total_rows: records.len(),
            valid_rows: records.len().saturating_sub(unique_error_rows),
            error_rows: unique_error_rows,
            warning_rows: warning_count,
            errors,
        }
    }

    /// Validate person import records
    ///
    /// Duplicates worden NIET als errors gemarkeerd - eerste record wordt behouden,
    /// rest wordt genegeerd tijdens import.
    pub fn validate_persons(records: &[PersonImportRow]) -> ValidationResult {
        let mut seen_person_ids = HashMap::new();
        let mut seen_emails = HashMap::new();

        for (idx, person) in records.iter().enumerate() {
            let row = idx + 2; // +2 for header and 0-indexing

            // Skip validation if person_id is missing (auto-generation will handle it)
            let Some(person_id) = person.id.as_ref() else {
                continue;
            };

            // Track person_id duplicates (but don't error - just track for deduplication)
            if !seen_person_ids.contains_key(person_id) {
                seen_person_ids.insert(person_id.clone(), row);
            }
            // Duplicate person_ids worden genegeerd - eerste record wordt gebruikt

            // Track email duplicates (but don't error - just track for deduplication)
            if let Some(email) = &person.email {
                if !email.is_empty() && !seen_emails.contains_key(email) {
                    seen_emails.insert(email.clone(), row);
                }
                // Duplicate emails worden genegeerd - eerste record wordt gebruikt
            }

            // Removed all name field validations - accept any input
        }

        // Geen errors voor duplicaten - alles is valid
        // Duplicaten worden tijdens import automatisch geskipt
        ValidationResult {
            valid: true, // Altijd valid - duplicaten zijn geen errors
            total_rows: records.len(),
            valid_rows: records.len(), // Alle rows zijn "valid"
            error_rows: 0,
            warning_rows: 0,
            errors: vec![], // Geen errors
        }
    }

    /// Simple email validation
    #[allow(dead_code)]
    fn is_valid_email(email: &str) -> bool {
        if let Some(at_pos) = email.find('@') {
            let local = &email[..at_pos];
            let domain = &email[at_pos + 1..];
            if local.is_empty() || domain.is_empty() {
                return false;
            }
            if !domain.contains('.') {
                return false;
            }
            // Ensure domain labels are non-empty (e.g., example.com)
            domain.split('.').all(|part| !part.is_empty())
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_person() {
        let persons = vec![PersonImportRow {
            id: Some("P001".to_string()),
            local_id: None,
            full_name: None,
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            email: Some("john@example.com".to_string()),
            department: None,
            job_title: None,
            manager: None,
            start_date: None,
            status: None,
            cost_center: None,
            country: None,
            billing_location: None,
            org_id: None,
        }];

        let result = Validator::validate_persons(&persons);
        assert!(result.valid);
        assert_eq!(result.total_rows, 1);
        assert_eq!(result.valid_rows, 1);
        assert_eq!(result.error_rows, 0);
    }

    #[test]
    fn test_validate_missing_email() {
        // Missing email is now considered valid - accept all input
        let persons = vec![PersonImportRow {
            id: Some("P001".to_string()),
            local_id: None,
            full_name: None,
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            email: None,
            department: None,
            job_title: None,
            manager: None,
            start_date: None,
            status: None,
            cost_center: None,
            country: None,
            billing_location: None,
            org_id: None,
        }];

        let result = Validator::validate_persons(&persons);
        assert!(result.valid); // Now valid - we accept all input
        assert_eq!(result.error_rows, 0);
    }

    #[test]
    fn test_validate_duplicate_person_id() {
        // Duplicates are now considered valid - deduplication happens in service layer
        let persons = vec![
            PersonImportRow {
                id: Some("P001".to_string()),
                local_id: None,
                full_name: None,
                first_name: Some("John".to_string()),
                last_name: Some("Doe".to_string()),
                email: Some("john@example.com".to_string()),
                department: None,
                job_title: None,
                manager: None,
                start_date: None,
                status: None,
                cost_center: None,
                country: None,
                billing_location: None,
                org_id: None,
            },
            PersonImportRow {
                id: Some("P001".to_string()),
                local_id: None,
                full_name: None,
                first_name: Some("Jane".to_string()),
                last_name: Some("Doe".to_string()),
                email: Some("jane@example.com".to_string()),
                department: None,
                job_title: None,
                manager: None,
                start_date: None,
                status: None,
                cost_center: None,
                country: None,
                billing_location: None,
                org_id: None,
            },
        ];

        let result = Validator::validate_persons(&persons);
        assert!(result.valid); // Now valid - duplicates are handled in service layer
        assert_eq!(result.error_rows, 0);
        assert_eq!(result.errors.len(), 0); // No errors for duplicates
    }

    #[test]
    fn test_is_valid_email() {
        assert!(Validator::is_valid_email("test@example.com"));
        assert!(Validator::is_valid_email("user.name@company.co.uk"));
        assert!(!Validator::is_valid_email("invalid"));
        assert!(!Validator::is_valid_email("@example.com"));
        assert!(!Validator::is_valid_email("test@"));
    }
}
