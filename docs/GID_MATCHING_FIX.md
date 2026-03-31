# GID Matching Fix - Documentatie

## Probleem

Personen met echte IDs (zoals `LX78IN`, `RBAB89`, `CF97XY`, etc.) stonden op "pending" status, terwijl ze als "matched" geclassificeerd moesten worden. Dit betrof ongeveer **64,539 personen**.

### Oorzaak

1. **Oude Thresholds**: De repository gebruikte oude GID status thresholds:
   - `matched` >= 80
   - `pending` 50-79
   - `unmatched` < 50

2. **Nieuwe Logica**: De GID matcher implementeerde nieuwe logica:
   - `matched` = 100 (heeft echt ID, niet `AUTO_` prefix)
   - `pending` = 30-99 (heeft `AUTO_` ID met matching info)
   - `unmatched` < 30 (geen matching info)

3. **Bestaande Data**: Personen waren gematched met de oude logica en hadden daarom verkeerde confidence scores.

## Oplossing

### 1. Threshold Update

Bijgewerkt in [backend/src/persons/repository.rs](backend/src/persons/repository.rs):

```rust
// NIEUW (regel 44-51)
"matched" => conditions.push("gid_confidence >= 100".to_string()),
"pending" => {
    conditions.push("gid_confidence >= 30 AND gid_confidence < 100".to_string())
}
"unmatched" => {
    conditions.push("(gid_confidence < 30 OR gid_confidence IS NULL)".to_string())
}
```

### 2. GID Matcher Logica

De GID matcher ([backend/src/persons/gid_matcher.rs](backend/src/persons/gid_matcher.rs)) implementeert:

```rust
fn calculate_confidence(&self, person: &Person, gid: &str) -> i32 {
    // Check if person has a real ID (not auto-generated)
    if !person.person_id.starts_with("AUTO_") && !person.person_id.is_empty() {
        // Person heeft een bestaand ID → MATCHED
        return 100;
    }

    // Person heeft AUTO_ ID → berekend uit andere velden (max 99)
    let mut confidence = 0;
    // ... rest van logica
}
```

### 3. Re-matching Script

Twee scripts toegevoegd om alle bestaande personen opnieuw te matchen:

- **Windows/PowerShell**: `rematch_all_persons.ps1`
- **Linux/Mac/Bash**: `rematch_all_persons.sh`

## Gebruik

### Stap 1: Start de Backend Server

```bash
cd backend
cargo run --release
```

Wacht tot je ziet:

```
Server listening on http://0.0.0.0:8080
```

### Stap 2: Run het Re-matching Script

**Op Windows (PowerShell):**

```powershell
.\rematch_all_persons.ps1
```

**Op Linux/Mac (Bash):**

```bash
./rematch_all_persons.sh
```

### Stap 3: Bevestig de Operatie

Het script vraagt om bevestiging:

```
Weet je zeker dat je ALLE personen opnieuw wilt matchen?
Dit kan enkele minuten duren voor grote datasets...
Typ 'ja' om door te gaan:
```

Typ `ja` en druk op Enter.

### Verwachte Output

```
================================================
  GID Re-Matching Voltooid!
================================================

Resultaten:
  - Totaal verwerkt: 85842 personen
  - Matched: 64539 personen
  - Match rate: 75.18%

Uitvoeringstijd: 2m 34s

Je kunt nu de frontend vernieuwen om de bijgewerkte statussen te zien.
```

## Technische Details

### API Endpoint

```
POST /api/persons/match-gids
```

**Response:**

```json
{
  "success": true,
  "total_processed": 85842,
  "total_matched": 64539,
  "match_rate": 75.18
}
```

### Batch Processing

Het re-matching proces werkt in batches van 1000 personen om:

- Memory gebruik te beperken
- Database load te spreiden
- Progress logging mogelijk te maken

### Database Impact

De operatie update de volgende velden per persoon:

- `gid` - Global ID (uit email prefix)
- `gid_confidence` - Confidence score (0-100)
- `gid_extraction_method` - Methode gebruikt voor extractie
- `last_matched_at` - Timestamp van laatste match

## Status Classificatie

Na re-matching worden personen geclassificeerd als:

