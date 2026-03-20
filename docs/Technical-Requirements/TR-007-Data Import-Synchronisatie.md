# TR-007: Data Import & Synchronisatie - Technische Specificaties

**Status:** In Review
**Datum:** 2026-02-18
**Auteur(s):** Ahmad Alhaj Asaad
**Implementeert:** [FR-007](../Functional-Requirements/FR-007-Data-Synchronization.md)
**Applies To:** Backend Rust applicatie, PostgreSQL database, React frontend

---

## Scope

Dit document definieert de technische specificaties voor het import systeem van organisaties en personen, inclusief:

- File upload en parsing (CSV, Excel)
- Validatie van import data
- Preview functionaliteit
- Merge logica (import vs. database)
- Soft-delete gedrag
- Rollback mechanisme
- Error handling en reporting

---

## Architectuur Overzicht

```
┌──────────────────┐     ┌──────────────────┐     ┌──────────────────┐
│  React Frontend  │────▶│   Rust Backend   │────▶│   PostgreSQL     │
│  (Import UI)     │     │   File Parser    │     │   Transactions   │
└──────────────────┘     └──────────────────┘     └──────────────────┘
        │                         │
        │ Upload File             │ Parse & Validate
        │                         │
        ▼                         ▼
   ┌─────────┐          ┌──────────────────┐
   │ Preview │          │  Merge Engine    │
   └─────────┘          └──────────────────┘
        │                         │
        │ Confirm                 │ Apply Changes
        │                         │
        ▼                         ▼
   ┌─────────┐          ┌──────────────────┐
   │ Execute │          │  Soft-Delete     │
   │ Import  │          │  Reactivation    │
   └─────────┘          └──────────────────┘
```

### Data Flow

1. Admin uploadt CSV/Excel bestand
2. Backend parseert bestand en extraheert data
3. **Minimale Validatie**: ALLEEN duplicate checks binnen het bestand (person_id en email duplicaten)
4. Preview: vergelijk import met database, toon wijzigingen
5. Admin bevestigt preview
6. Execute: transactie start
   - Update bestaande records (merge logica met placeholders)
   - Insert nieuwe records (met placeholders voor lege velden)
   - Soft-delete missing persons
   - Reactiveer personen die terugkomen
   - Auto-genereer IDs waar nodig
7. Transactie commit of rollback bij fout
8. Resultaat tonen met statistieken

**Belangrijke Wijziging (2026-02-23):** Volledig flexibele imports zonder format of missing field validatie:

- **ENIGE VALIDATIE:** Duplicaten binnen het bestand (person_id en email) → ERROR (blocking)
- **GEEN validatie** voor: ontbrekende velden, email formaat, org_id formaat, budget formaat, etc.
- Ontbrekende `person_id` → automatisch gegenereerd (bijv. `AUTO_jsmith` of `AUTO_{UUID}`)
- Ontbrekende `email` → placeholder `unknown_{person_id}@placeholder.local`
- Ongeldige email (geen @ of incorrect formaat) → gewoon accepteren zoals het is
- Ontbrekende `first_name`/`last_name` → placeholder `[To Be Determined]`
- Ontbrekende `org_id` → automatisch gegenereerd (bijv. `ORG_ENGINEERING` of `ORG_{UUID}`)
- Ontbrekende `org_name` → placeholder `[Organization Name To Be Determined]`

Dit stelt gebruikers in staat om:

- **Elke data** te importeren zonder validatie blokkades
- Data stapsgewijs te importeren
- Ontbrekende gegevens later aan te vullen via herhaalde imports
- Te werken met incomplete datasets zonder foutmeldingen

---

## Componenten Structuur

```
backend/
├── src/
│   ├── imports/
│   │   ├── mod.rs                    # Module exports
│   │   ├── types.rs                  # Data types en DTOs
│   │   ├── parser.rs                 # CSV/Excel parsing
│   │   ├── validator.rs              # Data validation
│   │   ├── merger.rs                 # Merge logic
│   │   ├── repository.rs             # Database queries
│   │   ├── service.rs                # Business logic
│   │   └── error.rs                  # Error handling
│   ├── routes/
│   │   └── imports.rs                # HTTP route handlers
│   └── main.rs
└── migrations/
    └── 002_persons_organizations.sql

frontend/
├── src/
│   ├── pages/
│   │   └── ImportPage.tsx            # Import UI
│   ├── components/
│   │   ├── FileUpload.tsx            # Drag-drop upload
│   │   ├── ImportPreview.tsx         # Preview resultaat
│   │   └── ValidationErrors.tsx      # Foutmeldingen
│   └── api/
│       └── imports.ts                # API client calls
```

---

## Database Schema

### Imports Tabel

```sql
CREATE TABLE IF NOT EXISTS imports (
    id SERIAL PRIMARY KEY,
    import_id VARCHAR(50) NOT NULL UNIQUE,  -- e.g., IMP-2026-0217-001
    file_name VARCHAR(255) NOT NULL,
    file_size INTEGER NOT NULL,
    record_type VARCHAR(20) NOT NULL,  -- 'Person' or 'Organization'
    status VARCHAR(20) NOT NULL DEFAULT 'Pending',
    user_id VARCHAR(255) NOT NULL,

    -- Statistics
    total_rows INTEGER DEFAULT 0,
    imported INTEGER DEFAULT 0,
    updated INTEGER DEFAULT 0,
    skipped INTEGER DEFAULT 0,
    errors INTEGER DEFAULT 0,

    -- Rollback info (optioneel voor MVP)
    rollback_available BOOLEAN DEFAULT TRUE,
    rollback_deadline TIMESTAMPTZ,
    rollback_data JSONB,

    error_details JSONB,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE INDEX idx_imports_status ON imports(status);
CREATE INDEX idx_imports_user ON imports(user_id);
CREATE INDEX idx_imports_type ON imports(record_type);
CREATE INDEX idx_imports_created ON imports(created_at DESC);
```

