# TR-003: Atlassian API Cache - Technische Specificaties

**Status:** Draft
**Date:** 2026-02-16
**Author(s):** Backend Engineer Agent
**Applies To:** Backend Rust applicatie, PostgreSQL database

---

## Scope

Dit document definieert de technische specificaties voor de Atlassian API cache implementatie, inclusief:

- Architectuur en componenten
- Database schema
- API specificaties
- Security requirements
- Error handling
- Monitoring en logging

---

## Architectuur Overzicht

```
┌──────────────┐     ┌──────────────────┐     ┌─────────────────────┐
│   Frontend   │────▶│   Rust Backend   │────▶│   Atlassian Cloud   │
│   (React)    │     │   (Axum)         │     │   API               │
└──────────────┘     └────────┬─────────┘     └─────────────────────┘
                              │
                              ▼
                     ┌──────────────────┐
                     │   PostgreSQL     │
                     │   (Cache)        │
                     └──────────────────┘
```

### Data Flow

1. Frontend vraagt data op via REST API
2. Backend checkt PostgreSQL cache
3. Als cache geldig: return cached data
4. Als cache verlopen: fetch van Atlassian API, update cache, return data
5. Bij `force_refresh=true`: altijd fetch van Atlassian API

---

## Componenten Structuur

```
backend/
├── src/
│   ├── main.rs
│   ├── config.rs                 # Configuratie laden
│   ├── atlassian/
│   │   ├── mod.rs
│   │   ├── client.rs             # HTTP client voor Atlassian API
│   │   ├── service.rs            # Business logic met caching
│   │   ├── types.rs              # Data types en DTOs
│   │   └── error.rs              # Error handling
│   ├── cache/
│   │   ├── mod.rs
│   │   └── repository.rs         # PostgreSQL cache operaties
│   ├── routes/
│   │   └── atlassian.rs          # API endpoints
│   └── jobs/
│       └── daily_sync.rs         # Background sync job
├── migrations/
│   └── 001_atlassian_cache.sql   # Database schema
└── Cargo.toml
```

---

## Database Schema

### Cache Tabellen

```sql
-- Atlassian gebruikers cache
CREATE TABLE atlassian_users_cache (
    account_id VARCHAR(128) PRIMARY KEY,
    account_type VARCHAR(50) NOT NULL,
    email VARCHAR(255),
    display_name VARCHAR(255) NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true,
    raw_data JSONB NOT NULL,
    cached_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL
);

-- Atlassian groepen cache
CREATE TABLE atlassian_groups_cache (
    group_id VARCHAR(128) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    raw_data JSONB NOT NULL,
    cached_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL
);

-- Groep membership relatie
CREATE TABLE atlassian_group_members_cache (
    group_id VARCHAR(128) REFERENCES atlassian_groups_cache(group_id) ON DELETE CASCADE,
    account_id VARCHAR(128) REFERENCES atlassian_users_cache(account_id) ON DELETE CASCADE,
    cached_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (group_id, account_id)
);

-- Sync status tracking
CREATE TABLE atlassian_sync_status (
    sync_type VARCHAR(50) PRIMARY KEY,
    last_sync_at TIMESTAMPTZ,
    last_success_at TIMESTAMPTZ,
    last_error TEXT,
    items_synced INTEGER DEFAULT 0
);
```

### Indexes

```sql
CREATE INDEX idx_users_cache_expires ON atlassian_users_cache(expires_at);
CREATE INDEX idx_groups_cache_expires ON atlassian_groups_cache(expires_at);
CREATE INDEX idx_users_cache_email ON atlassian_users_cache(email);
CREATE INDEX idx_users_cache_display_name ON atlassian_users_cache(display_name);
```

---

## API Specificaties

### Endpoints

#### GET /api/atlassian/users

Haalt alle Atlassian gebruikers op.

**Query Parameters:**

| Parameter       | Type    | Verplicht | Default | Beschrijving               |
| --------------- | ------- | --------- | ------- | -------------------------- |
| `force_refresh` | boolean | Nee       | `false` | Bypass cache indien `true` |

