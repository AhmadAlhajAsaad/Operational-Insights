# Realisatiedocument

## Equans Operational Insights Dashboard

---

| | |
|---|---|
| **Projecttitel** | Equans Operational Insights Dashboard |
| **Studentnaam** | Ahmad Alhaj Asaad |
| **Opleiding** | HBO-ICT, Software Engineering |
| **Bedrijf/Organisatie** | Equans Digital Technology |
| **Datum** | 24 maart 2026 |
| **Versie** | 1.0 |

---

## 1. Inleiding

### 1.1 Doel van dit document

Ik heb voor Equans Digital Technology een dashboard gebouwd. Het doel: inzicht geven in hoeveel licenties ze afnemen bij vendors als Atlassian en GitHub Enterprise, wat dat kost, en welke medewerkers die licenties eigenlijk gebruiken. Dit realisatiedocument is mijn manier om uit te leggen hoe ik dat technisch heb gedaan.

Eerlijk gezegd heb ik bij het schrijven hiervan wel gemerkt hoe nuttig het is om achteraf alles op een rij te zetten. Tijdens het bouwen neem je constant beslissingen. Soms denk je er goed over na, maar net zo vaak kies je gewoon iets omdat het op dat moment logisch lijkt. En dan later, als je het opschrijft, denk je: oh ja, dat had ik eigenlijk anders moeten doen. Of juist: eigenlijk was dat best een slimme keuze. De opbouw van dit document begint bij de architectuur, gaat dan naar de codestructuur, dan het databaseontwerp, en sluit af met stukken code die ik eruit wil lichten.

### 1.2 Beschreven onderdelen

Het project heeft drie grote stukken. De backend is een REST API geschreven in Rust met Axum. Die doet eigenlijk al het zware werk: data ophalen bij Atlassian en GitHub, verwerken, en via endpoints beschikbaar maken. De frontend is React 19 met TypeScript, gebundeld met Vite. En dan heb je nog de PostgreSQL database waar alles in opgeslagen wordt: personen, organisaties, importhistorie, gecachte vendor-data.

Ik heb er even over nagedacht om de vendor-integraties als losse microservices neer te zetten. Maar ja, voor wie eigenlijk? Het team is klein, de applicatie is overzichtelijk. Al die extra complexiteit van losse services (Docker netwerken, service-to-service communicatie, bij elke deploy drie containers in de juiste volgorde opstarten) levert netto gewoon niet genoeg op. Dus alles zit in een monoliet. Werkt prima. Debuggen is makkelijker, deployen is makkelijker.

### 1.3 Technische uitgangspunten

Een paar dingen stonden voor mij vast toen ik begon met bouwen. Deels omdat Equans dat wilde, deels omdat ik er zelf tegenaan liep.

Het eerste: type safety. Rust aan de backend, TypeScript aan de frontend. Ik wil gewoon dat de compiler me vertelt als ik iets fout doe. Niet dat ik er pas in productie achter kom dat ik `emial` heb geschreven in plaats van `email`. Bij sqlx gaat dat zelfs zover dat SQL-queries op compile-time gevalideerd worden tegen het echte databaseschema. Dat is best bijzonder als je erover nadenkt.

Dan performance. De CSV-bestanden die Equans aanlevert zijn niet klein: 85.000+ rijen. Die wil je niet synchroon verwerken in een webserver die ook gewoon requests moet afhandelen. Dus: Tokio voor async, connection pooling naar Postgres, en een cache-laag voor de externe API-calls zodat je niet telkens opnieuw Atlassian hoeft te bevragen.

Beveiliging gaat via Azure AD. Elk endpoint is standaard afgeschermd met JWT-tokens. Maar voor lokaal ontwikkelen heb ik een vlag ingebouwd waarmee je auth uit kunt zetten, want anders zit je de hele dag tokens te kopieren vanuit Azure. Niet ideaal. En tot slot: Docker. De hele stack start met `docker compose up`. Geen gezeur met lokale Postgres-installaties of conflictende Rust-versies.

---

## 2. Architectuur

### 2.1 Architectuurkeuze

Ik ben uitgekomen op een gelaagde architectuur. Repository Pattern, Service Layer Pattern, dat soort dingen. Klinkt wat schools misschien, maar de kern is eigenlijk simpel: zorg dat je HTTP-handlers geen SQL bevatten, en zorg dat je database-code geen businesslogica bevat. Houd het gescheiden.

Microservices heb ik serieus overwogen. Maar dan moet je gaan nadenken over hoe services met elkaar praten, hoe je logging centraliseert, hoe je health checks tussen services opzet. Fowler (2002) heeft het in zijn boek over het Repository Pattern als een manier om datalogica af te schermen van de rest. En dat was eigenlijk precies wat ik zocht. Ik wilde dat een collega die het project voor het eerst opent niet hoeft te puzzelen waar de query voor het personen-overzicht staat. Die zit in `persons/repository.rs`. Klaar.

