# FR-003: Atlassian API Cache

**Status:** Draft
**Date:** 2026-02-16
**Author(s):** Ahmad Alhaj Asaad  
**Related BR:** [BR-001-Multi-Vendor-License-Insights](../Business-Requirements/BR-001-Multi-Vendor-License-Insights.md)

---

## Doel

Het systeem moet data van de Atlassian Cloud API ophalen en beschikbaar stellen aan de frontend, waarbij de data lokaal gecached wordt in PostgreSQL om de belasting op de Atlassian API te minimaliseren.

---

## User Stories

### US-1: Bekijk Atlassian Gebruikers

**As a** License Administrator
**I want to** een lijst van alle Atlassian gebruikers zien
**So that** ik kan analyseren wie toegang heeft tot Atlassian producten

### US-2: Bekijk Atlassian Groepen

**As a** Team Manager
**I want to** alle Atlassian groepen en hun leden zien
**So that** ik kan verifiëren of de juiste mensen in de juiste groepen zitten

### US-3: Forceer Data Verversing

**As a** System Administrator
**I want to** geforceerd verse data ophalen van Atlassian
**So that** ik direct de meest actuele informatie kan bekijken na wijzigingen

### US-4: Bekijk Cache Status

**As a** Operations Engineer
**I want to** zien wanneer de data voor het laatst is gesynchroniseerd
**So that** ik weet hoe actueel de getoonde informatie is

---

## Functionele Requirements

### FR-003.1: Data Ophalen uit Cache

**Beschrijving:** Het systeem haalt standaard data op uit de lokale PostgreSQL cache.

**Acceptatiecriteria:**

- [ ] Wanneer een gebruiker data opvraagt via de API, wordt eerst de cache geraadpleegd
- [ ] Indien geldige cache aanwezig is, wordt deze data geretourneerd
- [ ] De response bevat metadata over de cache status (wanneer gecached, wanneer verloopt)
- [ ] Cache requests worden binnen 100ms afgehandeld

---

### FR-003.2: Dagelijkse Synchronisatie

**Beschrijving:** Het systeem synchroniseert automatisch 1x per dag alle Atlassian data.

**Acceptatiecriteria:**

- [ ] Elke 24 uur wordt automatisch verse data opgehaald van Atlassian
- [ ] De synchronisatie draait op de achtergrond zonder impact op gebruikers
- [ ] Bij fouten tijdens synchronisatie blijft de bestaande cache beschikbaar
- [ ] Synchronisatie activiteit wordt gelogd met timestamp en resultaat
- [ ] Cache TTL is 25 uur (iets langer dan sync interval als buffer)

---

### FR-003.3: Geforceerde Cache Bypass

**Beschrijving:** Elke API endpoint ondersteunt een optie om de cache te omzeilen.

**Acceptatiecriteria:**

- [ ] Door `?force_refresh=true` toe te voegen aan een request wordt verse data opgehaald
- [ ] Na een geforceerde refresh wordt de cache bijgewerkt met de nieuwe data
- [ ] De response geeft aan dat de data vers is opgehaald (`cached: false`)
- [ ] Force refresh is beschikbaar voor alle Atlassian endpoints

---

### FR-003.4: Beschikbare Data Endpoints

**Beschrijving:** De volgende Atlassian data wordt beschikbaar gesteld via de API.

| Endpoint                           | Data         | Beschrijving                               |
| ---------------------------------- | ------------ | ------------------------------------------ |
| `GET /api/atlassian/users`         | Gebruikers   | Alle Atlassian gebruikers met account info |
| `GET /api/atlassian/groups`        | Groepen      | Alle Atlassian groepen                     |
| `GET /api/atlassian/organizations` | Organisaties | Organisatie informatie                     |

**Acceptatiecriteria:**

- [ ] Elk endpoint retourneert data in JSON formaat
- [ ] Elk endpoint ondersteunt de `?force_refresh=true` query parameter
- [ ] Elk endpoint bevat cache metadata in de response