### Import Errors Tabel

```sql
CREATE TABLE IF NOT EXISTS import_errors (
    id SERIAL PRIMARY KEY,
    import_id VARCHAR(50) NOT NULL REFERENCES imports(import_id) ON DELETE CASCADE,
    row_number INTEGER NOT NULL,
    field VARCHAR(100),
    value TEXT,
    error_type VARCHAR(50) NOT NULL,
    message TEXT NOT NULL,
    severity VARCHAR(20) NOT NULL DEFAULT 'ERROR',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_import_errors_import ON import_errors(import_id);
```

---

## API Specificaties

### Base URL

```
/api/imports
```

### Authenticatie

Alle endpoints vereisen JWT Bearer token met **admin** rol.

---

### POST /api/imports/upload

**Beschrijving:** Upload CSV of Excel bestand voor parsing en validatie.

**Request:**

```http
POST /api/imports/upload
Content-Type: multipart/form-data
Authorization: Bearer {jwt_token}

file: [binary file data]
```

**Supported Formats:**

- `.csv` (UTF-8 encoding)
- `.xlsx` (Excel 2007+)

**Response (200 OK):**

```json
{
  "upload_id": "UPL-2026-0223-001",
  "file_name": "Persons Feb 23 2026.csv",
  "file_size": 45678,
  "total_rows": 187,
  "parsed_organizations": 49,
  "parsed_persons": 187,
  "validation": {
    "valid": false,
    "total_rows": 187,
    "valid_rows": 186,
    "invalid_rows": 1,
    "errors": [
      {
        "row_number": 89,
        "field": "person_id",
        "value": "CCJ183",
        "error_type": "DUPLICATE",
        "message": "person_id 'CCJ183' komt meerdere keren voor (rij 12, 89)",
        "severity": "ERROR"
      }
    ]
  }
}
```

**Opmerking (2026-02-23):** Alleen duplicaten blokkeren de import. Ontbrekende velden, ongeldige email formats, etc. genereren GEEN errors of warnings meer - ze worden automatisch verwerkt met placeholders of auto-generatie.

**Error Responses:**

- `400 Bad Request` - Unsupported file format
- `413 Payload Too Large` - File > 50MB
- `401 Unauthorized` - Not admin
- `500 Internal Server Error` - Parse error

---

### POST /api/imports/preview

**Beschrijving:** Genereer preview van import wijzigingen.

**Request Body:**

```json
{
  "upload_id": "UPL-2026-0218-001",
  "import_valid_only": false
}
```

**Response (200 OK):**

```json
{
  "preview_id": "PRV-2026-0218-001",
  "organizations": {
    "new": 5,
    "updated": 44,
    "unchanged": 0,
    "new_ids": ["ORG0045", "ORG0046", "ORG0047", "ORG0048", "ORG0049"],
    "updated_ids": ["ORG0042", "ORG0043", "ORG0044"]
  },
  "persons": {
    "new": 23,
    "updated": 150,
    "soft_deleted": 14,
    "reactivated": 0,
    "unchanged": 0,
    "new_ids": ["ABC123", "DEF456"],
    "updated_ids": ["CCJ183", "DEI311"],
    "soft_deleted_ids": ["OLD001", "OLD002"],
    "reactivated_ids": []
  },
  "changes": [
    {
      "record_type": "Person",
      "person_id": "CCJ183",
      "change_type": "Update",
      "field_changes": [
        {
          "field": "first_name",
          "old_value": "Thomas",
          "new_value": "Thomas",
          "changed": false
        },
        {
          "field": "country",
          "old_value": "Austria",
          "new_value": null,
          "changed": false,
          "kept_old": true
        },
        {
          "field": "job_title",
          "old_value": "Engineer",
          "new_value": "Senior Engineer",
          "changed": true
        }
      ]
    }
  ]
}
```

---

### POST /api/imports/execute

**Beschrijving:** Voert de import uit op basis van preview.

**Request Body:**

```json
{
  "preview_id": "PRV-2026-0218-001",
  "confirmed": true
}
```

**Response (200 OK):**

```json
{
  "import_id": "IMP-2026-0218-001",
  "status": "Completed",
  "organizations": {
    "added": 5,
    "updated": 44
  },
  "persons": {
    "added": 23,
    "updated": 150,
    "soft_deleted": 14,
    "reactivated": 0
  },
  "duration_ms": 2456,
  "completed_at": "2026-02-18T10:35:00Z"
}
```

**Error Response (500):**

```json
{
  "error": "Import failed: database error",
  "import_id": "IMP-2026-0218-001",
  "status": "Failed",
  "rollback": "completed"
}
```

---

### GET /api/imports

**Beschrijving:** Haalt lijst van eerdere imports op.

**Query Parameters:**

| Parameter     | Type    | Default | Beschrijving                         |
| ------------- | ------- | ------- | ------------------------------------ |
| `status`      | string  | -       | Filter op status                     |
| `record_type` | string  | -       | Filter op type (Person/Organization) |
| `page`        | integer | `1`     | Paginanummer                         |
| `per_page`    | integer | `25`    | Resultaten per pagina                |

