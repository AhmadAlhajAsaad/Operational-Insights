# TR-004: API Authenticatie en Autorisatie - Technische Specificaties

**Status:** Draft
**Date:** 2026-02-16
**Author(s):** Backend Engineer Agent
**Applies To:** Backend Rust applicatie (Axum), Frontend React applicatie
**Related ADR:** [ADR-004-API-Authentication](../ADRs/ADR-004-api-authentication.md)

---

## Scope

Dit document definieert de technische specificaties voor API beveiliging, inclusief:

- JWT token validatie in Rust backend
- MSAL integratie in React frontend
- Rol-gebaseerde autorisatie (RBAC)
- Security best practices voor Rust en React

---

## Architectuur Overzicht

```
┌──────────────┐         ┌──────────────┐         ┌──────────────────┐
│   Browser    │◀───────▶│   Azure AD   │◀───────▶│   JWKS Endpoint  │
│   (React)    │  OAuth  │   (Entra ID) │  Keys   │   (Public Keys)  │
└──────┬───────┘         └──────────────┘         └──────────────────┘
       │                                                    │
       │ Bearer Token                                       │
       ▼                                                    ▼
┌──────────────────┐                              ┌──────────────────┐
│   Rust Backend   │──────────────────────────────│   JWT Validation │
│   (Axum)         │      Verify Signature        │   Middleware     │
└──────────────────┘                              └──────────────────┘
```

### Authentication Flow

```
1. User ──────▶ React App ──────▶ Azure AD Login
                                        │
2.                              ◀────── JWT Token (access_token)
                                        │
3. React App ──────▶ API Request ──────▶ Rust Backend
                     + Bearer Token            │
                                               ▼
4.                              JWT Validation (signature, exp, aud)
                                               │
5.                              ◀────── Response (200/401/403)
```

---

## Backend Implementatie (Rust/Axum)

### Componenten Structuur

```
backend/
├── src/
│   ├── auth/
│   │   ├── mod.rs
│   │   ├── jwt.rs           # JWT validatie logic
│   │   ├── claims.rs        # Token claims types
│   │   ├── middleware.rs    # Axum auth middleware
│   │   ├── roles.rs         # RBAC logic
│   │   └── error.rs         # Auth error types
│   ├── routes/
│   │   └── ...              # Protected routes
│   └── main.rs
└── Cargo.toml
```

### Dependencies (Cargo.toml)

```toml
[dependencies]
# JWT validation
jsonwebtoken = "9"

# HTTP client for JWKS
reqwest = { version = "0.12", features = ["json"] }

# Async runtime
tokio = { version = "1", features = ["full"] }

# Web framework
axum = "0.7"
axum-extra = { version = "0.9", features = ["typed-header"] }
tower-http = { version = "0.5", features = ["cors", "trace"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Error handling
thiserror = "2"

# Caching for JWKS
moka = { version = "0.12", features = ["future"] }

# Time handling
chrono = { version = "0.4", features = ["serde"] }
```

### JWT Claims Type

```rust
// src/auth/claims.rs
use serde::{Deserialize, Serialize};

/// Azure AD JWT Claims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AzureAdClaims {
    /// Subject (user ID)
    pub sub: String,

    /// User principal name (email)
    pub upn: Option<String>,

    /// Preferred username
    pub preferred_username: Option<String>,

    /// Display name
    pub name: Option<String>,

    /// Email address
    pub email: Option<String>,

    /// Azure AD Object ID
    pub oid: String,

    /// Tenant ID
    pub tid: String,

    /// Audience (must match our app)
    pub aud: String,

    /// Issuer
    pub iss: String,

    /// Issued at (Unix timestamp)
    pub iat: i64,

    /// Not before (Unix timestamp)
    pub nbf: i64,

    /// Expiration (Unix timestamp)
    pub exp: i64,

    /// Azure AD groups (for RBAC)
    #[serde(default)]
    pub groups: Vec<String>,

    /// Roles assigned in app registration
    #[serde(default)]
    pub roles: Vec<String>,
}

impl AzureAdClaims {
    /// Get user identifier for logging
    pub fn user_id(&self) -> &str {
        self.upn.as_deref()
            .or(self.preferred_username.as_deref())
            .or(self.email.as_deref())
            .unwrap_or(&self.sub)
    }

    /// Check if user has a specific role
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    /// Check if user is member of a specific group
    pub fn in_group(&self, group_id: &str) -> bool {
        self.groups.iter().any(|g| g == group_id)
    }
}
```