Testbaarheid was ook een overweging, hoewel ik eerlijk moet zeggen dat ik daar in het begin minder mee bezig was dan ik had gemoeten. Pas toen de import-logica complexer werd (merge-regels, validatie, rollback) merkte ik hoe fijn het was dat ik de repositorylaag los kon mocken. Dan hoef je niet de hele database op te starten voor een unit test.

De communicatie loopt zo:

```
[Frontend (React)] --> [API Routes (Axum)] --> [Service Layer] --> [Repository Layer] --> [PostgreSQL]
                                                     |
                                               [External APIs]
                                          (Atlassian, GitHub Enterprise)
```

Elke laag praat alleen met zijn directe buur. Simpel principe, maar je moet het wel bewust volhouden. In het begin had ik weleens de neiging om vanuit een route-handler even snel een query te doen. Dat werkt, maar het is het begin van spaghetti.

### 2.2 Laagindeling

Zes lagen in totaal:

| Laag | Verantwoordelijkheid | Technologie |
|------|---------------------|-------------|
| **Presentatielaag** | UI-rendering, gebruikersinteractie, state management | React 19, TypeScript, TailwindCSS, Radix UI |
| **API Client** | HTTP-communicatie met de backend, foutafhandeling | Custom `fetchApi<T>()` wrapper |
| **Route-laag** | Verwerken van HTTP requests en responses, input validatie | Axum 0.7 route handlers |
| **Service-laag** | Businesslogica, caching-strategieen, data-transformatie | Rust services (AtlassianService, ImportService) |
| **Repository-laag** | Database CRUD-operaties, queries, migraties | sqlx 0.8, PostgreSQL |
| **Externe integraties** | Communicatie met vendor APIs | reqwest HTTP client |

De grens tussen route-laag en service-laag was trouwens niet altijd even duidelijk. Waar stop je met "request verwerken" en waar begint "businesslogica"? In de eerste weken had ik stukken logica in de Axum handlers staan die daar niet hoorden. Op een gegeven moment had ik een route-handler van 80 regels. Dat was het moment dat ik dacht: dit moet anders. Alles opgeschoond, logica naar services verplaatst. Het kostte wel een middag refactoren maar daarna was het een stuk overzichtelijker.

---

## 3. Projectstructuur

### 3.1 Frontend

React 19, TypeScript, Vite. Die combinatie bevalt goed. Vite gebruikt SWC onder de motorkap en de hele build is klaar in iets van anderhalve seconde. Ik heb eerder met Webpack gewerkt en het verschil is echt enorm. Je merkt het vooral bij hot module replacement: je slaat een bestand op en het is er meteen.

De structuur is vrij standaard component-based. Pages bevatten de logica (data ophalen, state bijhouden), componenten in `components/` zijn vooral visueel. Auth loopt via een Context die ik in `AuthContext.tsx` heb staan. API-calls gaan allemaal via `backendClient.ts`, daar kom ik later bij de code snippets nog op terug.

```
frontend/src/
├── main.tsx                     # Entry point (MSAL + AuthProvider)
├── App.tsx                      # Hoofdrouter en state-management
├── pages/                       # Pagina-componenten (8 views)
│   ├── OrganizationOverview/    # Organisatiehierarchie
│   ├── OrganizationDetail/      # Organisatiedetails
│   ├── Users/                   # Personenlijst met zoeken/filteren
│   ├── UserDetail/              # Persoonsdetails met GID-status
│   ├── ProductBreakdown/        # Licentieverdeling per product
│   ├── DataImport/              # CSV/Excel upload workflow
│   ├── GitHubVendor/            # GitHub licentie-inzicht
│   └── Login/                   # Authenticatie
├── components/
│   ├── layout/                  # Sidebar, Topbar
│   ├── charts/                  # Recharts wrappers
│   ├── auth/                    # ProtectedRoute, AuthContext
│   ├── ui/                      # 50+ Radix UI primitives
│   └── ErrorBoundary.tsx        # Global error boundary
├── api/backendClient.ts         # Centraal API-client met typed errors
├── context/AuthContext.tsx       # MSAL state management
└── config/msalConfig.ts         # Azure AD configuratie
```

Geen Redux. Bewuste keuze. Al die boilerplate voor actions, reducers, stores, dat past bij grote applicaties met complexe client-side state. Maar hier leeft de data op de server. Elke pagina haalt gewoon zijn eigen data op met een `useEffect`. `useState` en `useCallback` doen de rest. Ik heb bij een vorig project Redux ingezet en achteraf vond ik het daar al te veel gedoe voor wat het opleverde. Hier zag ik al helemaal geen reden voor.

### 3.2 Core/Businesslaag

De echte logica zit in de Rust backend. Ik heb het opgesplitst in service-modules die elk hun eigen stukje afhandelen.

`AtlassianService` (in `atlassian/service.rs`) haalt data op bij de Atlassian Admin API en cachet dat lokaal. Ik heb daar een cache-first strategie voor bedacht. Waarom? Omdat de Atlassian API soms traag is. Gewoon traag. En als die er een keer uit ligt, wil ik niet dat het hele dashboard leeg is. Dan liever verouderde data met een waarschuwingsmelding.

