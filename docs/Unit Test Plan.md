# Unit Test Plan

**Equans Operational Insights Dashboard**

Studentnaam: Ahmad Alhaj Asaad

Project: Equans Operational Insights Dashboard

Opleiding: Informatica – Hogeschool Rotterdam

Organisatie: Equans Nederland – SLS Digital Platforms (DevOps Forge)

Begeleiders: Viktor Klein (bedrijf), Brian Veltman (technisch), Jeroen Boogaard (school)

Studiejaar: 2025 - 2026

Referentie: MTP-001 – Master Test Plan

Inhoudsopgave

[**1\. Inleiding** 4](#_Toc224560729)

[**2\. Scope** 4](#_Toc224560730)

[**3\. Testobjecten** 6](#_Toc224560731)

[**4\. Testaanpak** 8](#_Toc224560732)

[**5\. Tools en frameworks** 10](#_Toc224560733)

[**6\. Testcases** 12](#_Toc224560734)

[**7\. Traceability matrix** 15](#_Toc224560735)

[**8\. Acceptatiecriteria** 16](#_Toc224560736)

[**9\. Risico's en mitigatie** 17](#_Toc224560737)

[**10\. Conventies en richtlijnen** 18](#_Toc224560738)

[**11\. Conclusie** 19](#_Toc224560739)

[**Referenties** 20](#_Toc224560740)

**Samenvatting**

Dit Unit Test Plan (UTP) beschrijft de aanpak voor het geïsoleerd testen van individuele softwarecomponenten binnen het Operational Insights Dashboard. Het document identificeert concrete testobjecten, definieert de testaanpak en beschrijft de tooling die wordt ingezet. Daarnaast bevat het concrete testcases die zijn gekoppeld aan functionele en technische requirements.

Unit testing vormt de basis van de testpyramide (Myers et al., 2012) en is gericht op het vroegtijdig detecteren van defecten in businesslogica, validatie en dataverwerking. De nadruk ligt op white-box testen van Rust-backendlogica en black-box testen van React-frontendcomponenten, conform de risk-based teststrategie uit het Master Test Plan (MTP-001).

**1\. Inleiding**

**1.1 Doel van dit document**

Dit Unit Test Plan beschrijft welke componenten worden getest op unitniveau, hoe deze tests zijn opgezet en welke tools worden ingezet. Het document dient als praktische leidraad voor de ontwikkelaar en als kwaliteitsverantwoording voor begeleiders en stakeholders.

**1.2 Relatie met Master Test Plan**

Dit UTP is een detailuitwerking van sectie 4.1 (Unit Testing) uit het Master Test Plan. De scope, teststrategie en coveragedoelen zijn hieruit overgenomen. De testpyramide schrijft voor dat 60% van alle testactiviteiten op unitniveau plaatsvindt (MTP-001, §3.3).

**1.3 Projectcontext**

|              |                                                               |
| ------------ | ------------------------------------------------------------- |
| Aspect       | Invulling                                                     |
| Project      | Equans Operational Insights Dashboard                         |
| Backend      | Rust 1.7x, Axum 0.7, SQLx 0.8, PostgreSQL 16                  |
| Frontend     | React 19, TypeScript 5.9, Vite 6.4, Tailwind CSS              |
| Testtools BE | Rust #\[test\] macro, cargo test, cargo tarpaulin, tokio-test |
| Testtools FE | Vitest (aanbevolen), React Testing Library                    |

**2\. Scope**

**2.1 In scope**

De unit-testscope richt zich op componenten die businesslogica, datavalidatie of beveiligingslogica bevatten. De selectie is gebaseerd op de risk-based teststrategie (MTP-001, §3.1): componenten met een hoog risicoprofiel worden prioritair getest.

**Backend (Rust)**

|                        |                         |               |                                                                                               |
| ---------------------- | ----------------------- | ------------- | --------------------------------------------------------------------------------------------- |
| Module                 | Component               | Risicoprofiel | Argumentatie                                                                                  |
| persons/gid_matcher    | GID-matching algoritme  | Hoog          | Kernlogica voor persoon-identificatie; foutieve matching leidt tot onjuiste licentiekoppeling |
| imports/validator      | CSV/Excel-validatie     | Hoog          | Valideert externe invoer; fouten in validatie leiden tot corrupte data                        |
| imports/parser         | Bestandsparsing         | Hoog          | Verantwoordelijk voor correct inlezen van CSV/Excel-bestanden                                 |
| imports/merger         | Datamerging             | Midden        | Samenvoegen van geïmporteerde en bestaande data; dedupplicatielogica                          |
| auth/claims            | JWT-claimsvalidatie     | Hoog          | Beveiligingskritiek: bepaalt gebruikersidentiteit en autorisatie                              |
| auth/jwt               | JWT-configuratie        | Hoog          | Foutieve configuratie leidt tot authenticatiefouten of beveiligingslekken                     |
| auth/middleware        | Authenticatiemiddleware | Hoog          | Poortwachter voor alle beveiligde endpoints                                                   |
| atlassian/service      | Businesslogica          | Midden        | Verwerking van Atlassian-gebruikersdata en licentieberekeningen                               |
| atlassian/link_service | Gebruikerskoppeling     | Midden        | Koppeling van personen aan Atlassian-accounts                                                 |
| github/token_manager   | Tokenmanagement         | Hoog          | Beheer van GitHub App-tokens; foutief beheer leidt tot API-storingen                          |
| config                 | Applicatieconfiguratie  | Laag          | Inlezen en valideren van omgevingsvariabelen                                                  |
| health                 | Health checks           | Laag          | Eenvoudige statuscontrole                                                                     |

**Frontend (React/TypeScript)**

|                   |               |                                                                      |
| ----------------- | ------------- | -------------------------------------------------------------------- |
| Component         | Risicoprofiel | Argumentatie                                                         |
| AuthContext       | Hoog          | Beheert authenticatiestatus; fouten leiden tot beveiligingsproblemen |
| ProtectedRoute    | Hoog          | Route guard; foutieve werking geeft ongeautoriseerde toegang         |
| backendClient     | Midden        | Centraal API-communicatiepunt; foutafhandeling en tokenmanagement    |
| Chartcomponenten  | Laag          | UI-weergave; functioneel risico beperkt                              |
| Importcomponenten | Midden        | Bestandsupload en -validatie aan clientzijde                         |

**2.2 Out of scope**

|                                 |                                                                               |
| ------------------------------- | ----------------------------------------------------------------------------- |
| Component                       | Reden                                                                         |
| Repository-laag (repository.rs) | Betreft databaseinteractie; valt onder integratietests (MTP-001, §4.2)        |
| Route handlers (routes/)        | HTTP-request/response-afhandeling; getest via integratietests met testservers |
| jobs/daily_sync                 | Orchestratielogica met externe afhankelijkheden; valt onder integratietests   |
| cache/repository                | Database-caching; vereist databaseconnectie voor zinvolle tests               |
| Radix UI-componenten (ui/)      | Third-party bibliotheek; verantwoordelijkheid van de library-maintainers      |
| Pagina-componenten (pages)      | Vereisen volledige context (routing, data); vallen onder E2E-tests            |
| Layout-componenten              | Puur presentationeel; beperkt functioneel risico                              |

**3\. Testobjecten**

De onderstaande testobjecten zijn geïdentificeerd op basis van de broncode-analyse en de architectuur zoals beschreven in het Software Design Document (SDD).

**3.1 Backend-testobjecten**

**3.1.1 GID Matcher (backend/src/persons/gid_matcher.rs)**

**Verantwoordelijkheid:** Extractie van Global ID (GID) uit e-mailadressen en berekening van een confidence-score voor persoon-identificatie.

**Te testen logica:**

- GID-extractie uit e-mailadres (lokale deel vóór @)
- Confidence-scoreberekening op basis van beschikbare identifiers (person_id, email, local_id)
- Drielaags classificatie: MATCHED (100), PENDING (30–99), UNMATCHED (<30)

**3.1.2 Import Validator (backend/src/imports/validator.rs)**

**Verantwoordelijkheid:** Validatie van geïmporteerde CSV/Excel-persoongegevens voordat deze worden opgeslagen.

**Te testen logica:**

- Validatie van individuele persoonsrecords
- E-mailformaatvalidatie
- Telling van valide/invalide rijen
- Foutrapportage per rij

**3.1.3 Import Parser (backend/src/imports/parser.rs)**

**Verantwoordelijkheid:** Parsing van CSV- en Excel-bestanden naar gestructureerde dataobjecten.

**Te testen logica:**

- CSV-bestandsparsing met kolomherkenning
- Excel-bestandsparsing (via calamine)
- Afhandeling van ontbrekende of onjuiste kolommen
- Bestandstype-detectie

**3.1.4 JWT Claims (backend/src/auth/claims.rs)**

**Verantwoordelijkheid:** Parsing en validatie van Azure AD JWT-claims voor authenticatie en autorisatie.

**Te testen logica:**

- Extractie van gebruikers-ID (UPN-voorkeur)
- Rolcontrole (case-insensitive)
- Groepslidmaatschap-controle
- Admin-detectie (via groep of rol)

**3.1.5 JWT Validator (backend/src/auth/jwt.rs)**

**Verantwoordelijkheid:** Configuratie en initialisatie van JWT-validatie tegen Microsoft Entra ID.

**Te testen logica:**

- Laden van AuthConfig uit omgevingsvariabelen
- Constructie van JWKS URI op basis van tenant-ID
- Foutafhandeling bij ontbrekende configuratie

**3.1.6 Import Merger (backend/src/imports/merger.rs)**

**Verantwoordelijkheid:** Samenvoegen van geïmporteerde persoongegevens met bestaande records.

**Te testen logica:**

- Deduplicatie op basis van person_id of e-mail
- Veldprioriteitsregels bij conflicten
- Merge-resultaat rapportage

**3.1.7 GitHub Token Manager (backend/src/github/token_manager.rs)**

**Verantwoordelijkheid:** Beheer van GitHub App-installatietokens.

**Te testen logica:**

- Token-expiratie detectie
- JWT-generatie voor GitHub App-authenticatie

**3.2 Frontend-testobjecten**

**3.2.1 AuthContext (frontend/src/context/AuthContext.tsx)**

**Verantwoordelijkheid:** React Context provider voor authenticatiestatus via MSAL.

**Te testen logica:**

- Initiële authenticatiestatus
- Login/logout state transitions
- Tokenverstrekking aan child-componenten

**3.2.2 ProtectedRoute (frontend/src/components/auth/ProtectedRoute.tsx)**

**Verantwoordelijkheid:** Route guard die ongeauthenticeerde gebruikers omleidt.

**Te testen logica:**

- Doorlaten van geauthenticeerde gebruikers
- Redirect van ongeauthenticeerde gebruikers
- Weergave van loading-status tijdens authenticatiecontrole

**3.2.3 Backend Client (frontend/src/api/backendClient.ts)**

**Verantwoordelijkheid:** Gecentraliseerde HTTP-client voor backend-communicatie.

**Te testen logica:**

- Correcte URL-constructie
- Toevoeging van Authorization-header
- Foutafhandeling bij HTTP-fouten

**4\. Testaanpak**

**4.1 Arrange-Act-Assert (AAA)**

Alle unit tests volgen het Arrange-Act-Assert-patroon (Myers et al., 2012):

1.  **Arrange:** Voorbereiding van testdata en afhankelijkheden. In Rust wordt dit gerealiseerd met factory-functies zoals create_test_person() en create_test_claims().
2.  **Act:** Uitvoering van de te testen functie of methode.
3.  **Assert:** Verificatie van het resultaat via assertions (assert_eq!, assert! in Rust; expect() in JavaScript).

**Voorbeeld (Rust):**

#\[test\]

fn test_extract_gid_from_email() {

_// Arrange_

let matcher = GidMatcher::new();

_// Act_

let result = matcher.extract_gid_from_email("john.doe@equans.com");

_// Assert_

assert_eq!(result, Some("john.doe".to_string()));

}

**Voorbeeld (TypeScript):**

test('redirects unauthenticated users', () => {

_// Arrange_

const mockAuth = { isAuthenticated: false };

_// Act_

render(&lt;ProtectedRoute auth={mockAuth}&gt;&lt;Dashboard /&gt;&lt;/ProtectedRoute&gt;);

_// Assert_

expect(screen.queryByText('Dashboard')).not.toBeInTheDocument();

});

**4.2 Isolatie en mocking**

Unit tests worden geïsoleerd uitgevoerd zonder afhankelijkheid van externe systemen (databases, API's, bestandssystemen). Isolatie wordt bereikt door:

|                            |                                                               |                                                |
| -------------------------- | ------------------------------------------------------------- | ---------------------------------------------- |
| Techniek                   | Toepassing                                                    | Voorbeeld                                      |
| Pure functies testen       | Functies zonder side effects direct aanroepen                 | GidMatcher::extract_gid_from_email()           |
| Testdata factory           | Helper-functies die representatieve testobjecten genereren    | create_test_person(), create_test_claims()     |
| Trait-based mocking (Rust) | Traits definiëren voor services; mock-implementaties in tests | Mock HttpClient trait voor API-calls           |
| Env-variabele injectie     | Configuratie via std::env::set_var in testcontext             | AuthConfig::from_env() tests                   |
| React context mocking      | Provider-componenten met gecontroleerde testwaarden           | Mock AuthContext met vaste authenticatiestatus |
| API mocking (frontend)     | msw (Mock Service Worker) of vi.mock() voor fetch-calls       | Backend Client tests met gemockte responses    |

**4.3 Testtechnieken**

|                      |                                                                       |
| -------------------- | --------------------------------------------------------------------- |
| Techniek             | Toepassing                                                            |
| Equivalentieklassen  | E-mailvalidatie: geldige domein, ongeldige string, lege invoer        |
| Grenswaarden         | Confidence-score drempels: 0, 29, 30, 99, 100                         |
| Beslissingsafdekking | Alle branches in match*person(): bestaand ID vs. AUTO* vs. geen match |
| Foutpad-testen       | Result&lt;T, E&gt; branches: Ok-pad én Err-pad afdekken               |

**5\. Tools en frameworks**

**5.1 Backend (Rust)**

|                 |        |                                                             |
| --------------- | ------ | ----------------------------------------------------------- |
| Tool            | Versie | Functie                                                     |
| cargo test      | —      | Ingebouwde testrunner voor Rust #\[test\] en #\[cfg(test)\] |
| cargo tarpaulin | 0.27+  | Codecoverage-rapportage (LCOV/HTML)                         |
| tokio-test      | 0.4    | Async testmacro's voor tokio-gebaseerde code                |

Rust maakt gebruik van het ingebouwde testframework via de #\[test\]-attribuutmacro en #\[cfg(test)\]-module. Er is geen extern testframework vereist — dit is een bewuste ontwerpkeuze van de Rust-taal die testinfrastructuur als eersteklas feature biedt.

**Uitvoering:**

_\# Alle unit tests uitvoeren_

cargo test --lib

_\# Specifieke module testen_

cargo test --lib persons::gid_matcher

_\# Met coverage-rapport_

cargo tarpaulin --out Html --output-dir target/coverage

**5.2 Frontend (TypeScript/React)**

|                           |        |                                                    |
| ------------------------- | ------ | -------------------------------------------------- |
| Tool                      | Versie | Functie                                            |
| Vitest                    | 3.x    | Testrunner, compatibel met Vite-buildconfiguratie  |
| React Testing Library     | 16.x   | Component-rendering en DOM-queries                 |
| @testing-library/jest-dom | 6.x    | Uitgebreide DOM-matchers (toBeInTheDocument, etc.) |
| jsdom                     | —      | Browser-omgeving simulatie voor Node.js            |

**Opmerking:** Vitest is gekozen boven Jest vanwege de native integratie met de bestaande Vite-buildconfiguratie. Dit elimineert dubbele configuratie en maakt gebruik van dezelfde transformatiepipeline.

**Uitvoering:**

_\# Alle frontend tests uitvoeren_

npx vitest run

_\# In watch-modus tijdens ontwikkeling_

npx vitest

_\# Met coverage-rapport_

npx vitest run --coverage

**5.3 CI/CD-integratie**

Unit tests worden automatisch uitgevoerd in de GitHub Actions CI-pipeline bij elke push en pull request. Het CI-script bevat:

_\# Backend tests_

\- name: Run backend unit tests

run: cargo test --lib

_\# Frontend tests_

\- name: Run frontend unit tests

run: npx vitest run

**6\. Testcases**

De onderstaande testcases zijn geselecteerd op basis van risicoprofiel en dekking van kernfunctionaliteit. Elke testcase is gekoppeld aan een functioneel (FR) of technisch (TR) requirement.

**TC-UT-001: GID-extractie uit e-mailadres**

|                    |                                                                                                                                                                                                                   |
| ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Attribuut          | Waarde                                                                                                                                                                                                            |
| ID                 | TC-UT-001                                                                                                                                                                                                         |
| Doel               | Verifiëren dat het GID correct wordt geëxtraheerd uit een e-mailadres                                                                                                                                             |
| Testobject         | GidMatcher::extract_gid_from_email()                                                                                                                                                                              |
| Scenario           | Voer GID-extractie uit voor drie e-mailadressen: een standaard bedrijfsadres (thomas.wagensonner@equans.com), een extern adres (john.doe@gmail.com) en een adres met speciale tekens (Test_User-123@example.org). |
| Arrange            | Maak een nieuw GidMatcher-object aan.                                                                                                                                                                             |
| Act                | Roep extract_gid_from_email() aan met elk e-mailadres.                                                                                                                                                            |
| Verwacht resultaat | Respectievelijk Some("thomas.wagensonner"), Some("john.doe") en Some("test_user-123") (lowercase).                                                                                                                |
| Requirement        | FR-005 (Person Management), TR-005 (Person Management)                                                                                                                                                            |
| Prioriteit         | Hoog                                                                                                                                                                                                              |

**TC-UT-002: Confidence-scoreberekening persoon-matching**

|                    |                                                                                                                                                                                                                                     |
| ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Attribuut          | Waarde                                                                                                                                                                                                                              |
| ID                 | TC-UT-002                                                                                                                                                                                                                           |
| Doel               | Verifiëren dat de confidence-score correct wordt berekend op basis van beschikbare identifiers                                                                                                                                      |
| Testobject         | GidMatcher::match_person()                                                                                                                                                                                                          |
| Scenario           | Test drie scenario's: (1) persoon met bestaand person*id → confidence 100 (MATCHED), (2) persoon met AUTO*-prefix → confidence 30–99 (PENDING), (3) persoon met AUTO\_-prefix en onbekend e-mailadres → confidence <50 (UNMATCHED). |
| Arrange            | Maak testpersonen aan via create_test_person() met verschillende person_id- en e-mailcombinaties.                                                                                                                                   |
| Act                | Roep match_person() aan voor elke testpersoon.                                                                                                                                                                                      |
| Verwacht resultaat | Scenario 1: confidence = 100. Scenario 2: 30 ≤ confidence < 100. Scenario 3: confidence < 50.                                                                                                                                       |
| Requirement        | FR-005 (Person Management), TR-005 (Person Management)                                                                                                                                                                              |
| Prioriteit         | Hoog                                                                                                                                                                                                                                |

**TC-UT-003: CSV-importvalidatie van persoonsgegevens**

|                    |                                                                                                                                                                                                                           |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Attribuut          | Waarde                                                                                                                                                                                                                    |
| ID                 | TC-UT-003                                                                                                                                                                                                                 |
| Doel               | Verifiëren dat de importvalidator persoonsrecords correct classificeert als valide of invalide                                                                                                                            |
| Testobject         | Validator::validate_persons()                                                                                                                                                                                             |
| Scenario           | Test drie scenario's: (1) geldig record met alle verplichte velden → valide, (2) record zonder e-mail → valide (accepteert alle invoer), (3) twee records met hetzelfde person_id → valide (deduplicatie in servicelaag). |
| Arrange            | Maak PersonImportRow-objecten aan met variërende velden.                                                                                                                                                                  |
| Act                | Roep Validator::validate_persons() aan met de testdata.                                                                                                                                                                   |
| Verwacht resultaat | Alle drie scenario's: result.valid == true, geen fouten gerapporteerd. E-mailvalidatie-hulpfunctie: is_valid_email("test@example.com") == true, is_valid_email("invalid") == false.                                       |
| Requirement        | FR-007 (Data Synchronization), TR-007 (Data Import)                                                                                                                                                                       |
| Prioriteit         | Hoog                                                                                                                                                                                                                      |

**TC-UT-004: JWT-claimsvalidatie en autorisatie**

|                    |                                                                                                                                                                                                                                                                                                                   |
| ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Attribuut          | Waarde                                                                                                                                                                                                                                                                                                            |
| ID                 | TC-UT-004                                                                                                                                                                                                                                                                                                         |
| Doel               | Verifiëren dat JWT-claims correct worden geparsed en autorisatiebeslissingen juist zijn                                                                                                                                                                                                                           |
| Testobject         | AzureAdClaims methodes: user_id(), has_role(), in_group(), is_admin()                                                                                                                                                                                                                                             |
| Scenario           | Test vijf scenario's: (1) user_id() retourneert UPN als deze aanwezig is, (2) has_role("Viewer") retourneert true (case-insensitive), (3) in_group("group-1") retourneert true voor bestaande groep, (4) is_admin() retourneert true bij admin-groepslidmaatschap, (5) is_admin() retourneert true bij Admin-rol. |
| Arrange            | Maak testclaims aan via create_test_claims() met bekende rollen en groepen.                                                                                                                                                                                                                                       |
| Act                | Roep de respectievelijke methodes aan op het claims-object.                                                                                                                                                                                                                                                       |
| Verwacht resultaat | (1) "user@equans.com", (2) true, (3) true, (4) true met admin-groep / false met andere groep, (5) true met Admin-rol.                                                                                                                                                                                             |
| Requirement        | FR-004 (API Authentication), TR-004 (API Authentication)                                                                                                                                                                                                                                                          |
| Prioriteit         | Hoog                                                                                                                                                                                                                                                                                                              |

**TC-UT-005: AuthConfig laden uit omgevingsvariabelen**

|                    |                                                                                                                                                                                                                                         |
| ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Attribuut          | Waarde                                                                                                                                                                                                                                  |
| ID                 | TC-UT-005                                                                                                                                                                                                                               |
| Doel               | Verifiëren dat de authenticatieconfiguratie correct wordt geladen uit omgevingsvariabelen en dat de JWKS URI correct wordt geconstrueerd                                                                                                |
| Testobject         | AuthConfig::from_env(), JwtValidator::new()                                                                                                                                                                                             |
| Scenario           | Test twee scenario's: (1) stel omgevingsvariabelen in (AZURE_AD_TENANT_ID, AZURE_AD_CLIENT_ID, AZURE_AD_AUDIENCE, ADMIN_GROUP_ID) en laad de config, (2) maak een JwtValidator aan met een bekende tenant-ID en controleer de JWKS URI. |
| Arrange            | Stel omgevingsvariabelen in via std::env::set_var. Maak een AuthConfig-struct aan met bekende waarden.                                                                                                                                  |
| Act                | Roep AuthConfig::from_env() aan. Maak een JwtValidator::new(&config) aan.                                                                                                                                                               |
| Verwacht resultaat | (1) Config bevat tenant_id = "test-tenant", client_id = "test-client", audience = "api://test". (2) JWKS URI = "https://login.microsoftonline.com/my-tenant-id/discovery/v2.0/keys".                                                    |
| Requirement        | TR-004 (API Authentication), TR-011 (GitHub App Authentication)                                                                                                                                                                         |
| Prioriteit         | Hoog                                                                                                                                                                                                                                    |

**7\. Traceability matrix**

De onderstaande matrix toont de koppeling tussen unit-testcases en requirements.

|           |                             |        |                |        |
| --------- | --------------------------- | ------ | -------------- | ------ |
| Testcase  | Testobject                  | FR     | TR             | Risico |
| TC-UT-001 | GidMatcher – extractie      | FR-005 | TR-005         | Hoog   |
| TC-UT-002 | GidMatcher – confidence     | FR-005 | TR-005         | Hoog   |
| TC-UT-003 | Validator – CSV-import      | FR-007 | TR-007         | Hoog   |
| TC-UT-004 | AzureAdClaims – autorisatie | FR-004 | TR-004         | Hoog   |
| TC-UT-005 | AuthConfig – configuratie   | —      | TR-004, TR-011 | Hoog   |

**8\. Acceptatiecriteria**

De unit-testfase wordt als succesvol afgerond beschouwd wanneer aan de volgende criteria is voldaan:

**8.1 Kwantitatieve criteria**

|                                        |               |            |         |
| -------------------------------------- | ------------- | ---------- | ------- |
| Criterium                              | Drempelwaarde | Doelwaarde | Bron    |
| Codecoverage backend (Rust)            | ≥ 70%         | 85%        | MTP-001 |
| Codecoverage frontend (TypeScript)     | ≥ 60%         | 75%        | MTP-001 |
| Percentage geslaagde tests             | 100%          | 100%       | —       |
| Alle high-risk componenten gedekt      | 100%          | 100%       | §2.1    |
| Geen openstaande Critical/High defects | 0             | 0          | MTP-001 |

**8.2 Kwalitatieve criteria**

- Alle vijf gedefinieerde testcases (TC-UT-001 t/m TC-UT-005) zijn geïmplementeerd en slagen.
- Alle Result&lt;T, E&gt;-branches in geteste functies zijn afgedekt (zowel Ok- als Err-pad).
- Testcode volgt het Arrange-Act-Assert-patroon en bevat leesbare factory-functies voor testdata.
- Coverage-rapporten zijn gegenereerd (cargo tarpaulin, Vitest coverage) en opgeslagen in de repository.
- Tests draaien succesvol in de CI-pipeline (GitHub Actions) zonder handmatige configuratie.

**8.3 Exit-criteria**

De unit-testfase is afgerond wanneer:

1.  Alle kwantitatieve drempelwaarden zijn behaald.
2.  Alle gedefinieerde testcases slagen in de CI-pipeline.
3.  Coverage-rapporten zijn beoordeeld door de technisch begeleider.
4.  Eventuele gevonden defects zijn geregistreerd en geclassificeerd conform MTP-001, §7.

**9\. Risico's en mitigatie**

|     |                                             |        |        |                                                                                                                                                          |
| --- | ------------------------------------------- | ------ | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| #   | Risico                                      | Impact | Kans   | Mitigatie                                                                                                                                                |
| 1   | **Onvoldoende testdekking door tijdsdruk**  | Hoog   | Midden | Prioriteer high-risk componenten (auth, GID-matching, validatie); schrijf tests parallel aan implementatie (shift-left).                                 |
| 2   | **Moeilijk testbare async code**            | Midden | Midden | Gebruik tokio-test macro's; isoleer async logica in aparte functies die synchroon testbaar zijn.                                                         |
| 3   | **Gebrek aan frontend-testinfrastructuur**  | Hoog   | Hoog   | Vitest en React Testing Library configureren in Sprint 5; begin met AuthContext en ProtectedRoute.                                                       |
| 4   | **Flaky tests door shared state**           | Midden | Laag   | Vermijd static mut en globale state in tests; gebruik #\[serial\]-attribuut indien nodig. Gebruik std::env::set_var met zorg in tests (single-threaded). |
| 5   | **Testdata reflecteert productiedata niet** | Midden | Midden | Baseer factory-functies op reële datavelden uit de CSV-importspecificatie; valideer met productierepresentatieve datasets.                               |
| 6   | **Coverage rapportage onnauwkeurig**        | Laag   | Laag   | Gebruik cargo tarpaulin met --ignore-tests flag; vergelijk met handmatige code-inspectie voor kritieke modules.                                          |

**10\. Conventies en richtlijnen**

**10.1 Testorganisatie (Rust)**

backend/src/

├── module_name/

│ ├── mod.rs # Publieke interface

│ ├── service.rs # Businesslogica

│ └── service.rs # Bevat #\[cfg(test)\] mod tests { ... }

Tests worden in hetzelfde bestand als de implementatie geplaatst, in een #\[cfg(test)\]-module onderaan het bestand. Dit is de idiomatische Rust-conventie en biedt directe toegang tot private functies.

**10.2 Testorganisatie (Frontend)**

frontend/src/

├── components/

│ ├── auth/

│ │ ├── ProtectedRoute.tsx

│ │ └── ProtectedRoute.test.tsx # Co-located test

Frontend-tests worden naast het bronbestand geplaatst met het .test.tsx-suffix. Dit maakt co-locatie mogelijk en vereenvoudigt imports.

**10.3 Naamgeving**

- **Rust:** test\_&lt;functienaam&gt;\_&lt;scenario&gt; — bijv. test_extract_gid_from_email
- **TypeScript:** Beschrijvende strings — bijv. 'redirects unauthenticated users to login'

**10.4 Factory-functies**

Elke testmodule definieert herbruikbare factory-functies voor testdata:

- create_test_person(email, local_id) — standaard Person met configureerbare velden
- create_test_claims() — standaard AzureAdClaims met bekende rollen en groepen

**11\. Conclusie**

Dit Unit Test Plan beschrijft een concrete, praktisch toepasbare testaanpak voor het Equans Operational Insights Dashboard. De scope is gebaseerd op risk-based prioritering, waarbij authenticatie, persoonsmatching en datavalidatie als hoogste prioriteit zijn aangemerkt.

De bestaande unit tests in de Rust-backend volgen een consistent patroon (Arrange-Act-Assert met factory-functies) en dekken de kernlogica van het systeem. De frontend-testinfrastructuur moet nog worden opgezet met Vitest en React Testing Library.

Door de combinatie van white-box testen (beslissingsafdekking in Rust) en black-box testen (equivalentieklassen en grenswaarden) wordt een evenwichtige testdekking bereikt die aansluit bij de testpyramide en risk-based strategie uit het Master Test Plan.

**Referenties**

1.  Myers, G. J., Badgett, T., & Sandler, C. (2012). _THE ART OF SOFTWARE TESTING_ (Third Edition) \[Book\]. John Wiley & Sons, Inc. https://malenezi.github.io/malenezi/SE401/Books/114-the-art-of-software-testing-3-edition.pdf
