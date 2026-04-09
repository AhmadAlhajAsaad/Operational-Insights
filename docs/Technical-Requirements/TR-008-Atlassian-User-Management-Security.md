# TR-008: Atlassian User Management - Technical Security Requirements

**Status:** Draft
**Datum:** 2026-02-23
**Auteur(s):** Backend Engineer
**Implementeert:** [FR-008-Atlassian-User-Management](../Functional-Requirements/FR-008-Atlassian-User-Management.md)
**Applies To:** Backend Rust applicatie, React frontend, Atlassian Admin API v2

---

## Samenvatting

Dit document definieert de technische requirements en security standaarden voor het veilig integreren met de Atlassian Admin API voor gebruikersbeheer. De focus ligt op het beschermen van API credentials, secure data transmission, en het implementeren van best practices voor REST API integratie.

---

## Architectuur Overzicht

```
┌─────────────────────┐       JWT Auth         ┌──────────────────────┐
│   React Frontend    │◄──────────────────────►│   Rust Backend       │
│   (Public)          │    HTTPS Only          │   (Internal API)     │
└─────────────────────┘                        └──────────────────────┘
                                                         │
                                                         │ API Key
                                                         │ (Secret)
                                                         │
                                                         ▼
                                                ┌──────────────────────┐
                                                │  Atlassian Admin API │
                                                │  (External)          │
                                                └──────────────────────┘
```

### Veiligheidslagen

1. **Frontend ↔ Backend:** JWT authentication, HTTPS
2. **Backend ↔ Atlassian API:** API Key authentication, HTTPS
3. **Credentials Storage:** Environment variables, never in code
4. **Data Transmission:** TLS 1.2+ encryption

---

## Security Requirements

### 1. API Credential Management

#### 1.1 API Key Storage (CRITICAL)

**MUST:**
- API keys worden ALLEEN opgeslagen in environment variables
- NOOIT in code committen (gebruik .gitignore)
- Gebruik Docker secrets of GitHub Secrets in production
- Roteer API keys minimaal elke 90 dagen

**Implementation:**

```bash
# .env file (NEVER commit)
ATLASSIAN_API_KEY=your_secret_api_key_here
ATLASSIAN_ORG_ID=your_org_id_here
ATLASSIAN_API_BASE_URL=https://api.atlassian.com
```

```rust
// Backend configuration
use std::env;

pub struct AtlassianConfig {
    pub api_key: String,
    pub org_id: String,
    pub base_url: String,
}

impl AtlassianConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            api_key: env::var("ATLASSIAN_API_KEY")
                .map_err(|_| ConfigError::MissingApiKey)?,
            org_id: env::var("ATLASSIAN_ORG_ID")
                .map_err(|_| ConfigError::MissingOrgId)?,
            base_url: env::var("ATLASSIAN_API_BASE_URL")
                .unwrap_or_else(|_| "https://api.atlassian.com".to_string()),
        })
    }
}
```

#### 1.2 API Key Protection

**MUST:**
- API keys worden NOOIT naar frontend gestuurd
- API keys worden NOOIT gelogd (mask in logs)
- Frontend heeft geen directe toegang tot Atlassian API
- Alle Atlassian API calls gaan via backend proxy

**Implementation:**

```rust
// Request logging met masked API key
impl fmt::Display for AtlassianRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let masked_key = format!("{}***{}",
            &self.api_key[..4],
            &self.api_key[self.api_key.len()-4..]
        );
        write!(f, "Atlassian API request with key: {}", masked_key)
    }
}
```

---
### 2. Authentication & Authorization

#### 2.1 Frontend Authentication

**MUST:**
- Frontend gebruikt JWT tokens voor authenticatie
- Tokens worden opgeslagen in httpOnly cookies (niet localStorage)
- Token expiration: max 24 uur
- Refresh tokens voor automatische renewal

**Implementation (Frontend):**