**Response (200 OK):**

```json
{
  "data": [
    {
      "import_id": "IMP-2026-0218-001",
      "file_name": "Persons Feb 17 2026.csv",
      "record_type": "Person",
      "status": "Completed",
      "total_rows": 187,
      "imported": 173,
      "user_id": "admin@equans.com",
      "created_at": "2026-02-18T10:30:00Z",
      "rollback_available": true
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 25,
    "total": 12,
    "total_pages": 1
  }
}
```

---

### GET /api/imports/:import_id

**Beschrijving:** Haalt details van één import op.

**Response (200 OK):**

```json
{
  "import_id": "IMP-2026-0218-001",
  "file_name": "Persons Feb 17 2026.csv",
  "file_size": 45678,
  "record_type": "Person",
  "status": "Completed",
  "user_id": "admin@equans.com",
  "total_rows": 187,
  "imported": 173,
  "updated": 150,
  "skipped": 14,
  "errors": 0,
  "rollback_available": true,
  "rollback_deadline": "2026-02-25T10:30:00Z",
  "created_at": "2026-02-18T10:30:00Z",
  "completed_at": "2026-02-18T10:35:00Z",
  "error_details": null
}
```

---

## Rust Implementatie Details

### File Parsing

```rust
use csv::ReaderBuilder;
use calamine::{Reader, Xlsx, open_workbook};

pub struct FileParser;

impl FileParser {
    /// Parse CSV file
    pub fn parse_csv(file_data: &[u8]) -> Result<Vec<HashMap<String, String>>, ParseError> {
        let cursor = Cursor::new(file_data);
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(cursor);

        let headers = reader.headers()?.clone();
        let mut records = Vec::new();

        for result in reader.records() {
            let record = result?;
            let mut map = HashMap::new();

            for (i, field) in record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    map.insert(header.to_string(), field.to_string());
                }
            }

            records.push(map);
        }

        Ok(records)
    }

    /// Parse Excel file
    pub fn parse_excel(file_data: &[u8]) -> Result<Vec<HashMap<String, String>>, ParseError> {
        let cursor = Cursor::new(file_data);
        let mut workbook: Xlsx<_> = open_workbook_from_rs(cursor)?;

        // Get first worksheet
        let worksheet = workbook
            .worksheet_range_at(0)
            .ok_or(ParseError::NoWorksheet)??;

        let mut records = Vec::new();
        let mut headers: Vec<String> = Vec::new();

        for (row_idx, row) in worksheet.rows().enumerate() {
            if row_idx == 0 {
                // First row is headers
                headers = row.iter()
                    .map(|cell| cell.to_string())
                    .collect();
            } else {
                let mut map = HashMap::new();
                for (col_idx, cell) in row.iter().enumerate() {
                    if let Some(header) = headers.get(col_idx) {
                        map.insert(header.clone(), cell.to_string());
                    }
                }
                records.push(map);
            }
        }

        Ok(records)
    }
}
```

### Data Validation

**Belangrijke Update (2026-02-23):** Validatie is MAXIMAAL vereenvoudigd - alleen duplicate checks binnen het bestand worden uitgevoerd. Geen format validatie, geen missing field validatie.

```rust
pub struct Validator;

impl Validator {
    /// Validate organizations - ONLY duplicate checks
    pub fn validate_organizations(
        records: &[ParsedOrganization],
    ) -> ValidationResult {
        let mut errors = Vec::new();
        let mut seen_org_ids = HashMap::new();

        for (idx, org) in records.iter().enumerate() {
            let row = idx + 2; // +2 for header and 0-indexing

            // Skip if no org_id (auto-generation will handle it)
            if org.org_id.is_none() {
                continue;
            }

            let org_id = org.org_id.as_ref().unwrap();

            // Check unique org_id (ONLY duplicate check - NO other validation)
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

            // NO format validation
            // NO missing field validation
            // Accept ANY input except duplicates
        }

        // Count unique rows with errors
        let rows_with_errors: HashSet<_> = errors
            .iter()
            .filter(|e| matches!(e.severity, ErrorSeverity::Error))
            .map(|e| e.row)
            .collect();
        let unique_error_rows = rows_with_errors.len();

        ValidationResult {
            valid: unique_error_rows == 0,
            total_rows: records.len(),
            valid_rows: records.len().saturating_sub(unique_error_rows),
            error_rows: unique_error_rows,
            warning_rows: 0, // No warnings anymore
            errors,
        }
    }

    /// Validate persons - ONLY duplicate checks
    pub fn validate_persons(
        records: &[ParsedPerson],
    ) -> ValidationResult {
        let mut errors = Vec::new();
        let mut seen_person_ids = HashMap::new();
        let mut seen_emails = HashMap::new();

        for (idx, person) in records.iter().enumerate() {
            let row = idx + 2;

            // Skip if no person_id (auto-generation will handle it)
            if person.id.is_none() {
                continue;
            }

            let person_id = person.id.as_ref().unwrap();

            // Check unique person_id (ONLY duplicate check)
            if let Some(first_row) = seen_person_ids.get(person_id) {
                errors.push(ValidationError {
                    row,
                    field: "person_id".to_string(),
                    value: Some(person_id.clone()),
                    error_type: ValidationErrorType::Duplicate,
                    message: format!(
                        "person_id '{}' komt meerdere keren voor (rij {}, {})",
                        person_id, first_row, row
                    ),
                    severity: ErrorSeverity::Error,
                });
            } else {
                seen_person_ids.insert(person_id.clone(), row);
            }

            // Check unique email if present (ONLY duplicate check)
            if let Some(email) = &person.email {
                if !email.is_empty() {
                    if let Some(first_row) = seen_emails.get(email) {
                        errors.push(ValidationError {
                            row,
                            field: "person_email".to_string(),
                            value: Some(email.clone()),
                            error_type: ValidationErrorType::Duplicate,
                            message: format!(
                                "person_email '{}' komt meerdere keren voor (rij {}, {})",
                                email, first_row, row
                            ),
                            severity: ErrorSeverity::Error,
                        });
                    } else {
                        seen_emails.insert(email.clone(), row);
                    }
                }
            }

            // NO email format validation
            // NO missing field validation
            // NO name field validation
            // Accept ANY input except duplicates
        }

        // Count unique rows with errors
        let rows_with_errors: HashSet<_> = errors
            .iter()
            .filter(|e| matches!(e.severity, ErrorSeverity::Error))
            .map(|e| e.row)
            .collect();
        let unique_error_rows = rows_with_errors.len();

        ValidationResult {
            valid: unique_error_rows == 0,
            total_rows: records.len(),
            valid_rows: records.len().saturating_sub(unique_error_rows),
            error_rows: unique_error_rows,
            warning_rows: 0, // No warnings anymore
            errors,
        }
    }
}
```

