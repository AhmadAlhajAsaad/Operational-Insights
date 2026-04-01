# Unit Testing Guide

**Project:** Equans Operational Insights
**Last Updated:** 2026-03-25

---

## Inhoudsopgave

1. [Overzicht](#1-overzicht)
2. [Backend tests draaien (Rust)](#2-backend-tests-draaien-rust)
3. [Frontend tests draaien (React/TypeScript)](#3-frontend-tests-draaien-reacttypescript)
4. [Nieuwe backend tests schrijven](#4-nieuwe-backend-tests-schrijven)
5. [Nieuwe frontend tests schrijven](#5-nieuwe-frontend-tests-schrijven)
6. [Testconventies](#6-testconventies)
7. [CI/CD integratie](#7-cicd-integratie)
8. [Troubleshooting](#8-troubleshooting)

---

## 1. Overzicht

De unit tests zijn verdeeld over twee stacks:

| Stack | Framework | Locatie | Aantal tests |
|---|---|---|---|
| **Backend** (Rust) | `cargo test` + `tokio-test` | `backend/src/**/*.rs` (inline `#[cfg(test)]`) | 29 |
| **Frontend** (TypeScript/React) | Vitest + React Testing Library | `frontend/src/**/__tests__/*.test.{ts,tsx}` | 12 |

### Geteste modules

**Backend:**

| Module | Bestand | Tests | Beschrijving |
|---|---|---|---|
| GID Matcher | `src/persons/gid_matcher.rs` | 3 | Email-extractie, confidence-berekening, status-drempels |
| Validator | `src/imports/validator.rs` | 4 | Persoonsvalidatie, email-validatie, duplicaten |
| Parser | `src/imports/parser.rs` | 4 | Bestandsdetectie, veldextractie, case-insensitief |
| Merger | `src/imports/merger.rs` | 8 | Mergelogica, datumverwerking, optionele velden |
| JWT Claims | `src/auth/claims.rs` | 5 | User ID, rollen, groepen, admin-check |
| JWT Config | `src/auth/jwt.rs` | 2 | Config laden, JWKS URI constructie |

**Frontend:**

| Component | Bestand | Tests | Beschrijving |
|---|---|---|---|
| AuthContext | `src/context/__tests__/AuthContext.test.tsx` | 4 | Provider-verplichting, initieel state, login, logout |
| ProtectedRoute | `src/components/auth/__tests__/ProtectedRoute.test.tsx` | 3 | Loading-spinner, loginpagina-redirect, content-rendering |
| backendClient | `src/api/__tests__/backendClient.test.ts` | 5 | ApiError-klasse, fetch-succes, foutafhandeling |

---

## 2. Backend tests draaien (Rust)

### Vereisten

- Rust toolchain (1.7x+) via `rustup`
- Geen actieve database nodig (SQLx offline modus)

### Alle unit tests draaien

```powershell
cd backend
$env:SQLX_OFFLINE = "true"
cargo test
```

> **Let op:** Het project gebruikt `sqlx::query!` macro's die normaal een database nodig hebben bij compilatie. Door `SQLX_OFFLINE=true` te zetten worden de gecachede query-metadata uit de `.sqlx/` directory gebruikt.

### Specifieke module testen

```powershell
# Alleen GID matcher tests
cargo test persons::gid_matcher

# Alleen auth tests
cargo test auth::

# Alleen import tests
cargo test imports::
```

### Eén specifieke test draaien

```powershell
cargo test test_extract_gid_from_email
```

### Tests met output tonen

```powershell
cargo test -- --nocapture
```

### Codecoverage meten (optioneel)

```powershell
cargo install cargo-tarpaulin
cargo tarpaulin --out html --output-dir coverage/
```

---

## 3. Frontend tests draaien (React/TypeScript)

### Vereisten

- Node.js 18+ en npm
- Dependencies geïnstalleerd: `npm install`

### Alle tests draaien

```powershell
cd frontend
npm test
```

### Watch-modus (herstart bij wijzigingen)

```powershell
npm run test:watch
```

### Met coverage-rapport

```powershell
npm run test:coverage
```

### Specifieke test-file draaien

```powershell
npx vitest run src/context/__tests__/AuthContext.test.tsx
```

### Specifieke test by name

```powershell
npx vitest run -t "throws when useAuth is used outside"
```

---

## 4. Nieuwe backend tests schrijven

### Waar tests plaatsen

Backend tests staan **inline** in hetzelfde bronbestand, onderaan in een `#[cfg(test)]` module. Dit is de standaard Rust-conventie:

```rust
// src/mijn_module/logica.rs

pub fn bereken_score(waarde: i32) -> i32 {
    waarde * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bereken_score() {
        assert_eq!(bereken_score(5), 10);
    }
}
```

### Arrange-Act-Assert patroon

Volg altijd het AAA-patroon:

```rust
#[test]
fn test_voorbeeld() {
    // Arrange — testdata klaarzetten
    let matcher = GidMatcher::new();
    let person = create_test_person("thomas@equans.com", None);

    // Act — functie aanroepen
    let result = matcher.match_person(&person);

    // Assert — resultaat controleren
    assert!(result.is_some());
    assert_eq!(result.unwrap().confidence, 100);
}
```

### Factory-functies voor testdata

Gebruik herbruikbare factory-functies om testdata aan te maken. Voorbeeld uit `gid_matcher.rs`:

```rust
fn create_test_person(email: &str, local_id: Option<String>) -> Person {
    Person {
        id: 1,
        person_id: "TEST001".to_string(),
        first_name: "Test".to_string(),
        last_name: "Person".to_string(),
        email: email.to_string(),
        local_id,
        // ... overige velden op None/defaults
        status: "Active".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}
```

### Async tests

Voor async functies gebruik `tokio::test`:

```rust
#[tokio::test]
async fn test_async_functie() {
    let result = mijn_async_functie().await;
    assert!(result.is_ok());
}
```

### Error-paden testen

Test altijd zowel het `Ok`- als het `Err`-pad:

```rust
#[test]
fn test_ongeldige_invoer() {
    let result = FileParser::detect_format(b"", "bestand.pdf");
    assert!(result.is_err());
}
```

### Grenswaarden testen

```rust
#[test]
fn test_confidence_grenzen() {
    let matcher = GidMatcher::new();

    // Grens: confidence wordt gecapped op 99 voor auto-IDs
    let mut person = create_test_person("test@equans.com", None);
    person.person_id = "AUTO_test".to_string();
    let result = matcher.match_person(&person).unwrap();
    assert!(result.confidence <= 99);
}
```

---

## 5. Nieuwe frontend tests schrijven

### Waar tests plaatsen

Maak een `__tests__/` directory naast het bronbestand:

```
src/
├── context/
│   ├── AuthContext.tsx
│   └── __tests__/
│       └── AuthContext.test.tsx
├── components/
│   └── auth/
│       ├── ProtectedRoute.tsx
│       └── __tests__/
│           └── ProtectedRoute.test.tsx
└── api/
    ├── backendClient.ts
    └── __tests__/
        └── backendClient.test.ts
```

### Basisstructuur testbestand

```typescript
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import React from "react";

describe("MijnComponent", () => {
  it("toont de juiste tekst", () => {
    render(<MijnComponent />);
    expect(screen.getByText("Hallo")).toBeInTheDocument();
  });
});
```

### Context mocken

Wanneer een component `useAuth()` gebruikt, mock je de context:

```typescript
const mockUseAuth = vi.fn();
vi.mock("../../context/AuthContext", () => ({
  useAuth: () => mockUseAuth(),
}));

// In elke test:
mockUseAuth.mockReturnValue({
  isAuthenticated: true,
  isLoading: false,
  user: { name: "Test User" },
  login: vi.fn(),
  logout: vi.fn(),
});
```

### Fetch / API calls mocken

```typescript
const originalFetch = globalThis.fetch;

beforeEach(() => {
  globalThis.fetch = vi.fn();
});

afterEach(() => {
  globalThis.fetch = originalFetch;
});

it("haalt data op", async () => {
  (globalThis.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
    ok: true,
    json: () => Promise.resolve({ data: "test" }),
  });

  const result = await mijnApiFunctie();
  expect(result).toEqual({ data: "test" });
});
```

### Gebruikersinteractie testen

Gebruik `@testing-library/user-event` voor realistische interactietests:

```typescript
import userEvent from "@testing-library/user-event";
import { waitFor } from "@testing-library/react";

it("reageert op klik", async () => {
  const user = userEvent.setup();

  render(<MijnComponent />);

  await user.click(screen.getByRole("button", { name: "Opslaan" }));

  await waitFor(() => {
    expect(screen.getByText("Opgeslagen!")).toBeInTheDocument();
  });
});
```

### Async state-updates afwachten

Gebruik `waitFor` voor asynchrone state-veranderingen:

```typescript
import { waitFor } from "@testing-library/react";

it("toont data na laden", async () => {
  render(<MijnComponent />);

  await waitFor(() => {
    expect(screen.getByText("Geladen")).toBeInTheDocument();
  });
});
```

### Beschikbare matchers (jest-dom)

Dankzij `@testing-library/jest-dom` zijn deze matchers beschikbaar:

```typescript
expect(element).toBeInTheDocument();
expect(element).toBeVisible();
expect(element).toHaveTextContent("tekst");
expect(element).toBeDisabled();
expect(element).toHaveAttribute("href", "/pad");
expect(element).toHaveClass("actief");
```

---

## 6. Testconventies

### Naamgeving

| Convention | Voorbeeld |
|---|---|
| **Rust testfunctie** | `test_extract_gid_from_email` |
| **Rust testmodule** | `mod tests { }` binnen `#[cfg(test)]` |
| **Frontend testbestand** | `ComponentNaam.test.tsx` of `module.test.ts` |
| **Frontend `describe`** | Componentnaam: `describe("AuthContext", ...)` |
| **Frontend `it`** | Beschrijvende zin: `it("throws when useAuth is used outside AuthProvider")` |

### Isolatie

- **Geen database** in unit tests — gebruik factory-functies en mocks
- **Geen netwerk** — mock `fetch` of externe API's
- **Geen bestandssysteem** — gebruik in-memory data (`&[u8]`, `Cursor`)
- **Geen omgevingsvariabelen** — stel ze in de test zelf in met `std::env::set_var` (Rust) of `vi.stubEnv` (Vitest)

### Coverage-doelen

| Target | Drempel | Doel |
|---|---|---|
| Backend (Rust) | >= 70% | 85% |
| Frontend (TypeScript) | >= 60% | 75% |
| High-risk modules | 100% | 100% |

---

## 7. CI/CD integratie

### Backend stap in pipeline

```yaml
- name: Run backend unit tests
  working-directory: backend
  env:
    SQLX_OFFLINE: "true"
  run: cargo test
```

### Frontend stap in pipeline

```yaml
- name: Run frontend unit tests
  working-directory: frontend
  run: |
    npm ci
    npm test
```

---

## 8. Troubleshooting

### Probleem: `error communicating with database` bij `cargo test`

**Oorzaak:** SQLx probeert tijdens compilatie verbinding te maken met de database.

**Oplossing:** Zet de omgevingsvariabele:

```powershell
$env:SQLX_OFFLINE = "true"
```

Of voeg toe aan `.env`:
```
SQLX_OFFLINE=true
```

### Probleem: `Cannot find module '@testing-library/dom'`

**Oorzaak:** Peer dependency ontbreekt.

**Oplossing:**
```powershell
cd frontend
npm install --save-dev @testing-library/dom
```

### Probleem: Frontend test timeout bij fake timers

**Oorzaak:** `vi.useFakeTimers()` conflicteert met `userEvent`.

**Oplossing:** Gebruik `waitFor` in plaats van fake timers voor async state-updates:

```typescript
// Niet doen:
vi.useFakeTimers();
vi.advanceTimersByTime(600);

// Wel doen:
await waitFor(() => {
  expect(screen.getByTestId("result")).toHaveTextContent("ok");
});
```

### Probleem: `no library targets found` bij `cargo test --lib`

**Oorzaak:** Het backend is een binary crate, geen library.

**Oplossing:** Gebruik `cargo test` zonder de `--lib` vlag:

```powershell
cargo test          # correct
cargo test --lib    # fout — binary crate
```

### Probleem: Tests falen na schema-wijziging

**Oorzaak:** De `.sqlx/` cache bevat verouderde query-metadata.

**Oplossing:** Regenereer de cache met een actieve database:

```powershell
cargo sqlx prepare
```
