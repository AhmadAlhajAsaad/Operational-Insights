# Database Toegang Gids

## Huidige Status

**Database:** PostgreSQL 16 (draait in Docker container)

- **Host:** postgres
- **Poort:** 5432
- **Database:** equans_insights
- **Gebruiker:** equans
- **Wachtwoord:** equans_password

**Huidige Inhoud:**

- ✅ Personen: **0 records**
- ✅ Organisaties: **0 records**
- ✅ Imports: **0 records**

_(Database is leeg - je eerste import heeft de oude backend gebruikt die geen data opsloeg)_

---

## Methode 1: Command Line (psql) 🚀

### Basis Commando's

**Connect naar database:**

```bash
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights
```

**Eenmalige query:**

```bash
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "SELECT * FROM persons;"
```

### Handige Queries

**Alle personen:**

```bash
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "
SELECT person_id, full_name, email, department, status
FROM persons
LIMIT 10;"
```

**Alle organisaties:**

```bash
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "
SELECT org_id, org_name, cost_center, manager
FROM organizations
LIMIT 10;"
```

**Import geschiedenis:**

```bash
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "
SELECT import_id, file_name, import_type, status, created_at, record_count
FROM imports
ORDER BY created_at DESC
LIMIT 10;"
```

**Database statistieken:**

```bash
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "
SELECT
    (SELECT COUNT(*) FROM persons) as persons,
    (SELECT COUNT(*) FROM organizations) as organizations,
    (SELECT COUNT(*) FROM imports) as imports;"
```

### Interactieve psql (met commands)

```bash
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights
```

Dan binnen psql:

- `\dt` - Toon alle tabellen
- `\d persons` - Toon structuur van persons tabel
- `\d organizations` - Toon structuur van organizations tabel
- `\l` - Toon alle databases
- `\q` - Quit

---

## Methode 2: Via Backend API 🔌

**Check persons via API:**

```bash
curl -s http://localhost:8080/api/persons | jq '.'
```

**Check organizations via API:**

```bash
curl -s http://localhost:8080/api/organizations | jq '.'
```

**Check imports:**

```bash
curl -s -H "Authorization: Bearer YOUR_TOKEN" http://localhost:8080/api/imports | jq '.'
```

---

## Methode 3: Database Schema Bekijken 📋

**Persons tabel structuur:**

```bash
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "\d persons"
```

**Alle kolommen van een tabel:**

```bash
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "
SELECT column_name, data_type, is_nullable, column_default
FROM information_schema.columns
WHERE table_name = 'persons'
ORDER BY ordinal_position;"
```

---

## Methode 4: Database Tool (VS Code Extension) 🎨

### Installeer PostgreSQL Extension

1. Open VS Code Extensions (Ctrl+Shift+X)
2. Zoek: **"PostgreSQL" by Chris Kolkman**
3. Installeer de extension

### Configureer Connectie

1. Druk op F1 en type: `PostgreSQL: New Connection`
2. Vul in:
   - **Host:** postgres
   - **Port:** 5432
   - **Database:** equans_insights
   - **Username:** equans
   - **Password:** equans_password

3. Nu kun je:
   - Tabellen browsen
   - Queries uitvoeren
   - Data bekijken in een grid

---

## Quick Reference Commands

### Snelle Checks

```bash
# Hoeveel persons?
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "SELECT COUNT(*) FROM persons;"

# Hoeveel organizations?
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "SELECT COUNT(*) FROM organizations;"

# Laatste 5 imports?
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "SELECT * FROM imports ORDER BY created_at DESC LIMIT 5;"

# Alle personen met email?
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "SELECT person_id, full_name, email FROM persons WHERE email IS NOT NULL;"
```

### Data Exploratie

```bash
# Unieke departments
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "SELECT DISTINCT department FROM persons WHERE department IS NOT NULL;"

# Personen per status
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "SELECT status, COUNT(*) as count FROM persons GROUP BY status;"

# Organisatie hiërarchie
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "SELECT org_id, org_name, parent_org_id FROM organizations ORDER BY parent_org_id NULLS FIRST;"
```

---

## Handige Aliases (Optioneel)

Voeg toe aan je `~/.bashrc` of `~/.zshrc`:

```bash
alias db='PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights'
alias db-persons='PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "SELECT * FROM persons LIMIT 20;"'
alias db-orgs='PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "SELECT * FROM organizations LIMIT 20;"'
alias db-imports='PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "SELECT * FROM imports ORDER BY created_at DESC LIMIT 10;"'
alias db-count='PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "SELECT (SELECT COUNT(*) FROM persons) as persons, (SELECT COUNT(*) FROM organizations) as orgs;"'
```

Dan kun je simpelweg typen:

- `db` - Open database shell
- `db-persons` - Toon persons
- `db-orgs` - Toon organizations
- `db-imports` - Toon imports
- `db-count` - Toon totalen

---

## Problemen Oplossen

### Database niet bereikbaar?

```bash
# Check of postgres container draait
docker ps | grep postgres

# Check database connectie
PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "SELECT version();"
```

### Wachtwoord werkt niet?

De credentials staan in: [.devcontainer/docker-compose.yml](.devcontainer/docker-compose.yml)

---

## Volgende Stap: Test de Import!

Nu je de database kunt bekijken, test de import functionaliteit:

1. **Upload test bestand:**

   ```bash
   curl -X POST http://localhost:8080/api/imports/upload \
     -H "Authorization: Bearer YOUR_TOKEN" \
     -F "file=@test_import.csv" \
     -F "import_type=person"
   ```

2. **Check database:**
   ```bash
   PGPASSWORD=equans_password psql -h postgres -U equans -d equans_insights -c "SELECT COUNT(*) FROM persons;"
   ```

Zie [IMPORT_GUIDE.md](IMPORT_GUIDE.md) voor volledige import instructies.