```typescript
// HTTP client met automatic token refresh
import axios from 'axios';

const apiClient = axios.create({
  baseURL: '/api',
  withCredentials: true, // Send cookies
});

// Interceptor voor automatische token refresh
apiClient.interceptors.response.use(
  response => response,
  async error => {
    if (error.response?.status === 401) {
      // Token expired, try refresh
      await refreshToken();
      return apiClient.request(error.config);
    }
    return Promise.reject(error);
  }
);
```

#### 2.2 Backend Authorization

**MUST:**
- Valideer JWT token bij elke request
- Check user role/permissions voor user management acties
- Admin role required voor: invite, suspend, delete users
- Regular users kunnen alleen read-only access hebben

**Implementation (Backend):**

```rust
use axum::{
    extract::{Extension, State},
    middleware,
    routing::{get, post, delete},
    Router,
};

// Route protection middleware
async fn require_admin(
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<(), StatusCode> {
    if !user.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(())
}

// Protected routes
pub fn atlassian_routes() -> Router {
    Router::new()
        // Read-only routes (all authenticated users)
        .route("/users", get(list_users))
        .route("/users/:account_id", get(get_user_details))
        // Admin-only routes
        .route("/users/invite", post(invite_user))
        .route("/users/:account_id/suspend", post(suspend_user))
        .route("/users/:account_id", delete(delete_user))
        .layer(middleware::from_fn(require_admin))
}
```

---

### 3. Secure HTTP Communication

#### 3.1 TLS/HTTPS Requirements

**MUST:**
- All communication uses HTTPS (TLS 1.2 or higher)
- Validate SSL certificates (no self-signed in production)
- Use secure cipher suites only

**Implementation (Backend HTTP Client):**

```rust
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

pub fn create_atlassian_client() -> Result<Client, reqwest::Error> {
    ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .min_tls_version(reqwest::tls::Version::TLS_1_2) // Enforce TLS 1.2+
        .https_only(true) // Reject all HTTP connections
        .use_rustls_tls() // Use rustls for TLS
        .build()
}
```

#### 3.2 Request Headers

**MUST:**
- Set proper Content-Type headers
- Include Authentication header with API key
- Set User-Agent for tracking
- Include request ID for correlation

**Implementation:**

```rust
impl AtlassianClient {
    async fn make_request(
        &self,
        method: Method,
        endpoint: &str,
    ) -> Result<Response, AtlassianError> {
        let request_id = Uuid::new_v4();

        let response = self.client
            .request(method, format!("{}{}", self.base_url, endpoint))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("User-Agent", "Equans-Operational-Insights/1.0")
            .header("X-Request-ID", request_id.to_string())
            .send()
            .await?;

        tracing::info!(
            request_id = %request_id,
            status = %response.status(),
            "Atlassian API request completed"
        );

        Ok(response)
    }
}
```

---

### 4. Rate Limiting & Retry Logic

#### 4.1 Respect Atlassian Rate Limits

**Atlassian API Rate Limits:**
- Standard: 10,000 requests per hour per organization
- Burst: 100 requests per minute
- Rate limit headers: X-RateLimit-Limit, X-RateLimit-Remaining

**MUST:**
- Parse rate limit headers from responses
- Implement exponential backoff bij 429 (Too Many Requests)
- Cache responses waar mogelijk
- Batch requests waar de API dit ondersteunt

**Implementation:**