### JWT Validator

```rust
// src/auth/jwt.rs
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;

use crate::auth::claims::AzureAdClaims;
use crate::auth::error::AuthError;

/// JWKS (JSON Web Key Set) response from Azure AD
#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize)]
struct Jwk {
    kid: String,
    kty: String,
    n: String,
    e: String,
    #[serde(default)]
    x5c: Vec<String>,
}

pub struct JwtValidator {
    client: Client,
    jwks_uri: String,
    tenant_id: String,
    client_id: String,
    audience: String,
    // Cache JWKS keys for 1 hour
    key_cache: Cache<String, DecodingKey>,
}

impl JwtValidator {
    pub fn new(config: &AuthConfig) -> Self {
        let jwks_uri = format!(
            "https://login.microsoftonline.com/{}/discovery/v2.0/keys",
            config.tenant_id
        );

        Self {
            client: Client::new(),
            jwks_uri,
            tenant_id: config.tenant_id.clone(),
            client_id: config.client_id.clone(),
            audience: config.audience.clone(),
            key_cache: Cache::builder()
                .time_to_live(Duration::from_secs(3600))
                .max_capacity(10)
                .build(),
        }
    }

    /// Validate JWT token and extract claims
    pub async fn validate(&self, token: &str) -> Result<AzureAdClaims, AuthError> {
        // Decode header to get key ID (kid)
        let header = decode_header(token)
            .map_err(|e| AuthError::InvalidToken(e.to_string()))?;

        let kid = header.kid
            .ok_or_else(|| AuthError::InvalidToken("Missing kid in header".into()))?;

        // Get decoding key (from cache or fetch)
        let decoding_key = self.get_decoding_key(&kid).await?;

        // Configure validation
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&self.audience]);
        validation.set_issuer(&[
            format!("https://login.microsoftonline.com/{}/v2.0", self.tenant_id),
            format!("https://sts.windows.net/{}/", self.tenant_id),
        ]);
        validation.validate_exp = true;
        validation.validate_nbf = true;

        // Decode and validate token
        let token_data = decode::<AzureAdClaims>(token, &decoding_key, &validation)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    AuthError::TokenExpired
                }
                jsonwebtoken::errors::ErrorKind::InvalidAudience => {
                    AuthError::InvalidAudience
                }
                jsonwebtoken::errors::ErrorKind::InvalidIssuer => {
                    AuthError::InvalidIssuer
                }
                _ => AuthError::InvalidToken(e.to_string()),
            })?;

        Ok(token_data.claims)
    }

    async fn get_decoding_key(&self, kid: &str) -> Result<DecodingKey, AuthError> {
        // Check cache first
        if let Some(key) = self.key_cache.get(kid).await {
            return Ok(key);
        }

        // Fetch JWKS from Azure AD
        let jwks: JwksResponse = self.client
            .get(&self.jwks_uri)
            .send()
            .await
            .map_err(|e| AuthError::JwksError(e.to_string()))?
            .json()
            .await
            .map_err(|e| AuthError::JwksError(e.to_string()))?;

        // Find matching key
        let jwk = jwks.keys.iter()
            .find(|k| k.kid == kid)
            .ok_or_else(|| AuthError::KeyNotFound(kid.to_string()))?;

        // Create decoding key from RSA components
        let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
            .map_err(|e| AuthError::InvalidKey(e.to_string()))?;

        // Cache the key
        self.key_cache.insert(kid.to_string(), decoding_key.clone()).await;

        Ok(decoding_key)
    }
}
```

### Auth Middleware

