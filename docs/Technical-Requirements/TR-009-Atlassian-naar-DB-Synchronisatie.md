# TR-009: Atlassian-naar-DB Synchronisatie — Technische Specificaties

**Status:** Draft
**Datum:** 2026-02-24
**Auteur(s):** Backend Engineer Agent
**Van toepassing op:** Backend Rust applicatie, PostgreSQL database
**Gerelateerde FR:** [FR-009-Atlassian-DB-Sync](../Functional-Requirements/FR-009-Atlassian-DB-Sync.md)
**Gerelateerde TR:** [TR-003-Atlassian-Cache](TR-003-Atlassian-Cache.md) · [TR-005-Person-Management](TR-005-Person-Management.md) · [TR-006-Organization-Management](TR-006-Organization-Management.md)

---

## Scope

Dit document beschrijft de technische implementatie voor:

1. **Schema-uitbreiding** van `persons` en `organizations` met Atlassian-koppelvelden
2. **Matching-logica** om Atlassian-accounts te koppelen aan personen
3. **Sync-flow** die koppeling uitvoert na Atlassian-synchronisatie en CSV-import
4. **API endpoints** voor koppelbeheer
5. **Migratiestrategie** voor bestaande data

---

## Architectuuroverzicht

```
┌─────────────────────────┐       ┌─────────────────────────┐
│   CSV Import Pipeline   │       │   Atlassian Sync Job    │
│  (imports/service.rs)   │       │   (jobs/daily_sync.rs)  │
└──────────┬──────────────┘       └──────────┬──────────────┘
           │  na import voltooid             │  na sync voltooid
           ▼                                 ▼
┌─────────────────────────────────────────────────────────┐
│              Link Matching Service                       │
│         (atlassian/link_service.rs)                     │
│                                                         │
│  Stap 1: persons.local_id  ──▶  atlassian.email         │
│          (bijv. CCJ183@equans.com)   ← primaire match   │
│                                                         │
│  Stap 2: persons.email     ──▶  atlassian.email         │
│          (bijv. jan.devries@equans.com)  ← fallback     │
│                                                         │
│  Stap 3: Handmatig via API (altijd beschikbaar)         │
│                                                         │
│  ⚠ person_id (GH5745) ≠ account_id (557058:...)        │
│    Deze velden worden NIET voor matching gebruikt        │
└──────────────────────────────────────────────────────────┘
           │
           ▼
┌─────────────────────────┐       ┌─────────────────────────┐
│   persons tabel          │       │  atlassian_users_cache  │
│  + atlassian_account_id  │◀─────▶│  (account_id, email...) │
│  + atlassian_link_status │       └─────────────────────────┘
│  + atlassian_linked_at   │
│  + local_id (koppelsleutel) │
└─────────────────────────┘
```

---

## Database Schema Wijzigingen

### Migratie: 003_atlassian_person_link.sql

Zie het volledige SQL-bestand: [`migrations/003_atlassian_person_link.sql`](../../backend/migrations/003_atlassian_person_link.sql)

#### Nieuwe kolommen in `persons`

```sql
-- Atlassian koppeling
atlassian_account_id VARCHAR(128) UNIQUE REFERENCES atlassian_users_cache(account_id) ON DELETE SET NULL,
atlassian_link_status VARCHAR(30) NOT NULL DEFAULT 'unlinked',
atlassian_linked_at TIMESTAMPTZ,
atlassian_link_method VARCHAR(30),  -- 'auto_local_id' | 'auto_email' | 'manual'
```

**Constraints:**

- `atlassian_account_id` is UNIQUE: één Atlassian-account per persoon
- `ON DELETE SET NULL`: wanneer een Atlassian-gebruiker uit de cache wordt verwijderd, wordt de koppeling verbroken (niet de persoon)
- `atlassian_link_status` CHECK constraint: `('unlinked', 'linked_auto_local_id', 'linked_auto_email', 'linked_manual', 'no_atlassian_account')`

#### Nieuwe tabel: `organization_atlassian_groups`

