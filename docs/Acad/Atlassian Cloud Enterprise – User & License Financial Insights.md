# Atlassian Cloud Enterprise – User & License Financial Insights

## Context

- **Afdeling:** SLS Digital Platform – DevOps Forge
- **Team:** 6 personen (1 PO, 3 beheerders, 2 ontwikkelaars)
- **Missie:** Teams faciliteren met Atlassian, GitHub Enterprise en JFrog
- **Platformstack:** Rust (backend), React (frontend), Postgres, Docker (on‑prem), microservices‑ready
- **Niet in scope:** Azure/AWS‑clouddeployments en .NET

---

## Doel van de stage-opdracht

Bouw een end‑to‑end "vertical slice" die inzicht geeft in licentiegebruik, kosten en gebruikersactiviteit binnen Atlassian Cloud Enterprise (start met één product, bijv. Jira Software).

- Signaleer besparingskansen (onbenutte seats, inactieve accounts, externe accounts, te veel admins).
- Presenteer duidelijke aanbevelingen in een dashboard en lever reproduceerbare rapportages.

---

## MVP-scope

Eén product als start (bijv. Jira Software), uitbreidbaar naar Confluence en JSM.

### Kernmetrics

- Toegewezen seats, actief gebruik (30/60/90 dagen), utilization (%)
- Kosten per product/site/team (showback)
- Top‑kansen: inactief met toegang, externen met toegang, admin‑rollen boven norm

### Dashboard

- Overzicht (KPI's, trends op basis van snapshots)
- Drill‑downs (teams, users), CSV‑export
- Instellingen voor prijzen, drempels en team‑mapping

---

## Werkzaamheden stagiair

1. **Analyse en ontwerp:** datamodel, berekeningen, privacy/security‑kaders
2. **Backend (Rust):** collecte + aggregatie, Postgres‑opslag, REST‑API
3. **Frontend (React):** overzichts- en detailpagina's, filters en export
4. **CI/CD en packaging:** Docker‑compose, GitHub Actions (build/test/scan)
5. **Documentatie:** architectuur, API, runbook, gebruikers- en beheerdershandleiding

---

## Op te leveren resultaten

- Werkende end‑to‑end slice met betrouwbare berekening van seats vs. actieve gebruikers en kosten
- Dashboard met top‑5 besparingsmogelijkheden en toepasbare aanbevelingen
- Reproduceerbare setup (`docker‑compose`) en basis‑CI/CD
- Security‑hygiëne: geen secrets in code, PII‑minimalisatie, least‑privilege

---

## Technologie en tools

| Categorie                 | Technologie                                            |
| ------------------------- | ------------------------------------------------------ |
| **Backend**               | Rust (bijv. Axum/Actix), serde, sqlx/diesel            |
| **Frontend**              | React, TypeScript, lichte charting‑bibliotheek         |
| **Data**                  | Postgres                                               |
| **Dev/Infra**             | Docker, docker‑compose, GitHub Actions, GitHub Copilot |
| **Documentatie/planning** | Confluence en Jira                                     |

---

## Werkwijze en begeleiding

- Korte sprints met wekelijkse check‑ins; demo/retro per sprint
- Code reviews en pair programming met het DevOps Forge‑team
- PO voor prioritering en acceptatiecriteria
- Hybride werken mogelijk; accounts/werkplek worden geregeld

---

## Succescriteria (meetbaar)

| Criterium         | Doel                                                                                     |
| ----------------- | ---------------------------------------------------------------------------------------- |
| **Functioneel**   | Juiste seats/activiteit/kosten voor minimaal 1 product en 1–2 sites/teams                |
| **Kwaliteit**     | p95 < 500 ms voor kernendpoints (interne setup), kernlogica testdekking ~60% (afstemmen) |
| **Security**      | Geen hardcoded secrets; logging zonder PII; dependency‑scan zonder kritieke issues       |
| **Bruikbaarheid** | Minimaal 3 aanbevelingen die door een team zijn opgepakt of geaccepteerd                 |

---

## Planning (indicatief, 10–12 weken voor MVP)

| Periode      | Activiteit                                                      |
| ------------ | --------------------------------------------------------------- |
| **Wk 1–2**   | Onboarding, scope, datamodel, privacy/security‑plan             |
| **Wk 3–5**   | Backend collecte + aggregatie, database, basis‑API              |
| **Wk 6–7**   | Frontend dashboard MVP                                          |
| **Wk 8–9**   | Hardening, tests, CI/CD, documentatie                           |
| **Wk 10–12** | Validatie met stakeholders, fine‑tuning, einddemo en overdracht |

---

## Uitbreidingen (na MVP)

### Cross‑product en multi‑site

- Uitbreiden naar Confluence en JSM; geconsolideerd overzicht over alle sites
- Cross‑product totalen per team/cost center en per regio/BU

### Governance en security

- Rolbewaking: site‑/product‑admins, afwijkende permissies, policy‑checks
- Periodieke access reviews en attestatie‑rapporten

### Automatisering en integraties

- Automatisch Jira‑taken aanmaken voor offboarding/opschoonacties
- Exports naar Finance/BI (CSV/SFTP/Power BI) voor showback/chargeback
- Notificaties/alerts (e‑mail/Teams/Slack) bij drempeloverschrijdingen

### Kostenoptimalisatie en forecasting

- Trendanalyse en prognoses van seatbehoefte t.o.v. contractdrempels
- What‑if simulaties (inactief = 60/90 dagen; effect op kosten)
- Detectie van kostenanomalieën (plotselinge seat‑groei)

### Marketplace en aanvullende producten

- Inzicht in betaalde Marketplace‑apps: seats, kosten, gebruikssignalen

### Datakwaliteit en privacy

- Team/cost center mapping verfijnen (directory‑attributen, regels)
- Pseudonimisering optioneel instelbaar; dataretentie‑beleid en purge
- Audittrail op datamutaties en berekeningen

### Platformverbeteringen

- RBAC binnen de app (viewer/admin), feature flags, configuratiebeheer
- Observability (metrics, health‑checks, structured logging)
- Self‑service exports en geplande rapporten

---

## Gevraagde kennis en vaardigheden

- Basis Rust of motivatie dit snel te leren
- Ervaring met JavaScript/TypeScript en React
- Kennis van REST, JSON en SQL
- **Pré:** secure coding, software‑architectuur en design patterns
- Affiniteit met DevOps en developer experience

---

## Niet in scope

- Deployments naar publieke cloud (Azure/AWS) en .NET‑ontwikkeling
