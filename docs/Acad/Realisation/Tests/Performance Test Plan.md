# Performance test plan

## Equans Operational Insights Dashboard

|                 |                                                                             |
| --------------- | --------------------------------------------------------------------------- |
| **Versie**      | 1.0                                                                         |
| **Studentnaam** | Ahmad Alhaj Asaad (1035912)                                                 |
| **Project**     | Equans Operational Insights Dashboard                                       |
| **Opleiding**   | Informatica, Hogeschool Rotterdam                                           |
| **Organisatie** | Equans Nederland, SLS Digital Platforms (DevOps Forge)                      |
| **Begeleiders** | Viktor Klein (bedrijf), Brian Veltman (technisch), Jeroen Boogaard (school) |
| **Studiejaar**  | 2025 - 2026                                                                 |
| **Referentie**  | TR-001, Performance and Security Standards                                  |

---

## Inhoudsopgave

1. [Inleiding](#1-inleiding)
2. [Doel van de performance tests](#2-doel-van-de-performance-tests)
3. [Scope](#3-scope)
4. [Teststrategie en methode](#4-teststrategie-en-methode)
5. [Testomgeving](#5-testomgeving)
6. [Kritieke performance scenario's](#6-kritieke-performance-scenarios)
7. [Meetwaarden en metrics](#7-meetwaarden-en-metrics)
8. [Acceptatiecriteria](#8-acceptatiecriteria)
9. [Risico's en mitigatie](#9-risicos-en-mitigatie)
10. [Rapportage en evaluatie](#10-rapportage-en-evaluatie)
11. [Bronnen](#11-bronnen)

---

## 1. Inleiding

### 1.1 Wat is performance testing?

Met performance testing meet je hoe snel en stabiel een systeem reageert als er gebruikers mee werken. Functionele tests controleren of iets werkt, performance tests laten zien of het ook snel genoeg werkt. Dat klinkt misschien als een bijzaak, maar bij dit project merkte ik al vrij snel dat het verschil tussen een bruikbaar en een onbruikbaar dashboard echt in de laadtijden zit. Het Equans Operational Insights Dashboard is bedoeld voor licentiebeheerders en teammanagers die dagelijks hun licentie- en kostendata moeten checken. Als zo'n pagina er vijf seconden over doet om te laden, klikt niemand daar nog op.

Waar ik tegenaan liep was dat bepaalde pagina's met grotere datasets (de Atlassian-gebruikerslijst heeft 7.700+ records) al bij het eerste prototype traag aanvoelden. Dat was het moment dat ik besloot hier een apart testplan voor te schrijven in plaats van het erbij te nemen.

### 1.2 Relevante performance-aspecten

Ik heb vier aspecten geidentificeerd die het meest relevant zijn:

| Aspect             | Toelichting                                             | Relevantie                                 |
| ------------------ | ------------------------------------------------------- | ------------------------------------------ |
| **Responstijden**  | Hoe lang duurt het van klik tot resultaat               | API-calls, zoekresultaten, dashboard laden |
| **Doorvoer**       | Hoeveel requests per seconde het systeem aankan         | Meerdere gebruikers tegelijk actief        |
| **Schaalbaarheid** | Blijft het snel genoeg bij meer gebruikers of meer data | Groei richting 7.700+ Atlassian-gebruikers |
| **Stabiliteit**    | Blijft het consistent presteren over langere tijd       | Dagelijkse sync-taken, 8 uur draaitijd     |

### 1.3 Relatie met eisen uit de SRS

Elk scenario in dit testplan is terug te herleiden naar een eis uit de SRS (SRS-001 v2.0) of het technische document TR-001. Dat was voor mij de manier om te zorgen dat ik niet willekeurig dingen aan het testen ben, maar echt de eisen valideer die we hebben afgesproken.

| Eis       | Omschrijving                               | Meetbaar criterium        |
| --------- | ------------------------------------------ | ------------------------- |
| **TM-08** | API-responstijd P95 < 200 ms               | Alle backend endpoints    |
| **TM-09** | Dashboard laadt binnen 3 sec (FCP < 1,5 s) | Frontend initieel bezoek  |
| **TM-12** | Frontend bundel (gzip) < 300 KB            | Productie-build           |
| **TS-02** | Database queries < 50 ms                   | Query execution time      |
| **TC-02** | 100 gelijktijdige gebruikers               | Load testing threshold    |
| **M-04**  | Vendor-sync < 5 minuten                    | Achtergrondsynchronisatie |

---

## 2. Doel van de performance tests

Ik wil met deze tests een paar dingen weten. Allereerst: voldoet het systeem aan TM-08, TM-09 en TM-12? Dat zijn de harde eisen. Maar daarnaast wil ik ook snappen waar eventuele vertragingen vandaan komen. Zit het in de frontend? In de API-laag? In de database? Of misschien in de externe calls naar Atlassian? Tijdens het ontwikkelen was dat niet altijd makkelijk te achterhalen, omdat de hele keten achter elkaar zit.

Concreet wil ik antwoord op deze vragen:

| Vraag                                                       | Eis   |
| ----------------------------------------------------------- | ----- |
| Blijven API-endpoints onder P95 < 200 ms bij 50 gebruikers? | TM-08 |
| Laadt het dashboard binnen 3 seconden?                      | TM-09 |
| Bij hoeveel gebruikers gaat de responstijd boven 500 ms?    | TC-02 |
| Blijft het geheugen stabiel na 8 uur draaien?               | TC-02 |
| Lukt een CSV-import van 1.000+ records binnen 20 seconden?  | M-12  |

Naast het beantwoorden van deze vragen leg ik ook een baseline vast. Die baseline gebruik ik als referentie, zodat je bij toekomstige wijzigingen kunt vergelijken of de performance achteruit is gegaan.

---

## 3. Scope

### 3.1 In scope

Ik richt me op de onderdelen die de meeste impact hebben op wat de gebruiker merkt, en op de zwaarste operaties in het systeem.

**API-responstijden** die ik meet:

| Endpoint-groep   | Endpoints                                                                               | Waarom                           |
| ---------------- | --------------------------------------------------------------------------------------- | -------------------------------- |
| **Personen**     | `GET /api/persons` (gepagineerd + zoeken), `GET /api/persons/:id`                       | Meest gebruikte endpoint         |
| **Organisaties** | `GET /api/organizations`, `GET /api/organizations/:org_id/persons`                      | Dashboard en kostendoorbelasting |
| **Atlassian**    | `GET /api/atlassian/organizations/:org_id/users`, `.../groups`, `.../licenses/:product` | Grootste dataset, hoogste risico |
| **GitHub**       | `GET /api/github/users`, `GET /api/github/licenses`, `GET /api/github/copilot`          | Vendor-overzicht                 |
| **Health**       | `GET /health`                                                                           | Moet altijd snel zijn            |

**Pagina-laadtijden** van Organization Overview, Users/Persons, Product Breakdown, GitHub Vendor en Data Import. Bij elke pagina meet ik Time to Interactive en First Contentful Paint.

**Import-verwerking** was misschien wel het lastigste stuk om goed werkend te krijgen. Een CSV met 1.000 records moet geparsed, gevalideerd en vergeleken worden met de bestaande data. Dat zijn drie afzonderlijke API-calls (`upload`, `preview`, `execute`) en elke stap heeft zijn eigen performance-kenmerken. De execute-stap bleek in het begin erg traag, want elk record werd apart ingevoerd. Pas na het overschakelen op batch-inserts werd het acceptabel.

**Achtergrondprocessen** zoals de dagelijkse Atlassian- en GitHub-sync (beide moeten < 5 minuten duren) en de cache-cleanup.

**UI-responsiviteit** bij zoeken, pagineren, filteren en sorteren.

### 3.2 Buiten scope

Niet meegenomen: UI-animaties van Radix/Tailwind (externe libs, niet mijn verantwoordelijkheid), uptime van externe API's (buiten mijn controle, wel test ik de fallback), netwerklatentie van derden, productie-infrastructuur (draait nog alleen in Docker Compose), de JFrog-integratie (nog niet af) en stress testing tot crash (niet realistisch in een lokale omgeving, al meet ik wel de degradatie).

---

## 4. Teststrategie en methode

### 4.1 Testtypen

Op basis van Molyneaux (2014) heb ik vier testtypen geselecteerd. Eerlijk gezegd twijfelde ik of spike testing de moeite waard was voor een lokale Docker-setup, maar het scenario "maandagochtend, iedereen opent tegelijk het dashboard" is wel realistisch bij Equans, dus heb ik het er toch bij gezet.

| Type                  | Wat ik doe                                                                            | Prioriteit |
| --------------------- | ------------------------------------------------------------------------------------- | ---------- |
| **Load testing**      | 50 gebruikers, normaal werkdaggebruik: dashboard laden, zoeken, organisaties bekijken | Hoog       |
| **Stress testing**    | Opschalen van 50 naar 200 users, kijken waar het misgaat                              | Middel     |
| **Endurance testing** | 30 gebruikers, 8 uur lang, memory leaks opsporen                                      | Middel     |
| **Spike testing**     | Van 10 naar 150 en weer terug, hersteltijd meten                                      | Laag       |

### 4.2 Meetwijze

Als tooling gebruik ik k6 van Grafana Labs voor de backend (responstijden, throughput, error rates), Chrome Lighthouse voor de frontend (FCP, LCP, TTI) en PostgreSQL `EXPLAIN ANALYZE` voor de query-performance. Dat zijn de geautomatiseerde metingen.

Handmatig doe ik er ook wat bij. Bij de importflow bleek het bijvoorbeeld nuttig om gewoon de browser open te hebben en mee te kijken, want de perceptie van snelheid is soms heel anders dan wat de metriek zegt. Een loading spinner die 3 seconden draait voelt prima, maar 3 seconden naar een wit scherm staren voelt als een eeuwigheid. Daarnaast gebruik ik Rust `tracing` logs om de server-side timing per onderdeel te bekijken en het DevTools Network tab voor waterfall-analyses.

Elke test voer ik minimaal 3 keer uit. Als er meer dan 20% verschil zit tussen de runs, doe ik er nog twee extra. Dat was een paar keer nodig toen Docker net was herstart en de caches koud waren.

### 4.3 Randvoorwaarden

Alles draait op localhost via Docker Compose, dus netwerklatentie speelt geen rol. Ik test zowel cold start (net opgestart) als warm start (cache gevuld). De database is geseeded met realistische data: 1.000+ personen, 50 organisaties, 7.700 Atlassian-gebruikers. Authenticatie staat uit voor de load tests (optional auth mode), want ik wil de applicatie-performance meten, niet de JWT-validatie. Tijdens het testen draait er niks anders op de machine dan Docker en k6.

---

## 5. Testomgeving

### 5.1 Systeemconfiguratie

| Component          | Configuratie                                                       |
| ------------------ | ------------------------------------------------------------------ |
| **Client**         | Windows 11, 16 GB RAM, SSD, Chrome/Edge                            |
| **Backend**        | Rust/Axum 0.7, Docker container, poort 8080                        |
| **Database**       | PostgreSQL 16, Docker container, poort 5433, max 50 connections    |
| **Frontend**       | React 19.2, TypeScript 5.9, Vite 6.4, poort 3000 (productie-build) |
| **Load test tool** | k6 v0.50+ (lokaal)                                                 |

### 5.2 Docker Compose

```yaml
services:
  db:       PostgreSQL 16 (5433:5432), user: equans, database: equans_oi
  backend:  Rust Axum (8080:8080), max_connections: 50, idle_timeout: 600s
  frontend: React/Vite (3000:3000), production build
```

### 5.3 Database

De connection pool had ik eerst op de standaardwaarde staan. Daar liep ik mee vast bij meerdere gelijktijdige requests, omdat er gewoon te weinig connecties beschikbaar waren. Na wat proberen ben ik op 50 connections uitgekomen met een acquire timeout van 30 seconden. De idle timeout staat op 600 seconden en max lifetime op 1.800 seconden. Alle indexes uit migratie 007 zijn actief.

### 5.4 Testdata

| Dataset               | Omvang         | Bron                          |
| --------------------- | -------------- | ----------------------------- |
| Personen              | 1.000+ records | CSV-import                    |
| Organisaties          | 50+ records    | Hierarchisch met parent-child |
| Atlassian users cache | 7.700+ records | Gesynchroniseerd              |
| GitHub users cache    | 500+ records   | Gesynchroniseerd              |
| Import history        | 20+ imports    | Met rollback-data             |

---

## 6. Kritieke performance scenario's

### Scenario P1: Dashboard laden

Een licentiebeheerder opent het Organization Overview na inloggen. Het dashboard laadt statistieken en Recharts-grafieken. Dit is verreweg de meest uitgevoerde actie (UC-01). Wat mij opviel: de grafieken werden pas zichtbaar nadat alle API-calls klaar waren. Visueel voelde dat trager dan het feitelijk was, omdat je eerst een leeg scherm ziet.

**Meetpunt**: TTI van het dashboard. **Criterium**: TTI < 3 sec, FCP < 1,5 sec (TM-09). **Methode**: Lighthouse audit, 3x herhaald.

### Scenario P2: Personen zoeken

Een teammanager zoekt op naam of e-mail in de personenlijst (gepagineerd, 25 per pagina). Gecombineerd met filters op organisatie en status is dit een van de zwaardere queries. De full-text search index uit migratie 007 maakte hier een groot verschil, zonder die index zat je al snel boven de 400 ms.

**Meetpunt**: responstijd `GET /api/persons?search=<term>&page=1&per_page=25`. **Criterium**: P95 < 200 ms (TM-08). **Methode**: k6, 50 VUs, 30 minuten.

### Scenario P3: CSV-import (upload tot execute)

Een IT-beheerder importeert 1.000 persoonsrecords via CSV. Het systeem parseert, valideert, toont een preview en voert de import uit in een transactie. Zoals ik al noemde was de execute-stap aanvankelijk traag door individuele inserts. Na de overstap naar batch-inserts was het een stuk beter, maar ik wil bevestigen dat het onder de 20 seconden totaal blijft.

**Meetpunt**: end-to-end doorlooptijd van `upload` + `preview` + `execute`. **Criterium**: totaal < 20 sec. **Methode**: handmatig 3x + `tracing` logs.

### Scenario P4: Atlassian-gebruikerslijst (grote dataset)

De Atlassian-pagina toont 7.700+ gecachede gebruikers uit `atlassian_users_cache`, gepagineerd. Dit is veruit de grootste dataset. Hier moest de paginering echt op de database gebeuren. In een eerdere versie haalde de frontend alles op en pagineerde client-side, dat was direct onwerkbaar.

**Meetpunt**: responstijd `GET /api/atlassian/organizations/:org_id/users?page=1&per_page=25`. **Criterium**: P95 < 200 ms (TM-08). **Methode**: k6, 50 VUs.

### Scenario P5: Peak load (100 gebruikers)

Simulatie van 100 medewerkers die tegelijk het dashboard openen. Mijn verwachting is dat de connection pool (max 50) hier de bottleneck wordt. Elke sessie genereert meerdere parallelle calls.

**Meetpunt**: P95 over alle endpoints, error rate, throughput. **Criterium**: P95 < 500 ms, errors < 1% (TC-02). **Methode**: k6, ramp-up naar 100 VUs, 15 min sustained.

### Scenario P6: Dagelijkse sync

De achtergrondtaak synchroniseert Atlassian-gebruikers, groepen en licenties, en tegelijk de GitHub Enterprise data. Tijdens het testen bleek dat bulk-updates van `atlassian_users_cache` kort locks veroorzaakten, waardoor parallelle reads even moesten wachten. Dat is acceptabel zolang het binnen de marge blijft.

**Meetpunt**: sync-duur + impact op API-responstijden. **Criterium**: sync < 5 min, API max 50% trager tijdens sync. **Methode**: handmatige sync-trigger + k6 gelijktijdig.

---

## 7. Meetwaarden en metrics

| Metric                           | Eenheid | Bron                  |
| -------------------------------- | ------- | --------------------- |
| Response Time P50, P95, P99, max | ms      | k6                    |
| End-to-End Duration              | s       | Handmatig + `tracing` |
| FCP, TTI, LCP, TBT               | ms      | Lighthouse            |
| Throughput                       | req/s   | k6                    |
| Error Rate                       | %       | k6                    |
| Database Query Time              | ms      | `EXPLAIN ANALYZE`     |
| CPU / Memory Usage               | % / MB  | Docker stats          |
| Database Connections             | count   | `pg_stat_activity`    |
| Bundle Size                      | KB      | Vite build            |

---

## 8. Acceptatiecriteria

### 8.1 Harde criteria

| #     | Criterium                    | Drempelwaarde              |
| ----- | ---------------------------- | -------------------------- |
| AC-01 | API-responstijden bij 50 VUs | P95 < 200 ms               |
| AC-02 | Dashboard laden              | TTI < 3 sec, FCP < 1,5 sec |
| AC-03 | Frontend bundel              | < 300 KB gzip              |
| AC-04 | Database queries             | < 50 ms                    |
| AC-05 | Error rate bij 50 VUs        | 0%                         |
| AC-06 | Error rate bij 100 VUs       | < 1%                       |
| AC-07 | Vendor-sync                  | < 5 minuten                |

### 8.2 Zachte criteria

| #     | Criterium                | Drempelwaarde   |
| ----- | ------------------------ | --------------- |
| AC-08 | API bij 100 VUs          | P95 < 500 ms    |
| AC-09 | Geheugen na 8 uur        | Max 10% toename |
| AC-10 | Herstel na spike         | Binnen 30 sec   |
| AC-11 | CSV-import 1.000 records | < 20 sec totaal |
| AC-12 | Zoeken personen          | < 200 ms        |

### 8.3 Gebruikerservaring

Acties langer dan 500 ms moeten een loading-indicator tonen. De UI mag niet bevriezen tijdens imports of syncs (main thread max 100 ms geblokkeerd). Fouten moeten als begrijpelijke melding verschijnen via Sonner, niet als stacktrace.

---

## 9. Risico's en mitigatie

| #    | Risico                                    | Kans   | Wat ik eraan doe                                                     |
| ---- | ----------------------------------------- | ------ | -------------------------------------------------------------------- |
| R-01 | Productie is trager dan localhost         | Middel | Resultaten rapporteren als localhost-baseline, marge van 2x inbouwen |
| R-02 | Externe API-latentie bij Atlassian/GitHub | Hoog   | Cache TTL van 25 uur, async sync, fallback op cached data            |
| R-03 | Grote datasets drukken op performance     | Middel | Paginering overal, database-indexes (migratie 007), geen `SELECT *`  |
| R-04 | Connection pool raakt vol                 | Laag   | Pool van 50, acquire timeout 30s, monitoring via `pg_stat_activity`  |
| R-05 | Import blokkeert andere requests          | Middel | Async verwerking via tokio, batch-inserts, korte transacties         |
| R-06 | Memory leaks na lang draaien              | Laag   | Rust ownership-model, endurance test van 8 uur, Docker memory limit  |
| R-07 | Localhost-metingen niet representatief    | Hoog   | Expliciet documenteren, marge inbouwen, productie-test aanbevelen    |

Een ding waar ik wel mee zit: risico R-07 is eigenlijk het grootste probleem. Alles draait lokaal, dus de metingen zullen er in productie ongetwijfeld anders uitzien. Ik kan daar niet zoveel aan doen behalve het eerlijk opschrijven en een veiligheidsmarge inbouwen.

---

## 10. Rapportage en evaluatie

### 10.1 Resultaten vastleggen

De testresultaten sla ik op als k6 JSON (ruwe data) en k6 HTML report (visueel overzicht), Lighthouse rapporten voor de frontend, screenshots van DevTools en `EXPLAIN ANALYZE` output voor de queries. De reden dat ik meerdere formaten gebruik is dat Viktor vooral een samenvatting wil zien, terwijl Brian de ruwe data nodig heeft.

### 10.2 Wat ik oplever

Na de tests lever ik een resultaatrapport op per scenario (P1 t/m P6) met pass/fail per criterium. Bij scenario's die niet slagen, schrijf ik een analyse met mogelijke oorzaken en verbetervoorstellen. Daarnaast leg ik de baseline vast voor toekomstige regressietests.

### 10.3 Presentatie

| Stakeholder               | Vorm                       | Focus                       |
| ------------------------- | -------------------------- | --------------------------- |
| Viktor Klein (Business)   | Management-samenvatting    | Pass/fail, risico's         |
| Brian Veltman (Technisch) | Technische analyse         | Bottlenecks, optimalisaties |
| Jeroen Boogaard (School)  | Academische verantwoording | Methodiek, reflectie        |

---

## 11. Bronnen

| #   | Bron                                                                                                                                           |
| --- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | Grafana Labs. (2024). _k6 Documentation_. Geraadpleegd op https://k6.io/docs/                                                                  |
| 2   | Google. (2024). _Lighthouse Performance Scoring_. Geraadpleegd op https://developer.chrome.com/docs/lighthouse/performance/performance-scoring |
| 3   | Molyneaux, I. (2014). _The Art of Application Performance Testing_ (2nd ed.). O'Reilly Media.                                                  |
| 4   | Alhaj Asaad, A. (2026). _TR-001: Performance and Security Standards_. Equans Operational Insights, intern document.                            |
| 5   | Alhaj Asaad, A. (2026). _Software Requirements Specification (SRS-001 v2.0)_. Equans Operational Insights, intern document.                    |