```rust
use std::time::Duration;
use tokio::time::sleep;

impl AtlassianClient {
    async fn request_with_retry<T>(
        &self,
        request_fn: impl Fn() -> BoxFuture<'static, Result<T, AtlassianError>>,
        max_retries: u32,
    ) -> Result<T, AtlassianError> {
        let mut retries = 0;

        loop {
            match request_fn().await {
                Ok(result) => return Ok(result),
                Err(e) if e.is_rate_limit() && retries < max_retries => {
                    let backoff = Duration::from_secs(2u64.pow(retries));
                    tracing::warn!(
                        "Rate limit hit, retrying in {:?} (retry {}/{})",
                        backoff, retries + 1, max_retries
                    );
                    sleep(backoff).await;
                    retries += 1;
                }
                Err(e) => return Err(e),
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AtlassianError {
    #[error("Rate limit exceeded")]
    RateLimitExceeded {
        retry_after: Option<u64>,
    },
    // ... other errors
}

impl AtlassianError {
    pub fn is_rate_limit(&self) -> bool {
        matches!(self, AtlassianError::RateLimitExceeded { .. })
    }
}
```

#### 4.2 Circuit Breaker Pattern

**SHOULD:**
- Implement circuit breaker voor Atlassian API calls
- Open circuit na 5 consecutive failures
- Half-open state na 60 seconden
- Fallback naar cached data indien beschikbaar

**Implementation:**

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct CircuitBreaker {
    state: Arc<RwLock<CircuitState>>,
    failure_threshold: u32,
    timeout: Duration,
}

enum CircuitState {
    Closed { failures: u32 },
    Open { opened_at: Instant },
    HalfOpen,
}

impl CircuitBreaker {
    pub async fn call<T, F, Fut>(
        &self,
        f: F,
    ) -> Result<T, CircuitBreakerError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, AtlassianError>>,
    {
        // Check if circuit is open
        let can_proceed = {
            let state = self.state.read().await;
            match *state {
                CircuitState::Open { opened_at } => {
                    if opened_at.elapsed() > self.timeout {
                        // Transition to half-open
                        drop(state);
                        let mut state = self.state.write().await;
                        *state = CircuitState::HalfOpen;
                        true
                    } else {
                        false
                    }
                }
                _ => true,
            }
        };

        if !can_proceed {
            return Err(CircuitBreakerError::CircuitOpen);
        }

        // Execute function
        match f().await {
            Ok(result) => {
                // Success - reset failures
                let mut state = self.state.write().await;
                *state = CircuitState::Closed { failures: 0 };
                Ok(result)
            }
            Err(e) => {
                // Failure - increment counter
                let mut state = self.state.write().await;
                match *state {
                    CircuitState::Closed { failures } => {
                        let new_failures = failures + 1;
                        if new_failures >= self.failure_threshold {
                            *state = CircuitState::Open {
                                opened_at: Instant::now(),
                            };
                            tracing::error!("Circuit breaker opened after {} failures", new_failures);
                        } else {
                            *state = CircuitState::Closed { failures: new_failures };
                        }
                    }
                    CircuitState::HalfOpen => {
                        *state = CircuitState::Open {
                            opened_at: Instant::now(),
                        };
                    }
                    _ => {}
                }
                Err(CircuitBreakerError::RequestFailed(e))
            }
        }
    }
}
```

---

### 5. Data Validation & Sanitization

#### 5.1 Input Validation

**MUST:**
- Valideer alle user input voordat het naar Atlassian API gaat
- Email format validation
- Account ID format validation
- Sanitize strings om injection attacks te voorkomen

**Implementation:**

```rust
use regex::Regex;
use validator::Validate;

#[derive(Debug, Validate)]
pub struct InviteUserRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1, max = 50))]
    pub products: Vec<String>,

    #[validate(custom = "validate_atlassian_account_id")]
    pub invited_by: Option<String>,
}

fn validate_atlassian_account_id(account_id: &str) -> Result<(), ValidationError> {
    let regex = Regex::new(r"^[a-f0-9]{24}$").unwrap();
    if regex.is_match(account_id) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_account_id"))
    }
}

