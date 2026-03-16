# TR-005: Personen Beheer - Technische Specificaties

**Status:** In Review
**Datum:** 2026-02-18
**Auteur(s):** Backend Engineer
**Implementeert:** [FR-005](../Functional-Requirements/FR-005-Person-Management.md)
**Applies To:** Backend Rust applicatie, PostgreSQL database, React frontend

---

## Scope

Dit document definieert de technische specificaties voor het personen beheer systeem, inclusief:

- Database schema voor personen
- REST API endpoints
- Rust backend implementatie
- React frontend componenten
- Zoek en filter functionaliteit
- GID matching systeem
- Vendor identifiers structuur

---

## Architectuur Overzicht

```
┌───────────────────┐      ┌──────────────────┐      ┌──────────────────┐
│   React Frontend  │─────▶│   Rust Backend   │─────▶│   PostgreSQL     │
│   (Personen UI)   │◀─────│   (Axum)         │◀─────│   (persons)      │
└───────────────────┘      └──────────────────┘      └──────────────────┘
                                     │
                                     │
                                     ▼
                           ┌──────────────────┐
                           │  External APIs   │
                           │  (GitHub, Atl.)  │
                           └──────────────────┘
```

### Data Flow

1. Frontend maakt API call naar `/api/persons` met filters/zoekterm
2. Backend valideert query parameters en authenticatie
3. Database query wordt uitgevoerd met filters en paginering
4. Resultaten worden getransformeerd naar DTO's
5. Response inclusief metadata wordt naar frontend gestuurd
6. Frontend rendert resultaten in tabel of detail view

---

## Componenten Structuur

```
backend/
├── src/
│   ├── persons/
│   │   ├── mod.rs                  # Module exports
│   │   ├── types.rs                # Data types en DTOs
│   │   ├── repository.rs           # Database queries
│   │   ├── service.rs              # Business logic
│   │   └── error.rs                # Error handling
│   ├── routes/
│   │   └── persons.rs              # HTTP route handlers
│   └── main.rs
└── migrations/
    └── 002_persons_organizations.sql

frontend/
├── src/
│   ├── pages/
│   │   ├── PersonsList.tsx         # Overzicht pagina
│   │   └── PersonDetail.tsx        # Detail pagina
│   ├── components/
│   │   ├── PersonTable.tsx         # Tabel component
│   │   ├── PersonFilters.tsx       # Filter component
│   │   └── GidStatusBadge.tsx      # GID status indicator
│   └── api/
│       └── persons.ts              # API client calls
```

---

## Database Schema

### Persons Tabel

```sql
CREATE TABLE IF NOT EXISTS persons (
    id SERIAL PRIMARY KEY,
    person_id VARCHAR(20) NOT NULL UNIQUE,  -- e.g., CCJ183
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    local_id VARCHAR(100),  -- person_id@equans.com
    language VARCHAR(10) DEFAULT 'EN',
    billing_location VARCHAR(10),
    country VARCHAR(100),
    job_title VARCHAR(255),
    department VARCHAR(255),
    manager VARCHAR(255),
    start_date DATE,
    org_id VARCHAR(20) REFERENCES organizations(org_id) ON DELETE SET NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Active',
    source VARCHAR(50) DEFAULT 'Excel Import',

    -- GID Matching fields
    gid VARCHAR(50),
    gid_confidence INTEGER DEFAULT 0,  -- 0-100
    gid_extraction_method VARCHAR(50),
    last_matched_at TIMESTAMPTZ,
    matching_metadata JSONB,

    -- Vendor identifiers
    vendor_identifiers JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Indexes

```sql
CREATE INDEX idx_persons_org ON persons(org_id);
CREATE INDEX idx_persons_email ON persons(email);
CREATE INDEX idx_persons_status ON persons(status);
CREATE INDEX idx_persons_country ON persons(country);
CREATE INDEX idx_persons_billing_location ON persons(billing_location);
CREATE INDEX idx_persons_name ON persons(last_name, first_name);
CREATE INDEX idx_persons_gid_status ON persons(gid_confidence);

