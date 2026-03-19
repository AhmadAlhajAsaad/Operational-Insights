**Sprints planning**

**Equans Operational Insights Dashboard**

- Versie: 1.0
- Studentnaam: Ahmad Alhaj Asaad
- Project: Equans Operational Insights Dashboard
- Methodiek: Scrum/ Agile
- Sprintduur: 2 weken (uitzonderingen: Sprint 4 & Sprint 5 duren 3 weken)
- Startdatum: 17 november 2025
- Einddatum: 11 april 2026
- Totale looptijd: 20 weken
- Schoolbegeleider: Jeroen Boogaard
- Bedrijfsbegeleider: Viktor Klein (Business Owner)
- Technisch begeleider: Brian Veltman

Inhoudsopgave

[1\. Scrum-werkwijze 2](#_Toc1739087194)

[2\. Sprintoverzicht 3](#_Toc1436003593)

[3\. Gedetailleerde Sprintbeschrijvingen 3](#_Toc841285763)

[Sprint 1 — Onboarding & Scope 3](#_Toc1763763539)

[Sprint 2 — Analyse & Requirements 4](#_Toc108532405)

[Sprint 3 — Backend Fundament 4](#_Toc748243056)

[Sprint 4 — Frontend MVP 5](#_Toc2097892201)

[Sprint 5 — Hardening & Testen 5](#_Toc1612429346)

[Sprint 6 — Usability-evaluatie & Feedbackverwerking 6](#_Toc1294335685)

[Sprint 7 — Fine-tuning & Stakeholder Demo 6](#_Toc1163932610)

[Sprint 8 — Einddemo & Afronding 7](#_Toc167553983)

[4\. Definition of Done 7](#_Toc984527774)

[5\. Risicobeheer per Sprint 8](#_Toc254108469)

**1\. Scrum-werkwijze**

Het project wordt uitgevoerd conform de Agile/Scrum-methodiek. Onderstaande tabel beschrijft de terugkerende Scrum-rituelen die gedurende het gehele project worden toegepast.

|     |     |     |     |
| --- | --- | --- | --- |
| Ritueel | Frequentie | Duur | Deelnemers |
| Daily Stand-up | Dagelijks | 15 minuten | Ontwikkelaar, bedrijfsbegeleider en technisch begeleider |
| Sprint Planning | Begin elke sprint | 1–2 uur | Ontwikkelaar (Ahmad) |
| Sprint Review | Einde elke sprint | 30–60 min | Ontwikkelaar + een van de begeleiders |
| Retrospective | Einde elke sprint | 30 minuten | Ontwikkelaar + begeleiders |
| Backlog Refinement | Wekelijks | 30 minuten | Ontwikkelaar |

**Tooling:**

- **Taakbeheer**: Jira (projectcode: SDPDOFS)
- **Documentatie**: Confluence
- **Versiebeheer**: GitHub (feature branches gekoppeld aan Jira-issues)
- **Communicatie**: Microsoft Teams (dagelijkse stand-up)

**2\. Sprintoverzicht**

|     |     |     |     |
| --- | --- | --- | --- |
| Sprint | Periode | Weken | Sprintdoel |
| 1   | 17 nov – 28 nov 2025 | 1–2 | Onboarding, scope-definitie en stakeholder interviews |
| 2   | 1 dec – 12 dec 2025 | 3–4 | Stakeholderanalyse en technische requirements |
| 3   | 15 dec – 26 dec 2025 | 5–6 | Backend fundament: datacollectie, database en basis-API |
| 4   | 29 dec 2025 – 16 jan 2026 | 7–9 | Frontend MVP: dashboard en KPI-overzicht |
| 5   | 19 jan – 6 feb 2026 | 10–12 | Hardening, testen, CI/CD en documentatie |
| 6   | 9 feb – 27 feb 2026 | 13–15 | Usability-evaluatie en verwerking van feedback |
| 7   | 2 mrt – 20 mrt 2026 | 16–18 | Fine-tuning, regressietests en stakeholder demo |
| 8   | 23 mrt – 11 apr 2026 | 19–20 | Einddemo, eindpresentatie en afronding |

**3\. Gedetailleerde Sprintbeschrijvingen**

**Sprint 1 — Onboarding & Scope**

|     |     |
| --- | --- |
| Veld | Details |
| Periode | 17 november – 28 november 2025 (Week 1–2) |
| Sprintdoel | Het project volledig opstarten, de scope vaststellen en de technische randvoorwaarden documenteren. |

**Belangrijkste activiteiten:**

|     |     |
| --- | --- |
| #   | Activiteit |
| 1   | Onboarding bij Equans SLS Digital Platforms: toegang tot tools (Jira, Confluence, GitHub) |
| 2   | Kennismakingsgesprekken met team, Viktor Klein en Brian Veltman |
| 3   | Scope-definitie vastleggen: welke platforms vallen binnen het project (Atlassian, GitHub, JFrog) |
| 4   | Eerste opzet van het datamodel (conceptueel ERD) |
| 5   | Lokale ontwikkelomgeving inrichten: Docker, PostgreSQL, Rust toolchain, Node.js |
| 6   | Repository aanmaken op GitHub met initiële projectstructuur |
| 7   | &nbsp;Privacy- en security-plan opstellen (GDPR-overwegingen, maskering van e-mailadressen) |

**Resultaten/ Deliverables:**

- Afstudeervoorstel (definitief)
- Scopedocument vastgesteld en goedgekeurd door Viktor Klein
- Conceptueel datamodel (ERD — versie 1)
- Privacy- en security-plan
- Werkende lokale ontwikkelomgeving (Docker Compose)
- GitHub-repository met initiële structuur en eerste commit

**Sprint 2 — Analyse & Requirements**

|     |     |
| --- | --- |
| Veld | Details |
| Periode | 1 december – 12 december 2025 (Week 3–4) |
| Sprintdoel | Systematische analyse van stakeholderbehoeften en het vastleggen van functionele en technische vereisten in een SRS. |

**Belangrijkste activiteiten:**

|     |     |
| --- | --- |
| #   | Activiteit |
| 1   | Stakeholderinterviews uitvoeren (IT, Finance, DevOps) |
| 2   | Functionele requirements opstellen |
| 3   | Technische requirements opstellen |
| 4   | Business requirements doornemen en valideren met Viktor Klein |
| 5   | SRS-document opstellen (MoSCoW-prioritering) |
| 6   | Atlassian Cloud API verkennen: endpoints, authenticatie, rate limits |
| 7   | GitHub Enterprise API verkennen: seats, Copilot, GHAS-endpoints |

**Resultaten/ Deliverables:**

- Software Requirements Specification v1.0
- Functionele Requirements
- Technische Requirements
- API-verkenningsrapport (Atlassian + GitHub)

**Sprint 3 — Backend Fundament**

|     |     |
| --- | --- |
| Veld | Details |
| Periode | 15 december – 26 december 2025 (Week 5–6) |
| Sprintdoel | Een werkende backend bouwen die data verzamelt via de Atlassian API, opslaat in PostgreSQL, en de basis-REST-API beschikbaar stelt. |

**Belangrijkste activiteiten:**

|     |     |
| --- | --- |
| #   | Activiteit |
| 1   | Databaseschema implementeren: persons, organizations, atlassian_users, atlassian_groups |
| 2   | Migratiebestanden aanmaken |
| 3   | Atlassian Admin API-client implementeren in Rust (paginering, authenticatie) |
| 4   | Datacollectie-module implementeren: gebruikers en groepen ophalen en opslaan |
| 5   | Basis-REST-API endpoints implementeren: /health, /api/atlassian/users, /api/atlassian/groups |
| 6   | Cron-job/ achtergrondtaak inrichten voor dagelijkse synchronisatie |
| 7   | Unit tests schrijven voor API-handlers |

**Resultaten/ Deliverables:**

- Werkende Rust-backend met verbinding naar PostgreSQL
- Atlassian-gebruikers en -groepen worden opgehaald en opgeslagen
- REST-API endpoint /api/atlassian/users reageert correct (PASS in testscript)
- Databasemigraties uitgevoerd
- Eerste unit tests geslaagd

**Sprint 4 — Frontend MVP**

|     |     |
| --- | --- |
| Veld | Details |
| Periode | 29 december 2025 – 16 januari 2026 (Week 7–9) |
| Sprintdoel | Figma-ontwerp, een werkend frontend-dashboard bouwen dat KPI's en gebruikersdata toont, gecommuniceerd via de backend-API. |

**Belangrijkste activiteiten:**

|     |     |
| --- | --- |
| #   | Activiteit |
| 1   | React-projectstructuur opzetten (TypeScript + Tailwind CSS) |
| 2   | Navigatiestructuur en routering implementeren (React Router) |
| 3   | KPI-kaarten implementeren voor Atlassian-licentiekosten (Jira, Confluence) |
| 4   | Gebruikerstabel implementeren met zoek- en filterfunctionaliteit |
| 5   | Organisatietabel implementeren met statistieken |
| 6   | API-communicatie opzetten: fetch-client voor backend-endpoints |
| 7   | Configuratiebestand voor productprijzen aanmaken (config/productPricing.ts) |
| 8   | Equans-huisstijl toepassen: kleurpallet, typografie, componentenstijl |
| 9   | Figma-wireframes finaleren en afstemmen met Viktor (Business Owner) |

**Resultaten/ Deliverables:**

- Werkende React-frontend (localhost:5173)
- Dashboard-overzichtspagina met KPI-kaarten voor Atlassian-producten
- Gebruikerstabel met zoek- en pagineeringfunctionaliteit
- Organisatietabel met statistieken
- Gevalideerde Figma-wireframes
- Equans-huisstijl consequent toegepast (kleurpallet #002439, #008163)

**Sprint 5 — Hardening & Testen**

|     |     |
| --- | --- |
| Veld | Details |
| Periode | 19 januari – 6 februari 2026 (Week 10–12) |
| Sprintdoel | De applicatie hardenen, uitgebreid testen, validatie uitvoeren met stakeholders. |

**Belangrijkste activiteiten:**

|     |     |
| --- | --- |
| #   | Activiteit |
| 1   | Integratietests schrijven voor alle API-endpoints (backend + database) |
| 2   | PowerShell-testscripts uitvoeren: test_atlassian_endpoints.ps1, test_github_endpoints.ps1 |
| 3   | GitHub Actions CI/CD-pipeline inrichten (build, test, lint) |
| 4   | Foutafhandeling versterken in backend (geen unwrap() in productiecode) |
| 5   | GDPR-nalevingscontrole: maskering van e-mailadressen in logs |
| 6   | Rate-limiting afhandeling implementeren (exponential backoff bij GitHub/Atlassian API) |
| 7   | Prestatiemeting uitvoeren: API P95 < 200ms, dashboard laadtijd < 3s |
| 8   | Stakeholdervalidatie: demo aan Viktor Klein + feedback verwerken |
| 9   | Technische documentatie bijwerken (README, API-documentatie) |

**Resultaten/ Deliverables:**

- Alle integratietests geslaagd (PASS-status in logbestanden)
- CI/CD-pipeline operationeel op GitHub Actions
- Prestatiemeting: P95 API-responstijd < 200ms geverifieerd
- GDPR-nalevingsrapport (maskering e-mail geverifieerd)
- Stakeholder demo uitgevoerd
- Bijgewerkte technische documentatie

**Sprint 6 — Usability-evaluatie & Feedbackverwerking**

|     |     |
| --- | --- |
| Veld | Details |
| Periode | 9 februari – 27 februari 2026 (Week 13–15) |
| Sprintdoel | Usability-evaluatie uitvoeren met eindgebruikers, feedback systematisch verwerken en testdocumentatie completeren. |

**Belangrijkste activiteiten:**

|     |     |
| --- | --- |
| #   | Activiteit |
| 1   | Usability-testplan opstellen |
| 2   | Usability-tests uitvoeren met minimaal 2 eindgebruikers (licentiebeheerder, finance medewerker) |
| 3   | Usability-testrapport opstellen met bevindingen en aanbevelingen |
| 4   | UI-verbeteringen doorvoeren op basis van testresultaten |
| 5   | Personen-GID-matching module testen en valideren |
| 6   | CSV-importfunctionaliteit testen met echte en incomplete datasets |
| 7   | Atlassian–persoon koppelingslogica verfijnen (e-mailbased matching) |
| 8   | Performance-testplan opstellen en uitvoeren |

**Resultaten/ Deliverables:**

- Usability-testplan (Confluence geverifieerd)
- Usability-testrapport met PASS/FAIL-bevindingen
- Verwerkte UI-verbeteringen (aantoonbaar via commits)
- Performance-testrapport
- Atlassian–persoon koppeling volledig functioneel en getest

**Sprint 7 — Fine-tuning & Stakeholder Demo**

|     |     |
| --- | --- |
| Veld | Details |
| Periode | 2 maart – 20 maart 2026 (Week 16–18) |
| Sprintdoel | De applicatie fijnstellen op basis van alle eerdere feedback, regressietests uitvoeren en een formele stakeholderdemo geven. |

**Belangrijkste activiteiten:**

|     |     |
| --- | --- |
| #   | Activiteit |
| 1   | Resterende Should Have-eisen implementeren (CSV-export, geavanceerde filtering) |
| 2   | Regressietests uitvoeren op alle functionaliteiten |
| 3   | Dashboard-fine-tuning: typografie, kleurgebruik, empty states, foutmeldingen |
| 4   | SRS-document finaliseren (versie 1.0 definitief) |
| 5   | Sprintplanning bijwerken en afronden |
| 6   | Formele stakeholderdemo geven aan Viktor Klein en Brian Veltman |
| 7   | Feedbackformulieren verzamelen van alle begeleiders |
| 8   | Afstudeerscriptie beginnen schrijven (inleiding, achtergrond, methode) |

**Resultaten/ Deliverables:**

- CSV-export functioneel voor alle dashboardweergaven
- Regressietestrapport (alle scenarios PASS)
- SRS v1.0 definitief en goedgekeurd
- Stakeholderdemo gehouden + feedbackformulieren ontvangen
- Eerste concepthoofdstukken afstudeerscriptie

**Sprint 8 — Einddemo & Afronding**

|     |     |
| --- | --- |
| Veld | Details |
| Periode | 23 maart – 11 april 2026 (Week 19–20) |
| Sprintdoel | Alle documentatie afronden, de einddemo en -presentatie voorbereiden en het project formeel opleveren aan Equans en de Hogeschool Rotterdam. |

**Belangrijkste activiteiten:**

|     |     |
| --- | --- |
| #   | Activiteit |
| 1   | Afstudeerscriptie completeren en controleren |
| 2   | Eindpresentatie voorbereiden (slides, demo-omgeving) |
| 3   | Alle documentatie samenvoegen en controleren op volledigheid |
| 4   | Definitieve code-freeze: geen functionele wijzigingen na deze sprint |
| 5   | Einddemo uitvoeren voor Equans stakeholders (Viktor Klein, Brian Veltman, Henk) |
| 6   | Eindpresentatie geven aan Hogeschool Rotterdam docenten |
| 7   | Projectrepository opschonen en archiveren |
| 8   | Overdrachts- en beheerdocumentatie opleveren aan Equans DevOps Forge |

**Resultaten/ Deliverables:**

- Definitieve afstudeerscriptie (ingeleverd bij Hogeschool Rotterdam)
- Eindpresentatie (slides + live demo)
- Alle testresultaten gedocumenteerd (PASS/FAIL-logbestanden)
- Volledig projectarchief op GitHub (gearchiveerd)
- Overdrachts- en beheerdocumentatie opgeleverd aan Equans
- Feedbackformulieren van alle begeleiders ontvangen en verwerkt

**4\. Definition of Done**

Een Jira-issue of user story wordt als **Done** beschouwd wanneer aan alle onderstaande criteria is voldaan:

|     |     |
| --- | --- |
| Criterium | Beschrijving |
| Code geïmplementeerd | De functionaliteit is volledig geïmplementeerd conform de acceptatiecriteria |
| Tests geslaagd | Relevante unit- en/of integratietests zijn aanwezig en slagen (PASS) |
| Code review | Wijziging is besproken met en geaccordeerd door de technisch begeleider (Brian Veltman) |
| Geen unwrap() in productie | Rust-code maakt geen gebruik van unwrap() buiten testcontext |
| Gedocumenteerd | Functionaliteit is gedocumenteerd in Confluence en/of codeopmerkingen |
| Jira bijgewerkt | Het Jira-issue heeft de status Done en commitberichten verwijzen naar het PAN-nummer |
| Gemerged naar main | De feature branch is via een pull request samengevoegd in de main-branch |

**5\. Risicobeheer per Sprint**

|     |     |     |     |     |
| --- | --- | --- | --- | --- |
| Sprint | Risico | Kans | Impact | Maatregel |
| 1   | Vertraagde toegang tot Equans-tools (Jira, GitHub, Confluence) | Middel | Hoog | Vroeg escaleren naar Viktor Klein; parallel starten met theoretische voorbereiding |
| 2   | Onvolledige of tegenstrijdige requirements van stakeholders | Middel | Middel | Iteratieve validatiesessies; MoSCoW-prioritering als kompas |
| 3   | Atlassian API rate limits of authenticatieproblemen | Hoog | Middel | Exponential backoff; caching van resultaten; mock-data als fallback |
| 4   | Scope creep in de frontend (extra features buiten MVP) | Hoog | Middel | Strikte naleving van Must Have-eisen; Could Have expliciet uitgesteld |
| 5   | Moeilijk recruteren van testdeelnemers voor usability-test | Middel | Laag | Alternatieve deelnemers: teamleden DevOps Forge |
| 6   | Stakeholder niet beschikbaar voor formele demo | Laag | Middel | Demo-opname als alternatief; asynchrone feedback via Confluence |
| 7   | Onvoldoende tijd voor afronding scriptie door late bevindingen | Middel | Hoog | Code-freeze na Sprint 7; alleen documentatie en presentatie in Sprint 8 |