```sql
CREATE TABLE IF NOT EXISTS organization_atlassian_groups (
    id SERIAL PRIMARY KEY,
    org_id VARCHAR(20) NOT NULL REFERENCES organizations(org_id) ON DELETE CASCADE,
    group_id VARCHAR(128) NOT NULL REFERENCES atlassian_groups_cache(group_id) ON DELETE CASCADE,
    link_method VARCHAR(30) NOT NULL DEFAULT 'manual',  -- 'auto_name' | 'manual'
    linked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    linked_by VARCHAR(255),
    UNIQUE (org_id, group_id)
);
```

**Reden voor aparte tabel:** Een organisatie kan aan meerdere Atlassian-groepen zijn gekoppeld (many-to-many).

---

## Rust Implementatie

### Nieuwe Module: `atlassian/link_service.rs`

```rust
/// Service responsible for linking persons to Atlassian accounts
/// and organizations to Atlassian groups.
pub struct AtlassianLinkService {
    pool: PgPool,
}

impl AtlassianLinkService {
    /// Attempt to link all unlinked persons to Atlassian accounts.
    /// Called after CSV import and after Atlassian sync.
    /// Step 1: match persons.local_id against atlassian_users_cache.email
    /// Step 2: match persons.email against atlassian_users_cache.email (fallback)
    pub async fn link_all_unlinked(&self) -> Result<LinkStats, LinkError>;

    /// Link a single person using the two-step matching strategy.
    /// Returns None if no Atlassian account could be found.
    pub async fn link_person_by_matching(&self, person_id: &str) -> Result<Option<LinkResult>, LinkError>;

    /// Step 1: match persons.local_id against atlassian_users_cache.email.
    pub async fn link_person_by_local_id(&self, person_id: &str) -> Result<Option<LinkResult>, LinkError>;

    /// Step 2: match persons.email against atlassian_users_cache.email (fallback).
    pub async fn link_person_by_email(&self, person_id: &str) -> Result<Option<LinkResult>, LinkError>;

    /// Manually link a person to a specific Atlassian account_id.
    pub async fn link_person_manual(
        &self,
        person_id: &str,
        account_id: &str,
        linked_by: &str,
    ) -> Result<LinkResult, LinkError>;

    /// Remove the Atlassian link from a person.
    pub async fn unlink_person(&self, person_id: &str) -> Result<(), LinkError>;

    /// Link an organization to an Atlassian group.
    pub async fn link_org_to_group(
        &self,
        org_id: &str,
        group_id: &str,
        linked_by: Option<&str>,
    ) -> Result<(), LinkError>;

    /// Retrieve unlinked Atlassian accounts (in cache but not linked to any person).
    pub async fn get_unlinked_atlassian_users(&self, page: i64, per_page: i64) -> Result<Vec<UnlinkedAtlassianUser>, LinkError>;
}
```

### Statistieken type `LinkStats`

```rust
pub struct LinkStats {
    pub linked_by_local_id: u32,  // Matched via persons.local_id → atlassian.email (step 1)
    pub linked_by_email: u32,     // Matched via persons.email → atlassian.email (step 2)
    pub already_linked: u32,      // Skipped: already had a link
    pub no_match: u32,            // No Atlassian account found via either step
    pub ambiguous: u32,           // Multiple Atlassian accounts matched same address
    pub errors: u32,
}
```

---

## Matching Algoritme

### Achtergrondinformatie: Identifiers vergeleken

| Identiteitsveld   | Bron                       | Voorbeeld                            | Gebruikt voor matching?                                    |
| ----------------- | -------------------------- | ------------------------------------ | ---------------------------------------------------------- |
| `person_id`       | CSV export (Palantir)      | `GH5745`                             | **Nee** — verschilt structureel van Atlassian `account_id` |
| `account_id`      | Atlassian API              | `557058:4598ea15-a483-43c4-98ef-...` | **Nee** — niet aanwezig in CSV                             |
| `local_id`        | CSV veld `person_local_id` | `CCJ183@equans.com`                  | **Ja — stap 1** (primaire match)                           |
| `email`           | CSV veld `person_email`    | `jan.devries@equans.com`             | **Ja — stap 2** (fallback)                                 |
| `atlassian.email` | Atlassian API              | `CCJ183@equans.com`                  | Matchingsdoel voor stap 1 en stap 2                        |

### Stap 1: local_id matching (primaire match)

