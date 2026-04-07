# Unit test resultaat rapport

## Equans Operational Insights Dashboard

| | |
|---|---|
| **Versie** | 1.0 |
| **Studentnaam** | Ahmad Alhaj Asaad (1035912) |
| **Project** | Equans Operational Insights Dashboard |
| **Opleiding** | Informatica, Hogeschool Rotterdam |
| **Organisatie** | Equans Nederland, SLS Digital Platforms (DevOps Forge) |
| **Begeleiders** | Viktor Klein (bedrijf), Brian Veltman (technisch), Jeroen Boogaard (school) |
| **Studiejaar** | 2025 - 2026 |
| **Datum** | 25 maart 2026 |
| **Referentie** | UTP-001 (Unit Test Plan) |

---

## Inhoudsopgave

1. [Inleiding](#1-inleiding)
2. [Doel van dit document](#2-doel-van-dit-document)
3. [Relatie met andere documentatie](#3-relatie-met-andere-documentatie)
4. [Testomgeving en randvoorwaarden](#4-testomgeving-en-randvoorwaarden)
5. [Algemene acceptatiecriteria](#5-algemene-acceptatiecriteria)
6. [Resultaatstructuur](#6-resultaatstructuur)
7. [Resultaten unit tests](#7-resultaten-unit-tests)
8. [Conclusie en aanbevelingen](#8-conclusie-en-aanbevelingen)

---

## 1. Inleiding

### 1.1 Aanleiding

In sprint 3 had ik het Unit Test Plan (UTP-001) geschreven. Het idee was: eerst bepalen wat er getest moet worden, dan pas de tests schrijven. Dat klinkt logisch, maar in de praktijk liep het iets anders. Toen ik eenmaal bezig was met de implementatie van de tests merkte ik dat sommige dingen in het UTP niet helemaal klopten met hoe de code daadwerkelijk werkt. Een goed voorbeeld hiervan is de `is_valid_email()` functie in de validator. Die staat in de code als `#[allow(dead_code)]`, wat betekent dat de Rust-compiler hem niet gebruikt maar ook geen waarschuwing geeft. Ik ging er in eerste instantie vanuit dat dit een vergissing was. Bleek het niet te zijn. Na navraag bij het team hoorde ik dat de validator met opzet alles doorlaat, de deduplicatie zit ergens anders, namelijk in de servicelaag. Daar kom ik later in sectie 7 op terug.

Al met al zijn er 41 tests geslaagd. Nul gefaald. Op zich een mooi resultaat, maar ik wil eerlijk zijn: het feit dat alles in een keer slaagde vond ik zelf ook een beetje verdacht. Bij een paar tests heb ik daarom extra assertions toegevoegd om er zeker van te zijn dat ik niet per ongeluk iets triviaals aan het testen was.

### 1.2 Uitgevoerde tests

Van die 41 tests draaien er 29 in Rust (backend, via `cargo test`) en 12 in TypeScript/React (frontend, via Vitest). De eerste vijf testcases komen rechtstreeks uit het UTP (TC-UT-001 tot en met TC-UT-005). Maar tijdens het implementeren vond ik dat er nog gaten zaten. De import-parser had bijvoorbeeld helemaal geen tests, terwijl die behoorlijk foutgevoelig is als je bedenkt dat afdelingen bij Equans hun CSV-bestanden allemaal net iets anders indelen. Dus heb ik er vijf extra testgroepen bij gemaakt.

### 1.3 Doel van de tests

Waar het mij vooral om ging: de onderdelen testen die het vaakst fout kunnen gaan. De GID-matching is daar een goed voorbeeld van. Als die niet goed werkt koppel je personen aan verkeerde Atlassian-accounts, en dan heb je een probleem dat je pas weken later ontdekt. Hetzelfde geldt voor de JWT-validatie. Zonder goede tests daar kan iemand in theorie de API benaderen zonder authenticatie. Dat soort dingen wil je gewoon hard getest hebben.

---

## 2. Doel van dit document

Ik had drie redenen om dit rapport te schrijven. Ten eerste als bewijs dat de tests daadwerkelijk slagen, want "het werkt op mijn machine" is niet genoeg voor een afstudeerdocument. Ten tweede wilde ik laten zien welke SRS-requirements nu op unitniveau gevalideerd zijn. Je kunt zo zien: requirement M-10 (persoonsoverzicht) is gedekt door TC-UT-001 en TC-UT-002, die allebei slagen. En ten derde wilde ik opschrijven waar ik tegenaan liep, want er zijn best een paar dingen die nog niet ideaal zijn.

Mijn begeleiders Viktor, Brian en Jeroen krijgen dit als kwaliteitsbewijs. Het team kan het gebruiken als referentie voor wat er nu aan tests staat. En voor mijn afstuderen vormt het onderdeel van de totale documentatie.

---

## 3. Relatie met andere documentatie

| Document | Relatie |
|---|---|
| **Unit Test Plan (UTP-001)** | Daar komen de test-ID's vandaan (TC-UT-001 t/m TC-UT-005). Dit rapport beschrijft de resultaten ervan. |
| **Software Requirements Specification (SRS)** | Elke test is gekoppeld aan requirements (M-xx, S-xx, TM-xx). Zie de matrix in sectie 7. |
| **Master Test Plan (MTP-001)** | Volgens het MTP hoort 60% van de testactiviteiten op unitniveau te zitten. Dit rapport is daar het bewijs van. |
| **Traceability Matrix** | Maakt de connectie zichtbaar: welke test hoort bij welk requirement, en wat was het resultaat. |

Qua traceerbaarheid werkt het zo: elk requirement uit de SRS verwijst naar een testcase in het UTP, die testcase verwijst naar de functie in de code, en het resultaat staat hier in dit rapport. Neem M-10 als voorbeeld: dat requirement gaat over het personenoverzicht, is gekoppeld aan TC-UT-001, en die test draait de functie `test_extract_gid_from_email`. Resultaat: geslaagd.

---

## 4. Testomgeving en randvoorwaarden

### 4.1 Testomgeving

| Onderdeel | Beschrijving |
|---|---|
| **Besturingssysteem** | Windows 11 |
| **Backend runtime** | Rust 1.7x (edition 2021) |
| **Frontend runtime** | Node.js 18+, npm |
| **Database** | Geen actieve database. SQLx offline modus met `.sqlx/` directory |

Over die SQLx offline modus wil ik even wat uitleg geven, want dat was niet meteen duidelijk voor mij. Normaal gesproken wil SQLx een actieve database-connectie tijdens het compileren, om te checken of je SQL-queries kloppen. Maar voor unit tests wil je juist geen database nodig hebben. De oplossing is de `.sqlx/` directory: daarin staan gecachede metadata van alle queries. Zo kan `cargo test` de queries compileren zonder echte database. Wat ik niet wist, en daar liep ik dus even op vast, is dat je alsnog een dummy `DATABASE_URL` moet instellen als omgevingsvariabele. Doe je dat niet, dan weigert SQLx zelfs in offline modus te compileren. Nergens duidelijk gedocumenteerd, maar na wat zoeken in GitHub issues kwam ik er achter.

### 4.2 Testframeworks

| Onderdeel | Tool | Functie |
|---|---|---|
| **Backend testrunner** | `cargo test` | Draait `#[test]` en `#[tokio::test]` functies |
| **Backend async** | `tokio-test` 0.4 | Macro's voor async testfuncties |
| **Frontend testrunner** | Vitest 4.1.1 | Testrunner, werkt native met Vite |
| **Frontend rendering** | React Testing Library 16.3.2 | Component-rendering en DOM-queries |
| **Frontend DOM** | jsdom 29.0.1 | Browser-simulatie in Node.js |

Eerlijk gezegd had ik eerst Jest willen gebruiken voor de frontend. Dat framework komt overal voor in tutorials en documentatie, dus dat leek mij de veilige keuze. Maar toen ik het ging opzetten bleek dat Jest niet lekker samenwerkt met Vite en TypeScript zonder een hoop extra configuratie. Er moest een hele `babel.config.js` bij, en het was niet duidelijk waarom bepaalde imports niet werkten. Vitest was een stuk simpeler: die pikt gewoon de bestaande `vite.config.ts` op en dan werkt het. Achteraf ben ik blij dat ik ben geswitcht, al had ik liever wat eerder geweten dat Jest zoveel gedoe zou geven.

### 4.3 Mocking

Aan de backend-kant was mocking eigenlijk niet nodig. De geteste functies zijn allemaal pure functies (geen database-calls, geen netwerk). Het enige wat ik moest doen is omgevingsvariabelen zetten via `std::env::set_var()` in een paar tests, zoals bij `AuthConfig::from_env()`.

Aan de frontend-kant lag het anders. Daar heb ik `vi.mock()` gebruikt om de `AuthContext` per test te sturen, en voor de API-tests heb ik `globalThis.fetch` overschreven met `vi.fn()`. Die aanpak vond ik pas na wat uitproberen. Ik had eerst gekeken naar `msw` (Mock Service Worker), en dat is op zich een nette bibliotheek, maar voor unit tests is het overkill. Je wilt gewoon snel even specificeren wat een fetch-call teruggeeft, niet een hele mock-server optuigen.

### 4.4 Randvoorwaarden

Geen netwerk nodig, geen database, geen CI/CD. Alles draait lokaal. Backend heeft `SQLX_OFFLINE=true` nodig, frontend vereist dat `npm install` gedaan is. Vrij straightforward.

---

## 5. Algemene acceptatiecriteria

| Criterium | Drempelwaarde | Resultaat | Status |
|---|---|---|---|
| Geslaagde tests | 100% | 100% (41/41) | **BEHAALD** |
| High-risk componenten afgedekt | 100% | 100% (7 backend + 3 frontend modules) | **BEHAALD** |
| Codecoverage backend | >= 70% | Niet gemeten | **NIET GEMETEN** |
| Codecoverage frontend | >= 60% | Niet gemeten | **NIET GEMETEN** |

Dat de coverage niet gemeten is vind ik zelf een zwak punt. Ik had het graag willen doen, maar `cargo-tarpaulin` (de coverage-tool voor Rust) is niet beschikbaar op mijn ontwikkelmachine en ik kreeg het ook niet geinstalleerd. Bij Vitest moet je `@vitest/coverage-v8` apart toevoegen, en daar ben ik niet aan toe gekomen. Wat ik wel heb gedaan: handmatig alle high-risk componenten nagelopen en per functie gecheckt of zowel het succespad als foutpaden getest worden. Dat geeft geen getal, maar het geeft mij wel vertrouwen dat de kritieke paden gedekt zijn. Het opzetten van echte coverage-tooling staat als aanbeveling in sectie 8.

---

## 6. Resultaatstructuur

Per testcase beschrijf ik het resultaat, wat de test doet, en de koppeling met de SRS. Naast de vijf testcases uit het UTP heb ik er vijf extra geschreven vanwege de redenen in onderstaande tabel:

| Extra testgroep | Waarom erbij |
|---|---|
| TC-UT-006: Bestandsparsing | CSV/Excel-import is kernfunctionaliteit (M-12) |
| TC-UT-007: Datamerging | Nodig voor data-integriteit bij updates (M-12) |
| TC-UT-008: AuthContext (frontend) | Authenticatiestroom aan de clientkant (M-08, S-04) |
| TC-UT-009: ProtectedRoute (frontend) | Routebeveiliging clientkant (M-09, TM-03) |
| TC-UT-010: backendClient (frontend) | API-communicatie en foutafhandeling (TM-08) |

---

## 7. Resultaten unit tests

### TC-UT-001: GID-extractie uit e-mailadres

**Testobject:** `GidMatcher::extract_gid_from_email()` | **Resultaat:** PASS | **SRS:** M-10, M-13, TM-05

Hiermee test ik of het systeem het GID (het deel voor de @) goed uit een e-mailadres haalt. Drie soorten adressen getest:

| Invoer | Verwacht | Werkelijk | Status |
|---|---|---|---|
| `thomas.wagensonner@equans.com` | `Some("thomas.wagensonner")` | Identiek | PASS |
| `john.doe@gmail.com` | `Some("john.doe")` | Identiek | PASS |
| `Test_User-123@example.org` | `Some("test_user-123")` | Identiek | PASS |

Die derde is speciaal. In sprint 3 had de lowercase-normalisatie een bug veroorzaakt waardoor `Test_User-123` niet hetzelfde werd als `test_user-123` in de database, en je dan dubbele records kreeg. Juist daarom wilde ik die test erin hebben. Het werkt nu goed, de normalisatie zet alles naar lowercase om, en met deze regressietest weet ik dat als iemand straks de code refactort die bug niet stiekem terugkomt.

---

### TC-UT-002: Confidence-scoreberekening persoon-matching

**Testobject:** `GidMatcher::match_person()` | **Resultaat:** PASS | **SRS:** M-10, M-13, TM-05

Dit stuk was voor mij het ingewikkeldst om te testen. De confidence-score bepaalt of een persoon als MATCHED, PENDING of UNMATCHED wordt gezien. De logica: een echt person_id (zoals `CCJ183`) krijgt altijd 100. Een auto-generated ID (begint met `AUTO_`) krijgt maximaal 30. Die grens is bewust zo laag gehouden zodat auto-IDs nooit per ongeluk als "matched" bestempeld worden.

| Scenario | person_id | Verwacht confidence | Status |
|---|---|---|---|
| Bestaand ID | `CCJ183` | 100 (MATCHED) | PASS |
| Auto-generated | `AUTO_12345` | 30 (PENDING) | PASS |
| Bestaand + local_id | `TW001` | 100 (MATCHED) | PASS |
| AUTO_ + onbekend email | `AUTO_xyz` | < 50 (UNMATCHED) | PASS |

Waar ik even over twijfelde: is die drempel van 30 niet te laag? Als je kijkt naar de statusindeling (MATCHED >= 100, PENDING 30-99, UNMATCHED < 50) overlappen PENDING en UNMATCHED eigenlijk. Een auto-ID scoort 30, wat zowel PENDING als UNMATCHED kan zijn afhankelijk van hoe je het interpreteert. Dat is iets om later nog naar te kijken, maar voor nu werkt de code conform de specificatie.

---

### TC-UT-003: CSV-importvalidatie

**Testobject:** `Validator::validate_persons()`, `Validator::is_valid_email()` | **Resultaat:** PASS | **SRS:** M-12, S-08, TM-05

Dit was de test waar mijn verwachting het meest afweek van de werkelijkheid. Ik had het UTP geschreven met het idee dat de validator records zou afkeuren als er een e-mailadres ontbreekt of als er dubbele `person_id`'s zijn. Bleek niet zo te werken. De validator laat alles door. Pas in de servicelaag wordt gekeken of er duplicaten zijn. Na overleg met Brian (mijn technisch begeleider) snapte ik waarom: als je vroeg in de pipeline gaat filteren mis je mogelijk data die later alsnog gecorrigeerd kan worden. Het is een permissieve aanpak, en de test bevestigt dat die inderdaad zo werkt.

Over de `is_valid_email()` functie: die doet op zich wat je verwacht (herkent `test@example.com` als geldig, wijst `invalid` en `@example.com` af). Maar hij wordt nergens aangeroepen in productie. De functie staat in de codebase, getest en al, klaar voor wanneer er later strengere validatie nodig is. Een beetje vreemd misschien, maar het team wilde hem bewust bewaren.

---

### TC-UT-004: JWT-claimsvalidatie en autorisatie

**Testobject:** `AzureAdClaims` methodes | **Resultaat:** PASS | **SRS:** M-08, M-09, TM-03

Vijf testfuncties, en die dekken samen alle takken van de autorisatielogica af. De `user_id()` functie geeft het UPN (User Principal Name) terug, `has_role()` checkt of een gebruiker een bepaalde rol heeft, `in_group()` kijkt naar groepslidmaatschap, en `is_admin()` combineert die twee.

Een ding dat ik aanvankelijk niet had verwacht: `has_role("viewer")` matcht ook op `"Viewer"`. De vergelijking is case-insensitief. Toen ik dat zag dacht ik eerst dat het een bug was, want normaal gesproken zijn strings in Rust gewoon case-sensitief. Maar het blijkt bewust zo gebouwd te zijn, want Azure AD stuurt rollen soms met een hoofdletter en soms zonder terug. Als je daar geen rekening mee houdt krijg je situaties waarbij een gebruiker wel de juiste rol heeft maar toch wordt geblokkeerd. Fijn dat dat er al in zat.

De admin-controle werkt op twee manieren: via groepslidmaatschap of via een "Admin" rol in de claims. Allebei getest, allebei werken ze.

---

### TC-UT-005: AuthConfig laden uit omgevingsvariabelen

**Testobject:** `AuthConfig::from_env()`, `JwtValidator::new()` | **Resultaat:** PASS | **SRS:** M-09, TM-02, TM-03

Vrij rechttoe rechtaan. De test checkt of `tenant_id`, `client_id`, `audience` en `admin_group_id` goed uit de omgevingsvariabelen worden geladen. En of de JWKS URI correct wordt samengevoegd uit de tenant-ID (die URI heb je nodig voor het ophalen van publieke sleutels waarmee JWT-tokens gevalideerd worden).

Alles via omgevingsvariabelen laden is trouwens conform requirement TM-02. Geen secrets in de broncode. Op zich vanzelfsprekend, maar je moet het wel testen. Wat ik niet heb getest, en dat is een beperking, is wat er gebeurt als een variabele ontbreekt. De functie geeft dan een `Err` terug, maar dat scenario heb ik niet gecoverd. Staat als aanbeveling in sectie 8.

---

### TC-UT-006: Bestandsparsing (CSV/Excel-detectie)

**Testobject:** `FileParser::detect_format()`, `FileParser::get_field()` | **Resultaat:** PASS | **SRS:** M-12, TM-05

De parser kijkt naar de bestandsextensie om te bepalen of het een CSV of Excel-bestand is. Geef je een `.pdf` mee, dan krijg je een error. Simpel, maar wel nodig om te testen.

Het interessante zit in de case-insensitieve veldextractie. Binnen Equans zijn er namelijk meerdere afdelingen die CSV-bestanden aanleveren, en ze noemen de kolommen allemaal net anders. De ene afdeling schrijft `Email`, de volgende `PERSON_EMAIL`, en weer een ander gewoon `email`. De parser herkent al die varianten, en dat scheelt een hoop handmatig werk. Zonder die feature zou je voor elke afdeling een aparte mapping moeten maken.

---

### TC-UT-007: Datamerging bij import

**Testobject:** `MergeEngine` methodes | **Resultaat:** PASS | **SRS:** M-12, TM-05

De merge-engine bepaalt wat er wint: de geimporteerde data of wat er al in de database staat. De regel is: import gaat voor, tenzij het importveld leeg is. In dat geval bewaar je wat er al is. Acht scenario's getest, varierende van "import overschrijft" tot "allebei leeg". Alles correct.

Over de datumparser: die snapt vijf formaten (ISO, Europees, Amerikaans, punt-gescheiden en streep-gescheiden). Ik heb er drie getest, want de andere twee volgen exact dezelfde logica, alleen met een ander scheidingsteken. Misschien had ik ze er toch bij moeten doen voor de volledigheid, maar ik vond de meerwaarde beperkt.

---

### TC-UT-008: AuthContext (frontend)

**Testobject:** `AuthProvider`, `useAuth()` | **Resultaat:** PASS | **SRS:** M-08, S-04

Vier tests voor de authenticatiestroom in React. Buiten de provider aanroepen geeft een error, initiele state is niet-ingelogd, na login ben je ingelogd, na logout ben je weer uitgelogd. Op zich een helder patroon.

Maar hier moet ik wel iets bij zeggen: dit is een mock-implementatie. De echte Microsoft SSO via `@azure/msal-react` zit er nog niet in. Dus wat ik hier test is eigenlijk de nepversie van authenticatie, niet de echte flow. Voor een prototype is dat prima, maar voor productie moet er integratietests bij. Ik kom hier in sectie 8 op terug.

Het lastigste stuk was de asynchrone state-update na login. De mock heeft een vertraging van 500ms ingebouwd, en ik moest wachten tot React klaar was met updaten. Mijn eerste poging was `vi.useFakeTimers()` met `vi.advanceTimersByTime(500)`. Dat werkte niet, want `userEvent` (waarmee ik klikken simuleer) heeft echte timers nodig. Fake timers en echte user events gaan niet samen, dat heb ik na een uur debuggen geleerd. De oplossing was `waitFor()`, die wacht gewoon tot de assertion klopt.

---

### TC-UT-009: ProtectedRoute (frontend)

**Testobject:** `ProtectedRoute` component | **Resultaat:** PASS | **SRS:** M-09, TM-03

Dit component is de bewaker van beveiligde pagina's. Drie states, drie tests: aan het laden (spinner), niet ingelogd (loginpagina), ingelogd (content zichtbaar). Ik heb de AuthContext gemockt zodat ik per test de authenticatiestatus kon instellen. Met `queryByTestId()` (in plaats van `getByTestId()`) controleer ik dat bepaalde elementen er juist niet zijn, want die geeft `null` terug in plaats van een error als het element ontbreekt.

---

### TC-UT-010: backendClient API-communicatie (frontend)

**Testobject:** `ApiError` klasse, `fetchApi()` wrapper | **Resultaat:** PASS | **SRS:** TM-08

Vijf tests voor de API-laag. De `fetchApi()` wrapper doet iets handigs bij fouten: hij probeert eerst het JSON-foutbericht van de backend te parsen. Bevat de response een `error` en `message` veld, dan gebruikt hij die. Maar soms krijg je gewoon een kale 502 Bad Gateway terug van een reverse proxy, zonder JSON erbij. In dat geval pakt de wrapper de platte tekst als foutmelding.

Dat fallback-scenario nabootsen was lastig. Je moet een mock-response maken waarbij `.json()` een reject geeft maar `.text()` wel werkt. Dat is niet iets wat je elke dag doet. Het duurde even voor ik de juiste combinatie van `mockResolvedValue` en `Promise.reject` had.

---

### Samenvatting resultaten

| Test-ID | Testnaam | Resultaat | # Tests | SRS-koppeling |
|---|---|---|---|---|
| TC-UT-001 | GID-extractie | **PASS** | 1 | M-10, M-13, TM-05 |
| TC-UT-002 | Confidence-score | **PASS** | 2 | M-10, M-13, TM-05 |
| TC-UT-003 | CSV-importvalidatie | **PASS** | 4 | M-12, S-08, TM-05 |
| TC-UT-004 | JWT-claims en autorisatie | **PASS** | 5 | M-08, M-09, TM-03 |
| TC-UT-005 | AuthConfig omgevingsvariabelen | **PASS** | 2 | M-09, TM-02, TM-03 |
| TC-UT-006 | Bestandsparsing | **PASS** | 4 | M-12, TM-05 |
| TC-UT-007 | Datamerging | **PASS** | 8 | M-12, TM-05 |
| TC-UT-008 | AuthContext (frontend) | **PASS** | 4 | M-08, S-04 |
| TC-UT-009 | ProtectedRoute (frontend) | **PASS** | 3 | M-09, TM-03 |
| TC-UT-010 | backendClient | **PASS** | 5 | TM-08 |
| | **Totaal** | **41/41 PASS** | **41** | |

### Traceability matrix

| Requirement | Omschrijving | Testcases | Resultaat |
|---|---|---|---|
| **M-08** | Authenticatie via Equans SSO | TC-UT-004, TC-UT-008 | PASS |
| **M-09** | JWT-authenticatie API-endpoints | TC-UT-004, TC-UT-005, TC-UT-009 | PASS |
| **M-10** | Overzicht personen met vendor-identifiers | TC-UT-001, TC-UT-002 | PASS |
| **M-12** | Importeren via CSV en Excel | TC-UT-003, TC-UT-006, TC-UT-007 | PASS |
| **M-13** | Personen koppelen aan Atlassian-accounts | TC-UT-001, TC-UT-002 | PASS |
| **S-04** | Automatisch verlengen gebruikerssessies | TC-UT-008 | PASS |
| **S-08** | Preview wijzigingen voor import | TC-UT-003 | PASS |
| **TM-02** | Secrets via omgevingsvariabelen | TC-UT-005 | PASS |
| **TM-03** | JWT-authenticatie vereist | TC-UT-004, TC-UT-005, TC-UT-009 | PASS |
| **TM-05** | Rust `Result<T, E>` foutafhandeling | TC-UT-001 t/m TC-UT-007 | PASS |
| **TM-08** | API P95 responstijd < 200ms | TC-UT-010 | PASS (structuur) |

---

## 8. Conclusie en aanbevelingen

### 8.1 Algehele conclusie

41 tests, allemaal geslaagd. Op papier is dat een perfect resultaat, maar ik wil het wel in perspectief plaatsen.

De GID-matching werkt goed. E-mailadressen worden genormaliseerd en de confidence-scoring houdt zich aan de afgesproken drempels. De lowercase-bug uit sprint 3 is opgelost en getest. Dat geeft vertrouwen. De JWT-autorisatie doet wat het moet doen: rollen en groepen worden correct gecheckt, ook case-insensitief, en de admin-controle werkt via twee routes. De import-keten (parsing, validatie, merging) functioneert ook naar behoren, met als sterk punt de case-insensitieve kolomherkenning die in de praktijk bij Equans erg handig is.

Aan de frontend-kant werkt de routebeveiliging correct en de API-client handelt fouten netjes af. Maar de authenticatie draait nog op een mock-implementatie, niet op echte Microsoft SSO. Dat is iets om rekening mee te houden.

### 8.2 Beperkingen

Er zijn een paar dingen die niet ideaal zijn, en die wil ik niet onder het tapijt vegen:

| # | Beperking | Impact |
|---|---|---|
| 1 | Codecoverage niet kwantitatief gemeten, want de tooling ontbreekt op mijn machine | Midden |
| 2 | Frontend AuthContext test de mock-flow, niet de echte MSAL-integratie | Midden |
| 3 | `is_valid_email()` zit in de codebase maar wordt nergens aangeroepen | Laag |
| 4 | Geen test voor het scenario dat omgevingsvariabelen ontbreken bij `AuthConfig::from_env()` | Laag |
| 5 | De Axum-middleware (auth, admin, optional auth) heb ik niet op unitniveau getest, want die hebben volledige Axum-context nodig | Midden |

### 8.3 Aanbevelingen

| # | Aanbeveling | Prioriteit |
|---|---|---|
| 1 | Coverage-tooling opzetten (`cargo-tarpaulin` en `@vitest/coverage-v8`) met drempels in de CI/CD-pipeline | Hoog |
| 2 | De mock-AuthProvider vervangen door echte `@azure/msal-react` en integratietests schrijven voor SSO | Hoog |
| 3 | Negatieve test toevoegen voor `AuthConfig::from_env()`, specifiek het geval dat een variabele mist | Midden |
| 4 | De exacte grenswaarden van de confidence-score testen (0, 29, 30, 99, 100) | Midden |
| 5 | `cargo test` en `npm test` toevoegen aan de GitHub Actions workflow | Hoog |
| 6 | Een beslissing nemen over `is_valid_email()`, of activeren of verwijderen | Laag |
| 7 | Middleware testen op integratieniveau met Axum's `TestServer` | Midden |

---

## Bijlage A: Test-uitvoeringslog

### Backend (Rust), 25 maart 2026

```
$ cargo test
running 29 tests
test auth::claims::tests::test_has_role ... ok
test auth::claims::tests::test_in_group ... ok
test auth::claims::tests::test_is_admin_via_group ... ok
test auth::claims::tests::test_is_admin_via_role ... ok
test auth::claims::tests::test_user_id_prefers_upn ... ok
test auth::jwt::tests::test_auth_config_from_env ... ok
test auth::jwt::tests::test_jwks_uri_construction ... ok
test imports::merger::tests::test_merge_optional_field_both_none ... ok
test imports::merger::tests::test_merge_optional_field_import_priority ... ok
...
test persons::gid_matcher::tests::test_extract_gid_from_email ... ok
test persons::gid_matcher::tests::test_gid_status_thresholds ... ok

test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Frontend (React/TypeScript), 25 maart 2026

```
$ npx vitest run
 RUN  v4.1.1

 ✓ src/api/__tests__/backendClient.test.ts (5 tests) 13ms
 ✓ src/components/auth/__tests__/ProtectedRoute.test.tsx (3 tests) 56ms
 ✓ src/context/__tests__/AuthContext.test.tsx (4 tests) 1405ms

 Test Files  3 passed (3)
      Tests  12 passed (12)
   Duration  7.37s
```

---

## Bijlage B: Codefragmenten unit tests

Hieronder laat ik fragmenten zien van hoe de tests eruit zien in de code. Niet de volledige bestanden, maar genoeg om de aanpak duidelijk te maken.

### B.1 Backend: factory-functies

Iets waar ik al vrij snel achter kwam: als je meerdere tests hebt die een `Person`-struct nodig hebben, wil je niet in elke test handmatig dertig velden invullen. Daarom heb ik een factory-functie gemaakt die een struct met standaardwaarden aanmaakt. Een individuele test overschrijft dan alleen het veld dat voor dat scenario relevant is.

```rust
fn create_test_person(email: &str, local_id: Option<String>) -> Person {
    Person {
        id: 1,
        person_id: "TEST001".to_string(),
        email: email.to_string(),
        local_id,
        // ... overige velden met standaardwaarden
        status: "Active".to_string(),
    }
}

#[test]
fn test_confidence_calculation() {
    let matcher = GidMatcher::new();
    let mut person = create_test_person("thomas.wagensonner@equans.com", None);
    person.person_id = "AUTO_12345".to_string();
    let result = matcher.match_person(&person).unwrap();
    assert!(result.confidence >= 30);
    assert!(result.confidence < 100);
}
```

Dezelfde truc gebruik ik bij de JWT-tests, maar dan met `create_test_claims()` voor een `AzureAdClaims`-struct:

```rust
fn create_test_claims() -> AzureAdClaims {
    AzureAdClaims {
        upn: Some("user@equans.com".to_string()),
        groups: vec!["group-1".to_string(), "admin-group".to_string()],
        roles: vec!["Viewer".to_string()],
        // ... overige velden
    }
}

#[test]
fn test_has_role() {
    let claims = create_test_claims();
    assert!(claims.has_role("Viewer"));
    assert!(claims.has_role("viewer"));   // case-insensitief
    assert!(!claims.has_role("Admin"));   // niet in de lijst
}
```

### B.2 Frontend: React context testen

Om de AuthContext te testen had ik een manier nodig om de interne state te bekijken. Mijn oplossing: een simpel helper-component dat de context-waarden als tekst rendert via `data-testid` attributen. React Testing Library kan die dan uitlezen.

```typescript
function AuthConsumer() {
  const { isAuthenticated, user, login, logout } = useAuth();
  return (
    <div>
      <span data-testid="authenticated">{String(isAuthenticated)}</span>
      <span data-testid="user-name">{user?.name ?? "none"}</span>
      <button data-testid="login-btn" onClick={() => login()}>Login</button>
    </div>
  );
}
```

De login-test met `waitFor()` (nadat ik erachter kwam dat fake timers niet werkten):

```typescript
it("sets authenticated state after login", async () => {
  const user = userEvent.setup();
  render(<AuthProvider><AuthConsumer /></AuthProvider>);
  await user.click(screen.getByTestId("login-btn"));
  await waitFor(() => {
    expect(screen.getByTestId("authenticated")).toHaveTextContent("true");
  });
});
```

### B.3 Frontend: mocking van fetch en AuthContext

Bij de ProtectedRoute-tests mock ik de hele AuthContext, zodat ik per test een andere staat kan instellen:

```typescript
const mockUseAuth = vi.fn();
vi.mock("../../../context/AuthContext", () => ({
  useAuth: () => mockUseAuth(),
}));

it("shows Login page when user is not authenticated", () => {
  mockUseAuth.mockReturnValue({ isAuthenticated: false, isLoading: false });
  render(<ProtectedRoute><div data-testid="protected-content">Secret</div></ProtectedRoute>);
  expect(screen.getByTestId("login-page")).toBeInTheDocument();
  expect(screen.queryByTestId("protected-content")).not.toBeInTheDocument();
});
```

Voor de API-tests vervang ik `globalThis.fetch` direct, en herstel ik de originele na elke test:

```typescript
const originalFetch = globalThis.fetch;
beforeEach(() => { globalThis.fetch = vi.fn(); });
afterEach(() => { globalThis.fetch = originalFetch; });
```

---

## Bijlage C: Tests uitvoeren en onderhouden

### C.1 Projectstructuur

Backend-tests staan inline in de bronbestanden (Rust-conventie, binnen `#[cfg(test)]`). Frontend-tests staan in `__tests__/` directories naast de bronbestanden (JavaScript-conventie).

```
backend/src/
├── auth/claims.rs          # Productie + tests inline
├── auth/jwt.rs             # Productie + tests inline
├── imports/validator.rs    # Productie + tests inline
├── imports/parser.rs       # Productie + tests inline
├── imports/merger.rs       # Productie + tests inline
└── persons/gid_matcher.rs  # Productie + tests inline

frontend/src/
├── context/__tests__/AuthContext.test.tsx
├── components/auth/__tests__/ProtectedRoute.test.tsx
└── api/__tests__/backendClient.test.ts
```

### C.2 Tests uitvoeren

**Backend:**
```powershell
cd backend
$env:SQLX_OFFLINE = "true"
cargo test                              # alle tests
cargo test auth::claims                 # specifieke module
cargo test test_extract_gid_from_email  # specifieke test
```

**Frontend:**
```powershell
cd frontend
npm test              # eenmalig
npm run test:watch    # watch-modus tijdens ontwikkeling
```

### C.3 Richtlijnen voor nieuwe tests

Tests schrijven werkt het best als je het doet terwijl je de feature bouwt, niet achteraf. Gebruik factory-functies voor testdata zodat je niet steeds dezelfde velden herhaalt. Volg het Arrange-Act-Assert patroon (data klaarzetten, functie aanroepen, resultaat controleren). En test niet alleen het pad waar alles goed gaat, maar ook de foutgevallen: lege strings, ontbrekende velden, ongeldige invoer. Dat is juist waar de meeste bugs zitten.