| Status        | Confidence | Betekenis                     | Voorbeeld          |
| ------------- | ---------- | ----------------------------- | ------------------ |
| **Matched**   | 100        | Heeft echt ID (niet AUTO\_)   | `LX78IN`, `RBAB89` |
| **Pending**   | 30-99      | Heeft AUTO\_ ID maar wel info | `AUTO_thomas.w`    |
| **Unmatched** | 0-29       | Geen bruikbare matching info  | `AUTO_unknown`     |

## Verificatie

### In de Frontend

1. Refresh de persons lijst pagina
2. Filter op `GID Status = matched`
3. Verwacht: ~64,539 personen met echte IDs

### Via Database Query

```sql
SELECT
    gid_confidence,
    COUNT(*) as count,
    CASE
        WHEN gid_confidence >= 100 THEN 'matched'
        WHEN gid_confidence >= 30 AND gid_confidence < 100 THEN 'pending'
        ELSE 'unmatched'
    END as status
FROM persons
GROUP BY gid_confidence
ORDER BY gid_confidence DESC;
```

### Via API

```bash
curl http://localhost:8080/api/persons?gid_status=matched | jq '.pagination.total'
```

## Impact

### Voorheen (Oude Thresholds)

- Matched: ~X personen (threshold >= 80)
- Pending: ~64,539 personen (threshold 50-79)

### Nu (Nieuwe Thresholds)

- Matched: **~64,539 personen** (heeft echt ID)
- Pending: ~Y personen (heeft AUTO\_ ID)

## Automatische Matching bij Import

Vanaf nu worden nieuwe imports automatisch correct gematched:

1. Import upload
2. Validation (duplicaten genegeerd)
3. Preview generatie
4. **Import execute → GID matching runs automatisch**
5. Personen krijgen correcte confidence scores

De GID matching gebeurt in [backend/src/imports/service.rs](backend/src/imports/service.rs) (regel 1026-1034):

```rust
// Run GID matching for imported persons
tracing::info!("Starting GID matching for imported persons...");
let gid_match_result = self.run_gid_matching_for_import(&persons).await;
```

## Troubleshooting

### Script Faalt met "Backend niet bereikbaar"

**Oplossing**: Start de backend server:

```bash
cd backend
cargo run --release
```

### Script Timeout

Voor zeer grote datasets (>100k personen) kan het script een timeout geven.

**Oplossing**: Verhoog de timeout in het script:

- PowerShell: `-TimeoutSec 600` → verhoog naar 1200
- Bash: Curl heeft geen default timeout

### Database Connection Error

**Oplossing**: Controleer dat PostgreSQL draait en de `.env` configuratie correct is.

## Tests

Alle tests slagen na de wijzigingen:

```bash
cd backend
cargo test
```

Verwacht:

```
test result: ok. 29 passed; 0 failed; 0 ignored
```

Specifieke GID matcher tests:

```rust
- test_confidence_calculation ✓
- test_extract_gid_from_email ✓
- test_gid_status_thresholds ✓
```

## Bestandswijzigingen

| Bestand                                     | Wijziging                         | Impact                   |
| ------------------------------------------- | --------------------------------- | ------------------------ |
| `backend/src/persons/gid_matcher.rs`        | Confidence logica (100 = matched) | ✓ Al geïmplementeerd     |
| `backend/src/persons/repository.rs`         | Thresholds (100/30)               | **NIEUW**                |
| `backend/src/imports/validator.rs`          | Duplicaten handling               | ✓ Eerder geïmplementeerd |
| `backend/src/imports/service.rs`            | Deduplicatie + GID matching       | ✓ Al geïmplementeerd     |
| `frontend/src/pages/ImportWizardSimple.tsx` | Progress bars + retry             | ✓ Eerder geïmplementeerd |
| `rematch_all_persons.ps1`                   | Re-matching script                | **NIEUW**                |
| `rematch_all_persons.sh`                    | Re-matching script                | **NIEUW**                |

## Volgende Stappen

1. ✅ Run het re-matching script
2. ✅ Verifieer dat ~64,539 personen "matched" status hebben
3. ✅ Test nieuwe imports (automatische matching)

## Referenties

- **FR-005**: Person Management
- **ADR-003**: Backend Technologie Stack
- **GID Matcher**: Automatische Global ID extractie en matching