`ImportService` (`imports/service.rs`) is het complexste stuk code in het hele project. Dat regelt alles rondom CSV-import: parsen, valideren, samenvoegen met bestaande data, preview genereren, en uiteindelijk wegschrijven. Bij 85.000 rijen moet dat ook nog eens snel. Maar daar heb ik een apart stuk over geschreven verderop. De `MergeEngine` (`imports/merger.rs`) bepaalt hoe import-data samensmelt met wat al in de database zit. De `Validator` (`imports/validator.rs`) checkt of velden kloppen voordat er iets weggeschreven wordt.

De `GidMatcher` (`persons/gid_matcher.rs`) was een apart avontuur. Die probeert personen automatisch te koppelen aan hun Global ID. Klinkt simpel, maar dat is het niet. E-mailadressen zijn niet altijd uniek, sommige mensen hebben meerdere accounts bij dezelfde vendor. Ik heb daarvoor een puntensysteem gebouwd dat meerdere signalen combineert tot een confidence score. Maar goed, dat leg ik uit bij de code snippets.

En dan zijn er nog twee achtergrondtaken in `jobs/daily_sync.rs` en `jobs/github_sync.rs`. Elke 24 uur halen die data op bij Atlassian en GitHub. Dat waren een van de laatste features die ik heb gebouwd. Ik wilde eerst zeker weten dat de handmatige flow goed werkte, voordat ik dingen ging automatiseren.

Alle types staan in `types.rs` per module. Rust dwingt je om alles van tevoren uit te denken qua datatypes. Enerzijds fijn, want het voorkomt een hele categorie bugs. Anderzijds: als je snel even iets wilt proberen en je moet eerst vijf structs definieren... dat remt.

### 3.3 Backend/API

Rust, Axum 0.7, Tokio. De structuur ziet er zo uit:

```
backend/src/
├── main.rs                      # Entry point, configuratie, router
├── config.rs                    # Environment variable loading
├── routes/                      # API endpoint handlers
│   ├── atlassian.rs             # GET /api/atlassian/*
│   ├── persons.rs               # CRUD /api/persons
│   ├── organizations.rs         # CRUD /api/organizations
│   └── imports.rs               # POST /api/imports/*
├── atlassian/                   # Vendor integratie
│   ├── client.rs                # HTTP client voor Atlassian Admin API
│   ├── service.rs               # Caching + businesslogica
│   ├── link_service.rs          # Koppeling Atlassian users <-> persons
│   ├── types.rs                 # Datamodellen
│   └── error.rs                 # Error types met HTTP mappings
├── auth/                        # Authenticatie
│   ├── jwt.rs                   # JWT validatie tegen Azure AD JWKS
│   ├── middleware.rs            # Bearer token extractie + validatie
│   ├── claims.rs                # Azure AD JWT claims
│   └── error.rs                 # Auth error types (401, 403)
├── persons/                     # Personenbeheer
│   ├── repository.rs            # CRUD + zoeken/filteren + paginatie
│   ├── gid_matcher.rs           # GID matching met confidence score
│   └── types.rs                 # Person, PersonDetail structs
├── organizations/               # Organisatiebeheer
│   ├── repository.rs            # CRUD met hierarchie
│   └── types.rs                 # Organization structs
├── imports/                     # Data-import (FR-007)
│   ├── service.rs               # Upload/parse/preview/execute orchestratie
│   ├── parser.rs                # CSV (single-pass) en Excel parsing
│   ├── validator.rs             # Schema- en veldvalidatie
│   ├── merger.rs                # Merge import met bestaande data
│   ├── repository.rs            # Import-historie en bulk operaties
│   └── types.rs                 # Import-gerelateerde structs
├── github.rs                    # GitHub Enterprise API client
├── github_cache.rs              # GitHub cache repository
├── github_link.rs               # GitHub <-> Person linking
├── cache/                       # Generiek cache-systeem (TTL-based)
├── jobs/                        # Achtergrondtaken
│   ├── daily_sync.rs            # Atlassian sync (24u interval)
│   └── github_sync.rs           # GitHub sync (24u interval)
└── health.rs                    # Health check endpoint
```

RESTful API. De endpoints:

| Methode | Endpoint | Functie |
|---------|----------|---------|
| `GET` | `/api/persons` | Personenoverzicht (gepagineerd, zoekbaar) |
| `GET` | `/api/persons/:id` | Persoonsdetails |
| `POST` | `/api/persons` | Persoon aanmaken |
| `PUT` | `/api/persons/:id` | Persoon bijwerken |
| `GET` | `/api/organizations` | Organisatieoverzicht |
| `GET` | `/api/organizations/:id` | Organisatiedetails |
| `GET` | `/api/organizations/:id/persons` | Personen binnen een organisatie |
| `POST` | `/api/imports/upload` | Bestand uploaden en parsen |
| `POST` | `/api/imports/preview` | Preview van wijzigingen |
| `POST` | `/api/imports/execute` | Import daadwerkelijk uitvoeren |
| `POST` | `/api/imports/quick-import` | Directe import (alles in een stap) |
| `GET` | `/api/atlassian/users` | Atlassian gebruikers (uit cache) |
| `GET` | `/api/atlassian/product-stats` | Productstatistieken |
| `GET` | `/api/github/users` | GitHub Enterprise gebruikers |
| `GET` | `/api/github/licenses` | Licentieconsumptie |

