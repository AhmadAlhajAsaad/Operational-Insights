# TR-011: GitHub API Authenticatie

**Status:** Draft
**Datum:** 2026-03-16
**Auteur(s):** Ahmad Alhaj Asaad
**Implementeert:** [FR-011-GitHub-Vendor-Integration](../Functional-Requirements/FR-011-GitHub-Vendor-Integration.md)
**Applies To:** Backend Rust applicatie, GitHub Enterprise API integratie

---

## Samenvatting

Dit document definieert de technische requirements voor authenticatie richting de GitHub Enterprise API. Het systeem maakt gebruik van een **Personal Access Token (PAT)** voor API-authenticatie. Dit biedt een eenvoudige, directe integratie zonder complexe token-management infrastructuur.

---

## Architectuur Overzicht

```
       JWT Auth         
   React Frontend       Rust Backend       
   (Public)              HTTPS Only             (Internal API)     
                        
                                                          
                                                           Personal Access
                                                           Token (PAT)
                                                          
                                               
                                                 GitHub Enterprise   
                                                 REST API v3         
                                               
```

### Authenticatieflow

```mermaid
sequenceDiagram
    autonumber
    participant Backend as Rust Backend
    participant GH as GitHub API

    Note over Backend,GH: API-aanroep met PAT
    Backend->>GH: GET /enterprises/{slug}/consumed-licenses<br/>Authorization: Bearer {PAT}
    GH-->>Backend: 200 OK  License data (JSON)

    Backend->>GH: GET /enterprises/{slug}/copilot/billing/seats<br/>Authorization: Bearer {PAT}
    GH-->>Backend: 200 OK  Copilot data (JSON)
```

---

## Security Requirements

### 1. PAT Configuratie

#### 1.1 Token Aanmaken (MUST)

- De PAT wordt aangemaakt via GitHub Enterprise: `https://github.com/settings/tokens`
- Gebruik een **Fine-grained PAT** of **Classic PAT** met de benodigde scopes
- De tokennaam moet herkenbaar zijn: `equans-operational-insights-backend`
- De token vervalt automatisch (aanbevolen: 1 jaar maximale levensduur)

#### 1.2 Vereiste Scopes (MUST)

De PAT wordt geconfigureerd met **uitsluitend read-only permissions** die nodig zijn voor het dashboard.

##### Classic PAT Scopes

| Scope | Toegangsniveau | Gebruikt voor |
|-------|---------------|---------------|
| `read:enterprise` | Read-only | Enterprise licentie-consumptie, billing info |
| `read:org` | Read-only | Organisatie-leden en per-gebruiker data |
| `copilot` | Read-only | Copilot seat-toewijzingen en gebruik |

##### Fine-grained PAT Permissions (alternatief)

| Permission | Toegangsniveau | Gebruikt voor |
|------------|---------------|---------------|
| `Enterprise administration` | Read-only | Enterprise licentie-consumptie |
| `Organization members` | Read-only | Ophalen van org-leden |
| `Copilot` | Read-only | Copilot seat-toewijzingen |

**MUST NOT:**
- Geen write-permissions toekennen
- Geen repository-content permissions toekennen

---

### 2. Token Opslag & Beheer

#### 2.1 Opslag (CRITICAL)

**MUST:**
- De PAT wordt opgeslagen als omgevingsvariabele (`GITHUB_PAT_TOKEN`)
- De PAT wordt **nooit** gecommit naar versiebeheer
- In productie wordt de token opgeslagen via Docker secrets of CI/CD secrets
- De token wordt **nooit** gelogd, ook niet gedeeltelijk

**MUST NOT:**
- PAT opslaan als bestand op het filesystem in productie
- PAT embedden in de applicatie binary
- PAT delen via onveilige kanalen (e-mail, chat)

#### 2.2 Token Rotatie (SHOULD)

- PAT-tokens worden minimaal elke 12 maanden geroteerd
- Bij vermoeden van compromittering wordt de token onmiddellijk geroteerd en gerevoked
- Na succesvolle rotatie wordt de oude token gerevoked via GitHub

#### 2.3 Omgevingsvariabelen

```bash
# .env file (NEVER commit)
GITHUB_PAT_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
GITHUB_ENTERPRISE_SLUG=equans
```

```bash
# .env.example (commit this)
GITHUB_PAT_TOKEN=your_github_pat_token_here
GITHUB_ENTERPRISE_SLUG=your_enterprise_slug_here
```

---

### 3. API Client Implementatie

#### 3.1 Client Configuratie (MUST)

```rust
use reqwest::Client;

pub struct GitHubApiClient {
    client: Client,
    pat_token: String,
    enterprise_slug: String,
}

impl GitHubApiClient {
    pub fn new(pat_token: String, enterprise_slug: String) -> Self {
        let client = reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .min_tls_version(reqwest::tls::Version::TLS_1_2)
            .https_only(true)
            .user_agent("Equans-Operational-Insights/1.0")
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            pat_token,
            enterprise_slug,
        }
    }

    pub async fn get<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, GitHubApiError> {
        let url = format!("https://api.github.com{}", endpoint);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.pat_token)
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .map_err(GitHubApiError::HttpRequest)?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(GitHubApiError::ApiError { status, body });
        }

        response.json().await.map_err(GitHubApiError::ResponseParse)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GitHubApiError {
    #[error("HTTP request failed: {0}")]
    HttpRequest(reqwest::Error),

    #[error("API error {status}: {body}")]
    ApiError {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("Failed to parse response: {0}")]
    ResponseParse(reqwest::Error),

    #[error("Unauthorized  check PAT token validity and scopes")]
    Unauthorized,

    #[error("Max retries exceeded")]
    MaxRetriesExceeded,
}
```

