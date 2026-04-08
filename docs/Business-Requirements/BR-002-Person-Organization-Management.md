# BR-002: Person-Organization Management

**Status:** Draft
**Datum:** 2026-02-17
**Auteur(s):** Ahmad Alhaj Asaad
**Stakeholders:** Viktor Klein, Brian Veltman, Henk

---

## Probleemstelling

Equans beschikt over twee afzonderlijke datasets — personen (medewerkers) en organisaties — maar er ontbreekt een geïntegreerd overzicht van welke medewerkers tot welke organisatie behoren. Dit leidt tot de volgende problemen:

- **Geen actuele headcount per organisatie:** Het veld `person_count` in de organisatietabel is leeg en wordt niet automatisch bijgewerkt
- **Ontbrekende vendor-koppelingen:** De velden `vendor_identifiers` en `matching_metadata` zijn leeg, waardoor licenties niet aan specifieke medewerkers gekoppeld kunnen worden
- **Handmatige dataverzameling:** Het koppelen van medewerkers aan organisaties vereist tijdrovend handmatig werk
- **Inconsistente rapportage:** Geen betrouwbare basis voor kostentoewijzing per organisatie of business unit

**Wie ervaart dit probleem:**

- Finance teams die kosten willen toewijzen aan organisaties
- HR voor accurate headcount rapportage
- IT voor licentieoptimalisatie per organisatie
- Management voor strategische besluitvorming

---

## Business Waarde

| Voordeel                      | Impact                                                                             |
| ----------------------------- | ---------------------------------------------------------------------------------- |
| **Accurate kostentoewijzing** | Licentiekosten kunnen correct worden doorbelast aan organisaties en business units |
| **Real-time headcount**       | Actueel inzicht in aantal medewerkers per organisatie zonder handmatig werk        |
| **Licentieoptimalisatie**     | Basis voor koppeling van vendor-accounts aan specifieke medewerkers                |
| **Compliance**                | Aantoonbaar overzicht van wie toegang heeft tot welke softwarelicenties            |
| **Tijdsbesparing**            | Eliminatie van handmatige koppelingen en rapportagewerk (geschat 10+ uur/maand)    |
| **Data-integriteit**          | Eén bron van waarheid voor person-organization relaties                            |

---

## Stakeholders

| Stakeholder   | Rol               | Prioriteit Focus                        |
| ------------- | ----------------- | --------------------------------------- |
| Viktor Klein  | Business Owner    | Accurate kostentoewijzing, rapportage   |
| Brian Veltman | Technical Lead    | Data-integriteit, API-koppelingen       |
| Henk          | Executive Sponsor | Executive overzichten per business unit |
| Finance Team  | Eindgebruiker     | Chargeback per organisatie              |
| HR            | Eindgebruiker     | Headcount rapportage                    |

---

## Succescriteria

De oplossing wordt als succesvol beschouwd wanneer:

- [ ] **Person Count Accuraatheid** — `person_count` per organisatie is automatisch berekend en 100% accuraat
- [ ] **Koppelingsdekking** — ≥95% van alle personen is gekoppeld aan een organisatie via `org_id`
- [ ] **Vendor Matching Readiness** — Datastructuur ondersteunt koppeling met externe vendor-accounts
- [ ] **Rapportage Beschikbaarheid** — Overzicht van personen per organisatie is beschikbaar binnen het platform
- [ ] **Automatische Updates** — Wijzigingen in personen of organisaties worden binnen 24 uur gereflecteerd
- [ ] **Drill-down Functionaliteit** — Gebruikers kunnen van organisatie naar individuele medewerkers navigeren

---

## Scope

### Binnen Scope

- Automatische koppeling van personen aan organisaties via `org_id`
- Berekening en bijwerking van `person_count` per organisatie
- Aggregatie per land, billing location en business unit
- Overzichtsweergave van organisaties met gekoppelde personen
- Export mogelijkheid naar CSV/Excel
- Basis voor toekomstige vendor identity matching

### Buiten Scope

- Koppeling met externe vendor APIs (GitHub, Atlassian, JFrog) — zie BR-001
- Handmatige correctie interface voor foutieve koppelingen
- Historische analyse van person-organization wijzigingen
- Real-time synchronisatie (batch-verwerking is voldoende voor MVP)

---

## Afhankelijkheden

| Afhankelijkheid                      | Type                 | Opmerkingen                                            |
| ------------------------------------ | -------------------- | ------------------------------------------------------ |
| Azure AD                             | Externe bron         | Primaire bron voor personen en organisaties            |
| PostgreSQL Database                  | Infrastructuur       | Opslag van gekoppelde data                             |
| BR-001 Multi-Vendor License Insights | Business Requirement | Vendor-koppelingen bouwen voort op person-org relaties |

---

## Data Koppelingsstrategie

### Primaire Koppeling

Het veld `org_id` is de gemeenschappelijke sleutel tussen personen en organisaties:

| Personen Veld | Organisaties Veld | Relatie                                        |
| ------------- | ----------------- | ---------------------------------------------- |
| `org_id`      | `org_id`          | 1-op-veel (één organisatie, meerdere personen) |

### Afgeleide Aggregaties

| Aggregatie        | Bron                      | Doel                                 |
| ----------------- | ------------------------- | ------------------------------------ |
| Person count      | COUNT(persons) per org_id | Vult `person_count` in organisations |
| Per land          | GROUP BY country          | Rapportage per land                  |
| Per business unit | GROUP BY business_unit    | Kostentoewijzing per SSC             |

---

## Gerelateerde Documenten

- Business Requirement: [BR-001-Multi-Vendor-License-Insights](BR-001-Multi-Vendor-License-Insights.md)
- Functional Requirement: FR-003-Person-Organization-Linking (nog aan te maken)
- Functional Requirement: FR-004-Vendor-Identity-Matching (nog aan te maken)
- Functional Requirement: FR-005-Organization-Dashboard (nog aan te maken)