// Usage in endpoint
async fn invite_user(
    State(state): State<AppState>,
    Json(payload): Json<InviteUserRequest>,
) -> Result<Json<InviteUserResponse>, ApiError> {
    // Validate input
    payload.validate()
        .map_err(|e| ApiError::ValidationError(e))?;

    // Proceed with API call
    // ...
}
```

#### 5.2 Output Sanitization

**MUST:**
- Sanitize data voordat het naar frontend wordt gestuurd
- Remove/mask sensitive fields indien nodig
- Escape HTML voor XSS prevention
- Consistent data formatting

**Implementation:**

```rust
use serde::{Deserialize, Serialize};

// Internal model (from Atlassian API)
#[derive(Debug, Deserialize)]
pub struct AtlassianUser {
    pub account_id: String,
    pub email: String,
    pub name: String,
    pub account_type: String,
    pub account_status: String,
    // ... more fields
}

// External DTO (to frontend)
#[derive(Debug, Serialize)]
pub struct UserDto {
    pub account_id: String,
    pub email: String, // May be masked for non-admin
    pub name: String,
    pub account_type: String,
    pub account_status: UserStatus,
    // Sensitive fields removed
}

impl From<AtlassianUser> for UserDto {
    fn from(user: AtlassianUser) -> Self {
        Self {
            account_id: user.account_id,
            email: user.email,
            name: sanitize_html(&user.name), // XSS prevention
            account_type: user.account_type,
            account_status: UserStatus::from_str(&user.account_status),
        }
    }
}

fn sanitize_html(input: &str) -> String {
    input
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}
```

---

### 6. Error Handling & Security

#### 6.1 Secure Error Messages

**MUST:**
- NOOIT internal details exposen in error messages naar frontend
- Log detailed errors server-side only
- Generic error messages naar gebruiker
- Correlation IDs voor debugging

**Implementation:**

```rust
#[derive(Debug, thiserror::Error)]
pub enum AtlassianError {
    #[error("Network error")]
    Network(#[from] reqwest::Error),

    #[error("Rate limit exceeded")]
    RateLimitExceeded { retry_after: Option<u64> },

    #[error("User not found")]
    UserNotFound { account_id: String },

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("Invalid request")]
    InvalidRequest { details: String },
}

impl IntoResponse for AtlassianError {
    fn into_response(self) -> Response {
        let request_id = Uuid::new_v4();

        // Log detailed error server-side
        tracing::error!(
            request_id = %request_id,
            error = ?self,
            "Atlassian API error"
        );

        // Return generic error to client
        let (status, message) = match self {
            AtlassianError::UserNotFound { .. } => (
                StatusCode::NOT_FOUND,
                "Gebruiker niet gevonden".to_string(),
            ),
            AtlassianError::InsufficientPermissions => (
                StatusCode::FORBIDDEN,
                "Onvoldoende rechten".to_string(),
            ),
            AtlassianError::RateLimitExceeded { retry_after } => (
                StatusCode::TOO_MANY_REQUESTS,
                format!("Te veel requests, probeer over {} seconden opnieuw",
                    retry_after.unwrap_or(60)),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Er is een fout opgetreden".to_string(),
            ),
        };

        (
            status,
            Json(json!({
                "error": message,
                "request_id": request_id,
            }))
        ).into_response()
    }
}
```

#### 6.2 Logging & Monitoring

**MUST:**
- Log all user management acties (audit trail)
- Include: timestamp, user, action, target, result
- Use structured logging (JSON format)
- Never log sensitive data (API keys, passwords)

**Implementation:**

```rust
use tracing::{info, warn, error};
use serde_json::json;