-- Full text search index
CREATE INDEX idx_persons_search ON persons USING gin(
    to_tsvector('simple',
        coalesce(first_name, '') || ' ' ||
        coalesce(last_name, '') || ' ' ||
        coalesce(email, '') || ' ' ||
        coalesce(person_id, '')
    )
);
```

### vendor_identifiers JSONB Structuur

```json
{
  "github": {
    "username": "twagensonner",
    "org_member": true,
    "copilot_enabled": true,
    "last_activity": "2026-02-16T10:30:00Z"
  },
  "atlassian": {
    "account_id": "5f4e3d2c1b0a9876543210fe",
    "jira_access": true,
    "confluence_access": true,
    "last_activity": "2026-02-17T14:20:00Z"
  },
  "jfrog": {
    "username": "thomas.wagensonner",
    "repositories": ["docker-local", "npm-remote"],
    "last_activity": "2026-02-15T08:15:00Z"
  }
}
```

---

## API Specificaties

### Base URL

```
/api/persons
```

### Authenticatie

Alle endpoints vereisen JWT Bearer token met `user` of `admin` rol.

---

### GET /api/persons

**Beschrijving:** Haalt lijst van personen op met filtering, zoeken en paginering.

**Query Parameters:**

| Parameter          | Type    | Verplicht | Default     | Beschrijving                                     |
| ------------------ | ------- | --------- | ----------- | ------------------------------------------------ |
| `search`           | string  | Nee       | -           | Zoektekst voor naam, email of person_id          |
| `org_id`           | string  | Nee       | -           | Filter op organisatie                            |
| `country`          | string  | Nee       | -           | Filter op land (comma-separated)                 |
| `billing_location` | string  | Nee       | -           | Filter op billing location                       |
| `language`         | string  | Nee       | -           | Filter op taal                                   |
| `status`           | string  | Nee       | `Active`    | Filter op status (Active/Inactive)               |
| `gid_status`       | string  | Nee       | -           | Filter op GID status (matched/pending/unmatched) |
| `page`             | integer | Nee       | `1`         | Paginanummer                                     |
| `per_page`         | integer | Nee       | `25`        | Aantal resultaten per pagina (max 100)           |
| `sort_by`          | string  | Nee       | `last_name` | Sorteer kolom                                    |
| `sort_order`       | string  | Nee       | `asc`       | Sorteer richting (asc/desc)                      |

**Response (200 OK):**

```json
{
  "data": [
    {
      "person_id": "CCJ183",
      "name": "Thomas WAGENSONNER",
      "email": "thomas.wagensonner@equans.com",
      "org_id": "ORG0042",
      "country": "Austria",
      "billing_location": "AT",
      "status": "Active",
      "gid_status": "matched"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 25,
    "total": 187,
    "total_pages": 8
  }
}
```

**Error Responses:**

- `400 Bad Request` - Invalid query parameters
- `401 Unauthorized` - Missing or invalid JWT token
- `500 Internal Server Error` - Database error

---

### GET /api/persons/:person_id

**Beschrijving:** Haalt volledige details van één persoon op.

**Path Parameters:**

| Parameter   | Type   | Beschrijving                             |
| ----------- | ------ | ---------------------------------------- |
| `person_id` | string | Unieke persoon identifier (bijv. CCJ183) |

**Response (200 OK):**

```json
{
  "person_id": "CCJ183",
  "first_name": "Thomas",
  "last_name": "WAGENSONNER",
  "email": "thomas.wagensonner@equans.com",
  "local_id": "CCJ183@equans.com",
  "language": "DE",
  "billing_location": "AT",
  "country": "Austria",
  "job_title": "Senior Engineer",
  "department": "Engineering",
  "manager": "Hans Mueller",
  "start_date": "2020-03-15",
  "org_id": "ORG0042",
  "status": "Active",
  "source": "Azure AD",

  "gid": "thomas.wagensonner",
  "gid_confidence": 95,
  "gid_extraction_method": "email_prefix",
  "gid_status": "matched",
  "last_matched_at": "2026-02-17T10:00:00Z",
  "matching_metadata": {
    "method": "email_prefix",
    "confidence_score": 95,
    "matched_at": "2026-02-17T10:00:00Z"
  },

  "vendor_identifiers": {
    "github": {
      "username": "twagensonner",
      "org_member": true,
      "copilot_enabled": true,
      "last_activity": "2026-02-16T10:30:00Z"
    },
    "atlassian": {
      "account_id": "5f4e3d2c1b0a9876543210fe",
      "jira_access": true,
      "confluence_access": true,
      "last_activity": "2026-02-17T14:20:00Z"
    }
  },

  "created_at": "2025-10-15T08:58:32Z",
  "updated_at": "2026-01-05T11:20:00Z"
}
```

**Error Responses:**

- `404 Not Found` - Person not found
- `401 Unauthorized` - Missing or invalid JWT token
- `500 Internal Server Error` - Database error

---

### GET /api/persons/export

**Beschrijving:** Exporteert personen lijst naar CSV formaat.

**Query Parameters:**

Zelfde als GET /api/persons (search, filters), maar zonder paginering.

**Response (200 OK):**

```csv
person_id,first_name,last_name,email,org_id,country,billing_location,status
CCJ183,Thomas,WAGENSONNER,thomas.wagensonner@equans.com,ORG0042,Austria,AT,Active
DEI311,Jürg,RUPPANNER,juerg.ruppanner@equans.com,ORG0042,Switzerland,CH,Active
```

**Headers:**

```
Content-Type: text/csv; charset=utf-8
Content-Disposition: attachment; filename="persons_export_2026-02-18.csv"
```

---

## Rust Implementatie Details

### Data Types

```rust
/// Person database model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub person_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub local_id: Option<String>,
    pub language: Option<String>,
    pub billing_location: Option<String>,
    pub country: Option<String>,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub manager: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub org_id: Option<String>,
    pub status: String,
    pub source: Option<String>,
    pub gid: Option<String>,
    pub gid_confidence: Option<i32>,
    pub gid_extraction_method: Option<String>,
    pub last_matched_at: Option<DateTime<Utc>>,
    pub matching_metadata: Option<serde_json::Value>,
    pub vendor_identifiers: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// GID matching status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GidStatus {
    Matched,    // confidence >= 80
    Pending,    // confidence 50-79
    Unmatched,  // confidence < 50 or null
}

