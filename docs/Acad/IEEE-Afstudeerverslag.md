# Equans Operational Insights: Een Full-Stack Dashboard voor Softwarelicentie- en Gebruiksbeheer

**Ahmad Alhaj Asaad**
CMI-Informatica
1035912@hr.nl

---

## Abstract

Bij Equans wist eigenlijk niemand precies hoeveel softwarelicenties er draaiden, wat dat kostte, en hoeveel accounts er gewoon stil lagen. Alles zat verspreid over losse admin-portals van Atlassian, GitHub Enterprise en JFrog. Binnen dit afstudeerproject heb ik een dashboard gebouwd dat die data automatisch ophaalt en bruikbaar maakt. De backend draait op Rust met Axum en SQLx, de frontend op React 19 met TypeScript, en alles slaat op in PostgreSQL 16. Twaalf functionele requirements zijn geimplementeerd: van persoons- en organisatiebeheer tot CSV-import, dagelijkse synchronisatie met vendorsystemen en authenticatie via Microsoft Entra ID. Toen ik voor het eerst de Atlassian-data analyseerde, bleek dat zo'n 5 tot 10 procent van alle licenties op accounts stond die al meer dan 90 dagen niks hadden gedaan. Bij de aantallen van Equans gaat dat echt over serieus geld.

**Keywords** -- Licentieoptimalisatie, Rust, Axum, React, Atlassian Cloud API, GitHub Enterprise API, Dashboard, PostgreSQL, Chargeback

---

## I. Introductie

Equans is een grote technische dienstverlener waar duizenden mensen dagelijks met Jira, Confluence, GitHub Enterprise en nog meer tools werken. De afdeling DevOps Forge beheert die platformen. Toen ik daar begon met mijn stage viel me vrij snel iets op: wil je weten hoeveel Jira-licenties er actief zijn? Dan log je in op de Atlassian-beheerconsole. Wil je weten wat GitHub kost? Ander portaal. Wil je vervolgens uitrekenen wat afdeling X kwijt is aan software? Dan zit je handmatig spreadsheets aan elkaar te knopen. Dat was de situatie.

Het project duurde 20 weken, verdeeld over 8 sprints. Ik was de enige ontwikkelaar, wat achteraf zowel een voordeel als een nadeel bleek. Voordeel: ik kon snel beslissingen nemen zonder eindeloos af te stemmen. Nadeel: als je ergens vastloopt is er niemand die even meekijkt. Brian Veltman (mijn technisch begeleider) hielp wel mee bij code reviews, maar het daadwerkelijke bouwen was aan mij.

### A. Probleemstelling

Equans miste een centraal overzicht van licentiegebruik. Kosten doorberekenen aan teams was handwerk en foutgevoelig. Ongebruikte licenties werden niet systematisch opgespoord. En medewerkers waren nergens gekoppeld aan hun organisatorische eenheid, waardoor rapportage lastig was.

Wat ik pas later ontdekte: Atlassian werkt met Maximum Quantity Billing. Je betaalt voor de piek van het aantal licenties in een factureringsperiode [1]. Zelfs als je aan het eind van de maand opruimt, betaal je voor het hoogste punt. Dat maakt continu monitoren eigenlijk noodzakelijk, niet iets wat je een keer per kwartaal doet.

### B. Onderzoeksvragen

Hoofdvraag: _"Hoe kan Equans inzicht krijgen in het gebruik van Atlassian Cloud Enterprise licenties en de bijbehorende kosten, om besparingsmogelijkheden te identificeren en te optimaliseren?"_

Vijf deelvragen:

1. Welke datastructuren en responseformaten bieden de Atlassian Cloud API en GitHub Enterprise Cloud API?
2. Hoe kan actief licentiegebruik gemeten worden, gegeven de beperkingen van de API's?
3. Hoe kan een kostenanalyse per product, site en team worden uitgevoerd?
4. Welke patronen van inefficient licentiegebruik zijn te herkennen?
5. Hoe presenteer je die informatie het effectiefst in een dashboard?

---

## II. Achtergrond

Eerlijk gezegd had ik niet verwacht dat ik me zo in IT Financial Management zou moeten verdiepen voor dit project. Maar het bleek al snel dat je geen nuttig dashboard kunt bouwen zonder te snappen hoe kostenallocatie werkt. Blokdyk [2] gaf me een goed beeld van hoe je licentiekosten kunt structureren per organisatorische eenheid. Few [3] hielp met het ontwerp van het dashboard zelf: welke grafieken werken, hoe je visuele ruis vermijdt, en waarom KPI's direct zichtbaar moeten zijn.

