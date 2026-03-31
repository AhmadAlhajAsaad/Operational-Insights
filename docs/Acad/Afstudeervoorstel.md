# Afstudeervoorstel

## 1 – Algemene informatie

**Studentnaam:** Ahmad Alhaj Asaad

**E-mailadres:** [1035912@hr.nl](mailto:1035912@hr.nl)

**Opleiding:** Informatica, Hogeschool Rotterdam

**Projecttitel:** Inzicht en optimalisatie van Atlassian Cloud Licentiegebruik bij Equans

## 2 – Bedrijfsinformatie & Projectbegeleider

**Bedrijfsnaam:** Equans Nederland – SLS Digital Platforms (DevOps Forge)

**Website:** https://www.equans.nl

**Bedrijfsstructuur en technische expertise:** Equans is een internationale dienstverlener met meer dan 90.000 medewerkers wereldwijd en circa 9.000 in Nederland. De afdeling SLS Digital Platforms levert interne ontwikkelplatforms en DevOps tooling aan software- en IT-teams binnen Equans. Het team DevOps Forge bestaat uit een product owner, drie beheerders en twee ontwikkelaars en biedt enterprise tools zoals Atlassian Jira/Confluence, GitHub Enterprise en JFrog Artifactory. Het team heeft expertise in backend development (Rust), frontend (React/TypeScript), CI/CD, security en platformbeheer.

**Bedrijfsprofiel:** Equans richt zich op technische dienstverlening, energie- en datatransitie en ondersteunt bedrijven en instellingen met innovatieve oplossingen. De stakeholders voor dit project zijn interne development teams, IT-management en de financiële afdeling.

**Bedrijfsbegeleider:** Viktor Klein - Product lead devops Forge ([viktor.klein@equans.com](mailto:viktor.klein@equans.com))

**Telefoon:** +31651621514 

**Technische begeleider:** Brian Veltman– Developer/Platform Engineer

## 3 – Projectoverzicht

**Huidige situatie:** Equans maakt gebruik van Atlassian Cloud Enterprise (Jira Software, Confluence, JSM). Op dit moment is er onvoldoende inzicht in het daadwerkelijke gebruik van licenties, kosten per product en team, en hoeveel accounts inactief zijn.

**Probleemstelling:** Er is geen geautomatiseerde manier om te bepalen hoeveel toegewezen licenties ook daadwerkelijk actief gebruikt worden. Het management wil inzicht in licentiegebruik, kosten per team en de belangrijkste besparingskansen.

**Hoofdvraag:** Hoe kan Equans inzicht krijgen in het gebruik van Atlassian Cloud Enterprise licenties en de bijbehorende kosten, om besparingsmogelijkheden te identificeren en te optimaliseren?

**Deelvragen:**

- Welke data is beschikbaar via de Atlassian Cloud API over gebruikers, licenties en kosten?
- Hoe kan het actieve gebruik van licenties gemeten worden (30/60/90 dagen)?
- Hoe kunnen de kosten per product, site en team inzichtelijk gemaakt worden?
- Welke patronen duiden op onbenut of inefficiënt gebruik (inactieve accounts, externe accounts, teveel admins)?
- Hoe kan deze informatie het beste gepresenteerd worden in een dashboard met duidelijke aanbevelingen?

**Projectbeschrijving:** Het project levert een end-to-end “vertical slice” op waarbij data wordt verzameld via de Atlassian Cloud API (Jira als startpunt), opgeslagen en geaggregeerd in Postgres, ontsloten via een Rust REST-API en gepresenteerd in een React-dashboard met KPI’s, drill-down per team en CSV-export. Het project genereert aanbevelingen voor kostenoptimalisatie.

## 4 – Methodologie en Technologie Stack

**Onderzoeksmethoden:** Interviews met stakeholders (IT, Finance, DevOps), documentanalyse (Atlassian licentievoorwaarden, kostenoverzichten).

**Ontwikkelmethode:** Agile/Scrum (wekelijkse sprint, stand-up, demo & retro).

**Ontwikkel-/deployomgeving**: Lokale Docker-omgeving met docker-compose, CI/CD met GitHub Actions.

**Programmeertalen:** Backend in Rust, Frontend in TypeScript/React.

**Frameworks/Libraries**: Backend met Axum/Actix, sqlx/diesel, frontend met React en een lichte charting library.

**Tools en technologieën:** Atlassian REST API, Postgres, Docker, GitHub Actions, Confluence, Jira.

## 5 – Planning en Mijlpalen

Startdatum: 17-11-2025  
Einddatum: 11-04-2026

**Mijlpalen:**  
\- Week 1–2: Onboarding, scope, data model, privacy/security plan

\- Week 3–4: Analyse van stakeholders en technische vereisten  
\- Week 5–6: Backend, datacollectie + aggregatie, database, basis-API  
\- Week 7–9: Frontend: dashboard MVP, KPI-overzicht  
\- Week 10–12: Hardening, testen, CI/CD, documentatie, validatie met stakeholders

\- Week 13–15: Usability evaluatie en feedback verwerken, testen, documentatie

\- Week 16–18: Fine-tuning, testen, demo  
\- Week 19–20: Einddemo presentatie en afronding

## 6 – Competenties

**Professional Skills & Manage and Control:** Afstudeervoorstel, aangetoond door wekelijkse voortgangsrapporten, planning en aanpassingen, communicatie met begeleiders en gebruik van Scrum-board.

**Analyse:** Analyse de huidige situatie van licentiegebruik, verzamelen van requirements, SRS-documentatie, usability test plan, performance test plan, feedbackformulier van de bedrijfsbegeleiders.

**Advies:** Aanbevelingen formuleren voor kostenoptimalisatie en licentiebeheer, gepresenteerd aan stakeholders, feedbackformulier van de bedrijfsbegeleiders.

**Design:** Ontwerp van datamodel, architectuurdiagram (ERD/UML) en front-end WIR frames, feedbackformulier van de bedrijfsbegeleiders.

**Realisatie:** Implementatie van een werkende oplossing met Rust (backend), React (front-end), Postgres (database), usability testrapport, performance testrapport, CI/CD en bedrijfsbegeleider feedbackformulier.