/// Query parameters for person list
#[derive(Debug, Deserialize, Default)]
pub struct PersonListParams {
    pub search: Option<String>,
    pub org_id: Option<String>,
    pub country: Option<String>,
    pub billing_location: Option<String>,
    pub language: Option<String>,
    pub status: Option<String>,
    pub gid_status: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}
```

### Repository Pattern

```rust
pub struct PersonRepository {
    pool: PgPool,
}

impl PersonRepository {
    // Get paginated list with filters
    pub async fn list(
        &self,
        params: PersonListParams,
    ) -> Result<(Vec<Person>, i64), sqlx::Error> {
        // Build dynamic query based on filters
        // Apply full-text search if search param present
        // Apply pagination
        // Return results + total count
    }

    // Get single person by ID
    pub async fn get_by_id(
        &self,
        person_id: &str,
    ) -> Result<Option<Person>, sqlx::Error> {
        // Simple SELECT by person_id
    }

    // Update person
    pub async fn update(
        &self,
        person_id: &str,
        updates: PersonUpdate,
    ) -> Result<Person, sqlx::Error> {
        // UPDATE with timestamp trigger
    }

    // Get person count by org
    pub async fn count_by_org(
        &self,
        org_id: &str,
    ) -> Result<i64, sqlx::Error> {
        // COUNT for organization statistics
    }
}
```

### Service Layer

```rust
pub struct PersonService {
    repository: PersonRepository,
}

impl PersonService {
    // Business logic for listing persons
    pub async fn list_persons(
        &self,
        params: PersonListParams,
    ) -> Result<PersonListResponse, AppError> {
        // Validate params
        // Call repository
        // Transform to DTOs
        // Build pagination metadata
    }

    // Business logic for person details
    pub async fn get_person_detail(
        &self,
        person_id: &str,
    ) -> Result<PersonDetail, AppError> {
        // Get person from repository
        // Return 404 if not found
        // Transform to detail DTO
    }

