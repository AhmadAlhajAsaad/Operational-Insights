# Onderzoeksverslag

## Operational Insights Dashboard

### Proof of Concept voor licentie- en gebruiksdata visualisatie

![Afbeelding met tekst, schermopname, Lettertype, nummer](media/image1.png)

**Project:** Operational Insights Dashboard
**Student:** Ahmad Alhaj Asaad
**Opleiding:** HBO-ICT
**Bedrijf:** Equans
**Datum:** November 2025
**Versie:** 1.0

---

## Inhoudsopgave

[1. Inleiding](#1-inleiding)
[2. Methodologie](#2-methodologie)
[3. Analyse van API-datastructuren en responseformaten](#3-analyse-van-api-datastructuren-en-responseformaten)
&nbsp;&nbsp;&nbsp;&nbsp;[3.1 Atlassian Cloud API – Gebruikers, groepen en licenties](#31-atlassian-cloud-api-gebruikers-groepen-en-licenties)
&nbsp;&nbsp;&nbsp;&nbsp;[3.2 GitHub Enterprise Cloud API – Licenties en gebruiksdata](#32-github-enterprise-cloud-api-licenties-en-gebruiksdata)
&nbsp;&nbsp;&nbsp;&nbsp;[3.3 Beperkingen van de Atlassian API en de rol van aanvullende data](#33-beperkingen-van-de-atlassian-api-en-de-rol-van-aanvullende-data)
[4. Meten van actief licentiegebruik](#4-meten-van-actief-licentiegebruik)
&nbsp;&nbsp;&nbsp;&nbsp;[4.1 Beperking van de Atlassian API: geen historische usage trends](#41-beperking-van-de-atlassian-api-geen-historische-usage-trends)
&nbsp;&nbsp;&nbsp;&nbsp;[4.2 Technische oplossing: zelf historische data opslaan](#42-technische-oplossing-zelf-historische-data-opslaan)
[5. Kostenanalyse per product, site en team](#5-kostenanalyse-per-product-site-en-team)
&nbsp;&nbsp;&nbsp;&nbsp;[5.1 Kostenmodel Atlassian Cloud Enterprise](#51-kostenmodel-atlassian-cloud-enterprise)
&nbsp;&nbsp;&nbsp;&nbsp;[5.2 Kostenmodel GitHub Enterprise Cloud](#52-kostenmodel-github-enterprise-cloud)
&nbsp;&nbsp;&nbsp;&nbsp;[5.3 Inzicht per product](#53-inzicht-per-product)
&nbsp;&nbsp;&nbsp;&nbsp;[5.4 Inzicht per site](#54-inzicht-per-site)
&nbsp;&nbsp;&nbsp;&nbsp;[5.5 Inzicht per team (chargeback-structuur)](#55-inzicht-per-team-chargeback-structuur)
&nbsp;&nbsp;&nbsp;&nbsp;[5.6 Architectuurvereisten voor betrouwbare kostenanalyse](#56-architectuurvereisten-voor-betrouwbare-kostenanalyse)
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

Equans maakt dagelijks gebruik van meerdere ontwikkelplatformen: Atlassian Cloud Enterprise (Jira Software, Confluence, Trello), GitHub Enterprise Cloud en JFrog Artifactory. Het probleem is dat niemand binnen de organisatie precies weet hoeveel licenties er werkelijk gebruikt worden, wat dat per team kost en hoeveel accounts er eigenlijk inactief zijn. Een geautomatiseerde oplossing om dit soort inzichten uit de API's te halen ontbreekt volledig. Vanuit het management kwam de vraag om hier grip op te krijgen, met name op licentiegebruik en besparingsmogelijkheden per team, product en site.

Binnen dit project is daarom onderzocht hoe Equans dat inzicht wél kan krijgen. De hoofdvraag luidt: "Hoe kan Equans inzicht krijgen in het gebruik van Atlassian Cloud Enterprise licenties en de bijbehorende kosten, om besparingsmogelijkheden te identificeren en te optimaliseren?" Om die vraag te beantwoorden zijn vijf deelvragen opgesteld die in dit verslag een voor een behandeld worden.

| Platform                                        | Status               | Opmerkingen                                                      |
| :---------------------------------------------- | :------------------- | :--------------------------------------------------------------- |
| Atlassian Cloud (Jira, Confluence, JSM, Trello) | MVP                  | Admin API ondersteunt alle benodigde gebruikers- en licentiedata |
| GitHub Enterprise Cloud                         | Uitbreidingen na MVP | Toegang tot enterprise data, licenties, Copilot, GHAS            |
| JFrog Artifactory                               | Toekomst             | API vereist aparte credentials                                   |

## 2. Methodologie

Om de hoofdvraag goed te kunnen beantwoorden, is een gecombineerde onderzoeksaanpak gebruikt. Hierbij zijn documentanalyse, literatuuronderzoek en empirische data-analyse gecombineerd. De reden voor deze aanpak is dat zowel de technische mogelijkheden als de organisatorische gevolgen van licentiebeheer in beeld moesten komen. Eén methode was niet genoeg om beide kanten goed te belichten.

Als eerste is de officiële ontwikkelaarsdocumentatie van Atlassian Cloud en GitHub Enterprise Cloud doorgenomen. Hierbij is specifiek gekeken naar de beschikbare REST API-endpoints voor gebruikersbeheer, licentiegebruik en administratieve gegevens. De Atlassian Organizations REST API is geanalyseerd voor het ophalen van gebruikers- en groepsinformatie (_The Organizations REST API REST API_, z.d.-a), en de GitHub Enterprise Licensing API voor inzicht in geconsumeerde en aangeschafte licenties (_Licensing - GitHub Enterprise Cloud Docs_, z.d.). Het doel was om vast te stellen welke data technisch beschikbaar is en waar de grenzen van de API's liggen.

Daarnaast zijn supportartikelen en communitybronnen geraadpleegd. In de praktijk bleek namelijk dat de officiële documentatie niet altijd het volledige plaatje geeft. Zo ontbreekt bijvoorbeeld een directe facturerings- of billing-API binnen Atlassian Cloud (Atlassian, z.d.-b). Deze aanvullende bronnen hielpen om de functionele grenzen van de API's beter te begrijpen.

Naast de technische analyse is ook literatuuronderzoek gedaan naar best practices op het gebied van IT Financial Management, licentieoptimalisatie en dashboardontwerp. Hierbij is onder andere gebruikgemaakt van Few (2013) over informatievisualisatie en Blokdyk (2020) over kostenallocatie binnen IT-organisaties. Dit theoretisch kader vormde de onderbouwing voor zowel de analyse van inefficiënt licentiegebruik als het ontwerp van het dashboard.

Tot slot is een empirische analyse uitgevoerd op basis van beschikbare voorbeelddata binnen de context van Equans. Hierbij zijn gebruikersstatussen, groepslidmaatschappen en activiteitsgegevens gecombineerd om patronen in activiteit en producttoewijzing te vinden. Tijdens het analyseren viel op dat een aanzienlijk deel van de accounts inactief bleek te zijn, terwijl de licenties nog steeds liepen. Deze praktijkanalyse diende als validatie van de theoretische bevindingen en maakte het mogelijk om concrete optimalisatiescenario's uit te werken.

De combinatie van technische documentanalyse, literatuurstudie en praktijkgerichte data-analyse heeft uiteindelijk geleid tot een onderbouwd concept-dashboard dat zowel technisch haalbaar als organisatorisch relevant is.

## 3. Analyse van API-datastructuren en responseformaten

Binnen dit project is gebruikgemaakt van verschillende API's om gegevens te verzamelen over gebruikers, licenties en productgebruik. API's zijn in dit geval de belangrijkste bron van operationele data, omdat ze gestructureerde toegang bieden tot informatie die in externe systemen opgeslagen is. Via HTTP-requests haalt de applicatie data op in JSON-formaat (JavaScript Object Notation), dat eenvoudig verwerkt kan worden in softwaretoepassingen (Fielding & Taylor, 2002).

Het doel van deze analyse was om de datastructuren en JSON-responseformaten van de gebruikte API-endpoints te bestuderen en documenteren. Dit was nodig om te begrijpen welke gegevens beschikbaar zijn, hoe die gegevens zijn opgebouwd en hoe ze verwerkt kunnen worden binnen het datamodel. Tijdens het uitpluizen van de API-responses bleek dat niet alle endpoints dezelfde structuur hanteren, wat later invloed had op hoe de data-mapping is opgezet.

Daarnaast vormde deze analyse de basis voor het ontwerp van de koppeling tussen externe systemen en het interne datamodel van het dashboard. De inzichten zijn later gebruikt voor het ontwerpen van database-tabellen, het bouwen van backend-services en het realiseren van dashboardvisualisaties. De analyse richt zich op twee platformen: Atlassian Cloud en GitHub Enterprise Cloud. Beide bieden REST-API's waarmee informatie over gebruikers, licenties en activiteiten opgehaald kan worden (Atlassian, z.d.; GitHub, 2022).

### 3.1 Atlassian Cloud API – Gebruikers, groepen en licenties

**Organisatiegegevens via de Atlassian Admin API**

De Atlassian Cloud Admin API biedt meerdere endpoints waarmee informatie over organisaties, gebruikers en groepen opgehaald kan worden. Een belangrijk uitgangspunt hierbij is dat alle queries draaien binnen de context van een organisatie-ID (`orgId`). Zonder dit ID lukt het niet om verdere gegevens op te halen. Dit bleek in de praktijk het eerste wat je nodig hebt om überhaupt iets nuttigs uit de API te kunnen trekken (Atlassian, z.d.).

Het eerste endpoint dat is gebruikt, haalt basisinformatie over de organisatie op, waaronder een unieke identifier en de organisatienaam. De response bevat een JSON-object met deze gegevens in een arraystructuur.

**Endpoint Get Organizations:**
URL: GET `https://api.atlassian.com/admin/v1/orgs`

Voorbeeldresponse via Postman:

![Afbeelding met tekst, schermopname](media/image2.png)

**Belangrijke velden:**

| Veld                      | Type   | Beschrijving                     |
| :------------------------ | :----- | :------------------------------- |
| data\[ \].id              | String | Uniek organisatie-ID (orgId)     |
| data\[ \].attributes.name | String | Naam van de organisatie          |
| links.self                | String | Link naar specifieke organisatie |

De belangrijkste waarde uit deze response is het organisatie-ID. Dat ID wordt in vrijwel alle vervolgqueries binnen de Atlassian Admin API gebruikt. Zonder dit ID lukt het simpelweg niet om gegevens over gebruikers, groepen of licenties op te vragen.

Wat opviel bij het analyseren van de API-response is dat Atlassian een vrij standaard JSON-structuur gebruikt. Objecten zitten in arrays onder de sleutel `data`, met daarin attributen als `id` en `name`. Dit type datastructuur zie je veel terug bij REST-API's en maakt het relatief eenvoudig om door grote datasets te itereren (Atlassian, z.d.).

**Gebruikersgegevens en producttoegang**

Naast organisatiegegevens biedt de API ook toegang tot gebruikersinformatie via het endpoint voor managed accounts. Dit levert een lijst op van alle geregistreerde gebruikers binnen de organisatie. De response bevat velden als `account_id`, `name`, `email`, `account_status` en `last_active`.

**Endpoint Get Users (Managed Accounts):**
URL: GET `https://api.atlassian.com/admin/v1/orgs/{orgId}/users`

**Voorbeeldresponse van Postman:**

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

Deze gegevens zijn onmisbaar voor het analyseren van licentiegebruik. Het veld `account_status` geeft direct aan of een account actief, inactief of geschorst is. Het veld `last_active` bevat een tijdstempel van de laatste activiteit binnen een Atlassian-product. Met die informatie kun je inactieve accounts opsporen en bepalen waar licentie-optimalisatie mogelijk is.

Een interessant onderdeel van de response is de array `product_access`. Hierin staat per gebruiker tot welke Atlassian-producten diegene toegang heeft, bijvoorbeeld Jira Software of Confluence. Omdat Atlassian-licenties vaak aan specifieke productgroepen gekoppeld zijn, is dit veld essentieel voor het bepalen van licentiegebruik per product (Atlassian, z.d.).

Hierbij bleek dat de API wel operationele gegevens levert over accounts en producttoegang, maar geen directe informatie bevat over organisatorische structuren zoals business units of cost centers. Dat betekent dat er aanvullende data nodig is om licentiegebruik te koppelen aan organisatorische eenheden. Hier wordt later verder op ingegaan.

**Groepen en licentiegroepen**

Een derde endpoint binnen de Atlassian Admin API gaat over groepen. Groepen spelen een centrale rol in het Atlassian licentiemodel, want producttoegang wordt vaak geregeld via groepslidmaatschap. Als een gebruiker lid is van een bepaalde groep, krijgt die automatisch toegang tot het bijbehorende product.

De API-response voor groepen bevat velden als `id`, `name`, `directoryId` en `externalSynced`. Met name de groepsnaam is hier relevant, want die verwijst vaak naar het product waarvoor de licentie geldt. Zo kom je namen tegen als `jira-software-users` of `confluence-users`. Door de groepsnamen te analyseren kun je vaststellen welke groepen verantwoordelijk zijn voor specifieke licenties. Dit vormt de basis voor het in kaart brengen van licentiegebruik.

**Gebruikers per groep**

Het laatste Atlassian-endpoint in deze analyse haalt gebruikers per groep op. Hiermee kun je bepalen welke gebruikers lid zijn van een specifieke groep.

**Endpoint:** Get Users per Group
URL: GET `https://api.atlassian.com/admin/v2/orgs/{orgId}/directories/-/users?groupIds={groupId}`

Voorbeeldresponse van Postman:

![Afbeelding met tekst, schermopname, Lettertype](media/image4.png)

De response bevat velden als `accountId`, `name`, `email`, `status` en `membershipStatus`. Alleen gebruikers waarvan zowel de accountstatus als de groepsstatus actief is, tellen mee in de berekening van licentiegebruik. Deze filtering is bewust toegepast, want gebruikers met een gedeactiveerd account of geschorst groepslidmaatschap verbruiken geen actieve licentie meer.

Door de gegevens uit al deze endpoints te combineren was het uiteindelijk mogelijk om te bepalen hoeveel actieve licenties per product in gebruik zijn. Dit vormt een belangrijke input voor het Operational Insights Dashboard.

### 3.2 GitHub Enterprise Cloud API – Licenties en gebruiksdata

Naast Atlassian Cloud is ook de GitHub Enterprise Cloud API geanalyseerd. Deze API biedt endpoints waarmee licentiegebruik en gebruikersactiviteit binnen GitHub-omgevingen onderzocht kan worden (GitHub, 2022).

**Licentieconsumptie**

Het eerste endpoint gaat over licentieconsumptie. Het retourneert informatie over het aantal licenties dat binnen de enterprise-omgeving verbruikt wordt en hoeveel er beschikbaar zijn.

**Endpoint Get License Consumption:**
URL: GET `https://api.github.com/enterprises/equans/consumed-licenses`

Voorbeeldresponse van Postman:

![Afbeelding met tekst, schermopname, Lettertype](media/image5.png)

**Belangrijke velden:**

| Veld                       | Type    | Beschrijving                              |
| :------------------------- | :------ | :---------------------------------------- |
| total_seats_consumed       | Integer | Aantal gebruikte licenties                |
| total_seats_purchased      | Integer | Totaal aantal beschikbare licenties       |
| users\[\].github_com_login | String  | GitHub-loginnaam                          |
| users\[\].github_com_name  | String  | Volledige naam van gebruiker              |
| users\[\].license_type     | String  | Licentietype (Enterprise, Business, etc.) |

De velden `total_seats_consumed` en `total_seats_purchased` maken het direct mogelijk om de licentie-benutting te berekenen. Wanneer het aantal gebruikte licenties dicht bij het beschikbare aantal zit, kan dat duiden op risico van overschrijding van de licentielimiet.

Daarnaast bevat de response een lijst van gebruikers met hun GitHub-loginnaam en het type licentie. Hiermee kun je licentiegebruik per gebruiker of per team analyseren.

**Copilot-gebruik**

Een tweede relevant endpoint betreft het gebruik van GitHub Copilot. Copilot is een AI-programmeerassistent waarvoor aparte licenties nodig zijn. Het endpoint retourneert het aantal beschikbare licenties en wie deze toegewezen heeft gekregen.

**Endpoint Get Copilot Seats:**
URL: GET `https://api.github.com/enterprises/equans/copilot/billing/seats`

Voorbeeldresponse van Postman:

![Afbeelding met tekst, schermopname, Lettertype](media/image6.png)

**Belangrijke velden:**

| Veld                       | Type    | Beschrijving                          |
| :------------------------- | :------ | :------------------------------------ |
| total_seats                | Integer | Totaal aantal Copilot-licenties       |
| seats\[\].plan_type        | String  | Type licentie (business / enterprise) |
| seats\[\].last_activity_at | String  | Laatste activiteit van gebruiker      |
| seats\[\].assignee.login   | String  | Gebruikersnaam met Copilot-seat       |

Met name het veld `last_activity_at` is hier waardevol. Wanneer een gebruiker langere tijd geen activiteit vertoont, is dat een aanwijzing dat de Copilot-licentie mogelijk overbodig is. Tijdens het doorlopen van de testdata viel op dat een deel van de Copilot-seats al maandenlang niet meer actief gebruikt werd.

**GitHub Advanced Security**

De GitHub API biedt ook informatie over het gebruik van GitHub Advanced Security (GHAS). Dit onderdeel biedt beveiligingsfunctionaliteiten zoals code-analyse en kwetsbaarheidsscans.

Het GHAS-endpoint geeft informatie over repositories en het aantal actieve committers dat gebruikmaakt van beveiligingsfuncties.

**Endpoint GHAS:**
URL: GET `https://api.github.com/enterprises/equans/settings/billing/advanced-security`

Voorbeeldresponse van Postman:

![Afbeelding met tekst, schermopname, Lettertype](media/image7.png)

**Belangrijke velden:**

| Veld                                                                   | Type    | Beschrijving                   |
| :--------------------------------------------------------------------- | :------ | :----------------------------- |
| repositories\[\].name                                                  | String  | Repositorynaam                 |
| repositories\[\].advanced_security_committers                          | Integer | Aantal actieve committers      |
| repositories\[\].advanced_security_committers_breakdown\[\].user_login | String  | Gebruikersnamen van committers |

Met deze gegevens kun je vaststellen in welke repositories beveiligingsfuncties daadwerkelijk actief gebruikt worden. Dat is nuttig voor rapportages over beveiligingsgebruik en compliance binnen de softwareontwikkelingsprojecten van Equans.

### 3.3 Beperkingen van de Atlassian API en de rol van aanvullende data

Hoewel de Atlassian Admin API waardevolle informatie biedt over gebruikers en producttoegang, bleek uit de analyse dat de beschikbare data beperkt is tot operationele accountinformatie. De API bevat geen gegevens over organisatorische structuren zoals business units, cost centers of geografische locaties.

Om licentiegebruik goed te analyseren is het noodzakelijk om gebruikersdata te koppelen aan organisatorische informatie. De Atlassian Admin API levert prima gegevens over gebruikers, producttoegang en activiteit, maar bevat slechts beperkte context over de organisatorische structuur. Velden als business units, cost centers en formele HR-identificaties ontbreken. Hierdoor is licentiegebruik niet direct te relateren aan de interne organisatiestructuur van Equans. Een uitdaging hierbij was om een manier te vinden om deze kloof te overbruggen.

**Beschikbare data**

Om deze beperking op te lossen is aanvullende masterdata uit het Palantir-platform gebruikt. Palantir fungeert binnen de organisatie als centrale bron voor organisatorische en personeelsinformatie. Door de gegevens uit Atlassian te verrijken met organisatorische informatie uit Palantir ontstaat een completer datamodel dat geschikt is voor licentieanalyse, kostenallocatie en rapportage.

**Wat is Palantir?**

Palantir Technologies ontwikkelt softwareplatforms waarmee organisaties grote en complexe datasets integreren, analyseren en beheren. De software brengt data uit verschillende systemen samen in één geïntegreerde omgeving, waardoor verbanden tussen datasets zichtbaar worden en besluitvorming ondersteund kan worden met data-gedreven inzichten (_Palantir Foundry_, z.d.).

Een kenmerk van het Palantir-platform is dat het data uit uiteenlopende bronnen combineert, zonder dat die bronnen eerst volledig opnieuw ingericht hoeven te worden. In plaats daarvan wordt een geïntegreerd datamodel gecreëerd waarin relaties tussen entiteiten (zoals personen, organisaties en systemen) expliciet worden vastgelegd. Hierdoor kunnen organisaties data uit verschillende operationele systemen analyseren binnen één context. Volgens Palantir maakt deze aanpak het mogelijk om operationele data, bedrijfsprocessen en organisatorische structuren met elkaar te verbinden (Palantir Technologies, z.d.).

Binnen veel organisaties wordt Palantir ingezet voor data-integratie en besluitondersteuning. Het systeem combineert bijvoorbeeld informatie uit HR-systemen, financiële systemen en operationele applicaties. Dit geeft organisaties de mogelijkheid om complexe datasets te analyseren en beter inzicht te krijgen in processen, kostenstructuren en operationele risico's.

**Rol van Palantir binnen de organisatie**

Binnen Equans vervult Palantir de rol van organisatorische masterdata-bron. In tegenstelling tot systemen als Atlassian, die voornamelijk operationele gegevens bevatten over gebruikersactiviteit en productgebruik, slaat Palantir informatie op over de formele structuur van de organisatie.

De dataset bevat onder andere informatie over personen en organisaties. Voor personen gaat het om velden als `person_id`, `person_first_name`, `person_last_name`, `person_email`, `person_local_id` en `org_id`. Daarnaast worden aanvullende gegevens vastgelegd, zoals het land waarin een medewerker werkzaam is en de bijbehorende billing-locatie. Voor organisaties zijn dat velden als `org_id`, `org_name`, `org_country`, `business_unit` en `org_billing_location`.

**Beschikbare data**

Deze informatie vormt de organisatorische context die nodig is om technische gebruikersdata te interpreteren. Door medewerkers te koppelen aan een `org_id` kun je bijvoorbeeld bepalen binnen welke business unit een licentie gebruikt wordt. Op die manier kunnen rapportages gegenereerd worden waarin licentiegebruik uitgesplitst is per organisatie, land of business unit.

**Dataverrijking via integratie van Atlassian en Palantir**

De integratie van Atlassian-data met Palantir-data vormt een kernonderdeel van het datamodel. Atlassian levert informatie over gebruikersaccounts, producttoegang en activiteit, terwijl Palantir de organisatorische structuur bevat. Door beide datasets te combineren ontstaat een completer beeld van het werkelijke gebruik van softwarelicenties.

De koppeling tussen beide datasets vindt plaats op basis van e-mailadressen. Atlassian identificeert gebruikers primair via een `account_id`, terwijl Palantir medewerkers identificeert via een `person_id`. Omdat deze identifiers niet direct overeenkomen, is een koppeling gemaakt via het e-mailadres dat in beide systemen aanwezig is. Hierdoor kan een Atlassian-account gekoppeld worden aan een specifieke persoon en organisatie binnen Palantir. In eerste instantie is geprobeerd om de koppeling puur op basis van namen te doen, maar dit bleek niet betrouwbaar genoeg vanwege naamvariaties en duplicaten. Het e-mailadres gaf een veel betere match.

Hierdoor wordt het mogelijk om licentiegebruik niet alleen op individueel niveau te analyseren, maar ook op organisatieniveau. Zo kun je bijvoorbeeld bepalen hoeveel licenties een specifieke business unit verbruikt en hoeveel daarvan daadwerkelijk actief zijn. Deze informatie is vervolgens bruikbaar voor kostenanalyse, licentieoptimalisatie en chargeback-rapportages.

De analyse laat zien dat Atlassian en Palantir verschillende soorten informatie bevatten die elkaar aanvullen. Atlassian levert operationele data over gebruikers, producttoegang en activiteit, terwijl Palantir organisatorische masterdata bevat over personen en organisaties. Geen van beide systemen bevat afzonderlijk alle informatie die nodig is voor een volledige licentieanalyse.

Door beide datasets te integreren ontstaat een datamodel waarin licentiegebruik gekoppeld kan worden aan organisatorische structuren. Dit maakt het mogelijk om licenties toe te wijzen aan business units, kostenanalyses uit te voeren en rapportages te genereren voor management en financiële afdelingen. De integratie van Atlassian-data met Palantir-masterdata vormt daarmee een essentieel onderdeel van de architectuur van het Operational Insights Dashboard.

**Beschikbare gegevens binnen Palantir**

Palantir fungeert binnen Equans als bron voor organisatorische en personele masterdata. De dataset bevat onder andere:

- Personen (`person_id`, `person_first_name`, `person_last_name`, `person_email`, `person_local_id`, `org_id`, `country`, `person_billing_location`, `gid`, `created_at`, `updated_at`)
- Organisaties (`org_id`, `org_name`, `org_country`, `org_billing_location`, `business_unit`, `person_count`, `created_at`, `updated_at`)

Deze gegevens maken het mogelijk om:

1. Medewerkers te koppelen aan formele organisatiestructuren
2. Licenties toe te wijzen aan business units
3. Rapportages op land- of regioniveau te genereren

Palantir bevat daarmee precies die organisatorische context die in Atlassian ontbreekt.

## 4. Meten van actief licentiegebruik

Een veelgebruikte maatstaf voor licentie-efficiëntie is het aantal actieve gebruikers binnen een bepaalde periode (30, 60 of 90 dagen). Atlassian biedt hiervoor binnen de Organizations API het endpoint "User's last active dates", waarmee per gebruiker de datum van de laatste activiteit per product opgevraagd kan worden.

Binnen Atlassian wordt "actief" gedefinieerd als het bezoeken van een productpagina gedurende een minimale tijdsduur. Een gebruiker geldt als licentiehouder wanneer die lid is van een productgroep én de accountstatus actief is.

Met deze gegevens kan Equans het percentage licenties berekenen dat echt in gebruik is geweest. Door het aantal gebruikers met een recente `last_active` datum te delen door het totale aantal toegewezen licenties ontstaat inzicht in de mate van benutting.

In de praktijk ziet de meetmethode er zo uit:

1. Alle gebruikers ophalen via de Atlassian Admin API.
2. Voor iedere gebruiker de `last_active` timestamp opvragen.
3. Gebruikers filteren op basis van activiteit binnen 30/60/90 dagen.
4. Het aantal actieve accounts vergelijken met het totaal aantal toegewezen licenties.

Hiermee kun je onderscheid maken tussen daadwerkelijk gebruikte licenties en ongebruikte seats.

### 4.1 Beperking van de Atlassian API: geen historische usage trends

Hierbij bleek een flinke beperking van de Atlassian Cloud API: de API retourneert uitsluitend een actuele `last_active` timestamp. De API biedt géén historische trenddata, zoals:

- Gebruik over de afgelopen 30 dagen (geaggregeerd gebruik)
- Historische activiteitsontwikkeling per gebruiker
- Maandelijkse groei- of dalingstrends

Dit betekent dat je rechtstreeks via de API geen historische gebruikspatronen kunt analyseren. Dat was een probleem, want juist die trends zijn nodig om goed onderbouwde besparingsbeslissingen te nemen.

### 4.2 Technische oplossing: zelf historische data opslaan

Om de deelvraag "Hoe kan het actieve gebruik van licenties gemeten worden (30/60/90 dagen)?" volledig te beantwoorden, was een aanvullende architectuuroplossing noodzakelijk. De Atlassian API levert deze data simpelweg niet out-of-the-box.

De voorgestelde oplossing bestaat uit:

- Het opslaan van gebruikers- en activiteitssnapshots in een eigen PostgreSQL-database
- Het dagelijks uitvoeren van een geautomatiseerde achtergrondtaak (cron job)
- Het bewaren van historische gegevens in een append-only model
- Het berekenen van trends op basis van opgeslagen snapshots

Door iedere nacht een geplande synchronisatie uit te voeren, wordt de status van gebruikers en hun `last_active` waarde vastgelegd. Op basis van deze historische dataset kunnen vervolgens:

- Maandelijkse activiteitsgrafieken
- Trendanalyses (stijgend/dalend gebruik)
- Identificatie van structureel inactieve accounts

berekend worden.

Deze aanpak zet een statische API-response om in een dynamisch historisch analysemodel. Hoewel Atlassian een `last_active` timestamp beschikbaar stelt, biedt de standaard API geen ingebouwde trendanalyse. Door een eigen data-opslaglaag en geplande synchronisaties te bouwen kan Equans alsnog betrouwbare 30/60/90-dagen analyses uitvoeren. Tijdens het ontwikkelen van deze oplossing viel op dat het append-only model ook handig is voor audit-doeleinden, omdat je zo precies kunt terugzien wanneer een account voor het laatst actief was.

## 5. Kostenanalyse per product, site en team

Voor effectieve optimalisatie van licentiekosten is inzicht in de kostenstructuur nodig van zowel Atlassian Cloud Enterprise als GitHub Enterprise Cloud. Beide platformen werken met een licentie-gebaseerd abonnementsmodel waarbij kosten berekend worden op basis van toegewezen gebruikers (seats). Om de deelvraag te beantwoorden ("Hoe kunnen de kosten per product, site en team inzichtelijk gemaakt worden?") is een gestructureerde analysemethode opgezet die zowel technische als financiële aspecten omvat.

### 5.1 Kostenmodel Atlassian Cloud Enterprise

Atlassian Cloud Enterprise hanteert het Maximum Quantity Billing (MQB)-principe. Dit houdt in dat binnen een factureringsperiode gefactureerd wordt op basis van het hoogste aantal toegewezen licenties, ongeacht latere deactivatie (_Atlassian Support_, z.d.-a). Hierdoor kunnen tijdelijke pieken in gebruikersaantallen leiden tot structureel hogere factuurbedragen. In de praktijk betekent dit: als je in één maand 500 seats hebt en vervolgens 50 deactiveert, betaal je nog steeds voor die 500.

Aangezien Atlassian geen publieke REST API biedt voor directe facturerings- of kosteninformatie (Atlassian, z.d.-b), is de kostenanalyse gebaseerd op:

1. Het aantal toegewezen gebruikers per product (via Admin API).
2. De eenheidsprijs per licentie (uit het abonnementsoverzicht).
3. Historische snapshots om piekwaarden te detecteren.

### 5.2 Kostenmodel GitHub Enterprise Cloud

GitHub Enterprise Cloud werkt ook met een seat-based abonnementsmodel. Via de Enterprise Licensing API zijn het aantal geconsumeerde licenties (`total_seats_consumed`) en aangeschafte licenties (`total_seats_purchased`) op te halen (GitHub, z.d.-a).

Voor aanvullende producten zoals:

- GitHub Copilot Business/Enterprise
- GitHub Advanced Security (GHAS)

worden aparte API-endpoints gebruikt om adoptie en activiteit te meten (GitHub, z.d.-b; GitHub, z.d.-c). In tegenstelling tot Atlassian biedt GitHub meer inzicht in licentiegebruik via de Enterprise Admin API. Maar ook hier geldt dat factuurbedragen niet rechtstreeks via de API beschikbaar zijn en op basis van contracttarieven berekend moeten worden.

### 5.3 Inzicht per product

Voor zowel Atlassian als GitHub is inzicht per product gerealiseerd door:

- Gebruikers te identificeren via productgroepen (Atlassian) of enterprise seats (GitHub).
- Actieve en inactieve gebruikers te onderscheiden.
- Seat-toewijzing te koppelen aan tariefstructuur.

Dit levert een overzicht op van:

- Totale kosten per product
- Percentage actieve seats
- Potentiële besparingsmogelijkheden

### 5.4 Inzicht per site

Binnen Atlassian Enterprise vallen meerdere sites onder één abonnement. Per site is inzicht te verkrijgen door:

- Groepen en gebruikers per site te aggregeren.
- Seat-distributie per site te berekenen.
- Kosten per site apart te visualiseren.

Dit maakt geografische of organisatorische vergelijking mogelijk. GitHub Enterprise hanteert een enterprise-brede structuur, maar repositories en teams kunnen gebruikt worden om kostenallocatie op suborganisatieniveau te realiseren.

### 5.5 Inzicht per team (chargeback-structuur)

Om kosten per team inzichtelijk te maken, is een koppeling nodig tussen:

- Gebruikersdata (API)
- Organisatorische mapping (bijvoorbeeld Business Unit of Active Directory-groepen)

Vervolgens kan kostenallocatie worden uitgevoerd conform IT Financial Management-principes, waarbij kosten worden toegerekend aan organisatorische entiteiten op basis van daadwerkelijk gebruik (Blokdyk, 2020).

Deze aanpak ondersteunt chargeback- en showback-mechanismen. In de praktijk betekent dit dat een teammanager precies kan zien wat zijn team kost aan licenties, wat een prikkel geeft tot kostenbewust omgaan met tooling.

### 5.6 Architectuurvereisten voor betrouwbare kostenanalyse

Omdat beide platformen geen historische billingdata via hun API leveren, is een aanvullende architectuur noodzakelijk:

- Dagelijkse synchronisatie (cron jobs)
- Opslag van gebruikers- en seat-snapshots
- Append-only datamodel
- Trendberekening over tijd

Hierdoor kunnen:

- Maandelijkse kostenontwikkeling
- Seat-fluctuaties
- Structurele overallocatie

geanalyseerd worden. Deze architectuur waarborgt reproduceerbaarheid en auditability van financiële rapportages. Dit was een bewuste keuze: zonder deze historische laag zou iedere analyse alleen een momentopname zijn, en dat is niet genoeg voor weloverwogen besparingsbeslissingen.

### 5.7 Visualisatie en besluitvorming

Kosteninformatie moet zo gepresenteerd worden dat besluitvorming ondersteund wordt. Volgens Few (2013) moeten dashboards kerninformatie direct zichtbaar maken en hiërarchisch structureren.

Binnen dit project betekent dat:

- KPI-kaarten (totale kosten, besparingspotentieel)
- Staafdiagrammen (kosten per product)
- Lijngrafieken (trendontwikkeling)
- Drill-down structuur (BU → Product → Team)

Hoewel Atlassian en GitHub geen directe kosten-API's aanbieden, maken hun administratieve endpoints het wel mogelijk om via berekening en aggregatie betrouwbare kosteninzichten te genereren.

## 6. Patronen van inefficiënt gebruik

Het vaststellen van patronen die wijzen op inefficiënt of onbenut gebruik van Atlassian Cloud Enterprise en GitHub Enterprise Cloud licenties vormt een essentieel onderdeel van dit project. Inefficiënt licentiegebruik leidt niet alleen tot directe financiële verspilling, maar brengt ook risico's met zich mee op het gebied van governance, toegangsbeheer en informatiebeveiliging. In dit hoofdstuk worden de belangrijkste indicatoren van inefficiënt gebruik systematisch geanalyseerd.

### 6.1 Langdurig inactieve accounts

De meest voor de hand liggende indicator is langdurig inactieve accounts. Binnen Atlassian Cloud levert de Organizations REST API per gebruiker een `last_active` timestamp (_The Organizations REST API REST API_, z.d.-a). Wanneer daaruit blijkt dat een gebruiker al 60 of 90 dagen geen activiteit vertoont, terwijl de accountstatus nog "active" is en de gebruiker lid blijft van een productgroep, is de kans groot dat de toegewezen licentie niet nodig is.

Dit krijgt extra gewicht binnen het Maximum Quantity Billing-model, waarbij het hoogste aantal toegekende licenties in een factureringsperiode bepalend is voor de kosten, ongeacht feitelijk gebruik (Atlassian, z.d.-b). Concreet betekent dit dat volledig inactieve accounts financieel doorbelast worden zolang ze niet expliciet gedeactiveerd worden. Een kwantitatieve analyse van `last_active`-gegevens in combinatie met groepslidmaatschap maakt het daarom mogelijk om concreet besparingspotentieel te identificeren.

Tijdens het analyseren van de testdata viel op dat er accounts waren die al meer dan zes maanden inactief waren, maar nog steeds volledige producttoegang hadden. Dat zijn potentieel de snelste besparingswinsten.

### 6.2 Externe gebruikers met billable toegang

Een tweede patroon doet zich voor bij externe gebruikers met billable toegang. Binnen Atlassian Guard worden zowel managed accounts als externe accounts met producttoegang als factureerbaar beschouwd (Atlassian, z.d.-c). Wanneer externe gebruikers (contractors of tijdelijke medewerkers) structureel toegang behouden zonder aantoonbare recente activiteit, staan de licentiekosten niet in verhouding tot het werkelijke gebruik. Dezelfde logica geldt voor GitHub Enterprise Cloud, waar iedere gebruiker met een toegewezen seat meetelt in de totale licentiekosten (_Licensing - GitHub Enterprise Cloud Docs_, z.d.).

Door e-maildomeinen en directory-attributen te analyseren kun je vaststellen welke accounts extern zijn en of hun gebruik een volledige seat rechtvaardigt. Het systematisch monitoren van deze categorie voorkomt dat tijdelijke of incidentele gebruikers structureel kosten genereren.

### 6.3 Governance en overmatige adminrechten

Naast gebruiksdata speelt governance ook een rol in het signaleren van inefficiëntie. Een onnodig hoog aantal beheerdersaccounts kan duiden op gebrekkig rolbeheer en onvoldoende periodieke herziening van toegangsrechten. Atlassian onderscheidt verschillende administratieve rollen, waaronder Site Administrators en Product Administrators (Atlassian, z.d.-d). Hoewel deze rollen niet direct hogere kosten veroorzaken, vergroten ze de kans op onbedoelde configuratiewijzigingen en ongecontroleerde licentietoewijzing.

Vanuit informatiebeveiligingsperspectief schrijft het principe van "least privilege" voor dat gebruikers uitsluitend de minimaal noodzakelijke rechten mogen bezitten (ISO/IEC, 2022). Wanneer het aantal administratieve accounts structureel hoger is dan functioneel nodig, wijst dit op een governanceprobleem dat indirect bijdraagt aan inefficiënt licentiebeheer.

### 6.4 Structurele overallocatie van licenties

Een vierde indicator betreft structurele overallocatie van licenties (ook wel overprovisioning). Dit patroon ontstaat wanneer het aantal aangeschafte seats consequent hoger ligt dan het gemiddelde actieve gebruik. Binnen GitHub Enterprise kun je via de Licensing API het verschil tussen `total_seats_purchased` en `total_seats_consumed` vaststellen (_Licensing - GitHub Enterprise Cloud Docs_, z.d.).

Wanneer dit verschil structureel aanwezig is (bijvoorbeeld doordat piekgebruik bepalend is voor contractafspraken maar het gemiddelde gebruik veel lager ligt) ontstaat een inefficiënte kostenstructuur. Het periodiek analyseren van trenddata maakt het mogelijk om contractonderhandelingen en seat-reducties beter te onderbouwen.

### 6.5 Ontbrekende kostenallocatie per organisatorische eenheid

Tot slot is het ontbreken van een koppeling tussen licentiekosten en organisatorische eenheden een belangrijke oorzaak van inefficiënt gebruik. Wanneer gebruikers niet systematisch gekoppeld zijn aan Business Units of teams, ontbreekt de financiële verantwoordelijkheid. IT Financial Management-literatuur benadrukt dat kostenallocatie en chargeback-mechanismen essentieel zijn om kostenbewust gedrag te stimuleren (Blokdyk, 2020). Zonder inzicht in kosten per afdeling of project blijft optimalisatie een centrale IT-verantwoordelijkheid, terwijl het gebruik feitelijk bij de business ligt.

Het koppelen van gebruikersdata aan organisatorische structuren vergroot daarom niet alleen het inzicht, maar versterkt ook de governance rondom licentiebeheer.

Samengevat manifesteert inefficiënt licentiegebruik zich in terugkerende patronen: langdurig inactieve maar factureerbare accounts, externe gebruikers zonder aantoonbare noodzaak, te veel administratieve rechten, structurele overprovisioning van seats en het ontbreken van kostenallocatie per organisatorische eenheid. Door deze patronen systematisch te monitoren via API-gebaseerde data-analyse kan Equans gerichte optimalisatiemaatregelen nemen die zowel financiële besparingen als verbeterde governance opleveren.

## 7. Presentatie in dashboard en aanbevelingen

Het verzamelen en analyseren van gegevens over licentiegebruik en kosten is eigenlijk slechts een tussenstap. De echte meerwaarde ontstaat pas wanneer deze inzichten zo gepresenteerd worden dat ze besluitvorming ondersteunen. Binnen dit project is daarom niet alleen gekeken naar de technische haalbaarheid van data-analyse via de API's, maar ook naar de vraag hoe deze informatie effectief gevisualiseerd kan worden in een dashboard dat zowel management als operationele teams helpt. De vraag was: hoe toon je de verzamelde informatie op een manier die direct leidt tot actie?

### 7.1 Overzichtelijkheid en directe zichtbaarheid van KPI's

Een goed dashboard geeft in één oogopslag inzicht in de meest relevante prestatie-indicatoren. Volgens Few (2013) moeten kerncijfers direct zichtbaar zijn, zonder dat de gebruiker eerst door meerdere schermen moet navigeren. Binnen het Operational Insights Dashboard betekent dit dat indicatoren als het totaal aantal licenties per product, het percentage actieve versus inactieve gebruikers, de maandelijkse kosten en het geschatte besparingspotentieel bovenaan het dashboard staan. Op die manier is de meest relevante informatie direct beschikbaar voor besluitvormers.

### 7.2 Hiërarchische ordening van informatie

Naast directe zichtbaarheid is de hiërarchische ordening van informatie minstens zo belangrijk. Tufte (2001) benadrukt dat kwantitatieve informatie zo gepresenteerd moet worden dat de structuur intuïtief begrijpelijk is. Dit betekent dat het dashboard opgebouwd is van algemeen naar specifiek. Strategische informatie (totale kosten, globale benuttingspercentages) krijgt een centrale positie (Tufte, 2001). Daaronder volgen tactische inzichten, zoals kostenverdelingen per product of Business Unit en trendanalyses over meerdere maanden. Op het meest gedetailleerde niveau kunnen specifieke gebruikersgegevens of lijsten met inactieve accounts bekeken worden.

Deze gelaagde structuur zorgt ervoor dat verschillende typen gebruikers (van management tot IT-beheerders) dezelfde applicatie kunnen gebruiken, maar op een ander detailniveau. In de praktijk bleek dat managers vooral geïnteresseerd zijn in besparingspotentieel, terwijl beheerders juist de detaillijsten met inactieve accounts willen zien.

### 7.3 Keuze van visualisatietypen

De keuze van visualisatietypen speelt een grote rol in de begrijpelijkheid van het dashboard. Datavisualisatie moet aansluiten bij het type data. Discrete categorieën (zoals productnamen of teams) worden het best weergegeven met staafdiagrammen, omdat die directe vergelijking mogelijk maken. Trends over tijd lenen zich juist voor lijngrafieken, omdat die ontwikkeling en richting zichtbaar maken (Few, 2013). Door deze principes consequent toe te passen, wordt het risico op misinterpretatie beperkt en ontstaat een consistente visuele taal.

### 7.4 Van inzicht naar concrete aanbevelingen

Een aspect dat nadrukkelijk is meegenomen in het ontwerp is de vertaling van analyse naar concrete aanbevelingen. IT Financial Management benadrukt dat transparantie in kosten pas effectief is als het gekoppeld wordt aan verantwoordelijkheid en actie (Blokdyk, 2020). Een dashboard dat alleen cijfers toont zonder interpretatie draagt onvoldoende bij aan optimalisatie.

Daarom zijn in het voorgestelde ontwerp aanbevelingen expliciet gekoppeld aan de gevisualiseerde data. Wanneer bijvoorbeeld een significant aantal gebruikers gedurende negentig dagen geen activiteit vertoont, wordt dat niet alleen getoond in een grafiek, maar ook vertaald naar een concrete suggestie: deactiveer deze inactieve accounts, en dit levert geschat X euro besparing op. Op die manier is het dashboard niet slechts een rapportagetool, maar een instrument voor besluitvorming.

### 7.5 Interactiviteit en gebruikerservaring

Interactiviteit vormt een essentieel onderdeel van de presentatie. Hierbij is gekozen voor een Single Page Application-architectuur op basis van React, waarmee filters en drill-downfuncties werken zonder pagina-herlaadmomenten. Hierdoor kan de gebruiker eenvoudig navigeren van Business Unit naar productniveau en uiteindelijk naar individuele gebruikers. Deze vorm van interactie stimuleert actieve analyse en maakt het mogelijk om vragen direct binnen het dashboard te beantwoorden.

De combinatie van hiërarchische structuur, juiste visualisatiekeuzes en expliciete aanbevelingen levert een dashboard op dat niet alleen inzicht biedt, maar ook richting geeft aan optimalisatiebeslissingen.

Samenvattend wordt de informatie over licentiegebruik en kosten het best gepresenteerd in een hiërarchisch opgebouwd, interactief dashboard waarin kernindicatoren direct zichtbaar zijn, trends visueel worden ondersteund en aanbevelingen gekoppeld zijn aan geconstateerde inefficiënties. Op deze manier wordt het dashboard een strategisch hulpmiddel dat financiële transparantie combineert met concrete optimalisatieacties.

## 8. Technologische keuzes en onderzochte alternatieven

Bij de ontwikkeling van het Operational Insights Dashboard is bewust gekozen voor een architectuur met een Rust-backend en een React-frontend met TypeScript. Deze keuze is niet gebaseerd op persoonlijke voorkeur, maar op een systematische vergelijking van alternatieven. Performance, onderhoudbaarheid, schaalbaarheid en veiligheid waren de beoordelingscriteria.

### 8.1 Frontend-keuze

**Onderzoek naar Angular**

Angular is een compleet frontend-framework dat veel out-of-the-box biedt: dependency injection, routing, state management en een sterke projectstructuur (Google Books, z.d.). In enterprise-omgevingen wordt Angular veel ingezet vanwege deze gestructureerde aanpak.

Tijdens de analysefase is Angular serieus overwogen vanwege:

- De uitgebreide ingebouwde functionaliteit
- Enterprise-geschiktheid
- TypeScript-integratie

Toch is Angular uiteindelijk niet gekozen. De belangrijkste reden was de complexiteit: Angular vereist het beheersen van meerdere concepten tegelijk (modules, services, decorators, dependency injection), wat leidt tot een steile leercurve en relatief veel boilerplate-code. Voor een dashboardtoepassing zonder complexe formulieren of uitgebreide businesslogica was dat disproportioneel.

Daarnaast is Angular zwaarder in bundlegrootte vergeleken met React, wat een negatieve impact kan hebben op performance bij data-intensieve visualisaties (_Angular_, z.d.).

Conclusie: Angular biedt sterke structuur, maar de complexiteit en overhead zijn niet in verhouding tot de scope van dit project.

**Onderzoek naar Vanilla JavaScript**

Een alternatief was pure JavaScript zonder framework. Dit biedt maximale controle en minimale afhankelijkheid van externe libraries.

Hoewel dit aantrekkelijk lijkt vanuit eenvoud, heeft deze aanpak duidelijke beperkingen:

- Geen gestandaardiseerde componentstructuur
- Handmatig DOM-beheer
- Geen ingebouwde state-management patronen
- Geen statische typecontrole

Bij groei van de applicatie zou dit leiden tot verminderde onderhoudbaarheid en meer runtime-fouten. Onderzoek naar software-architectuur toont aan dat component-gebaseerde structuren bijdragen aan schaalbaarheid en herbruikbaarheid (Bass, Clements, & Kazman, 2022).

Conclusie: Vanilla JavaScript is onvoldoende schaalbaar voor een groeiend dashboard met meerdere views, filters en drill-downfunctionaliteit.

**Onderzoek naar server-rendered UI (Razor, Django Templates)**

Server-side rendering (SSR) is onderzocht vanwege de eenvoud bij traditionele webapplicaties. Frameworks als ASP.NET Razor of Django Templates bieden snelle implementatie en duidelijke scheiding tussen data en presentatie.

Voor een interactieve dashboardomgeving met veel dynamische filtering, grafieken en client-side interactie bleek SSR minder geschikt. Iedere interactie zou een server request vereisen, wat zorgt voor:

- Extra latency
- Minder vloeiende gebruikerservaring
- Complexere state-handling

Volgens moderne webarchitectuurprincipes zijn Single Page Applications (SPA's) beter geschikt voor data-analyseplatformen vanwege hun interactieve karakter (Tilkov & Vinoski, 2010).

Conclusie: Voor een data-intensief dashboard is een client-side SPA-architectuur geschikter dan server-rendering.

**Keuze voor React met TypeScript**

React is gekozen vanwege de componentgebaseerde architectuur en het brede ecosysteem (React, z.d.). Het framework is lichtgewicht en biedt flexibiliteit in architectuurkeuzes.

TypeScript is toegevoegd om statische typecontrole te hebben. Statische typisering vermindert runtime-fouten en verhoogt onderhoudbaarheid in grotere applicaties (Microsoft, z.d.). In een dashboard met complexe datamodellen en API-responses is typeveiligheid echt nodig. Tijdens het ontwikkelen bleek dit vooral handig bij het verwerken van API-responses, omdat je fouten direct bij het builden ziet in plaats van pas in productie.

Voordelen van React + TypeScript:

- Component-gebaseerde opbouw
- Hoge herbruikbaarheid
- Sterke community en ecosystem (Recharts, MUI)
- Compile-time typecontrole
- Geschikt voor schaalbare SPA-architecturen

### 8.2 Backend-keuze

**Onderzoek naar Python (Flask/FastAPI)**

Python is onderzocht vanwege de snelle ontwikkelsnelheid en brede ondersteuning voor API-integraties. Frameworks als FastAPI bieden relatief eenvoudige implementatie van REST-API's.

Hoewel Python prima is voor prototyping, heeft het beperkingen:

- Dynamische typing (fouten pas zichtbaar bij runtime)
- Global Interpreter Lock (GIL) bij CPU-intensieve processen
- Minder voorspelbare performance bij hoge concurrency

Voor een systeem dat meerdere externe API's parallel moet aanroepen (Atlassian, GitHub, JFrog) is concurrency erg belangrijk. Volgens performance-onderzoek presteert Python minder efficiënt bij parallelle workloads dan gecompileerde talen (McKinney, 2017).

Conclusie: Python is geschikt voor snelle prototypes, maar minder optimaal voor een performante productie-omgeving.

**Onderzoek naar .NET**

.NET (ASP.NET Core) is ook onderzocht. Het platform biedt sterke enterprise-integratie, goede performance en uitgebreide tooling (Microsoft, z.d.).

Hoewel .NET een solide keuze zou zijn geweest, is uiteindelijk gekozen voor Rust vanwege:

- Hogere controle over geheugenbeheer
- Geen garbage collection
- Compile-time concurrency checks
- Zeer lage latency

**Keuze voor Rust**

Rust biedt memory safety zonder garbage collector en voorkomt data races via het ownership-model (Klabnik & Nichols, 2023). Dit maakt Rust bijzonder geschikt voor:

- Gelijktijdige API-calls
- Rate-limit handling
- Hoge performance workloads
- Veilig verwerken van externe data

Binnen dit project is performance erg belangrijk, omdat:

- Grote datasets opgehaald worden via meerdere API's
- Rate limits gerespecteerd moeten worden
- Data veilig opgeslagen moet worden

Rust combineert lage latency met hoge betrouwbaarheid en compile-time foutdetectie. Een uitdaging hierbij was de leercurve van het ownership-model, maar de voordelen in termen van veiligheid en performance wegen daar ruimschoots tegen op.

Op basis van de uitgevoerde analyse vormen React met TypeScript en Rust de meest geschikte combinatie voor dit project. Alternatieven als Angular, Vanilla JavaScript, server-rendered UI, Python en .NET zijn onderzocht, maar bleken minder passend gezien de eisen aan performance, schaalbaarheid en onderhoudbaarheid.

De uiteindelijke keuze is niet puur technologisch gemotiveerd, maar gebaseerd op een afweging tussen complexiteit, onderhoudbaarheid, veiligheid en performance.

## 9. Conclusie

Dit onderzoek had als doel te bepalen hoe Equans inzicht kan krijgen in het gebruik van Atlassian Cloud Enterprise-licenties en de bijbehorende kosten, om besparingsmogelijkheden te identificeren en te optimaliseren. Op basis van de analyse kan geconcludeerd worden dat de Atlassian Cloud Admin API's, ondanks het ontbreken van een directe kosten- of facturerings-API, voldoende gegevens leveren om een betrouwbaar beeld van licentiegebruik op te bouwen.

Via endpoints voor organisaties, groepen, gebruikers en last-active data is nauwkeurig vast te stellen welke accounts echt actief zijn en welke licenties structureel onbenut blijven. Hoewel Atlassian geen kosteninformatie via de API beschikbaar stelt, maakt het Maximum Quantity Billing-model het mogelijk om op basis van seat-aantallen en contracttarieven een valide kostenberekening uit te voeren (The Organizations REST API REST API, z.d.-a).

Door deze technische gegevens te combineren met organisatorische mapping (op Business Unit-niveau) ontstaat inzicht in kostenallocatie en benuttingsgraad per afdeling. Hieruit blijkt dat inefficiënt licentiegebruik zich met name toont in langdurig inactieve accounts, externe gebruikers met billable toegang zonder recente activiteit en structurele overallocatie van seats. Deze patronen zijn systematisch te identificeren met periodieke dataverzameling en analyse. Daarmee wordt het mogelijk om niet alleen reactief, maar ook proactief kostenoptimalisatie toe te passen.

Daarnaast blijkt uit het onderzoek dat een goed ontworpen dashboard essentieel is voor effectieve besluitvorming. Door kernindicatoren hiërarchisch en visueel consistent te presenteren, wordt financiële transparantie bevorderd en worden optimalisatiemogelijkheden direct zichtbaar (Few, 2013). Het dashboard fungeert daarmee niet slechts als rapportage-instrument, maar als strategisch hulpmiddel binnen IT Financial Management (Blokdyk, 2020).

Tot slot is de gekozen technologische architectuur (een Rust-backend met een React/TypeScript-frontend) passend voor de aard van het probleem. De backend vereist efficiënte verwerking van externe API-calls en veilige gelijktijdige dataverwerking, terwijl de frontend gebaat is bij een schaalbare, onderhoudbare en typeveilige implementatie. De onderzochte alternatieven boden waardevolle inzichten, maar bleken minder geschikt gezien de eisen aan performance, schaalbaarheid en onderhoudbaarheid.

Samengevat kan Equans met relatief beperkte technische ingrepen aanzienlijke verbeteringen realiseren in kosteninzicht, licentiebeheer en governance rondom Atlassian- en GitHub-licenties.

## 10. Aanbevelingen

Op basis van de bevindingen worden de volgende aanbevelingen gedaan.

- In de eerste plaats is het advies om het voorgestelde dashboard daadwerkelijk te implementeren en structureel te integreren in de IT-beheerprocessen van Equans. De effectiviteit hangt niet alleen af van de technische realisatie, maar ook van periodieke monitoring en organisatorische inbedding. Een dagelijkse of wekelijkse geautomatiseerde synchronisatie van gebruikers- en licentiegegevens via de Atlassian- en GitHub-API's is nodig om actuele en betrouwbare inzichten te behouden.

- Daarnaast is het aan te raden om periodieke licentie-audits te formaliseren. Door bijvoorbeeld elk kwartaal een evaluatie uit te voeren van inactieve accounts en externe gebruikers kan structurele kostenbesparing gerealiseerd worden. Deze audits moeten gekoppeld worden aan duidelijke verantwoordelijkheden per Business Unit, zodat kostenbewust gedrag gestimuleerd wordt.

- Verder is het advies om kostenallocatie expliciet te koppelen aan organisatorische entiteiten. Wanneer kosten per Business Unit transparant zichtbaar zijn, ontstaat een natuurlijke prikkel tot optimalisatie en efficiënter gebruik. Dit sluit aan bij best practices binnen IT Financial Management, waarbij transparantie en toewijsbaarheid van kosten centraal staan (Blokdyk, 2020).

- Tot slot wordt aanbevolen om het dashboard in een latere fase uit te breiden met trendanalyses over langere perioden, automatische waarschuwingen bij overallocatie en integratie met bredere Enterprise-dataplatformen zoals Palantir. Hiermee kan het systeem uitgroeien van een kosteninzichttool naar een integraal governance-instrument.

Door deze aanbevelingen te implementeren kan Equans niet alleen besparen op licentiekosten, maar ook de beheersbaarheid, transparantie en veiligheid van het applicatielandschap verbeteren.

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