**Response (200 OK):**

```json
{
  "data": [
    {
      "account_id": "5b10a2844c20165700ede21g",
      "account_type": "atlassian",
      "email": "user@example.com",
      "display_name": "John Doe",
      "active": true
    }
  ],
  "cache": {
    "cached": true,
    "cached_at": "2026-02-15T08:00:00Z",
    "expires_at": "2026-02-16T09:00:00Z"
  }
}
```

**Error Responses:**

| Status | Beschrijving                                           |
| ------ | ------------------------------------------------------ |
| 401    | Atlassian authenticatie mislukt                        |
| 500    | Interne server fout                                    |
| 503    | Atlassian API niet beschikbaar, geen cache beschikbaar |

---

#### GET /api/atlassian/groups

Haalt alle Atlassian groepen op.

**Query Parameters:**

| Parameter       | Type    | Verplicht | Default | Beschrijving               |
| --------------- | ------- | --------- | ------- | -------------------------- |
| `force_refresh` | boolean | Nee       | `false` | Bypass cache indien `true` |

**Response (200 OK):**

```json
{
  "data": [
    {
      "group_id": "3b4c5d6e-7f8g-9h0i-1j2k-3l4m5n6o7p8q",
      "name": "jira-administrators"
    }
  ],
  "cache": {
    "cached": true,
    "cached_at": "2026-02-15T08:00:00Z",
    "expires_at": "2026-02-16T09:00:00Z"
  }
}
```

---

### Response Types (Rust)

```rust
#[derive(Serialize, Deserialize)]
pub struct AtlassianUser {
    pub account_id: String,
    pub account_type: String,
    pub email: Option<String>,
    pub display_name: String,
    pub active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AtlassianGroup {
    pub group_id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct CacheInfo {
    pub cached: bool,
    pub cached_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub cache: CacheInfo,
}
```

---

## Configuratie

### Environment Variables

| Variable              | Type    | Verplicht | Default | Beschrijving                 |
| --------------------- | ------- | --------- | ------- | ---------------------------- |
| `DATABASE_URL`        | String  | Ja        | -       | PostgreSQL connection string |
| `ATLASSIAN_BASE_URL`  | String  | Ja        | -       | Atlassian instance URL       |
| `ATLASSIAN_EMAIL`     | String  | Ja        | -       | Service account email        |
| `ATLASSIAN_API_TOKEN` | String  | Ja        | -       | API token (secret)           |
| `CACHE_TTL_HOURS`     | Integer | Nee       | `25`    | Cache geldigheid in uren     |
| `SYNC_INTERVAL_HOURS` | Integer | Nee       | `24`    | Sync interval in uren        |
| `RUST_LOG`            | String  | Nee       | `info`  | Log level                    |

### Voorbeeld .env

```env
DATABASE_URL=postgres://user:pass@localhost:5432/devex
ATLASSIAN_BASE_URL=https://company.atlassian.net
ATLASSIAN_EMAIL=service-account@company.com
ATLASSIAN_API_TOKEN=ATATT3xFfGF0...
CACHE_TTL_HOURS=25
SYNC_INTERVAL_HOURS=24
RUST_LOG=info
```

---

