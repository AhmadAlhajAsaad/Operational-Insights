**Sprintplanning**

**Equans Operational Insights Dashboard**

- Versie: 1.0
- Studentnaam: Ahmad Alhaj Asaad
- Project: Equans Operational Insights Dashboard
- Methodiek: Scrum/Agile
- Sprintduur: 2 weken (uitzonderingen: Sprint 4 en Sprint 5 duren 3 weken)
- Startdatum: 17 november 2025
- Einddatum: 11 april 2026
- Totale looptijd: 20 weken
- Schoolbegeleider: Jeroen Boogaard
- Bedrijfsbegeleider: Viktor Klein (Business Owner)
- Technisch begeleider: Brian Veltman

---

## 1. Scrum-werkwijze

Binnen dit project werk ik volgens de Agile/Scrum-methodiek. Ik heb hiervoor gekozen omdat het goed past bij een project dat in korte iteraties moet opleveren, en omdat het DevOps Forge-team bij Equans zelf ook met Scrum werkt. Elke ochtend is er een stand-up van een kwartier, waar ik samen met mijn begeleiders de voortgang, eventuele blokkades en de planning voor de dag bespreek. Aan het begin van elke sprint plan ik de taken (1 tot 2 uur), en aan het einde volgt er een review (30 tot 60 minuten) waarin ik het resultaat laat zien aan een van de begeleiders, gevolgd door een korte retrospective. Wekelijks neem ik ook even de backlog door om te kijken of de prioriteiten nog kloppen.

| Ritueel | Frequentie | Duur | Deelnemers |
| --- | --- | --- | --- |
| Daily Stand-up | Dagelijks | 15 minuten | Ontwikkelaar, bedrijfsbegeleider en technisch begeleider |
| Sprint Planning | Begin elke sprint | 1 tot 2 uur | Ontwikkelaar (Ahmad) |
| Sprint Review | Einde elke sprint | 30 tot 60 min | Ontwikkelaar + een van de begeleiders |
| Retrospective | Einde elke sprint | 30 minuten | Ontwikkelaar + begeleiders |
| Backlog Refinement | Wekelijks | 30 minuten | Ontwikkelaar |

Voor het taakbeheer gebruik ik Jira (projectcode: SDPDOFS), documentatie leg ik vast in Confluence, versiebeheer loopt via GitHub (met feature branches gekoppeld aan Jira-issues) en de dagelijkse stand-up doen we via Microsoft Teams.

## 2. Sprintoverzicht

Het project loopt over 8 sprints verdeeld over 20 weken. De meeste sprints duren 2 weken, maar Sprint 4 en Sprint 5 heb ik bewust 3 weken gegeven. Sprint 4 bevat de volledige frontend MVP, en dat bleek in de praktijk meer werk dan in 2 weken past. Sprint 5 combineert hardening met testen en CI/CD, wat ook meer ruimte nodig had.

| Sprint | Periode | Weken | Sprintdoel |
| --- | --- | --- | --- |
| 1 | 17 nov - 28 nov 2025 | 1-2 | Onboarding, scope-definitie en stakeholder interviews |
| 2 | 1 dec - 12 dec 2025 | 3-4 | Stakeholderanalyse en technische requirements |
| 3 | 15 dec - 26 dec 2025 | 5-6 | Backend fundament: datacollectie, database en basis-API |
| 4 | 29 dec 2025 - 16 jan 2026 | 7-9 | Frontend MVP: dashboard en KPI-overzicht |
| 5 | 19 jan - 6 feb 2026 | 10-12 | Hardening, testen, CI/CD en documentatie |
| 6 | 9 feb - 27 feb 2026 | 13-15 | Usability-evaluatie en verwerking van feedback |
| 7 | 2 mrt - 20 mrt 2026 | 16-18 | Fine-tuning, regressietests en stakeholder demo |
| 8 | 23 mrt - 11 apr 2026 | 19-20 | Einddemo, eindpresentatie en afronding |

## 3. Gedetailleerde sprintbeschrijvingen

### Sprint 1: Onboarding en scope (week 1-2)

**Periode:** 17 november tot 28 november 2025

**Doel:** Het project volledig opstarten, de scope vaststellen en de technische randvoorwaarden documenteren.

