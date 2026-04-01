# Technisch adviesrapport

## Equans Operational Insights Dashboard

---

| | |
|---|---|
| **Projecttitel** | Equans Operational Insights Dashboard |
| **Onderwerp** | Technisch adviesrapport voor doorontwikkeling van het licentie-inzicht dashboard |
| **Studentnaam** | Ahmad Alhaj Asaad (1035912) |
| **Opleiding** | Informatica - Hogeschool Rotterdam |
| **Organisatie** | Equans Nederland - SLS Digital Platforms (DevOps Forge) |
| **Datum** | 25 maart 2026 |
| **Versie** | 1.0 |

---

## Inhoudsopgave

- [1. Inleiding](#1-inleiding)
- [2. Context en probleemstelling](#2-context-en-probleemstelling)
- [3. Samenvatting onderzoeksbevindingen](#3-samenvatting-onderzoeksbevindingen)
- [4. Technische adviezen](#4-technische-adviezen)
  - [4.1 Frontend-technologie](#41-frontend-technologie)
  - [4.2 Backend-architectuur](#42-backend-architectuur)
  - [4.3 Datastorage](#43-datastorage)
  - [4.4 Integraties met externe vendor-API's](#44-integraties-met-externe-vendor-apis)
  - [4.5 Performance en caching](#45-performance-en-caching)
  - [4.6 Authenticatie en autorisatie](#46-authenticatie-en-autorisatie)
  - [4.7 Privacy en beveiliging](#47-privacy-en-beveiliging)
- [5. Risico's en aandachtspunten](#5-risicos-en-aandachtspunten)
- [6. Eindadvies](#6-eindadvies)
- [7. Aanbevelingen voor doorontwikkeling](#7-aanbevelingen-voor-doorontwikkeling)
- [8. Bronnen](#8-bronnen)

---

## 1. Inleiding

### 1.1 Aanleiding en doel

Voor mijn afstudeerproject heb ik gewerkt aan het Equans Operational Insights Dashboard. Dit was bij de afdeling SLS Digital Platforms, ook wel DevOps Forge genoemd, van Equans Nederland. Het idee erachter is eigenlijk vrij rechttoe rechtaan. Equans heeft een hoop softwarelicenties lopen bij Atlassian, GitHub Enterprise en straks waarschijnlijk ook JFrog. Maar het probleem was dat er nergens een centrale plek bestond waar je kon zien wat dat allemaal kost en wie welke licentie daadwerkelijk gebruikt. Daar moest dit dashboard verandering in brengen.

Dit adviesrapport heb ik geschreven om alles wat ik geleerd heb tijdens het bouwen en onderzoeken om te zetten naar adviezen die echt hout snijden. Ik wilde niet zomaar wat losse tips geven, maar onderbouwde keuzes presenteren. Gebaseerd op dingen die ik zelf ben tegengekomen, op wat de literatuur erover zegt, en op de resultaten van mijn onderzoek. Het rapport is geschreven voor Viktor en Brian (mijn begeleiders), maar ook voor de ontwikkelaars die na mij verdergaan met het systeem.

### 1.2 Bronnen voor de adviezen

Mijn adviezen steunen op drie dingen. Het eerste is gewoon mijn eigen ervaring: ik heb meerdere proof-of-concepts gebouwd, verschillende technologieën uitgeprobeerd (sommige werkten goed, andere helemaal niet) en uiteindelijk een werkend MVP afgeleverd. Met daarin 12 functionele requirements en 12 technische requirements. Het tweede is vakliteratuur. Ik heb me behoorlijk ingelezen in softwarearchitectuur, hoe je goede REST-API's ontwerpt, hoe dashboards effectief werken volgens mensen als Stephen Few, en hoe IT-kostentransparantie eruitziet in de praktijk. Daarnaast heb ik veel uren doorgebracht in de API-documentatie van Atlassian, GitHub en Microsoft. En als derde bron zijn er de bevindingen uit mijn onderzoeksverslag, waar ik vijf deelvragen heb uitgewerkt.

### 1.3 Leeswijzer

Hoofdstuk 2 gaat over waar het project vandaan komt en welk probleem het probeert op te lossen. In hoofdstuk 3 vat ik kort samen wat ik uit het onderzoek heb gehaald. Het belangrijkste stuk is hoofdstuk 4: daar geef ik per technisch onderdeel een advies, met uitleg waarom, en wat ik heb afgewezen. Hoofdstuk 5 bespreekt de risico's die ik zie. Hoofdstuk 6 trekt alles samen in één eindadvies en in hoofdstuk 7 staat een roadmap voor hoe het systeem zich verder kan ontwikkelen. In hoofdstuk 8 staan de bronnen.

---

## 2. Context en probleemstelling

### 2.1 Projectcontext

Equans is een grote technische dienstverlener. Duizenden mensen werken er dagelijks met Jira, Confluence, JSM, GitHub, en nog meer tools. De afdeling DevOps Forge, waar ik stage liep, beheert die platforms. Toen ik begon aan het project viel me vrij snel iets op: er was simpelweg geen overzicht. Wil je weten hoeveel Jira-licenties er actief zijn? Dan log je in op de Atlassian-beheerconsole. Wil je weten wat GitHub kost? Ander portaal. Wil je vervolgens uitrekenen wat afdeling X kwijt is aan software? Dan zit je handmatig spreadsheets aan elkaar te knopen. Dat was de situatie.

### 2.2 Oorspronkelijke probleemstelling

De onderzoeksvraag die ik heb geformuleerd was: *"Hoe kan Equans inzicht verkrijgen in het Atlassian Cloud Enterprise-licentiegebruik en de daarmee samenhangende kosten, om besparingsmogelijkheden te identificeren en te optimaliseren?"*

Wat dat concreet betekende: het systeem moest automatisch licentiedata binnenhalen uit de API's van Atlassian en GitHub. Daar moest je patronen in kunnen herkennen, denk aan accounts die al maanden niet worden gebruikt maar nog gewoon op een betaalde licentie staan. Het moest ook mogelijk zijn om kosten per afdeling te berekenen, zodat afdelingen zelf kunnen zien wat ze uitgeven. En dat alles in een dashboard dat een manager kan begrijpen zonder technische achtergrond.

### 2.3 Technische uitdagingen tijdens het project

Ik ben eerlijk: sommige dingen liepen heel anders dan ik verwachtte.

Het eerste waar ik tegenaan liep was de Atlassian API. Ik nam aan dat er gewoon een eindpunt zou zijn waar prijzen of factuurinfo in zit. Dat is niet zo. Er zitten geen billing-gegevens in de Cloud Admin API (Atlassian Support, z.d.). Dat betekende dat ik kostberekeningen helemaal zelf moest bouwen, op basis van het aantal seats en tarieven die ik handmatig moest configureren. Best vervelend, want het voelde als iets wat de API gewoon had moeten bieden.

Dan de rate limits. Zowel Atlassian als GitHub beperken hoeveel API-calls je per minuut mag doen. Mijn eerste aanpak was simpel: gewoon bij elke paginalading de data vers ophalen. Dat werkte prima met één gebruiker. Met vijf gebruikers tegelijk liep het al vast. Ik moest dus een hele andere strategie bedenken, en daar is uiteindelijk het caching-systeem uit voortgekomen.

Wat ik ook niet had verwacht: organisatiedata (welke kostenplaats, welke manager, welk budget) zit niet in de vendor-API's. Helemaal niet. Die informatie moest via CSV-bestanden uit Palantir Foundry geïmporteerd worden. Dit ontdekte ik pas echt in sprint 2, op het moment dat ik dacht dat het dashboard al bijna af was. Dat was een flinke tegenvaller, want het betekende dat ik een hele import-module moest bouwen die ik niet had ingepland.

Het koppelen van accounts over vendors heen was ook een ding op zich. Iemand heeft een Atlassian account-ID, een GitHub loginnaam, en intern een GID-nummer. In theorie koppel je die op e-mailadres. In de praktijk bleek dat mensen soms een persoonlijk mailadres bij GitHub hadden staan, of dat namen net anders gespeld waren. "De Vries" versus "de Vries" versus "DeVries", dat soort dingen. Ik heb uiteindelijk een heuristische matcher gebouwd die redelijk goed werkt, maar perfect is het niet.

De SSO-integratie vergde ook meer werk dan gepland. Equans draait op Microsoft Entra ID, en ik wilde de backend in Rust bouwen. Probleem: er bestaat geen MSAL-library voor Rust. Ik moest de JWT-validatie dus zelf schrijven. Dat was een paar dagen extra werk, en het debuggen van token-validatiefouten is niet het leukste programmeerwerk dat ik ooit heb gedaan.

En Rust zelf, ja. Ik heb er bewust voor gekozen (daar kom ik later op terug waarom), maar de ownership-regels en lifetimes waren in het begin echt frustrerend. Er waren dagen dat ik meer tijd kwijt was aan het oplossen van compileerfouten dan aan het schrijven van nieuwe functionaliteit. Na een paar weken ging het beter, maar die eerste periode was pittig.

---

## 3. Samenvatting onderzoeksbevindingen

Uit mijn onderzoek komen een aantal bevindingen die direct invloed hadden op hoe ik het systeem heb gebouwd. Ik noem de tien belangrijkste.

Atlassian werkt met iets dat Maximum Quantity Billing (MQB) heet. Kort gezegd: je krijgt een factuur gebaseerd op het hoogste aantal licenties dat je op enig moment in de factureringsperiode had toegewezen (Atlassian, 2024). Dus als je op 15 januari 500 licenties hebt en op 20 januari schaalt naar 550, betaal je de hele maand voor 550. Zelfs als je op 21 januari weer teruggaat naar 500. Dat maakt continu monitoren noodzakelijk, want opruimen aan het eind van de maand helpt niet meer.

Wat ik al noemde: de Atlassian API levert geen prijsinformatie. Helemaal niets over billing. Kosten moet je zelf uitrekenen op basis van seats maal een tarief dat je ergens configureert. Eerlijk gezegd had ik gehoopt dat dat anders zou zijn.

Bij GitHub Enterprise was het een ander verhaal. De API geeft netjes `total_seats_consumed` versus `total_seats_purchased`, en ook hoeveel mensen Copilot gebruiken en GHAS-statistieken (GitHub Docs, z.d.). Dat maakte die integratie een stuk makkelijker om te bouwen.

Toen ik voor het eerst de data uit de Atlassian-sync bekeek, bleek dat zo'n 5 tot 10 procent van alle licenties op accounts stond die al meer dan 90 dagen niks hadden gedaan. Nul activiteit. Maar ze stonden nog gewoon op een betaalde licentie. Bij de aantallen van Equans gaat dat over serieus geld.

De vendor-API's leveren geen informatie over de interne organisatiestructuur. Je ziet wel wie een licentie heeft, maar niet bij welke afdeling die persoon hoort of welke kostenplaats daarbij past. Die koppeling moet via externe CSV-import gemaakt worden. Dat had grote gevolgen voor het ontwerp.

Na wat experimenteren met directe API-calls werd het me duidelijk dat cache-first de enige werkbare aanpak was. De API's zijn te traag en te streng gelimiteerd om bij elke dashboardpagina aan te spreken. Een TTL van 25 uur (net iets meer dan de dagelijkse sync) houdt de data vers genoeg voor dashboardgebruik.

Geen van de vendor-API's biedt historische data. Wil je trends tonen, dan moet je zelf periodiek snapshots bewaren. Dit stond niet in mijn oorspronkelijke ontwerp en moest er later bij.

De heuristische GID-matching (personen automatisch koppelen aan vendor-accounts op e-mail en naam) werkte beter dan ik had verwacht. In de testperiode zijn 85.842 personen gematcht. Maar er zaten fouten tussen. Iemand die van naam veranderd was door een huwelijk, iemand met een typefout in het systeem, dat soort edge cases. Handmatige controle blijft nodig.

Vanuit de literatuur (Few, 2013 en Tufte, 2001) leerde ik dat dashboards gericht moeten zijn op snel inzicht met de mogelijkheid om dieper in te zoomen. Gewoon een grote tabel met alle data neerzetten werkt niet; je moet visualisaties bieden die iets vertellen op het eerste gezicht.

Tot slot bevestigde mijn onderzoek dat Rust goed past bij dit type systeem. De compile-time checks, het geheugenmodel zonder garbage collector, en de efficiënte async verwerking zijn allemaal relevant voor een server die continu met externe API's praat (Klabnik & Nichols, 2023). Meer hierover in hoofdstuk 4.

---

## 4. Technische adviezen

### 4.1 Frontend-technologie

#### Advies

**Aanbevolen: React 19 + TypeScript + Vite**

Gewoon doorgaan met de huidige stack. React 19 met TypeScript in strict mode, Vite als bundler met SWC, Radix UI voor componenten, Tailwind voor styling, Recharts voor grafieken.

#### Motivatie

Ik ga niet doen alsof dit een moeilijke keuze was, want dat was het niet echt.

React past gewoon goed bij dit soort applicaties. Een dashboard bestaat uit herbruikbare brokken: een chart die je op drie pagina's laat zien, filtervelden die overal hetzelfde werken, tabellen met paginatie. React's componentmodel maakt dat heel natuurlijk. Ik heb uiteindelijk 8 pagina's gebouwd en ze delen bijna allemaal dezelfde basiscomponenten. Dat werkte goed.

TypeScript in strict mode was eigenlijk geen keuze maar een logisch gevolg. Als je in de backend Rust gebruikt waar de compiler alles checkt, dan wil je in de frontend niet ineens losse JavaScript schrijven waarbij een verkeerd type pas opvalt als een gebruiker op een knop klikt.

Vite met SWC als transpiler maakt je leven als ontwikkelaar gewoon beter. De HMR werkt vrijwel instant en de productiebundle komt uit op 280 KB gzip. Binnen het requirement van TM-12 dus. Tijdens het bouwen scheelt die snelheid je echt frustratie, want je zit constant te itereren.

Voor Radix UI heb ik gekozen omdat het een headless library is: meer dan 50 componenten waar de accessibility (WCAG 2.1) al in zit, maar waar je zelf de styling op doet via Tailwind. Keyboard-navigatie en screenreader-support hoef je niet zelf te bouwen. Dat scheelt een hoop werk.

En het meest praktische argument: React is de standaard binnen Equans Digital Platforms. De volgende ontwikkelaar die aan dit project gaat werken, kent React. Als ik Vue of Svelte had gekozen, moet diegene dat eerst leren. Dat is een reëel probleem bij overname van een project.

In de praktijk bewezen: 8 pagina's met interactieve charts, real-time syncstatussen, en het laadt onder de 3 seconden (TM-09).

#### Afgeraden alternatieven

| Alternatief | Reden voor afwijzing |
|---|---|
| **Angular 17+** | Te veel boilerplate voor een dashboard van deze omvang. Modules, dependency injection, RxJS, het is best veel overhead. De bundlesize is ook groter. Beter voor hele grote enterprise-apps met meerdere teams (Angular, z.d.). |
| **Vue 3 + Composition API** | Op zich een goed framework, maar Vue is geen standaard bij Equans. Het ecosysteem voor enterprise-dashboardcomponenten is ook wat kleiner. Kennisdeling en onderhoud worden lastiger als de rest van het team Vue niet kent. |
| **Vanilla JS / Web Components** | Kleinere bundle, ja. Maar probeer maar eens een multi-vendor dashboard te bouwen met state management, routing en interactieve charts in vanilla JavaScript. De ontwikkeltijd wordt enorm. |

#### Secundaire optie

Mocht Equans ooit overstappen op een ander framework, dan is Svelte 5 het bekijken waard. De compiler-gebaseerde aanpak levert kleinere bundles op. Maar het ecosysteem is nog niet volwassen genoeg voor dit soort projecten, althans nu nog niet.


---

### 4.2 Backend-architectuur

#### Advies

**Aanbevolen: Rust + Axum 0.7 + SQLx met gelaagde service-architectuur**

Doorgaan met Rust, Axum als webframework, SQLx voor de database, Tokio als async runtime. En de gelaagde opbouw behouden: Routes, Handlers, Services, Repositories.

#### Motivatie

De keuze voor Rust was niet vanzelfsprekend en ik wil daar eerlijk over zijn. Het was een afweging met serieuze nadelen. Maar ik ben achteraf overtuigd dat het voor dit project de juiste keuze was.

Wat mij het meest overtuigde was SQLx in combinatie met Rust's typesysteem. SQLx valideert je SQL-queries tijdens het compileren, tegen het echte databaseschema. Schrijf je `SELECR` in plaats van `SELECT`? Compileert niet. Verwijs je naar een kolom die niet bestaat? Compileert niet. In het begin vond ik dat eerlijk gezegd wat overdreven streng. Maar na een paar weken merkte ik dat ik gewoon nauwelijks SQL-bugs had (Klabnik & Nichols, 2023). Geen enkele. Normaal zit je toch regelmatig runtime errors te debuggen die door een typefout in een query komen. Dat viel hier gewoon weg.

Het ownership-model van Rust is een apart verhaal. Het voorkomt buffer overflows, use-after-free, data races, allemaal op compilatieniveau. Voor een server die 24 uur per dag draait en tegelijkertijd achtergrondsyncs doet en API-requests afhandelt, wil je dat soort garanties hebben. Ik heb tijdens het hele project geen enkele memory-gerelateerde crash gehad.

Tokio als async runtime doet wat het moet doen. Bij load testing met k6 haalde ik consistent P95-responsietijden onder 200ms met 100 simultane gebruikers (TM-08). Het geheugengebruik bleef stabiel, geen spikes. Dat is iets wat bij een taal met garbage collector moeilijker te garanderen is, omdat je willekeurige pauzes krijgt wanneer de GC actief wordt.

Axum paste als framework het beste bij de rest. Type-safe routing, een modulaire middleware-stack, en het sluit naadloos aan op Tokio. Ik heb ook even naar Actix Web gekeken, maar Axum voelde als de meer moderne keuze die beter ondersteund werd.

De architectuur heb ik opgebouwd in lagen: Routes → Handlers → Services → Repositories. Dat klinkt misschien als textbook-architectuur uit een boek van Robert Martin (2017), en dat is het ook. Maar het bewees zijn waarde toen ik GitHub als tweede vendor toevoegde. Ik kon hetzelfde patroon volgen als bij Atlassian, zonder bestaande code aan te passen. Nieuwe client, nieuwe service, nieuwe repository, klaar. Het repository pattern met traits maakt het ook haalbaar om later de cache-strategie te wisselen zonder de hele servicelaag overhoop te halen.

Concreet heb ik hiermee 12 functionele requirements gebouwd, 4 externe integraties (Atlassian, GitHub, Azure AD JWKS, CSV uit Palantir), en background sync jobs die elke 24 uur draaien. De foutafhandeling gaat via Rust's `Result<T, E>` pattern en ik heb in de hele projectperiode geen panics in productie gehad.

De keerzijde was echt de leercurve. De eerste twee, drie weken was ik meer bezig met de compiler tevreden houden dan met functionaliteit schrijven. Lifetimes, borrowing, de async syntax; het duurde even voor het klikte. Na die initiële drempel ging de productiviteit echt omhoog, maar ik snap dat dit voor een toekomstig team een drempel kan zijn.

#### Afgeraden alternatieven

| Alternatief | Reden voor afwijzing |
|---|---|
| **Node.js + Express** | Geen compile-time SQL-verificatie, geen compile-time typegaranties op API-contracten. De garbage collector geeft onvoorspelbare latency spikes. En het single-threaded event-loop model is minder efficiënt dan Tokio's multi-threaded runtime als je veel externe API-calls tegelijk doet. |
| **Python + FastAPI** | De GIL beperkt echte parallelle verwerking. Performance is merkbaar lager dan Rust. Geen compile-time SQL-checks. |
| **Go** | Eenvoudiger om te leren, compileert snel. Maar de typering is losser, er is geen SQLx-equivalent, en de foutafhandeling (`if err != nil` op elke derde regel) maakt complexe integratiecode lastig leesbaar. |
| **C# / .NET** | Volwassen ecosysteem. Maar de CLR heeft een grotere memory footprint. Voor een service die voornamelijk API-calls doet en data aggregeert, biedt Rust meer met minder resources. |

#### Secundaire optie

Als Rust echt een te hoge drempel is voor het team dat het overneemt, zou Go met sqlc een alternatief zijn. Sqlc genereert type-safe code uit SQL-queries; niet helemaal hetzelfde als SQLx, maar het komt in de buurt. De taal zelf is veel simpeler te leren.


---

### 4.3 Datastorage

#### Advies

**Aanbevolen: PostgreSQL 16 met hybride relationeel + JSONB model**

Behoud PostgreSQL 16 als enige database. Relationele tabellen voor personen, organisaties en imports. JSONB-kolommen voor de rest, met name de vendor-API-responses.

#### Motivatie

Ik heb bewust gekozen voor één database. Geen apart MongoDB-cluster erbij, geen Redis-cache ernaast. Gewoon PostgreSQL. Daar zat een pragmatische overweging achter die ik pas maakte nadat ik had geëxperimenteerd met MongoDB als aanvulling.

De ACID-garanties van PostgreSQL zijn onmisbaar voor de import-workflow. Die workflow gaat van uploaden naar parsen, dan valideren, dan preview tonen, en dan uitvoeren. Er zit een rollback-optie bij met een venster van 30 dagen. Als er halverwege iets fout gaat, dan mag je niet met half-geïmporteerde data achterblijven. Ik heb daar expliciet op getest en PostgreSQL handelt het goed af; je krijgt alles of niks.

Voor de vendor-cache gebruik ik JSONB-kolommen met GIN-indexen. De API-responses van Atlassian en GitHub hebben elk een eigen structuur en die structuur verandert weleens. Bij relationele kolommen zou ik bij elke API-wijziging een migratie moeten draaien. Met JSONB sla ik de response flexibel op, en door de GIN-indexen blijft de query-performance acceptabel. Niet super snel, maar ruim snel genoeg.

SQLx in combinatie met PostgreSQL was een doorslaggevend punt. Alle SQL-queries worden tijdens compilatie gecontroleerd tegen het echte schema. Na de eerste opzet van het databaseschema heb ik bijna geen SQL-gerelateerde bugs meer gehad. Echt bijna nul.

Eén database houden maakt het leven simpeler. Eén backup-strategie, één ding om te monitoren, één migratieproces. Ik heb overwogen om Redis erbij te zetten voor caching, maar bij minder dan 500 gebruikers is dat overkill. PostgreSQL is snel genoeg.

Qua schaalbaarheid: ik heb 85.842 personeelsrecords succesvol verwerkt, 8 migraties uitgevoerd zonder dataverlies, en de zwaarste query's zitten op 100 tot 150ms. De connection pool staat op max 50, wat voldoende is voor API-verkeer plus achtergrondtaken samen.

#### Afgeraden alternatieven

| Alternatief | Reden voor afwijzing |
|---|---|
| **MongoDB** | Past bij semi-gestructureerde data, maar het systeem heeft ook joins nodig (personen aan organisaties koppelen, vendor-accounts linken). Twee databases naast elkaar maakt alles complexer dan nodig. PostgreSQL's JSONB geeft me beide modellen in één systeem. |
| **MySQL 8** | JSON-ondersteuning is minder volwassen dan PostgreSQL's JSONB. Geen `jsonb_path_query`, beperktere containment operators. En geen compile-time SQL-verificatie via SQLx. |
| **Redis erbij** | Zou de cache sneller maken, maar dat is een extra component om te beheren en te monitoren. Nu niet nodig. Wél aan te raden als het systeem naar 500+ gebruikers groeit (zie hoofdstuk 7). |
| **SQLite** | Niet bedoeld voor meerdere gebruikers die tegelijk schrijven. Het file-level locking zou meteen een bottleneck zijn, zeker met de achtergrondsyncs die parallel draaien. |

#### Secundaire optie

Bij groei richting 500+ gelijktijdige gebruikers adviseer ik om Redis toe te voegen voor het zware leesverkeer. PostgreSQL blijft dan de "bron van waarheid" en Redis doet het cachingwerk.


---

### 4.4 Integraties met externe vendor-API's

#### Advies

**Aanbevolen: gestandaardiseerd integratiepatroon met cache-first strategie en configureerbare sync-jobs**

Behoud het huidige patroon: background sync jobs die elke 24 uur draaien (met startup-sync als de cache leeg is), caching met 25 uur TTL in PostgreSQL die terugvalt op stale data als de vendor-API plat ligt, en per vendor een opbouw van Client, Service en Repository.

#### Motivatie

Dit patroon heb ik niet van tevoren zo bedacht. Het is ontstaan omdat de eerste aanpak niet werkte.

Mijn oorspronkelijke idee was simpel: bij elk dashboardverzoek gewoon live de vendor-API aanroepen. Dat liep bij vijf testgebruikers al tegen de rate limits van Atlassian aan. Zo'n 100 requests per minuut is de grens, en een dashboardpagina die voor 20 mensen tegelijk data ophaalt overschrijdt dat moeiteloos. Hierdoor moest ik overstappen op periodiek synchroniseren en lokaal cachen.

Het resultaat is dat dashboardverzoeken nu onder de 200ms worden afgehandeld. Ze komen gewoon uit de lokale PostgreSQL-cache. Geen wachttijd op een externe API.

Iets waar ik blij mee ben: de graceful degradation. Halverwege het project viel de Atlassian API een keer acht uur uit, gepland onderhoud waar ik niet van op de hoogte was. Omdat het systeem al op de cache-first strategie draaide, bleef het dashboard gewoon werken. Gebruikers zagen een melding dat de data van acht uur geleden was, maar verder functioneerde alles normaal. Dat gaf me vertrouwen dat de aanpak klopt (conform TM-11).

Toen ik GitHub als tweede vendor toevoegde, merkte ik hoe goed het gestandaardiseerde patroon werkt. Nieuwe client-struct, nieuwe service met caching-logica, nieuwe repository voor de database. Hetzelfde recept als bij Atlassian. Dat betekent dat JFrog of Trello toevoegen ook weer hetzelfde stramien volgt. Geen architectuurwijziging nodig.

Bij rate-limited responses doet het systeem exponential backoff: de request wordt herhaald met steeds langere wachttijden. Dat is wat Fielding & Taylor (2002) beschrijven als standaardpraktijk.

#### Afgeraden alternatieven

| Alternatief | Reden voor afwijzing |
|---|---|
| **Real-time API-proxying** | Onhaalbaar. Rate limits, latency van 2 tot 10 seconden per pagina, en volledige afhankelijkheid van externe beschikbaarheid. Als de vendor-API eruit ligt, ligt je dashboard ook plat. |
| **Webhook-based updates** | Atlassian biedt geen webhooks voor licentie- of gebruikerswijzigingen op admin-niveau. GitHub heeft wel webhooks maar niet voor enterprise licentiedata. Een hybride model zou complexiteit toevoegen zonder veel voordeel. |
| **ETL met Apache Airflow** | Met 3 vendors en een dagelijkse sync is een volledig ETL-platform als Airflow gewoon te zwaar. Tokio background tasks doen precies hetzelfde in een fractie van de operationele overhead. Airflow wordt pas nuttig bij 10+ vendors of complexere transformaties. |

#### Secundaire optie

Boven de 5 vendors zou ik kijken naar een message queue, iets als RabbitMQ of NATS, om sync-jobs los te koppelen van de API-server. Dan kan een falende vendor de rest niet blokkeren.


---

### 4.5 Performance en caching

#### Advies

**Aanbevolen: PostgreSQL-gebaseerde caching met indexering, server-side paginatie en frontend bundle-optimalisatie**

Doorgaan met de huidige strategie. Cache in PostgreSQL met 25 uur TTL en een `expires_at`-index. Indexen op `email`, `person_id`, `gid`, `account_id`, `org_id`. Server-side paginatie (standaard 25 per pagina). Connection pool op max 50. Frontend code splitting met lazy loading.

#### Motivatie

Alle performance-doelen die ik vooraf had gesteld zijn gehaald. Maar dat ging niet vanzelf.

De API-responstijden zitten onder de 200ms (P95), getest met k6 bij 100 gelijktijdige gebruikers. Cache-hits worden in zo'n 50ms afgehandeld, databasequeries in 100 tot 150ms. Dat is binnen TM-08. Het voelt in de praktijk gewoon snel: je klikt en het resultaat staat er.

Het dashboard laadt binnen 3 seconden (TM-09). Eerste paint na 500ms, data laden in 1 tot 2 seconden via parallelle requests, rendering nog eens 300ms. Die parallelle requests waren een bewuste keuze. In de eerste versie haalde ik eerst personen op, daarna organisaties, daarna licenties. Sequentieel. Dat duurde 4 seconden. Door alles tegelijk op te halen werd dat 2 seconden.

De productiebundle is 280 KB gzip. Onder de 300 KB van TM-12. Code splitting (aparte chunks voor dashboard, imports, admin) en tree shaking via Vite.

Een pijnlijke les was het N+1 query probleem. In de eerste versie van de personenpagina haalde ik per persoon apart de organisatie op. 100 personen op een pagina, dat zijn 101 queries. De pagina deed er 800ms over. Na het toevoegen van eager loading (één query met een JOIN) ging het naar 120ms. Dat soort problemen merk je pas als je test met realistische hoeveelheden data. Met 10 testrecords lijkt alles snel.

De connection pool op 50 verbindingen is een afweging: te weinig geeft wachttijden bij pieken, te veel overbelast de database. Met 50 heb ik ruimte voor normaal verkeer plus de achtergrondsyncs.

#### Afgeraden alternatieven

| Alternatief | Reden voor afwijzing |
|---|---|
| **In-memory cache (Rust HashMap)** | Weg bij een server-herstart. Vendor-data wordt maar één keer per 24 uur opgehaald, dus dat is onacceptabel. PostgreSQL-cache overleeft herstarts en je kunt er queries op draaien voor debugging. |
| **Redis** | Extra component, extra beheer. Bij minder dan 100.000 records presteert PostgreSQL JSONB met GIN-indexen vergelijkbaar. |
| **Client-side caching (Service Workers)** | Geen consistentie tussen sessies. Server-side invalidatie is onmogelijk. Dashboarddata mag maximaal 25 uur oud zijn. |
| **GraphQL** | De queryvrijheid van GraphQL is niet nodig voor een dashboard met vaste views. REST is simpeler te cachen en heeft een lagere implementatiecomplexiteit. |

#### Secundaire optie

ETags en `Cache-Control`-headers op niet-gepersonaliseerde endpoints zouden de responstijden nog wat verder omlaag kunnen brengen. Geen extra infra nodig.


---

### 4.6 Authenticatie en autorisatie

#### Advies

**Aanbevolen: Microsoft Entra ID (Azure AD) met JWT-validatie en MSAL**

Behoud de huidige opzet. Frontend met MSAL (@azure/msal-react) voor SSO. Backend valideert JWT-tokens tegen het Azure JWKS-endpoint. Rollen via Azure AD-groepen. Tokens in session memory, niet in localStorage.

#### Motivatie

De keuze voor Entra ID was grotendeels pragmatisch. Equans gebruikt het al. Al hun interne applicaties draaien erop. Door aan te sluiten op die bestaande infrastructuur hoef je geen apart inlogsysteem te bouwen en kunnen medewerkers gewoon met hun Equans-account inloggen. Geen nieuw wachtwoord, geen registratieproces. Dat verlaagt de drempel enorm.

Maar er zit ook echt een beveiligingsargument achter. Elke API-call vereist een geldig JWT-token. Geen gedeelde API-keys die iemand per ongeluk ergens naartoe stuurt, geen sessie-cookie die onderschept kan worden (OWASP, 2021). Elke request is traceerbaar naar een specifiek persoon via het `oid`-claim.

De rollen-implementatie bleek makkelijker dan ik dacht. In plaats van zelf iets te bouwen, map ik Azure AD-groepen naar applicatierollen. Zit je `oid` in de admin-groep (geconfigureerd via `admin_group_id`)? Dan ben je admin. Zo niet, dan ben je een gewone gebruiker. IT-beheerders kunnen dit gewoon via het Azure-portaal regelen.

Wat betreft tokenopslag: MSAL slaat tokens op in session memory. Niet in localStorage. Het verschil is dat localStorage benaderbaar is vanuit elke JavaScript-code die op de pagina draait, inclusief een XSS-aanval. Session memory niet (McKay & Cooper, 2019). Tokens leven maximaal 1 uur en worden automatisch ververst.

De JWT-validatie in Rust moest ik helemaal zelf schrijven, want er is geen MSAL-library voor Rust. Ik valideer vier dingen: signature tegen Azure's publieke sleutels, expiratie, issuer, audience. De sleutels worden gecachet. Het debuggen hiervan kostte me twee dagen. Op een gegeven moment kwamen tokens niet door en het bleek dat de audience-check hoofdlettergevoelig was. Dat soort dingen.

#### Afgeraden alternatieven

| Alternatief | Reden voor afwijzing |
|---|---|
| **Session-based (cookies)** | Vereist server-side session storage. Schaalt slechter bij meerdere backend-instances. Geen ingebouwde koppeling met enterprise identity providers. |
| **API-key authenticatie** | Geen gebruikersidentificatie, geen rollen. Als de key uitlekt, heeft iedereen toegang. Niet acceptabel bij persoonsgegevens. |
| **Auth0 / Okta** | Extra externe afhankelijkheid, extra kosten. Entra ID is er al. Functioneel voegt het niets toe. |
| **Zelf JWT-tokens uitgeven** | Dan moet je ook zelf sleutels beheren, revocatie bouwen, MFA ondersteunen. Precies wat een identity provider al doet. Dit zelf bouwen terwijl er een enterprise-oplossing beschikbaar is, vind ik een antipattern. |


---

### 4.7 Privacy en beveiliging

#### Advies

**Aanbevolen: privacy by design met AVG-conforme dataverwerking, e-mailmaskering en auditlogging**

Handhaaf de beveiligingsmaatregelen en versterk ze op een paar punten. E-mailmaskering in logs (john.doe@equans.com wordt j***@e***.com), parameterized SQL via SQLx, TLS 1.2+ overal, AVG-rechten (inzage, rectificatie, verwijdering, portabiliteit), logging met correlation IDs, en een retentiebeleid: licentiedata 2 jaar, loginhistorie 1 jaar, IP-adressen 90 dagen.

#### Motivatie

Het systeem verwerkt persoonsgegevens: namen, e-mailadressen, accountstatussen, licentietoewijzingen. Dat maakt de AVG van toepassing. Ik heb vanaf het begin rekening gehouden met privacy by design (Art. 25), dataminimalisatie (Art. 5) en het recht op verwijdering (Art. 17). Niet omdat dat een afrader was, maar omdat het achteraf toevoegen van privacy-maatregelen aan een bestaand systeem een nachtmerrie is. Dat heb ik bij een eerder project gezien en dat wilde ik vermijden.

SQL-injectie is in dit project eigenlijk structureel onmogelijk gemaakt. SQLx dwingt parameterized queries af op compilatieniveau. Je kunt geen dynamische SQL-string in elkaar plakken; de compiler laat het niet toe (OWASP A03:2021). Dat is een sterkere garantie dan alleen runtime-bescherming.

XSS-preventie zit op twee niveaus. Tokens in session memory (niet localStorage), dus een XSS-aanval kan ze niet lezen. En Content Security Policy headers die inline scripts beperken.

Ik heb alle tien categorieën van de OWASP Top 10 (2021) geadresseerd. Niet allemaal even diep, maar ze zijn alle tien afgedekt. Geverifieerd met specifieke security tests (AUTH-001 t/m 005, AUTHZ-001 t/m 004, TOK-001 t/m 006).

In het Privacy- en Beveiligingsplan staan acht risico's (P-01 t/m P-08), elk met kans, impact en maatregel. Wat ik daarvan heb geleerd: beveiliging is geen feature die je aan het eind toevoegt. Het moet in je ontwerpkeuzes zitten, vanaf dag één.

De lastigste afweging was logging. Je wilt genoeg loggen om bugs te debuggen en voor compliance. Maar je mag er niet zomaar e-mailadressen in dumpen. Het duurde een paar iteraties om de juiste balans te vinden. De drielaagse maskeringsstrategie (logs, API-responses, staging-omgevingen) is het resultaat van die iteraties.

#### Afgeraden alternatieven

| Alternatief | Reden voor afwijzing |
|---|---|
| **Alles in de database versleutelen** | De performance-impact weegt niet op tegen het voordeel. Alleen de gevoeligste velden (API-tokens, credentials) versleutelen, gecombineerd met PostgreSQL's AES-256 TDE voor at-rest encryptie, is een betere balans. |
| **Tokens in localStorage** | Kwetsbaar voor XSS. MSAL's session memory is veiliger. |
| **Helemaal geen PII loggen** | Klinkt goed op papier, maar als er een bug opduikt met een specifiek account, moet je dat kunnen traceren. Gemaskeerde PII is het compromis. |


---

## 5. Risico's en aandachtspunten

Er zitten risico's aan de keuzes die ik in dit rapport adviseer. Ik wil daar eerlijk over zijn. Hieronder heb ik ze gegroepeerd in drie categorieën.

### 5.1 Technologierisico's

| # | Risico | Kans | Impact | Beheersmaatregel |
|---|---|---|---|---|
| R-01 | **Rust leercurve**: niet iedereen kent Rust. Nieuwe teamleden moeten het eerst leren en dat kost tijd | Middel | Hoog | Er is uitgebreide documentatie geschreven (ADR's, codestandaarden). Een onboarding-guide helpt. Voor minder kritieke services kan eventueel Go worden overwogen. |
| R-02 | **Vendor-API breaking changes**: Atlassian of GitHub veranderen hun API | Middel | Hoog | JSONB vangt structuurwijzigingen deels op. Per vendor een API-client met adapter pattern. Deprecation-meldingen van vendors actief in de gaten houden. |
| R-03 | **PostgreSQL schaalbaarheid**: bij 500+ gelijktijdige gebruikers kan de database het cachingwerk niet meer alleen aan | Laag | Middel | Het cache-repository trait in de architectuur maakt het mogelijk om later Redis toe te voegen met minimale codewijzigingen. |
| R-04 | **MSAL afhankelijkheid**: Microsoft past de library aan | Laag | Middel | MSAL is Microsoft's eigen standaard. De AuthContext abstractielaag isoleert de rest van de code van directe MSAL-calls. |

### 5.2 Operationele risico's

| # | Risico | Kans | Impact | Beheersmaatregel |
|---|---|---|---|---|
| R-05 | **Geen penetratietest gedaan**: er kunnen kwetsbaarheden zijn die onze interne tests niet hebben gevonden | Middel | Hoog | Voor productielancering een CREST-gecertificeerde partij inschakelen voor een pentest. De OWASP-tests die ik heb gedaan dekken niet alles. |
| R-06 | **Tarieven veranderen**: vendors passen hun prijsmodel aan zonder dat het systeem wordt bijgewerkt | Middel | Middel | De tarieven zitten in `productPricing.ts` en zijn snel te wijzigen. Stel een kwartaalcontrole in om de geconfigureerde tarieven te vergelijken met echte facturen. |
| R-07 | **GID-matching fouten**: de heuristische matcher koppelt de verkeerde persoon aan het verkeerde account | Middel | Middel | Het confidence-scoresysteem (0-100) markeert onzekere matches. Onder de drempelwaarde is handmatige controle nodig. |
| R-08 | **Onvolledige AVG-verwijdering**: bij een verwijderverzoek worden niet alle gegevens gewist | Laag | Hoog | De geautomatiseerde procedure (TS-06) is getest met GDPR-001 en GDPR-002. Periodieke audits zijn aan te raden. |

### 5.3 Projectrisico's

| # | Risico | Kans | Impact | Beheersmaatregel |
|---|---|---|---|---|
| R-09 | **Kennisborging**: na mijn afstuderen verdwijnt de domeinkennis | Hoog | Hoog | Ik heb veel gedocumenteerd: 6 ADR's, 12 FR's, 12 TR's, een SDD, een Master Test Plan. Er is ook een technische walkthrough gepland. De AI-agents codificeren de reviewstandaarden. Maar ik maak me hier wel zorgen over. Documentatie is niet hetzelfde als iemand die het systeem kent. |
| R-10 | **Scope creep bij nieuwe vendors**: JFrog of Trello toevoegen verandert aannames | Middel | Middel | Het Client - Service - Repository patroon is gestandaardiseerd. De toevoeging van GitHub bewees dat nieuwe vendors hetzelfde stramien volgen. |

---

## 6. Eindadvies

### 6.1 Overkoepelend advies

Na het onderzoek, het bouwen van het systeem, en het vergelijken van alternatieven, kom ik tot dit advies:

**Ga door met de huidige technologiestack (Rust + Axum, React + TypeScript, PostgreSQL) en gebruik de vastgelegde architectuurpatronen als basis voor verdere ontwikkeling.**

Ik adviseer dit niet zomaar. De stack heeft tijdens het project bewezen te werken. Het is geen theoretisch advies; het is gebaseerd op een werkend systeem met 12 geïmplementeerde functionele requirements, geteste performance, en beveiliging die de OWASP Top 10 afdekt.

### 6.2 Aanbevolen technologiecombinatie

| Laag | Technologie | Motivatie |
|---|---|---|
| **Frontend** | React 19 + TypeScript + Vite + Radix UI + Tailwind CSS | Type-safe, organisatiestandaard, bundle onder 300 KB |
| **Backend** | Rust + Axum 0.7 + SQLx + Tokio | Compile-time veiligheid, async, P95 onder 200ms |
| **Database** | PostgreSQL 16 (relationeel + JSONB) | ACID-compliant, hybride opslag, compile-time SQL-checks |
| **Authenticatie** | Microsoft Entra ID + MSAL + JWT | Bestaande SSO, zero-trust, rollen via AD-groepen |
| **Caching** | PostgreSQL met 25 uur TTL | Simpel, persisteert over herstarts |
| **Integraties** | Cache-first met background sync | Respecteert rate limits, werkt bij vendor-uitval |
| **Beveiliging** | OWASP Top 10, AVG-conform, e-mailmaskering | Wettelijke compliance |

### 6.3 Waarom deze combinatie

Ik heb hier best lang over nagedacht en mijn conclusie is dat het niet gaat om de "beste" technologie per onderdeel. Het gaat om hoe ze samenwerken.

**Veiligheid** zit verweven in de hele stack. Rust vangt fouten op compilatieniveau. SQLx controleert SQL-queries voordat je ze ooit uitvoert. JWT via Entra ID zorgt dat elke request aan een persoon gekoppeld is. Beveiliging is hier geen achteraf-toevoeging maar een eigenschap van de technologiekeuzes zelf.

**Performance** is gehaald zonder externe caching-infra. P95 onder 200ms, dashboard laadt in 3 seconden, bundle onder 300 KB. Puur PostgreSQL met goede indexen.

**Schaalbaarheid** is niet overengineered. De stateless API kan achter een load balancer. Redis kan later worden toegevoegd voor caching. Het vendor-patroon is al bewezen uitbreidbaar met de tweede vendor.

**Onderhoudbaarheid** zit in de documentatie en de architectuur. Gelaagde structuur, type safety aan beide kanten, en AI-agents die code reviews gestandaardiseerd houden.

**Compliance** was er vanaf dag één. AVG, OWASP Top 10, TLS 1.2+, logging met maskering, configureerbaar retentiebeleid.

---

## 7. Aanbevelingen voor doorontwikkeling

### 7.1 Korte termijn (0-3 maanden)

| Prioriteit | Aanbeveling | Toelichting |
|---|---|---|
| **Hoog** | Externe beveiligingsaudit | Laat een CREST-gecertificeerde partij een penetratietest doen op de API en de authenticatieflow. Voor productielancering. |
| **Hoog** | API rate limiting | Op dit moment zit er geen rate limiting op onze eigen endpoints. Dat moet erin, per-gebruiker en per-IP (token bucket). |
| **Hoog** | Monitoring | Prometheus metrics, logs naar Azure Monitor of ELK. Zonder monitoring vlieg je blind. |
| **Middel** | API-versioning | Beleid maken voor backward compatibility bij toekomstige API-wijzigingen, of dat nu via URL-path of headers is. |
| **Middel** | UAT met gebruikers | Formele acceptatietest met licentiemanagers en financemensen. Klopt het dashboard met hoe zij beslissingen nemen? |

### 7.2 Middellange termijn (3-6 maanden)

| Prioriteit | Aanbeveling | Toelichting |
|---|---|---|
| **Hoog** | JFrog-integratie | Derde vendor toevoegen via het bestaande Client - Service - Repository patroon. Was Could Have C-04. |
| **Hoog** | Chargeback-module | Kosten automatisch toerekenen aan afdelingen. De data is er (licenties + organisatiestructuur uit CSV), de berekening moet geautomatiseerd worden. |
| **Middel** | Alertering | Notificaties bij ongebruikelijke kostenpieken, lang inactieve accounts, overschrijdingen. Maakt het dashboard proactief. |
| **Middel** | Export uitbreiden | CSV en Excel export van alle data. Het financeteam wil dit in hun Power BI-rapporten gebruiken. |
| **Laag** | Circuit breaker | De exponential backoff werkt, maar een echte circuit breaker (bijv. via `tower`) is robuuster bij lange vendor-storingen. |

### 7.3 Lange termijn (6-12 maanden)

| Prioriteit | Aanbeveling | Toelichting |
|---|---|---|
| **Middel** | Redis cache | Bij 500+ gelijktijdige gebruikers. Het repository trait maakt de overstap technisch eenvoudig. |
| **Middel** | Predictive analytics | Licentie-forecasting op basis van historische data. In plaats van achteraf rapporteren, vooruit adviseren. |
| **Middel** | Event sourcing | Voor compliance: elke wijziging traceerbaar en herstelbaar. |
| **Laag** | Power BI-integratie | Read-only API of connector voor het bestaande BI-tooling van finance. |
| **Laag** | Anomalie-detectie | ML-gebaseerd, voor het herkennen van vreemde patronen in licentiegebruik. Was Could Have C-01. |

### 7.4 Visuele roadmap

```
Q2 2026        Q3 2026           Q4 2026          Q1 2027
────────────── ───────────────── ──────────────── ──────────────
Security audit  JFrog integratie  Redis cache      Predictive
Rate limiting   Chargeback module Circuit breaker  analytics
Monitoring      Alerting          Event sourcing   Power BI
API versioning  Export uitbreiden                   Anomalie-det.
UAT                                                 
```

---

## 8. Bronnen

1. Angular. (z.d.). *Angular Documentation*. https://angular.io/docs

2. Atlassian. (2024, 23 oktober). *How maximum quantity billing works*. Atlassian Support. https://support.atlassian.com/subscriptions-and-billing/docs/how-maximum-quantity-billing-works/

3. Atlassian. (z.d.). *Atlassian Cloud Admin REST API*. https://developer.atlassian.com/cloud/admin/

4. Blokdyk, G. (2020). *IT Cost Transparency A Complete Guide - 2020 edition*. 5starcooks.

5. Few, S. (2013). *Information Dashboard Design: Displaying Data for At-a-glance Monitoring*. Analytics Press.

6. Fielding, R. T., & Taylor, R. N. (2002). Principled design of the modern Web architecture. *ACM Transactions On Internet Technology*, 2(2), 115-150. https://doi.org/10.1145/514183.514185

7. GitHub. (z.d.). *GitHub Enterprise Cloud REST API*. https://docs.github.com/en/enterprise-cloud@latest

8. GitHub. (z.d.). *REST API endpoints for Copilot user management*. https://docs.github.com/en/enterprise-cloud@latest/rest/copilot/copilot-user-management

9. Klabnik, S., & Nichols, C. (2023). *The Rust Programming Language, 2nd Edition*. No Starch Press.

10. Martin, R. C. (2017). *Clean Architecture: A Craftsman's Guide to Software Structure and Design*. Prentice Hall.

11. McKay, K. A., & Cooper, D. A. (2019). *Guidelines for TLS Implementations* (NIST SP 800-52r2). National Institute of Standards and Technology. https://doi.org/10.6028/nist.sp.800-52r2

12. Microsoft. (z.d.). *Microsoft Authentication Library (MSAL) Overview*. Microsoft Learn. https://learn.microsoft.com/en-us/entra/identity-platform/msal-overview

13. OWASP Foundation. (2021). *OWASP Top Ten*. https://owasp.org/www-project-top-ten/

14. PostgreSQL Global Development Group. (2024). *PostgreSQL 16 Documentation*. https://www.postgresql.org/docs/16/

15. Regulation (EU) 2016/679 (GDPR). EUR-Lex. https://eur-lex.europa.eu/eli/reg/2016/679/oj

16. React. (z.d.). *React Documentation*. https://react.dev/

17. Tufte, E. R. (2001). *The Visual Display of Quantitative Information* (2nd ed.). Graphics Press.