Technisch heb ik me gebaseerd op het Repository Pattern van Fowler [4] om de database-logica gescheiden te houden van de rest. Dat klinkt misschien vanzelfsprekend, maar in het begin had ik stukken SQL in mijn route-handlers staan. Dat werkte prima tot het onoverzichtelijk werd. Fowler's aanpak dwong me om na te denken over waar welke code thuishoort. Klabnik en Nichols [5] waren mijn referentie voor Rust zelf, vooral voor de ownership-regels en async patronen, want daar liep ik in het begin flink tegenaan.

Wat betreft de API's: de Atlassian Cloud Admin API levert gebruikers-, groeps- en licentiegegevens, maar dus geen billing-data [1]. Dat was een tegenvaller. Bij GitHub Enterprise was het een ander verhaal, daar geeft de API netjes `total_seats_consumed` versus `total_seats_purchased`, wat de integratie een stuk makkelijker maakte.

---

## III. Resultaten

Ik heb uiteindelijk een werkend systeem opgeleverd met een Rust-backend, een React-frontend en een PostgreSQL-database. Twaalf functionele requirements zijn afgerond (FR-001 t/m FR-012). Niet alles ging zoals gepland, maar het eindresultaat doet wat het moet doen.

### A. Backend

De backend is geschreven in Rust met Axum 0.7, draaiend op Tokio. In het begin was Rust echt frustrerend. Er waren dagen dat ik meer tijd kwijt was aan het oplossen van compileerfouten dan aan het schrijven van nieuwe functionaliteit. Na een paar weken ging het beter, maar die eerste periode was pittig. De ownership-regels en lifetimes zijn gewoon lastig als je ze voor het eerst tegenkomt.

Hierbij is gekozen voor een gelaagde architectuur: route-handlers, services en repositories. In de eerste weken had ik logica in de route-handlers staan die daar niet thuishoorde. Op een gegeven moment had ik een handler van 80 regels. Dat was het moment dat ik dacht: dit moet anders. SQLx 0.8 gebruik ik als database-client, en het bijzondere daaraan is dat SQL-queries al tijdens het compileren worden gevalideerd tegen het echte schema. In de praktijk betekent dat nul SQL-runtime-fouten, wat echt een opluchting was.

De Atlassian-integratie cachet data lokaal met een TTL van 25 uur. Mijn eerste aanpak was simpel: bij elke paginalading de data vers ophalen bij Atlassian. Dat werkte prima met een gebruiker. Met vijf gebruikers tegelijk liep het al vast vanwege rate limits. Dus ik moest een hele andere strategie bedenken, en daar is het caching-systeem uit voortgekomen. Achtergrondtaken in `jobs/daily_sync.rs` en `jobs/github_sync.rs` synchroniseren elke 24 uur automatisch.

De data-importmodule bleek het complexste stuk van het hele project. CSV-bestanden van Equans zijn niet klein: 85.000+ rijen. De pipeline verwerkt ze via uploaden, type-detectie, parsen, valideren, preview genereren en dan pas wegschrijven in een databasetransactie. Ik moest `tokio::task::spawn_blocking` gebruiken om de webserver niet te blokkeren. Eerlijk gezegd had ik de import-module helemaal niet zo groot ingepland. Pas in Sprint 2 ontdekte ik dat organisatiegegevens (kostenplaats, manager, budget) helemaal niet in de vendor-API's zitten, maar via CSV uit Palantir Foundry moesten worden geimporteerd. Dat was een flinke tegenvaller.

De GID-matcher (`persons/gid_matcher.rs`) koppelt personen aan hun GlobalID via een puntensysteem: e-mail match levert 30 punten, lokaal ID 20, GitHub-username 10, Atlassian-account 10. De som geeft een confidence score van 0 tot 100. Perfect is het niet. Mensen hebben soms een persoonlijk mailadres bij GitHub, of namen zijn net anders gespeld ("De Vries" versus "de Vries" versus "DeVries"). Maar voor de meeste gevallen werkt het acceptabel.

### B. Frontend

React 19, TypeScript 5.9, Vite 6. Die combinatie bevalt goed. Vite is echt merkbaar sneller dan Webpack, je slaat een bestand op en het is er meteen. De productiebundle is 280 KB gzip, laadtijd onder de 3 seconden.

Alle API-calls gaan via een gecentraliseerde client in `backendClient.ts` met een generieke `fetchApi<T>()` functie. Tijdens het ontwikkelen viel op dat zo'n centrale client veel duplicatie voorkomt, zeker wanneer er steeds meer endpoints bijkomen. State management is bewust simpel gehouden met `useState`, `useEffect` en Context API. Geen Redux. Bij een vorig project had ik Redux ingezet en achteraf vond ik het al te veel gedoe. Hier leeft de data voornamelijk server-side, dus ik zag geen reden voor al die boilerplate.