### 3.4 Overige modules

Er zijn nog een paar onderdelen die niet in de drie hoofdcomponenten vallen maar wel nodig zijn. De `infra/` map bevat de Docker Compose setup: PostgreSQL 16, de backend en de frontend allemaal in containers. Een commando en alles staat. Database-migraties (8 SQL-bestanden) zitten in `backend/migrations/` en draaien automatisch mee als de backend opstart. Verder staan er in `scripts/` een paar utility-scripts en in `docs/` de volledige documentatie: van FR-001 tot FR-012, Architecture Decision Records, testplannen.

---

## 4. Database

### 4.1 Databasekeuze

PostgreSQL 16. De belangrijkste reden: JSONB. Het punt is dat vendor-specifieke data per vendor verschilt. Atlassian heeft account IDs en product access, GitHub heeft usernames en Copilot seat info. Als ik voor elke vendor aparte kolommen zou aanmaken, moet ik het schema telkens aanpassen als er een vendor bijkomt. Nu heb ik een `vendor_identifiers` JSONB-kolom op de `persons` tabel en daarin dumpt elke vendor zijn eigen structuur (PostgreSQL Documentation, 2024). Schema hoeft niet te veranderen.

Wat ik niet had verwacht is hoe goed PostgreSQL's full-text search werkt voor dit project. Ik had er in het begin niet eens aan gedacht. Maar toen het personen-overzicht met een `ILIKE '%zoekterm%'` query bij 85.000 records merkbaar ging vertragen (je zat al snel boven de 400ms), ben ik gaan kijken naar alternatieven. Elasticsearch leek overdreven voor wat ik nodig had. Toen ben ik gestuit op `tsvector` met `GIN` indexering en dat bleek precies genoeg. Zoeken gaat nu in een paar milliseconden.

Referentiele integriteit heb ik ook bewust ingericht. De foreign key van `persons.org_id` naar `organizations` heeft `ON DELETE SET NULL`. Dus als je een organisatie weggooit, worden personen niet verwijderd maar alleen losgekoppeld. Ik heb genoeg horrorverhalen gehoord over `ON DELETE CASCADE` die onbedoeld duizenden records meeneemt. Verder: connection pool van max 50 connecties, indexen op `org_id`, `email`, `status`, `gid_confidence`.

### 4.2 Datamodel

Drie soorten tabellen.

De kerntabellen zijn de basis. `persons` is de hoofdtabel met `person_id` als primaire sleutel, `email` (uniek), `org_id` als foreign key, `gid` en `gid_confidence` voor de matching-status, en die `vendor_identifiers` JSONB-kolom. `organizations` bevat de bedrijfsstructuur met een self-referencing `parent_org_id` voor de hierarchie. Elke organisatie heeft ook een `cost_center` en `budget` veld, want Equans wilde licentiekosten per afdeling kunnen toewijzen. De `imports` tabel slaat metadata op van elke import-actie, inclusief `rollback_data` in JSONB zodat je een import terug kunt draaien als het mis gaat. Dat laatste heb ik er pas later aan toegevoegd, nadat een collega vroeg: "en als het fout gaat, wat dan?"

Cache-tabellen zijn er voor de vendor-data. `atlassian_users_cache` en `atlassian_groups_cache` hebben een TTL van 25 uur. Daarnaast `github_users_cache`, `github_licenses_cache` en `github_copilot_cache`. Het idee is dat het dashboard niet bij elk paginaverzoek de externe APIs hoeft te bevragen. Dat zou veel te langzaam zijn en bovendien loop je tegen rate limits aan.

Dan de audit-tabellen. `atlassian_link_audit` en `github_link_audit` loggen elke koppeling die wordt gemaakt tussen een interne persoon en een vendor-account. Dat was een eis van Equans: ze willen kunnen terugzien wie wanneer wat heeft gekoppeld. Snap ik ook wel, want als er iets fout zit in een koppeling wil je weten wie dat heeft gedaan en wanneer.

### 4.3 Database-communicatie

Ik gebruik sqlx 0.8 voor alle database-interactie. Geen ORM dus, gewoon SQL, maar dan met compile-time verificatie. Dat is het verschil met iets als Diesel: sqlx checkt op het moment dat je `cargo build` doet of je queries kloppen tegen het echte schema. Verkeerde kolomnaam? Compilatiefout. Verkeerd type? Compilatiefout. Dat voelt in het begin wat streng maar je went eraan. En het scheelt enorm in runtime bugs.

