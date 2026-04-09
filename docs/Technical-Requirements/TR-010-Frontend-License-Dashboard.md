# TR-010: Frontend Vernieuwing – Technische Specificaties

**Status:** Implemented
**Datum:** 2026-02-26
**Auteur(s):** Ahmad Alhaj Asaad
**Implementeert:** [FR-010](../Functional-Requirements/FR-010-Frontend-License-Dashboard.md)
**Applies To:** React/TypeScript frontend, Vite build, Rust/Axum backend API

---

## Scope

Dit document beschrijft de technische implementatie van de vernieuwde frontend voor het Operational Insights License Dashboard. Het omvat:

- Componentstructuur en bestandsindeling
- TypeScript type-definities
- API-integratie met de Rust backend
- Productprijs-configuratie
- Build- en deploymentconfiguratie
- Testbenadering

---

## Architectuuroverzicht

```
┌─────────────────────────────────────────────────────────┐
│                  React Frontend (Vite)                   │
│                                                         │
│  App.tsx (routing via useState)                         │
│    ├── LicenseDashboard.tsx      ← FR-010 kernpagina    │
│    │     ├── ProductCard.tsx     ← kaart per product    │
│    │     └── TotaalRij.tsx       ← geaggregeerde totalen│
│    ├── PersonsPage.tsx           ← FR-005               │
│    ├── OrganizationsPage.tsx     ← FR-006               │
│    ├── ImportPage.tsx            ← FR-007               │
│    └── BackendStatus.tsx         ← health check         │
│                                                         │
│  config/productPricing.ts  ← gecentraliseerde prijzen   │
│  api/client.ts             ← fetch wrapper              │
│  types/atlassian.ts        ← TypeScript interfaces      │
└─────────────────────────────────────────────────────────┘
                        │ fetch /api/*
                        ▼
┌─────────────────────────────────────────────────────────┐
│              Rust Backend (Axum)  :8080                  │
│   /api/health                                           │
│   /api/atlassian/organizations                          │
│   /api/atlassian/organizations/:id/licenses/:product    │
│   /api/persons                                          │
│   /api/organizations                                    │
└─────────────────────────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────┐
│              PostgreSQL (via docker-compose)             │
└─────────────────────────────────────────────────────────┘
```

---

## Componentenstructuur

```
frontend/
├── index.html
├── package.json
├── vite.config.ts                 # proxy /api → http://backend:8080
├── tsconfig.json
├── src/
│   ├── main.tsx                   # React root mount
│   ├── App.tsx                    # Navigatie + pagina-routing
│   ├── App.css
│   ├── index.css
│   ├── config/
│   │   └── productPricing.ts      # ProductPricing interface + tarieven
│   ├── types/
│   │   ├── atlassian.ts           # AtlassianOrg, LicenseCount, LicenseCostSummary
│   │   ├── person.ts              # Person types
│   │   ├── organization.ts        # Organization types
│   │   ├── import.ts              # Import types
│   │   └── index.ts               # Re-exports
│   ├── api/
│   │   └── client.ts              # fetchApi<T> + sub-clients
│   ├── pages/
│   │   ├── LicenseDashboard.tsx   # Main licentie kostenoverzicht (FR-010)
│   │   ├── PersonsPage.tsx        # Personenoverzicht (FR-005)
│   │   ├── PersonDetailPage.tsx   # Persoon detail (FR-005)
│   │   ├── OrganizationsPage.tsx  # Organisatiesoverzicht (FR-006)
│   │   ├── ImportPage.tsx         # Import wizard (FR-007)
│   │   ├── ImportWizardSimple.tsx
│   │   └── QuickImport.tsx
│   ├── components/
│   │   ├── BackendStatus.tsx      # /api/health polling
│   │   └── imports/
│   │       ├── ImportHistory.tsx
│   │       ├── ImportPreview.tsx
│   │       ├── ImportProgress.tsx
│   │       ├── ImportStats.tsx
│   │       └── ImportUpload.tsx
│   ├── hooks/
│   │   └── useImport.ts
│   └── services/
│       └── importService.ts
```

---

## TypeScript Interfaces

### `config/productPricing.ts`

```typescript
export interface ProductPricing {
  name: string;           // Weergavenaam
  product: string;        // API-sleutel (jira-software, confluence, trello)
  costPerUser: number;    // Inkoopprijs per gebruiker/maand (€)
  billablePerUser: number;// Factureerbaar tarief per gebruiker/maand (€)
  margin: number;         // billablePerUser - costPerUser
}

export const ATLASSIAN_PRODUCTS: ProductPricing[] = [
  { name: 'Jira Software', product: 'jira-software',
    costPerUser: 8.50, billablePerUser: 11.00, margin: 2.50 },
  { name: 'Confluence',    product: 'confluence',
    costPerUser: 6.25, billablePerUser:  9.00, margin: 2.75 },
  { name: 'Trello',        product: 'trello',
    costPerUser: 4.50, billablePerUser:  6.00, margin: 1.50 },
];
```

### `types/atlassian.ts` – relevante interfaces

```typescript
export interface AtlassianOrg     { id: string; name: string; }
export interface LicenseCount     {
  product: string; org_id: string;
  total_users: number; billable_users: number; non_billable_users: number;
}
export interface LicenseCostSummary {
  product: string; productName: string; userCount: number;
  costPerUser: number; billablePerUser: number;
  totalCost: number; totalBillable: number;
  totalMargin: number; marginPercentage: string;
}
```