    // Export to CSV
    pub async fn export_persons(
        &self,
        params: PersonListParams,
    ) -> Result<String, AppError> {
        // Get all persons matching filters (no pagination)
        // Generate CSV string
    }
}
```

---

## Frontend Implementatie

### React Components

#### PersonsList Component

```typescript
interface PersonsListProps {
  // No props needed - manages own state
}

export const PersonsList: React.FC<PersonsListProps> = () => {
  const [persons, setPersons] = useState<PersonSummary[]>([]);
  const [filters, setFilters] = useState<PersonFilters>({});
  const [pagination, setPagination] = useState<Pagination>();
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    fetchPersons(filters, pagination?.page);
  }, [filters, pagination?.page]);

  return (
    <div>
      <PersonFilters
        filters={filters}
        onFilterChange={setFilters}
      />
      <PersonTable
        persons={persons}
        loading={loading}
      />
      <Pagination
        {...pagination}
        onPageChange={handlePageChange}
      />
    </div>
  );
};
```

#### PersonDetail Component

```typescript
interface PersonDetailProps {
  personId: string;
}

export const PersonDetail: React.FC<PersonDetailProps> = ({ personId }) => {
  const [person, setPerson] = useState<PersonDetail | null>(null);
  const [activeTab, setActiveTab] = useState<'general' | 'vendors' | 'matching'>('general');

  useEffect(() => {
    fetchPersonDetail(personId);
  }, [personId]);

  return (
    <div>
      <PersonHeader person={person} />
      <Tabs activeTab={activeTab} onTabChange={setActiveTab}>
        <Tab name="general">
          <GeneralInfo person={person} />
        </Tab>
        <Tab name="vendors">
          <VendorIdentifiers identifiers={person?.vendor_identifiers} />
        </Tab>
        <Tab name="matching">
          <GidMatchingInfo person={person} />
        </Tab>
      </Tabs>
    </div>
  );
};
```

---

## Zoeken en Filteren

### Full-Text Search

PostgreSQL GIN index wordt gebruikt voor snelle full-text search:

```sql
SELECT * FROM persons
WHERE to_tsvector('simple',
    first_name || ' ' || last_name || ' ' || email || ' ' || person_id
) @@ plainto_tsquery('simple', $1)
LIMIT $2 OFFSET $3;
```

### Implementatie in Rust

```rust
let mut query = String::from("SELECT * FROM persons WHERE 1=1");
let mut params: Vec<Box<dyn ToSql + Send>> = Vec::new();
let mut param_count = 1;

// Add search condition
if let Some(search) = &list_params.search {
    query.push_str(&format!(
        " AND to_tsvector('simple', first_name || ' ' || last_name || ' ' || email || ' ' || person_id) @@ plainto_tsquery('simple', ${})",
        param_count
    ));
    params.push(Box::new(search.clone()));
    param_count += 1;
}

// Add org_id filter
if let Some(org_id) = &list_params.org_id {
    query.push_str(&format!(" AND org_id = ${}", param_count));
    params.push(Box::new(org_id.clone()));
    param_count += 1;
}

// Add country filter (multi-select, comma-separated)
if let Some(countries) = &list_params.country {
    let country_list: Vec<&str> = countries.split(',').collect();
    query.push_str(&format!(" AND country = ANY(${})", param_count));
    params.push(Box::new(country_list));
    param_count += 1;
}

// Add pagination
query.push_str(&format!(" LIMIT ${} OFFSET ${}", param_count, param_count + 1));
```

---

## GID Matching Systeem

### Confidence Berekening

```rust
pub fn calculate_gid_confidence(
    person: &Person,
    gid: &str,
) -> i32 {
    let mut confidence = 50; // Base score for having an email with extracted GID

    // @equans.com domain (+30)
    // Rationale: @equans.com emails are highly trustworthy for GID extraction
    // This ensures persons with only email match reach "matched" status (80+)
    if person.email.ends_with("@equans.com") {
        confidence += 30;
    }

    // Local ID match (+30)
    if let Some(local_id) = &person.local_id {
        if local_id.to_lowercase().contains(gid) {
            confidence += 30;
        }
    }

    // Vendor identifier match (+20)
    if person.vendor_identifiers
        .as_ref()
        .and_then(|v| v.get("github"))
        .and_then(|g| g.get("username"))
        .and_then(|u| u.as_str())
        .map(|u| u == gid)
        .unwrap_or(false)
    {
        confidence += 20;
    }

    confidence.min(100) // Cap at 100
}

