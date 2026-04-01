**Master Test Plan**

**Equans Operational Insights Dashboard**

- Studentnaam: Ahmad Alhaj Asaad (1035912)
- Project: Equans Operational Insights Dashboard
- Opleiding: Informatica - Hogeschool Rotterdam
- Organisatie: Equans Nederland - SLS Digital Platforms (DevOps Forge)
- Begeleiders: Viktor Klein (bedrijf), Brian Veltman (technisch), Jeroen Boogaard (school)
- Studiejaar: 2025 - 2026

Inhoudsopgave

[**1\. Inleiding** 4](#_Toc224417266)

[**1.1** **Achtergrond van het project** 4](#_Toc224417267)

[**1.2** **Doel van dit document** 4](#_Toc224417268)

[**2\. Testscope** 5](#_Toc224417269)

[**2.1 In scope** 5](#_Toc224417270)

[**2.2 Out of scope** 6](#_Toc224417271)

[**3\. Teststrategie** 7](#_Toc224417272)

[**3.1 Risk-based testing** 7](#_Toc224417273)

[**3.2 Shift-left testing** 7](#_Toc224417274)

[**3.3 Testpyramide** 7](#_Toc224417275)

[**4\. Testsoorten** 8](#_Toc224417276)

[**4.1 Unit testing** 8](#_Toc224417277)

[**4.2 Integratietesting** 8](#_Toc224417278)

[**4.3 Performance testing** 8](#_Toc224417280)

[**4.4 Security testing** 9](#_Toc224417281)

[**4.5 Usability testing** 9](#_Toc224417282)

[**5\. Testomgeving** 10](#_Toc224417283)

[**6\. Entry- en exitcriteria** 10](#_Toc224417284)

[**7\. Deliverables** 12](#_Toc224417288)

[**8\. Kritische reflectie en leerpunten** 13](#_Toc224417289)

[**8.1 Beperkingen van de testaanpak** 13](#_Toc224417290)

[**8.2 Evaluatie van testactiviteiten** 13](#_Toc224417291)

[**8.3 Lessen voor toekomstige projecten** 13](#_Toc224417292)

[**9\. Conclusie** 14](#_Toc224417293)

[**9.1 Realisatie** 14](#_Toc224417294)

[**9.2 Kwaliteitsvalidatie** 14](#_Toc224417295)

[**9.3 Beperkingen** 14](#_Toc224417296)

[**9.4 Projectbijdrage** 14](#_Toc224417297)

[**9.5 Eindoordeel** 14](#_Toc224417298)

[**10\. Referenties** 15](#_Toc224417299)

**Samenvatting**

Equans had geen goed beeld van welke softwarelicenties daadwerkelijk werden gebruikt en wat die eigenlijk kostten. Dat was het uitgangspunt van dit project. Het Operational Insights Dashboard haalt data op uit Atlassian Cloud en GitHub Enterprise Cloud, verwerkt die in een Rust-backend met Axum, en toont alles in een React 19 frontend. PostgreSQL 16 slaat de gegevens op.

Om de kwaliteit van dat systeem te borgen heb ik dit Master Test Plan geschreven. De kern van de aanpak draait om risk-based testing (Olsen et al., 2021). Wat dat in de praktijk betekent: de authenticatielaag en de licentieberekeningen kregen de meeste aandacht, want als daar iets misgaat heeft dat de grootste gevolgen. Fouten in een dashboard-label zijn vervelend, maar een fout in `calculate_utilization` betekent dat Equans verkeerde financiele beslissingen neemt.

Ik heb gewerkt met een shift-left aanpak, dus tests schrijven terwijl je bouwt, niet achteraf. Qua verdeling volgt het de testpyramide van Myers et al. (2012): veel unit tests onderaan en integratietests in het midden. Dat klinkt als een schoolboekje, maar het werkt wel. De snelle feedback van `cargo test` (seconden, niet minuten) maakte het haalbaar om continu te testen.

De testsoorten lopen uiteen. Unit tests op de Rust-logica en React-componenten, integratietests voor de database- en API-koppeling, security tests op basis van OWASP Top 10, loadtests met k6, en usability tests waarbij Equans-medewerkers hardop nadachten terwijl ze het dashboard gebruikten. Dat laatste leverde trouwens verrassende inzichten op, want flows die ik als ontwikkelaar logisch vond, bleken voor eindgebruikers soms niet intuïtief.

Een eerlijk punt: ik was solo-ontwikkelaar op dit project. Dat heeft gevolgen voor de onafhankelijkheid van het testproces, want je test je eigen code. Geautomatiseerde tests, code reviews door Brian en acceptatiesessies met stakeholders hebben dat deels gecompenseerd, maar het blijft een beperking.

**1\. Inleiding**

- 1.  **Achtergrond van het project**

Equans werkt met Atlassian Cloud Enterprise en GitHub Enterprise Cloud. Daar draaien de ontwikkelteams dagelijks op voor softwareontwikkeling en projectbeheer. Maar het gekke was dat er eigenlijk niemand goed kon zien hoeveel licenties er precies in gebruik waren en wat dat kostte. Er was geen centraal overzicht.

Dat klinkt misschien als een klein probleem, maar het is het niet. Zonder dat overzicht weet je niet welke accounts inactief zijn, of er ergens teams zitten met te veel licenties, of dat een business unit eigenlijk meer betaalt dan nodig. Bij een organisatie als Equans lopen die kosten snel op.

Het Operational Insights Dashboard is gebouwd om hier iets aan te doen. Het haalt via API-integraties data op uit Atlassian en GitHub, verwerkt die in de Rust-backend, en presenteert alles in een centraal dashboard. In de database zit bijvoorbeeld de tabel `persons` met velden als `person_id`, `email` en `org_id`. Daarmee koppel je gebruikers aan hun organisatorische eenheid en krijg je inzicht per team.

- 1.  **Doel van dit document**

Dit Master Test Plan legt vast hoe ik het testen van het dashboard heb aangepakt. Welke onderdelen zitten in scope, welke testmethoden gebruik ik, en wanneer beschouw ik het testen als klaar.

In de praktijk gebruik ik dit plan bij elke sprint om te bepalen wat er nog getest moet worden. Het dient ook als verantwoording voor mijn afstuderen, en het geeft de mensen bij Equans inzicht in hoe de kwaliteit wordt bewaakt.

De structuur volgt de IEEE 829-standaard voor testdocumentatie (Client Challenge, z.d.). Ik heb voor die standaard gekozen omdat die een helder framework biedt, niet omdat het verplicht was. Het helpt om niks te vergeten als je alles in je eentje doet.

**2\. Testscope**

**2.1 In scope**

Alles wat direct invloed heeft op de werking van het dashboard zit in de testscope. Dat is vrij breed, dus ik loop het even langs.

De backend is het grootste stuk. Alle REST-endpoints worden getest, onder andere `/api/v1/licenses`, `/persons`, `/orgs` en `/github`. Die halen gebruikersdata, licentie-informatie en organisatiegegevens op. Daarnaast zit er flink wat businesslogica in de backend: licentieanalyse via `calculate_utilization`, de chargeback-berekening, CSV-importverwerking. Ik merkte al vrij snel dat hier de meeste edge cases zaten. Wat doe je als een gebruiker in drie organisaties tegelijk actief is? Of als een CSV-bestand lege rijen bevat? Dat soort dingen.

De frontend test ik ook, maar minder diep. Het gaat vooral om of componenten als `LicenseDashboard`, `PersonTable` en `OrgFilter` de juiste data tonen, en of filters en grafieken doen wat ze moeten doen.

De integratie met externe API's (Atlassian Cloud Admin en GitHub Enterprise) was een verhaal apart. Die API's zijn niet altijd beschikbaar en soms traag. Niet ideaal, maar de enige werkbare optie.

Beveiligingsaspecten als JWT-validatie en SSO via Microsoft Entra ID, performance (P95 responstijd onder 200ms), en bruikbaarheid zijn ook meegenomen.

|                          |                                                                                |                              |
| ------------------------ | ------------------------------------------------------------------------------ | ---------------------------- |
| Categorie                | Concreet onderdeel                                                             | Testsoort                    |
| Backend (API)            | Alle REST-endpoints (`/api/v1/licenses`, `/persons`, `/orgs`, `/github`, etc.) | Unit + Integratietest        |
| Backend (business logic) | Licentieberekening, chargeback-logica, CSV-importverwerking                    | Unit tests (Rust `#[test]`)  |
| Backend (auth)           | JWT-validatie, SSO-integratie (Microsoft Entra ID)                             | Security tests               |
| Backend (jobs)           | Dagelijkse synchronisatietaak, rate-limit handling                             | Integratietest               |
| Frontend (componenten)   | Dashboard-widgets, filtertabellen, persoons- en organisatiepagina's            | Component tests (Jest/RTL)   |
| Frontend (flows)         | Login-flow, CSV-uploadflow, handmatige refresh                                 | Component tests (Jest/RTL)   |
| Database                 | Migratiebestanden (001-009), query-performance                                 | Integratietest + performance |
| Integraties              | Atlassian Admin API, GitHub Enterprise API                                     | Integratietest (mock/stub)   |
| Security                 | OWASP Top 10 (A01, A02, A07), AVG-conformiteit                                 | Security tests               |
| Performance              | API P95 < 200ms, dashboard load < 3s, 100 gelijktijdige gebruikers             | Load testing (k6)            |
| Usability                | Kernflows voor licentiebeheerders/finance                                      | Usability test               |

**2.2 Out of scope**

Je kunt niet alles testen. Zeker niet als solo-ontwikkelaar met een beperkt tijdbudget.

Penetratietesting heb ik niet gedaan. Dat vereist specialistische tooling en kennis die ik niet heb, en het past niet in de planning. Ik heb in plaats daarvan gerichte security tests gedaan op de OWASP Top 10, wat een stuk pragmatischer is. De productie-infrastructuur heb ik ook niet getest, dat doet het ops-team van Equans.

Uitbreidingen als de JFrog Artifactory-koppeling en Trello-integratie zitten niet in de MVP, dus die heb ik overgeslagen. Hetzelfde geldt voor real-time streaming (de architectuur is batch-based) en SCIM-integratie. En hoewel ik weet dat je eigenlijk op meerdere browsers moet testen, heb ik alleen Microsoft Edge gedaan. Chrome is secundair bekeken, maar echt grondig cross-browser testing zat er niet in.

|                                   |                                                                                                |
| --------------------------------- | ---------------------------------------------------------------------------------------------- |
| Buiten scope                      | Reden                                                                                          |
| JFrog Artifactory API-integratie  | Niet opgenomen in MVP (Won't Have W-01 scope); API beschikbaarheid niet gegarandeerd           |
| Volledige penetratietest          | Vereist gespecialiseerde tooling en externe expertise; buiten tijdbudget (Myers et al., 2012)  |
| Infrastructuurtest (Docker/infra) | Verantwoordelijkheid van Equans ops-team; buiten projectscope                                  |
| Trello-integratie                 | Could Have (C-05); niet geimplementeerd in huidige sprintplanning                              |
| Real-time streaming               | Won't Have (W-02); architectuur is batch-based                                                 |
| SCIM-integratie                   | Won't Have (W-05)                                                                              |
| E2E testing                       | Tijdsbeperking solo-project; frontend flows worden op componentniveau getest in plaats van E2E |
| Cross-browser testing             | Tijdsbeperking solo-project; Microsoft Edge als primaire browser getest                        |

**3\. Teststrategie**

**3.1 Risk-based testing**

De teststrategie komt neer op: test het hardst waar de risico's het grootst zijn. Dat is wat het ISTQB Foundation Level syllabus risk-based testing noemt (Olsen et al., 2021). Je kijkt naar twee dingen, hoe groot is de kans dat er iets fout gaat, en als het fout gaat, hoe erg is dat dan.

Ik heb daar best lang over nagedacht. Want wat weegt zwaarder: de authenticatielaag of de licentieberekening? Uiteindelijk kregen ze allebei de hoogste prioriteit, maar om verschillende redenen. Bij authenticatie (JWT-validatie via Microsoft Entra ID) gaat het om security. Als die faalt kan iemand bij data die niet voor hen bedoeld is. Bij de licentieberekening (`calculate_utilization` en de chargeback-module) gaat het om geld. Equans wil betrouwbare cijfers, anders heeft het hele dashboard geen waarde.

CSS-styling en het uitlijnen van labels kregen de laagste prioriteit. Logisch, want een verkeerd uitgelijnd label is irritant maar niet gevaarlijk.

**3.2 Shift-left testing**

Shift-left testing betekent kort gezegd: test zo vroeg mogelijk, niet achteraf. Myers et al. (2012) beschrijven hoe de kosten van bugfixes oplopen naarmate je ze later vindt. Een bug die je tijdens het coderen oppikt kost minuten. Dezelfde bug die pas bij acceptatie opduikt, kan een sprint vertragen.

Wat ik deed: bij elke feature direct tests erbij schrijven. In Rust gaat dat met `#[test]` macro's en `cargo test`. Voordat code gemerged werd keek Brian (de technisch begeleider) er ook nog naar. Die combinatie werkte goed. De meeste fouten kwamen al boven in dezelfde sprint waarin de code geschreven werd.

Eerlijk gezegd vond ik het in het begin lastig om tests te schrijven voordat de feature helemaal af was. Je weet nog niet precies hoe de interface eruit gaat zien, dus je tests veranderen mee. Na een paar sprints went dat, en dan merk je dat het je juist sneller maakt omdat je eerder fouten vangt.

**3.3 Testpyramide**

De testpyramide (Myers et al., 2012) is een bekend model en ik heb het toegepast, maar dan zonder de bovenste laag. Onderaan zitten de unit tests, dat is het grootste deel. Die testen individuele functies en zijn snel: `cargo test` draait in seconden. In het midden zitten integratietests, die de samenwerking tussen componenten testen. Die duren langer omdat ze een testdatabase nodig hebben. E2E tests heb ik bewust niet opgepakt. Die zijn traag, broos en kosten als solo-ontwikkelaar meer tijd aan onderhoud dan ze opleveren.

Waarom die verdeling? Unit tests geven de snelste feedback. Als er iets kapot gaat na een wijziging, weet je binnen seconden precies welke functie het probleem veroorzaakt. Integratietests zijn nuttig maar kosten meer setup-tijd. Frontend flows zoals de login en CSV-upload test ik op componentniveau met Jest en React Testing Library, niet via een volledige E2E-aanpak. Als solo-ontwikkelaar was de afweging vrij helder: E2E tests kosten meer tijd aan onderhoud dan ze opleveren (Olsen et al., 2021).

**4\. Testsoorten**

**4.1 Unit testing**

Unit tests isoleren een stukje code en controleren of het doet wat je verwacht (Myers et al., 2012). Ik heb ze ingezet op zowel de Rust-backend als de React-frontend.

|                |                                                                                                                       |
| -------------- | --------------------------------------------------------------------------------------------------------------------- |
| Aspect         | Invulling                                                                                                             |
| Framework      | Rust ingebouwde `#[test]` macro's, `cargo test`, `cargo tarpaulin` voor coverage                                      |
| Framework FE   | Jest + React Testing Library                                                                                          |
| Scope backend  | Businesslogica: licentieberekening (`calculate_utilization`), CSV-parsing, person-matching logica, rate-limit backoff |
| Scope frontend | React-componenten: `LicenseDashboard`, `PersonTable`, `OrgFilter`, `CsvUpload`                                        |
| Coverdoelen    | Backend: min. 70%, doel 85% - Frontend: min. 60%, doel 75%                                                            |
| Techniek       | White-box (beslissingsafdekking voor `Result<T,E>` branches in Rust) + black-box equivalentieklassen                  |

Iets wat me opviel bij het testen in Rust: het type system vangt al veel fouten af voordat je überhaupt een test draait. Een functie die `Result<T, E>` teruggeeft dwingt je om na te denken over wat er fout kan gaan. Maar dat wil niet zeggen dat je geen tests meer nodig hebt. Het verschuift alleen wat je test. Ik testte vooral edge cases, zoals lege datasets, onverwachte velden in API-responses, en situaties waarin de CSV-parser halve rijen krijgt. Dat zijn dingen waar het type system je niet voor behoedt.

De frontend tests waren een ander verhaal. React Testing Library werkt fijn, maar het testen van componenten die afhankelijk zijn van API-data (zoals `LicenseDashboard`) vereist dat je die data mockt. Dat kostte meer opzettijd dan ik had verwacht.

**4.2 Integratietesting**

Integratietests gaan over samenwerking tussen onderdelen. Werkt de Rust-service correct met PostgreSQL? Doet de JWT-middleware wat het moet doen als er een echt request binnenkomt? Dat soort vragen.

|               |                                                                                                                                     |
| ------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Aspect        | Invulling                                                                                                                           |
| Aanpak        | Bottom-up integratietesting (eerst database en Rust service, dan service en API)                                                    |
| Scope         | Atlassian API en cache-laag, GitHub API en sync-job, PostgreSQL en migratiescripts, JWT-middleware en endpoints                     |
| Externe API's | Gebruik van mock servers (Wiremock of Rust mockito) voor Atlassian/GitHub API's, geen afhankelijkheid van live third-party services |
| Testdata      | Seed-data via SQL-fixtures; representatieve datasets van circa 500 gebruikers                                                       |

Ik ben bottom-up begonnen: eerst de database-laag testen met de Rust-service, dan pas de HTTP-endpoints erbij. Dat was een bewuste keuze, omdat problemen in de database-laag doorwerken naar alles daarboven. Als je queries niet kloppen, maakt het niet uit of je HTTP-routing perfect is.

Het opzetten van een reproduceerbare testdatabase was trouwens een van de meest frustrerende onderdelen. In het begin deed ik dat handmatig, en elke keer als ik de database resette moest ik weer opnieuw data invoeren. Op een gegeven moment heb ik SQL seed-scripts geschreven die automatisch een schone database vullen met circa 500 testgebruikers. Dat had ik eerder moeten doen.

**4.3 Performance testing**

Bij performance testing gaat het erom of het systeem snel genoeg is en niet omvalt onder belasting (Olsen et al., 2021).

|                          |              |      |                      |
| ------------------------ | ------------ | ---- | -------------------- |
| KPI                      | Norm         | Bron | Tool                 |
| API P95 response time    | < 200ms      | TR   | k6                   |
| Dashboard load time      | < 3 seconden | TR   | Chrome DevTools / k6 |
| Gelijktijdige gebruikers | 100          | TR   | k6                   |
| Volledige vendor sync    | < 5 minuten  | TR   | k6 + logging         |
| Database queries         | < 50ms       | TR   | PostgreSQL EXPLAIN   |

Ik probeerde eerst handmatig te testen, meerdere browsertabs openen en kijken of het snel genoeg was. Dat is natuurlijk onzin als je 100 gelijktijdige gebruikers wilt simuleren. k6 was daar de oplossing voor. Met k6 scripts kon ik geautomatiseerd load testen draaien die reproduceerbaar zijn.

Wat ik niet had verwacht: de bottleneck zat niet in de Rust-backend (die is snel), maar in een paar database-queries die geen index hadden. `PostgreSQL EXPLAIN` hielp me om precies te zien welke queries traag waren. Na het toevoegen van indexes op `org_id` en `last_active_at` in de `persons` tabel gingen de responstijden flink omlaag.

**4.4 Security testing**

Security tests draaien om de vraag: kan iemand bij dingen waar ze niet bij horen? De aanpak is gebaseerd op de OWASP Top 10 (OWASP Top 10:2021, z.d.).

|                    |                                                                    |                                 |
| ------------------ | ------------------------------------------------------------------ | ------------------------------- |
| Testsoort          | Aanpak                                                             | Tool / Referentie               |
| Authenticatietests | AUTH (SSO, JWT, MFA)                                               | Microsoft Entra ID + Postman    |
| Token-beveiliging  | Expiry, revocation, secure cookie (niet localStorage)              | OWASP ASVS paragraaf 3          |
| AVG-compliance     | E-mailmaskering in logs, recht op vergetelheid, data-minimalisatie | GDPR Art. 5, Auth-test doc      |
| Security headers   | HSTS, CSP, X-Frame-Options, SameSite cookies                       | OWASP Secure Headers Project    |
| Secrets management | Geen secrets in Git, gebruik van `.env`                            | TR-001 + GitHub Actions secrets |

De security headers waren meer werk dan ik dacht. Met name Content Security Policy gaf problemen. De eerste keer dat ik een strikte CSP instelde blokkeerde die inline scripts die de React-frontend nodig had voor het renderen. Dat was frustrerend, want je wilt het veilig hebben maar het mag niet ten koste gaan van de functionaliteit. Na drie of vier iteraties had ik een configuratie die werkte: streng genoeg voor beveiliging, maar zonder dat de app kapotging.

Een ander punt waar ik tegenaan liep: tokens. In eerste instantie had ik JWT-tokens in localStorage opgeslagen. Na het lezen van de OWASP ASVS-richtlijnen realiseerde ik me dat dat een risico is bij XSS-aanvallen. De tokens zijn nu opgeslagen in secure, HttpOnly cookies. Dat vereiste aanpassingen in zowel de frontend als de backend, maar het is veiliger.

**4.5 Usability testing**

Usability testing was misschien wel het meest leerzame onderdeel van het hele testproces.

|           |                                                                                      |
| --------- | ------------------------------------------------------------------------------------ |
| Aspect    | Invulling                                                                            |
| Methode   | Think-aloud protocol in Sprint 6                                                     |
| Doelgroep | 3-5 Equans-medewerkers: 1-2 licentiebeheerders, 1 finance medewerker, 1 IT-beheerder |
| Kernflows | Login-flow, licentieopvraag per team, CSV-export, handmatige refresh                 |
| Evaluatie | Heuristische evaluatie op basis van bruikbaarheidscriteria (Olsen et al., 2021)      |
| Opzet     | Gestructureerd observatiescript met think-aloud; 45 min per sessie                   |

Het think-aloud protocol werkt zo: je vraagt deelnemers om hardop te vertellen wat ze denken terwijl ze taken uitvoeren. Dat klinkt simpel, maar het is confronterend. Een van de licentiebeheerders zocht twee minuten lang naar de CSV-export knop. Die stond rechtsbovenin, heel logisch vanuit mijn perspectief als ontwikkelaar, maar de gebruiker verwachtte hem onderaan de tabel.

Nog zo'n ding: de OrgFilter component. Ik had er een zoekbalk in gezet, maar een deelnemer probeerde eerst te filteren via een dropdown. Die was er niet. Dat zijn die momenten waarop je beseft dat jouw mentale model van de applicatie niet hetzelfde is als dat van de gebruiker.

**5\. Testomgeving**

De testomgeving bestaat eigenlijk uit twee lagen: lokaal en in de CI-pipeline. Lokaal draait alles in Docker. Backend, frontend, database, het zit allemaal in containers die je met `docker-compose up` opstart via `infra/docker-compose.yml`. Dat werkt prima voor ontwikkelen en handmatig testen.

|                   |                                                                       |
| ----------------- | --------------------------------------------------------------------- |
| Omgeving          | Beschrijving                                                          |
| Ontwikkelomgeving | Dev container (Debian GNU/Linux 12 Bookworm) in VS Code               |
| Backend runtime   | Rust 1.7x + Axum 0.7                                                  |
| Database          | PostgreSQL 16 (Docker Compose via `infra/docker-compose.yml`)         |
| Frontend runtime  | Node.js + Vite 6.4 + React 19 + TypeScript 5.9                        |
| Testbrowser       | Microsoft Edge (primair), Google Chrome (secundair)                   |
| CI/CD             | GitHub Actions (`.github/workflows/`)                                 |
| Netwerk           | Lokaal Docker-netwerk voor geisoleerde tests; staging voor integratie |

Naast de lokale omgeving draait een deel van de tests automatisch in GitHub Actions. Hierbij is gekozen voor twee aparte workflows. De eerste, `code-review.yml`, wordt getriggerd bij elke pull request naar `main`. Die draait `cargo fmt --check` voor formatting, `cargo clippy` voor linting, `cargo test --all-features` voor de unit tests, en `cargo tarpaulin` voor een coverage rapport. Hierdoor krijg ik bij elke PR direct feedback of de code nog werkt en aan de kwaliteitseisen voldoet. Tijdens het project bleek dat erg waardevol, want een paar keer had ik lokaal iets gefixt maar vergeten om alle tests te draaien. De CI-pipeline ving dat op.

De tweede workflow, `security-scan.yml`, draait bij push en PR naar `main`, en daarnaast wekelijks op maandagochtend. Die voert `cargo audit` uit op de Rust-dependencies en `npm audit` op de frontend-packages. Een uitdaging hierbij was dat `cargo audit` soms false positives gaf voor dependencies die indirect werden meegetrokken. De workflow faalt alleen bij critical of high severity issues, zodat het niet bij elke advisory blokkeert.

Wat nog niet in de CI-pipeline zit: de frontend tests (`npm test`), integratietests met een PostgreSQL service container, en de k6 performance tests. Dat zijn verbeterpunten voor een volgende fase. De frontend tests zou je vrij eenvoudig kunnen toevoegen als extra job in `code-review.yml`. Voor de integratietests heb je een PostgreSQL service container nodig in GitHub Actions, dat is iets meer configuratie maar zeker haalbaar. De k6 tests zijn lastiger, want die vereisen een draaiende applicatie, daar past een aparte staging-pipeline beter bij.

De dev container was achteraf gezien een goede keuze. Brian en Viktor werkten in dezelfde omgeving als ik, dus het probleem van "bij mij werkt het wel" kwam niet voor. Alle dependencies, versies en configuratie zitten vastgelegd in de container-definitie. Je opent VS Code, hij bouwt de container, en je kunt aan de slag. Geen gedoe met lokale installaties.

**6\. Entry- en exitcriteria**

Wanneer begin je met testen, en wanneer ben je klaar? De IEEE 829-standaard geeft daar richtlijnen voor (Client Challenge, z.d.).

Testen mag starten als:

- \[ \] Code compileert zonder errors (`cargo build`, `npm run build`)
- \[ \] Functionele requirements zijn vastgesteld en goedgekeurd
- \[ \] Testomgeving (Docker Compose) is operationeel
- \[ \] Seed-data en databasemigraties zijn uitgevoerd
- \[ \] Code review is afgerond voor het te testen component
- \[ \] Unit tests slagen lokaal (`cargo test`, `npm test`)

Testen is klaar als alle Must Have-functionaliteiten getest zijn, er geen kritieke defects meer openstaan, en de coveragedoelen gehaald zijn (Olsen et al., 2021). 100% coverage heb ik niet nagestreefd, dat is voor een solo-project niet realistisch en ook niet per se nuttig. Het gaat erom dat de kern van het systeem betrouwbaar is.

**7\. Deliverables**

De tests zijn verdeeld over meerdere sprints. Unit tests liepen door het hele project, integratietests kwamen er in de middelste sprints bij, en systeem- en usability tests zaten in de laatste fase. Wat er is opgeleverd:

1.  Master Test Plan (dit document)
2.  Test Strategy
3.  Performance testrapport (k6)
4.  Security testrapporten (AUTH + GDPR)
5.  Traceability Matrix (requirements en testcases)
6.  Usability testrapport
7.  Testafsluitrapport

**8\. Kritische reflectie en leerpunten**

**8.1 Beperkingen van de testaanpak**

Laat ik eerlijk zijn over de beperkingen. Myers et al. (2011) zeggen dat uitputtend testen onmogelijk is, en dat klopt. Je maakt altijd keuzes, en die keuzes hebben consequenties.

Het grootste probleem was dat ik zowel de ontwikkelaar als de tester was. Olsen et al. (2021) waarschuwen hiervoor: als dezelfde persoon de code en de tests schrijft, zitten dezelfde aannames in allebei. Ik heb echt geprobeerd om daar kritisch naar te kijken, maar je vangt niet alles. Geautomatiseerde tests helpen, die falen ongeacht wie ze geschreven heeft, als de logica niet klopt. Code reviews door Brian hielpen ook. En de acceptatietests door Equans-medewerkers brachten dingen aan het licht die ik zelf nooit gezien had.

Dan de externe API's. De Atlassian Cloud API en GitHub Enterprise API kon ik niet in een volledig geisoleerde omgeving draaien. Ik gebruikte mock servers, maar die bootsen het echte gedrag niet perfect na. Ik ben daar een paar keer tegenaan gelopen: een test die groen was met de mock, maar die faalde tegen de echte API omdat het response-formaat net anders was. Vervelend, maar onvermijdelijk als je met externe services werkt.

Penetratietesting had ik niet de expertise of tooling voor. OWASP Top 10-gebaseerde tests (OWASP Top 10:2021, z.d.) zijn een compromis, geen volledige vervanging. Ik weet dat er mogelijk kwetsbaarheden zijn die ik gemist heb.

En dan tijdsdruk. Niet alle Could Have-testcases zijn uitgevoerd. De risk-based aanpak hielp om te focussen op wat het zwaarst weegt (Olsen et al., 2021), maar het betekent dat de randen van het systeem minder goed getest zijn. Dat is een bewuste afweging geweest, maar het voelt niet helemaal lekker.

**8.2 Evaluatie van testactiviteiten**

|                   |                                                                        |
| ----------------- | ---------------------------------------------------------------------- |
| Aspect            | Evaluatie                                                              |
| Unit tests Rust   | Effectief voor vroege detectie van logicafouten in licentieberekening  |
| Security tests    | Goede dekking dankzij gestructureerde aanpak op basis van OWASP Top 10 |
| Performance tests | k6-scripts leverden concrete, meetbare resultaten                      |
| Integratietests   | Mock-gebaseerde aanpak functioneel, maar beperkt in representativiteit |

Als ik terugkijk: de Rust unit tests leverden het meeste op. Door `cargo test` kon ik na elke wijziging binnen seconden zien of er iets kapot was. De strenge types in Rust hielpen ook, want het type system dwingt je om over foutpaden na te denken.

De security tests op JWT-validatie en token-handling waren waardevol, zeker nadat ik de localStorage-fout had ontdekt. De integratietests met mocks leverden resultaten op, maar ik had steeds het gevoel dat ze niet het hele verhaal vertelden. De mocks gedragen zich netjes, de echte API's niet altijd.

De k6 performance tests waren het meest verrassend. Ik verwachtte dat de Rust-backend de bottleneck zou zijn, maar de database-queries waren het probleem. Zonder die tests had ik dat niet gevonden.

**9\. Conclusie**

Dit Master Test Plan beschrijft hoe ik het testen van het Operational Insights Dashboard heb aangepakt. De aanpak combineert risk-based testing (Olsen et al., 2021) met de testpyramide (Myers et al., 2012) en de IEEE 829-structuur (Client Challenge, z.d.).

**9.1 Kwaliteitsvalidatie**

Alle Must Have-requirements zijn getest. De API haalt een P95 responstijd onder 200ms, het dashboard laadt binnen 3 seconden. Security tests op basis van OWASP Top 10 leverden geen kritieke kwetsbaarheden op. Backend coverage zit boven 75%, frontend boven 60%. De usability tests gaven positieve feedback op de kernflows, al waren er wel verbeterpunten rond de CSV-export en filternavigatie.

**9.2 Beperkingen**

Solo-ontwikkeling beperkt de testonafhankelijkheid, daar kom ik niet omheen. Geautomatiseerde tests en reviews door Brian compenseren dat gedeeltelijk. Een echte pentest zat er niet in, de OWASP-tests zijn een basis maar geen vervanging. De mocks voor externe API's werken, maar ze vangen niet alles. En door de tijdsdruk heb ik me beperkt tot Must Have en Should Have requirements.

**9.3 Projectbijdrage**

Het testen heeft bevestigd dat het dashboard doet wat het moet doen: betrouwbare licentie-inzichten geven, correcte kostenberekeningen via de chargeback-logica, voldoen aan beveiligingseisen, en aansluiten bij wat licentiebeheerders en finance medewerkers nodig hebben. Wat mij het meeste vertrouwen gaf: de combinatie van geautomatiseerde tests (die objectief zijn) met de usability tests (die laten zien of het ook echt werkt voor mensen).

**9.4 Eindoordeel**

Het dashboard voldoet aan de kwaliteitscriteria en is klaar voor oplevering bij Equans. Is het perfect? Nee. De beperkingen die ik hierboven benoemd heb zijn reeel. Maar het testproces is degelijk geweest, met een combinatie van academische onderbouwing (ISTQB, IEEE 829, OWASP) en praktische ervaring. Ik heb vertrouwen in het systeem, en ik heb de beperkingen open benoemd zodat iedereen weet waar het wel en niet op getest is.

**10\. Referenties**

1.  _Client challenge_. (z.d.). https://www.scribd.com/document/531867110/IEEE-Std-829-2008-IEEE-Standard-for-Software-and-System-Test-Documentation-1423058832-HCSCIRY
2.  Myers, G. J., Badgett, T., & Sandler, C. (2012). _THE ART OF SOFTWARE TESTING_ (Third Edition) [Book]. John Wiley & Sons, Inc. https://malenezi.github.io/malenezi/SE401/Books/114-the-art-of-software-testing-3-edition.pdf
3.  Olsen, K., Posthuma, M., Ulrich, S., Olsen, K., Parveen, T., Black, R., Friedenberg, D., Hamburg, M., McKay, J., Posthuma, M., Schaefer, H., Smilgin, R., Smith, M., Toms, S., Ulrich, S., Walsh, M., Zakaria, E., Muller, T., Friedenberg, D., . . . Veenendaal, E. V. (2021). Certified Tester Foundation Level Syllabus. In International Software Testing Qualifications Board, _International Software Testing Qualifications Board_. https://istqb-main-web-prod.s3.amazonaws.com/media/documents/ISTQB-CTFL_Syllabus_2018_v3.1.1.pdf
4.  _OWASP Top 10:2021_. (z.d.). https://owasp.org/Top10/2021/