## Dependencies (Cargo.toml)

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "chrono", "json"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "2"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
base64 = "0.22"
dotenvy = "0.15"
tower-http = { version = "0.5", features = ["cors", "trace"] }
```

---

## Security Requirements

### Credentials Beheer

- [ ] API tokens worden **nooit** in code of git opgeslagen
- [ ] Credentials worden geladen via environment variables
- [ ] In productie: gebruik Azure Key Vault of vergelijkbare secret manager
- [ ] `.env` bestanden staan in `.gitignore`

### Atlassian Authenticatie

Atlassian Cloud API gebruikt Basic Authentication:

```rust
// Authorization header format
let credentials = format!("{}:{}", email, api_token);
let encoded = base64::engine::general_purpose::STANDARD.encode(credentials);
let header_value = format!("Basic {}", encoded);
```

**Service Account Permissions:**

- Minimale permissions: alleen read-access voor users en groups
- Geen admin rechten tenzij noodzakelijk
- Aparte service account, niet persoonlijk account

### Data Protection

- [ ] Email adressen niet loggen in productie
- [ ] `raw_data` JSONB kan persoonlijke data bevatten - behandel als PII
- [ ] Implementeer data retention policy (cleanup na X dagen)
- [ ] GDPR: documenteer welke data wordt opgeslagen

### Transport Security

- [ ] Alleen HTTPS verbindingen naar Atlassian API
- [ ] TLS 1.2+ vereist
- [ ] Certificate validation ingeschakeld (geen `danger_accept_invalid_certs`)

---

## Error Handling

### Error Types

```rust
#[derive(Error, Debug)]
pub enum AtlassianError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    Parse(reqwest::Error),

    #[error("Unauthorized - check API credentials")]
    Unauthorized,

    #[error("Forbidden - insufficient permissions")]
    Forbidden,

    #[error("Rate limited - too many requests")]
    RateLimited,

    #[error("API error: {status} - {body}")]
    ApiError { status: u16, body: String },
}

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Atlassian API error: {0}")]
    Atlassian(#[from] AtlassianError),

    #[error("Cache error: {0}")]
    Cache(#[from] CacheError),
}
```

### HTTP Response Mapping

```rust
impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ServiceError::Atlassian(AtlassianError::Unauthorized) => {
                (StatusCode::UNAUTHORIZED, "Atlassian authentication failed")
            }
            ServiceError::Atlassian(AtlassianError::RateLimited) => {
                (StatusCode::TOO_MANY_REQUESTS, "Rate limited by Atlassian")
            }
            _ => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        // Log actual error server-side
        tracing::error!("Request failed: {}", self);

        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

### Fallback Strategie

```
API Request
    │
    ▼
Cache geldig? ──Ja──▶ Return cached data
    │
    Nee
    │
    ▼
Atlassian API ──Succes──▶ Update cache, return fresh data
    │
    Fout
    │
    ▼
Verlopen cache beschikbaar? ──Ja──▶ Return stale data + warning header
    │
    Nee
    │
    ▼
Return error (503 Service Unavailable)
```

---

## Background Jobs

### Daily Sync Job

```rust
pub async fn start_daily_sync_job(service: Arc<AtlassianService>) {
    // Initial delay to stagger startup
    tokio::time::sleep(Duration::from_secs(60)).await;

    let interval = Duration::from_secs(24 * 60 * 60); // 24 hours
    let mut ticker = tokio::time::interval(interval);

    loop {
        ticker.tick().await;

        tracing::info!("Starting scheduled Atlassian sync");

        match sync_all_data(&service).await {
            Ok(stats) => {
                tracing::info!(
                    users = stats.users_count,
                    groups = stats.groups_count,
                    "Daily sync completed successfully"
                );
            }
            Err(e) => {
                tracing::error!(error = %e, "Daily sync failed");
                // Don't panic - old cache still available
            }
        }
    }
}
```

### Startup Sync

Bij applicatie startup:

1. Check of cache tabellen leeg zijn
2. Indien leeg: voer initiële sync uit (blocking)
3. Start background sync job (non-blocking)

```rust
pub async fn initialize_cache(service: &AtlassianService) -> Result<(), ServiceError> {
    let cache_empty = service.is_cache_empty().await?;

    if cache_empty {
        tracing::info!("Cache is empty, performing initial sync");
        service.refresh_all().await?;
        tracing::info!("Initial sync completed");
    }

    Ok(())
}
```

---

## Performance Requirements

| Metric                | Requirement | Beschrijving                      |
| --------------------- | ----------- | --------------------------------- |
| Cache read latency    | P95 < 50ms  | PostgreSQL query voor cached data |
| API response time     | P95 < 100ms | Endpoint response met cached data |
| Atlassian API timeout | 30 seconden | Maximum wachttijd per request     |
| Daily sync duration   | < 5 minuten | Totale sync tijd voor alle data   |

### Optimalisaties

- [ ] Database connection pooling (SQLx built-in)
- [ ] Prepared statements voor cache queries
- [ ] Batch inserts bij cache updates
- [ ] Indexes op veelgebruikte kolommen

---

## Monitoring & Logging

### Structured Logging

```rust
// Voorbeeld log statements
tracing::info!(
    endpoint = "users",
    cache_hit = true,
    items = users.len(),
    "Returning cached Atlassian users"
);

tracing::warn!(
    endpoint = "users",
    cache_age_hours = age.num_hours(),
    "Using stale cache due to API error"
);

tracing::error!(
    endpoint = "users",
    error = %e,
    "Failed to fetch from Atlassian API"
);
```

### Metrics (voor toekomstige implementatie)

| Metric                           | Type      | Beschrijving             |
| -------------------------------- | --------- | ------------------------ |
| `atlassian_cache_hits_total`     | Counter   | Aantal cache hits        |
| `atlassian_cache_misses_total`   | Counter   | Aantal cache misses      |
| `atlassian_api_requests_total`   | Counter   | API calls naar Atlassian |
| `atlassian_api_duration_seconds` | Histogram | API response tijden      |
| `atlassian_sync_last_success`    | Gauge     | Timestamp laatste sync   |
| `atlassian_cached_users_count`   | Gauge     | Aantal gecachede users   |

### Health Check Endpoint

```rust
// GET /health
{
  "status": "healthy",
  "database": "connected",
  "atlassian_cache": {
    "users_count": 150,
    "groups_count": 25,
    "last_sync": "2026-02-16T08:00:00Z",
    "cache_valid": true
  }
}
```

---

## Testing

### Unit Tests

- [ ] Cache repository: CRUD operaties
- [ ] Cache expiration logic
- [ ] Error type conversions
- [ ] Config parsing

### Integration Tests

- [ ] API endpoints met mock Atlassian responses
- [ ] `force_refresh` functionaliteit
- [ ] Fallback bij API failures
- [ ] Database migrations

### Test Voorbeeld

```rust
#[tokio::test]
async fn test_force_refresh_bypasses_cache() {
    let app = TestApp::new().await;

    // First request - populates cache
    let res = app.get("/api/atlassian/users").await;
    assert_eq!(res.status(), 200);
    let body: ApiResponse<Vec<AtlassianUser>> = res.json().await;
    assert!(!body.cache.cached); // Fresh data

    // Second request - uses cache
    let res = app.get("/api/atlassian/users").await;
    let body: ApiResponse<Vec<AtlassianUser>> = res.json().await;
    assert!(body.cache.cached); // From cache

    // Force refresh - bypasses cache
    let res = app.get("/api/atlassian/users?force_refresh=true").await;
    let body: ApiResponse<Vec<AtlassianUser>> = res.json().await;
    assert!(!body.cache.cached); // Fresh data again
}
```

---

## Deployment

### Database Migraties

```bash
# Installeer sqlx-cli
cargo install sqlx-cli

# Voer migraties uit
sqlx migrate run

# Of via applicatie startup (embedded migrations)
sqlx::migrate!("./migrations").run(&pool).await?;
```

### Docker Compose (development)

```yaml
services:
  backend:
    build: ./backend
    environment:
      - DATABASE_URL=postgres://devex:devex@db:5432/devex
      - ATLASSIAN_BASE_URL=${ATLASSIAN_BASE_URL}
      - ATLASSIAN_EMAIL=${ATLASSIAN_EMAIL}
      - ATLASSIAN_API_TOKEN=${ATLASSIAN_API_TOKEN}
    depends_on:
      - db

  db:
    image: postgres:16
    environment:
      - POSTGRES_USER=devex
      - POSTGRES_PASSWORD=devex
      - POSTGRES_DB=devex
    volumes:
      - postgres_data:/var/lib/postgresql/data
```

---

## Referenties

- [Atlassian Cloud REST API Documentation](https://developer.atlassian.com/cloud/jira/platform/rest/v3/)
- [Axum Framework Documentation](https://docs.rs/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [FR-003: Atlassian Cache Functionele Requirements](../Functional-Requirements/FR-003-Atlassian-Cache.md)