```rust
// src/auth/middleware.rs
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;

use crate::auth::{claims::AzureAdClaims, jwt::JwtValidator, error::AuthError};

/// Extension type for accessing authenticated user in handlers
#[derive(Clone)]
pub struct AuthenticatedUser(pub AzureAdClaims);

/// Authentication middleware - validates JWT token
pub async fn auth_middleware(
    State(validator): State<Arc<JwtValidator>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    // Extract Bearer token from Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::MissingToken)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthError::InvalidAuthHeader)?;

    // Validate token
    let claims = validator.validate(token).await?;

    // Log authenticated request (without sensitive data)
    tracing::info!(
        user_id = claims.user_id(),
        "Authenticated request"
    );

    // Add claims to request extensions for use in handlers
    request.extensions_mut().insert(AuthenticatedUser(claims));

    Ok(next.run(request).await)
}

/// Require specific role middleware
pub fn require_role(role: &'static str) -> impl Fn(AuthenticatedUser) -> Result<(), AuthError> + Clone {
    move |user: AuthenticatedUser| {
        if user.0.has_role(role) {
            Ok(())
        } else {
            Err(AuthError::InsufficientPermissions)
        }
    }
}
```

### Auth Errors

```rust
// src/auth/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Missing authentication token")]
    MissingToken,

    #[error("Invalid Authorization header format")]
    InvalidAuthHeader,

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Token has expired")]
    TokenExpired,

    #[error("Invalid token audience")]
    InvalidAudience,

    #[error("Invalid token issuer")]
    InvalidIssuer,

    #[error("Key not found in JWKS: {0}")]
    KeyNotFound(String),

    #[error("Invalid key: {0}")]
    InvalidKey(String),

    #[error("Failed to fetch JWKS: {0}")]
    JwksError(String),

    #[error("Insufficient permissions")]
    InsufficientPermissions,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AuthError::MissingToken => {
                (StatusCode::UNAUTHORIZED, "Authentication required")
            }
            AuthError::InvalidAuthHeader => {
                (StatusCode::UNAUTHORIZED, "Invalid Authorization header")
            }
            AuthError::InvalidToken(_) => {
                (StatusCode::UNAUTHORIZED, "Invalid authentication token")
            }
            AuthError::TokenExpired => {
                (StatusCode::UNAUTHORIZED, "Session expired, please login again")
            }
            AuthError::InvalidAudience | AuthError::InvalidIssuer => {
                (StatusCode::UNAUTHORIZED, "Invalid authentication token")
            }
            AuthError::KeyNotFound(_) | AuthError::InvalidKey(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Authentication service error")
            }
            AuthError::JwksError(_) => {
                (StatusCode::SERVICE_UNAVAILABLE, "Authentication service unavailable")
            }
            AuthError::InsufficientPermissions => {
                (StatusCode::FORBIDDEN, "You don't have permission to perform this action")
            }
        };

        // Log actual error for debugging
        tracing::warn!(error = %self, "Authentication error");

        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

### Router Setup

```rust
// src/main.rs (excerpt)
use axum::{
    middleware,
    routing::get,
    Router,
};
use std::sync::Arc;

pub fn create_router(
    jwt_validator: Arc<JwtValidator>,
    atlassian_service: Arc<AtlassianService>,
) -> Router {
    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/health", get(health_check));

    // Protected routes (auth required)
    let protected_routes = Router::new()
        .route("/api/atlassian/users", get(get_users))
        .route("/api/atlassian/groups", get(get_groups))
        .layer(middleware::from_fn_with_state(
            jwt_validator.clone(),
            auth_middleware,
        ));

    // Admin routes (auth + admin role required)
    let admin_routes = Router::new()
        .route("/api/admin/sync", post(trigger_sync))
        .route("/api/admin/cache/clear", post(clear_cache))
        .layer(middleware::from_fn_with_state(
            jwt_validator.clone(),
            auth_middleware,
        ))
        .layer(middleware::from_fn(require_admin_role));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(admin_routes)
        .with_state(AppState {
            jwt_validator,
            atlassian_service,
        })
}

/// Admin role check middleware
async fn require_admin_role(
    user: AuthenticatedUser,
    request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    if user.0.has_role("Admin") || user.0.in_group(&std::env::var("ADMIN_GROUP_ID").unwrap_or_default()) {
        Ok(next.run(request).await)
    } else {
        Err(AuthError::InsufficientPermissions)
    }
}
```

---

## Frontend Implementatie (React)

### Dependencies (package.json)

```json
{
  "dependencies": {
    "@azure/msal-browser": "^3.10.0",
    "@azure/msal-react": "^2.0.12",
    "react": "^18.2.0",
    "react-dom": "^18.2.0"
  }
}
```

### MSAL Configuratie

```typescript
// src/auth/msalConfig.ts

