# Import Troubleshooting Guide

## Probleem: Import Failed

Je ziet het volgende:
- **Preview**: 2,872 nieuwe records, 83,066 updates
- **Error**: "Import failed: Import failed"
- **Aantal pogingen**: 3x op "Import Uitvoeren" geklikt

## Mogelijke Oorzaken & Oplossingen

### 1. Preview Cache Timeout ⏱️

**Probleem**: De preview wordt in-memory gecached. Als je te lang wacht tussen preview en execute, kan de cache verlopen zijn.

**Oplossing**:
1. Genereer de preview opnieuw (klik op "Genereer Preview")
2. Klik direct daarna op "Import Uitvoeren" (binnen 5-10 minuten)

### 2. Database Constraints 🔒

**Probleem**: Met 83,066 updates kunnen er unique constraint violations zijn (bijv. duplicate emails).

**Oplossing - Controleer Backend Logs**:
```bash
cd /workspace/backend
cargo run
```

Zoek in de output naar:
- `"duplicate key value violates unique constraint"`
- `"foreign key constraint violation"`
- `"Update batch X/Y failed:"`

### 3. Corrupt CSV Data 📄

**Probleem**: De "Nieuwe IDs" in de preview tonen raw CSV data in plaats van alleen IDs. Dit suggereert parsing problemen.

**Voorbeeld corrupt data**:
```
FJD 27, FHA084,Petar,CICKOVIC,petar.cickovic@equans.com,...
```

**Oplossing**:
1. Open je CSV bestand
2. Controleer of:
   - Alle rijen dezelfde aantal kolommen hebben
   - Geen komma's in data velden (of correct ge-quoted met `""`)
   - Geen extra line breaks binnen velden
3. Gebruik het correcte CSV formaat:
   ```csv
   person_id,first_name,last_name,email,country,org_id
   FHA084,Petar,CICKOVIC,petar.cickovic@equans.com,AT,ORG0042
   ```

### 4. Grote Dataset Performance 🚀

**Probleem**: 83,066 updates is een grote batch die lange tijd kan duren.

**Oplossing**:
1. Verhoog de timeout in de frontend:
   ```typescript
   // In ImportWizardSimple.tsx
   setTimeout: 600000 // 10 minuten in plaats van standaard
   ```

2. Split de import in kleinere batches:
   - Deel je CSV op in meerdere bestanden (bijv. 20k per bestand)
   - Importeer ze één voor één

### 5. Memory Issues 💾

**Probleem**: De backend kan out-of-memory gaan met zeer grote imports.

**Oplossing - Restart Backend met Meer Memory**:
```bash
# Stop huidige backend (Ctrl+C)
cd /workspace/backend

# Start met release mode (geoptimaliseerd)
cargo run --release
```

## Debug Steps

### Stap 1: Check Backend Logs

1. Start de backend in debug mode:
   ```bash
   cd /workspace/backend
   RUST_LOG=debug cargo run
   ```

2. Doe de import opnieuw
3. Zoek naar error messages die beginnen met:
   - `"ERROR"`
   - `"Update batch X/Y failed:"`
   - `"Transaction failed"`

### Stap 2: Controleer Database

Run deze query om te zien of er conflicten zijn:

```sql
-- Check voor duplicate emails
SELECT email, COUNT(*)
FROM persons
GROUP BY email
HAVING COUNT(*) > 1;

-- Check voor duplicate person_ids
SELECT person_id, COUNT(*)
FROM persons
GROUP BY person_id
HAVING COUNT(*) > 1;
```

### Stap 3: Test met Kleine Dataset

1. Maak een test CSV met alleen 10-20 records uit je grote bestand
2. Probeer die te importeren
3. Als dat werkt, is het een performance/memory issue

## Verbeterde Error Logging

Ik heb zojuist **verbeterde error logging** toegevoegd aan de backend. Nu zie je:
- Exacte batch nummer waar de fout optreedt
- Specifieke database error messages
- Context over welke operatie faalt (insert/update/soft-delete)

### Nieuwe Backend Starten

```bash
cd /workspace/backend
cargo build
cargo run
```

Nu zul je bij een fout zien:
```
ERROR Update batch 83/84 failed: duplicate key value violates unique constraint "persons_email_key"
DETAIL: Key (email)=(test@example.com) already exists.
```

In plaats van alleen:
```
Import failed: Import failed
```

## Veelvoorkomende Fouten

### Error: "Preview not found"

**Oorzaak**: Preview cache is verlopen of backend is herstart.

**Oplossing**: Genereer preview opnieuw.

### Error: "duplicate key value violates unique constraint"

**Oorzaak**: Je CSV bevat duplicates of de database heeft al die data.

**Oplossing**:
1. Check je CSV voor duplicates
2. Of: verwijder eerst oude data

### Error: "transaction too old"

**Oorzaak**: Import duurt te lang voor één transactie.

**Oplossing**: Dit is al geïmplementeerd - we gebruiken batches van 1000 records.

## Aanbevolen Workflow

Voor grote imports (>50,000 records):

1. **Valideer eerst**:
   ```bash
   # Check CSV syntax
   csvlint yourfile.csv
   ```

2. **Test met subset**:
   - Importeer eerst 100 records
   - Dan 1,000 records
   - Dan de volledige dataset

3. **Monitor Progress**:
   - Kijk naar backend logs tijdens import
   - Noteer hoelang elke batch duurt
   - Verwachte tijd: ~30-60 seconden per 1000 records

4. **Run tijdens rustige uren**:
   - Grote imports doen tijdens daluren
   - Zo min mogelijk concurrent gebruik

## Snelle Fix Checklist

- [ ] Backend herstart: `cargo run --release`
- [ ] Preview opnieuw gegenereerd
- [ ] Direct na preview op "Import Uitvoeren" geklikt
- [ ] Backend logs gecheckt voor specifieke errors
- [ ] CSV gevalideerd (geen corrupt data)
- [ ] Eventueel dataset gesplit in kleinere batches

## Contact & Support

Als het probleem blijft:

1. **Deel backend logs**:
   ```bash
   cargo run 2>&1 | tee import_error.log
   # Dit slaat alle output op in import_error.log
   ```

2. **Deel preview data**:
   - Aantal nieuwe records
   - Aantal updates
   - Eerste paar IDs uit "Nieuwe IDs"

3. **Database stats**:
   ```sql
   SELECT COUNT(*) FROM persons;
   SELECT COUNT(*) FROM persons WHERE status = 'Active';
   ```

## Preventie

Voor toekomstige imports:

1. **Valideer CSV eerst** met een kleiner test bestand
2. **Gebruik release mode** voor backend: `cargo run --release`
3. **Split grote imports** (>100k) in batches
4. **Monitor memory** tijdens import: `htop` of `top`
5. **Backup database** voor grote imports:
   ```bash
   pg_dump -U postgres equans_insights > backup_$(date +%Y%m%d).sql
   ```