De eerste twee weken stonden in het teken van opstarten. Ik moest toegang krijgen tot alle tools (Jira, Confluence, GitHub), kennismaken met het team en met Viktor Klein en Brian Veltman in gesprek gaan over wat het project precies moest opleveren. Hierbij bleek dat de scope scherper moest dan ik in eerste instantie dacht: welke platforms vallen er precies binnen het project? Uiteindelijk zijn dat Atlassian, GitHub en JFrog geworden. Daarnaast heb ik een eerste versie van het datamodel gemaakt (conceptueel ERD), de lokale ontwikkelomgeving ingericht met Docker, PostgreSQL, de Rust toolchain en Node.js, en het GitHub-repository aangemaakt met een initiele projectstructuur. Ook het privacy- en securityplan heb ik in deze sprint opgezet, omdat GDPR-overwegingen (zoals maskering van e-mailadressen) vanaf het begin meegenomen moesten worden.

**Deliverables:** Afstudeervoorstel (definitief), scopedocument (goedgekeurd door Viktor Klein), conceptueel datamodel (ERD versie 1), privacy- en securityplan, werkende lokale ontwikkelomgeving (Docker Compose) en het GitHub-repository met initiele structuur en eerste commit.

### Sprint 2: Analyse en requirements (week 3-4)

**Periode:** 1 december tot 12 december 2025

**Doel:** Systematische analyse van stakeholderbehoeften en het vastleggen van functionele en technische vereisten in een SRS.

In deze sprint heb ik interviews gehouden met stakeholders van IT, Finance en DevOps om te achterhalen wat het dashboard precies moet kunnen. Op basis van die gesprekken heb ik de functionele en technische requirements opgesteld, en die vervolgens met Viktor Klein doorgenomen en gevalideerd. Het resultaat was een SRS-document met MoSCoW-prioritering. Parallel daaraan heb ik de Atlassian Cloud API en de GitHub Enterprise API verkend: welke endpoints zijn er, hoe werkt de authenticatie, en waar zitten de rate limits? Dat laatste bleek later in Sprint 3 nog relevant, want de Atlassian API heeft vrij strikte limieten die ik in het ontwerp moest meenemen.

**Deliverables:** Software Requirements Specification v1.0, functionele en technische requirements, en een API-verkenningsrapport voor Atlassian en GitHub.

### Sprint 3: Backend fundament (week 5-6)

**Periode:** 15 december tot 26 december 2025

**Doel:** Een werkende backend bouwen die data verzamelt via de Atlassian API, opslaat in PostgreSQL, en de basis-REST-API beschikbaar stelt.

Dit was de sprint waarin het echte bouwwerk begon. Ik heb het databaseschema geimplementeerd met tabellen als `persons`, `organizations`, `atlassian_users` en `atlassian_groups`, en de bijbehorende migratiebestanden aangemaakt. De Atlassian Admin API-client heb ik in Rust geschreven, inclusief paginering en authenticatie. Een uitdaging hierbij was dat de API soms onverwachte responses teruggeeft bij grote datasets, waardoor ik de foutafhandeling robuuster moest maken dan gepland. Daarnaast heb ik de basis-REST-API endpoints gebouwd (`/health`, `/api/atlassian/users`, `/api/atlassian/groups`), een achtergrondtaak opgezet voor dagelijkse synchronisatie, en de eerste unit tests geschreven.

**Deliverables:** Werkende Rust-backend met PostgreSQL-verbinding, Atlassian-gebruikers en -groepen worden opgehaald en opgeslagen, REST-API endpoint `/api/atlassian/users` reageert correct (PASS in testscript), databasemigraties uitgevoerd, en eerste unit tests geslaagd.

### Sprint 4: Frontend MVP (week 7-9)

**Periode:** 29 december 2025 tot 16 januari 2026

**Doel:** Figma-ontwerp maken en een werkend frontend-dashboard bouwen dat KPI's en gebruikersdata toont via de backend-API.

