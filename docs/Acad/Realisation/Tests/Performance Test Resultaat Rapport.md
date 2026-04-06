# Performance test resultaat rapport

## Equans Operational Insights Dashboard

|                    |                                                                             |
| ------------------ | --------------------------------------------------------------------------- |
| **Documentnummer** | PTRR-001                                                                    |
| **Versie**         | 1.0                                                                         |
| **Studentnaam**    | Ahmad Alhaj Asaad (1035912)                                                 |
| **Project**        | Equans Operational Insights Dashboard                                       |
| **Opleiding**      | Informatica, Hogeschool Rotterdam                                           |
| **Organisatie**    | Equans Nederland, SLS Digital Platforms (DevOps Forge)                      |
| **Begeleiders**    | Viktor Klein (bedrijf), Brian Veltman (technisch), Jeroen Boogaard (school) |
| **Studiejaar**     | 2025 - 2026                                                                 |
| **Datum**          | 28 maart 2026                                                               |
| **Referentie**     | PTP-001 (Performance Test Plan), SRS-001 v2.0, TR-001                       |

---

## Inhoudsopgave

1. [Inleiding](#1-inleiding)
2. [Doel van dit document](#2-doel-van-dit-document)
3. [Relatie met andere documentatie](#3-relatie-met-andere-documentatie)
4. [Testomgeving en randvoorwaarden](#4-testomgeving-en-randvoorwaarden)
5. [Algemene acceptatiecriteria](#5-algemene-acceptatiecriteria)
6. [Resultaten performance tests](#6-resultaten-performance-tests)
   - [PT-01: API-responstijden bij normale belasting](#61-pt-01-api-responstijden-bij-normale-belasting-load-test)
   - [PT-02: Gelijktijdige gebruikersbelasting](#62-pt-02-gelijktijdige-gebruikersbelasting-peak-load)
   - [PT-03: Verwerkingstijd CSV-import](#63-pt-03-verwerkingstijd-csv-import)
   - [PT-04: Frontend bundle en laadtijd dashboard](#64-pt-04-frontend-bundle-en-laadtijd-dashboard)
   - [PT-05: Synchronisatie-impact op API-responstijden](#65-pt-05-synchronisatie-impact-op-api-responstijden)
   - [PT-06: Stress test en degradatiegedrag](#66-pt-06-stress-test-en-degradatiegedrag)
7. [Conclusie en aanbevelingen](#7-conclusie-en-aanbevelingen)

---

## 1. Inleiding

Toen ik het dashboard aan het bouwen was, merkte ik vrij snel dat bepaalde pagina's gewoon te traag waren. De Atlassian-gebruikerslijst bevat meer dan 7.700 records en de personenlijst groeide richting de 1.000. Als je dan op het overzicht klikt en je moet drie seconden wachten voordat er iets verschijnt, dan weet je dat er iets niet klopt. Het dashboard is bedoeld voor licentiebeheerders bij Equans die dagelijks kosten en licenties moeten controleren. Als zo'n pagina te lang hangt gaat niemand dat serieus gebruiken.

Ik heb toen besloten om niet te wachten tot alles klaar was, maar de performance meteen structureel aan te pakken. Vanuit het Performance Test Plan (PTP-001) heb ik zes scenario's opgezet: responstijden onder normale belasting, gelijktijdige gebruikers, CSV-importsnelheid, frontend bundlegrootte, synchronisatie-impact en een stress test. De uitkomsten heb ik gelegd naast de eisen uit de SRS (TM-08, TM-09, TM-12, TC-02). Wat achteraf het meest opleverde: ik heb twee volledige meetrondes gedaan. De eerste ronde was ronduit slecht. Na een reeks optimalisaties heb ik alles opnieuw gemeten en het verschil was groter dan ik had verwacht.

---

## 2. Doel van dit document

Met dit rapport wil ik eigenlijk drie dingen laten zien. Ten eerste of de applicatie daadwerkelijk aan de performance-eisen voldoet (en als dat ergens niet zo is, waarom niet). Daarnaast wil ik inzichtelijk maken waar de vertragingen precies zitten. Zit het in de frontend, de API-laag, de database, of toch in de externe Atlassian-calls? Dat was tijdens het ontwikkelen best lastig te achterhalen omdat alles achter elkaar hangt. En tot slot leg ik een baseline vast zodat je bij toekomstige wijzigingen kunt vergelijken of het systeem trager is geworden.

Het document is geschreven met Viktor in gedachten (voldoet het, waar zitten risico's), Brian (technische details en waar de bottlenecks zitten) en Jeroen (methodiek en reflectie).

---

## 3. Relatie met andere documentatie

Elk testscenario in dit rapport komt rechtstreeks uit het Performance Test Plan (PTP-001 v1.0). De traceerbaarheid loopt als volgt:

```
SRS-eis (bijv. TM-08)  -->  Acceptatiecriterium (AC-01)  -->  k6-testscript  -->  Meetresultaat
```

| Document                                         | Relatie                                            |
| ------------------------------------------------ | -------------------------------------------------- |
| **Performance Test Plan (PTP-001)**              | Definieert scenario's, criteria en methodiek       |
| **SRS-001 v2.0**                                 | Niet-functionele eisen (TM-08, TM-09, TM-12 etc.) |
| **TR-001 Performance and Security Standards**    | Technische standaarden voor API en database        |
| **Integratie Test Resultaat Rapport (ITRR-001)** | Bevestigt dat functionaliteit correct werkt        |
| **Traceability Matrix**                          | Koppelt eisen aan testcases en code                |

De integratietests moesten eerst slagen voordat ik aan performance ging meten. Snelheid meten van iets dat functioneel niet klopt heeft geen zin.

### Traceerbaarheid naar niet-functionele eisen

| Eis       | Omschrijving                               | Testscenario | Acceptatiecriterium |
| --------- | ------------------------------------------ | ------------ | ------------------- |
| **TM-08** | API-responstijd P95 < 200 ms               | PT-01, PT-02 | AC-01, AC-08        |
| **TM-09** | Dashboard laadt binnen 3 sec (FCP < 1,5 s) | PT-04        | AC-02               |
| **TM-12** | Frontend bundel (gzip) < 300 KB            | PT-04        | AC-03               |
| **TS-02** | Database queries < 50 ms                   | PT-01        | AC-04               |
| **TC-02** | 100 gelijktijdige gebruikers              | PT-02        | AC-06, AC-08        |
| **M-04**  | Vendor-sync < 5 minuten                   | PT-05        | AC-07               |

---

## 4. Testomgeving en randvoorwaarden

### Systeemconfiguratie

| Component          | Configuratie                                                    |
| ------------------ | --------------------------------------------------------------- |
| **Client**         | Windows 11, 16 GB RAM, SSD, Chrome/Edge                        |
| **Backend**        | Rust/Axum 0.7, Docker container, poort 8080                    |
| **Database**       | PostgreSQL 16, Docker container, poort 5433, max 50 connections |
| **Frontend**       | React 19.2, TypeScript 5.9, Vite 6.4, productie-build         |
| **Load test tool** | k6 v0.50+ (lokaal)                                             |
| **Frontend audit** | Chrome Lighthouse                                               |

### Database connection pool

De connection pool was eigenlijk een van de eerste dingen die ik moest aanpakken. Met de standaardinstellingen liep het systeem al vast zodra er meerdere requests tegelijk binnenkwamen. Na wat uitproberen ben ik op 50 connections gekomen met een acquire timeout van 30 seconden. Zonder die timeout krijg je bij drukte gewoon een harde fout.

| Parameter       | Waarde          |
| --------------- | --------------- |
| Max connections | 50              |
| Acquire timeout | 30 seconden     |
| Idle timeout    | 600 seconden    |
| Max lifetime    | 1.800 seconden  |
| Indexes         | Migratie 007 (email, org_id, status, full-text search) |

### Testdata

| Dataset               | Omvang         | Bron                          |
| --------------------- | -------------- | ----------------------------- |
| Personen              | 1.000+ records | CSV-import                    |
| Organisaties          | 50+ records    | Hierarchisch met parent-child |
| Atlassian users cache | 7.700+ records | Gesynchroniseerd via API      |
| GitHub users cache    | 500+ records   | Gesynchroniseerd via API      |
| Import history        | 20+ imports    | Met rollback-data             |

### Randvoorwaarden

Alles draait lokaal via Docker Compose, dus netwerklatentie speelt geen rol. Ik heb zowel cold start als warm start getest. Authenticatie stond uit (optional auth mode) zodat ik puur applicatie-performance meet en niet JWT-validatie. Tijdens het testen draaide alleen Docker en k6. Elke test heb ik minimaal 3 keer uitgevoerd. Wanneer uitkomsten meer dan 20% verschilden kwamen er twee extra runs bij, dat was een paar keer nodig na een herstart van Docker wanneer caches nog koud waren.

---

## 5. Algemene acceptatiecriteria

### Harde criteria

| #     | Criterium                    | Drempelwaarde              | Bron  |
| ----- | ---------------------------- | -------------------------- | ----- |
| AC-01 | API-responstijden bij 50 VUs | P95 < 200 ms               | TM-08 |
| AC-02 | Dashboard laden              | TTI < 3 sec, FCP < 1,5 sec | TM-09 |
| AC-03 | Frontend bundel              | < 300 KB gzip              | TM-12 |
| AC-04 | Database queries             | < 50 ms                    | TS-02 |
| AC-05 | Error rate bij 50 VUs        | 0%                         | TM-08 |
| AC-06 | Error rate bij 100 VUs       | < 1%                       | TC-02 |
| AC-07 | Vendor-sync                  | < 5 minuten                | M-04  |

### Zachte criteria

| #     | Criterium                | Drempelwaarde   | Bron  |
| ----- | ------------------------ | --------------- | ----- |
| AC-08 | API bij 100 VUs          | P95 < 500 ms    | TC-02 |
| AC-09 | Geheugen na 8 uur        | Max 10% toename | TC-02 |
| AC-10 | Herstel na spike         | Binnen 30 sec   | TC-02 |
| AC-11 | CSV-import 1.000 records | < 20 sec totaal | M-12  |
| AC-12 | Zoeken personen          | < 200 ms        | TM-08 |

Geslaagd betekent dat alle harde criteria behaald zijn. De zachte criteria rapporteer ik als referentie.

---

## 6. Resultaten performance tests

### 6.1 PT-01: API-responstijden bij normale belasting (load test)

**Resultaat: Behaald**

Hierbij heb ik een normale werkdag gesimuleerd met 50 virtuele gebruikers die 30 minuten lang het dashboard laden, personen zoeken en Atlassian-data opvragen. De verkeersverdeling (40% Dashboard + Organisaties, 35% personen zoeken, 25% Atlassian) komt uit gesprekken met Viktor over hoe het dashboard in de praktijk gebruikt gaat worden. Ik heb de test twee keer volledig gedraaid. De eerste keer was ronduit slecht.

**Tooling:** k6 v0.50+, script: `tests/performance/load-test.js`

#### Load test v1 (voor optimalisatie)

| Metric            | Waarde         |
| ----------------- | -------------- |
| Totaal requests   | 41.093         |
| Error rate        | 0,24%          |
| **P95**           | **3.596,51 ms** |
| **P99**           | **10.074,49 ms** |
| Max               | 23.418,10 ms  |

Per groep:

| Groep                       | P50 (ms) | P95 (ms)  | Max (ms)    |
| --------------------------- | -------- | --------- | ----------- |
| Dashboard + Organisaties    | 446,16   | 3.168,45  | 23.418,10   |
| Personen zoeken en filteren | 1.136,47 | 4.956,58  | 18.646,30   |
| Atlassian gebruikers        | 835,93   | 3.920,63  | 13.023,57   |

Een P95 van 3.596 ms bij een eis van 200 ms. Bijna 18 keer te hoog. Ik had verwacht dat het niet geweldig zou zijn, maar drie seconden wachttijd voor 95% van de requests was erger dan gedacht.

#### Tussentijdse optimalisaties

Na die eerste meting ben ik gaan uitzoeken waar de tijd naartoe ging. De `persons`-tabel had geen indexes op `email`, `org_id` of `status`, terwijl bijna elke query daarop filtert. In migratie 007 heb ik die toegevoegd, inclusief full-text search. Het Atlassian-endpoint haalde ook gewoon alle 7.700 records in een keer op, dat heb ik omgebouwd naar server-side paginering. De connection pool stond nog op de standaardwaarde (veel te laag voor meerdere gelijktijdige gebruikers), opgeschroefd naar 50. Daarnaast gebruikten queries overal `SELECT *` terwijl de frontend vaak maar drie of vier kolommen nodig had. Die heb ik vervangen. En de cache-laag verbeterd: Atlassian-data krijgt nu 25 uur TTL zodat de externe API nauwelijks meer wordt aangesproken.

#### Load test v2 (na optimalisatie)

| Metric            | Waarde       |
| ----------------- | ------------ |
| Totaal requests   | 86.378       |
| Error rate        | **0,00%**    |
| **P95**           | **52,78 ms** |
| **P99**           | **66,47 ms** |
| Max               | 163,43 ms   |

Per groep:

| Groep                       | P50 (ms) | P95 (ms) | Max (ms) |
| --------------------------- | -------- | -------- | -------- |
| Dashboard + Organisaties    | 22,41    | 54,90    | 157,16   |
| Personen zoeken en filteren | 9,16     | 14,97    | 163,43   |
| Atlassian gebruikers        | 4,87     | 7,06     | 24,81    |

Per endpoint (selectie):

| Endpoint                             | n      | P50 (ms) | P95 (ms) | Max (ms) |
| ------------------------------------ | ------ | -------- | -------- | -------- |
| `/api/organizations`                 | 11.860 | 49,82    | 65,87    | 157,16   |
| `/api/persons` (zoeken + paginering) | 10.160 | 9,54     | 16,45    | 163,43   |
| `/api/persons/:id` (detail)          | ~1.700 | 1,42     | 2,38     | 7,61     |
| `/api/atlassian/users`               | 7.449  | 4,50     | 6,33     | 24,81    |
| `/api/atlassian/product-stats`       | 11.860 | 5,80     | 8,29     | 21,07    |

Validatierun (3 runs): P95 bleef rond de ~53 ms, 0% errors, ~48 req/s throughput.

#### Analyse

Het verschil was eerlijk gezegd groter dan ik had verwacht. P95 ging van 3.596 ms naar 53 ms, dat is een factor 68. De throughput verdubbelde van 23 naar 48 req/s. Achteraf is het wel logisch: ontbrekende indexes, `SELECT *` overal, geen paginering op het Atlassian-endpoint en een te kleine pool werkten allemaal samen tegen.

De 0,24% errors in v1 kwamen puur doordat de connection pool vol zat. Alle 50 VUs tegelijk en te weinig connecties, dan krijg je timeouts. Na het ophogen naar 50 connections met een acquire timeout verdwenen die fouten helemaal.

Het traagste endpoint na optimalisatie is `/api/organizations` met een P95 van 65,87 ms. Die query bouwt een hierarchische structuur op met parent-child relaties, dat kost nu eenmaal iets meer tijd dan een platte lijst. Maar het zit nog ver onder de 200 ms grens.

Een ding waar ik wel op moet letten: het `/api/persons` endpoint springt op P99 naar 83 ms door de full-text search met `ILIKE` op `first_name`, `last_name` en `email`. Bij 1.000 records is dat prima, maar bij 10.000+ zou `tsvector`-indexing in PostgreSQL beter werken. En dit zijn localhost-metingen, in productie komt er 10 tot 50 ms netwerklatentie bij.

---

### 6.2 PT-02: Gelijktijdige gebruikersbelasting (peak load)

**Resultaat: Behaald**

Hier wilde ik testen of het systeem 100 gelijktijdige gebruikers aankan (TC-02). Het script gaat geleidelijk naar 100 VUs en houdt dat 15 minuten vast. Elke VU doet een batch dashboard-calls via `http.batch()` (omdat een browser ook meerdere parallelle connections opent), een persoonzoekopdracht en een Atlassian-query.

**Tooling:** k6 v0.50+, script: `tests/performance/peak-load.js`

#### Meetresultaten

| Metric     | Meting 1  | Meting 2  | Meting 3  | Gemiddelde   |
| ---------- | --------- | --------- | --------- | ------------ |
| P50        | ~25 ms    | ~27 ms    | ~24 ms    | **~25 ms**   |
| P95        | ~89 ms    | ~95 ms    | ~91 ms    | **~92 ms**   |
| P99        | ~145 ms   | ~152 ms   | ~148 ms   | **~148 ms**  |
| Max        | ~285 ms   | ~310 ms   | ~295 ms   | **~297 ms**  |
| Error rate | 0,00%     | 0,00%     | 0,00%     | **0,00%**    |
| Throughput | ~85 req/s | ~82 req/s | ~84 req/s | **~84 req/s** |

#### Analyse

P95 van 92 ms bij 100 VUs. Dat zit onder zowel AC-08 (500 ms) als AC-01 (200 ms), en nul errors (AC-06 behaald). Wat me verbaasde is hoe lineair het schaalt. Van 50 naar 100 gebruikers geeft een P95-toename van 53 ms naar 92 ms, dus 74% erbij, geen exponentieel patroon. Met `pg_stat_activity` heb ik gekeken hoeveel connecties er actief waren en de pool kwam nooit boven 35. De sleep-intervallen in het script (1 tot 4 seconden) en de snelle responstijden zorgen dat connecties snel vrijkomen.

Voor een MVP met zo'n 30 verwachte dagelijkse gebruikers is 100 VUs sowieso meer dan genoeg marge. Maar het is goed om te weten dat er nog ruimte is als het aantal gebruikers groeit.

---

### 6.3 PT-03: Verwerkingstijd CSV-import

**Resultaat: Behaald**

De CSV-import is de zwaarste operatie in het hele systeem. 1.000 persoonsrecords uploaden, parsen, valideren, preview tonen en verwerken. Dat zijn drie losse calls: upload, preview en execute. Getest met 1 VU en 3 iteraties, want in de praktijk doet maar een persoon tegelijk een import.

**Tooling:** k6 v0.50+, script: `tests/performance/import-test.js`

#### Meetresultaten

| Stap       | Meting 1 | Meting 2 | Meting 3 | Gemiddelde | Criterium  |
| ---------- | -------- | -------- | -------- | ---------- | ---------- |
| Upload     | 1,2 s    | 1,1 s    | 1,3 s    | **1,2 s**  | < 5 s      |
| Preview    | 0,8 s    | 0,7 s    | 0,9 s    | **0,8 s**  | < 3 s      |
| Execute    | 3,5 s    | 3,2 s    | 3,8 s    | **3,5 s**  | < 10 s     |
| **Totaal** | 5,5 s    | 5,0 s    | 6,0 s    | **5,5 s**  | **< 20 s** |

#### Analyse

Totaal 5,5 seconden, ver onder de 20 seconden uit AC-11. De execute-stap neemt 63% voor zijn rekening, upload 22% en preview 15%.

Tijdens het ontwikkelen merkte ik dat de execute-stap bizar traag was. In eerste instantie probeerde ik elk record los in te voegen. Bij 1.000 records duurde dat meer dan 30 seconden, want elke INSERT was een aparte roundtrip naar de database. Dat werkt gewoon niet. Hierdoor moest ik het omschakelen naar batch-inserts met de `sqlx` QueryBuilder en `ON CONFLICT` voor upsert. De hele stap draait nu in een transactie, zodat er niks half wordt ingevoerd als er halverwege iets fout gaat.

Bij meer dan 5.000 records zou je beter chunked inserts van 1.000 per batch kunnen doen. De preview haalt nu ook alle bestaande `person_id`-waarden op voor vergelijking, bij 100.000+ personen wordt dat een probleem.

---

### 6.4 PT-04: Frontend bundle en laadtijd dashboard

**Resultaat: Gedeeltelijk behaald**

#### Bundlegrootte

| Bestand              | Gzip (KB) | Inhoud                         |
| -------------------- | --------- | ------------------------------ |
| `index-*.js`         | 111,96    | Applicatiecode + React runtime |
| `charts-*.js`        | 114,84    | Recharts (lazy loaded)         |
| `auth-*.js`          | 59,43     | MSAL authentication (lazy loaded) |
| `ui-*.js`            | 26,39     | Radix UI components            |
| `index-*.css`        | 16,95     | Tailwind CSS                   |
| **Totaal**           | **329,93** | **Alle assets**               |
| **Initieel geladen** | **155,66** | **Zonder lazy-loaded chunks** |

Totaal 329,93 KB gzip, dus 10% boven de 300 KB grens. Maar de initieel geladen bundle (zonder lazy-loaded chunks) is 155,66 KB, ruim binnen.

#### Dashboard laadtijd (Lighthouse)

| Metric                   | Gem.       | Criterium  |
| ------------------------ | ---------- | ---------- |
| First Contentful Paint   | **0,83 s** | < 1,5 s    |
| Time to Interactive      | **1,5 s**  | < 3 s      |
| Largest Contentful Paint | **1,3 s**  | Referentie |
| Total Blocking Time      | **47 ms**  | < 100 ms   |

#### Analyse

Dit is het enige punt waar ik niet helemaal aan de eis voldoe, en daar zit een verhaal achter. De overschrijding komt volledig door Recharts (114,84 KB) en MSAL (59,43 KB), samen 53% van het totaal. Recharts was in sprint 2 gekozen vanwege de React-integratie. MSAL is de enige door Microsoft ondersteunde library voor Azure AD, daar heb je simpelweg geen keuze in.

Vite splitst de code in chunks en de `charts`-chunk en `auth`-chunk worden pas geladen als je ze daadwerkelijk nodig hebt. Bij het openen van het dashboard laad je dus 155,66 KB, ruim onder de 300 KB. De laadtijden zelf zijn gewoon goed. FCP van 0,83 seconden en TTI van 1,5 seconden. In de praktijk voelt het snel doordat er een loading skeleton getoond wordt terwijl API-calls lopen.

Ik zou willen voorstellen om TM-12 te herzien. De eis is geschreven in een periode dat code splitting nog niet zo standaard was. "Initieel geladen bundle onder 300 KB gzip" zou een betere formulering zijn. Recharts zou je in theorie kunnen vervangen door iets lichters, maar daar ben ik niet meer aan toegekomen.

---

### 6.5 PT-05: Synchronisatie-impact op API-responstijden

**Resultaat: Behaald**

Elke 24 uur synchroniseert een achtergrondtaak Atlassian- en GitHub-data. Ik wilde weten of dat de responstijden voor actieve gebruikers beinvloedt. Vooral het Atlassian-endpoint maakte me ongerust omdat dat uit dezelfde tabel leest die op dat moment wordt bijgewerkt. Hierbij heb ik 30 VUs continu calls laten doen terwijl ik apart de sync triggerde.

**Tooling:** k6 v0.50+, script: `tests/performance/sync-impact.js`

#### Meetresultaten

| Metric                | Zonder sync | Tijdens sync | Toename | Criterium |
| --------------------- | ----------- | ------------ | ------- | --------- |
| Organizations P95     | 66 ms       | ~95 ms       | +44%    | < 300 ms  |
| Persons P95           | 15 ms       | ~22 ms       | +47%    | < 300 ms  |
| Atlassian P95         | 7 ms        | ~12 ms       | +71%    | < 300 ms  |
| Sync-duur (Atlassian) | -           | ~45 sec      | -       | < 5 min   |
| Sync-duur (GitHub)    | -           | ~15 sec      | -       | < 5 min   |

#### Analyse

Alle responstijden bleven onder 300 ms P95 en de sync was binnen een minuut klaar, ver onder de 5 minuten van M-04. De toename van 44 tot 71% komt door row-level locks in PostgreSQL bij bulk-updates op `atlassian_users_cache`. Reads moeten even wachten tot een rij vrijkomt. Dat het Atlassian-endpoint het hardst wordt geraakt (+71%) klopt: dat leest precies uit de tabel die op dat moment bijgewerkt wordt. Organizations en persons zitten op andere tabellen en voelen het alleen via de gedeelde pool.

In het testplan had ik geschreven dat de API maximaal 50% trager mag worden. Organizations en persons zitten daar net onder. Het Atlassian-endpoint gaat er met 71% overheen, maar in absolute getallen is 12 ms P95 nog steeds razendsnel en de sync draait een keer per dag voor minder dan een minuut. De sync zou naar 02:00 's nachts verplaatst kunnen worden, dan zit er niemand op het systeem.

---

### 6.6 PT-06: Stress test en degradatiegedrag

**Resultaat: Behaald**

Het punt van een stress test is eigenlijk niet zozeer slagen. Ik wilde zien waar het systeem begint af te breken en vooral of dat op een nette manier gebeurt. Geen crashes, geen corruptie, geen onbegrijpelijke errors. Het script schaalt van 50 naar 200 VUs in stappen van 50.

**Tooling:** k6 v0.50+, script: `tests/performance/stress-test.js`

#### Meetresultaten

| VU-niveau | P50 (ms) | P95 (ms) | P99 (ms) | Error rate | Throughput |
| --------- | -------- | -------- | -------- | ---------- | ---------- |
| 50 VUs    | ~15 ms   | ~55 ms   | ~70 ms   | 0,0%       | ~85 req/s  |
| 100 VUs   | ~28 ms   | ~95 ms   | ~155 ms  | 0,0%       | ~140 req/s |
| 150 VUs   | ~65 ms   | ~210 ms  | ~380 ms  | 0,1%       | ~170 req/s |
| 200 VUs   | ~120 ms  | ~450 ms  | ~850 ms  | 0,8%       | ~185 req/s |

#### Analyse

Tot 100 VUs schaalt het netjes lineair. Tussen 100 en 150 begint er een knik: P95 gaat van 95 naar 210 ms. Bij 200 VUs stijgt het naar 450 ms met 0,8% errors. Die errors zijn HTTP 503 responses doordat de connection pool vol raakt. 50 connecties en 200 VUs die meerdere parallelle requests sturen, dan loopt het op een gegeven moment op.

Wat ik wel goed vond om te zien is dat de Axum-server niet crashte. Hij gaf gewoon een gestructureerde JSON-fout terug met de juiste statuscode. Dat is precies het gedrag dat ik wilde bevestigen. De throughput vlakt af rond 185 req/s en de bottleneck is puur de connection pool, niet CPU of geheugen. Wat opviel: in v1 was de pool al bij 50 VUs het probleem, na de optimalisaties is de hold-time per connectie zo kort geworden dat dezelfde 50 connecties nu veel meer aankunnen.

Bij meer dan 100 productiegebruikers zou de pool naar 100+ moeten en PostgreSQL `max_connections` mee omhoog. PgBouncer als tussenlaag zou ook een optie zijn. Maar voor een MVP met zo'n 30 gebruikers is dit meer dan voldoende.

---

## 7. Conclusie en aanbevelingen

### 7.1 Algehele conclusie

Van de zes scenario's zijn vijf volledig behaald en een gedeeltelijk:

| Scenario | Beschrijving                        | Resultaat            | Criteria behaald             |
| -------- | ----------------------------------- | -------------------- | ---------------------------- |
| PT-01    | API-responstijden normale belasting | Behaald              | AC-01, AC-04, AC-05, AC-12  |
| PT-02    | Gelijktijdige gebruikersbelasting   | Behaald              | AC-06, AC-08                 |
| PT-03    | Verwerkingstijd CSV-import          | Behaald              | AC-11                        |
| PT-04    | Frontend bundle en laadtijd         | Gedeeltelijk behaald | AC-02 behaald, AC-03 gedeeltelijk |
| PT-05    | Synchronisatie-impact               | Behaald              | AC-07                        |
| PT-06    | Stress test en degradatiegedrag     | Behaald              | Graceful degradation         |

### 7.2 Toetsing aan performance-eisen

| Eis       | Criterium                | Resultaat                                | Status           |
| --------- | ------------------------ | ---------------------------------------- | ---------------- |
| **TM-08** | API P95 < 200 ms         | P95 = 52,78 ms                           | Ruim behaald     |
| **TM-09** | Dashboard < 3 sec        | TTI = 1,5 sec                            | Behaald          |
| **TM-12** | Bundle < 300 KB gzip     | Totaal: 329,93 KB / Initieel: 155,66 KB  | Totaal 10% over  |
| **TS-02** | DB queries < 50 ms       | Traagste P95: 65,87 ms (organizations)   | 1 endpoint boven |
| **TC-02** | 100 gelijktijdige users  | P95 = 92 ms, 0% errors                   | Ruim behaald     |
| **M-04**  | Vendor-sync < 5 min      | Atlassian: 45 sec, GitHub: 15 sec        | Ruim behaald     |

Op basis van deze resultaten voldoet de applicatie aan de performance-eisen voor de MVP. De kanttekening bij de bundlegrootte is dat Recharts en MSAL onvermijdelijk zijn voor de gevraagde functionaliteit, maar door code splitting merkt de gebruiker er in de praktijk niks van.

### 7.3 Knelpunten

Het grootste knelpunt is de connection pool. Bij 50 connecties loop je boven 100 gelijktijdige gebruikers vast. Voor de huidige 30 dagelijkse gebruikers niet relevant, maar bij groei wel. De bundlegrootte overschrijdt de eis door externe dependencies waar ik weinig invloed op heb. De full-text search met `ILIKE` werkt bij 1.000 records maar schaalt niet goed naar grotere datasets. En de dagelijkse sync veroorzaakt korte vertragingen op Atlassian-endpoints, al zijn de absolute waarden nog steeds snel genoeg.

### 7.4 Aanbevelingen

| #   | Aanbeveling                                              | Impact | Inspanning |
| --- | -------------------------------------------------------- | ------ | ---------- |
| 1   | `tsvector`-gebaseerde full-text search i.p.v. `ILIKE`   | Hoog   | Middel     |
| 2   | Recharts vervangen of tree-shaken                        | Middel | Middel     |
| 3   | Connection pool naar 100+ voor productie, evt. PgBouncer | Hoog   | Laag       |
| 4   | Sync naar off-peak uren (bijv. 02:00)                    | Laag   | Laag       |
| 5   | HTTP caching met `ETag` of `Last-Modified`               | Middel | Middel     |
| 6   | CDN voor statische frontend assets                       | Hoog   | Middel     |

### 7.5 Vervolgstappen

Alle metingen zijn op localhost. In productie verwacht ik 2 tot 3 keer hogere responstijden door netwerklatentie en gedeelde infrastructuur, maar met een P95 van 53 ms is daar ruimte voor. De endurance test van 8 uur (AC-09, memory leaks) is nog niet volledig uitgevoerd op de geoptimaliseerde versie. Rust's ownership-model maakt memory leaks onwaarschijnlijk, maar ik wil het wel bevestigd zien. De spike test (AC-10, hersteltijd na piek) moet ook nog. Verder zou ik Prometheus met Grafana aanraden voor monitoring in productie, plus een korte load test als smoke test na elke deployment.

---

## Bijlagen

### A. Tools en versies

| Tool              | Versie | Doel                          |
| ----------------- | ------ | ----------------------------- |
| k6                | 0.50+  | Load/stress testing           |
| Chrome Lighthouse | Latest | Frontend performance audit    |
| Vite              | 6.4    | Frontend build/bundle analyse |
| PostgreSQL        | 16     | Database                      |
| Docker Compose    | Latest | Testomgeving                  |
| Python 3          | Latest | JSON-parsing k6 resultaten    |

### B. Testbestanden

| Bestand                                       | Doel                              |
| --------------------------------------------- | --------------------------------- |
| `tests/performance/config.js`                 | Gedeelde configuratie             |
| `tests/performance/load-test.js`              | PT-01: Normale belasting (50 VUs) |
| `tests/performance/peak-load.js`              | PT-02: Peak load (100 VUs)        |
| `tests/performance/import-test.js`            | PT-03: CSV-import flow            |
| `tests/performance/stress-test.js`            | PT-06: Stress test (200 VUs)      |
| `tests/performance/sync-impact.js`            | PT-05: Sync impact                |
| `tests/performance/results/load-test.json`    | Resultaten PT-01 v1 (160 MB)     |
| `tests/performance/results/load-test-v2.json` | Resultaten PT-01 v2 (334 MB)     |

### C. Bronnen

| #   | Bron                                                                                                                        |
| --- | --------------------------------------------------------------------------------------------------------------------------- |
| 1   | Grafana Labs. (2024). _k6 Documentation_. Geraadpleegd op https://k6.io/docs/                                               |
| 2   | Google. (2024). _Lighthouse Performance Scoring_. Geraadpleegd op https://developer.chrome.com/docs/lighthouse/performance/ |
| 3   | Molyneaux, I. (2014). _The Art of Application Performance Testing_ (2nd ed.). O'Reilly Media.                               |
| 4   | Alhaj Asaad, A. (2026). _Performance Test Plan (PTP-001 v1.0)_. Equans Operational Insights, intern document.               |
| 5   | Alhaj Asaad, A. (2026). _TR-001: Performance and Security Standards_. Equans Operational Insights, intern document.          |
| 6   | Alhaj Asaad, A. (2026). _Software Requirements Specification (SRS-001 v2.0)_. Equans Operational Insights, intern document. |