pub fn determine_gid_status(confidence: i32) -> GidStatus {
    match confidence {
        80..=100 => GidStatus::Matched,    // High confidence
        50..=79 => GidStatus::Pending,     // Medium confidence
        _ => GidStatus::Unmatched,         // Low confidence or no match
    }
}
```

**Confidence Score Breakdown:**

| Factor             | Points  | Beschrijving                              |
| ------------------ | ------- | ----------------------------------------- |
| Base score         | +50     | Valid email met succesvolle GID extractie |
| @equans.com domain | +30     | Betrouwbaar corporate email domein        |
| Local ID match     | +30     | local_id bevat de GID                     |
| Vendor identifier  | +20     | GitHub/Atlassian username match           |
| **Maximum**        | **100** | Totale score wordt gecapped op 100        |

**Status Thresholds:**

| Score Range | Status        | Beschrijving                                             |
| ----------- | ------------- | -------------------------------------------------------- |
| 80-100      | **Matched**   | Hoge betrouwbaarheid                                     |
| 50-79       | **Pending**   | Medium betrouwbaarheid (typisch niet-@equans.com emails) |
| 0-49        | **Unmatched** | Lage betrouwbaarheid of special characters in email      |

**Typische Scenarios:**

- `name@equans.com` zonder local_id: **80 punten** (50 + 30) → **MATCHED** ✅
- `name@equans.com` met local_id match: **100 punten** (50 + 30 + 30, capped) → **MATCHED** ✅
- `name@other-domain.com`: **50 punten** (50) → **PENDING** ⏳
- `m'hammed@equans.com` (special chars): **<50 punten** → **UNMATCHED** ⚠️

**Productie Statistieken (23-02-2026):**

- **Matched:** 64,539 personen (75.2%) - Confidence ≥80
- **Pending:** 21,228 personen (24.7%) - Confidence 50-79
- **Unmatched:** 26 personen (0.03%) - Confidence <50

---

## Performance Optimalisatie

### Database Query Optimalisatie

1. **Indexes:** Alle vaak gefilterde kolommen hebben indexes
2. **Limited Results:** Max 1000 resultaten per query
3. **Connection Pooling:** PgPool met max 20 connections
4. **Prepared Statements:** Gebruik van sqlx query macros waar mogelijk

### Caching Strategie

- **Server-side:** Geen caching (data wijzigt frequent)
- **Client-side:** React Query met 5 minuten stale time
- **CDN:** Geen statische data

### Paginering

- Default: 25 items per pagina
- Max: 100 items per pagina
- Totaal count via separate COUNT query (gecached voor 1 minuut)

---

## Security

### Authenticatie

- JWT Bearer token vereist voor alle endpoints
- Token bevat user_id en role
- Token expiration: 24 uur

### Autorisatie

- **Users:** Kunnen alleen hun eigen persoon bekijken
- **Admins:** Kunnen alle personen bekijken en bewerken

### Data Validatie

```rust
pub fn validate_person_email(email: &str) -> Result<(), ValidationError> {
    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();

    if !email_regex.is_match(email) {
        return Err(ValidationError::InvalidEmail);
    }

    if !email.ends_with("@equans.com") {
        return Err(ValidationError::InvalidDomain);
    }

    Ok(())
}

pub fn validate_person_id(person_id: &str) -> Result<(), ValidationError> {
    let id_regex = Regex::new(r"^[A-Z]{2,3}\d{3,4}$").unwrap();

    if !id_regex.is_match(person_id) {
        return Err(ValidationError::InvalidPersonId);
    }

    Ok(())
}
```

---

## Error Handling

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum PersonError {
    #[error("Person not found: {0}")]
    NotFound(String),

    #[error("Invalid person_id format: {0}")]
    InvalidId(String),

    #[error("Email already exists: {0}")]
    EmailExists(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),
}