pub async fn suspend_user(
    Extension(current_user): Extension<AuthenticatedUser>,
    Path(account_id): Path<String>,
) -> Result<Json<SuspendResponse>, ApiError> {
    // Log action
    info!(
        user = current_user.email,
        action = "suspend_user",
        target_account_id = account_id,
        "User management action initiated"
    );

    match atlassian_client.suspend_user(&account_id).await {
        Ok(response) => {
            info!(
                user = current_user.email,
                action = "suspend_user",
                target_account_id = account_id,
                result = "success",
                "User suspended successfully"
            );
            Ok(Json(response))
        }
        Err(e) => {
            error!(
                user = current_user.email,
                action = "suspend_user",
                target_account_id = account_id,
                result = "failure",
                error = ?e,
                "Failed to suspend user"
            );
            Err(ApiError::from(e))
        }
    }
}
```

---

### 7. Caching Strategy

#### 7.1 Response Caching

**SHOULD:**
- Cache user list responses voor 5 minuten
- Cache user detail responses voor 2 minuten
- Invalidate cache bij mutations (invite, suspend, delete)
- Use in-memory cache

**Implementation:**

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::{Instant, Duration};

pub struct CacheEntry<T> {
    data: T,
    expires_at: Instant,
}

pub struct AtlassianCache {
    users: Arc<RwLock<HashMap<String, CacheEntry<UserDto>>>>,
    user_list: Arc<RwLock<Option<CacheEntry<Vec<UserDto>>>>>,
}

impl AtlassianCache {
    pub async fn get_user(&self, account_id: &str) -> Option<UserDto> {
        let cache = self.users.read().await;
        cache.get(account_id).and_then(|entry| {
            if entry.expires_at > Instant::now() {
                Some(entry.data.clone())
            } else {
                None
            }
        })
    }

    pub async fn set_user(&self, account_id: String, user: UserDto, ttl: Duration) {
        let mut cache = self.users.write().await;
        cache.insert(account_id, CacheEntry {
            data: user,
            expires_at: Instant::now() + ttl,
        });
    }

    pub async fn invalidate_user(&self, account_id: &str) {
        let mut cache = self.users.write().await;
        cache.remove(account_id);

        // Also invalidate user list
        let mut list_cache = self.user_list.write().await;
        *list_cache = None;
    }
}
```

---

### 8. Pagination & Performance

#### 8.1 Cursor-Based Pagination

**MUST:**
- Use cursor-based pagination voor consistente resultaten
- Max 100 results per page (Atlassian limit)
- Include next_cursor in response
- Cache pagination results

**Implementation:**

```rust
#[derive(Debug, Deserialize)]
pub struct ListUsersParams {
    pub cursor: Option<String>,
    pub limit: Option<u32>, // Max 100
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

impl AtlassianClient {
    pub async fn list_users(
        &self,
        params: ListUsersParams,
    ) -> Result<PaginatedResponse<UserDto>, AtlassianError> {
        let limit = params.limit.unwrap_or(50).min(100); // Enforce max

        let mut url = format!(
            "{}/admin/v2/orgs/{}/users?limit={}",
            self.base_url, self.org_id, limit
        );

        if let Some(cursor) = params.cursor {
            url.push_str(&format!("&cursor={}", cursor));
        }

        let response = self.make_request(Method::GET, &url).await?;

        let atlassian_response: AtlassianUsersResponse = response.json().await?;

        Ok(PaginatedResponse {
            data: atlassian_response.data.into_iter().map(UserDto::from).collect(),
            next_cursor: atlassian_response.links.next.map(|link| {
                // Extract cursor from next link
                extract_cursor_from_link(&link)
            }),
            has_more: atlassian_response.links.next.is_some(),
        })
    }
}
```

---

### 9. GDPR & Privacy Compliance

#### 9.1 Data Minimization

**MUST:**
- Only collect and store necessary user data
- Email masking voor non-admin users
- Data retention policy (auto-delete old data)
- User consent voor data processing

**Implementation:**