#### 3.2 Beveiligingsregels voor de token

- Token wordt **nooit** gelogd (zelfs niet gedeeltelijk)
- Token wordt **nooit** naar de frontend gestuurd
- Token wordt **nooit** opgenomen in error responses of stack traces

---

### 4. Rate Limiting & Retry Logic

#### 4.1 Rate Limits

| Authenticatiemethode | Rate Limit | Opmerking |
|---------------------|------------|-----------|
| PAT | 5.000 req/uur | Per gebruiker/token |

#### 4.2 Rate Limit Headers (MUST)

De backend **moet** de volgende response headers verwerken:

| Header | Beschrijving |
|--------|-------------|
| `X-RateLimit-Limit` | Maximaal aantal requests per uur |
| `X-RateLimit-Remaining` | Resterend aantal requests |
| `X-RateLimit-Reset` | UNIX timestamp wanneer de limiet reset |
| `Retry-After` | Aantal seconden wachten (bij 429) |

#### 4.3 Retry Strategie (MUST)

```rust
impl GitHubApiClient {
    async fn request_with_retry<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        max_retries: u32,
    ) -> Result<T, GitHubApiError> {
        let mut retries = 0;

        loop {
            let response = self
                .client
                .get(url)
                .bearer_auth(&self.pat_token)
                .header("Accept", "application/vnd.github+json")
                .header("X-GitHub-Api-Version", "2022-11-28")
                .send()
                .await
                .map_err(GitHubApiError::HttpRequest)?;

            match response.status().as_u16() {
                200..=299 => return Ok(response.json().await.map_err(GitHubApiError::ResponseParse)?),

                401 => {
                    tracing::error!("GitHub API 401  check PAT token validity and scopes");
                    return Err(GitHubApiError::Unauthorized);
                }

                429 => {
                    let retry_after = response
                        .headers()
                        .get("Retry-After")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse::<u64>().ok())
                        .unwrap_or(2u64.pow(retries));

                    tracing::warn!(
                        "GitHub API rate limit hit, retrying in {}s ({}/{})",
                        retry_after, retries + 1, max_retries
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(retry_after)).await;
                }

                status => {
                    let body = response.text().await.unwrap_or_default();
                    tracing::error!("GitHub API error {}: {}", status, body);
                    if retries >= max_retries {
                        return Err(GitHubApiError::MaxRetriesExceeded);
                    }
                    let backoff = std::time::Duration::from_secs(2u64.pow(retries));
                    tokio::time::sleep(backoff).await;
                }
            }

            retries += 1;
        }
    }
}
```

---

### 5. Benodigde API Endpoints

| Endpoint | Doel |
|----------|------|
| `GET /enterprises/{slug}/consumed-licenses` | Enterprise licentie-consumptie (totaal seats) |
| `GET /enterprises/{slug}/copilot/billing/seats` | Copilot seat-toewijzingen en gebruik |
| `GET /enterprises/{slug}/settings/billing/advanced-security` | GHAS billing info en actieve committers |
| `GET /orgs/{org}/members` | Organisatie-leden per-gebruiker data |

---

### 6. Benodigde Rust Crates

| Crate | Versie | Doel |
|-------|--------|------|
| `reqwest` | 0.12 | HTTP client (reeds in gebruik) |
| `tokio` | 1.0 | Async runtime (reeds in gebruik) |
| `thiserror` | 1.0 | Error types (reeds in gebruik) |
| `tracing` | 0.1 | Logging (reeds in gebruik) |

---

### 7. Monitoring & Logging

#### 7.1 Te Loggen Events (MUST)

| Event | Log Level | Details |
|-------|-----------|---------|
| API request succesvol | `DEBUG` | Endpoint, response time |
| API request mislukt | `ERROR` | HTTP status, error body (geen token) |
| Rate limit bereikt | `WARN` | Remaining requests, retry-after |
| Authenticatiefout (401) | `ERROR` | Endpoint (geen token) |
| Max retries bereikt | `ERROR` | Endpoint, aantal pogingen |

#### 7.2 Wat NOOIT Loggen

- PAT token (ook niet gedeeltelijk)
- Request Authorization headers
- Enige waarde die de token kan onthullen

---

## Acceptatiecriteria

- [ ] GitHub API-aanroepen gebruiken PAT authenticatie via `Authorization: Bearer {token}`
- [ ] PAT is opgeslagen als omgevingsvariabele `GITHUB_PAT_TOKEN`, niet in versiebeheer
- [ ] PAT wordt nooit gelogd of naar de frontend gestuurd
- [ ] Rate limit headers worden verwerkt met exponential backoff bij 429-responses
- [ ] Alle endpoints (`/api/github/*`) functioneren correct met PAT
- [ ] Applicatie start niet op als `GITHUB_PAT_TOKEN` ontbreekt of leeg is
- [ ] Foutafhandeling bij 401 geeft duidelijke log-melding zonder de token te onthullen

---

## Gerelateerde Documenten

- Functional Requirement: [FR-011-GitHub-Vendor-Integration](../Functional-Requirements/FR-011-GitHub-Vendor-Integration.md)
- Technical Requirement: [TR-001-Performance-Security-Standards](TR-001-Performance-Security-Standards.md)
- Technical Requirement: [TR-008-Atlassian-User-Management-Security](TR-008-Atlassian-User-Management-Security.md)
- GitHub Docs: [Creating a personal access token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token)
