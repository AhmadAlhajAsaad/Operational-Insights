# ADR-001: Projectopzet en repositorystructuur

| Metadata | Details |
|----------|---------|
| **Status** | Geaccepteerd |
| **Datum** | 2025-11-04 |
| **Auteur** | Ahmad Alhaj Asaad |
| **Laatst bijgewerkt** | 2025-12-05 |

---

## Context

Equans Operational Insights is een full-stack systeem dat gebruiks- en licentiedata verzamelt, verwerkt en visualiseert vanuit meerdere platformen zoals Atlassian, GitHub Enterprise en JFrog. Het doel is om organisaties inzicht te geven in hun softwarelicenties, kosten en gebruikersactiviteit.

Voordat de functionele ontwikkeling kon beginnen, moest er een duidelijke en onderhoudbare projectstructuur komen. Binnen dit project is gekozen voor een monorepo-aanpak, zodat de backend, frontend en infrastructuur naast elkaar staan in dezelfde repository. Hierbij is gekozen voor deze aanpak omdat het team klein is en de overhead van meerdere losse repository's niet opweegt tegen de voordelen. Tijdens het opzetten bleek al snel dat een goede mappenstructuur vanaf het begin veel verwarring voorkomt, zeker als er later nieuwe teamleden bijkomen.

De belangrijkste punten die in deze ADR worden behandeld zijn de repositorystructuur, de scheiding tussen backend en frontend, hoe documentatie is georganiseerd en de basis voor CI/CD en testing.

---

## Besluit

### Projectstructuur

Hierbij is gekozen voor een monorepo met een duidelijke scheiding tussen de verschillende lagen van het systeem. In de praktijk betekent dit dat elke map een eigen verantwoordelijkheid heeft. Een ontwikkelaar die aan de frontend werkt, hoeft niet door de backend-code te navigeren en andersom.

```
Equans-operational-insights/
+-- backend/
|   +-- src/
|   |   +-- main.rs                # Entrypoint van de API-server
|   |   +-- lib.rs                 # Library root
|   |   +-- config.rs              # Configuratie via environment variables
|   |   +-- error.rs               # Gedeelde foutafhandeling (AppError)
|   |   +-- health.rs              # Health check endpoint
|   |   +-- github.rs              # GitHub Enterprise API-client
|   |   +-- github_cache.rs        # GitHub cache repository
|   |   +-- github_link.rs         # Koppeling GitHub-accounts aan personen
|   |   +-- atlassian/             # Atlassian API-integratie
|   |   +-- auth/                  # Authenticatie (JWT, Azure AD)
|   |   +-- cache/                 # Cache-abstractie met TTL
|   |   +-- imports/               # Data-import (CSV/Excel)
|   |   +-- jobs/                  # Achtergrondtaken (sync)
|   |   +-- organizations/         # Organisatiebeheer
|   |   +-- persons/               # Persoonsbeheer en GID-matching
|   |   +-- routes/                # API-endpoint handlers
|   |   +-- security/              # Security headers en data-masking
|   |   +-- bin/                   # CLI-hulptools
|   +-- migrations/                # 8 SQL-migratiebestanden
|   +-- tests/                     # Backend unit- en integratietests
|   +-- .sqlx/                     # Gecompileerde queries (offline modus)
|   +-- Cargo.toml
|   +-- Cargo.lock
|
+-- frontend/                      # React + TypeScript dashboard
|   +-- src/
|   |   +-- main.tsx               # Entrypoint (MSAL + AuthProvider)
|   |   +-- App.tsx                # Hoofdcomponent met routing
|   |   +-- api/                   # Gecentraliseerde API-client
|   |   +-- pages/                 # Paginacomponenten (9 pagina's)
|   |   +-- components/            # Herbruikbare UI-componenten
|   |   +-- config/                # MSAL-configuratie (Azure AD)
|   |   +-- context/               # React Context (authenticatie)
|   |   +-- styles/                # Tailwind CSS configuratie
|   |   +-- assets/                # Statische bestanden (logo)
|   +-- package.json
|   +-- vite.config.ts
|
+-- infra/                         # Infrastructuur en deployment
|   +-- docker-compose.yml         # PostgreSQL, backend, frontend
|
+-- docs/                          # Alle projectdocumentatie
|   +-- Acad/                      # Academische documenten (HBO)
|   +-- ADRs/                      # Architecture Decision Records
|   +-- Business-Requirements/     # Zakelijke vereisten
|   +-- Functional-Requirements/   # Functionele vereisten
|   +-- Technical-Requirements/    # Technische vereisten
|   +-- testing/                   # Testdocumentatie
|
+-- scripts/                       # Hulpscripts voor ontwikkeling
+-- .devcontainer/                 # VS Code Dev Container configuratie
+-- .github/
|   +-- agents/                    # GitHub Copilot agent-configuraties
|   +-- prompts/                   # Prompt-bestanden voor agents
|   +-- workflows/                 # CI/CD pipelines (GitHub Actions)
|
+-- README.md
+-- .gitignore
+-- .pre-commit-config.yaml
```

