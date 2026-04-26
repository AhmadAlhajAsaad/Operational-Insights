# Import Functionaliteit Gebruiksgids

## Status

✅ **De import functionaliteit is nu ACTIEF** en beschikbaar via de API!

## Wat is er opgelost?

Het probleem was dat de backend een **oude binary** draaide (van 17 februari) zonder import functionaliteit. De nieuwe backend met volledige TR-007 implementatie draait nu.

## Beschikbare Functionaliteit

### CSV Import ✅

- **Volledig werkend** - Personen en Organisaties kunnen worden geïmporteerd
- Ondersteunt flexible column mapping (meerdere namen voor dezelfde velden)
- Validatie van data (duplicaten, verplichte velden, formaten)
- Preview functie om wijzigingen te zien vóór import
- Merge logic: "Import priority EXCEPT when import empty and DB filled"

### Excel Import ⏳

- **Tijdelijk uitgeschakeld** wegens technische issues met calamine library
- Gebruik CSV format voor nu
- Excel ondersteuning wordt binnenkort toegevoegd

## API Endpoints

Alle endpoints vereisen **JWT authentica met admin role**.

### 1. Upload Bestand

```bash
POST /api/imports/upload
Content-Type: multipart/form-data

Parameters:
- file: CSV bestand
- import_type: "person" of "organization"
```

**Response:**

```json
{
  "import_id": "uuid-hier",
  "file_name": "persons.csv",
  "status": "uploaded"
}
```

### 2. Preview Genereren

```bash
POST /api/imports/preview
Content-Type: application/json

Body:
{
  "import_id": "uuid-van-upload"
}
```

**Response:**

```json
{
  "import_id": "uuid",
  "total_records": 100,
  "valid_records": 98,
  "invalid_records": 2,
  "actions": [
    {
      "record_index": 0,
      "action": "insert",
      "current_data": null,
      "import_data": { "id": "P001", "full_name": "Jan de Vries", ... },
      "merged_data": { "id": "P001", "full_name": "Jan de Vries", ... }
    },
    {
      "record_index": 1,
      "action": "update",
      "current_data": { "id": "P002", "email": "old@example.com" },
      "import_data": { "id": "P002", "email": "new@example.com" },
      "merged_data": { "id": "P002", "email": "new@example.com" }
    }
  ],
  "validation_errors": [
    {
      "record_index": 99,
      "field": "email",
      "error": "Invalid format",
      "value": "not-an-email"
    }
  ]
}
```

### 3. Import Uitvoeren

```bash
POST /api/imports/execute
Content-Type: application/json

Body:
{
  "import_id": "uuid-van-upload"
}
```

**Response:**

```json
{
  "import_id": "uuid",
  "status": "success",
  "inserted_count": 80,
  "updated_count": 18,
  "failed_count": 2,
  "error_records": [
    {
      "record_index": 99,
      "error": "Database constraint violation"
    }
  ]
}
```

### 4. Import Lijst Ophalen

```bash
GET /api/imports
```

**Response:**

```json
[
  {
    "import_id": "uuid",
    "file_name": "persons.csv",
    "import_type": "person",
    "status": "completed",
    "created_at": "2024-01-15T10:30:00Z",
    "record_count": 100
  }
]
```

### 5. Import Details

```bash
GET /api/imports/{import_id}
```

## CSV Format Vereisten

### Personen Import

**Verplichte velden:**

- `person_id` (of `id`, `ID`)
- `person_email` (of `email`, `mail`)

**Optionele velden:**

- `full_name` (of `fullname`, `name`)
- `first_name` (of `firstname`, `given_name`)
- `last_name` (of `lastname`, `surname`, `family_name`)
- `department` (of `dept`)
- `job_title` (of `title`, `position`, `role`)
- `manager` (of `manager_id`, `reports_to`)
- `start_date` (of `hire_date`, `employment_start`)
- `status` (of `employment_status`)
- `cost_center` (of `costcenter`, `cc`)
- `country` (of `location`, `office_location`)
- `billing_location` (of `billing_office`)

**Voorbeeld CSV:**

```csv
person_id,full_name,first_name,last_name,person_email,department,job_title,status
P001,Jan de Vries,Jan,de Vries,jan.devries@example.com,IT,Software Engineer,active
P002,Marie Jansen,Marie,Jansen,marie.jansen@example.com,HR,HR Manager,active
```