**Samenvatting Validatie Regels:**

- ✅ **person_id duplicaat** → ERROR (blocking)
- ✅ **email duplicaat** → ERROR (blocking) indien aanwezig
- ✅ **org_id duplicaat** → ERROR (blocking) indien aanwezig
- ❌ Ontbrekende person_id → geen error, auto-generatie in service layer
- ❌ Ontbrekende email → geen error, auto-generatie in service layer
- ❌ Ongeldige email formaat → geen error, accepteer zoals het is
- ❌ Ontbrekende namen → geen error, placeholder in merge layer
- ❌ Ontbrekende org_id → geen error, auto-generatie in service layer
- ❌ Ontbrekende org_name → geen error, placeholder in merge layer
- ❌ Budget formaat → geen error, accepteer zoals het is
  field: Some("person_email".to_string()),
  value: None,
  error_type: ErrorType::MissingField,
  message: "person_email ontbreekt - record kan later worden aangevuld".to_string(),
  severity: Severity::Warning, // Changed from Error
  });
  } else {
  let email = person.email.as_ref().unwrap();

                  // Check unique email (ERROR - blocking)
                  if let Some(first_row) = seen_emails.get(email) {
                      errors.push(ValidationError {
                          row_number: row_num,
                          field: Some("person_email".to_string()),
                          value: Some(email.clone()),
                          error_type: ErrorType::Duplicate,
                          message: format!(
                              "person_email '{}' komt meerdere keren voor (rij {}, {})",
                              email, first_row, row_num
                          ),
                          severity: Severity::Error, // Still an error
                      });
                  } else {
                      seen_emails.insert(email.clone(), row_num);
                  }

                  // Check email format (WARNING only)
                  if !email.contains('@') {
                      errors.push(ValidationError {
                          row_number: row_num,
                          field: Some("person_email".to_string()),
                          value: Some(email.clone()),
                          error_type: ErrorType::FormatError,
                          message: "Ongeldig email formaat".to_string(),
                          severity: Severity::Warning, // Changed from Error
                      });
                  }
              }

              // Check name fields (WARNING only - not blocking)
              if person.first_name.is_none() && person.full_name.is_none() {
                  errors.push(ValidationError {
                      row_number: row_num,
                      field: Some("first_name/full_name".to_string()),
                      value: None,
                      error_type: ErrorType::MissingField,
                      message: "first_name of full_name ontbreekt - kan later worden aangevuld".to_string(),
                      severity: Severity::Warning, // Changed from Error
                  });
              }
          }

          // Only count rows with ERROR severity as invalid
          let error_rows: HashSet<_> = errors
              .iter()
              .filter(|e| matches!(e.severity, Severity::Error))
              .map(|e| e.row_number)
              .collect();

          ValidationResult {
              valid: error_rows.is_empty(), // Only errors block import
              total_rows: records.len(),
              valid_rows: records.len() - error_rows.len(),
              invalid_rows: error_rows.len(),
              errors,
          }
      }

  }

````

### Placeholder System en Auto-generatie

**Introductie (2026-02-23):** Om flexibele imports mogelijk te maken zonder strikte validatie, gebruikt het systeem een intelligent placeholder en auto-generatie mechanisme voor ontbrekende verplichte velden.

#### Auto-generatie Regels

**Voor Ontbrekende IDs:**

| Veld | Aanwezig | Auto-generatie Regel | Voorbeeld |
|------|----------|---------------------|-----------|
| `person_id` | `email` aanwezig | `AUTO_{email_prefix}` | email: `john.doe@equans.com` → `AUTO_john.doe` |
| `person_id` | `email` ontbreekt | `AUTO_{UUID}` | `AUTO_a3f2b8c9-4d5e-6f7g-8h9i-0j1k2l3m4n5o` |
| `org_id` | `org_name` aanwezig | `ORG_{name_sanitized}` | name: "IT Consulting" → `ORG_IT_Consulting` |
| `org_id` | `org_name` ontbreekt | `ORG_{UUID}` | `ORG_b1c2d3e4-5f6g-7h8i-9j0k-1l2m3n4o5p6q` |
| `email` | `person_id` aanwezig | `unknown_{person_id}@placeholder.local` | person_id: `CCJ183` → `unknown_CCJ183@placeholder.local` |
| `email` | `person_id` ontbreekt | `unknown_AUTO_{UUID}@placeholder.local` | `unknown_AUTO_abc123@placeholder.local` |