De pool is geconfigureerd met 50 connecties max, 30 seconden acquire timeout, 10 minuten idle timeout. Migraties draaien automatisch bij startup via `sqlx::migrate!()`. Queries zijn altijd geparameteriseerd (`$1`, `$2` enzovoort). Niet dat ik daar bewust over na hoef te denken trouwens, want sqlx staat simpelweg niet toe dat je strings gaat concateneren in queries. SQL-injectie is daarmee in feite onmogelijk.

---

## 5. Belangrijke functionaliteiten

### 5.1 Overzicht

Twaalf Functional Requirements vormen samen het systeem:

| # | Functionaliteit | Beschrijving |
|---|----------------|--------------|
| FR-001 | License Dashboard | Licentieoverzicht per product, organisatie en persoon met grafieken |
| FR-002 | Vendor Data Collection | Ophalen van data bij Atlassian en GitHub APIs |
| FR-003 | Atlassian Cache | 25-uur TTL caching om het aantal API-calls te beperken |
| FR-004 | API Authentication | Azure AD JWT-validatie, optioneel uitschakelbaar voor lokaal ontwikkelen |
| FR-005 | Person Management | CRUD voor personen, GID-matching, soft-delete |
| FR-006 | Organization Management | CRUD voor organisaties met hierarchie en kostenplaatsen |
| FR-007 | Data Import | CSV/Excel upload met validatie, preview en rollback |
| FR-008 | Atlassian User Management | Gebruikers uitnodigen en suspenden via de Atlassian API |
| FR-009 | Atlassian DB Sync | Synchronisatie van Atlassian data naar de lokale database |
| FR-010 | Frontend Dashboard | React UI voor het browsen van organisaties, gebruikers en licenties |
| FR-011 | GitHub Integration | Ophalen van licentie- en Copilot seat consumptie |
| FR-012 | GitHub DB Sync | GitHub data cachen en koppelen aan personen |

### 5.2 Technische realisatie highlights

Er zijn drie onderdelen die ik extra wil toelichten omdat ze technisch het meest uitdagend waren.

De data import pipeline (FR-007). Dit was met afstand het moeilijkste. Ik denk dat ik hier het meeste tijd aan kwijt ben geweest van het hele project. Het proces gaat als volgt: bestand uploaden, type detecteren (CSV of Excel), parsen met single-pass column mapping, velden valideren, duplicaten filteren, preview genereren (zodat je ziet wat er gaat veranderen), en dan pas echt wegschrijven in een database-transactie. Klinkt als een nette pipeline als je het zo leest, maar in werkelijkheid ging er van alles mis. Het eerste waar ik tegenaan liep: de CSV parsing van 85.000 rijen blokkeerde de Tokio executor. Andere endpoints reageerden niet meer. De oplossing was `tokio::task::spawn_blocking` om het rekenwerk naar een aparte threadpool te verplaatsen. Dat had ik eerder moeten weten eigenlijk, want het staat gewoon in de Tokio documentatie.

De cache-first strategy (FR-003). De Atlassian API is traag. Soms 3 seconden per request, soms 5. En er zijn rate limits. Na een paar keer dat het dashboard een halve minuut stond te laden omdat elk onderdeel apart de Atlassian API aanriep, heb ik een databasecache gebouwd met een TTL van 25 uur. Het systeem checkt eerst of er verse cache is. Zo ja, dan wordt de API niet eens aangeraapt. En als de API helemaal onbereikbaar is (is een keer gebeurd tijdens een demo, vrij genant), dan valt het terug op verouderde cachedata. Beter oude data dan geen data.

GID matching (FR-005). Personen koppelen aan hun Global ID. In het begin dacht ik: ik match gewoon op e-mailadres, klaar. Bleek niet zo simpel. E-mailadressen zijn niet altijd uniek. Sommige worden gedeeld. Sommige kloppen niet meer. Dus heb ik er een puntensysteem van gemaakt: e-mail prefix geeft 30 punten, local ID match 20 punten, GitHub username 10 punten, Atlassian account 10 punten. Totaal geeft een confidence score van 0 tot 100. Bij minder dan 100 krijgt de beheerder de match als suggestie te zien, zodat die het handmatig kan bevestigen.

---

## 6. Code snippets

### Snippet 1: Centrale API client met foutafhandeling (Frontend)

**Bestand:** `frontend/src/api/backendClient.ts`

```typescript
export class ApiError extends Error {
  readonly status: number;
  readonly code: string;

  constructor(status: number, code: string, message: string) {
    super(message);
    this.name = "ApiError";
    this.status = status;
    this.code = code;
  }
}

async function fetchApi<T>(
  endpoint: string,
  options?: RequestInit,
): Promise<T> {
  const response = await fetch(`${API_BASE}${endpoint}`, {
    headers: { "Content-Type": "application/json", ...options?.headers },
    ...options,
  });

  if (!response.ok) {
    let code = "UNKNOWN";
    let message = `Request failed (HTTP ${response.status})`;
    try {
      const body = await response.json();
      if (body && typeof body === "object") {
        if (typeof body.message === "string") message = body.message;
        else if (typeof body.error === "string" && !body.message)
          message = body.error;
        if (typeof body.error === "string") code = body.error;
      }
    } catch {
      const text = await response.text().catch(() => "");
      if (text) message = text;
    }
    throw new ApiError(response.status, code, message);
  }

  return response.json() as Promise<T>;
}
```