import { Configuration, LogLevel } from "@azure/msal-browser";

export const msalConfig: Configuration = {
  auth: {
    clientId: import.meta.env.VITE_AZURE_AD_CLIENT_ID,
    authority: `https://login.microsoftonline.com/${import.meta.env.VITE_AZURE_AD_TENANT_ID}`,
    redirectUri: window.location.origin,
    postLogoutRedirectUri: window.location.origin,
  },
  cache: {
    // Use sessionStorage for better security (cleared when browser closes)
    cacheLocation: "sessionStorage",
    // Set to true for IE11/Edge compatibility
    storeAuthStateInCookie: false,
  },
  system: {
    loggerOptions: {
      logLevel: LogLevel.Warning,
      loggerCallback: (level, message, containsPii) => {
        if (containsPii) return;
        switch (level) {
          case LogLevel.Error:
            console.error(message);
            break;
          case LogLevel.Warning:
            console.warn(message);
            break;
        }
      },
    },
  },
};

// Scopes for API access
export const apiScopes = {
  read: [`api://${import.meta.env.VITE_AZURE_AD_CLIENT_ID}/read`],
};

// Login request configuration
export const loginRequest = {
  scopes: ["openid", "profile", "email", ...apiScopes.read],
};
```

### Auth Provider Setup

```tsx
// src/main.tsx

import React from "react";
import ReactDOM from "react-dom/client";
import { PublicClientApplication } from "@azure/msal-browser";
import { MsalProvider } from "@azure/msal-react";
import { msalConfig } from "./auth/msalConfig";
import App from "./App";

const msalInstance = new PublicClientApplication(msalConfig);

// Initialize MSAL
msalInstance.initialize().then(() => {
  ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
      <MsalProvider instance={msalInstance}>
        <App />
      </MsalProvider>
    </React.StrictMode>
  );
});
```

### Auth Hook

```typescript
// src/auth/useAuth.ts

import { useMsal, useAccount } from "@azure/msal-react";
import { InteractionRequiredAuthError } from "@azure/msal-browser";
import { useCallback } from "react";
import { loginRequest, apiScopes } from "./msalConfig";

export interface AuthUser {
  id: string;
  name: string;
  email: string;
}

export function useAuth() {
  const { instance, accounts } = useMsal();
  const account = useAccount(accounts[0] || null);

  const login = useCallback(async () => {
    try {
      await instance.loginRedirect(loginRequest);
    } catch (error) {
      console.error("Login failed:", error);
      throw error;
    }
  }, [instance]);

  const logout = useCallback(async () => {
    try {
      await instance.logoutRedirect({
        postLogoutRedirectUri: window.location.origin,
      });
    } catch (error) {
      console.error("Logout failed:", error);
    }
  }, [instance]);

  const getAccessToken = useCallback(async (): Promise<string> => {
    if (!account) {
      throw new Error("No authenticated user");
    }

    try {
      // Try silent token acquisition first
      const response = await instance.acquireTokenSilent({
        scopes: apiScopes.read,
        account,
      });
      return response.accessToken;
    } catch (error) {
      if (error instanceof InteractionRequiredAuthError) {
        // Silent acquisition failed, need interactive login
        await instance.acquireTokenRedirect({
          scopes: apiScopes.read,
          account,
        });
        throw new Error("Redirecting to login...");
      }
      throw error;
    }
  }, [instance, account]);

  const user: AuthUser | null = account
    ? {
        id: account.localAccountId,
        name: account.name || "Unknown",
        email: account.username,
      }
    : null;

  return {
    isAuthenticated: !!account,
    user,
    login,
    logout,
    getAccessToken,
  };
}
```

### Protected Route Component

```tsx
// src/auth/ProtectedRoute.tsx

import { useIsAuthenticated, useMsal } from "@azure/msal-react";
import { InteractionStatus } from "@azure/msal-browser";
import { Navigate, useLocation } from "react-router-dom";
import { loginRequest } from "./msalConfig";

interface ProtectedRouteProps {
  children: React.ReactNode;
}

