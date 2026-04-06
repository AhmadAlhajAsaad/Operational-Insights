# Gebruikershandleiding: Tests Uitvoeren

## Equans Operational Insights Dashboard

| | |
|---|---|
| **Versie** | 1.0 |
| **Datum** | 2026-03-30 |
| **Project** | Equans Operational Insights Dashboard |
| **Auteur** | Ahmad Alhaj Asaad (1035912) |

---

## Inhoudsopgave

1. [Vereisten](#1-vereisten)
2. [Unit tests](#2-unit-tests)
3. [Integratietests](#3-integratietests)
4. [Performance tests](#4-performance-tests)
5. [Security tests](#5-security-tests)
6. [CI/CD pipeline](#6-cicd-pipeline)
7. [Veelgestelde problemen](#7-veelgestelde-problemen)

---

## 1. Vereisten

### 1.1 Software

Zorg dat de volgende tools geinstalleerd zijn voordat je tests uitvoert:

| Tool | Versie | Doel | Installatie |
|---|---|---|---|
| **Rust** | Stable (2021 edition) | Backend unit & integratietests | [rustup.rs](https://rustup.rs) |
| **Node.js** | LTS (20+) | Frontend unit tests | [nodejs.org](https://nodejs.org) |
| **Docker** | 20+ | PostgreSQL testdatabase | [docker.com](https://docker.com) |
| **k6** | 0.50+ | Performance tests | `winget install k6` of [k6.io](https://k6.io) |
| **cargo-audit** | Latest | Rust security audit | `cargo install cargo-audit` |
| **cargo-tarpaulin** | Latest | Code coverage | `cargo install cargo-tarpaulin` |

### 1.2 Omgeving opstarten

Start de testdatabase via Docker Compose:

```bash
cd infra
docker-compose up -d db
```

Controleer of de database draait:

```bash
docker ps | grep postgres
```

Stel de database-URL in als omgevingsvariabele:

```bash
# PowerShell
$env:DATABASE_URL = "postgres://equans:equans@localhost:5433/equans_oi"

# Bash/Linux
export DATABASE_URL="postgres://equans:equans@localhost:5433/equans_oi"
```

Voer de database-migraties uit:

```bash
cd backend
sqlx migrate run
```

---

## 2. Unit Tests

Unit tests testen individuele functies en modules zonder externe afhankelijkheden (database, netwerk). Ze vormen de basis van de testpyramide (60% van alle tests).

### 2.1 Backend unit tests (Rust)

**Locatie:** Inline `#[cfg(test)]` modules in `backend/src/`

| Module | Bestand | Aantal | Beschrijving |
|---|---|---|---|
| GID Matcher | `src/persons/gid_matcher.rs` | 3 | Email-extractie, confidence scores |
| Validator | `src/imports/validator.rs` | 4 | Persoonsvalidatie, email-checks, duplicaten |
| Parser | `src/imports/parser.rs` | 4 | CSV/Excel bestandsdetectie, veldextractie |
| Merger | `src/imports/merger.rs` | 8 | Mergelogica, datumverwerking |
| JWT Claims | `src/auth/claims.rs` | 5 | Token parsing, rollen, admin-check |
| JWT Config | `src/auth/jwt.rs` | 2 | Config laden, JWKS URI |
| Data Masking | `src/security/masking.rs` | 3+ | PII-maskering (AVG) |

#### Alle backend unit tests uitvoeren

```bash
cd backend
cargo test
```

#### Specifieke module testen

```bash
# Alleen de GID matcher tests
cargo test gid_matcher

# Alleen import-gerelateerde tests
cargo test imports

# Alleen auth tests
cargo test auth
```

#### Tests met gedetailleerde output

```bash
cargo test -- --nocapture
```

#### Code coverage genereren

```bash
cargo tarpaulin --out Html --output-dir coverage
# Open coverage/tarpaulin-report.html in je browser
```

### 2.2 Frontend unit tests (React/TypeScript)

**Locatie:** `frontend/src/**/__tests__/*.test.{ts,tsx}`

| Component | Bestand | Aantal | Beschrijving |
|---|---|---|---|
| AuthContext | `src/context/__tests__/AuthContext.test.tsx` | 4 | Provider, login, logout, state |
| ProtectedRoute | `src/components/auth/__tests__/ProtectedRoute.test.tsx` | 3 | Loading, redirect, rendering |
| backendClient | `src/api/__tests__/backendClient.test.ts` | 5 | Fetch, error handling, ApiError |

#### Alle frontend tests uitvoeren

```bash
cd frontend
npm test
```

#### Tests in watch-modus (herstart automatisch bij wijzigingen)

```bash
npm run test:watch
```

#### Tests met coverage-rapport

```bash
npm run test:coverage
```

#### Een specifiek testbestand uitvoeren

```bash
npx vitest run src/api/__tests__/backendClient.test.ts
```

### 2.3 Verwachte output

Bij succesvolle backend tests zie je:

```
running 29 tests
test persons::gid_matcher::tests::test_extract_gid_from_email ... ok
test imports::validator::tests::test_validate_person_valid ... ok
test auth::claims::tests::test_user_id_from_oid ... ok
...
test result: ok. 29 passed; 0 failed; 0 ignored
```

Bij succesvolle frontend tests:

```
 ✓ src/api/__tests__/backendClient.test.ts (5 tests)
 ✓ src/context/__tests__/AuthContext.test.tsx (4 tests)
 ✓ src/components/auth/__tests__/ProtectedRoute.test.tsx (3 tests)

 Test Files  3 passed (3)
      Tests  12 passed (12)
```

---

## 3. Integratietests

Integratietests testen de samenwerking tussen componenten: API-endpoints, database-queries en caching. Ze vereisen een draaiende PostgreSQL-database.

### 3.1 Rust integratietests

**Locatie:** `backend/tests/`

| Bestand | Type | Beschrijving |
|---|---|---|
| `api_integration.rs` | API | REST endpoint tests (personen, organisaties) |
| `database_integration.rs` | Database | Migraties, CRUD-operaties, constraints |
| `common/mod.rs` | Helper | Gedeelde testconfiguratie en fixtures |

#### Integratietests uitvoeren

**Voorwaarde:** PostgreSQL moet draaien (zie sectie 1.2).

```bash
cd backend

# Alle integratietests
cargo test --test api_integration --test database_integration -- --test-threads=1

# Alleen API-integratietests
cargo test --test api_integration -- --test-threads=1

# Alleen database-integratietests
cargo test --test database_integration -- --test-threads=1
```

> **Let op:** Gebruik `--test-threads=1` om race conditions te voorkomen, aangezien de tests dezelfde database delen.

### 3.2 PowerShell E2E-tests

Deze scripts testen de API-endpoints tegen een draaiende backend server.

**Locatie:** `backend/tests/*.ps1`

| Script | Beschrijving |
|---|---|
| `test_person_endpoints.ps1` | CRUD-operaties op personen |
| `test_organization_endpoints.ps1` | Organisatiebeheer |
| `test_import_workflow.ps1` | CSV-import workflow |
| `test_atlassian_endpoints.ps1` | Atlassian-integratie |
| `test_github_endpoints.ps1` | GitHub Enterprise-integratie |
| `test_health_errors.ps1` | Health check en foutafhandeling |
| `test_gid_matching.ps1` | GID-matching logica |

#### PowerShell tests uitvoeren

**Voorwaarde:** De backend server moet draaien.

```powershell
# Start de backend server (in een aparte terminal)
cd backend
cargo run

# Voer alle E2E-tests uit
cd backend/tests
.\run_all_tests.ps1

# Of een specifieke test
.\test_person_endpoints.ps1
```

### 3.3 Verwachte output

Succesvolle integratietests:

```
running 8 tests
test test_health_endpoint ... ok
test test_persons_crud ... ok
test test_organizations_list ... ok
...
test result: ok. 8 passed; 0 failed
```

PowerShell E2E-tests:

```
========================================
  EQUANS OPERATIONAL INSIGHTS BACKEND
      COMPLETE TEST SUITE RUNNER
========================================

[PASS] Backend server is running
[PASS] Health check endpoint
[PASS] Person CRUD operations
[PASS] Organization endpoints
...
```

---

## 4. Performance Tests

Performance tests meten responstijden, doorvoer en stabiliteit onder belasting. We gebruiken k6 van Grafana Labs.

### 4.1 Overzicht testscripts

**Locatie:** `tests/performance/`

| Script | Type | VUs | Duur | Doel |
|---|---|---|---|---|
| `load-test.js` | Load | 50 | 30 min | Normaal werkdaggebruik |
| `peak-load.js` | Peak | 100 | 15 min | Piekbelasting |
| `stress-test.js` | Stress | 200 | 10 min | Systeemlimiet vinden |
| `spike-test.js` | Spike | 10→150→10 | 15 min | Hersteltijd na piek |
| `endurance-test.js` | Endurance | 30 | 8 uur | Memory leaks detecteren |
| `import-test.js` | Import | 1 | 3 runs | CSV-upload performance |
| `sync-impact.js` | Concurrent | 30 | 20 min | Impact van achtergrondtaken |

### 4.2 Acceptatiecriteria

| Metric | Eis | Bron |
|---|---|---|
| API P95 responstijd | < 200ms | TM-08 |
| Dashboard laadtijd | < 3 seconden | TM-09 |
| Gelijktijdige gebruikers | 100 | TC-02 |
| Database querytijd | < 50ms | TS-02 |
| Frontend bundel | < 300 KB gzip | TM-12 |

### 4.3 Tests uitvoeren

**Voorwaarde:** De backend server moet draaien op `http://localhost:8080`.

```bash
# Start de backend
cd backend
cargo run

# Voer een load test uit (in een andere terminal)
k6 run tests/performance/load-test.js

# Met aangepaste base URL
k6 run -e BASE_URL=http://staging:8080 tests/performance/load-test.js

# Resultaten opslaan als JSON
k6 run --out json=results/load-test.json tests/performance/load-test.js
```

#### Individuele tests

```bash
# Peak load test (100 gebruikers)
k6 run tests/performance/peak-load.js

# Stress test (200 gebruikers)
k6 run tests/performance/stress-test.js

# Spike test (plotselinge piek)
k6 run tests/performance/spike-test.js

# Endurance test (8 uur langlopend)
k6 run tests/performance/endurance-test.js

# Import performance
k6 run tests/performance/import-test.js
```

### 4.4 Resultaten interpreteren

k6 toont na afloop een samenvatting:

```
     data_received..................: 12 MB  6.7 kB/s
     data_sent......................: 1.2 MB 672 B/s
     http_req_duration..............: avg=45ms  min=12ms  med=38ms  max=890ms  p(90)=95ms  p(95)=142ms
     http_req_failed................: 0.00%  ✓ 0  ✗ 4521
     http_reqs......................: 4521   2.51/s
     vus............................: 50     min=0  max=50
```

**Waar je op moet letten:**

| Kolom | Betekenis | Goed als... |
|---|---|---|
| `http_req_duration p(95)` | 95e percentiel responstijd | < 200ms |
| `http_req_failed` | Percentage mislukte requests | 0% bij ≤50 VUs |
| `vus` | Aantal gelijktijdige virtuele gebruikers | Bereikt het doelaantal |

### 4.5 Aanvullende tools

```bash
# Frontend bundel grootte controleren
cd frontend
npm run build
# Bekijk de output voor chunk sizes

# Database query performance analyseren (in psql)
EXPLAIN ANALYZE SELECT * FROM persons WHERE email LIKE '%equans%';
```

---

## 5. Security Tests

Security tests controleren authenticatie, autorisatie, dependency-kwetsbaarheden en AVG-naleving.

### 5.1 Dependency audits

#### Rust dependencies scannen

```bash
cd backend

# Basis audit
cargo audit

# Gedetailleerde JSON-output
cargo audit --json > audit-results.json

# Alleen kritieke kwetsbaarheden tonen
cargo audit 2>&1 | findstr /i "critical high"
```

#### Frontend dependencies scannen

```bash
cd frontend

# Basis audit
npm audit

# Gedetailleerde JSON-output
npm audit --json > audit-results.json

# Alleen fixes toepassen (alleen veilige updates)
npm audit fix
```

### 5.2 Beveiligingsunit tests

De backend bevat specifieke security-gerelateerde unit tests:

```bash
cd backend

# JWT claims parsing tests
cargo test auth::claims

# JWT validatie tests
cargo test auth::jwt

# Data masking tests (AVG/PII)
cargo test security::masking
```

**Wat wordt getest:**

| Test | Module | Controle |
|---|---|---|
| JWT claims parsing | `auth/claims.rs` | Correcte extractie van user ID, rollen, groepen |
| JWT config | `auth/jwt.rs` | JWKS URI constructie, config laden |
| Email masking | `security/masking.rs` | `user@example.com` → `u***@e***.com` |
| Token masking | `security/masking.rs` | `ghp_xxxxxxxxxxxx` → `ghp_***` |

### 5.3 Handmatige security tests

De volgende tests worden handmatig uitgevoerd tegen een draaiende server:

#### A01: Broken Access Control (OWASP)

```bash
# Test: API-request zonder token → verwacht 401
curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/api/persons
# Verwachte output: 401

# Test: Request met verlopen token → verwacht 401
curl -s -o /dev/null -w "%{http_code}" \
  -H "Authorization: Bearer VERLOPEN_TOKEN" \
  http://localhost:8080/api/persons
# Verwachte output: 401
```

```powershell
# PowerShell equivalent
$response = Invoke-WebRequest -Uri "http://localhost:8080/api/persons" `
  -Method GET -SkipHttpErrorCheck
$response.StatusCode  # Verwacht: 401
```

#### A02: Cryptographic Failures

```bash
# Controleer of HTTPS actief is (staging/productie)
curl -I https://your-staging-url.com
# Controleer: Strict-Transport-Security header aanwezig

# Controleer dat API-tokens niet in responses staan
curl -s http://localhost:8080/api/github/users | findstr /i "ghp_ token"
# Verwacht: geen resultaten
```

#### A07: Cross-Site Scripting (XSS)

```bash
# Test: XSS-payload in zoekveld
curl -s "http://localhost:8080/api/persons?search=<script>alert(1)</script>"
# Verwacht: input wordt geescaped, geen script-executie
```

### 5.4 AVG-conformiteitscontroles

```bash
# Test: Controleer dat logs geen ongemaskeerde PII bevatten
# Start de backend met debug logging
$env:RUST_LOG = "debug"
cargo run

# Maak enkele API-calls en inspecteer de logs
# Verwacht: e-mailadressen gemaskeerd als u***@e***.com
```

### 5.5 Verwachte audit output

Succesvolle `cargo audit`:

```
    Fetching advisory database from `https://github.com/RustSec/advisory-db`
    Scanning Cargo.lock for vulnerabilities
    No vulnerable packages found
```

Succesvolle `npm audit`:

```
found 0 vulnerabilities
```

---

## 6. CI/CD Pipeline

Alle tests worden ook automatisch uitgevoerd via GitHub Actions bij iedere pull request naar `main`.

### 6.1 Code Review workflow (`.github/workflows/code-review.yml`)

| Stap | Commando | Beschrijving |
|---|---|---|
| Formatting | `cargo fmt --check` | Rust code-stijl |
| Linting | `cargo clippy --all-targets -- -D warnings` | Statische analyse |
| Backend tests | `cargo test --all-features` | Unit + integratietests |
| Coverage | `cargo tarpaulin --out Xml` | Coveragerapport |
| Type check | `npm run lint` (= `tsc --noEmit`) | TypeScript typecheck |
| Frontend build | `npm run build` | Productie build |
| Frontend tests | `npm test -- --coverage` | Unit tests + coverage |

### 6.2 Security Scan workflow (`.github/workflows/security-scan.yml`)

| Trigger | Beschrijving |
|---|---|
| Push naar `main` | Automatisch |
| Pull request | Automatisch |
| Elke maandag 09:00 UTC | Weekelijkse scan |

| Stap | Commando | Fail-conditie |
|---|---|---|
| Rust audit | `cargo audit` | Critical of high vulnerabilities |
| npm audit | `npm audit` | Critical vulnerabilities |

Audit-resultaten worden 30 dagen bewaard als GitHub Actions artifacts.

---

## 7. Veelgestelde Problemen

### Database-verbinding mislukt

```
Error: connection refused (os error 111)
```

**Oplossing:** Controleer of Docker draait en de database is gestart:

```bash
docker-compose -f infra/docker-compose.yml up -d db
```

### `SQLX_OFFLINE` fout bij unit tests

```
Error: failed to connect to database
```

**Oplossing:** Voor unit tests zonder database, stel de offline modus in:

```bash
$env:SQLX_OFFLINE = "true"
cargo test
```

### k6 niet gevonden

```
k6: The term 'k6' is not recognized
```

**Oplossing:** Installeer k6:

```powershell
winget install k6
# Of download van https://k6.io/docs/get-started/installation/
```

### Frontend tests falen met module-errors

```
Error: Cannot find module '@testing-library/react'
```

**Oplossing:** Installeer dependencies opnieuw:

```bash
cd frontend
npm ci
```

### cargo-audit niet gevonden

```
error: no such command: 'audit'
```

**Oplossing:**

```bash
cargo install cargo-audit
```

### Tests lopen vast (timeout)

**Oplossing:** Gebruik `--test-threads=1` voor integratietests:

```bash
cargo test --test api_integration -- --test-threads=1
```

---

## Snelstartgids

Hieronder een samenvatting van de meest gebruikte commando's:

```bash
# === OMGEVING ===
cd infra && docker-compose up -d db           # Start database
cd backend && sqlx migrate run                 # Migraties uitvoeren

# === UNIT TESTS ===
cd backend && cargo test                       # Backend unit tests
cd frontend && npm test                        # Frontend unit tests
cd frontend && npm run test:coverage           # Frontend + coverage

# === INTEGRATIETESTS ===
cd backend && cargo test --test api_integration -- --test-threads=1
cd backend/tests && .\run_all_tests.ps1        # E2E tests (server moet draaien)

# === PERFORMANCE TESTS ===
k6 run tests/performance/load-test.js          # Load test (50 users)
k6 run tests/performance/stress-test.js        # Stress test (200 users)

# === SECURITY TESTS ===
cd backend && cargo audit                      # Rust dependency scan
cd frontend && npm audit                       # npm dependency scan
cd backend && cargo test auth                  # Auth unit tests
cd backend && cargo test security              # Data masking tests
```