```sql
-- Find unlinked persons whose local_id matches an email in the Atlassian cache
SELECT p.person_id, p.local_id, a.account_id
FROM persons p
INNER JOIN atlassian_users_cache a
    ON LOWER(p.local_id) = LOWER(a.email)
WHERE p.atlassian_account_id IS NULL
  AND p.atlassian_link_status = 'unlinked'
  AND p.local_id IS NOT NULL
  AND a.active = true;
```

Bij een match: `atlassian_link_status = 'linked_auto_local_id'`, `atlassian_link_method = 'auto_local_id'`

**Conflictbehandeling:**

- Meerdere Atlassian-accounts met zelfde `local_id`-adres: geen automatische koppeling, `ambiguous` teller verhoogd, gelogd

### Stap 2: email matching (fallback)

Alleen uitgevoerd als stap 1 géén match heeft opgeleverd (`atlassian_account_id IS NULL` na stap 1).

```sql
-- Fallback: match persons.email against Atlassian email for still-unlinked persons
SELECT p.person_id, p.email, a.account_id
FROM persons p
INNER JOIN atlassian_users_cache a
    ON LOWER(p.email) = LOWER(a.email)
WHERE p.atlassian_account_id IS NULL
  AND p.atlassian_link_status = 'unlinked'
  AND p.email IS NOT NULL
  AND a.active = true;
```

Bij een match: `atlassian_link_status = 'linked_auto_email'`, `atlassian_link_method = 'auto_email'`

**Conflictbehandeling:**

- Meerdere Atlassian-accounts met zelfde `email`-adres: geen automatische koppeling, `ambiguous` teller verhoogd, gelogd

### Stap 3: Status bijwerken

```sql
UPDATE persons
SET
    atlassian_account_id = $1,
    atlassian_link_status = $2,  -- 'linked_auto_local_id' | 'linked_auto_email' | 'linked_manual'
    atlassian_linked_at = NOW(),
    atlassian_link_method = $3,  -- 'auto_local_id' | 'auto_email' | 'manual'
    updated_at = NOW()
WHERE person_id = $4;
```

---

## API Endpoints

### GET `/api/persons/:id/atlassian`

Geeft de Atlassian-koppelstatus en gegevens terug van een persoon.

**Response:**

```json
{
  "person_id": "CCJ183",
  "atlassian_account_id": "5e7b9a3c8f1d2e4a6b0c",
  "atlassian_link_status": "linked_auto_email",
  "atlassian_linked_at": "2026-02-24T08:00:00Z",
  "atlassian_link_method": "auto_email",
  "atlassian_profile": {
    "account_id": "5e7b9a3c8f1d2e4a6b0c",
    "display_name": "Jan de Vries",
    "email": "jan.devries@equans.com",
    "account_status": "active",
    "active": true,
    "last_active": "2026-02-24T14:30:00Z",
    "access_billable": true,
    "product_access": ["jira-software", "confluence"]
  }
}
```

**Note:** Indien niet gekoppeld is `atlassian_account_id` null en `atlassian_profile` null.

---

## Integratiepunten

### Na CSV-import (`imports/service.rs`)

Na succesvolle importafronding roept `execute_import` de `AtlassianLinkService` aan:

```rust
// In execute_import, after database writes complete:
if let Err(e) = link_service.link_all_unlinked().await {
    tracing::warn!("Atlassian link matching after import failed: {}", e);
    // Non-fatal: import itself is already completed
}
```

### Na Atlassian-synchronisatie (`jobs/daily_sync.rs`)

Na elke succesvolle Atlassian-sync worden ongekoppelde personen opnieuw geprobeerd:

```rust
// In daily_sync.rs, after sync_users completes:
match link_service.link_all_unlinked().await {
    Ok(stats) => tracing::info!(
        "Atlassian link matching: linked={}, no_match={}, ambiguous={}",
        stats.linked, stats.no_match, stats.ambiguous
    ),
    Err(e) => tracing::warn!("Atlassian link matching failed: {}", e),
}
```

---

## Error Handling