**Voor Ontbrekende Naam Velden:**

| Veld | Placeholder Waarde |
|------|-------------------|
| `first_name` | `"[To Be Determined]"` |
| `last_name` | `"[To Be Determined]"` |
| `org_name` | `"[Organization Name To Be Determined]"` |

#### Merge Prioriteit

Bij het samenvoegen van nieuwe import data met bestaande database records gebruikt het systeem deze prioriteit:

**Voor Reguliere Velden:**
1. **Import waarde** (als niet-leeg en geen placeholder)
2. **Database waarde** (als niet-leeg en geen placeholder)
3. **Placeholder waarde** (als laatste optie)

**Implementatie:**

```rust
/// Merge string field with placeholder support
fn merge_string_with_placeholder(
    import_val: Option<&String>,
    db_val: Option<&String>,
    placeholder: &str,
) -> String {
    // Helper to check if value is a placeholder
    let is_placeholder = |s: &str| {
        s == "[To Be Determined]"
        || s == "[Organization Name To Be Determined]"
        || s.starts_with("unknown_") && s.ends_with("@placeholder.local")
        || s.starts_with("AUTO_")
    };

    match (import_val, db_val) {
        // Import value is present and not placeholder
        (Some(imp), _) if !imp.is_empty() && !is_placeholder(imp) => imp.clone(),
        // DB value is present and not placeholder
        (_, Some(db)) if !db.is_empty() && !is_placeholder(db) => db.clone(),
        // Import value is placeholder
        (Some(imp), _) if !imp.is_empty() => imp.clone(),
        // DB value is placeholder
        (_, Some(db)) if !db.is_empty() => db.clone(),
        // Last resort: use placeholder
        _ => placeholder.to_string(),
    }
}

pub fn merge_person(
    import: &ParsedPerson,
    existing: Option<&Person>,
) -> Person {
    let db_person = existing;

    Person {
        person_id: import.id.clone().unwrap_or_else(|| {
            // Auto-generation handled in service layer
            "AUTO_UNKNOWN".to_string()
        }),
        first_name: merge_string_with_placeholder(
            import.first_name.as_ref(),
            db_person.and_then(|p| p.first_name.as_ref()),
            "[To Be Determined]"
        ),
        last_name: merge_string_with_placeholder(
            import.last_name.as_ref(),
            db_person.and_then(|p| p.last_name.as_ref()),
            "[To Be Determined]"
        ),
        email: merge_string_with_placeholder(
            import.email.as_ref(),
            db_person.map(|p| &p.email),
            "unknown@placeholder.local"
        ),
        // ... other fields
    }
}
````

#### Voorbeelden

**Voorbeeld 1: Nieuwe Record met Ontbrekende Email**

```csv
person_id,first_name,last_name,email
CCJ999,Jane,Smith,
```

**Resultaat:**

- `person_id`: `"CCJ999"` (uit import)
- `email`: `"unknown_CCJ999@placeholder.local"` (auto-gegenereerd)
- `first_name`: `"Jane"` (uit import)
- `last_name`: `"Smith"` (uit import)

**Voorbeeld 2: Update met Ontbrekende Namen**

Database heeft:

```
person_id: CCJ183, email: john.doe@equans.com, first_name: "John", last_name: "Doe"
```

Import heeft:

```csv
person_id,first_name,last_name,email
CCJ183,,,john.doe@equans.com
```

**Resultaat:**

- `person_id`: `"CCJ183"` (uit import)
- `email`: `"john.doe@equans.com"` (uit import)
- `first_name`: `"John"` (behouden uit database)
- `last_name`: `"Doe"` (behouden uit database)

**Voorbeeld 3: Nieuwe Record zonder IDs**

```csv
person_id,first_name,last_name,email
,Alice,Johnson,alice.johnson@equans.com
```

**Resultaat:**

- `person_id`: `"AUTO_alice.johnson"` (auto-gegenereerd uit email)
- `email`: `"alice.johnson@equans.com"` (uit import)
- `first_name`: `"Alice"` (uit import)
- `last_name`: `"Johnson"` (uit import)

**Voorbeeld 4: Volledige Placeholder Record**

```csv
person_id,first_name,last_name,email
,,,
```

**Resultaat:**

- `person_id`: `"AUTO_c4f7b2e9-..."` (auto-gegenereerd UUID)
- `email`: `"unknown_AUTO_c4f7b2e9@placeholder.local"` (auto-gegenereerd)
- `first_name`: `"[To Be Determined]"` (placeholder)
- `last_name`: `"[To Be Determined]"` (placeholder)

### Merge Logic

```rust
pub struct MergeEngine;

