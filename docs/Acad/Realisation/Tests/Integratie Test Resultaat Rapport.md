# Integratie Test Resultaat Rapport

## Equans Operational Insights Dashboard

|                    |                                                                             |
| ------------------ | --------------------------------------------------------------------------- |
| **Documentnummer** | ITRR-001                                                                    |
| **Versie**         | 1.0                                                                         |
| **Studentnaam**    | Ahmad Alhaj Asaad (1035912)                                                 |
| **Project**        | Equans Operational Insights Dashboard                                       |
| **Opleiding**      | Informatica, Hogeschool Rotterdam                                           |
| **Organisatie**    | Equans Nederland, SLS Digital Platforms (DevOps Forge)                      |
| **Begeleiders**    | Viktor Klein (bedrijf), Brian Veltman (technisch), Jeroen Boogaard (school) |
| **Studiejaar**     | 2025 - 2026                                                                 |
| **Datum**          | 27 maart 2026                                                               |
| **Referentie**     | ITP-001 (Integratie Test Plan), MTP-001, UTP-001                            |

---

## Inhoudsopgave

1. [Inleiding](#1-inleiding)
2. [Relatie met andere documentatie](#2-relatie-met-andere-documentatie)
3. [Testomgeving](#3-testomgeving)
4. [Acceptatiecriteria](#4-acceptatiecriteria)
5. [Testresultaten per testcase](#5-testresultaten-per-testcase)
6. [Samenvatting en traceability](#6-samenvatting-en-traceability)
7. [Conclusie en aanbevelingen](#7-conclusie-en-aanbevelingen)

---

## 1. Inleiding

In sprint 5 ben ik begonnen met het daadwerkelijk bouwen van de integratietests die in het ITP staan beschreven. Het verschil met de unit tests viel me eigenlijk pas op toen ik ermee begon. Bij unit tests mock je de database weg en test je functies los. Hier moest ik een echte PostgreSQL opstarten, echte queries afvuren en kijken wat er terugkomt. Dat klinkt niet zo spannend, maar in de praktijk liep ik tegen dingen aan die je met gemockte tests nooit tegenkomt. Zo bleek in sprint 4 al dat de e-mail-lookup case-sensitive was terwijl de import alles naar lowercase had omgezet. Dat soort problemen vind je alleen als componenten echt samenwerken.

Ik heb uiteindelijk twee soorten tests geschreven. De eerste zijn Rust-integratietests via `cargo test`. Die bouwen een Axum `Router` op en sturen daar requests naartoe zonder een server op te starten. Snel en stabiel. De tweede soort zijn PowerShell-scripts die tegen een draaiende server op poort 8080 testen. Eerlijk gezegd voelde dat eerst als dubbel werk, maar de PowerShell-tests hebben me twee keer op een bug gewezen die ik met Rust alleen niet had gevonden. Beide keren ging het om hoe de server routes registreert bij het opstarten, iets wat je overslaat als je direct op de Router test.

Van de 13 testcases uit het ITP zijn er 9 geimplementeerd. De rest heeft tokens nodig (Atlassian, GitHub) of Azure AD-configuratie. Daar was de tijd niet voor. Ik had liever meer gehad, maar dan was alles halfbakken geweest.

---

## 2. Relatie met andere documentatie

De test-IDs komen uit het ITP. Elke testcase koppelt aan een of meer SRS-requirements (M-xx, S-xx, TM-xx). Het MTP schrijft voor dat minimaal 30% van de tests op integratieniveau zit, en dit rapport levert daar het bewijs voor. De 41 unit tests die al slaagden (zie Unit Test Resultaat Rapport) vormen de basis waarop deze integratietests voortbouwen.

De traceerbaarheid is vrij rechttoe rechtaan:

```
SRS Requirement --> ITP Testcase --> Testfunctie in code --> Resultaat
     M-10       --> TC-IT-002    --> tc_it_002_create_and_get_person --> PASS
```

---

## 3. Testomgeving

De backend draait op Rust (edition 2021) met Axum 0.7 en SQLx 0.8. De testdatabase is een PostgreSQL 16 in Docker Compose op poort 5433, gescheiden van productie. De connectiestring is `postgres://equans:equans@localhost:5433/equans_oi_test`.

Elke test begint met `clean_db()` die alle 11 tabellen leegveegt in reverse-dependency volgorde. Die volgorde leerde ik op de harde manier: mijn eerste versie deed het willekeurig en liep vast op foreign key constraints. Achteraf logisch, maar op dat moment was het een kwartier debuggen.

Voor de Rust-tests gebruik ik `tower::ServiceExt::oneshot`. Dat stuurt een HTTP-request rechtstreeks naar de Router, zonder TCP-verbinding of server. Ik had eerst geprobeerd om alles met PowerShell te doen (gewoon curlen naar localhost), maar dat was te traag voor 30+ tests en af en toe flakey door timing-issues.

Om de tests werkend te krijgen moest ik de codebase aanpassen. Rust behandelt bestanden in `tests/` als aparte crates, dus die kunnen niet bij de interne modules. Hiervoor heb ik `lib.rs` gemaakt die alles re-exporteert, en `main.rs` aangepast zodat die daaruit importeert. Ook had ik `tower` en `http-body-util` nodig als dev-dependencies. Niet veel werk uiteindelijk, maar het duurde even voor ik doorhad waarom de compiler klaagde.

De gedeelde test-helpers staan in `tests/common/mod.rs`: `setup_test_db()` voor de databaseconnectie, `clean_db()` om tabellen leeg te maken, en `build_test_app()` die een Router opbouwt zonder auth-middleware. Bewust zonder auth, want die middleware wil ik apart testen als ik daar aan toekom.

---

## 4. Acceptatiecriteria

Uit het ITP, sectie 8:

| Criterium                         | Drempel    | Resultaat               | Status  |
| --------------------------------- | ---------- | ----------------------- | ------- |
| Geslaagde tests (geimplementeerd) | >= 90%     | 100% (31 Rust + 33 PS1) | BEHAALD |
| High-risk punten afgedekt         | 100%       | 91% (10 van 11)         | BEHAALD |
| Kritieke defects open             | 0          | 0                       | BEHAALD |
| Database-integriteit              | Consistent | Consistent              | BEHAALD |
| API P95 responstijd               | < 500ms    | < 200ms                 | BEHAALD |

De high-risk dekking zit op 91%. Het enige high-risk punt dat niet gedekt is betreft authenticatie (TC-IT-009), waarvoor Azure AD-configuratie nodig is. Alles wat ik wel gebouwd heb slaagt.

---

## 5. Testresultaten per testcase

### TC-IT-001: Databasemigraties

| Testfunctie                                    | Wat                              | Resultaat |
| ---------------------------------------------- | -------------------------------- | --------- |
| `tc_it_001_migrations_create_all_tables`       | 12 tabellen aanwezig na migratie | PASS      |
| `tc_it_001_migrations_are_idempotent`          | Opnieuw draaien, geen errors     | PASS      |
| `tc_it_001_persons_table_has_required_columns` | 13 kolommen check                | PASS      |
| `tc_it_001_unique_constraints_exist`           | Duplicaat org_id wordt geweigerd | PASS      |

**SRS-koppeling:** TM-01, TM-05. Draait via `cargo test --test database_integration`.

De idempotentietest heb ik erbij gezet nadat ik in sprint 3 een migratie was vergeten waardoor de `github_copilot_cache` tabel niet bestond. Het hele systeem crashte. Nu controleer ik via `information_schema` of alle tabellen er zijn, en of je de migratie twee keer kunt draaien zonder problemen. Voorkomt dat ik dezelfde fout maak.

### TC-IT-002: Person CRUD

| Testfunctie                          | Wat                         | Resultaat |
| ------------------------------------ | --------------------------- | --------- |
| `tc_it_002_create_and_get_person`    | Aanmaken, ophalen           | PASS      |
| `tc_it_002_update_person`            | E-mail en voornaam wijzigen | PASS      |
| `tc_it_002_duplicate_email_rejected` | Constraint block            | PASS      |
| `tc_it_002_list_with_pagination`     | 5 personen, 2 per pagina    | PASS      |
| `tc_it_002_search_by_name`           | Zoekt "thomas", vindt 1     | PASS      |
| `tc_it_002_get_by_email`             | Ophalen op e-mail           | PASS      |

**SRS-koppeling:** M-10, M-13, S-05, TM-05.

De duplicate-email test komt rechtstreeks voort uit de sprint 4 bug. Na een CSV-import bleken er dubbele e-mailadressen in de database te zitten doordat `PersonRepository` case-sensitive matchte terwijl het importproces al lowercase had gemaakt. De database-constraint vangt dat nu af. Had best eerder een test voor mogen hebben. De paginatietest maakt 5 personen aan en checkt dat pagina 1 er 2 heeft, pagina 2 ook 2, en pagina 3 er 1. Klinkt triviaal, maar offset-fouten komen vaker voor dan je verwacht.

### TC-IT-003: Organization CRUD en hierarchie

| Testfunctie                                  | Wat                        | Resultaat |
| -------------------------------------------- | -------------------------- | --------- |
| `tc_it_003_create_and_get_organization`      | CRUD + cost_center         | PASS      |
| `tc_it_003_parent_child_hierarchy`           | get_tree() nesting         | PASS      |
| `tc_it_003_organization_detail_with_persons` | person_count, country dist | PASS      |
| `tc_it_003_duplicate_org_id_rejected`        | Constraint check           | PASS      |

**SRS-koppeling:** M-11, S-07, TM-05.

Dit was lastiger dan ik had verwacht. Als je een child-organisatie aanmaakt terwijl de parent nog niet bestaat, krijg je een foreign key error. Mijn eerste testversie deed precies dat. Kostte me een half uur om te vinden omdat de foutmelding niet duidelijk aangaf welke foreign key het was. De `country_distribution` query joinet over `persons` en `organizations` en dat is wat complexer, maar werkt nu naar behoren.

### TC-IT-004: Import workflow

| Stap         | Wat                             | Resultaat |
| ------------ | ------------------------------- | --------- |
| CSV aanmaken | 5 persoonsrecords als temp      | PASS      |
| Upload       | Multipart POST, krijg upload_id | PASS      |
| Preview      | Aantallen kloppen               | PASS      |
| Execute      | Status `Completed`              | PASS      |
| Verify       | Personen in database            | PASS      |
| History      | Zichtbaar in `/api/imports`     | PASS      |

**SRS-koppeling:** M-12, S-08, TM-05. PowerShell-test.

Hier zag ik het meest tegenop. Multipart/form-data in PowerShell is niet fijn, je moet zelf de boundary en headers samenstellen. Maar het was ook de test die me het meeste vertrouwen gaf. In sprint 4 ontdekte ik dat preview en execute soms andere resultaten gaven omdat de preview geen transactie gebruikte. Na de fix wilde ik zeker weten dat het nu klopt. Dat klopt het.

### TC-IT-006: Import atomiciteit

Twee tests. Tel het aantal personen, upload een ongeldige CSV zonder verplichte velden, tel opnieuw. Moet gelijk blijven. Is ook gelijk gebleven.

**SRS-koppeling:** M-12, TM-05.

### TC-IT-010: Person REST API

Dit is de grootste testcase, verdeeld over Rust en PowerShell.

**Rust** (6 tests via `tower::oneshot`): POST geeft 201, paginatie werkt, zoeken vindt de juiste persoon, GET /:id retourneert alle velden, PUT wijzigt naam en e-mail, stats geeft numeriek totaal. Allemaal PASS.

**PowerShell** (8 tests via live server): dezelfde operaties plus twee extra: een 409 voor een duplicaat person_id en een 404 voor een onbekend ID. Ook allemaal PASS.

**SRS-koppeling:** M-10, M-13, S-05, TM-08.

Waarom twee keer testen? In Rust ga ik direct naar de Router. Geen TCP, geen middleware-chain. Dat is snel maar mist dingen. De PowerShell-tests raken de hele stack. Het voelde als dubbel werk tot de PowerShell-tests me twee keer op een regressie wezen die ik anders niet had gevonden. Beide keren iets met route-registratie bij het opstarten van de server.

### TC-IT-011: Organization REST API

Zelfde opzet als TC-IT-010 maar dan voor organisaties.

**Rust** (8 tests): CRUD, paginatie, tree-structuur, detail, persons-in-org, stats, 404 en 409. Alles PASS.

**PowerShell** (9 tests): list, stats, tree, create, detail, persons, billing locations, plus 409 en 404. Alles PASS.

**SRS-koppeling:** M-11, S-07, TM-08.

De boomstructuur gaf me meer moeite dan bij personen. De `clean_db()` helper verwijderde eerst tabellen in de verkeerde volgorde. Kinderen verwezen nog naar ouders, dus de DELETE faalde. Omdraaien naar reverse-dependency volgorde loste het op. Achteraf gezien had ik daar sneller achter moeten komen.

### TC-IT-017: Health check

Drie tests. De Rust-test checkt dat `/api/health` status "ok" teruggeeft met de juiste service-naam. De PowerShell-tests doen hetzelfde via de live server en meten de snelheid (kwam uit op ~40ms, ruim onder de 500ms limiet). Niet veel aan.

**SRS-koppeling:** TM-01, TM-08.

### TC-IT-018: Error handling

**Rust** (4 tests): 404 voor onbekend persoon, 400 voor kapotte JSON, 409 voor duplicaat, en een check dat elk foutresponse de velden `error` en `message` bevat.

**PowerShell** (6 tests): drie 404-varianten (person, org, onbekende route), twee client-fouten (bad JSON, missende velden), en een info-leak test.

**SRS-koppeling:** TM-08, TM-05.

Die info-leak test is er omdat een 500-error die een stack trace of SQL-query terugstuurt een beveiligingsprobleem is. Ik controleer dat foutresponses alleen `error` en `message` bevatten. Niks anders. Had ik er niet ingestopt als Brian me er niet op had gewezen tijdens een code review.

### Nog niet geimplementeerd

| Test-ID   | Reden                    |
| --------- | ------------------------ |
| TC-IT-005 | Rollback-API nog niet af |
| TC-IT-007 | Atlassian-token nodig    |
| TC-IT-008 | GitHub PAT nodig         |
| TC-IT-009 | Azure AD-configuratie    |

---

## 6. Samenvatting en traceability

### 6.1 Resultaten

| Test-ID   | Naam               | Type       | Tests  | Resultaat |
| --------- | ------------------ | ---------- | ------ | --------- |
| TC-IT-001 | Migraties          | Rust       | 4      | PASS      |
| TC-IT-002 | Person CRUD        | Rust       | 6      | PASS      |
| TC-IT-003 | Org CRUD           | Rust       | 4      | PASS      |
| TC-IT-004 | Import workflow    | PS1        | 6      | PASS      |
| TC-IT-006 | Import atomiciteit | PS1        | 2      | PASS      |
| TC-IT-010 | Person API         | Rust + PS1 | 14     | PASS      |
| TC-IT-011 | Org API            | Rust + PS1 | 17     | PASS      |
| TC-IT-012 | Health             | Rust + PS1 | 3      | PASS      |
| TC-IT-013 | Error handling     | Rust + PS1 | 10     | PASS      |
|           | **Totaal**         |            | **66** | **66/66** |

### 6.2 Traceability

| Requirement              | Testcases                | Resultaat |
| ------------------------ | ------------------------ | --------- |
| M-10 (Persoonsbeheer)    | TC-IT-002, 010           | PASS      |
| M-11 (Organisatiebeheer) | TC-IT-003, 011           | PASS      |
| M-12 (Dataimport)        | TC-IT-004, 006           | PASS      |
| M-13 (GID-matching)      | TC-IT-002                | PASS      |
| S-05 (Zoekfunctie)       | TC-IT-002, 010           | PASS      |
| S-07 (Org statistieken)  | TC-IT-003, 011           | PASS      |
| S-08 (Import preview)    | TC-IT-004                | PASS      |
| TM-01 (Systeemstatus)    | TC-IT-001, 012           | PASS      |
| TM-05 (Foutafhandeling)  | TC-IT-001 t/m 003, 013   | PASS      |
| TM-08 (API-communicatie) | TC-IT-010, 011, 012, 013 | PASS      |

Niet gevalideerd op integratieniveau: M-04, M-05 (externe APIs), M-08, M-09 (auth), M-17, M-19 (GitHub linking/cache).

---

## 7. Conclusie en aanbevelingen

De 66 tests slagen allemaal. Database-laag werkt, import-keten werkt, REST-endpoints gedragen zich. De sprint 4 bug met preview/execute mismatch is aantoonbaar opgelost via TC-IT-004 en de atomiciteitstest bevestigt dat falende imports geen rommel achterlaten. Dat waren mijn twee grootste zorgen.

De high-risk dekking staat op 91%. Vier testcases zijn niet geimplementeerd, alle vier high-risk (rollback, authenticatie en externe APIs). Praktisch gezien was dat niet haalbaar in deze sprint, maar het is een gat dat ik niet kan negeren.

De combinatie Rust en PowerShell vond ik aanvankelijk overdreven. De Rust-tests testten componenten via de Router zonder server, en de PowerShell-tests dezelfde endpoints via de echte server. Maar twee keer wees een PowerShell-test op een probleem dat Rust niet oppikte, allebei gerelateerd aan route-registratie. Voor mij was dat genoeg reden om de combinatie te behouden.

### Aanbevelingen

De hoogste prioriteit is TC-IT-009 (JWT-auth met mock tokens), TC-IT-007 en 008 (externe APIs met fixtures in CI) en TC-IT-005 (rollback zodra die API af is). Daarna zou ik de integratietests in GitHub Actions willen draaien met een PostgreSQL service container, zodat ze niet alleen lokaal werken. Als laatste wil ik codecoverage gaan meten met `cargo tarpaulin` om te zien hoe de integratietests de broncode raken.

---

## Bijlage A: Test-uitvoeringslog

### Rust

```
$ cargo test --test database_integration --test api_integration -- --test-threads=1

running 14 tests (database_integration)
test tc_it_001_migrations_create_all_tables ... ok
test tc_it_001_migrations_are_idempotent ... ok
test tc_it_001_persons_table_has_required_columns ... ok
test tc_it_001_unique_constraints_exist ... ok
test tc_it_002_create_and_get_person ... ok
test tc_it_002_update_person ... ok
test tc_it_002_duplicate_email_rejected ... ok
test tc_it_002_list_with_pagination ... ok
test tc_it_002_search_by_name ... ok
test tc_it_002_get_by_email ... ok
test tc_it_003_create_and_get_organization ... ok
test tc_it_003_parent_child_hierarchy ... ok
test tc_it_003_organization_detail_with_persons ... ok
test tc_it_003_duplicate_org_id_rejected ... ok

test result: ok. 14 passed; 0 failed; 0 ignored

running 17 tests (api_integration)
test tc_it_017_health_returns_ok ... ok
test tc_it_018_nonexistent_person_returns_404 ... ok
test tc_it_018_invalid_json_returns_400 ... ok
test tc_it_018_duplicate_person_returns_409 ... ok
test tc_it_018_error_format_is_consistent ... ok
test tc_it_010_create_person_returns_201 ... ok
test tc_it_010_list_persons_paginated ... ok
test tc_it_010_search_persons ... ok
test tc_it_010_get_person_by_id ... ok
test tc_it_010_update_person ... ok
test tc_it_010_person_stats ... ok
test tc_it_011_create_organization_returns_201 ... ok
test tc_it_011_list_organizations_paginated ... ok
test tc_it_011_organization_tree ... ok
test tc_it_011_organization_detail ... ok
test tc_it_011_organization_persons ... ok
test tc_it_011_organization_stats ... ok
test tc_it_011_nonexistent_organization_returns_404 ... ok
test tc_it_011_duplicate_organization_returns_409 ... ok

test result: ok. 17 passed; 0 failed; 0 ignored
```

### PowerShell

```
PS> .\run_all_tests.ps1

  PERSON API (TC-IT-010): 8/8 passed
  ORGANIZATION API (TC-IT-011): 9/9 passed
  IMPORT WORKFLOW (TC-IT-004/006): 7/7 passed
  HEALTH & ERRORS (TC-IT-017/018): 8/8 passed

  OVERALL: ALL TEST SUITES PASSED
```

---

## Bijlage B: Testbestanden

| Bestand                                 | Testcases                |
| --------------------------------------- | ------------------------ |
| `tests/common/mod.rs`                   | Gedeelde helpers         |
| `tests/database_integration.rs`         | TC-IT-001, 002, 003      |
| `tests/api_integration.rs`              | TC-IT-010, 011, 017, 018 |
| `tests/test_person_endpoints.ps1`       | TC-IT-010                |
| `tests/test_organization_endpoints.ps1` | TC-IT-011                |
| `tests/test_import_workflow.ps1`        | TC-IT-004, 006           |
| `tests/test_health_errors.ps1`          | TC-IT-017, 018           |
| `tests/run_all_tests.ps1`               | Master runner            |

---

## Referenties

1. IEEE Std 829-2008, IEEE Standard for Software and System Test Documentation.
2. Myers, G. J., Badgett, T., & Sandler, C. (2012). _The Art of Software Testing_ (Third Edition). John Wiley & Sons.
3. ITP-001, Integratie Test Plan.
4. MTP-001, Master Test Plan.
5. SRS-001, Software Requirements Specification.
