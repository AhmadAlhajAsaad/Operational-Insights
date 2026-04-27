//! File parsing module for CSV and Excel imports (FR-007)

use csv::ReaderBuilder;
use std::collections::HashMap;
use std::io::Cursor;

use crate::imports::error::ImportError;
use crate::imports::types::{OrgImportRow, PersonImportRow};

/// File parser for CSV and Excel files
pub struct FileParser;

impl FileParser {
    // -----------------------------------------------------------------------
    // Fast CSV parser  single pass, index-based column mapping (no per-row HashMap)
    // -----------------------------------------------------------------------

    /// Parse a CSV file directly into typed rows in a single pass.
    ///
    /// Returns `(persons, orgs, is_person_import)`.
    ///
    /// This is the recommended hot path for large files:
    /// - Reads headers once -> builds a column-index map
    /// - Accesses each field by integer index (no HashMap allocations per row)
    /// - Pre-allocates the output Vec with a reasonable capacity
    pub fn parse_csv_fast(
        file_data: &[u8],
    ) -> Result<(Vec<PersonImportRow>, Vec<OrgImportRow>, bool), ImportError> {
        let cursor = Cursor::new(file_data);
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(cursor);

        let headers = reader
            .headers()
            .map_err(|e| ImportError::ParseError(format!("Failed to read CSV headers: {}", e)))?
            .clone();

        // Determine import type from header names (no need to read any rows)
        let is_person = Self::detect_person_import_from_headers(&headers);

        if is_person {
            // Build column-index lookup table once
            let find = |names: &[&str]| -> Option<usize> {
                names
                    .iter()
                    .find_map(|n| headers.iter().position(|h| h.trim().to_lowercase() == *n))
            };

            let idx_id = find(&["person_id", "id"]);
            let idx_full_name = find(&["full_name", "fullname", "name"]);
            let idx_first_name =
                find(&["person_first_name", "first_name", "firstname", "given_name"]);
            let idx_last_name = find(&[
                "person_last_name",
                "last_name",
                "lastname",
                "surname",
                "family_name",
            ]);
            let idx_email = find(&["person_email", "email", "mail", "e-mail"]);
            let idx_local_id = find(&["person_local_id", "local_id"]);
            let idx_department = find(&["department", "dept"]);
            let idx_job_title = find(&["job_title", "title", "position", "role"]);
            let idx_manager = find(&["manager", "manager_id", "reports_to"]);
            let idx_start_date = find(&["start_date", "hire_date", "employment_start"]);
            let idx_status = find(&["status", "employment_status"]);
            let idx_cost_center = find(&["cost_center", "costcenter", "cc"]);
            let idx_country = find(&["country", "location", "office_location"]);
            let idx_billing_loc = find(&[
                "person_billing_location",
                "billing_location",
                "billing_office",
            ]);
            let idx_org_id = find(&["org_id", "organization_id"]);

            // Helper closure: get trimmed non-empty value at column index
            let get_val = |rec: &csv::StringRecord, idx: Option<usize>| -> Option<String> {
                idx.and_then(|i| rec.get(i))
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
            };

            let mut persons = Vec::with_capacity(90_000);
            for (row_idx, result) in reader.records().enumerate() {
                let rec = result.map_err(|e| {
                    ImportError::ParseError(format!(
                        "Failed to parse CSV row {}: {}",
                        row_idx + 2,
                        e
                    ))
                })?;
                persons.push(PersonImportRow {
                    id: get_val(&rec, idx_id),
                    full_name: get_val(&rec, idx_full_name),
                    first_name: get_val(&rec, idx_first_name),
                    last_name: get_val(&rec, idx_last_name),
                    email: get_val(&rec, idx_email),
                    local_id: get_val(&rec, idx_local_id),
                    department: get_val(&rec, idx_department),
                    job_title: get_val(&rec, idx_job_title),
                    manager: get_val(&rec, idx_manager),
                    start_date: get_val(&rec, idx_start_date),
                    status: get_val(&rec, idx_status),
                    cost_center: get_val(&rec, idx_cost_center),
                    country: get_val(&rec, idx_country),
                    billing_location: get_val(&rec, idx_billing_loc),
                    org_id: get_val(&rec, idx_org_id),
                });
            }
            Ok((persons, vec![], true))
        } else {
            // Org imports are small files; keep the generic HashMap path for simplicity
            let raw = Self::parse_csv(file_data)?;
            let orgs = Self::parse_org_records(raw)?;
            Ok((vec![], orgs, false))
        }
    }

    /// Detect whether a CSV describes persons or organisations from header names alone.
    fn detect_person_import_from_headers(headers: &csv::StringRecord) -> bool {
        let person_markers = [
            "person_id",
            "person_email",
            "email",
            "first_name",
            "last_name",
        ];
        let org_markers = ["org_name", "organization_name", "organization_id"];

        let has_person = person_markers
            .iter()
            .any(|m| headers.iter().any(|h| h.trim().to_lowercase().contains(m)));
        let has_org = org_markers
            .iter()
            .any(|m| headers.iter().any(|h| h.trim().to_lowercase().contains(m)));

        // Prefer person import when ambiguous
        has_person || !has_org
    }

    // -----------------------------------------------------------------------
    // Legacy / compatibility helpers (kept for org imports and tests)
    // -----------------------------------------------------------------------

