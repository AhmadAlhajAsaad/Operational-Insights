Mockups

Ontwerp van de Gebruikersinterface

Equans Operational Insights Dashboard

Projecttitel: Equans Operational Insights Dashboard

Studentnaam: Ahmad Alhaj Asaad

Opleiding: HBO-ICT Software Engineering

Organisatie: Equans/ SLS-DP-DevOps-Forge

Document title: Prototype Ontwerp: Operational Insights Dashboard

Schoolsbegeleider: Jeroen Boogaard

Bedrijfsbegeleider: Viktor Klein (Business Owner), Brian Veltman (Technisch begeleider) 

Studiejaar: 2025 - 2026

Versie 1.0

 

Inleiding

Inleiding

Dit document beschrijft het ontwerpproces van de grafische gebruikersinterface (GUI) voor het Operational Insights Dashboard. Het vormt een zelfstandige vastlegging van het prototypeontwerp zoals ontwikkeld in Figma, los van de overige projectdocumentatie.

Een gebruiksvriendelijke en intuïtieve interface is essentieel voor de acceptatie van het systeem door de beoogde gebruikersgroepen: licentiebeheerders, teammanagers, finance-medewerkers en IT-beheerders. Het ontwerp is niet louter een esthetische oefening, maar een cruciaal onderdeel van de functionele specificatie. Het vertaalt de gebruikersvereisten naar een concrete interactie- en informatiearchitectuur.

Het ontwerpproces volgde een user-centered design (UCD) methodologie (Norman, 2013), met een iteratieve cyclus van ontwerpen, prototypen, evalueren en verfijnen. Uitgangspunt waren de functionele vereisten zoals vastgelegd in de MoSCoW-analyse. Op basis van deze vereisten zijn eerst wireframes geschetst om de informatiearchitectuur en navigatiestructuur te bepalen, conform het Hub and Spoke-model (Tidwell, 2010). Vervolgens zijn deze wireframes in Figma uitgewerkt tot clickable prototypes, waarbij feedback van stakeholders (Viktor Klein, Brian Veltman) in opeenvolgende sprints is verwerkt.

Dit document presenteert de resultaten van deze ontwerpcycli. Hoofdstuk 2 beschrijft de eerste ontwerpiteratie: low-fidelity wireframes, waarin de nadruk lag op het valideren van de paginastructuur, de belangrijkste datavisualisaties en de algehele gebruikersreis. Deze vroege ontwerpen, hoewel nog niet volledig gedetailleerd of voorzien van de definitieve huisstijl, legden de functionele basis voor het dashboard. Binnen deze iteratie worden de volgende schermen getoond en toegelicht:

**Hoofdstuk 2: Eerste Ontwerpiteratie — Low-Fidelity Wireframes**

Dit hoofdstuk beschrijft de eerste ontwerpcyclus, waarin de nadruk lag op het valideren van de paginastructuur, de informatiearchitectuur en de algehele gebruikersreis. De hier gepresenteerde **low-fidelity wireframes** zijn bewust eenvoudig gehouden en bevatten minimale visuele detaillering. Deze aanpak, conform de principes van _lean UX_ (Gothelf & Seiden, 2016), maakt het mogelijk om snel te itereren en feedback te verzamelen op de functionele opzet, zonder afgeleid te worden door esthetische keuzes. Alle wireframes zijn ontwikkeld in Figma en dienden als basis voor de gebruikerssessies met stakeholders.

**2.1 Inlogscherm met Microsoft SSO**

**Figuur 1 — Low-fidelity wireframe van het inlogscherm**

_Dit wireframe toont de initiële opzet van het inlogscherm, waarbij authenticatie uitsluitend via Microsoft Single Sign-On (SSO) plaatsvindt._

**Toelichting:**

Figuur 1 presenteert het eerste ontwerp van het inlogscherm. In lijn met de beveiligingseisen (TM-01, SRS) en het besluit in ADR-004 is gekozen voor een exclusieve authenticatiemethode: **Microsoft Azure Active Directory (Entra ID)**. Het scherm is bewust eenvoudig gehouden en bevat alleen de essentiële elementen:

- Het **Equans-logo** voor visuele herkenning en brand consistency.
- De titel "Operational Insights" om de functie van de applicatie te communiceren.
- Een duidelijke **call-to-action** knop ("Sign in with Microsoft").
- Een korte **beveiligingsindicatie** ("Secured by Microsoft Entra ID") om gebruikersvertrouwen te versterken.
- Een **footer** met copyright en contactinstructie ("Need access? Contact your administrator").

