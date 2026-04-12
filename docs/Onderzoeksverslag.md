# Onderzoeksverslag

## Operational Insights Dashboard

### Proof of Concept voor Licentie & Gebruiksdata Visualisatie

![Afbeelding met tekst, schermopname, Lettertype, nummer](media/image1.png)

**Project:** Operational Insights Dashboard
**Student:** Ahmad Alhaj Asaad
**Opleiding:** HBO-ICT
**Bedrijf:** Equans
**Versie:** 1.0

---

## Inhoudsopgave

[1. Inleiding](#1-inleiding)
[2. Methodologie](#2-methodologie)
[3. Analyse van API-datastructuren en responseformaten](#3-analyse-van-api-datastructuren-en-responseformaten)
&nbsp;&nbsp;&nbsp;&nbsp;[3.1 Atlassian Cloud API – Gebruikers, Groepen en Licenties](#31-atlassian-cloud-api-gebruikers-groepen-en-licenties)
&nbsp;&nbsp;&nbsp;&nbsp;[3.2 GitHub Enterprise Cloud API – Licenties en gebruiksdata](#32-github-enterprise-cloud-api-licenties-en-gebruiksdata)
&nbsp;&nbsp;&nbsp;&nbsp;[3.3 Beperkingen van de Atlassian API en de rol van aanvullende data](#33-beperkingen-van-de-atlassian-api-en-de-rol-van-aanvullende-data)
[4. Meten van actief licentiegebruik](#4-meten-van-actief-licentiegebruik)
&nbsp;&nbsp;&nbsp;&nbsp;[4.1 Beperking van de Atlassian API: Geen historische usage trends](#41-beperking-van-de-atlassian-api-geen-historische-usage-trends)
&nbsp;&nbsp;&nbsp;&nbsp;[4.2 Technische oplossing: Zelf historische data opslaan](#42-technische-oplossing-zelf-historische-data-opslaan)
[5. Kostenanalyse per product, site en team](#5-kostenanalyse-per-product-site-en-team)
&nbsp;&nbsp;&nbsp;&nbsp;[5.1 Kostenmodel Atlassian Cloud Enterprise](#51-kostenmodel-atlassian-cloud-enterprise)
&nbsp;&nbsp;&nbsp;&nbsp;[5.2 Kostenmodel GitHub Enterprise Cloud](#52-kostenmodel-github-enterprise-cloud)
&nbsp;&nbsp;&nbsp;&nbsp;[5.3 Inzicht per product](#53-inzicht-per-product)
&nbsp;&nbsp;&nbsp;&nbsp;[5.4 Inzicht per site](#54-inzicht-per-site)
&nbsp;&nbsp;&nbsp;&nbsp;[5.5 Inzicht per team (chargeback-structuur)](#55-inzicht-per-team-chargeback-structuur)
&nbsp;&nbsp;&nbsp;&nbsp;[5.6 Architecturale vereisten voor betrouwbare kostenanalyse](#56-architecturale-vereisten-voor-betrouwbare-kostenanalyse)
&nbsp;&nbsp;&nbsp;&nbsp;[5.7 Visualisatie en besluitvorming](#57-visualisatie-en-besluitvorming)
[6. Patronen van inefficiënt gebruik](#6-patronen-van-inefficiënt-gebruik)
&nbsp;&nbsp;&nbsp;&nbsp;[6.1 Langdurig inactieve accounts](#61-langdurig-inactieve-accounts)
&nbsp;&nbsp;&nbsp;&nbsp;[6.2 Externe gebruikers met billable toegang](#62-externe-gebruikers-met-billable-toegang)
&nbsp;&nbsp;&nbsp;&nbsp;[6.3 Governance en overmatige adminrechten](#63-governance-en-overmatige-adminrechten)
&nbsp;&nbsp;&nbsp;&nbsp;[6.4 Structurele overallocatie van licenties](#64-structurele-overallocatie-van-licenties)
&nbsp;&nbsp;&nbsp;&nbsp;[6.5 Ontbrekende kostenallocatie per organisatorische eenheid](#65-ontbrekende-kostenallocatie-per-organisatorische-eenheid)
[7. Presentatie in dashboard en aanbevelingen](#7-presentatie-in-dashboard-en-aanbevelingen)
&nbsp;&nbsp;&nbsp;&nbsp;[7.1 Overzichtelijkheid en directe zichtbaarheid van KPI's](#71-overzichtelijkheid-en-directe-zichtbaarheid-van-kpis)
&nbsp;&nbsp;&nbsp;&nbsp;[7.2 Hiërarchische ordening van informatie](#72-hiërarchische-ordening-van-informatie)
&nbsp;&nbsp;&nbsp;&nbsp;[7.3 Keuze van visualisatietypen](#73-keuze-van-visualisatietypen)
&nbsp;&nbsp;&nbsp;&nbsp;[7.4 Van inzicht naar concrete aanbevelingen](#74-van-inzicht-naar-concrete-aanbevelingen)
&nbsp;&nbsp;&nbsp;&nbsp;[7.5 Interactiviteit en gebruikerservaring](#75-interactiviteit-en-gebruikerservaring)
[8. Technologische keuzes en onderzochte alternatieven](#8-technologische-keuzes-en-onderzochte-alternatieven)
&nbsp;&nbsp;&nbsp;&nbsp;[8.1 Frontend-keuze](#81-frontend-keuze)
&nbsp;&nbsp;&nbsp;&nbsp;[8.2 Backend-keuze](#82-backend-keuze)
[9. Conclusie](#9-conclusie)
[10. Aanbevelingen](#10-aanbevelingen)
[11. Bronnen](#11-bronnen)

---

## 1. Inleiding

Equans maakt gebruik van Atlassian Cloud Enterprise (Jira Software, Confluence, Trello), GitHub Enterprise Cloud en JFrog Artifactory, maar mist momenteel inzicht in de werkelijke licentiedekking, kosten per product en team, en het aantal inactieve accounts. Er bestaat geen geautomatiseerde API-oplossing om het daadwerkelijke licentiegebruik uit Atlassian te halen. Het management vraagt om zicht op licentiegebruik en besparingsmogelijkheden per team, product en site. In dit onderzoek is literatuur en Atlassian-documentatie geraadpleegd om antwoord te geven op de hoofdvraag: "Hoe kan Equans inzicht krijgen in het gebruik van Atlassian Cloud Enterprise licenties en de bijbehorende kosten, om besparingsmogelijkheden te identificeren en te optimaliseren?" Daarbij worden de vijf deelvragen systematisch behandeld.

| Platform                                        | Status               | Opmerkingen                                                      |
| :---------------------------------------------- | :------------------- | :--------------------------------------------------------------- |
| Atlassian Cloud (Jira, Confluence, JSM, Trello) | MVP                  | Admin API ondersteunt alle benodigde gebruikers- en licentiedata |
| GitHub Enterprise Cloud                         | Uitbreidingen na MVP | Toegang tot enterprise data, licenties, Copilot, GHAS            |
| JFrog Artifactory                               | Toekomst             | API vereist aparte credentials                                   |

## 2. Methodologie

Voor het beantwoorden van de centrale onderzoeksvraag is gekozen voor een gecombineerde onderzoeksaanpak waarin documentanalyse, literatuuronderzoek en empirische data-analyse met elkaar zijn geïntegreerd. Deze methodologische triangulatie is toegepast om zowel de technische mogelijkheden als de organisatorische implicaties van licentiebeheer binnen Equans systematisch te onderzoeken.

Allereerst is een documentanalyse uitgevoerd van de officiële ontwikkelaarsdocumentatie van Atlassian Cloud en GitHub Enterprise Cloud. Hierbij is specifiek gekeken naar de beschikbare REST API-eindpunten voor gebruikersbeheer, licentiegebruik en administratieve gegevens. De analyse richtte zich onder meer op de Atlassian Organizations REST API voor het ophalen van gebruikers- en groepsinformatie (_The Organizations REST API REST API_, z.d.-a) en op de GitHub Enterprise Licensing API voor inzicht in geconsumeerde en aangeschafte licenties (_Licensing - GitHub Enterprise Cloud Docs_, z.d.).

Door deze documentatie te bestuderen kon worden vastgesteld welke data technisch beschikbaar is voor analyse en welke beperkingen bestaan binnen de API-architectuur.

Daarnaast zijn supportartikelen en communitybronnen geraadpleegd om inzicht te verkrijgen in praktische beperkingen van de Cloud-omgevingen, zoals het ontbreken van een directe facturerings- of billing-API binnen Atlassian Cloud (Atlassian, z.d.-b). Deze aanvullende bronnen boden context bij de officiële documentatie en hielpen bij het interpreteren van de functionele grenzen van de beschikbare API's.

Vervolgens is literatuuronderzoek uitgevoerd naar best practices op het gebied van IT Financial Management, licentieoptimalisatie en dashboardontwerp. Hierbij is onder andere gebruikgemaakt van literatuur over informatievisualisatie en besluitvormingsondersteunende dashboards (Few, 2013) en publicaties over kostenallocatie en financiële transparantie binnen IT-organisaties (Blokdyk, 2020). Dit theoretisch kader diende als onderbouwing voor zowel de analyse van inefficiënt licentiegebruik als het ontwerp van het voorgestelde dashboard.

Tot slot is een empirische analyse uitgevoerd op basis van beschikbare voorbeelddata binnen de context van Equans. Hierbij is een gebruikersdataset onderzocht om patronen in activiteit, producttoewijzing en potentiële inefficiënties te identificeren. Door gebruikersstatussen, groepslidmaatschappen en activiteitsgegevens te combineren, kon worden vastgesteld in hoeverre inactieve accounts en overallocatie van licenties voorkomen. Deze praktijkanalyse fungeerde als validatie van de theoretische bevindingen en maakte het mogelijk om concrete optimalisatiescenario's te formuleren.

Door de combinatie van technische documentanalyse, literatuurstudie en praktijkgerichte data-analyse is een onderbouwd concept-dashboard ontwikkeld dat zowel technisch haalbaar als organisatorisch relevant is.

## 3. Analyse van API-datastructuren en responseformaten

Binnen dit onderzoek wordt gebruikgemaakt van verschillende Application Programming Interfaces (API's) om gegevens te verzamelen over gebruikers, licenties en productgebruik binnen de ontwikkelplatformen Atlassian Cloud en GitHub Enterprise Cloud. API's vormen een belangrijke bron van operationele data, omdat zij gestructureerde toegang bieden tot gegevens die in externe systemen zijn opgeslagen. Door middel van HTTP-requests kunnen applicaties data ophalen in een gestandaardiseerd formaat, meestal JSON (JavaScript Object Notation), dat eenvoudig verwerkt kan worden in softwaretoepassingen (Fielding & Taylor, 2002).

Het doel van deze analyse is het bestuderen en documenteren van de datastructuren en JSON-responseformaten van de gebruikte API-endpoints. Deze analyse is noodzakelijk om inzicht te krijgen in welke gegevens beschikbaar zijn, hoe deze gegevens zijn opgebouwd en op welke wijze zij kunnen worden verwerkt binnen het ontworpen datamodel. Door de structuur van de API-responses te analyseren kan worden bepaald hoe gegevens uit verschillende systemen met elkaar kunnen worden gekoppeld en opgeslagen in een centrale database.

Daarnaast vormt deze analyse de basis voor het ontwerpen van de data-mapping tussen externe systemen en het interne datamodel van het dashboard. De verkregen inzichten worden later gebruikt voor het ontwerpen van database-tabellen, het ontwikkelen van backend-services en het realiseren van dashboardvisualisaties. De analyse richt zich specifiek op twee platformen: Atlassian Cloud en GitHub Enterprise Cloud. Beide platformen bieden REST-API's waarmee informatie over gebruikers, licenties en activiteiten kan worden opgehaald (Atlassian, z.d.; GitHub, 2022).

### 3.1 Atlassian Cloud API – Gebruikers, Groepen en Licenties

**Organisatiegegevens via de Atlassian Admin API**

De Atlassian Cloud Admin API biedt verschillende endpoints waarmee informatie over organisaties, gebruikers en groepen kan worden opgehaald. Een belangrijk uitgangspunt van deze API is dat alle queries worden uitgevoerd binnen de context van een organisatie-identificatie (orgId). Deze identificatie fungeert als centrale sleutel voor het ophalen van verdere gegevens uit de Atlassian-omgeving (Atlassian, z.d.).

Het eerste endpoint dat in dit onderzoek wordt gebruikt is het endpoint voor het ophalen van organisaties. Dit endpoint retourneert basisinformatie over de organisatie, waaronder een unieke identifier en de naam van de organisatie. De API-response bevat een JSON-object waarin deze gegevens zijn opgeslagen in een arraystructuur.

**Endpoint Get Organizations:**
URL: GET `https://api.atlassian.com/admin/v1/orgs`

Voorbeeldresponse via postman:

![Afbeelding met tekst, schermopname](media/image2.png)

**Belangrijke velden:**

| Veld                      | Type   | Beschrijving                     |
| :------------------------ | :----- | :------------------------------- |
| data\[ \].id              | String | Uniek organisatie-ID (orgId)     |
| data\[ \].attributes.name | String | Naam van de organisatie          |
| links.self                | String | Link naar specifieke organisatie |

De belangrijkste waarde die uit deze response wordt verkregen is het organisatie-ID. Deze identifier wordt gebruikt in vrijwel alle vervolgqueries binnen de Atlassian Admin API. Zonder deze identifier is het niet mogelijk om gegevens over gebruikers, groepen of licenties op te vragen.

De analyse van deze API-response laat zien dat Atlassian gebruikmaakt van een gestandaardiseerde JSON-structuur waarin objecten worden georganiseerd in arrays onder de sleutel data. Binnen deze objecten bevinden zich attributen zoals id en name, die de kerninformatie over de organisatie bevatten. Dit type datastructuur komt veel voor in REST-API's en maakt het mogelijk om eenvoudig door grote datasets te itereren (Atlassian, z.d.).

**Gebruikersgegevens en producttoegang**

Naast organisatiegegevens biedt de Atlassian Admin API ook toegang tot gebruikersinformatie via het endpoint voor managed accounts. Dit endpoint levert een lijst van alle gebruikers die binnen de organisatie geregistreerd zijn. De response bevat onder andere velden zoals account_id, name, email, account_status en last_active.

**Endpoint Get Users (Managed Accounts):**
URL: GET `https://api.atlassian.com/admin/v1/orgs/{orgId}/users`

**Voorbeeldresponse van postman:**

![Afbeelding met tekst, schermopname, Lettertype](media/image3.png)

**Belangrijke velden:**

| Veld                    | Type   | Beschrijving                                       |
| :---------------------- | :----- | :------------------------------------------------- |
| account_id              | String | Unieke Atlassian user-ID                           |
| name                    | String | Volledige naam van de gebruiker                    |
| email                   | String | E-mailadres (AVG-gevoelig)                         |
| account_status          | String | Status van account (active / inactive / suspended) |
| last_active             | String | Laatste actieve tijdstip                           |
| product_access\[\].name | String | Naam van product (Jira, Confluence, Trello, enz.)  |
| product_access\[\].url  | String | URL van site of workspace                          |

Deze gegevens zijn essentieel voor het analyseren van licentiegebruik. Het veld account_status geeft bijvoorbeeld aan of een account actief, inactief of geschorst is. Het veld last_active bevat een tijdstempel van de laatste activiteit van een gebruiker binnen een Atlassian-product. Deze informatie kan worden gebruikt om inactieve accounts te identificeren en mogelijke licentie-optimalisaties te bepalen.

Een belangrijk onderdeel van de response is de array product_access. Hierin wordt per gebruiker aangegeven tot welke Atlassian-producten de gebruiker toegang heeft, bijvoorbeeld Jira Software of Confluence. Deze informatie is cruciaal voor het bepalen van licentiegebruik per product, aangezien Atlassian-licenties vaak gekoppeld zijn aan specifieke productgroepen (Atlassian, z.d.).

De analyse van dit endpoint laat zien dat Atlassian voornamelijk operationele gegevens aanbiedt over accounts en producttoegang. De API bevat echter geen directe informatie over organisatorische structuren zoals business units of cost centers. Hierdoor is aanvullende data nodig om licentiegebruik te kunnen relateren aan organisatorische eenheden.

**Groepen en licentiegroepen**

Een derde belangrijk endpoint binnen de Atlassian Admin API is het endpoint voor groepen. Groepen spelen een centrale rol binnen het licentiemodel van Atlassian, omdat producttoegang vaak wordt geregeld via groepslidmaatschap. Gebruikers die lid zijn van een bepaalde groep krijgen automatisch toegang tot het bijbehorende product.

De API-response voor groepen bevat onder andere velden zoals id, name, directoryId en externalSynced. De groepsnaam is hierbij bijzonder relevant, omdat deze vaak verwijst naar het product waarvoor de licentie geldt. Zo komen namen zoals jira-software-users of confluence-users regelmatig voor binnen Atlassian-omgevingen.

Door de groepsnamen te analyseren kan worden vastgesteld welke groepen verantwoordelijk zijn voor specifieke licenties. Dit vormt de basis voor het identificeren van licentiegebruik binnen de organisatie.

**Gebruikers per groep**

Het laatste Atlassian-endpoint dat in deze analyse wordt gebruikt is het endpoint voor het ophalen van gebruikers per groep. Met behulp van dit endpoint kan worden bepaald welke gebruikers lid zijn van een specifieke groep.

**Endpoint:** Get Users per Group
URL: GET `https://api.atlassian.com/admin/v2/orgs/{orgId}/directories/-/users?groupIds={groupId}`

Voorbeeldresponse van postman:

![Afbeelding met tekst, schermopname, Lettertype](media/image4.png)

De response bevat velden zoals accountId, name, email, status en membershipStatus. Alleen gebruikers waarvan zowel de accountstatus als de groepsstatus actief is, worden meegenomen in de berekening van licentiegebruik. Deze filtering is belangrijk omdat gebruikers met een gedeactiveerd account of een geschorst groepslidmaatschap geen actieve licentie meer gebruiken.

Door de gegevens uit deze endpoints te combineren kan uiteindelijk worden bepaald hoeveel actieve licenties per product in gebruik zijn. Dit vormt een belangrijke input voor het Operational Insights Dashboard.

### 3.2 GitHub Enterprise Cloud API – Licenties en gebruiksdata

Naast Atlassian Cloud wordt in dit onderzoek ook gebruikgemaakt van de GitHub Enterprise Cloud API. Deze API biedt endpoints waarmee licentiegebruik en gebruikersactiviteit binnen GitHub-omgevingen kan worden geanalyseerd (GitHub, 2022).

**Licentieconsumptie**

Het eerste GitHub-endpoint dat wordt gebruikt is het endpoint voor licentieconsumptie. Dit endpoint retourneert informatie over het aantal licenties dat binnen een enterprise-omgeving wordt gebruikt en hoeveel licenties er beschikbaar zijn.

**Endpoint Get License Consumption:**
URL: GET `https://api.github.com/enterprises/equans/consumed-licenses`

Voorbeeldresponse van postman:

![Afbeelding met tekst, schermopname, Lettertype](media/image5.png)

**Belangrijke velden:**

| Veld                       | Type    | Beschrijving                              |
| :------------------------- | :------ | :---------------------------------------- |
| total_seats_consumed       | Integer | Aantal gebruikte licenties                |
| total_seats_purchased      | Integer | Totaal aantal beschikbare licenties       |
| users\[\].github_com_login | String  | GitHub-loginnaam                          |
| users\[\].github_com_name  | String  | Volledige naam van gebruiker              |
| users\[\].license_type     | String  | Licentietype (Enterprise, Business, etc.) |

De response bevat onder andere de velden total_seats_consumed en total_seats_purchased. Deze waarden maken het mogelijk om de licentie-benutting te berekenen. Wanneer het aantal gebruikte licenties dicht bij het aantal beschikbare licenties ligt, kan dit wijzen op een verhoogd risico op overschrijding van de licentielimiet.

Daarnaast bevat de response een lijst van gebruikers met hun GitHub-loginnaam en het type licentie dat zij gebruiken. Deze gegevens kunnen worden gebruikt om licentiegebruik per gebruiker of per team te analyseren.

**Copilot-gebruik**

Een tweede relevant endpoint binnen de GitHub API betreft het gebruik van GitHub Copilot. Copilot is een AI-gebaseerde programmeerassistent waarvoor afzonderlijke licenties worden gebruikt. Het Copilot-endpoint retourneert informatie over het aantal beschikbare licenties en de gebruikers die deze licenties toegewezen hebben gekregen.

**Endpoint Get Copilot Seats:**
URL: GET `https://api.github.com/enterprises/equans/copilot/billing/seats`

Voorbeeldresponse van postman:

![Afbeelding met tekst, schermopname, Lettertype](media/image6.png)

**Belangrijke velden:**

| Veld                       | Type    | Beschrijving                          |
| :------------------------- | :------ | :------------------------------------ |
| total_seats                | Integer | Totaal aantal Copilot-licenties       |
| seats\[\].plan_type        | String  | Type licentie (business / enterprise) |
| seats\[\].last_activity_at | String  | Laatste activiteit van gebruiker      |
| seats\[\].assignee.login   | String  | Gebruikersnaam met Copilot-seat       |

De response bevat velden zoals total_seats, plan_type en last_activity_at. Vooral het veld last_activity_at is relevant voor het identificeren van inactieve licenties. Wanneer een gebruiker gedurende een langere periode geen activiteit vertoont, kan dit een indicatie zijn dat de licentie mogelijk overbodig is.

**GitHub Advanced Security**

Tot slot biedt de GitHub API ook informatie over het gebruik van GitHub Advanced Security (GHAS). Dit onderdeel van GitHub biedt beveiligingsfunctionaliteiten zoals code-analyse en kwetsbaarheidsscans.

Het GHAS-endpoint retourneert informatie over repositories en het aantal actieve committers dat gebruikmaakt van beveiligingsfunctionaliteiten.

**Endpoint GHAS:**
URL: GET `https://api.github.com/enterprises/equans/settings/billing/advanced-security`

Voorbeeldresponse van postman:

![Afbeelding met tekst, schermopname, Lettertype](media/image7.png)

**Belangrijke velden:**

| Veld                                                                   | Type    | Beschrijving                   |
| :--------------------------------------------------------------------- | :------ | :----------------------------- |
| repositories\[\].name                                                  | String  | Repositorynaam                 |
| repositories\[\].advanced_security_committers                          | Integer | Aantal actieve committers      |
| repositories\[\].advanced_security_committers_breakdown\[\].user_login | String  | Gebruikersnamen van committers |

Door deze gegevens te analyseren kan worden vastgesteld in welke repositories beveiligingsfunctionaliteiten actief worden gebruikt. Deze informatie kan worden gebruikt voor rapportages over beveiligingsgebruik en compliance binnen softwareontwikkelingsprojecten.

### 3.3 Beperkingen van de Atlassian API en de rol van aanvullende data

Hoewel de Atlassian Admin API waardevolle informatie biedt over gebruikers en producttoegang, blijkt uit de analyse dat de beschikbare data beperkt is tot operationele accountinformatie. Zo bevat de API geen gegevens over organisatorische structuren zoals business units, cost centers of geografische locaties.

Voor het analyseren van licentiegebruik binnen Atlassian Cloud is het noodzakelijk om gebruikersdata te koppelen aan organisatorische informatie. De Atlassian Admin API levert waardevolle gegevens over gebruikers, producttoegang en activiteit, maar bevat slechts beperkte context over de organisatorische structuur van een organisatie. Zo ontbreken bijvoorbeeld gegevens over business units, cost centers en formele HR-identificaties. Hierdoor is het niet mogelijk om licentiegebruik direct te relateren aan de interne organisatiestructuur van Equans.

**Beschikbare data**

Om deze beperking te overbruggen wordt in dit onderzoek gebruikgemaakt van aanvullende masterdata uit het Palantir-platform. Palantir fungeert binnen de organisatie als centrale bron voor organisatorische en personeelsinformatie en maakt het mogelijk om operationele data uit verschillende systemen te combineren. Door de gegevens uit Atlassian te verrijken met organisatorische informatie uit Palantir kan een completer datamodel worden opgebouwd dat geschikt is voor licentieanalyse, kostenallocatie en rapportage.

**Wat is Palantir?**

Palantir Technologies ontwikkelt softwareplatforms die organisaties ondersteunen bij het integreren, analyseren en beheren van grote en complexe datasets. De software is ontworpen om data uit verschillende systemen samen te brengen in één geïntegreerde omgeving, waardoor verbanden tussen datasets zichtbaar worden en besluitvorming kan worden ondersteund met data-gedreven inzichten (_Palantir Foundry_, z.d.).

Een belangrijk kenmerk van Palantir-platforms is dat zij data uit uiteenlopende bronnen kunnen combineren zonder dat deze bronnen eerst volledig opnieuw moeten worden ingericht. In plaats daarvan wordt een geïntegreerd datamodel gecreëerd waarin relaties tussen entiteiten --- zoals personen, organisaties en systemen --- expliciet worden vastgelegd. Hierdoor kunnen organisaties data uit verschillende operationele systemen analyseren binnen één uniforme context. Volgens Palantir maakt deze aanpak het mogelijk om operationele data, bedrijfsprocessen en organisatorische structuren met elkaar te verbinden binnen één analytisch platform (Palantir Technologies, z.d.).

Binnen veel organisaties wordt Palantir ingezet als platform voor data-integratie en besluitondersteuning. Het systeem kan bijvoorbeeld worden gebruikt om informatie uit HR-systemen, financiële systemen en operationele applicaties te combineren. Hierdoor kunnen organisaties complexe datasets analyseren en beter inzicht krijgen in processen, kostenstructuren en operationele risico's.

**Rol van Palantir binnen de organisatie**

Binnen Equans wordt Palantir gebruikt als bron voor organisatorische masterdata. In tegenstelling tot systemen zoals Atlassian, die voornamelijk operationele gegevens bevatten over gebruikersactiviteit en productgebruik, bevat Palantir informatie over de formele structuur van de organisatie.

Deze dataset bevat onder andere informatie over personen en organisaties. Voor personen worden bijvoorbeeld velden opgeslagen zoals person_id, person_first_name, person_last_name, person_email, person_local_id en org_id. Daarnaast worden aanvullende gegevens vastgelegd, zoals het land waarin een medewerker werkzaam is en de bijbehorende billing-locatie. Voor organisaties worden velden opgeslagen zoals org_id, org_name, org_country, business_unit en org_billing_location.

**Beschikbare data**

Deze informatie vormt de organisatorische context die nodig is om technische gebruikersdata te interpreteren. Door medewerkers te koppelen aan een organisatie-identificatie kan bijvoorbeeld worden bepaald binnen welke business unit een licentie wordt gebruikt. Hierdoor wordt het mogelijk om rapportages te genereren waarin licentiegebruik wordt uitgesplitst per organisatie, land of business unit.

**Dataverrijking door integratie van Atlassian en Palantir**

De integratie van Atlassian-data met Palantir-data vormt een belangrijk onderdeel van het datamodel dat in dit onderzoek wordt ontwikkeld. Atlassian levert informatie over gebruikersaccounts, producttoegang en activiteit, terwijl Palantir de organisatorische structuur van de organisatie bevat. Door beide datasets te combineren ontstaat een completer beeld van het daadwerkelijke gebruik van softwarelicenties.

De koppeling tussen beide datasets vindt plaats op basis van e-mailadressen. Atlassian identificeert gebruikers primair via een account_id, terwijl Palantir medewerkers identificeert via een person_id. Omdat deze identifiers niet direct overeenkomen, wordt een koppeling gemaakt via het e-mailadres dat in beide systemen aanwezig is. Hierdoor kan een Atlassian-account worden gekoppeld aan een specifieke persoon en organisatie binnen Palantir.

Door deze koppeling wordt het mogelijk om licentiegebruik niet alleen op individueel niveau te analyseren, maar ook op organisatieniveau. Zo kan bijvoorbeeld worden bepaald hoeveel licenties door een specifieke business unit worden gebruikt en hoeveel daarvan daadwerkelijk actief zijn. Deze informatie kan vervolgens worden gebruikt voor kostenanalyse, licentieoptimalisatie en chargeback-rapportages.

De analyse laat zien dat Atlassian en Palantir verschillende soorten informatie bevatten die elkaar aanvullen. Atlassian levert operationele data over gebruikers, producttoegang en activiteit, terwijl Palantir organisatorische masterdata bevat over personen en organisaties. Geen van beide systemen bevat afzonderlijk alle informatie die nodig is om licentiegebruik volledig te analyseren.

Door beide datasets te integreren ontstaat een geïntegreerd datamodel waarin licentiegebruik kan worden gekoppeld aan organisatorische structuren. Dit maakt het mogelijk om licenties toe te wijzen aan business units, kostenanalyses uit te voeren en rapportages te genereren voor management en financiële afdelingen. De integratie van Atlassian-data met Palantir-masterdata vormt daarmee een essentieel onderdeel van de architectuur van het Operational Insights Dashboard.

**Beschikbare gegevens binnen Palantir**

Palantir fungeert binnen Equans als bron voor organisatorische en personele masterdata. Deze dataset bevat onder andere:

- Personen (person_id, person_first_name, person_last_name, person_email, person_local_id, org_id, country, person_billing_location, gid, created_at, updated_at)
- Organisaties (org_id, org_name, org_country, org_billing_location, business_unit, person_count, created_at, updated_at)

Deze gegevens maken het mogelijk om:

1. Medewerkers te koppelen aan formele organisatiestructuren
2. Licenties toe te wijzen aan business units
3. Rapportages op land- of regioniveau te genereren

Palantir bevat daarmee de noodzakelijke organisatorische context die in Atlassian ontbreekt.

## 4. Meten van actief licentiegebruik

Een veelgebruikte maatstaf voor licentie-efficiëntie is het aantal actieve gebruikers binnen een bepaalde periode (bijvoorbeeld 30, 60 of 90 dagen). Atlassian biedt hiervoor binnen de Organizations API het endpoint "User's last active dates", waarmee per gebruiker de datum van de laatste activiteit per product kan worden opgevraagd.

Binnen Atlassian wordt "actief" gedefinieerd als het bezoeken van een productpagina gedurende een minimale tijdsduur. Een gebruiker wordt als licentiehouder beschouwd wanneer deze lid is van een productgroep én de accountstatus actief is.

Met behulp van deze gegevens kan Equans het percentage licenties berekenen dat daadwerkelijk in gebruik is geweest in een bepaalde periode. Door het aantal gebruikers met een recente last_active datum te delen door het totale aantal toegewezen licenties, ontstaat inzicht in de mate van benutting.

In de praktijk bestaat de meetmethode uit de volgende stappen:

1. Alle gebruikers ophalen via de Atlassian Admin API.
2. Voor iedere gebruiker de last_active timestamp opvragen.
3. Gebruikers filteren op basis van activiteit binnen 30/60/90 dagen.
4. Het aantal actieve accounts vergelijken met het totaal aantal toegewezen licenties.

Hiermee kan onderscheid worden gemaakt tussen daadwerkelijk gebruikte licenties en zogenoemde "unused seats".

### 4.1 Beperking van de Atlassian API: Geen historische usage trends

Een belangrijke beperking van de Atlassian Cloud API is dat deze uitsluitend een actuele last_active timestamp retourneert. De API biedt géén historische trenddata, zoals:

- Gebruik over de afgelopen 30 dagen (aggregated usage)
- Historische activiteitsontwikkeling per gebruiker
- Maandelijkse groei- of dalingstrends

Dit betekent dat het niet mogelijk is om rechtstreeks via de API historische gebruikspatronen te analyseren.

### 4.2 Technische oplossing: Zelf historische data opslaan

Om de deelvraag "Hoe kan het actieve gebruik van licenties gemeten worden (30/60/90 dagen)?" volledig te beantwoorden, is een aanvullende architectuuroplossing noodzakelijk.

De voorgestelde oplossing bestaat uit:

- Het opslaan van gebruikers- en activiteitssnapshots in een eigen PostgreSQL-database
- Het dagelijks uitvoeren van een geautomatiseerde achtergrondtaak (cron job)
- Het bewaren van historische gegevens volgens een append-only model
- Het berekenen van trends op basis van opgeslagen snapshots

Door iedere nacht een geplande synchronisatie uit te voeren, wordt de status van gebruikers en hun last_active waarde vastgelegd. Op basis van deze historische dataset kunnen vervolgens:

- Maandelijkse activiteitsgrafieken
- Trendanalyses (stijgend/dalend gebruik)
- Identificatie van structureel inactieve accounts

worden berekend.

Deze aanpak transformeert een statische API-response in een dynamisch historisch analysemodel. Hoewel Atlassian een last_active timestamp beschikbaar stelt, biedt de standaard API geen ingebouwde trendanalyse. Door het implementeren van een eigen data-opslaglaag en geplande synchronisaties kan Equans alsnog betrouwbare 30/60/90-dagen analyses uitvoeren en onderbouwde besparingsbeslissingen nemen.

## 5. Kostenanalyse per product, site en team

Voor een effectieve optimalisatie van licentiekosten is het noodzakelijk om inzicht te verkrijgen in de kostenstructuur van zowel Atlassian Cloud Enterprise als GitHub Enterprise Cloud. Beide platformen hanteren een licentie-gebaseerd abonnementsmodel waarbij kosten worden berekend op basis van toegewezen gebruikers (seats). Om de onderzoeksvraag te beantwoorden -- "Hoe kunnen de kosten per product, site en team inzichtelijk gemaakt worden?" -- is een gestructureerde analysemethode vereist die zowel technische als financiële componenten omvat.

### 5.1 Kostenmodel Atlassian Cloud Enterprise

Atlassian Cloud Enterprise hanteert het zogenoemde Maximum Quantity Billing (MQB)-principe. Dit houdt in dat binnen een factureringsperiode wordt gefactureerd op basis van het hoogste aantal toegewezen licenties, ongeacht latere deactivatie (_Atlassian Support_, z.d.-a). Hierdoor kunnen tijdelijke pieken in gebruikersaantallen leiden tot structureel hogere factuurbedragen. Aangezien Atlassian geen publieke REST API biedt voor directe facturerings- of kosteninformatie (Atlassian, z.d.-b), dient de kostenanalyse te worden gebaseerd op:

1. Het aantal toegewezen gebruikers per product (via Admin API).
2. De eenheidsprijs per licentie (afkomstig uit abonnementsoverzicht).
3. Historische snapshots om piekwaarden te detecteren.

### 5.2 Kostenmodel GitHub Enterprise Cloud

GitHub Enterprise Cloud werkt eveneens met een seat-based abonnementsmodel. Via de Enterprise Licensing API kan het aantal geconsumeerde licenties (total_seats_consumed) en aangeschafte licenties (total_seats_purchased) worden opgehaald (GitHub, z.d.-a).

Voor aanvullende producten zoals:

- GitHub Copilot Business/Enterprise
- GitHub Advanced Security (GHAS)

worden afzonderlijke API-eindpunten gebruikt om adoptie en activiteit te meten (GitHub, z.d.-b; GitHub, z.d.-c). In tegenstelling tot Atlassian biedt GitHub meer inzicht in licentiegebruik via de Enterprise Admin API. Echter, ook hier geldt dat factuurbedragen niet rechtstreeks via de API beschikbaar zijn en moeten worden berekend op basis van contracttarieven.

### 5.3 Inzicht per product

Voor zowel Atlassian als GitHub wordt inzicht per product gerealiseerd door:

- Gebruikers te identificeren via productgroepen (Atlassian) of enterprise seats (GitHub).
- Actieve en inactieve gebruikers te onderscheiden.
- Seat-toewijzing te koppelen aan tariefstructuur.

Hierdoor ontstaat een overzicht van:

- Totale kosten per product
- Percentage actieve seats
- Potentiële besparingsmogelijkheden

### 5.4 Inzicht per site

Binnen Atlassian Enterprise kunnen meerdere sites onder één abonnement vallen. Per site kan inzicht worden verkregen door:

- Groepen en gebruikers per site te aggregeren.
- Seat-distributie per site te berekenen.
- Kosten per site afzonderlijk te visualiseren.

Dit ondersteunt geografische of organisatorische vergelijking. GitHub Enterprise hanteert een enterprise-brede structuur, maar repositories en teams kunnen worden gebruikt om kostenallocatie op suborganisatieniveau te realiseren.

### 5.5 Inzicht per team (chargeback-structuur)

Om kosten per team inzichtelijk te maken, dient een koppeling te worden gerealiseerd tussen:

- Gebruikersdata (API)
- Organisatorische mapping (bijv. Business Unit of Active Directory-groepen)

Vervolgens kan kostenallocatie worden uitgevoerd conform IT Financial Management-principes, waarbij kosten worden toegerekend aan organisatorische entiteiten op basis van daadwerkelijk gebruik (Blokdyk, 2020).

Deze aanpak ondersteunt chargeback- en showback-mechanismen.

### 5.6 Architecturale vereisten voor betrouwbare kostenanalyse

Omdat beide platformen geen historische billingdata via API leveren, is een aanvullende architectuur noodzakelijk:

- Dagelijkse synchronisatie (cron jobs)
- Opslag van gebruikers- en seat-snapshots
- Append-only datamodel
- Trendberekening over tijd

Hierdoor kunnen:

- Maandelijkse kostenontwikkeling
- Seat-fluctuaties
- Structurele overallocatie

worden geanalyseerd. Deze architectuur waarborgt reproduceerbaarheid en auditability van financiële rapportages.

### 5.7 Visualisatie en besluitvorming

Kosteninformatie dient zodanig gepresenteerd te worden dat besluitvorming wordt ondersteund. Volgens Few (2013) moeten dashboards kerninformatie onmiddellijk zichtbaar maken en hiërarchisch structureren.

Voor dit onderzoek betekent dit:

- KPI-kaarten (totale kosten, besparingspotentieel)
- Staafdiagrammen (kosten per product)
- Lijngrafieken (trendontwikkeling)
- Drill-down structuur (BU → Product → Team)

Hoewel Atlassian en GitHub geen directe kosten-API's aanbieden, maken hun administratieve endpoints het mogelijk om via berekening en aggregatie betrouwbare kosteninzichten te genereren.

## 6. Patronen van inefficiënt gebruik

Het vaststellen van patronen die wijzen op inefficiënt of onbenut gebruik van Atlassian Cloud Enterprise en GitHub Enterprise Cloud licenties vormt een essentieel onderdeel van dit onderzoek. Inefficiënt licentiegebruik leidt niet alleen tot directe financiële verspilling, maar kan tevens risico's met zich meebrengen op het gebied van governance, toegangsbeheer en informatiebeveiliging. In dit hoofdstuk worden de belangrijkste indicatoren van inefficiënt gebruik systematisch geanalyseerd en onderbouwd aan de hand van beschikbare API-data en relevante literatuur.

### 6.1 Langdurig inactieve accounts

Een eerste en meest voor de hand liggende indicator betreft langdurig inactieve accounts. Binnen Atlassian Cloud kan via de Organizations REST API per gebruiker een last*active timestamp worden opgevraagd (\_The Organizations REST API REST API*, z.d.-a). Wanneer uit deze gegevens blijkt dat een gebruiker gedurende een periode van bijvoorbeeld 60 of 90 dagen geen activiteit heeft vertoond, terwijl de accountstatus nog steeds "active" is en de gebruiker lid blijft van een productgroep, kan worden geconcludeerd dat de toegewezen licentie mogelijk niet noodzakelijk is. Dit krijgt extra gewicht binnen het zogenoemde Maximum Quantity Billing-model, waarbij het hoogste aantal toegekende licenties in een factureringsperiode bepalend is voor de kosten, ongeacht feitelijk gebruik (Atlassian, z.d.-b). Dit betekent dat ook volledig inactieve accounts financieel worden doorbelast zolang zij niet expliciet worden gedeactiveerd. Een kwantitatieve analyse van last_active-gegevens in combinatie met groepslidmaatschap maakt het daarom mogelijk om concreet besparingspotentieel te identificeren.

### 6.2 Externe gebruikers met billable toegang

Een tweede patroon van inefficiëntie doet zich voor bij externe gebruikers met billable toegang. Binnen Atlassian Guard worden zowel managed accounts als externe accounts met producttoegang als factureerbaar beschouwd (Atlassian, z.d.-c). Wanneer externe gebruikers, zoals contractors of tijdelijke medewerkers, structureel toegang behouden zonder aantoonbare recente activiteit, ontstaat een situatie waarin licentiekosten niet in verhouding staan tot daadwerkelijk gebruik. Een vergelijkbare logica geldt binnen GitHub Enterprise Cloud, waar iedere gebruiker met een toegewezen seat meetelt in de totale licentiekosten (_Licensing - GitHub Enterprise Cloud Docs_, z.d.).

Door e-maildomeinen en directory-attributen te analyseren kan worden vastgesteld welke accounts extern zijn en in hoeverre hun gebruik het toekennen van een volledige seat rechtvaardigt. Het systematisch monitoren van deze categorie voorkomt dat tijdelijke of incidentele gebruikers structureel kosten genereren.

### 6.3 Governance en overmatige adminrechten

Naast gebruiksdata speelt ook governance een rol in het signaleren van inefficiëntie. Een disproportioneel aantal beheerdersaccounts kan duiden op gebrekkig rolbeheer en onvoldoende periodieke herziening van toegangsrechten. Atlassian onderscheidt verschillende administratieve rollen, waaronder Site Administrators en Product Administrators (Atlassian, z.d.-d). Hoewel deze rollen niet direct hogere kosten veroorzaken, vergroten zij de kans op onbedoelde configuratiewijzigingen en ongecontroleerde licentietoewijzing. Vanuit informatiebeveiligingsperspectief schrijft het principe van "least privilege" voor dat gebruikers uitsluitend de minimaal noodzakelijke rechten mogen bezitten (ISO/IEC, 2022). Wanneer het aantal administratieve accounts structureel hoger is dan functioneel noodzakelijk, wijst dit op een governanceprobleem dat indirect bijdraagt aan inefficiënt licentiebeheer.

### 6.4 Structurele overallocatie van licenties

Een vierde indicator betreft structurele overallocatie van licenties, ook wel overprovisioning genoemd. Dit patroon ontstaat wanneer het aantal aangeschafte seats consequent hoger ligt dan het gemiddelde actieve gebruik over een langere periode. Binnen GitHub Enterprise kan via de Licensing API het verschil tussen het aantal aangeschafte en daadwerkelijk gebruikte seats worden vastgesteld (_Licensing - GitHub Enterprise Cloud Docs_, z.d.).

Wanneer dit verschil structureel aanwezig is, bijvoorbeeld doordat piekgebruik bepalend is voor contractuele afspraken maar het gemiddelde gebruik substantieel lager ligt, ontstaat een inefficiënte kostenstructuur. Het periodiek analyseren van trenddata maakt het mogelijk om contractonderhandelingen en seat-reducties beter te onderbouwen.

### 6.5 Ontbrekende kostenallocatie per organisatorische eenheid

Ten slotte vormt het ontbreken van een expliciete koppeling tussen licentiekosten en organisatorische eenheden een belangrijke oorzaak van inefficiënt gebruik. Wanneer gebruikers niet systematisch worden gekoppeld aan Business Units of teams, ontbreekt financiële verantwoordelijkheid en transparantie. IT Financial Management-literatuur benadrukt dat kostenallocatie en chargeback-mechanismen essentieel zijn om kostenbewust gedrag binnen organisaties te stimuleren (Blokdyk, 2020). Zonder inzicht in kosten per afdeling of project blijft optimalisatie vaak een centrale IT-verantwoordelijkheid, terwijl het gebruik feitelijk bij de business ligt. Het koppelen van gebruikersdata aan organisatorische structuren vergroot daarom niet alleen het inzicht, maar versterkt ook de governance rondom licentiebeheer.

Samenvattend kan worden geconcludeerd dat inefficiënt licentiegebruik zich manifesteert in terugkerende patronen zoals langdurig inactieve maar factureerbare accounts, externe gebruikers zonder aantoonbare noodzaak, een disproportioneel aantal administratieve rechten, structurele overprovisioning van seats en het ontbreken van duidelijke kostenallocatie per organisatorische eenheid. Door deze patronen systematisch te monitoren via API-gebaseerde data-analyse en periodieke evaluatie kan Equans gerichte optimalisatiemaatregelen nemen die zowel financiële besparingen als verbeterde governance opleveren.

## 7. Presentatie in dashboard en aanbevelingen

De verzamelde inzichten verdienen een duidelijke visualisatie. Een licentie-dashboard moet kerncijfers (KPIs) Het verzamelen en analyseren van gegevens over licentiegebruik en kosten vormt slechts een tussenstap in het optimalisatieproces. De daadwerkelijke meerwaarde ontstaat pas wanneer deze inzichten op een zodanige wijze worden gepresenteerd dat zij besluitvorming ondersteunen. In het kader van dit onderzoek is daarom niet alleen gekeken naar de technische haalbaarheid van data-analyse via de Atlassian- en GitHub-API's, maar ook naar de vraag hoe deze informatie effectief kan worden gevisualiseerd in een dashboard dat zowel management als operationele teams ondersteunt. De centrale vraag luidt dan ook hoe de verzamelde informatie het beste kan worden gepresenteerd in een dashboard met duidelijke en uitvoerbare aanbevelingen.

### 7.1 Overzichtelijkheid en directe zichtbaarheid van KPI's

Een effectief dashboard kenmerkt zich door overzichtelijkheid en doelgerichtheid. Volgens Few (2013) moet een dashboard in één oogopslag inzicht geven in de meest relevante prestatie-indicatoren. Dit uitgangspunt impliceert dat kerncijfers direct zichtbaar moeten zijn, zonder dat de gebruiker eerst door meerdere schermen hoeft te navigeren. In het kader van het Operational Insights Dashboard betekent dit dat indicatoren zoals het totaal aantal licenties per product, het percentage actieve versus inactieve gebruikers, de maandelijkse kosten en het geschatte besparingspotentieel prominent worden weergegeven. Door deze informatie bovenaan het dashboard te positioneren, wordt voldaan aan het principe dat cruciale informatie onmiddellijk beschikbaar moet zijn voor besluitvormers.

### 7.2 Hiërarchische ordening van informatie

Naast directe zichtbaarheid is ook de hiërarchische ordening van informatie van belang. Tufte (2001) benadrukt dat kwantitatieve informatie zodanig moet worden gepresenteerd dat de structuur van de gegevens intuïtief begrijpelijk is. Dit betekent dat het dashboard moet zijn opgebouwd van algemeen naar specifiek. Strategische informatie, zoals totale kosten en globale benuttingspercentages, krijgt een centrale positie (Tufte, 2001). Daaronder volgen tactische inzichten, bijvoorbeeld kostenverdelingen per product of Business Unit en trendanalyses over meerdere maanden. Op het meest gedetailleerde niveau kunnen specifieke gebruikersgegevens of lijsten met inactieve accounts worden geraadpleegd. Deze gelaagde structuur zorgt ervoor dat verschillende typen gebruikers --- van management tot IT-beheerders --- dezelfde applicatie kunnen gebruiken, maar op een ander detailniveau.

### 7.3 Keuze van visualisatietypen

De keuze van visualisatietypen speelt eveneens een belangrijke rol in de begrijpelijkheid van het dashboard. Datavisualisatie moet aansluiten bij het type data dat wordt weergegeven. Discrete categorieën, zoals productnamen of teams, worden het best weergegeven met staafdiagrammen, omdat deze directe vergelijking mogelijk maken zonder continuïteit te suggereren. Trends over tijd lenen zich juist voor lijngrafieken, omdat deze ontwikkeling en richting zichtbaar maken (Few, 2013). Door deze principes consequent toe te passen, wordt het risico op misinterpretatie beperkt en ontstaat een consistente visuele taal binnen het dashboard.

### 7.4 Van inzicht naar concrete aanbevelingen

Een belangrijk aspect dat in dit onderzoek nadrukkelijk is meegenomen, is de vertaling van analyse naar concrete aanbevelingen. IT Financial Management benadrukt dat transparantie in kosten slechts effectief is wanneer deze wordt gekoppeld aan verantwoordelijkheid en actie (Blokdyk, 2020). Een dashboard dat uitsluitend cijfers toont, zonder interpretatie of aanbeveling, draagt onvoldoende bij aan optimalisatie. Daarom worden in het voorgestelde ontwerp aanbevelingen expliciet gekoppeld aan de gevisualiseerde data. Wanneer bijvoorbeeld een significant aantal gebruikers gedurende negentig dagen geen activiteit vertoont, wordt dit niet alleen getoond in een grafiek, maar tevens vertaald naar een concrete suggestie, zoals het deactiveren van inactieve accounts en het vermelden van de geschatte kostenbesparing. Op deze wijze wordt het dashboard niet slechts een rapportagetool, maar een instrument voor besluitvorming.

### 7.5 Interactiviteit en gebruikerservaring

Interactiviteit vormt tot slot een essentieel onderdeel van de presentatie. Door gebruik te maken van een Single Page Application-architectuur op basis van React kunnen filters en drill-downfunctionaliteiten worden toegepast zonder pagina-herlaadmomenten. Hierdoor kan de gebruiker eenvoudig navigeren van Business Unit naar productniveau en uiteindelijk naar individuele gebruikers. Deze vorm van interactie stimuleert actieve analyse en maakt het mogelijk om vragen direct binnen het dashboard te beantwoorden. De combinatie van hiërarchische structuur, juiste visualisatiekeuzes en expliciete aanbevelingen resulteert in een dashboard dat niet alleen inzicht biedt, maar ook richting geeft aan optimalisatiebeslissingen.

Geconcludeerd kan worden dat de informatie over licentiegebruik en kosten het beste wordt gepresenteerd in een hiërarchisch opgebouwd, interactief dashboard waarin kernindicatoren direct zichtbaar zijn, trends visueel worden ondersteund en aanbevelingen expliciet worden gekoppeld aan geconstateerde inefficiënties. Door deze aanpak wordt het dashboard een strategisch hulpmiddel dat financiële transparantie combineert met concrete optimalisatieacties.

## 8. Technologische keuzes en onderzochte alternatieven

Bij de ontwikkeling van het Operational Insights Dashboard is bewust gekozen voor een architectuur bestaande uit een Rust-backend en een frontend op basis van React met TypeScript. Deze keuze is niet uitsluitend gebaseerd op persoonlijke voorkeur, maar op een systematische vergelijking van alternatieven, waarbij performance, onderhoudbaarheid, schaalbaarheid en veiligheid als beoordelingscriteria zijn gehanteerd.

### 8.1 Frontend-keuze

**Onderzoek naar Angular**

Angular is een volledig frontend-framework dat veel functionaliteit "out-of-the-box" biedt, waaronder dependency injection, routing, state management en een sterke projectstructuur (Google Books, z.d.). In enterprise-omgevingen wordt Angular vaak toegepast vanwege deze gestructureerde aanpak.

Tijdens de analysefase is Angular overwogen vanwege:

- De uitgebreide ingebouwde functionaliteit
- Enterprise-geschiktheid
- TypeScript-integratie

Desondanks is Angular in dit project niet gekozen. De belangrijkste overweging hierbij was de complexiteit van het framework. Angular vereist het beheersen van meerdere concepten tegelijk (modules, services, decorators, dependency injection), wat leidt tot een steile leercurve en relatief veel boilerplate-code. Voor een dashboardtoepassing zonder complexe formulieren of uitgebreide businesslogica werd dit als disproportioneel beschouwd.

Daarnaast is Angular zwaarder in bundlegrootte vergeleken met React, wat mogelijk impact heeft op performance bij data-intensieve visualisaties (_Angular_, z.d.).

Conclusie: Angular biedt sterke structuur, maar de complexiteit en overhead zijn niet in verhouding tot de scope van dit afstudeerproject.

**Onderzoek naar Vanilla JavaScript**

Een alternatief was het gebruik van pure JavaScript zonder framework. Dit biedt maximale controle over de implementatie en minimale afhankelijkheid van externe libraries.

Hoewel dit aantrekkelijk lijkt vanuit eenvoud, kent deze aanpak belangrijke beperkingen:

- Geen gestandaardiseerde componentstructuur
- Handmatig DOM-beheer
- Geen ingebouwde state-management patronen
- Geen statische typecontrole

Bij groei van de applicatie zou dit leiden tot verminderde onderhoudbaarheid en verhoogde kans op runtime-fouten. Onderzoek naar software-architectuur toont aan dat component-gebaseerde structuren bijdragen aan schaalbaarheid en herbruikbaarheid (Bass, Clements, & Kazman, 2022).

Conclusie: Vanilla JavaScript is onvoldoende schaalbaar voor een groeiend dashboard met meerdere views, filters en drill-downfunctionaliteit.

**Onderzoek naar server-rendered UI (bijv. Razor of Django Templates)**

Server-side rendering (SSR) is onderzocht vanwege de eenvoud bij traditionele webapplicaties. Frameworks zoals ASP.NET Razor of Django Templates bieden snelle implementatie en duidelijke scheiding tussen data en presentatie.

Voor een interactieve dashboardomgeving met veel dynamische filtering, grafieken en client-side interactie bleek SSR echter minder geschikt. Iedere interactie zou een server request vereisen, wat leidt tot:

- Extra latency
- Minder vloeiende gebruikerservaring
- Complexere state-handling

Volgens moderne webarchitectuurprincipes zijn Single Page Applications (SPA's) beter geschikt voor data-analyseplatformen vanwege hun interactieve karakter (Tilkov & Vinoski, 2010).

Conclusie: Voor een data-intensief dashboard is een client-side SPA-architectuur geschikter dan server-rendering.

**Keuze voor React met TypeScript**

React is gekozen vanwege de componentgebaseerde architectuur en het brede ecosysteem (React, z.d.). Het framework is relatief lichtgewicht en biedt flexibiliteit in architectuurkeuzes.

TypeScript is toegevoegd om statische typecontrole te waarborgen. Statische typisering vermindert runtime-fouten en verhoogt onderhoudbaarheid in grotere applicaties (Microsoft, z.d.). In een dashboard met complexe datamodellen en API-responses is typeveiligheid essentieel.

Belangrijke voordelen van React + TypeScript:

- Component-gebaseerde opbouw
- Hoge herbruikbaarheid
- Sterke community en ecosystem (bijv. Recharts, MUI)
- Compile-time typecontrole
- Geschikt voor schaalbare SPA-architecturen

### 8.2 Backend-keuze

**Onderzoek naar Python (Flask/FastAPI)**

Python is onderzocht vanwege de snelle ontwikkelsnelheid en brede ondersteuning voor API-integraties. Frameworks zoals FastAPI bieden relatief eenvoudige implementatie van REST-API's.

Hoewel Python geschikt is voor prototyping, kent het beperkingen:

- Dynamische typing (fouten pas zichtbaar bij runtime)
- Global Interpreter Lock (GIL) bij CPU-intensieve processen
- Minder voorspelbare performance bij hoge concurrency

Voor een systeem dat meerdere externe API's parallel moet aanroepen (Atlassian, GitHub, JFrog), is concurrency van groot belang. Volgens performance-onderzoek presteert Python doorgaans minder efficiënt bij hoge parallelle workloads dan gecompileerde talen (McKinney, 2017).

Conclusie: Python is geschikt voor snelle prototypes, maar minder optimaal voor een robuuste, performante productie-architectuur.

**Onderzoek naar .NET**

.NET (ASP.NET Core) is eveneens onderzocht. Het platform biedt sterke enterprise-integratie, goede performance en uitgebreide tooling (Microsoft, z.d.).

Hoewel .NET een solide keuze zou zijn geweest, is uiteindelijk gekozen voor Rust vanwege:

- Hogere controle over geheugenbeheer
- Afwezigheid van garbage collection
- Compile-time concurrency checks
- Zeer lage latency

**Keuze voor Rust**

Rust biedt memory safety zonder garbage collector en voorkomt data races via het ownership-model (Klabnik & Nichols, 2023). Dit maakt Rust bijzonder geschikt voor:

- Concurrerende API-calls
- Rate-limit handling
- Hoge performance workloads
- Veilig verwerken van externe data

In dit project is performance cruciaal, aangezien:

- Grote datasets worden opgehaald via meerdere API's
- Rate limits moeten worden gerespecteerd
- Data veilig moet worden opgeslagen

Rust combineert lage latency met hoge betrouwbaarheid en compile-time foutdetectie.

Op basis van de uitgevoerde analyse kan worden geconcludeerd dat React met TypeScript en Rust de meest geschikte combinatie vormen voor dit project. Alternatieven zoals Angular, Vanilla JavaScript, server-rendered UI, Python en .NET zijn onderzocht, maar bleken minder passend binnen de context van een data-intensief, schaalbaar dashboard met hoge performance-eisen.

De uiteindelijke keuze is daarmee niet uitsluitend technologisch gemotiveerd, maar gebaseerd op een afweging tussen complexiteit, onderhoudbaarheid, veiligheid en performance.

## 9. Conclusie

Dit onderzoek had als doel te bepalen hoe Equans inzicht kan verkrijgen in het gebruik van Atlassian Cloud Enterprise-licenties en de bijbehorende kosten, teneinde besparingsmogelijkheden te identificeren en te optimaliseren. Op basis van de uitgevoerde analyse kan worden geconcludeerd dat de beschikbare Atlassian Cloud Admin API's, ondanks het ontbreken van een directe kosten- of facturerings-API, voldoende gegevens leveren om een betrouwbaar beeld van licentiegebruik te construeren.

Via endpoints voor organisaties, groepen, gebruikers en last-active data kunnen nauwkeurig worden vastgesteld welke accounts daadwerkelijk actief zijn en welke licenties structureel onbenut blijven. Hoewel Atlassian geen expliciete kosteninformatie via de API beschikbaar stelt, maakt het Maximum Quantity Billing-model het mogelijk om op basis van seat-aantallen en contracttarieven een valide kostenberekening uit te voeren (The Organizations REST API REST API, z.d.-a).

Door deze technische gegevens te combineren met organisatorische mapping, bijvoorbeeld op Business Unit-niveau, ontstaat inzicht in kostenallocatie en benuttingsgraad per afdeling. De analyse toont aan dat inefficiënt licentiegebruik zich met name manifesteert in langdurig inactieve accounts, externe gebruikers met billable toegang zonder recente activiteit en structurele overallocatie van seats. Deze patronen kunnen systematisch worden geïdentificeerd door periodieke dataverzameling en analyse. Daarmee wordt het mogelijk om niet alleen reactief, maar ook proactief kostenoptimalisatie toe te passen.

Daarnaast blijkt uit het onderzoek dat een goed ontworpen dashboard essentieel is voor effectieve besluitvorming. Door kernindicatoren hiërarchisch en visueel consistent te presenteren, wordt financiële transparantie bevorderd en worden optimalisatiemogelijkheden direct zichtbaar (Few, 2013). Het dashboard fungeert daarmee niet slechts als rapportage-instrument, maar als strategisch hulpmiddel binnen IT Financial Management (Blokdyk, 2020).

Ten slotte kan worden geconcludeerd dat de gekozen technologische architectuur --- een Rust-backend in combinatie met een React- en TypeScript-frontend --- passend is voor de aard van het probleem. De backend vereist efficiënte verwerking van externe API-calls en veilige gelijktijdige dataverwerking, terwijl de frontend gebaat is bij een schaalbare, onderhoudbare en typeveilige implementatie. De onderzochte alternatieven boden waardevolle inzichten, maar bleken minder geschikt binnen de context van performance-eisen, schaalbaarheid en onderhoudbaarheid.

Samenvattend kan worden gesteld dat Equans met relatief beperkte technische ingrepen aanzienlijke verbeteringen kan realiseren in kosteninzicht, licentiebeheer en governance rondom Atlassian- en GitHub-licenties.

## 10. Aanbevelingen

Op basis van de bevindingen van dit onderzoek worden de volgende aanbevelingen gedaan.

- In de eerste plaats wordt geadviseerd het voorgestelde dashboard daadwerkelijk te implementeren en structureel te integreren binnen de IT-beheerprocessen van Equans. De effectiviteit van het systeem hangt niet uitsluitend af van de technische realisatie, maar ook van periodieke monitoring en organisatorische inbedding. Een dagelijkse of wekelijkse geautomatiseerde synchronisatie van gebruikers- en licentiegegevens via de Atlassian- en GitHub-API's is noodzakelijk om actuele en betrouwbare inzichten te behouden.

- Daarnaast wordt aanbevolen om periodieke licentie-audits te formaliseren. Door bijvoorbeeld elk kwartaal een evaluatie uit te voeren van inactieve accounts en externe gebruikers, kan structurele kostenbesparing worden gerealiseerd. Deze audits dienen te worden gekoppeld aan duidelijke verantwoordelijkheden per Business Unit, zodat kostenbewust gedrag wordt gestimuleerd.

- Verder wordt geadviseerd om kostenallocatie expliciet te koppelen aan organisatorische entiteiten. Wanneer kosten per Business Unit transparant zichtbaar zijn, ontstaat een natuurlijke prikkel tot optimalisatie en efficiënter gebruik. Dit sluit aan bij best practices binnen IT Financial Management, waarbij transparantie en toewijsbaarheid van kosten centraal staan (Blokdyk, 2020).

- Tot slot wordt aanbevolen om het dashboard in een latere fase uit te breiden met aanvullende functionaliteiten, zoals trendanalyses over langere perioden, automatische waarschuwingen bij overallocatie en integratie met bredere Enterprise-dataplatformen zoals Palantir. Hiermee kan het systeem zich ontwikkelen van een kosteninzichttool naar een integraal governance-instrument.

Door deze aanbevelingen te implementeren kan Equans niet alleen besparen op licentiekosten, maar tevens de beheersbaarheid, transparantie en veiligheid van het applicatielandschap verbeteren.

## 11. Bronnen

- _Angular_. (z.d.-a). https://angular.io/docs

- Atlassian. (2020, 16 december). _Manage your bill for Atlassian Guard Standard_. Atlassian Support. https://support.atlassian.com/subscriptions-and-billing/docs/manage-your-bill-for-atlassian-guard-standard/

- Atlassian. (2024, 23 oktober). _How maximum quantity billing works_. Atlassian Support. https://support.atlassian.com/subscriptions-and-billing/docs/how-maximum-quantity-billing-works/

- _Atlassian Support_. (z.d.-a). Atlassian Support. https://support.atlassian.com/

- _Atlassian Support_. (z.d.-b). Atlassian Support. https://support.atlassian.com/search-results/?searchTerm=Atlassian+Guard+billing+and+external+users

- Blokdyk, G. (2020). IT Cost Transparency A Complete Guide - 2020 edition. 5starcooks.

- Few, S. (2013). Information Dashboard Design: Displaying Data for At-a-glance Monitoring.

- Fielding, R. T., & Taylor, R. N. (2002). Principled design of the modern Web architecture. ACM Transactions On Internet Technology, 2(2), 115--150. https://doi.org/10.1145/514183.514185

- _Google Books_. (z.d.). https://www.google.nl/books/edition/ISO_IEC_27001_2022_An_introduction_to_in/LtWbEAAAQBAJ?hl=fy&gbpv=1&dq=iso/iec.+(2022).+iso/iec+27001:2022+information+security+management+systems+%E2%80%94+requirements.++Book&printsec=frontcover

- Klabnik, S., & Nichols, C. (2023). _The Rust Programming Language, 2nd Edition_. No Starch Press.

- _Licensing - GitHub Enterprise Cloud Docs_. (z.d.). GitHub Docs. https://docs.github.com/en/enterprise-cloud@latest/rest/enterprise-admin/licensing

- _Licensing - GitHub Enterprise Cloud Docs_. (2022, 28 november). GitHub Docs. https://docs.github.com/en/enterprise-cloud@latest/rest/enterprise-admin/licensing?apiVersion=2022-11-28

- McKinney, W. (2017). _Python for Data Analysis: Data Wrangling with Pandas, Numpy, and Ipython_. O'Reilly Media.

- _Palantir Foundry_. (z.d.). Palantir. https://www.palantir.com/platforms/foundry/

- _React_. (z.d.). https://react.dev/

- _REST API endpoints for Copilot user management - GitHub Enterprise Cloud Docs_. (z.d.). GitHub Docs. https://docs.github.com/en/enterprise-cloud@latest/rest/copilot/copilot-user-management

- _REST API endpoints for Copilot user management - GitHub Enterprise Cloud Docs_. (2022, 28 november). GitHub Docs. https://docs.github.com/en/enterprise-cloud@latest/rest/copilot/copilot-user-management?apiVersion=2022-11-28

- _Subscriptions and billing | Atlassian Support_. (z.d.). Atlassian Support. https://support.atlassian.com/subscriptions-and-billing/resources/

- _The organizations REST API REST API_. (z.d.-a). https://developer.atlassian.com/cloud/admin/organization/rest/api-group-users/#api-group-users

- _The organizations REST API REST API_. (z.d.-b). https://developer.atlassian.com/cloud/admin/organization/rest/

- _The starting point for learning TypeScript_. (z.d.). https://www.typescriptlang.org/docs/

- Tufte, R. (2001). _The Visual Display of Quantitative Information_ (SECOND EDITION). Graphics Press LLC. https://lmscontent.embanet.com/USC/CMGT587/Tufte%20Ch2%20and%205.pdf

- Wadepickett. (z.d.). _ASP.NET documentation_. Microsoft Learn. https://learn.microsoft.com/aspnet/core/