| Fout                                            | Gedrag                                            |
| ----------------------------------------------- | ------------------------------------------------- |
| `account_id` bestaat niet in cache              | `400 Bad Request` met foutmelding                 |
| `account_id` al gekoppeld aan andere persoon    | `409 Conflict`                                    |
| Database-fout bij koppeling                     | `500 Internal Server Error`, fout gelogd          |
| Atlassian-cache verlopen tijdens matching       | Matching overgeslagen, gelogd als waarschuwing    |
| Meerdere Atlassian-gebruikers met zelfde e-mail | Overgeslagen, `ambiguous` teller verhoogd, gelogd |

---

## Logging

Alle koppeloperaties worden gelogd met `tracing`:

```rust
tracing::info!(
    person_id = %person_id,
    account_id = %account_id,
    method = %link_method,
    "Atlassian account linked to person"
);

tracing::warn!(
    person_id = %person_id,
    email = %email,
    matches = matches.len(),
    "Ambiguous Atlassian email match, skipping auto-link"
);
```

---

## Performance Overwegingen

| Aspect                                  | Aanpak                                                          |
| --------------------------------------- | --------------------------------------------------------------- |
| Bulk matching bij grote imports         | Gebruik `SELECT ... JOIN` in één query i.p.v. row-by-row        |
| Index op `atlassian_users_cache.email`  | Reeds aanwezig in migratie 001                                  |
| Index op `persons.atlassian_account_id` | Toegevoegd in migratie 003                                      |
| Maximum matchingsduur                   | Verwacht < 5 seconden voor 100K+ personen via geïndexeerde JOIN |

---

## Security

- `atlassian_account_id` wordt **nooit** teruggegeven aan niet-geauthenticeerde clients
- Handmatige koppeling vereist authenticatie (Bearer token, zie FR-004/TR-004)
- `account_id` waarden worden gevalideerd tegen de lokale cache, geen externe API-aanroepen bij koppeloperaties

---

## Testplan

### Unit tests (`backend/src/atlassian/link_service.rs`)

- [ ] `link_person_by_local_id` koppelt persoon bij exacte `local_id`-match op Atlassian `email`
- [ ] `link_person_by_local_id` slaat over bij geen match (returns `None`)
- [ ] `link_person_by_local_id` slaat over bij meerdere matches (ambiguous)
- [ ] `link_person_by_email` koppelt persoon bij exacte `email`-match op Atlassian `email` (fallback)
- [ ] `link_person_by_email` slaat over bij geen match (returns `None`)
- [ ] `link_person_by_matching` probeert stap 1, dan stap 2; stopt bij eerste match
- [ ] `link_person_by_matching` gebruikt `linked_auto_local_id` als stap 1 slaagt
- [ ] `link_person_by_matching` gebruikt `linked_auto_email` als alleen stap 2 slaagt
- [ ] `get_person_atlassian_link` retourneert correcte koppelstatus en Atlassian-gegevens
- [ ] `link_all_unlinked` rapporteert correcte statistieken (`linked_by_local_id`, `linked_by_email`, `no_match`)

### Integratietests (`backend/tests/`)

- [ ] Persoon met `local_id = CCJ183@equans.com` wordt gekoppeld aan Atlassian-account met `email = CCJ183@equans.com`
- [ ] Persoon zonder `local_id`-match maar met `email`-match wordt gekoppeld via stap 2
- [ ] Persoon zonder beide matches blijft `unlinked`
- [ ] GET `/api/persons/:id/atlassian` → 200 met koppelstatus en Atlassian-gegevens (product_access, last_active, account_status, access_billable)
- [ ] GET `/api/persons/:id/atlassian` → 200 met lege Atlassian-gegevens indien niet gekoppeld
- [ ] Na import: automatische koppeling via stap 1 en stap 2 wordt uitgevoerd
- [ ] Person detail response bevat Atlassian-gegevensvelden

---

## Afhankelijkheden

| Component    | Versie / Crate | Reden                                         |
| ------------ | -------------- | --------------------------------------------- |
| `sqlx`       | 0.8            | Async PostgreSQL queries                      |
| `tracing`    | huidig         | Gestructureerde logging                       |
| `axum`       | 0.7            | HTTP endpoints                                |
| Migratie 001 | —              | `atlassian_users_cache` tabel vereist         |
| Migratie 002 | —              | `persons` en `organizations` tabellen vereist |
| Migratie 003 | —              | Koppelvelden in `persons`                     |