Deze sprint duurde bewust 3 weken, omdat de frontend meer werk vergde dan de andere sprints. Ik ben begonnen met het opzetten van de React-projectstructuur (TypeScript + Tailwind CSS), de navigatie en routering (React Router), en de API-communicatie richting de backend. De kern van het werk zat in de KPI-kaarten voor Atlassian-licentiekosten (Jira, Confluence), de gebruikerstabel met zoek- en filterfunctionaliteit, en de organisatietabel met statistieken. Hierbij heb ik ook het configuratiebestand `config/productPricing.ts` aangemaakt voor productprijzen. De Equans-huisstijl (kleurpallet #002439 en #008163, typografie, componentenstijl) heb ik consequent toegepast. Vooraf had ik Figma-wireframes gemaakt en die afgestemd met Viktor. Tijdens het bouwen merkte ik dat sommige wireframes in de praktijk niet goed werkten, vooral bij lege toestanden en foutmeldingen, dus die heb ik gaandeweg aangepast.

**Deliverables:** Werkende React-frontend (localhost:5173), dashboard-overzichtspagina met KPI-kaarten, gebruikerstabel met zoek- en pagineringfunctionaliteit, organisatietabel met statistieken, gevalideerde Figma-wireframes, en Equans-huisstijl consequent toegepast.

### Sprint 5: Hardening en testen (week 10-12)

**Periode:** 19 januari tot 6 februari 2026

**Doel:** De applicatie hardenen, uitgebreid testen en validatie uitvoeren met stakeholders.

Ook deze sprint duurde 3 weken, en dat was nodig. Ik heb integratietests geschreven voor alle API-endpoints (backend + database), PowerShell-testscripts uitgevoerd (`test_atlassian_endpoints.ps1`, `test_github_endpoints.ps1`) en de GitHub Actions CI/CD-pipeline ingericht voor build, test en lint. In de backend heb ik de foutafhandeling versterkt: geen `unwrap()` meer in productiecode. De GDPR-nalevingscontrole (maskering van e-mailadressen in logs) heb ik ook in deze sprint afgerond, en de rate-limiting afhandeling met exponential backoff bij de GitHub en Atlassian API geimplementeerd. Tijdens de prestatiemeting bleek dat de API P95 responstijd ruim onder de 200ms zat, maar de dashboard-laadtijd was in eerste instantie iets boven de 3 seconden. Na het implementeren van lazy loading voor de gebruikerstabel kwam dat onder de grens. Tot slot heb ik een demo gegeven aan Viktor Klein en de feedback verwerkt.

**Deliverables:** Alle integratietests geslaagd (PASS), CI/CD-pipeline operationeel op GitHub Actions, prestatiemeting geverifieerd (P95 < 200ms), GDPR-nalevingsrapport, stakeholder demo uitgevoerd, en bijgewerkte technische documentatie.

### Sprint 6: Usability-evaluatie en feedbackverwerking (week 13-15)

**Periode:** 9 februari tot 27 februari 2026

**Doel:** Usability-evaluatie uitvoeren met eindgebruikers, feedback systematisch verwerken en testdocumentatie completeren.

In deze sprint heb ik het usability-testplan opgesteld en vervolgens usability-tests uitgevoerd met minimaal 2 eindgebruikers (een licentiebeheerder en een finance medewerker). Hieruit kwamen een aantal concrete verbeterpunten naar voren, die ik heb vastgelegd in het usability-testrapport. De UI-verbeteringen die daaruit volgden heb ik direct doorgevoerd, aantoonbaar via de commits in die periode. Daarnaast heb ik de personen-GID-matching module getest en gevalideerd, de CSV-importfunctionaliteit getest met echte en incomplete datasets (waarbij bleek dat incomplete datasets soms onverwachte edge cases opleverden), en de Atlassian-persoon koppelingslogica verfijnd op basis van e-mailmatching. Tot slot heb ik een performance-testplan opgesteld en uitgevoerd.

**Deliverables:** Usability-testplan (Confluence geverifieerd), usability-testrapport met PASS/FAIL-bevindingen, verwerkte UI-verbeteringen, performance-testrapport, en een volledig functionele en geteste Atlassian-persoon koppeling.

### Sprint 7: Fine-tuning en stakeholder demo (week 16-18)

**Periode:** 2 maart tot 20 maart 2026

**Doel:** De applicatie fijnstellen op basis van alle eerdere feedback, regressietests uitvoeren en een formele stakeholderdemo geven.

Hierbij heb ik de resterende Should Have-eisen geimplementeerd, zoals CSV-export en geavanceerde filtering. Regressietests heb ik uitgevoerd op alle functionaliteiten om te controleren of eerdere wijzigingen niets hadden gebroken. De dashboard-fine-tuning richtte zich op typografie, kleurgebruik, empty states en foutmeldingen. Het SRS-document heb ik gefinaliseerd naar versie 1.0, de sprintplanning bijgewerkt en afgerond, en een formele stakeholderdemo gegeven aan Viktor Klein en Brian Veltman. Feedbackformulieren zijn verzameld van alle begeleiders. Daarnaast ben ik begonnen met het schrijven van de afstudeerscriptie (inleiding, achtergrond en methode).

**Deliverables:** CSV-export functioneel voor alle dashboardweergaven, regressietestrapport (alle scenarios PASS), SRS v1.0 definitief en goedgekeurd, stakeholderdemo gehouden met feedbackformulieren ontvangen, en eerste concepthoofdstukken van de afstudeerscriptie.

### Sprint 8: Einddemo en afronding (week 19-20)

**Periode:** 23 maart tot 11 april 2026

**Doel:** Alle documentatie afronden, de einddemo en eindpresentatie voorbereiden en het project formeel opleveren aan Equans en de Hogeschool Rotterdam.

De laatste sprint. Hierin heb ik de afstudeerscriptie gecompleteerd en gecontroleerd, de eindpresentatie voorbereid (slides en demo-omgeving), en alle documentatie samengevoegd en op volledigheid gecontroleerd. Na deze sprint geldt een definitieve code-freeze: geen functionele wijzigingen meer. De einddemo heb ik uitgevoerd voor de Equans stakeholders (Viktor Klein, Brian Veltman en Henk). Daarna volgde de eindpresentatie aan de docenten van Hogeschool Rotterdam. Het projectrepository is opgeschoond en gearchiveerd, en de overdrachts- en beheerdocumentatie is opgeleverd aan het Equans DevOps Forge-team.

**Deliverables:** Definitieve afstudeerscriptie (ingeleverd bij Hogeschool Rotterdam), eindpresentatie (slides + live demo), alle testresultaten gedocumenteerd (PASS/FAIL-logbestanden), volledig projectarchief op GitHub, overdrachts- en beheerdocumentatie opgeleverd aan Equans, en feedbackformulieren van alle begeleiders ontvangen en verwerkt.

## 4. Definition of Done

Een Jira-issue of user story beschouw ik als Done wanneer aan alle onderstaande criteria is voldaan:

| Criterium | Beschrijving |
| --- | --- |
| Code geimplementeerd | De functionaliteit is volledig geimplementeerd conform de acceptatiecriteria |
| Tests geslaagd | Relevante unit- en/of integratietests zijn aanwezig en slagen (PASS) |
| Code review | Wijziging is besproken met en geaccordeerd door de technisch begeleider (Brian Veltman) |
| Geen unwrap() in productie | Rust-code maakt geen gebruik van unwrap() buiten testcontext |
| Gedocumenteerd | Functionaliteit is gedocumenteerd in Confluence en/of codeopmerkingen |
| Jira bijgewerkt | Het Jira-issue heeft de status Done en commitberichten verwijzen naar het PAN-nummer |
| Gemerged naar main | De feature branch is via een pull request samengevoegd in de main-branch |

## 5. Risicobeheer per sprint

Hieronder staan de risico's die ik per sprint heb geidentificeerd, inclusief de maatregelen die ik heb genomen of voorbereid. Sommige van deze risico's hebben zich daadwerkelijk voorgedaan (zoals de rate limits van de Atlassian API in Sprint 3), andere zijn gelukkig uitgebleven.

| Sprint | Risico | Kans | Impact | Maatregel |
| --- | --- | --- | --- | --- |
| 1 | Vertraagde toegang tot Equans-tools (Jira, GitHub, Confluence) | Middel | Hoog | Vroeg escaleren naar Viktor Klein; parallel starten met theoretische voorbereiding |
| 2 | Onvolledige of tegenstrijdige requirements van stakeholders | Middel | Middel | Iteratieve validatiesessies; MoSCoW-prioritering als kompas |
| 3 | Atlassian API rate limits of authenticatieproblemen | Hoog | Middel | Exponential backoff; caching van resultaten; mock-data als fallback |
| 4 | Scope creep in de frontend (extra features buiten MVP) | Hoog | Middel | Strikte naleving van Must Have-eisen; Could Have expliciet uitgesteld |
| 5 | Moeilijk recruteren van testdeelnemers voor usability-test | Middel | Laag | Alternatieve deelnemers: teamleden DevOps Forge |
| 6 | Stakeholder niet beschikbaar voor formele demo | Laag | Middel | Demo-opname als alternatief; asynchrone feedback via Confluence |
| 7 | Onvoldoende tijd voor afronding scriptie door late bevindingen | Middel | Hoog | Code-freeze na Sprint 7; alleen documentatie en presentatie in Sprint 8 |