### Organisaties Import

**Verplichte velden:**

- `org_id` (of `organization_id`, `id`)

**Optionele velden:**

- `org_name` (of `organization_name`, `name`)
- `parent_org_id` (of `parent_org`, `parent`)
- `cost_center` (of `costcenter`)
- `manager` (of `manager_id`)
- `budget` (of `annual_budget`)
- `org_type` (of `type`, `organization_type`)

**Voorbeeld CSV:**

```csv
org_id,org_name,parent_org_id,cost_center,manager
ORG001,IT Department,,CC-IT,P001
ORG002,Development Team,ORG001,CC-IT-DEV,P002
```

## Merge Logic

De import volgt deze regel:

> **"Import priority EXCEPT when import empty and DB filled"**

Dit betekent:

- Als import een waarde heeft → gebruik import waarde (overschrijft database)
- Als import leeg is EN database gevuld → behoud database waarde
- Als beide leeg zijn → blijft leeg
- Als import waarde heeft EN database een andere waarde → import wint

**Visual:**

```
Import: "John"  + Database: "Johnny"  → Result: "John"   (import wint)
Import: ""      + Database: "Johnny"  → Result: "Johnny" (database behouden)
Import: "John"  + Database: ""        → Result: "John"   (import wint)
Import: ""      + Database: ""        → Result: ""       (beide leeg)
```

## Validatie Regels

### Email Validatie

- Moet geldig email formaat hebben
- Moet uniek zijn binnen import bestand
- Dubbele emails worden gemarkeerd als error

### ID Validatie

- Personen: `person_id` verplicht
- Organisaties: `org_id` verplicht
- IDs moeten uniek zijn binnen import bestand

### Datum Validatie

- Datums worden geaccepteerd in diverse formaten
- Ongeldige datums worden niet geweigerd maar als string opgeslagen

## Authenticatie

De import endpoints vereisen:

1. **JWT token** in Authorization header
2. **Admin role** in token claims

Voorbeeld authenticatie:

```bash
curl -X POST http://localhost:8080/api/imports/upload \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -F "file=@persons.csv" \
  -F "import_type=person"
```

## Error Handling

Alle errors retourneren een JSON response:

```json
{
  "error": "Error message here"
}
```

Mogelijke HTTP status codes:

- `200` - Success
- `400` - Bad Request (validatie errors)
- `401` - Unauthorized (geen/invalide token)
- `403` - Forbidden (geen admin rechten)
- `404` - Not Found (import_id niet gevonden)
- `500` - Internal Server Error

## Transactie Garanties

De import gebruikt **database transacties**:

- Als één record faalt, worden ALLE veranderingen teruggedraaid
- Database blijft consistent
- Bij success zijn ALLE records opgeslagen
- Bij failure is NIETS opgeslagen

## Test Bestand

Een test CSV bestand is beschikbaar: `/workspace/test_import.csv`

## Volgende Stappen

1. **Authenticatie configureren** - Zorg dat je een geldig JWT token hebt met admin role
2. **Test de import** - Upload het test bestand via de API
3. **Controleer resultaat** - Bekijk de preview en voer de import uit
4. **Verifieer database** - Check of de data correct is opgeslagen

## Technische Details

- **Backend:** Rust met Axum 0.7
- **Parsing:** CSV crate v1.3 (Excel via calamine v0.26.1 - tijdelijk uitgeschakeld)
- **Database:** PostgreSQL met transactie support
- **Validatie:** Automatische duplicate detection en format checking
- **Merge:** Custom merge logic per TR-007 specificatie

## Bekende Beperkingen

1. **Excel tijdelijk uitgeschakeld** - Gebruik CSV format
2. **Admin role vereist** - Reguliere users kunnen niet importeren
3. **Single file upload** - Geen batch import van meerdere bestanden tegelijk
4. **Geen rollback UI** - Eenmaal uitgevoerd kan alleen handmatig worden teruggedraaid

## Support

Bij vragen of problemen:

- Check de backend logs voor gedetailleerde error messages
- Verifieer dat PostgreSQL draait
- Controleer dat de backend de juiste binary gebruikt (niet de oude van 17 feb)
- Test eerst met klein CSV bestand