Alles wat de frontend naar de backend stuurt gaat via `fetchApi<T>()`. De `<T>` parameter zorgt dat TypeScript weet welk type data je terugkrijgt. Scheelt type-casts door de hele codebase.

Wat misschien raar lijkt is die dubbele foutafhandeling. Eerst probeer ik het error-response als JSON te lezen (de backend stuurt normaal gesproken gestructureerde fouten met een `error` en `message` veld). Maar soms, en daar kwam ik pas achter na wat debuggen, stuurt Axum zelf een foutmelding terug die gewoon plain text is. Geen JSON. Dus dan moet ik terugvallen op `response.text()`. Die edge case miste ik in het begin en dan kreeg je van die onduidelijke "Failed to parse JSON" fouten in de console. Niet handig.

De `ApiError` class heeft een `code` property (zoals `"NOT_FOUND"` of `"VALIDATION_ERROR"`) waarmee ik in componenten kan switchen op specifieke fouttypes. Validatiefout? Toon inline errors. 404? Redirect. Ik had dit ook per component kunnen doen maar dat zou dezelfde logica op een stuk of tien plekken dupliceren. Liever een keer goed neerzetten.

---

### Snippet 2: JWT authenticatie middleware (Backend)

**Bestand:** `backend/src/auth/middleware.rs`

```rust
pub async fn auth_middleware(
    State(validator): State<Arc<JwtValidator>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::MissingToken)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthError::InvalidAuthHeader)?;

    let claims = validator.validate(token).await?;

    tracing::info!(
        user_id = claims.user_id(),
        method = %request.method(),
        path = %request.uri().path(),
        "Authenticated API request"
    );

    request.extensions_mut().insert(AuthenticatedUser(claims));

    Ok(next.run(request).await)
}
```

De auth middleware. Zit om alle beveiligde endpoints heen. Bearer token uit de header halen, valideren tegen Azure AD's publieke sleutels (via JWKS), en de claims als `AuthenticatedUser` meegeven aan de rest van de request-chain.

Het stukje logging was er niet meteen. Ik heb het erin gezet toen ik problemen had met een endpoint dat 403's gaf terwijl dat niet zou moeten. Door te loggen welk user_id welk pad aanvroeg kon ik het vrij snel traceren. Ik log bewust alleen user_id, methode en pad. Niet het token zelf. In een eerder prototype had ik dat per ongeluk wel gedaan en een medestudent die ernaar keek zei meteen: "dat is een security issue". Terecht.

Het mooie aan Axum is dat die `AuthenticatedUser` vervolgens in elke route-handler als extractor beschikbaar is. Je doet gewoon `AuthenticatedUser(claims): AuthenticatedUser` in de functiesignatuur en klaar. Geen handmatig doorsturen van tokens. Dat is echt een van de dingen die Axum fijn maakt om mee te werken (Axum Docs, 2024).

---

### Snippet 3: CSV parser met single-pass column mapping (Backend)

**Bestand:** `backend/src/imports/parser.rs`

```rust
pub fn parse_csv_fast(
    file_data: &[u8],
) -> Result<(Vec<PersonImportRow>, Vec<OrgImportRow>, bool), ImportError> {
    let cursor = Cursor::new(file_data);
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(cursor);

    let headers = reader
        .headers()
        .map_err(|e| ImportError::ParseError(
            format!("Failed to read CSV headers: {}", e)))?
        .clone();

    let is_person = Self::detect_person_import_from_headers(&headers);

    if is_person {
        let find = |names: &[&str]| -> Option<usize> {
            names.iter().find_map(|n| headers.iter()
                .position(|h| h.trim().to_lowercase() == *n))
        };

        let idx_id = find(&["person_id", "id"]);
        let idx_email = find(&["person_email", "email", "mail", "e-mail"]);
        let idx_first_name = find(&["person_first_name", "first_name",
            "firstname"]);
        // ... overige kolommen

        let mut persons = Vec::with_capacity(90_000);
        for (row_idx, result) in reader.records().enumerate() {
            let rec = result.map_err(|e| ImportError::ParseError(
                format!("Failed to parse CSV row {}: {}", row_idx + 2, e)
            ))?;
            persons.push(PersonImportRow {
                id: get_val(&rec, idx_id),
                email: get_val(&rec, idx_email),
                first_name: get_val(&rec, idx_first_name),
                // ...
            });
        }
        Ok((persons, vec![], true))
    } else { /* org import path */ }
}
```

Dit is versie twee van de CSV parser. Versie een werkte, maar was belachelijk traag. Ik gebruikte daar per rij een `HashMap<String, String>` om kolomnamen aan waardes te koppelen. Klinkt redelijk toch? Bij 85.000 rijen maak je dan 85.000 keer een HashMap aan, elke keer met string-allocaties voor de keys. De parse-tijd zat op ruim 20 seconden. Dat is voor een webapplicatie niet te doen.

