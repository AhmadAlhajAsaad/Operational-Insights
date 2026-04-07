# Unit Test Plan

## Equans Operational Insights Dashboard

| | |
|---|---|
| **Versie** | 1.0 |
| **Studentnaam** | Ahmad Alhaj Asaad (1035912) |
| **Project** | Equans Operational Insights Dashboard |
| **Opleiding** | Informatica -- Hogeschool Rotterdam |
| **Organisatie** | Equans Nederland -- SLS Digital Platforms (DevOps Forge) |
| **Begeleiders** | Viktor Klein (bedrijf), Brian Veltman (technisch), Jeroen Boogaard (school) |
| **Studiejaar** | 2025 - 2026 |
| **Referentie** | MTP-001 -- Master Test Plan |

---

## Inhoudsopgave

1. [Inleiding](#1-inleiding)
2. [Scope](#2-scope)
3. [Testaanpak](#3-testaanpak)
4. [Tools en frameworks](#4-tools-en-frameworks)
5. [Testcases](#5-testcases)
6. [Traceability matrix](#6-traceability-matrix)
7. [Acceptatiecriteria](#7-acceptatiecriteria)
8. [Risico's en mitigatie](#8-risicos-en-mitigatie)
9. [Referenties](#referenties)

---

## 1. Inleiding

### 1.1 Doel van dit document

Halverwege sprint 3 had ik de GID-matcher zo goed als werkend. Of dat dacht ik, want toen ik een batch met echte Equans-CSV's invoerde kwamen er vreemde confidence-scores uit. Het lokale deel van het e-mailadres werd niet netjes lowercase gemaakt, waardoor `Thomas.Wagensonner` en `thomas.wagensonner` als twee aparte personen werden gezien. Dat was de eerste keer dat ik dacht: hier had een unit test me een uur debuggen bespaard.

Dit Unit Test Plan beschrijft welke componenten op unitniveau worden getest, hoe die tests zijn opgezet en welke tooling ik daarvoor gebruik. Het document dient als leidraad voor mezelf als ontwikkelaar en als kwaliteitsverantwoording richting begeleiders en stakeholders.

### 1.2 Relatie met Master Test Plan

Dit UTP werkt sectie 4.1 (Unit Testing) uit het Master Test Plan verder uit. De scope, teststrategie en coveragedoelen zijn daaruit overgenomen. In de testpyramide uit het MTP staat dat 60% van alle testactiviteiten op unitniveau hoort plaats te vinden (MTP-001, sectie 3.3).

### 1.3 Projectcontext

| Aspect | Invulling |
|---|---|
| **Backend** | Rust 1.7x, Axum 0.7, SQLx 0.8, PostgreSQL 16 |
| **Frontend** | React 19, TypeScript 5.9, Vite 6.4, Tailwind CSS |
| **Testtools backend** | `cargo test`, `cargo tarpaulin`, `tokio-test` |
| **Testtools frontend** | Vitest, React Testing Library |

---

## 2. Scope

### 2.1 In scope

De selectie van testobjecten is gebaseerd op de risk-based teststrategie uit het MTP (sectie 3.1). In de praktijk bleek dat vooral drie gebieden het meeste risico dragen: authenticatielogica (als dat fout gaat heeft iedereen toegang), persoonsmatching (als de GID-matcher verkeerd koppelt, kloppen de licentiekosten niet) en datavalidatie bij imports (als foute CSV-data ongecontroleerd de database in gaat, heb je een probleem). Hierbij is gekozen voor het testen van de componenten die direct raken aan de functionele en technische eisen uit de Software Requirements Specification.

#### Backend (Rust)

| Module | Component | Risico | Gerelateerde eisen |
|---|---|---|---|
| `persons/gid_matcher` | GID-matching algoritme | Hoog | M-10, M-13 |
| `imports/validator` | CSV/Excel-validatie | Hoog | M-12, S-08 |
| `imports/parser` | Bestandsparsing | Hoog | M-12 |
| `imports/merger` | Datamerging | Midden | M-12 |
| `auth/claims` | JWT-claimsvalidatie | Hoog | M-08, M-09, TM-03 |
| `auth/jwt` | JWT-configuratie | Hoog | M-09, TM-03 |
| `auth/middleware` | Authenticatiemiddleware | Hoog | M-09, TM-03 |

#### Frontend (React/TypeScript)

| Component | Risico | Gerelateerde eisen |
|---|---|---|
| `AuthContext` | Hoog | M-08, S-04 |
| `ProtectedRoute` | Hoog | M-09, TM-03 |
| `backendClient` | Midden | TM-08 |

### 2.2 Buiten scope

De repository-laag (`repository.rs`), route handlers en `jobs/daily_sync` vallen onder integratietests omdat ze databasetoegang of externe API-calls vereisen. Radix UI-componenten en paginacomponenten zijn ook buiten scope: die worden op een ander testniveau afgedekt (E2E). Een uitdaging hierbij was dat de grens tussen "unit" en "integratie" niet altijd scherp is. Uiteindelijk heb ik de lijn getrokken bij: heeft het een databaseconnectie nodig? Dan integratie. Gaat het puur om logica en berekeningen? Dan unit.

---

## 3. Testaanpak

### 3.1 Arrange-Act-Assert

Alle unit tests volgen het Arrange-Act-Assert patroon (Myers et al., 2012). In de Rust-backend betekent dat concreet: eerst testdata klaarzetten met factory-functies als `create_test_person()`, dan de te testen functie aanroepen, en tot slot het resultaat controleren met `assert_eq!` of `assert!`.

```rust
#[test]
fn test_extract_gid_from_email() {
    // Arrange
    let matcher = GidMatcher::new();

    // Act
    let result = matcher.extract_gid_from_email("john.doe@equans.com");

    // Assert
    assert_eq!(result, Some("john.doe".to_string()));
}
```

### 3.2 Isolatie

Unit tests draaien zonder afhankelijkheid van database, API`s of bestandssysteem. In Rust gebruik ik pure functies waar mogelijk. Waar dat niet kan (denk aan de `AuthConfig` die omgevingsvariabelen inleest) stel ik die variabelen in de test zelf in via `std::env::set_var`. Op de frontend mock ik de `AuthContext` met vaste waarden zodat ik de `ProtectedRoute` kan testen zonder een echte MSAL-sessie.

Tijdens het opzetten van de tests voor de import-validator bleek dat de `validate_persons()` functie puur in-memory werkt en geen I/O doet. Hierdoor bleek dat een prima kandidaat voor unit testing: geen mocking nodig, gewoon data erin en kijken wat eruit komt.

### 3.3 Testtechnieken

Ik combineer equivalentieklassen (geldig e-mailadres vs. ongeldige string vs. lege invoer), grenswaardentesten (confidence-drempels op 0, 29, 30, 99, 100) en beslissingsafdekking (alle branches in `match_person()`). In Rust moet ook elke `Result<T, E>` branch afgedekt worden, dus zowel het Ok-pad als het Err-pad.

---

## 4. Tools en frameworks

### 4.1 Backend (Rust)

| Tool | Functie |
|---|---|
| `cargo test --lib` | Ingebouwde testrunner |
| `cargo tarpaulin` | Codecoverage (LCOV/HTML) |
| `tokio-test` | Async testmacro`s |

Rust biedt testinfrastructuur als eersteklas feature via de `#[test]`-macro. Daar is geen extern framework voor nodig, en dat was een van de redenen dat ik in het begin vrij snel unit tests kon schrijven zonder eerst een hele testomgeving op te tuigen.

### 4.2 Frontend (TypeScript/React)

| Tool | Functie |
|---|---|
| Vitest | Testrunner (native Vite-integratie) |
| React Testing Library | Component-rendering en DOM-queries |
| `@testing-library/jest-dom` | Uitgebreide DOM-matchers |

Vitest is gekozen boven Jest vanwege de directe integratie met de bestaande Vite-configuratie. In eerste instantie had ik Jest geprobeerd, maar dat vergde dubbele configuratie voor path-aliassen en TypeScript-transformatie. Met Vitest werkt dat out of the box.

---

## 5. Testcases

De testcases zijn geselecteerd op basis van risicoprofiel en dekking van kernfunctionaliteit. Elke testcase is gekoppeld aan functionele en technische eisen uit de SRS.

### TC-UT-001: GID-extractie uit e-mailadres

| Attribuut | Waarde |
|---|---|
| **ID** | TC-UT-001 |
| **Doel** | Verifieren dat het GID correct wordt geextraheerd uit een e-mailadres |
| **Testobject** | `GidMatcher::extract_gid_from_email()` |
| **Scenario** | GID-extractie voor drie adressen: bedrijfsadres (`thomas.wagensonner@equans.com`), extern adres (`john.doe@gmail.com`) en adres met speciale tekens (`Test_User-123@example.org`). |
| **Verwacht resultaat** | `Some("thomas.wagensonner")`, `Some("john.doe")` en `Some("test_user-123")` (lowercase). |
| **Functionele en technische eisen** | M-10, M-13 |
| **Prioriteit** | Hoog |

### TC-UT-002: Confidence-scoreberekening persoon-matching

| Attribuut | Waarde |
|---|---|
| **ID** | TC-UT-002 |
| **Doel** | Verifieren dat de confidence-score correct wordt berekend op basis van beschikbare identifiers |
| **Testobject** | `GidMatcher::match_person()` |
| **Scenario** | Drie scenario`s: (1) persoon met bestaand `person_id` geeft confidence 100 (MATCHED), (2) persoon met `AUTO_`-prefix geeft confidence 30-99 (PENDING), (3) persoon met `AUTO_`-prefix en onbekend e-mailadres geeft confidence <50 (UNMATCHED). |
| **Verwacht resultaat** | Scenario 1: confidence = 100. Scenario 2: 30 <= confidence < 100. Scenario 3: confidence < 50. |
| **Functionele en technische eisen** | M-10, M-13 |
| **Prioriteit** | Hoog |

### TC-UT-003: CSV-importvalidatie van persoonsgegevens

| Attribuut | Waarde |
|---|---|
| **ID** | TC-UT-003 |
| **Doel** | Verifieren dat de importvalidator persoonsrecords correct classificeert als valide of invalide |
| **Testobject** | `Validator::validate_persons()` |
| **Scenario** | Drie scenario`s: (1) geldig record met alle verplichte velden, (2) record zonder e-mail, (3) e-mailvalidatie via hulpfunctie `is_valid_email()`. |
| **Verwacht resultaat** | Scenario 1 en 2: `result.valid == true`. Hulpfunctie: `is_valid_email("test@example.com") == true`, `is_valid_email("invalid") == false`. |
| **Functionele en technische eisen** | M-12, S-08, TM-05 |
| **Prioriteit** | Hoog |

### TC-UT-004: JWT-claimsvalidatie en autorisatie

| Attribuut | Waarde |
|---|---|
| **ID** | TC-UT-004 |
| **Doel** | Verifieren dat JWT-claims correct worden geparsed en autorisatiebeslissingen juist zijn |
| **Testobject** | `AzureAdClaims` methodes: `user_id()`, `has_role()`, `in_group()`, `is_admin()` |
| **Scenario** | Vijf checks: (1) `user_id()` retourneert UPN, (2) `has_role("Viewer")` is case-insensitive true, (3) `in_group("group-1")` true voor bestaande groep, (4) `is_admin()` true bij admin-groep, (5) `is_admin()` true bij Admin-rol. |
| **Verwacht resultaat** | (1) `"user@equans.com"`, (2) `true`, (3) `true`, (4) `true`, (5) `true`. |
| **Functionele en technische eisen** | M-08, M-09, TM-03 |
| **Prioriteit** | Hoog |

### TC-UT-005: AuthConfig laden uit omgevingsvariabelen

| Attribuut | Waarde |
|---|---|
| **ID** | TC-UT-005 |
| **Doel** | Verifieren dat authenticatieconfiguratie correct uit omgevingsvariabelen wordt geladen en de JWKS URI klopt |
| **Testobject** | `AuthConfig::from_env()`, `JwtValidator::new()` |
| **Scenario** | (1) Omgevingsvariabelen instellen en config laden, (2) `JwtValidator` aanmaken en JWKS URI controleren. |
| **Verwacht resultaat** | (1) Config bevat `tenant_id = "test-tenant"`, `client_id = "test-client"`. (2) JWKS URI = `"https://login.microsoftonline.com/my-tenant-id/discovery/v2.0/keys"`. |
| **Functionele en technische eisen** | M-09, TM-02, TM-03 |
| **Prioriteit** | Hoog |

---

## 6. Traceability matrix

De matrix hieronder laat zien welke testcases gekoppeld zijn aan de functionele en technische eisen uit de SRS. Met deze koppeling kan ik bij een gefaalde test direct terugvinden welke requirement geraakt is, en andersom: als iemand vraagt "is M-12 getest?", kan ik dat hieruit aflezen.

| Testcase | Testobject | Functionele eisen | Technische eisen | Risico |
|---|---|---|---|---|
| TC-UT-001 | GidMatcher (extractie) | M-10, M-13 | TM-05 | Hoog |
| TC-UT-002 | GidMatcher (confidence) | M-10, M-13 | TM-05 | Hoog |
| TC-UT-003 | Validator (CSV-import) | M-12, S-08 | TM-05 | Hoog |
| TC-UT-004 | AzureAdClaims (autorisatie) | M-08, M-09 | TM-03 | Hoog |
| TC-UT-005 | AuthConfig (configuratie) | M-09 | TM-02, TM-03 | Hoog |

---

## 7. Acceptatiecriteria

### 7.1 Wanneer is het goed genoeg?

| Criterium | Drempelwaarde | Doelwaarde |
|---|---|---|
| Codecoverage backend (Rust) | >= 70% | 85% |
| Codecoverage frontend (TypeScript) | >= 60% | 75% |
| Geslaagde tests | 100% | 100% |
| High-risk componenten afgedekt | 100% | 100% |

De unit-testfase is afgerond zodra de drempelwaarden zijn behaald, alle gedefinieerde testcases slagen in de CI-pipeline, en de coverage-rapporten zijn beoordeeld. Eerlijk gezegd is 85% backend-coverage ambitieus gezien het tijdsbestek, maar 70% zou haalbaar moeten zijn als ik de tests parallel aan de implementatie schrijf.

---

## 8. Risico's en mitigatie

| # | Risico | Impact | Mitigatie |
|---|---|---|---|
| 1 | Onvoldoende testdekking door tijdsdruk | Hoog | High-risk componenten (auth, GID-matching, validatie) eerst testen; tests schrijven parallel aan implementatie. |
| 2 | Moeilijk testbare async code | Midden | `tokio-test` macro`s gebruiken; async logica isoleren in apart testbare functies. |
| 3 | Frontend-testinfrastructuur ontbreekt | Hoog | Vitest en React Testing Library configureren; beginnen met `AuthContext` en `ProtectedRoute`. |
| 4 | Testdata reflecteert productiedata niet | Midden | Factory-functies baseren op echte datavelden uit de CSV-importspecificatie. |

---

## Referenties

1. Myers, G. J., Badgett, T., & Sandler, C. (2012). *The Art of Software Testing* (Third Edition). John Wiley & Sons, Inc.