impl MergeEngine {
    /// Merge import person with database person
    /// Rule: Import priority EXCEPT when import field empty and DB field filled
    pub fn merge_person(
        db_person: &Person,
        import_person: &ParsedPerson,
    ) -> Person {
        Person {
            id: db_person.id,
            person_id: db_person.person_id.clone(),

            // Merge fields with logic
            first_name: Self::merge_field(
                &import_person.first_name,
                &db_person.first_name,
                true, // required field
            ),

            last_name: Self::merge_field(
                &import_person.last_name,
                &db_person.last_name,
                true, // required field
            ),

            email: Self::merge_field(
                &import_person.email,
                &db_person.email,
                true, // required field
            ),

            // Optional fields use same logic
            local_id: Self::merge_optional_field(
                &import_person.local_id,
                &db_person.local_id,
            ),

            language: Self::merge_optional_field(
                &import_person.language,
                &db_person.language,
            ),

            country: Self::merge_optional_field(
                &import_person.country,
                &db_person.country,
            ),

            job_title: Self::merge_optional_field(
                &import_person.job_title,
                &db_person.job_title,
            ),

            org_id: Self::merge_optional_field(
                &import_person.org_id,
                &db_person.org_id,
            ),

            gid: Self::merge_optional_field(
                &import_person.gid,
                &db_person.gid,
            ),

            // Preserve existing fields not in import
            status: db_person.status.clone(),
            source: db_person.source.clone(),
            gid_confidence: db_person.gid_confidence,
            vendor_identifiers: db_person.vendor_identifiers.clone(),

            // Update timestamp
            created_at: db_person.created_at,
            updated_at: Utc::now(),
        }
    }

    /// Merge required field
    fn merge_field(
        import_value: &str,
        db_value: &str,
        _required: bool,
    ) -> String {
        if import_value.is_empty() {
            // Import empty, keep database value
            db_value.to_string()
        } else {
            // Import has value, use it
            import_value.to_string()
        }
    }

    /// Merge optional field
    fn merge_optional_field(
        import_value: &Option<String>,
        db_value: &Option<String>,
    ) -> Option<String> {
        match (import_value, db_value) {
            (Some(imp), _) if !imp.is_empty() => {
                // Import has non-empty value, use it
                Some(imp.clone())
            }
            (Some(imp), Some(db)) if imp.is_empty() => {
                // Import empty but DB has value, keep DB
                Some(db.clone())
            }
            (None, Some(db)) => {
                // Import not provided, keep DB
                Some(db.clone())
            }
            _ => None,
        }
    }
}
```

### Preview Generation

```rust
pub struct PreviewGenerator;

impl PreviewGenerator {
    pub async fn generate_preview(
        &self,
        organizations: Vec<ParsedOrganization>,
        persons: Vec<ParsedPerson>,
        pool: &PgPool,
    ) -> Result<ImportPreview, PreviewError> {
        let mut preview = ImportPreview::default();

        // Get existing organizations from DB
        let existing_orgs = self.get_existing_organizations(pool).await?;
        let existing_org_ids: HashSet<String> =
            existing_orgs.iter().map(|o| o.org_id.clone()).collect();

        // Categorize organizations
        for org in organizations {
            if existing_org_ids.contains(&org.org_id) {
                preview.organizations.updated += 1;
                preview.organizations.updated_ids.push(org.org_id.clone());
            } else {
                preview.organizations.new += 1;
                preview.organizations.new_ids.push(org.org_id.clone());
            }
        }

        // Get existing persons from DB
        let existing_persons = self.get_existing_persons(pool).await?;
        let existing_person_ids: HashSet<String> =
            existing_persons.iter().map(|p| p.person_id.clone()).collect();
        let import_person_ids: HashSet<String> =
            persons.iter().map(|p| p.person_id.clone()).collect();

        // Categorize persons
        for person in &persons {
            if let Some(db_person) = existing_persons.iter()
                .find(|p| p.person_id == person.person_id)
            {
                // Check if reactivation
                if db_person.status == "Inactive" {
                    preview.persons.reactivated += 1;
                    preview.persons.reactivated_ids.push(person.person_id.clone());
                } else {
                    preview.persons.updated += 1;
                    preview.persons.updated_ids.push(person.person_id.clone());
                }

                // Generate field changes
                let changes = self.generate_field_changes(db_person, person);
                if !changes.is_empty() {
                    preview.changes.push(PersonChange {
                        record_type: "Person".to_string(),
                        person_id: person.person_id.clone(),
                        change_type: "Update".to_string(),
                        field_changes: changes,
                    });
                }
            } else {
                preview.persons.new += 1;
                preview.persons.new_ids.push(person.person_id.clone());
            }
        }

        // Find soft-deletes (in DB but not in import, and currently Active)
        for db_person in &existing_persons {
            if !import_person_ids.contains(&db_person.person_id)
                && db_person.status == "Active"
            {
                preview.persons.soft_deleted += 1;
                preview.persons.soft_deleted_ids.push(db_person.person_id.clone());
            }
        }

        Ok(preview)
    }

    fn generate_field_changes(
        &self,
        db_person: &Person,
        import_person: &ParsedPerson,
    ) -> Vec<FieldChange> {
        let mut changes = Vec::new();

        // Check each field
        self.check_field_change(
            &mut changes,
            "first_name",
            &db_person.first_name,
            &import_person.first_name,
        );

        self.check_field_change(
            &mut changes,
            "last_name",
            &db_person.last_name,
            &import_person.last_name,
        );

        // ... check all other fields

        changes
    }