Versie twee werkt met indexen. De headers lees ik een keer, en daarna weet ik: kolom 3 is email, kolom 7 is first_name, enzovoort. Verder reserveer ik met `Vec::with_capacity(90_000)` meteen genoeg geheugen. Zonder die pre-allocatie moet Rust de vector steeds vergroten en data kopieren naarmate die vol raakt.

De `find` closure is er omdat Equans CSV-bestanden van verschillende systemen krijgt en die systemen hanteren andere kolomnamen. "email" in het ene bestand is "person_email" in het andere, of "mail", of "e-mail". Na deze herschrijving was de parse-tijd 3 seconden. Nog steeds niet instant, maar werkbaar.

---

### Snippet 4: GID matching met confidence scoring (Backend)

**Bestand:** `backend/src/persons/gid_matcher.rs`

```rust
fn calculate_confidence(&self, person: &Person, gid: &str) -> i32 {
    if !person.person_id.starts_with("AUTO_") && !person.person_id.is_empty() {
        return 100;
    }

    let mut confidence = 0;

    if !gid.is_empty() {
        confidence += 30;
    }

    if let Some(local_id) = &person.local_id {
        if local_id.to_lowercase().contains(gid) {
            confidence += 20;
        }
    }

    if let Some(vendor_ids) = &person.vendor_identifiers {
        if let Some(github_username) = vendor_ids
            .get("github").and_then(|g| g.get("username"))
            .and_then(|u| u.as_str())
        {
            if github_username.to_lowercase() == gid {
                confidence += 10;
            }
        }

        if let Some(atlassian_email) = vendor_ids
            .get("atlassian").and_then(|a| a.get("email"))
            .and_then(|e| e.as_str())
        {
            if let Some(atlassian_gid) =
                self.extract_gid_from_email(atlassian_email)
            {
                if atlassian_gid == gid { confidence += 10; }
            }
        }
    }

    confidence.min(99)
}
```

Hier heb ik echt lang op zitten puzzelen. Het probleem: hoe weet je of persoon X in de database dezelfde is als account Y bij Atlassian of GitHub? In de ideale wereld heeft iedereen een uniek ID dat overal hetzelfde is. In de praktijk: niet dus.

Een persoon die al een `person_id` heeft (niet gegenereerd, dus zonder `AUTO_` prefix) krijgt score 100. Klaar, die is gematcht. Maar de automatisch aangemaakte personen (uit imports) moeten gematcht worden op basis van signalen. Het e-mail prefix levert 30 punten, een hit op local ID 20, GitHub username 10, Atlassian email 10.

De `.min(99)` aan het eind is er omdat ik ooit een bug had waarbij het systeem personen als "definitief gematcht" bestempelde terwijl het eigenlijk een automatische gok was. Score 100 is nu gereserveerd voor handmatig bevestigde matches. Alles daaronder is een suggestie die iemand nog moet controleren. Klein detail, maar het scheelde me een hoop gedoe achteraf toen bleek dat sommige automatische matches toch niet klopten.

---

### Snippet 5: Cache-first service met stale fallback (Backend)

**Bestand:** `backend/src/atlassian/service.rs`

```rust
pub async fn get_users(
    &self,
    org_id: &str,
    force_refresh: bool,
) -> ServiceResult<CachedResponse<Vec<User>>> {
    if !force_refresh {
        if let Ok(Some((users, cached_at, expires_at))) =
            self.cache.get_cached_users().await
        {
            tracing::info!("Returning {} users from cache", users.len());
            return Ok(CachedResponse::cached(
                users, cached_at, expires_at, false));
        }
    }

    match self.client.fetch_users(org_id).await {
        Ok(users) => {
            if let Err(e) = self.cache.store_users(&users).await {
                tracing::warn!("Failed to cache users: {}", e);
            }
            Ok(CachedResponse::fresh(users))
        }
        Err(api_error) => {
            tracing::warn!("API error, trying stale cache: {}", api_error);

            if let Ok(Some((users, cached_at, expires_at))) =
                self.cache.get_stale_users().await
            {
                return Ok(CachedResponse::cached(
                    users, cached_at, expires_at, true));
            }

            Err(ServiceError::Atlassian(api_error))
        }
    }
}
```

Dit is er gekomen na een vrij genant moment. Demo voor de opdrachtgever, ik open het dashboard, en de Atlassian API geeft een timeout. Leeg scherm. Niet echt indrukwekkend. Daarna heb ik dit patroon gebouwd.

Stap een: check of er cache is die nog geldig is (binnen 25 uur). Zo ja, direct teruggeven, Atlassian API niet aanraken. Stap twee: geen geldige cache, dan toch de API bevragen en het resultaat opslaan. Stap drie: API geeft een fout en er IS nog oude cache (verlopen maar niet gewist). Dan die oude data teruggeven met een `stale: true` vlag. De frontend toont dan een gele balk: "data is mogelijk niet actueel." Niet ideaal, maar de gebruiker kan tenminste gewoon doorwerken.