impl IntoResponse for PersonError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            PersonError::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Person {} not found", id)
            ),
            PersonError::InvalidId(id) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid person ID: {}", id)
            ),
            PersonError::EmailExists(email) => (
                StatusCode::CONFLICT,
                format!("Email {} already exists", email)
            ),
            PersonError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error occurred".to_string()
            ),
            PersonError::Validation(msg) => (
                StatusCode::BAD_REQUEST,
                msg
            ),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

---

## Logging en Monitoring

### Structured Logging

```rust
use tracing::{info, warn, error, instrument};

#[instrument(skip(self))]
pub async fn list_persons(
    &self,
    params: PersonListParams,
) -> Result<PersonListResponse, PersonError> {
    info!(
        "Listing persons with filters: org_id={:?}, country={:?}",
        params.org_id, params.country
    );

    let start = Instant::now();
    let result = self.repository.list(params).await;
    let duration = start.elapsed();

    match &result {
        Ok((persons, total)) => {
            info!(
                "Successfully retrieved {} persons (total: {}) in {:?}",
                persons.len(), total, duration
            );
        }
        Err(e) => {
            error!("Failed to retrieve persons: {}", e);
        }
    }

    result.map(|(persons, total)| {
        // Transform to response
    })
}
```

### Metrics

- Request count per endpoint
- Request duration (p50, p95, p99)
- Error rate per error type
- Active database connections
- Query duration

---

## Testing Strategie

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gid_confidence_calculation() {
        let person = Person {
            email: "thomas.wagensonner@equans.com".to_string(),
            local_id: Some("thomas.wagensonner@equans.com".to_string()),
            ..Default::default()
        };

        let confidence = calculate_gid_confidence(&person, "thomas.wagensonner");
        assert_eq!(confidence, 80);
    }

    #[test]
    fn test_gid_status_determination() {
        assert_eq!(determine_gid_status(95), GidStatus::Matched);
        assert_eq!(determine_gid_status(65), GidStatus::Pending);
        assert_eq!(determine_gid_status(30), GidStatus::Unmatched);
    }

    #[test]
    fn test_email_validation() {
        assert!(validate_person_email("test@equans.com").is_ok());
        assert!(validate_person_email("invalid").is_err());
        assert!(validate_person_email("test@other.com").is_err());
    }
}
```

### Integration Tests

```rust
#[sqlx::test]
async fn test_list_persons_with_filters(pool: PgPool) {
    let repo = PersonRepository::new(pool);

    // Insert test data
    // ...

    let params = PersonListParams {
        country: Some("Austria".to_string()),
        status: Some("Active".to_string()),
        page: Some(1),
        per_page: Some(25),
        ..Default::default()
    };

    let (persons, total) = repo.list(params).await.unwrap();

    assert!(persons.len() <= 25);
    assert!(persons.iter().all(|p| p.country == Some("Austria".to_string())));
    assert!(persons.iter().all(|p| p.status == "Active"));
}
```

---

## Deployment

### Environment Variables

```bash
DATABASE_URL=postgresql://equans:password@postgres:5432/equans_insights
JWT_SECRET=your-secret-key
RUST_LOG=info,equans_operational_insights_backend=debug
```

### Docker Compose

```yaml
services:
  backend:
    build: ./backend
    environment:
      - DATABASE_URL=postgresql://equans:equans_password@postgres:5432/equans_insights
      - JWT_SECRET=${JWT_SECRET}
    ports:
      - "8080:8080"
    depends_on:
      - postgres

  postgres:
    image: postgres:16
    environment:
      - POSTGRES_USER=equans
      - POSTGRES_PASSWORD=equans_password
      - POSTGRES_DB=equans_insights
    volumes:
      - postgres_data:/var/lib/postgresql/data
```

---

## Gerelateerde Documenten

- Functional Requirement: [FR-005](../Functional-Requirements/FR-005-Person-Management.md)
- Technical Requirement: [TR-006](TR-006-Organization-Management.md)
- Technical Requirement: [TR-007](TR-007-Data-Import.md)
- Business Requirement: [BR-002](../Business-Requirements/BR-002-Person-Organization-Management.md)