    fn check_field_change(
        &self,
        changes: &mut Vec<FieldChange>,
        field_name: &str,
        db_value: &str,
        import_value: &str,
    ) {
        let changed = db_value != import_value;
        let kept_old = import_value.is_empty() && !db_value.is_empty();

        changes.push(FieldChange {
            field: field_name.to_string(),
            old_value: Some(db_value.to_string()),
            new_value: if import_value.is_empty() {
                None
            } else {
                Some(import_value.to_string())
            },
            changed,
            kept_old,
        });
    }
}
```

### Import Execution

```rust
pub struct ImportService {
    repository: ImportRepository,
    merger: MergeEngine,
}

impl ImportService {
    pub async fn execute_import(
        &self,
        preview_id: &str,
        organizations: Vec<ParsedOrganization>,
        persons: Vec<ParsedPerson>,
        pool: &PgPool,
    ) -> Result<ImportResult, ImportError> {
        // Start transaction
        let mut tx = pool.begin().await?;

        let mut stats = ImportStats::default();

        // Import organizations
        for org in organizations {
            match self.import_organization(&mut tx, &org).await {
                Ok(is_new) => {
                    if is_new {
                        stats.organizations_added += 1;
                    } else {
                        stats.organizations_updated += 1;
                    }
                }
                Err(e) => {
                    // Rollback transaction
                    tx.rollback().await?;
                    return Err(ImportError::DatabaseError(e.to_string()));
                }
            }
        }

        // Get existing persons for comparison
        let existing_persons = self.repository
            .get_all_persons(&mut tx)
            .await?;

        let import_person_ids: HashSet<String> =
            persons.iter().map(|p| p.person_id.clone()).collect();

        // Import persons (new + updates + reactivations)
        for person in persons {
            match self.import_person(&mut tx, &person, &existing_persons).await {
                Ok(action) => {
                    match action {
                        PersonAction::Added => stats.persons_added += 1,
                        PersonAction::Updated => stats.persons_updated += 1,
                        PersonAction::Reactivated => stats.persons_reactivated += 1,
                    }
                }
                Err(e) => {
                    tx.rollback().await?;
                    return Err(ImportError::DatabaseError(e.to_string()));
                }
            }
        }

        // Soft-delete persons not in import (Active only)
        for db_person in existing_persons {
            if !import_person_ids.contains(&db_person.person_id)
                && db_person.status == "Active"
            {
                match self.soft_delete_person(&mut tx, &db_person.person_id).await {
                    Ok(_) => stats.persons_soft_deleted += 1,
                    Err(e) => {
                        tx.rollback().await?;
                        return Err(ImportError::DatabaseError(e.to_string()));
                    }
                }
            }
        }

        // Commit transaction
        tx.commit().await?;

        Ok(ImportResult {
            success: true,
            organizations: OrganizationStats {
                added: stats.organizations_added,
                updated: stats.organizations_updated,
            },
            persons: PersonStats {
                added: stats.persons_added,
                updated: stats.persons_updated,
                soft_deleted: stats.persons_soft_deleted,
                reactivated: stats.persons_reactivated,
            },
        })
    }

    async fn import_organization(
        &self,
        tx: &mut PgTransaction<'_>,
        org: &ParsedOrganization,
    ) -> Result<bool, sqlx::Error> {
        // Check if exists
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM organizations WHERE org_id = $1)"
        )
        .bind(&org.org_id)
        .fetch_one(&mut **tx)
        .await?;

        if exists {
            // Update
            sqlx::query(
                "UPDATE organizations SET name = $2, updated_at = NOW() WHERE org_id = $1"
            )
            .bind(&org.org_id)
            .bind(&org.name)
            .execute(&mut **tx)
            .await?;
            Ok(false)
        } else {
            // Insert
            sqlx::query(
                "INSERT INTO organizations (org_id, name) VALUES ($1, $2)"
            )
            .bind(&org.org_id)
            .bind(&org.name)
            .execute(&mut **tx)
            .await?;
            Ok(true)
        }
    }

    async fn import_person(
        &self,
        tx: &mut PgTransaction<'_>,
        person: &ParsedPerson,
        existing_persons: &[Person],
    ) -> Result<PersonAction, sqlx::Error> {
        if let Some(db_person) = existing_persons.iter()
            .find(|p| p.person_id == person.person_id)
        {
            // Check if reactivation
            if db_person.status == "Inactive" {
                // Reactivate
                let merged = self.merger.merge_person(db_person, person);
                self.update_person_with_status(tx, &merged, "Active").await?;
                return Ok(PersonAction::Reactivated);
            }

            // Update with merge logic
            let merged = self.merger.merge_person(db_person, person);
            self.update_person(tx, &merged).await?;
            Ok(PersonAction::Updated)
        } else {
            // Insert new
            self.insert_person(tx, person).await?;
            Ok(PersonAction::Added)
        }
    }

    async fn soft_delete_person(
        &self,
        tx: &mut PgTransaction<'_>,
        person_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE persons SET status = 'Inactive', updated_at = NOW() WHERE person_id = $1"
        )
        .bind(person_id)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }
}
```

---

## Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum ImportError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Validation failed: {0} errors")]
    ValidationError(usize),

    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),

    #[error("File too large: {0} bytes (max 50MB)")]
    FileTooLarge(usize),

    #[error("Import not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Transaction failed, rollback completed")]
    TransactionFailed,
}

impl IntoResponse for ImportError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ImportError::ParseError(msg) => (
                StatusCode::BAD_REQUEST,
                format!("Failed to parse file: {}", msg)
            ),
            ImportError::ValidationError(count) => (
                StatusCode::BAD_REQUEST,
                format!("Validation failed with {} errors", count)
            ),
            ImportError::UnsupportedFormat(fmt) => (
                StatusCode::BAD_REQUEST,
                format!("Unsupported format: {}. Use CSV or Excel (.xlsx)", fmt)
            ),
            ImportError::FileTooLarge(size) => (
                StatusCode::PAYLOAD_TOO_LARGE,
                format!("File too large: {} bytes (max 50MB)", size)
            ),
            ImportError::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Import {} not found", id)
            ),
            ImportError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", msg)
            ),
            ImportError::TransactionFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Import failed, all changes rolled back".to_string()
            ),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

---

## Security

### Admin Authorization

```rust
pub async fn admin_only(
    claims: Claims,
) -> Result<Claims, StatusCode> {
    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(claims)
}