export function ProtectedRoute({ children }: ProtectedRouteProps) {
  const isAuthenticated = useIsAuthenticated();
  const { inProgress, instance } = useMsal();
  const location = useLocation();

  // Show loading while MSAL is processing
  if (inProgress !== InteractionStatus.None) {
    return <div>Loading...</div>;
  }

  // Not authenticated - redirect to login
  if (!isAuthenticated) {
    // Trigger login redirect
    instance.loginRedirect({
      ...loginRequest,
      state: location.pathname, // Remember where user wanted to go
    });
    return <div>Redirecting to login...</div>;
  }

  return <>{children}</>;
}
```

### API Client met Auth

```typescript
// src/api/client.ts

import { useAuth } from "../auth/useAuth";
import { useCallback } from "react";

const API_BASE_URL = import.meta.env.VITE_API_URL || "";

interface ApiClientOptions {
  method?: "GET" | "POST" | "PUT" | "DELETE";
  body?: unknown;
}

export function useApiClient() {
  const { getAccessToken } = useAuth();

  const apiRequest = useCallback(
    async <T>(endpoint: string, options: ApiClientOptions = {}): Promise<T> => {
      const { method = "GET", body } = options;

      // Get fresh access token
      const token = await getAccessToken();

      const response = await fetch(`${API_BASE_URL}${endpoint}`, {
        method,
        headers: {
          Authorization: `Bearer ${token}`,
          "Content-Type": "application/json",
        },
        body: body ? JSON.stringify(body) : undefined,
      });

      // Handle auth errors
      if (response.status === 401) {
        throw new Error("Session expired. Please login again.");
      }

      if (response.status === 403) {
        throw new Error("You don't have permission to access this resource.");
      }

      if (!response.ok) {
        const error = await response.json().catch(() => ({}));
        throw new Error(error.message || `Request failed: ${response.status}`);
      }

      return response.json();
    },
    [getAccessToken]
  );

  return { apiRequest };
}
```

---

## Security Best Practices

### Rust Backend Security

| Requirement           | Implementatie                                              |
| --------------------- | ---------------------------------------------------------- |
| Token in header only  | Lees alleen uit `Authorization` header, nooit uit query    |
| Signature validation  | Valideer RS256 signature met Azure AD public keys          |
| Expiration check      | Reject tokens waar `exp` < current time                    |
| Audience validation   | Reject tokens waar `aud` niet matcht met onze app          |
| Issuer validation     | Alleen accepteer tokens van Azure AD tenant                |
| No sensitive logs     | Log geen tokens, alleen user_id voor audit                 |
| Secure errors         | Return generieke errors, log details server-side           |
| JWKS caching          | Cache public keys om JWKS endpoint niet te overbelasten    |

### React Frontend Security

| Requirement           | Implementatie                                              |
| --------------------- | ---------------------------------------------------------- |
| Token storage         | `sessionStorage` only, NOOIT `localStorage`                |
| No tokens in URL      | Tokens alleen in Authorization header                      |
| Automatic refresh     | MSAL handelt token refresh automatisch af                  |
| Logout cleanup        | Alle tokens verwijderen bij logout                         |
| HTTPS only            | Alle API calls over HTTPS                                  |
| XSS protection        | React's built-in XSS protection                            |
| CSRF protection       | Bearer tokens zijn CSRF-safe                               |

### OWASP Top 10 Alignment

| OWASP Risk                    | Mitigatie                                                  |
| ----------------------------- | ---------------------------------------------------------- |
| A01 Broken Access Control     | RBAC via Azure AD groups, middleware enforcement           |
| A02 Cryptographic Failures    | RS256 signature validation, TLS 1.2+                       |
| A03 Injection                 | Parametrized queries (SQLx), type-safe Rust                |
| A05 Security Misconfiguration | Environment-based config, no secrets in code               |
| A07 Auth Failures             | Azure AD integration, JWT validation, MFA support          |
| A09 Logging Failures          | Structured logging, correlation IDs, no PII                |

---

## Configuratie

### Environment Variables - Backend

| Variable              | Type   | Verplicht | Beschrijving                          |
| --------------------- | ------ | --------- | ------------------------------------- |
| `AZURE_AD_TENANT_ID`  | String | Ja        | Azure AD tenant ID                    |
| `AZURE_AD_CLIENT_ID`  | String | Ja        | Backend App Registration client ID    |
| `AZURE_AD_AUDIENCE`   | String | Ja        | Expected audience (api://...)         |
| `ADMIN_GROUP_ID`      | String | Nee       | Azure AD group ID for admin role      |

### Environment Variables - Frontend

| Variable                   | Type   | Verplicht | Beschrijving                          |
| -------------------------- | ------ | --------- | ------------------------------------- |
| `VITE_AZURE_AD_TENANT_ID`  | String | Ja        | Azure AD tenant ID                    |
| `VITE_AZURE_AD_CLIENT_ID`  | String | Ja        | Frontend App Registration client ID   |
| `VITE_API_URL`             | String | Ja        | Backend API base URL                  |

### Voorbeeld .env files

```bash
# backend/.env
AZURE_AD_TENANT_ID=12345678-1234-1234-1234-123456789012
AZURE_AD_CLIENT_ID=abcdefgh-abcd-abcd-abcd-abcdefghijkl
AZURE_AD_AUDIENCE=api://equans-operational-insights
ADMIN_GROUP_ID=98765432-9876-9876-9876-987654321098

