# FR-010: Frontend Vernieuwing – Operational Insights Dashboard

**Status:** Implemented
**Datum:** 2026-03-16
**Auteur(s):** Ahmad Alhaj Asaad
**Gerelateerde BR:** [BR-001-Multi-Vendor-License-Insights](../Business-Requirements/BR-001-Multi-Vendor-License-Insights.md)
**Implementeert vervanging van:** Oorspronkelijke Vite/React frontend (frontend/)

---

## Samenvatting

Deze requirement beschrijft de vervanging van de bestaande frontend door een vernieuwde React-applicatie met verbeterd licentie-dashboard, moderne UI-componenten, en uitgebreide integratie met de Rust backend. De nieuwe frontend introduceert product-specifieke kaarten per Atlassian-product (Jira Software, Confluence, Trello) met inzicht in kosten, factureerbare tarieven en consultancy-marges.

---

## User Stories

### US-1: Atlassian Licentie Kosten Dashboard

**Als een** Finance Medewerker / License Administrator
**Wil ik** een overzichtelijk dashboard zien met kosten per Atlassian-product
**Zodat** ik de inkoop- en factureerbare kosten per product in één oogopslag kan vergelijken

**Acceptatiecriteria:**

- [ ] Dashboard toont één kaart per Atlassian-product: Jira Software, Confluence, en Trello
- [ ] Per kaart wordt getoond:
  - Naam van het product
  - Aantal actieve gebruikers opgehaald via de backend
  - Inkoopprijs per gebruiker per maand (€)
  - Factureerbaar tarief per gebruiker per maand (€)
  - Consultancy marge per gebruiker (billable − cost) in € en %
  - Totale maandelijkse inkoopkosten (gebruikers × kostprijs)
  - Totaal factureerbaar per maand (gebruikers × factureerbaar tarief)
  - Totale consultancy-marge per maand
- [ ] Totaalrij onderaan toont geaggregeerde waarden over alle producten
- [ ] Valuta wordt getoond in € (EUR, nl-NL locale)

### US-2: Productprijzen Configuratie

**Als een** Beheerder
**Wil ik** de inkoopprijzen en factureerbare tarieven per product kunnen aanpassen
**Zodat** de dashboardberekeningen altijd de actuele contractprijzen weerspiegelen

**Acceptatiecriteria:**

- [ ] Productprijzen zijn gecentraliseerd in één configuratiebestand (`config/productPricing.ts`)
- [ ] Huidige geconfigureerde tarieven zijn:
  | Product                  | Inkoopprijs/gebruiker | Factureerbaar/gebruiker | Consultancy-marge |
  |--------------------------|-----------------------|-------------------------|------------------|
  | Jira Software            | € 8,55                | € 11,50                 | € 2,95 (34,5%)   |
  | Confluence               | € 6,40                | €  9,25                 | € 2,85 (44,5%)   |
  | Trello                   | € 5,50                | €  7,25                 | € 1,75 (31,8%)   |
  | Jira Service Management  | € 7,00                | €  9,50                 | € 2,50 (35,7%)   |
- [ ] Aanpassen van tarieven in het configuratiebestand wordt direct reflecteerd in het dashboard zonder codewijzigingen elders

### US-3: Gebruikers Overzicht

**Als een** License Administrator
**Wil ik** een tabeloverzicht van alle personen zien
**Zodat** ik snel kan controleren welke medewerkers actief zijn en aan welke organisatie ze zijn gekoppeld

**Acceptatiecriteria:**

- [ ] Osobentabel toont: naam, e-mail, organisatie, GID-matching status
- [ ] Zoekfunctionaliteit op naam en e-mail
- [ ] Paginering (25 items per pagina standaard)
- [ ] Klikbaar naar detailpagina per persoon

### US-4: Organisaties Overzicht

**Als een** Teammanager
**Wil ik** een overzicht van alle organisaties zien met hun licentiegebruik
**Zodat** ik chargeback-rapportages per afdeling kan opstellen

**Acceptatiecriteria:**

- [ ] Organisatietabel toont: naam, aantal personen, land, billing location
- [ ] Doorzoekbaar op organisatienaam
- [ ] Navigatie naar detail per organisatie

### US-5: Data Importeren

**Als een** IT Administrator
**Wil ik** via de UI Atlassian- en GitHub-gebruikersdata kunnen importeren
**Zodat** het dashboard altijd actuele gebruikersaantallen toont

**Acceptatiecriteria:**

- [ ] Import-wizard ondersteunt JSON-bestand upload
- [ ] Voortgangsindicator toont import status
- [ ] Importresultaat toont aantal toegevoegde/bijgewerkte/overgeslagen records
- [ ] Importfouten worden overzichtelijk getoond


---

## Scope – BUITEN deze requirement

- GitHub licentie-dashboard (onderdeel van FR-001)
- JFrog integratie (toekomstige sprint)
- Rol-gebaseerde toegangsbeheer (FR-004)
- Notificaties en alertering

---

## Prioriteit & Afhankelijkheden

| Item | Waarde |
|------|--------|
| Prioriteit | MUST HAVE |
| Afhankelijkheden | FR-003 (Atlassian Cache), FR-005 (Persons), FR-006 (Orgs) |
| Geblokkeerd door | Backend endpoints `/api/atlassian/licenses/*` |

---

## Demo Data (Testomgeving)

Bij afwezigheid van een live backend-verbinding toont de frontend fallback demodata:

```
Jira Software : 42 gebruikers × €8,55  = €359,10 kosten / €483,00 factureerbaar
Confluence    : 35 gebruikers × €6,40  = €224,00 kosten / €323,75 factureerbaar
Trello        : 28 gebruikers × €5,50  = €154,00 kosten / €203,00 factureerbaar
────────────────────────────────────────────────────────────────────────────────
TOTAAL        :                           €737,10 kosten / €1.009,75 factureerbaar
Consultancy marge: €272,65/maand
```