```rust
pub fn mask_email(email: &str, is_admin: bool) -> String {
    if is_admin {
        return email.to_string();
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return "***@***.***".to_string();
    }

    let username = parts[0];
    let domain = parts[1];

    if username.len() <= 3 {
        format!("{}@{}", "*".repeat(username.len()), domain)
    } else {
        format!("
{}***@{}", &username[..2], domain)
    }
}

// Usage
#[derive(Debug, Serialize)]
pub struct UserListDto {
    pub account_id: String,
    pub email: String, // Masked for non-admin
    pub name: String,
    pub status: UserStatus,
}

impl UserListDto {
    pub fn from_user(user: AtlassianUser, is_admin: bool) -> Self {
        Self {
            account_id: user.account_id,
            email: mask_email(&user.email, is_admin),
            name: user.name,
            status: UserStatus::from_str(&user.account_status),
        }
    }
}
```

#### 9.2 Audit Logging

**MUST:**
- Log all data access en mutations
- Retention: min 1 jaar
- Include: who, what, when, where, why
- Tamper-proof logging (append-only)

---

### 10. Testing Requirements

#### 10.1 Security Testing

**MUST:**
- [ ] API key niet exposed in frontend
- [ ] JWT token validation werkt
- [ ] Rate limiting correct geïmplementeerd
- [ ] Error messages geen sensitive data bevatten
- [ ] Input validation voorkomt injection

#### 10.2 Integration Testing

**SHOULD:**
- [ ] Mock Atlassian API voor unit tests
- [ ] E2E tests met test organization
- [ ] Rate limit scenarios testen
- [ ] Circuit breaker scenarios testen
- [ ] Error handling scenarios testen

---

## Deployment Checklist

### Pre-Production

- [ ] API keys in environment variables (niet in code)
- [ ] TLS certificates correctly configured
- [ ] Rate limiting geïmplementeerd
- [ ] Circuit breaker geïmplementeerd
- [ ] Caching strategy geïmplementeerd
- [ ] Logging configured (structured JSON)
- [ ] Error handling test completed

### Production

- [ ] HTTPS enforced (HSTS enabled)
- [ ] API key rotation policy established
- [ ] Monitoring dashboards configured
- [ ] Alert rules configured
- [ ] Backup strategy voor cached data
- [ ] GDPR compliance verified
- [ ] Security audit completed

---

## Monitoring & Alerting

### Metrics to Track

1. **Request Metrics**
   - Request rate (requests/minute)
   - Response time (p50, p95, p99)
   - Error rate (% of failed requests)
   - Rate limit hit rate

2. **Security Metrics**
   - Failed authentication attempts
   - Unauthorized access attempts
   - API key rotation events
   - Suspicious activity patterns

3. **Performance Metrics**
   - Cache hit/miss ratio
   - Database query performance
   - External API latency
   - Circuit breaker state changes

### Alert Thresholds

- Error rate > 5%: WARNING
- Error rate > 10%: CRITICAL
- Response time p95 > 500ms: WARNING
- Rate limit hit > 10/hour: WARNING
- Circuit breaker open: CRITICAL
- Failed auth > 100/hour: CRITICAL

---

## Security Incident Response

### Procedure

1. **Detection:** Monitoring alerts triggered
2. **Assessment:** Determine severity and scope
3. **Containment:** Rotate API keys immediately
4. **Eradication:** Fix vulnerability
5. **Recovery:** Restore normal operations
6. **Lessons Learned:** Post-mortem document

### Contact Information

- Security Team: security@equans.com
- On-call Engineer: +31 XX XXX XXXX
- Atlassian Support: https://support.atlassian.com

---

## Gerelateerde Documenten

- **Functional:** [FR-008: Atlassian User Management](../Functional-Requirements/FR-008-Atlassian-User-Management.md)
- **API Docs:** [Atlassian User Management API](../api/atlassian/user-management-api.md)
- **Security:** [TR-001: Performance & Security Standards](TR-001-Performance-Security-Standards.md)
- **Auth:** [TR-004: API Authentication](TR-004-API-Authentication.md)

---

## Revision History

| Versie | Datum | Auteur | Wijzigingen |
|--------|-------|--------|-------------|
| 1.0 | 2026-02-23 | Backend Engineer | Initial draft met technical security requirements |