Er is **géén optie voor een lokale login** met gebruikersnaam en wachtwoord. Dit is een bewuste ontwerpkeuze die meerdere doelen dient:

1.  **Verkleining van het aanvalsoppervlak:** Geen lokale wachtwoorddatabase om te compromitteren.
2.  **Uniforme gebruikerservaring:** Conform de strategie van Equans om voor alle interne applicaties SSO te gebruiken.
3.  **Centraal identiteitsbeheer:** Gebruikersaccounts en -rechten worden centraal beheerd in Azure AD, wat het in- en uitdienstproces vereenvoudigt.

Dit wireframe legde de basis voor alle verdere schermen, aangezien het het toegangspunt tot de applicatie definieert.

Figuur 1 low-fidelity wireframes login

Figuur 1 toont het low-fidelity wireframe van het inlogscherm, met de centrale positie van Microsoft SSO als enige authenticatiemethode.

**2.2 Organisatieoverzicht (Organization Overview)**

**Figuur 2 — Low-fidelity wireframe van het organisatieoverzicht**

_Dit wireframe toont de initiële opzet van het organisatieoverzicht, met KPI-kaarten, een kostentrendgrafiek en een doorzoekbare tabel._

**Toelichting:**

Figuur 2 toont het eerste ontwerp van het organisatieoverzicht, het centrale scherm van het dashboard. Het scherm is opgebouwd uit vier logische secties die de belangrijkste gebruikersvereisten adresseren:

1.  **KPI-kaarten (bovenaan):** Vier kaarten tonen de belangrijkste metrieken: 'Total Monthly Cost', 'Organizations', 'Total Users' en 'Avg Utilization'. De plaatsing bovenaan het scherm volgt het _inverted pyramid_\-model (Krug, 2014), waarbij de meest cruciale informatie direct zichtbaar is zonder scrollen. Placeholders (#, XX%) geven aan dat de exacte waarden later worden ingevuld.
2.  **Kostentrendgrafiek:** Een 'Monthly Cost Trend by Business Unit'-grafiek visualiseert de kostenontwikkeling over zes maanden. In dit low-fidelity stadium is de grafiek eenvoudig gehouden met louter blokken en placeholders, maar de _bedoeling_ (inzicht in trends) is duidelijk. De keuze voor een gestapelde grafiek (stacked chart) is een vooruitwijzing naar de definitieve visualisatie.
3.  **Organisatietabel:** De 'Organizations by Cost'-tabel is het belangrijkste interactieve element. Gebruikers kunnen zoeken via een zoekbalk ('Search by Organization ID') en door de tabel bladeren. De kolommen (Org ID, Org Name, Business Unit, Licenses, Users, Cost, Utilization) zijn geselecteerd op basis van de Must Have-vereisten M-01 (licentieoverzicht) en M-11 (organisaties met personen).
4.  **Voorbeelddata:** Enkele rijen zijn ingevuld met voorbeelddata (bijv. 'Energy Solutions', 'Smart Buildings') om de functionaliteit te illustreren tijdens presentaties.

Dit wireframe diende als basis voor discussies met Viktor Klein en Brian Veltman over de vraag of de juiste informatie op de juiste plek stond.

Figuur 2 Low-fidelity wireframes Organization overview

Figuur 2 presenteert het low-fidelity wireframe van het organisatieoverzicht (Organization Overview), met de eerste aanzet tot KPI-kaarten, een kostentrendgrafiek en een organisatietabel.

**2.3 Productdetailpagina (Product Details)**

**Figuur 3 — Low-fidelity wireframe van de productdetailpagina**

_Dit wireframe toont de initiële opzet van de productdetailpagina, met licentiedistributie en gebruikerstabel._

**Toelichting:**

Figuur 3 illustreert het eerste ontwerp van de productdetailpagina, bereikbaar door op een product in de tabel (Figuur 2) te klikken. Dit scherm geeft gedetailleerd antwoord op de vraag: "Hoe wordt een specifiek product (bijv. Jira) gebruikt binnen de organisatie?".

De pagina is opgebouwd uit de volgende elementen:

- **KPI-kaarten:** Consistent met het organisatieoverzicht tonen ook hier vier kaarten de kerncijfers voor het geselecteerde product ('Total Licenses', 'Active Users', 'Monthly Cost', 'Utilization Rate'). Deze visuele consistentie is een belangrijk ontwerpprincipe.
- **Licentieverdeling:** Een eenvoudige visuele weergave (in dit wireframe nog als blokken) toont de verhouding tussen 'Active Licenses' en 'Unused Licenses' voor verschillende producten (Jira, Confluence, GitHub, Copilot, JForg). Dit adresseert direct de behoefte aan inzicht in ongebruikte licenties (M-01).
- **Top Customers by Usage:** Een tabel toont de grootste afnemers van het product, met kolommen voor 'Total Licenses', 'Active Users' en 'Monthly Cost'. Dit geeft inzicht in kostenconcentratie.
- **Cost & User Trends:** Een eenvoudige trendgrafiek (lijn- of staafdiagram) toont de ontwikkeling van actieve gebruikers en maandelijkse kosten over tijd. Dit voorziet in de behoefte aan trendvisualisatie (M-01).

Dit wireframe maakte duidelijk dat er behoefte was aan een _product selector_ (dropdown) om tussen producten te kunnen wisselen, wat in de tweede iteratie is toegevoegd.

Figuur 3 Low-fidelity wireframes ProductDetails

Figuur 3 illustreert het low-fidelity wireframe van de productdetailpagina (Product Details), waar per product licentiedetails en gebruiksgegevens worden gevisualiseerd.

**2.4 Gebruikersbeheer (Users)**

**Figuur 4 — Low-fidelity wireframe van het gebruikersbeheerscherm**

_Dit wireframe toont de initiële opzet van het gebruikersbeheerscherm, met zoek- en filterfunctionaliteit en een overzichtstabel._

**Toelichting:**

Figuur 4 toont het eerste ontwerp van het gebruikersbeheerscherm, ontworpen voor IT-beheerders. Dit scherm geeft een volledig overzicht van alle personen in het systeem en biedt functionaliteit om hen te beheren (UC-06).

De belangrijkste componenten zijn:

- **KPI-kaarten:** Ook hier worden kerncijfers getoond ('Total Users', 'Active Users', 'Inactive Users', 'Avg Licenses/User'), met duidelijke trends (+15 this month, 80% of total). Dit geeft de beheerder direct inzicht in de algehele gebruikerspopulatie.
- **Zoekbalk:** Een prominente zoekbalk ('Search users by name, email, or department...') stelt de beheerder in staat om snel een specifieke gebruiker te vinden.
- **Filteropties:** Een _dropdown_ voor 'All Status' (later uitgebreid naar 'Active'/'Inactive') en een 'Export'-knop (voor CSV-export, UC-08) zijn al aanwezig in dit wireframe.
- **Gebruikerstabel:** De centrale tabel toont alle gebruikers met de kolommen: User, Email, Department, Licenses, Last Active, Status. Dit zijn de essentiële velden voor gebruikersbeheer. De 'Status'-kolom gebruikt (zelfs in dit low-fidelity stadium) een visueel onderscheid ('Active' vs leeg) om de scanbaarheid te verbeteren.
- **Voorbeelddata:** De rijen met 'Jan Vermeulen' en 'Sophie De Vries' dienen als realistische voorbeelden om de functionaliteit te demonstreren.

Dit wireframe is in latere sessies verfijnd met uitgebreidere filters (op department) en een duidelijker visueel onderscheid voor de status.

Figuur 4 Low-fidelity wireframes Users

Figuur 4 toont het low-fidelity wireframe van het gebruikersbeheerscherm (Users), met zoek- en filterfunctionaliteit en een overzichtstabel van alle gebruikers.

**2.5 Evaluatie van de Eerste Ontwerpiteratie**

De presentatie van deze low-fidelity wireframes aan de stakeholders (Viktor Klein, Brian Veltman) leverde de volgende belangrijke inzichten op:

1.  **Validatie van de navigatiestructuur:** De sidebar als primair navigatiemiddel en de drill-down van organisatie → product → gebruiker werden als intuïtief en logisch beoordeeld.
2.  **Behoefte aan meer filters:** Met name in het gebruikersbeheerscherm (Figuur 4) werd de wens geuit om te kunnen filteren op afdeling (_department_) en een duidelijker onderscheid tussen actieve en inactieve statussen.
3.  **Productselectie:** Voor de productdetailpagina (Figuur 3) werd opgemerkt dat een expliciete productkiezer (_dropdown_) nodig was om de gebruikerservaring te verbeteren.
4.  **Datahiërarchie:** De focus op Business Units en organisaties werd bevestigd als de juiste centrale invalshoek voor het dashboard.

Deze feedback is meegenomen naar de tweede ontwerpiteratie, die in het volgende hoofdstuk wordt beschreven

Ontwerpmethodologie

Het ontwerpproces volgde een user-centered design (UCD) aanpak (Norman, 2013). Uitgangspunt waren de gebruikersvereisten zoals vastgelegd in de MoSCoW-analyse. Op basis van deze vereisten zijn eerst wireframes geschetst om de informatiearchitectuur en navigatiestructuur te bepalen. Vervolgens zijn deze wireframes in Figma uitgewerkt tot clickable prototypes, waarbij feedback van stakeholders (Viktor Klein, Brian Veltman) is verwerkt. De hier getoonde figuren zijn de resultaten van deze eerste ontwerpcyclus.

Hoofdstuk 3 gaat vervolgens in op de tweede ontwerpiteratie: high-fidelity mockups, waarin de focus verschuift naar visuele detaillering, het aanbrengen van de Equans-huisstijl en het optimaliseren van de gebruikersinterface op basis van de eerste evaluatieronde.

Overzicht van de Figma-werkruimte:

Figuur 5 Overzicht van de Figma-werkruimte

Figuur 5: Overzicht van de Figma-werkruimte

Deze figuur toont een screenshot van de Figma-werkruimte met de paginastructuur van het ontwerpproject.

Toelichting:

Figuur 5 geeft een overzicht van de ontwikkelomgeving in Figma. Links in het paneel zijn de verschillende pagina's te zien die zijn aangemaakt om de verschillende onderdelen van het dashboard te ontwerpen. Deze gestructureerde aanpak maakte het mogelijk om parallel te werken aan verschillende schermen en componenten, terwijl de consistentie behouden bleef. De aanwezigheid van zowel 'Flows' als individuele 'Frames' illustreert de gelaagde ontwerpaanpak: eerst werden de gebruikersstromen (flows) in kaart gebracht, waarna deze werden uitgewerkt tot concrete schermontwerpen (frames). Deze werkwijze conformeert aan het Hub and Spoke-model (Tidwell, 2010), waarbij de navigatie tussen pagina's centraal staat. De 'Preview'-functionaliteit in Figma werd gebruikt om de eerste klikbare prototypes te testen en te valideren met stakeholders, nog voordat er code was geschreven.

Inlogscherm met Microsoft SSO

Figuur 6 Inlogscherm met Microsoft SSO

Figuur 6: Inlogscherm met Microsoft SSO

Deze figuur toont het initiële ontwerp van het inlogscherm, waarbij authenticatie uitsluitend via Microsoft Single Sign-On (SSO) plaatsvindt.

Toelichting:

Figuur 6 presenteert het eerste ontwerp van het inlogscherm. In lijn met de beveiligingseisen (TM-01, SRS) en het besluit in ADR-004 is gekozen voor een exclusieve authenticatiemethode: Microsoft Azure Active Directory (Entra ID). Het scherm is bewust eenvoudig gehouden en bevat alleen de essentiële elementen: het Equans-logo, een duidelijke call-to-action ("Sign in with your Microsoft account") en een korte beveiligingsindicatie ("Secured by Microsoft Entra ID"). Er is géén optie voor een lokale login met gebruikersnaam en wachtwoord, wat het aanvalsoppervlak verkleint en de gebruikerservaring uniform houdt met andere Equans-applicaties. Deze eerste versie legde de nadruk op helderheid en veiligheid, zonder afleidende visuele elementen.

Navigatiestructuur en Organisatieoverzicht (eerste iteratie)

Figuur 7 Navigatiestructuur en Organisatieoverzicht (eerste iteratie)

Figuur 7: Navigatiestructuur en Organisatieoverzicht (eerste iteratie)

Deze figuur toont het eerste ontwerp van het hoofdscherm na inloggen, met de nadruk op de sidebar-navigatie en een eerste aanzet tot het organisatieoverzicht.

Toelichting:

Figuur 7 toont de eerste iteratie van de hoofdnavigatie. Centraal staat de sidebar aan de linkerzijde, die fungeert als de primaire navigatiehub. De sidebar bevat de hoofdcategorieën: 'Dashboard', 'Organizations', 'Users', 'Products' en 'Settings'. Dit ontwerp volgt het gangbare patroon voor applicaties met veel data, waarbij de gebruiker altijd toegang heeft tot de hoofdsecties. De rechterzijde van het scherm toont een eerste, nog summiere weergave van het organisatieoverzicht. In deze vroege fase lag de focus nog niet op datarijkdom, maar op het valideren van de algemene paginastructuur en de plaatsing van de sidebar. Deze versie diende als basis voor de meer uitgewerkte detailpagina's.

Uitgewerkt Organisatieoverzicht (Organization Overview)

Figuur 8 Uitgewerkt Organisatieoverzicht (Organization Overview)

Figuur 8: Uitgewerkt Organisatieoverzicht (Organization Overview)

Deze figuur toont het uitgewerkte ontwerp van het organisatieoverzicht, met KPI-kaarten, een kostentrendgrafiek, een doorzoekbare tabel en inzichten in kostendrijvers.

Toelichting:

Figuur 2 toont een significant verfijnde versie van het organisatieoverzicht. Dit scherm vormt de kern van de dashboardfunctionaliteit. Het ontwerp is opgebouwd uit vier logische secties:

1\. KPI-kaarten: Bovenaan worden vier kerncijfers getoond: 'Total Monthly Cost', 'Organizations', 'Total Users' en 'Avg Utilization'. De toevoeging van een trendindicator (↑ +5.2%) geeft direct inzicht in de ontwikkeling ten opzichte van de vorige maand.

2\. Kostentrendgrafiek: De 'Monthly Cost Trend by Business Unit' is een stacked area chart die de kostenontwikkeling over zes maanden visualiseert, uitgesplitst naar Business Unit. Dit biedt inzicht in seizoenseffecten en de relatieve bijdrage van elke eenheid aan de totale kosten. De keuze voor een stacked chart is conform de datavisualisatierichtlijnen (ADR-00X).

3\. Organisatietabel: De 'Organizations by Cost'-tabel is het belangrijkste interactieve element. De gebruiker kan zoeken op vrije tekst ('Search by Organization ID, name...') en door de genummerde pagina's bladeren. Elke rij bevat cruciale data: ID, naam, Business Unit, aantallen en kosten. De 'Utilization'-kolom is visueel gemaakt met gekleurde badges (groen bij >75%, oranje bij lager), wat de scanbaarheid verbetert.

4\. Inzichten: De panels 'Top Cost Drivers' en 'Utilization Leaders' bieden directe, geaggregeerde inzichten zonder dat de gebruiker zelf hoeft te rekenen. Dit zijn voorbeelden van data storytelling.

Dit ontwerp adresseert direct de Must Have-vereisten M-01 (geconsolideerd overzicht), M-04/M-05 (datacollectie) en M-11 (organisaties met personen). De tabel met een lange ID (21959ca7-236b-11j7-k470) toont aan dat het ontwerp rekening houdt met realistische data uit externe systemen, niet alleen met geanonimiseerde voorbeelden.

Productdetailpagina's (Product Details)

Figuur 9 Productdetailpagina's (Product Details)

Figuur 10 Productdetailpagina's License Comparison

Figuur 11 Productdetailpagina's Usage Distribution

Figuur 12 Productdetailpagina's (Product Details) Select Product

Figuur 4-7: Productdetailpagina's (Product Details)

Deze figuren tonen de evolutie van het ontwerp voor de productdetailpagina, waar per product (bijv. Jira) licentiedetails, gebruiksgegevens en kosten worden gevisualiseerd.

Toelichting:

De figuren 4 tot en met 7 (Figuur 4 PruductDetails.png t/m Figuur 7 PruductDetails4.png) illustreren het iteratieve ontwerpproces voor de productdetailpagina. Dit scherm geeft antwoord op vragen als "Hoe wordt Jira gebruikt binnen de organisatie?".

• Figuur 4 toont de eerste structuur. Dezelfde KPI-kaarten ('Total Licenses', 'Active Users', etc.) zorgen voor visuele consistentie met het organisatieoverzicht. Een dropdown menu ('Select Product') stelt de gebruiker in staat om tussen producten te wisselen. De 'License Comparison Across Products'-grafiek is een eerste poging om producten naast elkaar te zetten.

• Figuur 5 en 6 verfijnen deze grafiek en voegen een donut chart ('Jira Usage Distribution') toe. Dit visualiseert de verhouding tussen actieve en ongebruikte licenties. Cruciaal is de toevoeging van de metrische blokken 'Cost per Active License' en 'Potential Savings'. Dit adresseert direct de behoefte van Finance aan kosteninzicht (M-03) en geeft de beheerder een handelingsperspectief: ongebruikte licenties kunnen worden opgezegd.

• Figuur 7 toont een meer uitgewerkte tabel met gebruikers die een specifiek product (Jira) gebruiken. De kolommen 'Last Active' en 'Monthly Cost' zijn essentieel voor het identificeren van inactieve accounts en het doorbelasten van kos ten. Deze tabel is een directe voorloper van de uiteindelijke implementatie in React.

Deze ontwerpiteraties laten zien hoe feedback (bijv. "toon de besparingspotentie") is verwerkt om het dashboard van een passief rapportagemiddel naar een actief managementinstrument te transformeren.

Gebruikersdetailpagina (User Details)

Figuur 8: Gebruikersdetailpagina (User Details)

Deze figuur toont het ontwerp van de gebruikersdetailpagina, met persoonlijke informatie, een lijst van toegewezen licenties en een samenvatting van kosten.

Toelichting:

Figuur 8 (Figuur8 UserDetails.png) toont het ontwerp van de gebruikersdetailpagina, bereikbaar door op een gebruiker te klikken in een van de tabellen (bijv. in Figuur 7). Het scherm toont alle relevante informatie over één specifieke medewerker:

1\. Identificatie: Naam, e-mail, status ('Active User') en de datum van laatste activiteit.

2\. Actieknoppen: 'Edit User' en 'Manage Licenses' (voorbehouden aan beheerders).

3\. Toegewezen licenties: Een lijst van alle producten (Jira, GitHub, Copilot, Trello) die aan deze gebruiker zijn gekoppeld, inclusief de activeringsdatum. De 'Remove'-knop staat voor de beheerder klaar.

4\. Kostensamenvatting: Rechts worden de totalen getoond ('Total Licenses', 'Monthly Cost') en de accountstatus ('Full Access').

Dit ontwerp adresseert de noodzaak om van een overzicht naar detail te kunnen drill-downen, een cruciaal aspect van de gebruikerservaring. Het koppelt de Palantir-persoonsdata (naam, e-mail) aan de Atlassian-productdata (licenties), wat de kern vormt van het datamodel (US-14).

Gebruikersbeheer (User Management)

Figuur xx-:xx Gebruikersbeheer (User Management)

Deze figuren tonen de opeenvolgende ontwerpiteraties voor het gebruikersbeheerscherm, met zoek- en filterfunctionaliteit, een gebruikers-tabel en exportmogelijkheden.

Toelichting:

De figuren x, x en xx documenteren de ontwikkeling van het centrale gebruikersbeheerscherm. Dit scherm is ontworpen voor IT-beheerders en geeft een volledig overzicht van alle personen in het systeem.

• Figuur x toont de eerste opzet met KPI-kaarten bovenaan, een simpele zoekbalk en een eerste tabel. De 'Export'-knop is al aanwezig, wat het belang van data-export (UC-08) onderstreept.

• Figuur x verfijnt de filters. In plaats van één 'All Status'-knop, is er nu een toggle met 'All Status', 'Active' en 'Inactive'. Dit maakt het filteren op status een stuk intuïtiever.

• Figuur x is de meest uitgewerkte versie. De filteropties zijn verder uitgebreid met een dropdown voor 'Departments' (Engineering, Product Management, etc.). De tabel bevat nu ook de 'Last Active'-datum, essentieel voor het identificeren van inactieve accounts. De status wordt visueel gemaakt met gekleurde badges (blauw voor 'Active', grijs voor 'Inactive').

Deze iteratieve verfijning van zoek- en filterfunctionaliteit is een direct gevolg van de use cases UC-06 (Beheer Personen) en de wens om grote datasets (1600+ gebruikers) hanteerbaar te maken. De uiteindelijke implementatie in React heeft deze filters naar de backend verplaatst (server-side filtering en paginering), maar de interactie zoals in Figuur 11 ontworpen, is bepalend geweest voor de uiteindelijke gebruikerservaring.

**5.2 Conclusie van de Eerste Ontwerpiteratie**

De eerste ontwerpcyclus in Figma heeft een solide basis gelegd voor de uiteindelijke implementatie van het dashboard. De belangrijkste inzichten uit deze fase waren:

• De validatie van de informatiearchitectuur, met de sidebar als centrale hub en detailpagina's als spokes.

• Het belang van consistente KPI-kaarten op alle overzichtspagina's voor snelle scanbaarheid.

• De noodzaak van krachtige zoek- en filteropties (waaronder server-side paginering) om met grote datasets om te gaan.

• De waarde van het visualiseren van potentiële besparingen ('Potential Savings') om het dashboard van rapportage naar actie te laten leiden.

Deze ontwerpen zijn in de volgende fase van het project gebruikt als blauwdruk voor de ontwikkeling van de React-componenten, waarbij de functionele logica (zoals filteren) naar de backend is verplaatst. De tweede ontwerpiteratie, die in het volgende hoofdstuk wordt beschreven, richtte zich op het verfijnen van de visuele details en het strikt toepassen van de Equans-huisstijl.

Wil je dat ik ook een introductie voor het hele hoofdstuk schrijf, of de overgang naar de tweede ontwerpiteratie voorbereid?

Yes

Hier is een academische introductie voor het hele hoofdstuk en een vloeiende overgang naar de tweede ontwerpiteratie. Deze teksten sluiten naadloos aan op de zojuist beschreven figuren.

Hoofdstuk 5: Ontwerp van de Gebruikersinterface (Introductie)

Dit hoofdstuk beschrijft het ontwerpproces van de grafische gebruikersinterface (GUI) voor het Operational Insights Dashboard. Een gebruiksvriendelijke en intuïtieve interface is essentieel voor de acceptatie van het systeem door de beoogde gebruikersgroepen: licentiebeheerders, teammanagers, finance-medewerkers en IT-beheerders. Het ontwerp is niet louter een esth etische oefening, maar een cruciaal onderdeel van de functionele specificatie. Het vertaalt de gebruikersvereisten (hoofdstuk 4) naar een concrete interactie- en informatiearchitectuur.

Het ontwerpproces volgde een user-centered design (UCD) methodologie (Norman, 2013), met een iteratieve cyclus van ontwerpen, prototypen, evalueren en verfijnen. Uitgangspunt waren de functionele vereisten zoals vastgelegd in de MoSCoW-analyse. Op basis van deze vereisten zijn eerst wireframes geschetst om de informatiearchitectuur en navigatiestructuur te bepalen, conform het Hub and Spoke-model (Tidwell, 2010). Vervolgens zijn deze wireframes in Figma uitgewerkt tot clickable prototypes, waarbij feedback van stakeholders (Viktor Klein, Brian Veltman) in opeenvolgende sprints is verwerkt.

Dit hoofdstuk presenteert de resultaten van deze ontwerpcycli. Paragraaf 5.1 beschrijft de eerste ontwerpiteratie, waarin de nadruk lag op het valideren van de paginastructuur, de belangrijkste datavisualisaties en de algehele gebruikersreis. Deze vroege ontwerpen, hoewel nog niet volledig gedetailleerd of voorzien van de definitieve huisstijl, legden de functionele basis voor het dashboard. Paragraaf 5.2 gaat vervolgens in op de tweede ontwerpiteratie, waarin de focus verschuift naar visuele detaillering, het aanbrengen van de Equans-huisstijl en het optimaliseren van de gebruikersinterface op basis van de eerste evaluatieronde.

5.1 Eerste Ontwerpiteratie: Functionele Basis en Informatiearchitectuur

Deze paragraaf beschrijft de eerste ontwerpcyclus. De hier getoonde figuren zijn de initiële ontwerpen in Figma, die als basis hebben gediend voor de latere, meer verfijnde versies.

\*(Hier volgen de beschrijvingen van Figuren 1 t/m 11 zoals hierboven uitgewerkt)\*

5.2 Tweede Ontwerpiteratie: Visuele Verfijning en Implementatie van de Equans-huisstijl

Na de validatie van de functionele basis in de eerste ontwerpcyclus, verschoof de focus in de tweede iteratie naar de visuele detaillering en het strikt toepassen van de Equans-huisstijl. Deze fase was cruciaal om het dashboard van een functioneel wireframe te transformeren naar een professioneel product dat past binnen de bestaande Equans-softwareportfolio. De belangrijkste doelen van deze iteratie waren:

1\. Implementatie van de Equans-kleurpalet: Vervanging van de grijze en blauwe placeholder-kleuren door de officiële bedrijfskleuren (donkerblauw #002439, donkergroen #008163, turquoise #70BD95) en de bijbehorende accentkleuren (ADR-00X).

2\. Toepassing van typografische regels: Consistente toepassing van het Roboto-lettertype in de juiste gewichten (Light, Regular, Medium, Bold) voor titels, body text, labels en bijschriften, zoals vastgelegd in het typografisch systeem van Equans.

3\. Verfijning van datavisualisaties: Het consistent toepassen van de opaciteitsregels (100% voor primaire data, 60% voor secundaire data) in grafieken en tabellen om de visuele hiërarchie te versterken.

4\. Optimalisatie van componenten: Het herbruikbaar maken van ontwerpelementen zoals knoppen, kaarten en zoekbalken, in lijn met de principes van Design Systems.

Vergelijking van de Ontwerpevolutie (Eerste vs. Tweede Iteratie)

Figuur 12: Vergelijking van de Ontwerpevolutie (Eerste vs. Tweede Iteratie)

Deze figuur toont een side-by-side vergelijking van het organisatieoverzicht uit de eerste iteratie (links) en de tweede, visueel verfijnde iteratie (rechts).

Toelichting:

Figuur 12 illustreert de belangrijkste veranderingen tussen de twee ontwerpcycli. In de tweede iteratie (rechts) zijn de volgende verbeteringen zichtbaar:

•Kleurgebruik: De donkerblauwe (#002439) koptekst en de donkergroene (#008163) accenten voor KPI-kaarten zijn geïntroduceerd, waardoor het scherm een professionele en herkenbare Equans-uitstraling krijgt.

• Typografie: De titels zijn prominenter (Roboto Bold) en de labels op de kaarten zijn subtieler (Roboto Medium), wat de leesbaarheid en scanbaarheid ten goede komt.

• Datavisualisatie: De 'Monthly Cost Trend by Business Unit'-grafiek maakt nu gebruik van de Equans-accentkleuren (oranje, azuurblauw, violet) voor de verschillende Business Units, met een lagere opaciteit voor de achtergrondvlakken, zoals voorgeschreven in de richtlijnen.

• Consistentie: De KPI-kaarten, knoppen en de zoekbalk hebben nu een uniforme styling met afgeronde hoeken en subtiele schaduwen, wat bijdraagt aan een samenhangend geheel.

Definitief Ontwerp van de Gebruikersdetailpagina met Equans-styling

Figuur 13: Definitief Ontwerp van de Gebruikersdetailpagina met Equans-styling

Deze figuur toont de uiteindelijke, gestylde versie van de gebruikersdetailpagina.

Toelichting:

Figuur 13 toont de tweede iteratie van de gebruikersdetailpagina (eerder getoond in Figuur 8). De belangrijkste wijzigingen zijn:

• De informatie is nu beter gegroepeerd in twee kolommen: links de toegewezen licenties, rechts de financiële samenvatting.

• De licenties worden getoond in gekleurde kaarten, elk met een icoon dat de eerste letter van het product toont. Dit maakt de lijst visueel aantrekkelijker en sneller te scannen.

• De 'Total Licenses' en 'Monthly Cost' zijn prominenter weerge geven in grotere, vette cijfers, waardoor de kerngegevens eruit springen.

• De huisstijlkleuren (donkergroen voor de samenvattingskaart, azuurblauw voor de 'Add New License'-knop) zijn consistent doorgevoerd.

 

Conclusie

De twee ontwerpiteraties in Figma hebben geleid tot een robuust en visueel aantrekkelijk ontwerp voor het Operational Insights Dashboard. De eerste iteratie legde de nadruk op functionaliteit en gebruikersreis, terwijl de tweede iteratie het ontwerp verfijnde en in lijn bracht met de Equans-huisstijl.

Het resultaat is een ontwerp dat niet alleen intuïtief en functioneel is, maar ook naadloos aansluit bij de visuele identiteit van de organisatie, wat de gebruikersacceptatie en het professionele karakter van het eindproduct ten goede komt.

De belangrijkste ontwerpbeslissingen en hun onderbouwing zijn in dit document vastgelegd. Het hier beschreven ontwerp dient als blauwdruk voor de implementatiefase van het project en vormt een referentiepunt voor toekomstige uitbreidingen of wijzigingen aan de gebruikersinterface.

 

Bronnenlijst

\[1\] Norman, D. (2013). The Design of Everyday Things (2e ed.). Basic Books.

\[2\] Tidwell, J. (2010). Designing Interfaces: Patterns for Effective Interaction Design (2e ed.). O'Reilly Media.

\[3\] Garrett, J. J. (2011). The Elements of User Experience: User-Centered Design for the Web and Beyond (2e ed.). New Riders.

\[4\] Equans. (2025). Equans Corporate Style Guide: Kleur- en Typografierichtlijnen. Intern bedrijfsdocument.

\[5\] Klein, V., & Veltman, B. (2026). Persoonlijke communicatie over dashboardvereisten. Stakeholderinterviews, januari-februari 2026.

\[3\] Gothelf, J., & Seiden, J. (2016). _Lean UX: Designing Great Products with Agile Teams_ (2e ed.). O'Reilly Media.

\[4\] Krug, S. (2014). _Don't Make Me Think, Revisited: A Common Sense Approach to Web Usability_ (3e ed.). New Riders.