# frontend/.env
VITE_AZURE_AD_TENANT_ID=12345678-1234-1234-1234-123456789012
VITE_AZURE_AD_CLIENT_ID=mnopqrst-mnop-mnop-mnop-mnopqrstuvwx
VITE_API_URL=https://api.equans-insights.com
```

---

## Testing

### Backend Unit Tests

```rust
#[tokio::test]
async fn test_missing_auth_header_returns_401() {
    let app = test_app().await;
    let response = app.get("/api/atlassian/users").await;
    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_invalid_token_returns_401() {
    let app = test_app().await;
    let response = app
        .get("/api/atlassian/users")
        .header("Authorization", "Bearer invalid.token.here")
        .await;
    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_expired_token_returns_401() {
    let app = test_app().await;
    let expired_token = create_expired_test_token();
    let response = app
        .get("/api/atlassian/users")
        .header("Authorization", format!("Bearer {}", expired_token))
        .await;
    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_valid_token_returns_200() {
    let app = test_app().await;
    let valid_token = create_valid_test_token();
    let response = app
        .get("/api/atlassian/users")
        .header("Authorization", format!("Bearer {}", valid_token))
        .await;
    assert_eq!(response.status(), 200);
}
```

### Frontend Tests

```typescript
// src/auth/useAuth.test.ts
describe("useAuth", () => {
  it("should return null user when not authenticated", () => {
    const { result } = renderHook(() => useAuth());
    expect(result.current.isAuthenticated).toBe(false);
    expect(result.current.user).toBeNull();
  });

  it("should include Bearer token in API requests", async () => {
    // Mock MSAL to return test token
    mockMsal.acquireTokenSilent.mockResolvedValue({
      accessToken: "test-token",
    });

    const { result } = renderHook(() => useApiClient());
    await result.current.apiRequest("/api/test");

    expect(fetch).toHaveBeenCalledWith(
      expect.any(String),
      expect.objectContaining({
        headers: expect.objectContaining({
          Authorization: "Bearer test-token",
        }),
      })
    );
  });
});
```

---

## Monitoring & Audit

### Logging Requirements

```rust
// Succesvolle authenticatie
tracing::info!(
    user_id = %claims.user_id(),
    method = %request.method(),
    path = %request.uri().path(),
    "Authenticated API request"
);

// Mislukte authenticatie
tracing::warn!(
    error = %error,
    path = %request.uri().path(),
    "Authentication failed"
);

// Autorisatie geweigerd
tracing::warn!(
    user_id = %claims.user_id(),
    required_role = %role,
    path = %request.uri().path(),
    "Authorization denied - insufficient permissions"
);
```

### Audit Trail

Alle API calls worden gelogd met:

- Timestamp
- User ID (uit JWT claims)
- HTTP method en path
- Response status
- Request duration

---

## Referenties

- [Microsoft Identity Platform Documentation](https://docs.microsoft.com/en-us/azure/active-directory/develop/)
- [MSAL.js Documentation](https://github.com/AzureAD/microsoft-authentication-library-for-js)
- [jsonwebtoken Crate](https://docs.rs/jsonwebtoken/)
- [OWASP Top 10](https://owasp.org/Top10/)
- [ADR-004: API Authentication Strategy](../ADRs/ADR-004-api-authentication.md)
- [FR-004: API Authenticatie en Autorisatie](../Functional-Requirements/FR-004-API-Authentication.md)