---

### FR-003.5: Cache Metadata in Response

**Beschrijving:** Elke API response bevat informatie over de cache status.

**Response structuur:**

```json
{
  "data": [...],
  "cache": {
    "cached": true,
    "cached_at": "2026-02-15T08:00:00Z",
    "expires_at": "2026-02-16T09:00:00Z"
  }
}
```

**Acceptatiecriteria:**

- [ ] Response bevat veld `cached` (boolean): of data uit cache komt
- [ ] Response bevat veld `cached_at` (ISO 8601 timestamp): wanneer data gecached is
- [ ] Response bevat veld `expires_at` (ISO 8601 timestamp): wanneer cache verloopt

---

### FR-003.6: Fallback bij API Fouten

**Beschrijving:** Bij Atlassian API fouten wordt verlopen cache data als fallback gebruikt.

**Acceptatiecriteria:**

- [ ] Het systeem blijft beschikbaar ook als de Atlassian API niet bereikbaar is
- [ ] Verlopen cache data wordt geretourneerd met een waarschuwing
- [ ] API fouten worden gelogd voor monitoring
- [ ] Gebruikers worden niet geblokkeerd door externe API problemen

---

## Gebruikers Scenario's

### Scenario 1: Normale Data Opvraag

```
Gebruiker vraagt GET /api/atlassian/users op
→ Systeem checkt PostgreSQL cache
→ Cache is geldig (niet verlopen)
→ Data wordt uit cache geretourneerd
→ Response bevat: cached=true, cached_at, expires_at
```

### Scenario 2: Geforceerde Refresh

```
Gebruiker vraagt GET /api/atlassian/users?force_refresh=true op
→ Systeem haalt data op van Atlassian Cloud API
→ Nieuwe data wordt opgeslagen in PostgreSQL cache
→ Verse data wordt geretourneerd
→ Response bevat: cached=false, cached_at (nu), expires_at (+25 uur)
```

### Scenario 3: Verlopen Cache

```
Gebruiker vraagt GET /api/atlassian/users op
→ Systeem checkt PostgreSQL cache
→ Cache is verlopen (>25 uur oud)
→ Systeem haalt data op van Atlassian Cloud API
→ Verse data wordt geretourneerd en gecached
→ Response bevat: cached=false
```

### Scenario 4: Atlassian API Onbereikbaar

```
Dagelijkse sync start
→ Atlassian API is niet bereikbaar (timeout/error)
→ Fout wordt gelogd met details
→ Bestaande cache blijft intact
→ Gebruikers kunnen nog steeds (oudere) data opvragen
→ Volgende sync poging over 24 uur
```

### Scenario 5: Eerste Opstart (Lege Cache)

```
Applicatie start op
→ Systeem detecteert lege cache
→ Initiële sync wordt automatisch gestart
→ Data wordt opgehaald van Atlassian API
→ Cache wordt gevuld
→ Background sync job wordt gestart (elke 24 uur)
```

---

## Prioritering (MoSCoW)

### Must Have

- [ ] FR-003.1: Data ophalen uit cache
- [ ] FR-003.2: Dagelijkse synchronisatie
- [ ] FR-003.3: Geforceerde cache bypass
- [ ] FR-003.4: Users en Groups endpoints
- [ ] FR-003.5: Cache metadata in response

### Should Have

- [ ] FR-003.6: Fallback bij API fouten
- [ ] Organizations endpoint

### Could Have

- [ ] Cache status dashboard endpoint
- [ ] Handmatige trigger voor sync via admin UI

### Won't Have (this release)

- [ ] Real-time webhooks van Atlassian
- [ ] Incrementele sync (alleen wijzigingen)

---

## Afhankelijkheden

- PostgreSQL database moet beschikbaar zijn
- Atlassian Cloud API credentials (email + API token)
- Network toegang tot Atlassian Cloud API