De UI-componenten zijn gebouwd met Radix UI en Tailwind CSS. Voor de Equans-huisstijl heb ik het kleurpalet overgenomen (donkerblauw #002439, donkergroen #008163, turquoise #70BD95), maar niet blindelings. Leesbaarheid wint het van strikt de brandguide volgen. Viktor en ik zaten op een gegeven moment naar het scherm te kijken en hij zei: "het is best druk." Hij had gelijk. Vanaf dat moment maar een accentkleur per pagina.

### C. Database

PostgreSQL 16 met een mix van relationele tabellen en JSONB-kolommen. Drie categorieen: kerntabellen (`persons`, `organizations`, `imports`), cachetabellen (`atlassian_users_cache`, `github_users_cache`) met TTL-invalidatie, en audittabellen voor traceerbaarheid. Full-text search met `tsvector` en GIN-indexering haalde de zoekprestaties bij 85.000 records terug van 400ms naar een paar milliseconden. Dat verschil was echt enorm. Ik had eerst overwogen om MongoDB te gebruiken voor de ruwe API-responses, maar PostgreSQL's JSONB dekte dat prima af.

| Laag     | Technologie               | Versie           |
| -------- | ------------------------- | ---------------- |
| Backend  | Rust + Axum               | 0.7              |
| Database | PostgreSQL + SQLx         | 16 + 0.8         |
| Frontend | React + TypeScript + Vite | 19.2 + 5.9 + 6.4 |
| Auth     | Microsoft Entra ID        | OAuth 2.0        |
| Infra    | Docker + Docker Compose   | Latest           |

_Tabel I. Technologiestack._

### D. Beantwoording onderzoeksvragen

Bij deelvraag 1 heb ik de Atlassian en GitHub API's geanalyseerd. Hierbij bleek dat Atlassian niet overal dezelfde JSON-structuur hanteert, wat later invloed had op hoe ik de data-mapping heb opgezet. GitHub was een stuk consistenter.

Bij deelvraag 2 liep ik tegen het feit aan dat Atlassian geen historische usage trends biedt. De oplossing: zelf snapshots maken door dagelijks de last-active data op te halen en lokaal op te slaan. Niet ideaal, maar het werkt.

Bij deelvraag 3 moest ik kostenberekeningen helemaal zelf bouwen op basis van seat-aantallen en tarieven. Ik had gehoopt dat de API gewoon prijzen zou leveren. Dat is niet zo.

Bij deelvraag 4 bleek 5 tot 10 procent van alle licenties op inactieve accounts te staan (meer dan 90 dagen geen activiteit). Daarnaast vond ik externe gebruikers met billable toegang en structurele overallocatie.

Bij deelvraag 5 heb ik het Hub and Spoke-navigatiemodel gekozen. Een lineaire flow paste niet bij hoe beheerders echt werken: die switchen constant heen en weer. KPI-kaarten maken de hoofdzaken direct zichtbaar, drill-down pagina's geven de details.

---

## IV. Competenties

### A. Professional Skills, Manage and Control

Het project heb ik bijgehouden in Jira (SDPDOFS), 8 sprints, feature branches per issue. Elke ochtend was er een stand-up met het DevOps Forge-team. In het begin voelde dat onwennig (als stagiair tussen ervaren developers), maar later merkte ik dat die dagelijkse afstemming hielp om op koers te blijven.

De Definition of Done was: code af, tests geslaagd, review door Brian, geen `unwrap()` in productiecode, Jira bijgewerkt, gemerged naar main. Communicatie was soms tricky: het team was internationaal (Frans, Indiaas, Nederlands). Met Viktor praatte ik over functionaliteit en resultaten, met Brian over code en architectuur. Viktor leerde me al in Sprint 1: "Laat zien wat er werkt, praat niet om de techniek heen."

Een les die ik pas vrij laat leerde: documentatie als Jira-taken plannen, niet als bijzaak. In Sprint 7 raakte ik in tijdnood met de scriptie, juist omdat ik dat steeds had uitgesteld.

### B. Analyse

Ik begon met interviews met drie stakeholders: Viktor Klein (Product Owner), Brian Veltman (developer) en Henk Soppe (Director SLS). Ieder had andere verwachtingen. Viktor wilde stuurinformatie, Henk wilde kostenoverzichten, Brian wilde weten of het technisch schaalbaar was. Die gesprekken leverden de MoSCoW-prioritering op in het SRS-document.

Daarna heb ik de API-documentatie van Atlassian en GitHub doorgespit. Hieruit bleek dat Atlassian geen billing-data levert, wat flinke gevolgen had. Ik moest de kostberekening helemaal zelf bouwen. Op de beschikbare data heb ik vervolgens een empirische analyse gedaan, en daarbij kwam dat percentage inactieve licenties bovendrijven.

### C. Ontwerp

In Figma heb ik twee rondes gedaan: ruwe wireframes en daarna nettere mockups. Die eerste ronde was met opzet lelijk gehouden. Als iets er te "af" uitziet, gaan stakeholders zeuren over kleurtjes terwijl ik wilde weten of de indeling klopte. In het Software Design Document (SDD-001) staan de architectuurkeuzes, het ERD en de sequentiediagrammen.

### D. Realisatie

De hele stack gebouwd als enige ontwikkelaar. Twaalf functionele requirements, negen frontend-pagina's, acht SQL-migraties. Testen volgden een risk-based aanpak: unit tests in Rust voor de complexe logica (import-parsing, GID-matching), integratietests voor de samenwerking tussen componenten, performance tests (P95 onder 200ms bij 100 gebruikers), security tests (JWT, CORS, OWASP) en usability tests met twee eindgebruikers via de think-aloud methode. Die usability tests leverden trouwens verrassende inzichten op: gebruikers begrepen het verschil tussen "nieuw" en "bijgewerkt" bij de import-preview niet, en de zoekbalk viel niet genoeg op.

### E. Advies

In het Technisch Adviesrapport heb ik per onderdeel beschreven wat ik aanraad en waarom. Kort samengevat: React + TypeScript + Vite behouden (Angular te veel boilerplate, Vue geen standaard bij Equans), Rust + Axum + SQLx behouden (met Go als terugvaloptie als Rust een te hoge drempel vormt), PostgreSQL behouden. Daarnaast: kwartaalaudits op inactieve licenties formaliseren en het systeem uitbreiden met trendanalyses en koppeling aan Palantir Foundry.

---

## V. Conclusie en Reflectie

De Atlassian API's leveren genoeg data om een betrouwbaar beeld van licentiegebruik op te bouwen, ook al ontbreekt er een billing-API. Dat was in het begin een tegenvaller, maar door seat-aantallen te combineren met configureerbare tarieven en organisatorische mapping kon ik toch een werkbare kostenberekening opzetten.

Het inefficiente gebruik zit hem vooral in inactieve accounts (5-10% van alle licenties), externe gebruikers die nog op een betaald account staan, en structurele overallocatie van seats. Die patronen zijn met dagelijkse dataverzameling systematisch te vinden.

Terugkijkend heb ik een paar dingen geleerd die ik niet uit een boek had gehaald. Rust was steiler dan verwacht. De import-module was veel groter dan ingepland. JFrog heb ik bewust buiten scope gehouden, anders was de planning niet haalbaar geweest. End-to-end tests (Cypress, Playwright) heb ik niet meer kunnen doen vanwege tijdsdruk. En de SSO-integratie met Microsoft Entra ID kostte extra werk omdat er simpelweg geen MSAL-library voor Rust bestaat. Die JWT-validatie moest ik zelf schrijven.

Maar het eindresultaat is er. Equans heeft nu een dashboard waarmee ze concreet kunnen zien waar licentiegeld heen gaat, welke accounts opgeruimd kunnen worden, en hoe kosten per afdeling verdeeld liggen.

---

## VI. Op te Leveren Producten

De Graduate Folder is ingedeeld naar de vijf HBO-competenties. In Analysis staan het onderzoeksverslag, het gebruikersbehoeftenonderzoek en de SRS. In Design het Software Design Document en de Figma-mockups. Realisation bevat het realisatiedocument, de volledige broncode (Rust backend, React frontend, SQL-migraties, Docker Compose) en alle testplannen en testresultaten. In Advice staat het technisch adviesrapport. En in Professional Skills, Manage and Control staan het projectbeheersdocument, de sprintsplanning, het Master Test Plan en mijn reflectie op professionele vaardigheden.

---

## Referenties

[1] Atlassian. (z.d.). _About Cloud Admin REST APIs_. Geraadpleegd via https://developer.atlassian.com/cloud/admin/rest/

[2] G. Blokdyk. (2020). _IT Cost Allocation: A Complete Guide_. 5STARCooks.

[3] S. Few. (2013). _Information Dashboard Design: Displaying Data for At-a-Glance Monitoring_ (2nd ed.). Analytics Press.

[4] M. Fowler. (2002). _Patterns of Enterprise Application Architecture_. Addison-Wesley.

[5] S. Klabnik en C. Nichols. (2023). _The Rust Programming Language_ (2nd ed.). No Starch Press.