---

## API Endpoints (Backend)

| Methode | Pad                                                        | Beschrijving                          | Response type           |
|---------|------------------------------------------------------------|---------------------------------------|-------------------------|
| GET     | `/api/health`                                              | Backend bereikbaarheidcheck           | `{ status: "ok" }`      |
| GET     | `/api/atlassian/organizations`                             | Alle Atlassian-organisaties           | `AtlassianOrg[]`        |
| GET     | `/api/atlassian/organizations/:id/licenses/:product`       | Licentieaantallen per org+product     | `LicenseCount`          |
| GET     | `/api/atlassian/organizations/:id/licenses/:product/details` | Gedetailleerde gebruikerslijst      | `LicenseDetails`        |
| GET     | `/api/atlassian/users`                                     | Alle Atlassian-gebruikers (gepagineerd)| `AtlassianUsersResponse`|
| GET     | `/api/persons`                                             | Personenlijst (gepagineerd + filter)  | `PaginatedResponse<PersonSummary>` |
| GET     | `/api/persons/:id`                                         | Persoon detail                        | `PersonDetail`          |
| GET     | `/api/organizations`                                       | Organisatielijst                      | `PaginatedResponse<OrganizationSummary>` |

Alle endpoints vereisen de header `Content-Type: application/json`. Authenticatie wordt afgehandeld via de sessie/token middleware (zie FR-004).

---

## Vite Proxy Configuratie

```typescript
// vite.config.ts
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
      },
    },
  },
});
```

---

## Productprijs Berekeningslogica

De kosten worden in de frontend berekend (niet door de backend) op basis van het aantal gebruikers dat door de backend wordt gerapporteerd:

```
totalCost      = costPerUser    × userCount
totalBillable  = billablePerUser × userCount
totalMargin    = margin          × userCount
marginPct      = (margin / costPerUser) × 100
```

Fallback bij API-fout: `userCount = 0`, dashboard toont € 0,00 voor dat product.

---

## Foutafhandeling

| Scenario | Gedrag |
|----------|--------|
| Backend niet bereikbaar | BackendStatus toont rood; dashboard toont "Backend niet beschikbaar" |
| Geen Atlassian-organisaties | Melding "Geen organisaties gevonden" getoond |
| Licentie API fout (4xx/5xx) | ProductCard toont 0 gebruikers + waarschuwingsbadge |
| Network timeout | Automatische retry na 5 seconden (max 3 pogingen) |

---

## Build & Deploymentconfiguratie

### Vereisten

| Pakket | Versie |
|--------|--------|
| Node.js | ≥ 20 LTS |
| npm | ≥ 10 |
| TypeScript | ~5.9.x |
| Vite | ^7.x |
| React | ^19.x |

### Scripts

```bash
npm install          # Afhankelijkheden installeren
npm run dev          # Ontwikkelserver starten op poort 5173
npm run build        # Productie-build naar dist/
npm run lint         # ESLint + TypeScript-controle
npm run preview      # Preview van productie-build
```

### Docker Compose (infra/docker-compose.yml)

De frontend kan als container draaien. De Vite dev-proxy stuurt `/api`-verzoeken door naar de backend-container op poort `8080`.

---

## Testing Aanpak

### Handmatige integratietests

1. Start docker-compose: `docker-compose -f infra/docker-compose.yml up`
2. Open `http://localhost:5173`
3. Navigeer naar **Licenties** – dashboard laadt producten en berekent kosten
4. Wijzig organisatiedropdown – gebruikersaantallen worden herladen
5. Check BackendStatus badge – groen bij actieve backend

### Acceptatietestscenario's (FR-010)

| Test | Stap | Verwacht resultaat |
|------|------|-------------------|
| T-010-01 | Open dashboard zonder backend | BackendStatus: rood; Kostenoverzicht: "Backend niet beschikbaar" |
| T-010-02 | Open dashboard met backend | Drie productkaarten geladen (Jira, Confluence, Trello) |
| T-010-03 | Selecteer organisatie in dropdown | Gebruikersaantallen herladen per product |
| T-010-04 | Wijzig prijs in `productPricing.ts` + rebuild | Dashboard toont bijgewerkte berekeningen |
| T-010-05 | Controleer totaalrij | Som van drie producten klopt met individuele kaarten |
| T-010-06 | Controleer valutaformaat | Bedragen getoond als `€ 8,50` (nl-NL locale) |

---

## Prestatievereisten

| Metriek | Doel |
|---------|------|
| Tijd tot eerste render (FCP) | < 1,5 seconden op LAN |
| API response afhandeling | < 500 ms voor licentieaantallen |
| Bundle grootte (gzip) | < 300 KB |
| Lighthouse Performance | ≥ 85 |

---

## Bekende Beperkingen / Toekomstig Werk

- Tarieven zijn statisch geconfigureerd; toekomstige sprint voegt beheerinterface toe (FR-010/US-2)
- Realtime updates via WebSocket zijn niet geïmplementeerd; polling elke 30s
- GitHub- en JFrog-producten worden in latere sprints toegevoegd aan het kostenoverzicht