Een uitdaging hierbij was het vinden van de juiste granulariteit. In eerste instantie stond alles voor de backend in een paar grote bestanden (zoals `atlassian.rs` en `github.rs` die routes, services en models combineerden). Hierbij bleek dat dit snel onoverzichtelijk werd naarmate er meer functionaliteit bijkwam. Hierdoor is de backend opgesplitst in aparte modules per domein, zoals `atlassian/`, `imports/` en `persons/`, elk met hun eigen types, repository en service-logica.

### Technologiestack

| Laag | Technologie | Waarom deze keuze |
|------|-------------|-------------------|
| **Backend API** | Rust met Axum 0.7 en SQLx | Typeveiligheid, snelheid, async runtime met Tokio |
| **Frontend** | React 19 met TypeScript en Vite 6 | Snel ontwikkelen, typeveiligheid, snelle builds |
| **Database** | PostgreSQL 16 | Volwassen, betrouwbaar, goede ondersteuning voor JSONB en complexe queries |
| **Lokale ontwikkeling** | Docker en Docker Compose | Reproduceerbare omgevingen, geIsoleerde services |
| **Deployment** | Docker containers + GitHub Actions | Geautomatiseerde CI/CD, container-klaar |
| **Documentatie** | Markdown + ADR-structuur | Versiebeheer naast de code, doorzoekbaar |
| **Versiebeheer** | GitHub (Equans DevOps Forge org) | Enterprise-integratie, compliance |

Hierbij is gekozen voor Rust als backend-taal omdat het team hier al ervaring mee had. Daarnaast biedt Rust compile-time typeveiligheid, waardoor veel fouten al tijdens het bouwen worden gevonden in plaats van in productie. Het Axum framework sluit hier goed op aan vanwege de goede ondersteuning voor async middleware en routing. Op basis van deze analyse bleek dat de combinatie van Rust, Axum en SQLx een goede balans biedt tussen veiligheid en ontwikkelsnelheid. SQLx valideert SQL-queries al tijdens het compileren, wat in de praktijk veel runtime-fouten voorkomt.

Voor de frontend is gekozen voor React 19 met TypeScript en Vite. Vite zorgt voor zeer snelle builds en hot module replacement (onder de 100ms), wat het ontwikkelen aangenaam maakt. React met TypeScript biedt typeveiligheid aan de frontend-kant, wat goed aansluit bij de strenge typering in Rust aan de backend-kant. De UI-componenten zijn gebouwd met Radix UI en Tailwind CSS, en voor grafieken wordt Recharts gebruikt.

### Frontend-backend integratie

Tijdens lokale ontwikkeling stuurt de Vite-devserver alle `/api/*` verzoeken door naar de backend via een proxy. De frontend draait op `http://localhost:5173` en de backend op `http://localhost:8080`. Dit voorkomt CORS-problemen tijdens het ontwikkelen, zonder dat daar extra configuratie voor nodig is. In de productie-omgeving worden statische bestanden direct vanuit de backend of via een CDN aangeboden.

De communicatie verloopt via een gecentraliseerde API-client in `frontend/src/api/backendClient.ts`. Hierin zit een generieke `fetchApi<T>()` functie die automatisch Content-Type headers instelt en fouten afhandelt via een `ApiError` class. Tijdens het ontwikkelen viel op dat een centrale API-client veel duplicatie voorkomt, zeker wanneer er steeds meer endpoints bijkomen.

---

## Motivatie

### Waarom deze structuur