// Use in routes
async fn upload_file(
    admin: Admin,  // Extractor checks admin role
    multipart: Multipart,
) -> Result<Json<UploadResponse>, ImportError> {
    // ...
}
```

### File Size Limit

```rust
const MAX_FILE_SIZE: usize = 50 * 1024 * 1024; // 50MB

pub async fn check_file_size(
    file_data: &[u8],
) -> Result<(), ImportError> {
    if file_data.len() > MAX_FILE_SIZE {
        return Err(ImportError::FileTooLarge(file_data.len()));
    }
    Ok(())
}
```

### Input Sanitization

```rust
pub fn sanitize_import_data(record: &mut HashMap<String, String>) {
    for (_key, value) in record.iter_mut() {
        // Trim whitespace
        *value = value.trim().to_string();

        // Remove null bytes
        *value = value.replace('\0', "");

        // Limit length
        if value.len() > 1000 {
            value.truncate(1000);
        }
    }
}
```

---

## Testing Strategie

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_logic_empty_import() {
        let db_person = Person {
            first_name: "Thomas".to_string(),
            country: Some("Austria".to_string()),
            ..Default::default()
        };

        let import_person = ParsedPerson {
            first_name: "Thomas".to_string(),
            country: None, // Empty in import
            ..Default::default()
        };

        let merged = MergeEngine::merge_person(&db_person, &import_person);

        // Should keep database value when import is empty
        assert_eq!(merged.country, Some("Austria".to_string()));
    }

    #[test]
    fn test_validation_duplicate_person_id() {
        let persons = vec![
            ParsedPerson {
                person_id: "CCJ183".to_string(),
                email: "test1@equans.com".to_string(),
                ..Default::default()
            },
            ParsedPerson {
                person_id: "CCJ183".to_string(),
                email: "test2@equans.com".to_string(),
                ..Default::default()
            },
        ];

        let result = Validator::validate_persons(&persons);

        assert!(!result.valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].error_type, ErrorType::Duplicate);
    }
}
```

### Integration Tests

```rust
#[sqlx::test]
async fn test_import_execution(pool: PgPool) {
    let service = ImportService::new(pool.clone());

    // Insert test organization
    sqlx::query("INSERT INTO organizations (org_id, name) VALUES ('ORG0042', 'Test Org')")
        .execute(&pool)
        .await
        .unwrap();

    // Import persons
    let persons = vec![
        ParsedPerson {
            person_id: "TEST001".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@equans.com".to_string(),
            org_id: Some("ORG0042".to_string()),
            ..Default::default()
        },
    ];

    let result = service
        .execute_import("preview-1", vec![], persons, &pool)
        .await
        .unwrap();

    assert!(result.success);
    assert_eq!(result.persons.added, 1);

    // Verify in database
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM persons WHERE person_id = 'TEST001'")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(count, 1);
}
```

---

## Performance Optimalisatie

### Batch Insert

```rust
pub async fn batch_insert_persons(
    tx: &mut PgTransaction<'_>,
    persons: &[ParsedPerson],
) -> Result<(), sqlx::Error> {
    // Use COPY or multi-row INSERT for better performance
    let mut query_builder = QueryBuilder::new(
        "INSERT INTO persons (person_id, first_name, last_name, email, org_id)"
    );

    query_builder.push_values(persons.iter().take(500), |mut b, person| {
        b.push_bind(&person.person_id)
            .push_bind(&person.first_name)
            .push_bind(&person.last_name)
            .push_bind(&person.email)
            .push_bind(&person.org_id);
    });

    query_builder.build().execute(&mut **tx).await?;

    Ok(())
}
```

### Streaming for Large Files

```rust
pub async fn parse_large_csv_streaming<R: AsyncRead + Unpin>(
    reader: R,
) -> impl Stream<Item = Result<ParsedPerson, ParseError>> {
    let reader = BufReader::new(reader);
    let mut csv_reader = AsyncReaderBuilder::new()
        .has_headers(true)
        .create_reader(reader);

    stream::unfold(csv_reader, |mut reader| async move {
        match reader.deserialize::<ParsedPerson>().next().await {
            Some(Ok(person)) => Some((Ok(person), reader)),
            Some(Err(e)) => Some((Err(ParseError::from(e)), reader)),
            None => None,
        }
    })
}
```

---

## Gerelateerde Documenten

- Functional Requirement: [FR-007](../Functional-Requirements/FR-007-Data-Synchronization.md)
- Technical Requirement: [TR-005](TR-005-Person-Management.md)
- Technical Requirement: [TR-006](TR-006-Organization-Management.md)
- Business Requirement: [BR-002](../Business-Requirements/BR-002-Person-Organization-Management.md)