Nygard (2018) beschrijft dit als graceful degradation. Het systeem doet wat het kan met wat het heeft. Beter dat dan helemaal plat gaan.

---

### Snippet 6: Merge engine voor data import (Backend)

**Bestand:** `backend/src/imports/merger.rs`

```rust
pub fn merge_person(
    db_person: &Person,
    import_person: &PersonImportRow
) -> Person {
    Person {
        person_id: db_person.person_id.clone(),

        first_name: Self::merge_string_with_placeholder(
            import_person.first_name.as_deref(),
            &db_person.first_name,
            "[To Be Determined]",
        ),

        email: Self::merge_string_with_placeholder(
            import_person.email.as_deref(),
            &db_person.email,
            "unknown@placeholder.local",
        ),

        local_id: Self::merge_optional_field(
            import_person.local_id.as_ref(),
            db_person.local_id.as_ref(),
        ),

        country: Self::merge_optional_field(
            import_person.country.as_ref(),
            db_person.country.as_ref(),
        ),

        gid: db_person.gid.clone(),
        gid_confidence: db_person.gid_confidence,
        vendor_identifiers: db_person.vendor_identifiers.clone(),
        // ...
    }
}
```

CSV-exports uit Palantir bevatten niet altijd alle kolommen. Soms ontbreekt de kolom `country`, soms `local_id`. Als je dan gewoon alles blind overschrijft met de import-data verlies je bestaande informatie. Dat wilde ik niet. De merge-logica werkt zo: import heeft een waarde? Die wint. Import is leeg maar database niet? Database blijft staan. Allebei leeg? Placeholder.

Er is een uitzondering: `gid`, `gid_confidence` en `vendor_identifiers` worden nooit overschreven door een import. Die worden uitsluitend door het systeem zelf beheerd (via de GidMatcher en de vendor sync-jobs). Dit heb ik er pas ingebouwd nadat het een keer fout ging. Ik draaide een test-import en opeens waren de GID-scores van een paar duizend personen weg. De CSV had die kolommen niet, dus de merge overschreef ze met lege waardes. Dat was een vrij stressvolle middag van handmatig herstellen. Sindsdien zijn die velden hardcoded uitgesloten.

---

## 7. Samenvatting

Het is af. Of ja, "af". Software is nooit echt af. Maar het werkt, het doet wat Equans nodig heeft, en ik ben er redelijk trots op. De stack (Rust/Axum, React 19, PostgreSQL) heeft goed uitgepakt. Het is snel, het is stabiel, en het is uitbreidbaar als er later meer vendors bij moeten komen.

Rust was een uitdaging. Ik ga daar niet omheen draaien. De leercurve is steil en vooral de borrow checker bezorgde me in het begin regelmatig hoofdpijn. Maar als je het eenmaal snapt betaalt het zich terug. De CSV parser doet 85.000 rijen in 3 seconden. Compile-time checks vangen fouten op die je in Python of JavaScript pas in productie zou merken. Dat is het waard.

De import pipeline en het GID matching systeem hebben veruit het meeste tijd gekost. Meerdere iteraties, meerdere keren dingen weggooien en opnieuw beginnen. Het confidence score systeem bestond in de eerste versie niet eens, dat is ontstaan door trial and error met echte Equans-data. De merge-strategie idem: pas nadat ik een keer data kwijtraakte door een onvolledige CSV snapte ik hoe nodig het was om daar goed over na te denken. Dat zijn precies het soort lessen die je niet uit een boek leert.

---

## 8. Bronnen

1. Fowler, M. (2002). *Patterns of Enterprise Application Architecture*. Addison-Wesley.
2. Martin, R.C. (2003). *Agile Software Development: Principles, Patterns, and Practices*. Pearson.
3. Nygard, M.T. (2018). *Release It! Design and Deploy Production-Ready Software* (2e druk). Pragmatic Bookshelf.
4. Axum Documentation (2024). *Axum, Ergonomic and modular web framework*. Geraadpleegd op https://docs.rs/axum/latest/axum/
5. PostgreSQL Documentation (2024). *JSON Types*. Geraadpleegd op https://www.postgresql.org/docs/16/datatype-json.html
6. sqlx Documentation (2024). *sqlx, Async SQL toolkit for Rust*. Geraadpleegd op https://docs.rs/sqlx/latest/sqlx/
7. React Documentation (2024). *React 19, The library for web and native user interfaces*. Geraadpleegd op https://react.dev/
8. Microsoft Identity Platform (2024). *Microsoft Authentication Library (MSAL)*. Geraadpleegd op https://learn.microsoft.com/en-us/entra/identity-platform/
9. Atlassian Admin API (2024). *Atlassian Cloud Admin REST API*. Geraadpleegd op https://developer.atlassian.com/cloud/admin/
10. GitHub REST API (2024). *GitHub Enterprise Cloud REST API*. Geraadpleegd op https://docs.github.com/en/enterprise-cloud@latest/rest