    /// Parse CSV file into raw records (HashMap per row).
    /// Prefer `parse_csv_fast` for large person imports.
    pub fn parse_csv(file_data: &[u8]) -> Result<Vec<HashMap<String, String>>, ImportError> {
        let cursor = Cursor::new(file_data);
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(cursor);

        let headers = reader
            .headers()
            .map_err(|e| ImportError::ParseError(format!("Failed to read CSV headers: {}", e)))?
            .clone();

        let mut records = Vec::new();

        for (idx, result) in reader.records().enumerate() {
            let record = result.map_err(|e| {
                ImportError::ParseError(format!("Failed to parse CSV row {}: {}", idx + 2, e))
            })?;

            let mut map = HashMap::new();

            for (i, field) in record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    map.insert(header.to_string(), field.trim().to_string());
                }
            }

            records.push(map);
        }

        Ok(records)
    }

    /// Parse Excel file into raw records
    pub fn parse_excel(_file_data: &[u8]) -> Result<Vec<HashMap<String, String>>, ImportError> {
        // TODO: Excel parsing temporarily disabled due to calamine type inference issues
        // CSV import works perfectly - Excel support coming soon
        Err(ImportError::ParseError(
            "Excel import temporarily unavailable. Please use CSV format for now.".to_string(),
        ))
    }

    /// Parse raw records into PersonImportRow structs
    pub fn parse_person_records(
        raw_records: Vec<HashMap<String, String>>,
    ) -> Result<Vec<PersonImportRow>, ImportError> {
        let mut persons = Vec::new();

        for record in raw_records {
            let person = PersonImportRow {
                id: Self::get_field(&record, &["person_id", "id", "ID"]),
                full_name: Self::get_field(&record, &["full_name", "fullname", "name"]),
                first_name: Self::get_field(
                    &record,
                    &["person_first_name", "first_name", "firstname", "given_name"],
                ),
                last_name: Self::get_field(
                    &record,
                    &[
                        "person_last_name",
                        "last_name",
                        "lastname",
                        "surname",
                        "family_name",
                    ],
                ),
                email: Self::get_field(&record, &["person_email", "email", "mail", "e-mail"]),
                local_id: Self::get_field(&record, &["person_local_id", "local_id"]),
                department: Self::get_field(&record, &["department", "dept"]),
                job_title: Self::get_field(&record, &["job_title", "title", "position", "role"]),
                manager: Self::get_field(&record, &["manager", "manager_id", "reports_to"]),
                start_date: Self::get_field(
                    &record,
                    &["start_date", "hire_date", "employment_start"],
                ),
                status: Self::get_field(&record, &["status", "employment_status"]),
                cost_center: Self::get_field(&record, &["cost_center", "costcenter", "cc"]),
                country: Self::get_field(&record, &["country", "location", "office_location"]),
                billing_location: Self::get_field(
                    &record,
                    &[
                        "person_billing_location",
                        "billing_location",
                        "billing_office",
                    ],
                ),
                org_id: Self::get_field(&record, &["org_id", "organization_id"]),
            };

            persons.push(person);
        }

        Ok(persons)
    }

    /// Parse raw records into OrgImportRow structs
    pub fn parse_org_records(
        raw_records: Vec<HashMap<String, String>>,
    ) -> Result<Vec<OrgImportRow>, ImportError> {
        let mut orgs = Vec::new();

        for record in raw_records {
            let org = OrgImportRow {
                org_id: Self::get_field(&record, &["org_id", "organization_id", "id"]),
                org_name: Self::get_field(&record, &["org_name", "organization_name", "name"]),
                parent_org: Self::get_field(&record, &["parent_org_id", "parent_org", "parent"]),
                cost_center: Self::get_field(&record, &["cost_center", "costcenter"]),
                manager: Self::get_field(&record, &["manager", "manager_id"]),
                budget: Self::get_field(&record, &["budget", "annual_budget"]),
                org_type: Self::get_field(&record, &["org_type", "type", "organization_type"]),
            };

            orgs.push(org);
        }

        Ok(orgs)
    }

    /// Get field value from record, trying multiple possible column names
    fn get_field(record: &HashMap<String, String>, possible_names: &[&str]) -> Option<String> {
        for name in possible_names {
            // Try exact match (case-insensitive)
            for (key, value) in record.iter() {
                if key.to_lowercase() == name.to_lowercase() {
                    let trimmed = value.trim().to_string();
                    return if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed)
                    };
                }
            }
        }
        None
    }

    /// Detect if file is CSV or Excel based on content
    pub fn detect_format(_file_data: &[u8], file_name: &str) -> Result<FileFormat, ImportError> {
        let extension = file_name.rsplit('.').next().unwrap_or("").to_lowercase();

        match extension.as_str() {
            "csv" | "txt" => Ok(FileFormat::Csv),
            "xlsx" | "xls" => Ok(FileFormat::Excel),
            _ => Err(ImportError::UnsupportedFormat(format!(
                "Unsupported file extension: .{}",
                extension
            ))),
        }
    }
}

/// File format enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    Csv,
    Excel,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_csv_format() {
        let result = FileParser::detect_format(b"", "test.csv").unwrap();
        assert_eq!(result, FileFormat::Csv);
    }

    #[test]
    fn test_detect_excel_format() {
        let result = FileParser::detect_format(b"", "test.xlsx").unwrap();
        assert_eq!(result, FileFormat::Excel);
    }

    #[test]
    fn test_unsupported_format() {
        let result = FileParser::detect_format(b"", "test.pdf");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_field() {
        let mut record = HashMap::new();
        record.insert("Email".to_string(), "test@example.com".to_string());

        let result = FileParser::get_field(&record, &["email", "mail"]);
        assert_eq!(result, Some("test@example.com".to_string()));
    }

    #[test]
    fn test_get_field_case_insensitive() {
        let mut record = HashMap::new();
        record.insert("PERSON_EMAIL".to_string(), "test@example.com".to_string());

        let result = FileParser::get_field(&record, &["person_email"]);
        assert_eq!(result, Some("test@example.com".to_string()));
    }
}