De monorepo-aanpak is gekozen omdat het team relatief klein is (een enkele ontwikkelaar voor de afstudeerstage) en alle onderdelen nauw met elkaar samenhangen. Hierdoor kunnen wijzigingen aan de API en de frontend in dezelfde commit worden doorgevoerd, wat de consistentie ten goede komt.

De mappenstructuur is zo opgezet dat elke laag onafhankelijk kan worden ontwikkeld, getest en gedeployed. Teamleden (of toekomstige ontwikkelaars) kunnen aan de frontend werken zonder de backend-code te hoeven begrijpen, en andersom. De backend is modulair opgebouwd met aparte mappen per domein. Zo heeft de `imports/` module een eigen `service.rs`, `parser.rs`, `validator.rs` en `merger.rs`, wat het overzichtelijk houdt ook als de logica complexer wordt.

De ADR-structuur in `docs/ADRs/` zorgt ervoor dat technische beslissingen traceerbaar zijn. Op basis van deze analyse bleek dat zonder zulke documentatie het "waarom" achter beslissingen snel verloren gaat, vooral als het project later wordt overgedragen.

Docker Compose maakt het mogelijk om de hele stack (PostgreSQL, backend en frontend) met een enkel commando op te starten. Hierbij is gekozen voor PostgreSQL 16 op poort 5433 (om conflicten met lokale installaties te voorkomen), de backend op poort 8080 en de frontend op poort 3000 (productie) of 5173 (ontwikkeling).

### Waarom Rust voor de backend

Rust is de standaard binnen dit team. Daarnaast biedt het betere prestaties voor dataverwerking vergeleken met alternatieven zoals Python of Node.js. De compile-time typeveiligheid vermindert bugs in productie aanzienlijk, en het geheugengebruik is efficienter voor services die lang draaien. Een uitdaging hierbij was de steilere leercurve, maar de voordelen op het gebied van veiligheid en prestaties wegen hier ruimschoots tegenop.

---

## Gevolgen

### Positieve uitkomsten

Het project wordt hiermee toegankelijker voor nieuwe ontwikkelaars. Ze kunnen de repository clonen en de README volgen om lokaal aan de slag te gaan. Elke map heeft een duidelijke verantwoordelijkheid, waardoor je snel weet waar je moet zoeken. De backend en frontend kunnen los van elkaar worden ontwikkeld, getest en uitgerold.

De CI/CD-integratie met GitHub Actions maakt het mogelijk om automatisch tests te draaien voor gewijzigde componenten. Docker builds zijn deterministisch en draagbaar, wat betekent dat wat lokaal werkt ook in productie werkt. Deployment pipelines kunnen specifieke services targeten, zodat niet het hele systeem opnieuw hoeft te worden gebouwd bij een kleine wijziging.

De modulaire opzet zorgt ervoor dat frontend-componenten niet afhankelijk zijn van implementatiedetails van de backend. Ze communiceren via API-contracten. Backend endpoints kunnen worden gestubd voor frontend-ontwikkeling, wat het parallel werken vergemakkelijkt.

De documentatie via ADR's creert een doorzoekbare geschiedenis van genomen beslissingen. Toekomstige ontwikkelaars kunnen de afwegingen begrijpen zonder het aan iemand te hoeven vragen.

### Aandachtspunten en beperkingen

Hierbij bleek dat de ADR-structuur alleen werkt als het team zich eraan committeert. Als documentatie achterblijft bij de code, neemt de verwarring snel toe. Dit vraagt discipline, zeker bij een project waar de oorspronkelijke ontwikkelaar op een gegeven moment vertrekt.

Rust heeft een steilere leercurve dan Python of JavaScript. SQL-queryoptimalisatie vraagt databasekennis die verder gaat dan wat je normaal bij webontwikkeling tegenkomt. Daarnaast kosten async Rust-patronen wat tijd om goed onder de knie te krijgen.

De eerste keer het project opzetten vereist het installeren van Rust, Node.js, Docker en PostgreSQL. Sommige ontwikkelaars hebben hier mogelijk ondersteuning bij nodig. De Dev Container configuratie in `.devcontainer/` helpt hierbij, omdat het een kant-en-klare ontwikkelomgeving biedt via VS Code of GitHub Codespaces.
