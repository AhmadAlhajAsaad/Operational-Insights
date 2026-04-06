# Integratie Test Plan

## Equans Operational Insights Dashboard

|                    |                                                                             |
| ------------------ | --------------------------------------------------------------------------- |
| **Documentnummer** | ITP-001                                                                     |
| **Versie**         | 1.0                                                                         |
| **Studentnaam**    | Ahmad Alhaj Asaad (1035912)                                                 |
| **Project**        | Equans Operational Insights Dashboard                                       |
| **Opleiding**      | Informatica, Hogeschool Rotterdam                                           |
| **Organisatie**    | Equans Nederland, SLS Digital Platforms (DevOps Forge)                      |
| **Begeleiders**    | Viktor Klein (bedrijf), Brian Veltman (technisch), Jeroen Boogaard (school) |
| **Studiejaar**     | 2025 - 2026                                                                 |
| **Referentie**     | MTP-001, UTP-001                                                            |

---

## Inhoudsopgave

1. [Inleiding](#1-inleiding)
2. [Scope](#2-scope)
3. [Testaanpak](#3-testaanpak)
4. [Tools en frameworks](#4-tools-en-frameworks)
5. [Testomgeving](#5-testomgeving)
6. [Testcases](#6-testcases)
7. [Traceability matrix](#7-traceability-matrix)
8. [Acceptatiecriteria](#8-acceptatiecriteria)
9. [Risico's en mitigatie](#9-risicos-en-mitigatie)
10. [Referenties](#referenties)

---

## 1. Inleiding

### 1.1 Waarom dit document

Ik had in sprint 4 een probleem waar ik lang niet uitkwam. De GID-matcher werkte foutloos in de unit tests, maar in combinatie met de `ImportService` en een echte database kwamen er dubbele e-mailadressen terug. Bleek dat `PersonRepository` een case-sensitive lookup deed, terwijl de matcher alles al lowercase had gemaakt. Twee componenten die los van elkaar prima werken, maar samen niet.

Dat probleem was de directe aanleiding om integratietests serieus aan te pakken. In dit document leg ik vast welke koppelingen ik test, waarom juist die, en hoe ik dat doe. Het is ook mijn verantwoording naar de begeleiders toe.

### 1.2 Plek in de teststrategie

Het MTP (sectie 4.2) schrijft voor dat zo'n 30% van de testactiviteiten op integratieniveau hoort te zitten. In het UTP heb ik de scheidslijn zo getrokken: zodra een test een databaseverbinding of een netwerkconnectie nodig heeft, hoort die hier. Alles wat puur logica is (GID-extractie, validatieregels) staat in het UTP.

### 1.3 Technische context

De backend draait op Rust met Axum 0.7 en SQLx 0.8, gekoppeld aan een PostgreSQL 16 database. De frontend is React 19 met TypeScript 5.9 via Vite 6.4. Er zijn twee externe API's in het spel: de Atlassian Cloud Admin API en de GitHub Enterprise API. Authenticatie loopt via Azure AD (JWT-tokens). Voor het testen gebruik ik PowerShell scripts, `cargo test` en Docker Compose.

---

## 2. Scope

### 2.1 Wat test ik wel

De keuze welke punten ik test komt uit de risk-based strategie in het MTP. Plekken waar data een systeemgrens passeert kregen voorrang, want daar ging het bij mij steeds mis.

**Database-integratie (laag 1 en 2)**

De `PersonRepository` en `OrganizationRepository` praten met de `persons` en `organizations` tabellen. Dat is de kern van het systeem, dus hoog risico. De `ImportService` is misschien nog lastiger, want die raakt bij een import soms honderden records in `imports`, `persons` en `organizations` tegelijk. De cache-laag voor Atlassian (`atlassian_users_cache` e.d.) en GitHub (`github_users_cache`, `github_licenses_cache`, `github_copilot_cache`) heb ik als middelmatig risico ingeschat, maar toch meegenomen omdat ik er eerder bugs in had.

Gerelateerde eisen: M-10, M-11, M-12, M-13, M-17, M-18, M-19, S-05, S-07, S-08, TM-05, TM-07.

**API-endpoints (laag 3)**

Alle REST-routes die door de service-laag naar de database lopen. Dat zijn de person- en organization-endpoints, de import-endpoints (multipart upload, preview, execute), de Atlassian- en GitHub-endpoints (die ook de cache en externe API's raken) en de authenticatie-middleware. Concreet: als een `GET /api/persons` binnenkomt, moet de hele keten werken: routing, service, database query, JSON-serialisatie en terug.

Gerelateerde eisen: M-04, M-05, M-07, M-08, M-09, M-15, TM-03, TM-08.

### 2.2 Wat test ik niet

Frontend-componenttests en pure businesslogica staan in het UTP. Performance-testen doe ik apart met k6 (zie MTP sectie 4.3). Penetratietesting zit niet in de scope, daar heb ik simpelweg de tijd en tooling niet voor. De JFrog Artifactory-koppeling zit niet in de MVP (W-01), dus die sla ik over.

---

## 3. Testaanpak

### 3.1 Bottom-up, en waarom

Ik werk van onder naar boven. Daar zit een simpele reden achter: als er iets mis is in de database-laag, maakt het niet uit of de HTTP-routing perfect werkt. Je krijgt toch verkeerde data terug.

Laag 1 is puur database: migraties draaien, seed-data erin, kijken of constraints werken. Laag 2 voegt de services toe: transacties, rollbacks, paginatie, zoekfilters. Laag 3 is de volledige HTTP-cyclus, van request tot JSON-response.

### 3.2 Testdata

Ik had in het begin geen goede seed-data. Elke test vulde ik de database handmatig, en na drie rondes was ik daar helemaal klaar mee. Uiteindelijk heb ik SQL seed-scripts geschreven: zo'n 500 personen verdeeld over 10 organisaties (met variatie in GID-status), 200 Atlassian-gebruikers in de cache, 150 GitHub-gebruikers en 5 historische imports. Gebaseerd op echte Equans CSV-structuren, anders test je met data die in de praktijk nooit voorkomt.

### 3.3 Mocking van externe API's

Atlassian en GitHub zijn niet altijd bereikbaar en hebben rate limits. In de CI-pipeline gebruik ik vastgelegde JSON-responses. Handmatig draai ik `test_atlassian_endpoints.ps1` en `test_github_endpoints.ps1` tegen de echte API's. Alleen mocks gebruiken bleek niet genoeg: ik miste een keer dat Atlassian hun response-formaat had aangepast, waardoor de parser brak. De live tests vingen dat op. Myers et al. (2012) noemen dit het spanningsveld tussen reproduceerbaarheid en representativiteit.

### 3.4 Transactie-testing

De import-workflow is van alle integratiepunten het lastigst. Een import kan honderden records tegelijk aanraken in meerdere tabellen, allemaal in één transactie. Ik test drie dingen: atomiciteit (faalt de import, dan blijft de database schoon), rollback (een eerder uitgevoerde import terugdraaien, maar alleen binnen 30 dagen) en concurrency (twee imports tegelijk mogen niet botsen). Dat laatste leverde een keer een deadlock op in de testdatabase.

---

## 4. Tools en frameworks

**Backend:** `cargo test` voor de Rust integratietests in `backend/tests/`. De PowerShell scripts (bijv. `test_atlassian_endpoints.ps1`) test ik apart, die voeren HTTP-calls uit tegen een draaiende server. In eerste instantie wilde ik alles in Rust doen, maar PowerShell was gewoon sneller voor het doortesten van endpoints. Docker Compose levert de PostgreSQL 16 testdatabase. SQLx regelt de migraties.

**CI/CD:** Bij elke pull request draait `cargo test --all-features` via de GitHub Actions pipeline (`code-review.yml`). Coverage meet ik met `cargo tarpaulin`.

---

## 5. Testomgeving

De database draait als PostgreSQL 16 container via Docker Compose (`infra/docker-compose.yml`). De backend start op `localhost:8080`, de Vite dev server op `localhost:5173` met een `/api` proxy naar de backend. Alles draait in een lokaal Docker-netwerk, los van productie.

Elke testronde begint met een schone database. Ik drop alles, draai de 8 migraties (`001_atlassian_cache.sql` t/m `008_github_db_sync.sql`) en laad de seed-data. In CI start per run een verse PostgreSQL container.

**Omgevingsvariabelen:**

| Variabele             | Testwaarde                                               |
| --------------------- | -------------------------------------------------------- |
| `DATABASE_URL`        | `postgres://equans:equans@localhost:5433/equans_oi_test` |
| `RUST_LOG`            | `debug`                                                  |
| `BACKEND_PORT`        | `8080`                                                   |
| `ATLASSIAN_API_TOKEN` | Mock of echt token                                       |
| `GITHUB_PAT_TOKEN`    | Mock of echt token                                       |
| `AZURE_AD_TENANT_ID`  | `test-tenant` (optioneel)                                |

---

## 6. Testcases

Hieronder de 13 testcases, geordend per laag. Ik heb voor de kerngevallen (import, auth) meer detail opgeschreven, want daar zit het meeste risico.

### TC-IT-001: Databasemigraties

| Attribuut              | Waarde                                                                                                                                                                                                                                                                                                                       |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **ID**                 | TC-IT-001                                                                                                                                                                                                                                                                                                                    |
| **Doel**               | Controleer of alle migraties correct draaien op een lege database en idempotent zijn                                                                                                                                                                                                                                         |
| **Integratielaag**     | Laag 1 (database)                                                                                                                                                                                                                                                                                                            |
| **Testobjecten**       | SQLx migraties, PostgreSQL 16                                                                                                                                                                                                                                                                                                |
| **Precondities**       | Lege PostgreSQL 16 container                                                                                                                                                                                                                                                                                                 |
| **Teststappen**        | 1. Start een lege PostgreSQL 16 container. 2. Voer `sqlx migrate run` uit. 3. Controleer of alle tabellen bestaan (`persons`, `organizations`, `imports`, `atlassian_users_cache`, `github_users_cache`, etc.). 4. Check foreign keys, constraints en indexes. 5. Draai de migraties nog een keer om idempotentie te testen. |
| **Verwacht resultaat** | Alle tabellen aanwezig met correcte foreign keys en constraints. Herhaald draaien van migraties geeft geen fouten.                                                                                                                                                                                                           |
| **Eisen**              | TM-01, TM-05                                                                                                                                                                                                                                                                                                                 |
| **Prioriteit**         | Hoog                                                                                                                                                                                                                                                                                                                         |

### TC-IT-002: Person CRUD

| Attribuut              | Waarde                                                                                                                                                                                   |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **ID**                 | TC-IT-002                                                                                                                                                                                |
| **Doel**               | Test of `PersonRepository` correct personen aanmaakt, ophaalt, bijwerkt en soft-deletet in PostgreSQL                                                                                    |
| **Integratielaag**     | Laag 2 (service + database)                                                                                                                                                              |
| **Testobjecten**       | PersonRepository, `persons` tabel                                                                                                                                                        |
| **Precondities**       | Database met migraties gedraaid                                                                                                                                                          |
| **Teststappen**        | 1. Maak een persoon aan via `create()`. 2. Haal op via `get_by_id()`. 3. Werk het e-mailadres bij. 4. Doe een soft-delete via status. 5. Probeer een duplicaat e-mailadres aan te maken. |
| **Verwacht resultaat** | CRUD-operaties werken correct. Duplicaat e-mailadres geeft een constraint-fout.                                                                                                          |
| **Eisen**              | M-10, M-13, S-05, TM-05                                                                                                                                                                  |
| **Prioriteit**         | Hoog                                                                                                                                                                                     |

### TC-IT-003: Organization CRUD en hierarchie

| Attribuut              | Waarde                                                                                                                                                                                                                                          |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **ID**                 | TC-IT-003                                                                                                                                                                                                                                       |
| **Doel**               | Test het aanmaken, wijzigen en verwijderen van organisaties met parent-child relaties                                                                                                                                                           |
| **Integratielaag**     | Laag 2 (service + database)                                                                                                                                                                                                                     |
| **Testobjecten**       | OrganizationRepository, `organizations` tabel                                                                                                                                                                                                   |
| **Precondities**       | Database met migraties gedraaid                                                                                                                                                                                                                 |
| **Teststappen**        | 1. Maak een parent-organisatie aan. 2. Maak een child-organisatie met `parent_org_id`. 3. Haal de boom op via `GET /api/organizations/tree` en check de nesteling. 4. Wijzig `cost_center`. 5. Verwijder de parent en kijk naar cascade-gedrag. |
| **Verwacht resultaat** | Hierarchie klopt, wijzigingen worden doorgevoerd, cascade-gedrag werkt correct.                                                                                                                                                                 |
| **Eisen**              | M-11, S-07, TM-05                                                                                                                                                                                                                               |
| **Prioriteit**         | Hoog                                                                                                                                                                                                                                            |

### TC-IT-004: Import workflow (upload, preview, execute)

| Attribuut              | Waarde                                                                                                                                                                                                                                                                                                                                                                    |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **ID**                 | TC-IT-004                                                                                                                                                                                                                                                                                                                                                                 |
| **Doel**               | Test de volledige importketen van CSV-upload tot en met database-opslag                                                                                                                                                                                                                                                                                                   |
| **Integratielaag**     | Laag 2 en 3 (service + API + database)                                                                                                                                                                                                                                                                                                                                    |
| **Testobjecten**       | ImportService, `imports` tabel, `persons` tabel                                                                                                                                                                                                                                                                                                                           |
| **Precondities**       | Database met migraties en seed-data, een geldige CSV met 10 persoonsrecords                                                                                                                                                                                                                                                                                               |
| **Teststappen**        | 1. Upload een CSV met 10 persoonsrecords via multipart/form-data naar `POST /api/imports/upload`. 2. Check dat de response een `import_id` en preview bevat met de juiste aantallen. 3. Voer `execute` uit. 4. Controleer of de personen in de database staan via `GET /api/persons`. 5. Controleer dat de import-status `Completed` is en `rollback_data` is opgeslagen. |
| **Verwacht resultaat** | Preview toont correcte aantallen, execute slaat alle personen op, status wordt `Completed`, rollback-data is aanwezig. Hierbij bleek tijdens sprint 4 dat het preview-endpoint soms afweek van execute omdat de preview geen transactie gebruikte. Dat is sindsdien gefixed.                                                                                              |
| **Eisen**              | M-12, S-08, TM-05                                                                                                                                                                                                                                                                                                                                                         |
| **Prioriteit**         | Hoog                                                                                                                                                                                                                                                                                                                                                                      |

### TC-IT-005: Import rollback

| Attribuut              | Waarde                                                                                                                                                                                                                                   |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **ID**                 | TC-IT-005                                                                                                                                                                                                                                |
| **Doel**               | Test of een uitgevoerde import correct teruggedraaid kan worden                                                                                                                                                                          |
| **Integratielaag**     | Laag 2 (service + database)                                                                                                                                                                                                              |
| **Testobjecten**       | ImportService, `imports` tabel, `persons` tabel                                                                                                                                                                                          |
| **Precondities**       | TC-IT-004 is geslaagd, er is een import met status `Completed`                                                                                                                                                                           |
| **Teststappen**        | 1. Noteer de database-staat na een import. 2. Voer rollback uit. 3. Check dat de status `Rolled Back` wordt. 4. Controleer dat aangemaakte personen verdwenen zijn. 5. Controleer dat bijgewerkte personen hun oude waardes terughebben. |
| **Verwacht resultaat** | Status wordt `Rolled Back`, alle wijzigingen zijn teruggedraaid. Een randgeval hierbij was dat rollback van organisatie-wijzigingen niet goed terugdraaide omdat `rollback_data` die niet opsloeg.                                       |
| **Eisen**              | M-12, S-08                                                                                                                                                                                                                               |
| **Prioriteit**         | Hoog                                                                                                                                                                                                                                     |

### TC-IT-006: Import atomiciteit

| Attribuut              | Waarde                                                                                                                                                                                             |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **ID**                 | TC-IT-006                                                                                                                                                                                          |
| **Doel**               | Test dat een import met fouten de database niet in een inconsistente staat achterlaat                                                                                                              |
| **Integratielaag**     | Laag 2 (service + database)                                                                                                                                                                        |
| **Testobjecten**       | ImportService, `imports` tabel, `persons` tabel                                                                                                                                                    |
| **Precondities**       | Database met migraties, een CSV met 5 geldige en 5 ongeldige records                                                                                                                               |
| **Teststappen**        | 1. Upload een CSV met 5 geldige en 5 ongeldige records. 2. Check dat de preview de fouten rapporteert. 3. Probeer execute. 4. Controleer of het aantal personen in de database ongewijzigd blijft. |
| **Verwacht resultaat** | Preview toont foutmeldingen. Na execute is de database ongewijzigd. Import-status is `Failed` of `Completed with errors`, met details in `error_details` (JSONB).                                  |
| **Eisen**              | M-12, TM-05                                                                                                                                                                                        |
| **Prioriteit**         | Hoog                                                                                                                                                                                               |

### TC-IT-007: Atlassian API en caching

| Attribuut              | Waarde                                                                                                                                                                                                                                                                                                                                                       |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **ID**                 | TC-IT-007                                                                                                                                                                                                                                                                                                                                                    |
| **Doel**               | Test de Atlassian API-integratie en het caching-mechanisme                                                                                                                                                                                                                                                                                                   |
| **Integratielaag**     | Laag 3 (API + cache + externe API)                                                                                                                                                                                                                                                                                                                           |
| **Testobjecten**       | Atlassian endpoints, `atlassian_users_cache` tabel                                                                                                                                                                                                                                                                                                           |
| **Precondities**       | Werkend Atlassian API-token (of mock), database met migraties                                                                                                                                                                                                                                                                                                |
| **Teststappen**        | 1. Roep `GET /api/atlassian/organizations` aan. 2. Roep `.../users` aan en check velden `account_id`, `email`, `active`. 3. Roep `.../groups` en `.../licenses/:product` aan. 4. Controleer of de data in `atlassian_users_cache` belandt. 5. Doe dezelfde call nogmaals en meet of het sneller gaat (cache hit). 6. Check dat `expires_at` op 25 uur staat. |
| **Verwacht resultaat** | Alle endpoints geven geldige data terug, cache wordt gevuld, tweede call is sneller. Een uitdaging hierbij was dat de Atlassian API soms inconsistent pagineert, waardoor je bij het cachen niet alle users meekrijgt als je de paginatie niet goed afhandelt.                                                                                               |
| **Eisen**              | M-04, M-18, M-07, S-09, TM-07                                                                                                                                                                                                                                                                                                                                |
| **Prioriteit**         | Hoog                                                                                                                                                                                                                                                                                                                                                         |

### TC-IT-008: GitHub Enterprise API en caching

| Attribuut              | Waarde                                                                                                                                                                                                                                    |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **ID**                 | TC-IT-008                                                                                                                                                                                                                                 |
| **Doel**               | Test de GitHub Enterprise API-integratie en het caching-mechanisme                                                                                                                                                                        |
| **Integratielaag**     | Laag 3 (API + cache + externe API)                                                                                                                                                                                                        |
| **Testobjecten**       | GitHub endpoints, `github_users_cache`, `github_licenses_cache`, `github_copilot_cache`                                                                                                                                                   |
| **Precondities**       | Werkend GitHub PAT-token (of mock), database met migraties                                                                                                                                                                                |
| **Teststappen**        | 1. Roep `GET /api/github/validate` aan om het token te checken. 2. Roep `/overview`, `/copilot/seats`, `/ghas/users` en `/license/users` aan. 3. Controleer de cache-tabellen. 4. Controleer dat het PAT-token niet in de response staat. |
| **Verwacht resultaat** | Alle endpoints geven geldige data terug, cache-tabellen worden gevuld, het PAT-token lekt niet.                                                                                                                                           |
| **Eisen**              | M-04, M-05, M-03, M-15, M-17, M-19, TM-07                                                                                                                                                                                                 |
| **Prioriteit**         | Hoog                                                                                                                                                                                                                                      |

### TC-IT-009: JWT-authenticatie

| Attribuut              | Waarde                                                                                                                                                                                                                                                                                                                                                                         |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **ID**                 | TC-IT-009                                                                                                                                                                                                                                                                                                                                                                      |
| **Doel**               | Test de `auth_middleware` met verschillende scenario's                                                                                                                                                                                                                                                                                                                         |
| **Integratielaag**     | Laag 3 (API + middleware)                                                                                                                                                                                                                                                                                                                                                      |
| **Testobjecten**       | auth_middleware, JWT-validatie                                                                                                                                                                                                                                                                                                                                                 |
| **Precondities**       | Backend draait, Azure AD configuratie beschikbaar (of optionele modus)                                                                                                                                                                                                                                                                                                         |
| **Teststappen**        | 1. Stuur een request zonder Authorization-header (verwacht 401). 2. Stuur een ongeldig of verlopen token (verwacht 401). 3. Stuur een geldig token (verwacht 200, check `user_id` in context). 4. Stuur een token met verkeerde audience (verwacht 403). 5. Zet `AZURE_AD_TENANT_ID` uit en check dat endpoints zonder token werken (optionele modus voor lokaal ontwikkelen). |
| **Verwacht resultaat** | Elk scenario geeft de juiste HTTP-statuscode. De optionele modus werkt voor lokaal ontwikkelen, zodat je niet elke keer een echt Azure AD token hoeft te regelen tijdens het debuggen.                                                                                                                                                                                         |
| **Eisen**              | M-08, M-09, TM-03                                                                                                                                                                                                                                                                                                                                                              |
| **Prioriteit**         | Hoog                                                                                                                                                                                                                                                                                                                                                                           |

### TC-IT-010: Person REST API

| Attribuut              | Waarde                                                                                                                                                                                                                                                                                                   |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **ID**                 | TC-IT-010                                                                                                                                                                                                                                                                                                |
| **Doel**               | Test alle person-endpoints via HTTP                                                                                                                                                                                                                                                                      |
| **Integratielaag**     | Laag 3 (API + service + database)                                                                                                                                                                                                                                                                        |
| **Testobjecten**       | Person endpoints, PersonService, `persons` tabel                                                                                                                                                                                                                                                         |
| **Precondities**       | Backend draait, database met seed-data                                                                                                                                                                                                                                                                   |
| **Teststappen**        | 1. `GET /api/persons` met paginatie. 2. `GET /api/persons?search=thomas`. 3. `POST` met geldig JSON (verwacht 201). 4. `GET /:person_id` (alle velden aanwezig). 5. `PUT` met gewijzigd e-mail (verwacht 200). 6. `POST` met duplicaat e-mail (verwacht 409). 7. `GET /stats` (consistent met database). |
| **Verwacht resultaat** | Paginatie werkt, zoeken geeft correcte resultaten, CRUD gaat goed, duplicaten worden afgevangen, stats kloppen.                                                                                                                                                                                          |
| **Eisen**              | M-10, M-13, S-05, TM-08                                                                                                                                                                                                                                                                                  |
| **Prioriteit**         | Hoog                                                                                                                                                                                                                                                                                                     |

### TC-IT-011: Organization REST API

| Attribuut              | Waarde                                                                                                                                                                                                                                                |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **ID**                 | TC-IT-011                                                                                                                                                                                                                                             |
| **Doel**               | Test alle organization-endpoints via HTTP                                                                                                                                                                                                             |
| **Integratielaag**     | Laag 3 (API + service + database)                                                                                                                                                                                                                     |
| **Testobjecten**       | Organization endpoints, OrganizationService, `organizations` tabel                                                                                                                                                                                    |
| **Precondities**       | Backend draait, database met seed-data inclusief organisatie-hierarchie                                                                                                                                                                               |
| **Teststappen**        | 1. `GET /api/organizations` met paginatie. 2. `GET /tree` (geneste structuur). 3. `GET /:org_id` (check `cost_center`, `manager`, `budget`). 4. `GET /:org_id/persons` (gekoppelde personen). 5. `GET /stats` (totalen). 6. `GET /billing-locations`. |
| **Verwacht resultaat** | Paginatie werkt, boomstructuur klopt, detail bevat alle velden, gekoppelde personen zijn correct, stats en billing-locations geven geldige data.                                                                                                      |
| **Eisen**              | M-11, S-07, TM-08                                                                                                                                                                                                                                     |
| **Prioriteit**         | Hoog                                                                                                                                                                                                                                                  |

### TC-IT-012: Health check

| Attribuut              | Waarde                                                                                                                                           |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| **ID**                 | TC-IT-012                                                                                                                                        |
| **Doel**               | Test of het health-endpoint correct de systeemstatus rapporteert                                                                                 |
| **Integratielaag**     | Laag 3 (API + database)                                                                                                                          |
| **Testobjecten**       | Health endpoint, PostgreSQL-verbinding                                                                                                           |
| **Precondities**       | Backend draait, database beschikbaar                                                                                                             |
| **Teststappen**        | 1. `GET /health`, verwacht `status` = `ok`. 2. Stop PostgreSQL en check het foutgedrag. 3. Herstart PostgreSQL en check of het vanzelf herstelt. |
| **Verwacht resultaat** | Health endpoint geeft `ok` bij werkende database, meldt een fout als PostgreSQL onbereikbaar is, en herstelt automatisch na herstart.            |
| **Eisen**              | TM-01, TM-08                                                                                                                                     |
| **Prioriteit**         | Midden                                                                                                                                           |

### TC-IT-013: Error handling

| Attribuut              | Waarde                                                                                                                                                                                                                                                                         |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **ID**                 | TC-IT-013                                                                                                                                                                                                                                                                      |
| **Doel**               | Test of de API consistent en veilig omgaat met foutscenario's                                                                                                                                                                                                                  |
| **Integratielaag**     | Laag 3 (API)                                                                                                                                                                                                                                                                   |
| **Testobjecten**       | Error handling middleware, alle endpoints                                                                                                                                                                                                                                      |
| **Precondities**       | Backend draait                                                                                                                                                                                                                                                                 |
| **Teststappen**        | 1. `GET /api/persons/NONEXISTENT` (verwacht 404 met `{ "error": "NOT_FOUND" }`). 2. Stuur ongeldig JSON in een POST (verwacht 400). 3. Stuur een duplicaat `person_id` (verwacht 409). 4. Forceer een interne fout en check dat de 500-response geen technische details bevat. |
| **Verwacht resultaat** | 404, 400, 409 en 500 geven elk de juiste statuscode. Alle fouten volgen het formaat `{ "error": "CODE", "message": "..." }`. Interne fouten lekken geen technische details.                                                                                                    |
| **Eisen**              | TM-08, TM-05                                                                                                                                                                                                                                                                   |
| **Prioriteit**         | Midden                                                                                                                                                                                                                                                                         |

---

## 7. Traceability matrix

Hieronder de koppeling van testcases naar eisen. Als er iets faalt weet ik direct welke requirement geraakt wordt.

| Testcase  | Wat wordt getest   | Functionele eisen | Technische eisen | Risico |
| --------- | ------------------ | ----------------- | ---------------- | ------ |
| TC-IT-001 | Migraties          | --                | TM-01, TM-05     | Hoog   |
| TC-IT-002 | Person CRUD        | M-10, M-13, S-05  | TM-05            | Hoog   |
| TC-IT-003 | Organization CRUD  | M-11, S-07        | TM-05            | Hoog   |
| TC-IT-004 | Import E2E         | M-12, S-08        | TM-05            | Hoog   |
| TC-IT-005 | Import rollback    | M-12, S-08        | TM-05            | Hoog   |
| TC-IT-006 | Import atomiciteit | M-12              | TM-05            | Hoog   |
| TC-IT-007 | Atlassian + cache  | M-04, M-18, M-07  | TM-07            | Hoog   |
| TC-IT-008 | GitHub + cache     | M-05, M-03, M-17  | TM-07            | Hoog   |
| TC-IT-009 | JWT auth           | M-08, M-09        | TM-03            | Hoog   |
| TC-IT-010 | Person API         | M-10, M-13, S-05  | TM-08            | Hoog   |
| TC-IT-011 | Organization API   | M-11, S-07        | TM-08            | Hoog   |
| TC-IT-012 | Health check       | --                | TM-01, TM-08     | Midden |
| TC-IT-013 | Error handling     | --                | TM-05, TM-08     | Midden |

### Welke requirements zijn gedekt

| Requirement                    | Testcases                       |
| ------------------------------ | ------------------------------- |
| M-04 (Atlassian datacollectie) | TC-IT-007                       |
| M-05 (GitHub datacollectie)    | TC-IT-008                       |
| M-06 (PostgreSQL opslag)       | TC-IT-001, TC-IT-002, TC-IT-003 |
| M-07 (Atlassian gebruikers)    | TC-IT-007                       |
| M-08 (Authenticatie)           | TC-IT-009                       |
| M-09 (Autorisatie)             | TC-IT-009                       |
| M-10 (Persoonsbeheer)          | TC-IT-002, TC-IT-010            |
| M-11 (Organisatiebeheer)       | TC-IT-003, TC-IT-011            |
| M-12 (Dataimport)              | TC-IT-004, TC-IT-005, TC-IT-006 |
| M-13 (GID-matching)            | TC-IT-002, TC-IT-010            |
| M-18 (Atlassian cache)         | TC-IT-007                       |
| M-19 (GitHub cache)            | TC-IT-008                       |
| S-05 (Zoekfunctie)             | TC-IT-010                       |
| S-07 (Organisatiestatistieken) | TC-IT-011                       |

---

## 8. Acceptatiecriteria

### 8.1 Drempelwaarden

| Criterium                          | Minimaal   | Doel       |
| ---------------------------------- | ---------- | ---------- |
| Geslaagde testcases                | >= 90%     | 100%       |
| High-risk punten afgedekt          | 100%       | 100%       |
| Kritieke defects open              | 0          | 0          |
| Database-integriteit na alle tests | Consistent | Consistent |
| API-responstijd (P95)              | < 500ms    | < 200ms    |
| Import-rollback werkt              | Ja         | Ja         |
| Cache werkt                        | Ja         | Ja         |

### 8.2 Wanneer stoppen

De testfase is klaar zodra alle testcases met prioriteit hoog zijn geslaagd en er geen kritieke defects meer openstaan. De traceability matrix moet bevestigen dat alle Must Have-eisen gedekt zijn. Brian reviewt de testcode voor ik afsluit.

---

## 9. Risico's en mitigatie

| #   | Risico                        | Impact | Wat ik eraan doe                                                                 |
| --- | ----------------------------- | ------ | -------------------------------------------------------------------------------- |
| 1   | Externe API's niet bereikbaar | Hoog   | Mock-servers in CI. Live tests alleen handmatig als de tokens werken.            |
| 2   | Testdatabase inconsistent     | Hoog   | Elke ronde: drop, migrate, seed. Geen state tussen tests.                        |
| 3   | Race conditions bij imports   | Midden | Concurrency-tests (TC-IT-006), database-locks en transactie-isolatie testen.     |
| 4   | Mocks wijken af van echte API | Hoog   | Regelmatig live tests draaien. Mocks baseren op echte responses.                 |
| 5   | Tijdsdruk (solo-project)      | Midden | Focussen op high-risk cases. Bottom-up garandeert dat de basis altijd getest is. |
| 6   | Cache-invalidatie bugs        | Midden | TTL-tests in TC-IT-007. Tijd manipuleren om expiry te testen.                    |

---

## Referenties

1. Client Challenge. (z.d.). _IEEE Std 829-2008 -- IEEE Standard for Software and System Test Documentation_. https://www.scribd.com/document/531867110/IEEE-Std-829-2008
2. Myers, G. J., Badgett, T., & Sandler, C. (2012). _The Art of Software Testing_ (Third Edition). John Wiley & Sons, Inc.
3. Olsen, K., Posthuma, M., Ulrich, S., et al. (2021). _Certified Tester Foundation Level Syllabus_. ISTQB. https://istqb-main-web-prod.s3.amazonaws.com/media/documents/ISTQB-CTFL_Syllabus_2018_v3.1.1.pdf
4. MTP-001, Master Test Plan.
5. UTP-001, Unit Test Plan.
6. SRS-001, Software Requirements Specification.
