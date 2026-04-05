---
Projecttitel: Equans Operational Insights Dashboard
Studentnaam: Ahmad Alhaj Asaad
Opleiding: HBO-ICT Software Engineering
Organisatie: Equans / SLS-DP-DevOps-Forge
Documenttitel: Prototype ontwerp, Operational Insights Dashboard
Schoolbegeleider: Jeroen Boogaard
Bedrijfsbegeleiders: Viktor Klein (Business Owner), Brian Veltman (Technisch begeleider)
Studiejaar: 2025-2026
Versie: 1.0
---

# Mockups: Operational Insights Dashboard

## Inhoudsopgave

1. [Inleiding](#1-inleiding)
2. [Ontwerpmethodologie](#2-ontwerpmethodologie)
3. [Visuele identiteit: Equans huisstijl](#3-visuele-identiteit-equans-huisstijl)
   - 3.1 Kleurpalet
   - 3.2 Opaciteitsregels
   - 3.3 Toepassing in het dashboard
4. [Eerste ontwerpiteratie, low-fidelity wireframes](#4-eerste-ontwerpiteratie--low-fidelity-wireframes)
   - 4.1 Inlogscherm met Microsoft SSO
   - 4.2 Navigatiestructuur en Figma-werkruimte
   - 4.3 De hoofdnavigatie
   - 4.4 Organisatieoverzicht
   - 4.5 Productdetailpagina
   - 4.6 Gebruikersdetailpagina
   - 4.7 Gebruikersbeheer
   - 4.8 Organisatiedetailpagina (eerste ontwerpiteratie)
   - 4.9 Evaluatie van de eerste ontwerpiteratie
5. [Tweede ontwerpiteratie, high-fidelity mockups](#5-tweede-ontwerpiteratie--high-fidelity-mockups)
   - 5.1 Implementatie van de Equans-huisstijl
   - 5.2 Ontwerpiteraties overview
   - 5.3 Definitief ontwerp van de gebruikersdetailpagina
   - 5.4 Product details (tweede ontwerpiteratie)
   - 5.5 Data-importpagina
   - 5.6 Organisatiedetailpagina (tweede ontwerpiteratie)
6. [Conclusie](#6-conclusie)
7. [Bronnenlijst](#7-bronnenlijst)

---

## 1. Inleiding

Ik heb voor dit project de complete GUI van het Operational Insights Dashboard moeten bedenken en uittekenen. Klinkt misschien simpel, maar eerlijk gezegd was het nogal wat. Je hebt namelijk te maken met heel verschillende gebruikers. Licentiebeheerders kijken vooral waar licentiekosten weglekken, teammanagers willen per Business Unit kunnen sturen, finance wil trends over meerdere maanden zien, en IT-beheer zit eigenlijk alleen maar te wachten op een snelle manier om accounts en licenties te regelen. Probeer al die wensen maar eens in een dashboard te proppen zonder dat het een rommeltje wordt.

Voordat ik begon dacht ik eerlijk gezegd dat het lastigste stuk de datavisualisaties zouden zijn. Grafieken mooi krijgen, dat soort dingen. Maar toen ik met Viktor en Brian ging zitten voor de eerste feedbacksessie bleek dat helemaal niet het probleem. Het echte gevecht zat in de afweging: welke informatie moet er altijd staan, en wat stop je achter een extra klik? Die vraag kwam telkens weer terug. Ik heb uiteindelijk besloten om dit soort keuzes niet zelf in te vullen maar steeds voor te leggen aan de stakeholders. Beter dat zij zeggen "nee, dat moet juist altijd zichtbaar zijn" dan dat ik daar achteraf achter kom.

In dit document loop ik door hoe al die ontwerpkeuzes uiteindelijk tot stand zijn gekomen. Eerst ga ik in op de werkmethode (hoofdstuk 2), dan behandel ik de Equans-huisstijl en hoe die in het dashboard is verwerkt (hoofdstuk 3). De rest van het document gaat over de twee ontwerprondes: de ruwe wireframes en daarna de nettere high-fidelity versies met alle feedback en wijzigingen die daarbij hoorden.

---

## 2. Ontwerpmethodologie

Mijn aanpak was een mix van user-centered design (Norman, 2013) en Lean UX (Gothelf & Seiden, 2016). Concreet betekende dat: iets tekenen, voorleggen, aanpassen, opnieuw voorleggen. En dat steeds opnieuw. Die manier van werken was eigenlijk noodzaak, want de eisen waren aan het begin nog lang niet afgerond. Stakeholders wisten zelf soms ook nog niet precies wat ze wilden. Door die korte feedbackrondes werd dat gaandeweg steeds scherper.

Ik ben gestart vanuit de MoSCoW-eisen in het Functioneel Ontwerp. Daarmee had ik een basis, maar ik ben bewust niet meteen met nette schermen aan de slag gegaan. Eerst simpele wireframes. Waarom? Nou, omdat ik al eerder had meegemaakt dat als iets er te "af" uitziet, mensen gaan zeuren over kleurtjes en iconen. Terwijl ik op dat moment wilde weten: klopt de indeling? Staan de juiste dingen op de juiste plek? Die aanpak bleek te werken, want de feedback in die eerste ronde ging echt over structuur. Dingen als "ik mis hier een filter" of "die drill-down voelt logisch", dat soort reacties.

Het navigatiemodel is het Hub and Spoke-model van Tidwell (2010) geworden. Het organisatieoverzicht fungeert als hub, en van daaruit navigeer je naar detailpagina's. Ik had ook even gekeken of een lineaire flow (stap 1, stap 2, stap 3) iets zou zijn, maar dat paste gewoon niet bij hoe beheerders echt werken. Die switchen constant heen en weer, van overzicht naar een specifiek account en dan weer terug naar de grote lijnen. Een starre volgorde zou daarbij alleen maar in de weg zitten.

Nog iets dat me opviel: sommige ontwerpkeuzes hadden direct gevolgen voor de techniek. Neem het gebruikersbeheerscherm. Toen ik daar de zoek- en filterfunctionaliteit concreet begon uit te tekenen, was al snel duidelijk dat dit niet client-side kon. Equans heeft duizenden gebruikers. Die data in de browser filteren gaat niet vliegen. Dus server-side paginering en filtering is al vroeg als harde eis meegenomen bij het development-team. Daar heb ik mijn ontwerp ook op aangepast.

Kort gezegd: de methode was er niet alleen voor mooie plaatjes. Vooral om risico's snel boven tafel te krijgen en keuzes te onderbouwen met echte feedback van mensen die het straks gaan gebruiken.

---

## 3. Visuele identiteit: Equans huisstijl

Pas nadat de functionele opzet redelijk stond ben ik met de visuele kant begonnen. Ik had de huisstijlrichtlijnen van Equans erbij gepakt (Equans, z.d.), maar ik heb die niet blindelings overgenomen. Op papier zien die richtlijnen er prima uit, maar zodra je ze loslaat op een dashboard vol tientallen rijen data en drie grafieken naast elkaar, merk je dat je toch eigen afwegingen moet maken. Leesbaarheid wint het uiteindelijk van strikt de brandguide volgen.

### 3.1 Kleurpalet

Het uitgangspunt was het Equans-palet: donkerblauw (#002439), donkergroen (#008163), turquoise (#70BD95) en wit. Daarnaast had ik nog wat extra kleuren nodig voor grafieken en signalering, dus daar kwamen oranje, azuurblauw, violet en lichtblauw bij.

Waar ik vrij snel tegenaan liep: ik had overal accentkleuren gestopt. Tabellen, grafieken, knoppen, overal een andere kleur. Leek levendig in mijn hoofd, maar toen Viktor en ik ernaar zaten te kijken zei hij: "het is best druk." En hij had gelijk. Je ogen weten niet waar ze heen moeten. Vanaf dat moment heb ik mezelf de regel opgelegd om per pagina maar één accentkleur dominant te gebruiken. Extra kleuren alleen als ze echt iets toevoegen aan het begrip. Dat scheelde enorm.

### 3.2 Opaciteitsregels

Iets waar ik zelf niet zo over had nagedacht maar wat achteraf een flink verschil maakte: opaciteitsniveaus. Ik werk nu met drie levels: 100% voor de hoofdinformatie, 60% voor bijzaken, en 20% voor achtergrondelementen.

Hoe ik daarop kwam: in de eerste versie van de grafieken stonden de achtergrondvlakken op volle sterkte. De daadwerkelijke trendlijn verdronk daardoor in alle kleur eromheen. Na een paar keer aanpassen raakte ik op de goede verhouding. Het grappige is dat het echt maar een paar procentjes verschil is, maar gebruikers vinden de hoofdtrend nu veel sneller. Ze vergelijken reeksen ook nauwkeuriger met elkaar.

### 3.3 Toepassing in het dashboard

Door het kleurgebruik en de opaciteitsniveaus samen toe te passen zijn er in de praktijk drie dingen bereikt. Ten eerste herken je het dashboard meteen als Equans-product. Ten tweede blijven de pagina's ook bij veel data rustig ogen. En ten derde, en dat was eigenlijk het hoofddoel, springt de info die ertoe doet er direct uit. Binnen dit project was dat doorslaggevend want het dashboard moet een stuurinstrument zijn, geen rapport dat je op je gemak doorneemt.

Nog een bijkomend voordeel dat ik niet had verwacht: doordat ik de kleuren en de componentstijl al vroeg had vastgelegd, was het voor het vertalen naar React veel makkelijker. De ontwikkelaar kon dezelfde componenten over pagina's heen hergebruiken zonder dat er visuele inconsistenties ontstonden. Scheelde in de bouwfase flink wat heen-en-weer met correcties.

---

## 4. Eerste ontwerpiteratie, low-fidelity wireframes

Hier ga ik in op de eerste ronde wireframes. Die waren met opzet heel simpel gehouden. Eigenlijk saai. In het begin was mijn eerste impuls om er meteen kleuren en nette componenten op te gooien, maar dat heb ik vrij snel losgelaten. De reden is simpel: zodra iets er "af" uitziet, gaan stakeholders reageren op stijl. "Kan die knop niet blauwer?" "Dat icoon is lelijk." Terwijl ik op dat moment wilde horen: begrijp je waar je moet klikken? Staat de info waar je hem verwacht? Alle wireframes heb ik in Figma gemaakt en die heb ik samen met Viktor Klein en Brian Veltman doorgelopen.

### 4.1 Inlogscherm met Microsoft SSO

_Figuur 1 — Low-fidelity wireframe van het inlogscherm, met Microsoft SSO als enige authenticatiemethode._

Het inlogscherm was qua ontwerp het simpelste scherm. Er hoefde niet zoveel op. Maar qua consequenties was het wel een van de meest impactvolle beslissingen. TM-01 uit de SRS en ADR-004 schreven al voor dat authenticatie uitsluitend via Microsoft Azure Active Directory (Entra ID) moest gaan. Geen eigen accountsysteem dus, geen wachtwoord-reset, geen lokale database met credentials.

Het scherm bevat daardoor niet meer dan het Equans-logo, de titel "Operational Insights", een "Sign in with Microsoft"-knop, een beveiligingsmelding en onderaan contactgegevens. Klaar. De gebruiker heeft hier precies één ding te doen en dat is inloggen.

Waarom heb ik lokale login er bewust niet in gestopt? Allereerst hoef je dan geen wachtwoorden op te slaan, en dat scheelt een groot beveiligingsrisico. Verder kennen Equans-medewerkers die Microsoft-login al, dus er is geen extra leercurve. En als iemand uit dienst gaat wordt het account in Entra ID uitgezet, waardoor de toegang tot het dashboard automatisch vervalt. Wat ik me nog herinner is dat een paar stakeholders in het begin vroegen om een soort noodlogin, voor het geval Microsoft er even uitligt. Maar na overleg hebben we besloten dat de beveiligingsrisico's daarvan niet opwegen tegen het gemak.

### 4.2 Navigatiestructuur en Figma-werkruimte

_Figuur 2 — Overzicht van de Figma-werkruimte met de paginastructuur van het ontwerpproject._

De manier waarop ik Figma had ingericht hielp me uiteindelijk meer dan verwacht. Ik had alles opgedeeld in Flows en Frames. In de Flows tekende ik de navigatiepaden uit (hoe gaat iemand van A naar B?), en de Frames waren de losse schermen zelf. Door die scheiding dwong ik mezelf om niet meteen visueel te denken maar eerst na te gaan hoe de gebruiker door het systeem beweegt.

Een bijeffect dat ik niet had voorzien was dat het reviewgesprekken ook wat makkelijker maakte. Ik kon zeggen: "we kijken nu alleen naar de flow, niet naar hoe het eruitziet." Dan bleven we niet hangen in discussies over lettertypes terwijl de structuur nog niet klopte. De klikbare preview in Figma was daarbij trouwens erg handig. Ik kon mensen echt laten doorklikken terwijl er in React nog helemaal niets gebouwd was.

### 4.3 De hoofdnavigatie

_Figuur 3 — Eerste iteratie van de navigatiestructuur, met de sidebar als primaire navigatiehub en het organisatieoverzicht als centraal startscherm._

Ik heb gekozen voor een vaste sidebar aan de linkerkant: Dashboard, Organizations, Users, Products en Data Import. Dat is een vrij standaard patroon in beheer-applicaties en dat is niet voor niks. Beheerders willen snel kunnen wisselen tussen secties, zonder telkens helemaal terug te moeten navigeren.

Eerlijk gezegd twijfelde ik even tussen een navigatiebalk bovenaan of een sidebar. Een topnavigatie oogt wat lichter. Maar met vijf secties en de verwachting dat er later nog pagina's bijkomen werd het bovenaan al snel te krap, zeker op een wat kleiner scherm. De sidebar loste dat probleem op: het hoofdgebied in het midden blijft ruim en als er uiteindelijk een zesde of zevende sectie bijkomt past dat er gewoon in.

### 4.4 Organisatieoverzicht

_Figuur 4 — Low-fidelity wireframe van het organisatieoverzicht (Organization Overview), met KPI-kaarten, een kostentrendgrafiek en een doorzoekbare organisatietabel._

Na het inloggen kom je op het organisatieoverzicht terecht. Dat is bewust het startscherm geworden. Ik heb het in vier blokken opgebouwd. Helemaal bovenaan vier KPI-kaarten: Total Monthly Cost, Organizations, Total Users en Avg Utilization. De gedachte erachter is het inverted pyramid-principe van Krug (z.d.), de kern eerst en dan pas details. Tijdens de wireframefase waren het nog lege placeholders, want de echte data moest nog uit de backend komen die op dat moment niet af was.

Daaronder heb ik een trendgrafiek geplaatst, "Monthly Cost Trend by Business Unit". In de wireframe was die nog heel schematisch, maar het idee was al duidelijk: je wilt trends over tijd kunnen zien zonder ergens doorheen te hoeven klikken. De tabel eronder ("Organizations by Cost") werd het voornaamste interactieve element. De kolommen Org ID, Org Name, Business Unit, Licenses, Users, Cost en Utilization kwamen direct uit requirements M-01 en M-11.

Wat ik leerde van de stakeholdersessies: lege voorbeeldrijen in een wireframe leveren vage feedback op. Op het moment dat ik namen als Energy Solutions en Smart Buildings invulde kwamen er veel concretere opmerkingen. Iemand zei letterlijk: "oh, maar dan zou ik hier willen sorteren op Cost." Dat soort reacties krijg je niet met lege vakjes.

### 4.5 Productdetailpagina

_Figuur 5 — Low-fidelity wireframe van de productdetailpagina (Product Details), met licentieverdeling, gebruiksstatistieken en kostentrendvisualisatie per product._

Het hele punt van deze pagina is eigenlijk één vraag beantwoorden: gebruiken we een product als Jira daadwerkelijk, of gooien we geld weg aan licenties die niemand aanraakt? Ik heb er daarom vier vaste blokken opgezet: KPI-kaarten, een licentieverdeling, topgebruikers en een trendgrafiek met kosten en gebruikersaantallen naast elkaar.

Ik hield de opbouw bewust vergelijkbaar met het organisatieoverzicht, zodat gebruikers niet élke pagina opnieuw hoeven te leren. Maar hier heb ik zelf een fout gemaakt in de eerste wireframe. Ik had geen productselector neergezet. Dat viel in de review meteen op. Zonder een dropdown is het niet duidelijk hoe je van Jira naar Confluence wisselt. Een beetje gênant, maar juist daarom zijn die vroege wireframerondes zo nuttig. Beter nu dan na het bouwen.

### 4.6 Gebruikersdetailpagina

_Figuur 6 — Low-fidelity wireframe van de gebruikersdetailpagina (User Details), met persoonlijke informatie, toegewezen licenties en een kostensamenvatting._

Via het gebruikersoverzicht kun je doorklikken naar een individueel persoon. Op die pagina staan drie blokken: identificatiegegevens, de licenties die zijn toegewezen, en een kostenoverzicht. Ziet er vrij rechttoe rechtaan uit, maar onder de motorkap zit wel een lastige koppeling. De persoonsgegevens komen namelijk uit Palantir, terwijl de licentie-informatie uit Atlassian komt (US-14). Twee compleet verschillende systemen.

Ik had even overwogen om gewoon alles in één grote tabel te dumpen. Maar dat leest niet lekker. Te veel informatie door elkaar en je moet dan als beheerder zelf ontcijferen wat bij wat hoort. Dus heb ik het gesplitst in duidelijke secties. Die keuze had ook gevolgen voor de backend: de API-responses moesten niet zomaar een platte lijst teruggeven maar gestructureerd zijn naar deze logische groepen.

### 4.7 Gebruikersbeheer

_Figuur 7 — Low-fidelity wireframes van het gebruikersbeheerscherm (User Management), met zoek- en filterfunctionaliteit, KPI-kaarten en een overzichtstabel van alle gebruikers._

Het gebruikersbeheer is het scherm voor dagelijks werk (UC-06). Bovenaan staan KPI-kaarten, daaronder een zoekbalk en filters, en vervolgens de tabel met alle gebruikersdata. CSV-export (UC-08) zat er vanaf versie één al in, want die vraag kwam werkelijk in elk gesprek met stakeholders naar voren. Elke keer weer.

De filters waren een dingetje. Eerst had ik alleen een simpele statusfilter. Te weinig, bleek in reviews. Dus heb ik er een active/inactive-toggle aan gehangen, en later ook filtering op afdeling. De kolom Last Active is er ook bijgekomen na feedback, want die heb je nodig om te onderbouwen waarom je een licentie zou moeten intrekken. Zonder dat veld heb je simpelweg geen basis voor zo'n beslissing.

En dan die technische consequentie waar ik het eerder al over had: Equans zit op datasets van 80.000+ records. Dat in de browser filteren is gewoon onmogelijk, je browser loopt vast. Dus toen deze filtereisen concreet werden, hebben we meteen server-side filtering en paginering in de Rust/Actix-web backend als harde eis neergezet. Echt een goed voorbeeld van hoe een UX-beslissing direct doorwerkt in de architectuur.

### 4.8 Organisatiedetailpagina (eerste ontwerpiteratie)

_Figuur 8 toont het initiële ontwerp van de organisatiedetailpagina, waar de gebruiker dieper kan inzoomen op een specifieke organisatie-eenheid._

In deze fase was de pagina nog helemaal ongestyled, maar functioneel eigenlijk al compleet. Breadcrumb bovenaan, dan de organisatie-identificatie, KPI-kaarten, een kostentrendgrafiek, de product/licentietabel en een zoekveld.

Ik had de producttabel eerst vrij laag op de pagina staan. Slechte keuze bleek achteraf. Tijdens de review scrolden Viktor en Brian er voorbij voordat ze bij het nuttigste onderdeel waren. Ik heb de tabel daarna naar boven verplaatst en er een duidelijke hint bij gezet: "Click any product to view user-level details." Werkte meteen beter, mensen snapten nu direct dat ze konden doorklikken.

De gedachte achter de opbouw was om UC-07 en UC-06 samen te ondersteunen. Je begint op organisatieniveau, gaat gecontroleerd door naar product en dan naar persoon. Zo blijft de informatiehiërarchie overeind.

### 4.9 Evaluatie van de eerste ontwerpiteratie

Na de reviews met Viktor en Brian had ik genoeg input om mee verder te gaan. De sidebar en het drill-down principe vonden ze logisch, dus dat bevestigde dat de basisnavigatie op de goede weg zat. Maar op de filters kwam flinke feedback: meer onderscheid in statussen, extra filteropties, dat soort dingen. De ontbrekende productselector op de productpagina werd ook meteen benoemd. Wat wel goed was: de datahiërarchie (Business Unit naar organisatie, naar product, naar gebruiker) voelde voor hen als een logische volgorde.

Achteraf gezien deed die eerste iteratie precies waarvoor hij bedoeld was. Fouten naar boven halen. Bevestigen wat goed zat. En een stevige basis leggen voordat je gaat polijsten.

---

## 5. Tweede ontwerpiteratie, high-fidelity mockups

Oké, na de functionele check was het tijd om er een echt Equans-dashboard van te maken. De vraag verschoof van "klopt de informatie op de pagina?" naar "voelt dit als één product en helpt het de gebruiker echt bij wat hij moet doen?"

### 5.1 Implementatie van de Equans-huisstijl

De huisstijl heb ik uitgewerkt op vier vlakken: kleur, typografie, grafiekstijl en componentgedrag. De kleuren zijn donkerblauw (#002439), donkergroen (#008163) en turquoise (#70BD95). Accentkleuren kwamen alleen terug op plekken waar ze echt functioneel nut hadden, zoals in grafieken waar je reeksen moet onderscheiden.

Qua typografie is het Roboto geworden, met per informatielaag een ander gewicht. Gaf eigenlijk meer rust dan ik van tevoren dacht, vooral in die tabellen. Labels en waarden zijn nu visueel in een oogopslag uit elkaar te halen.

Bij de grafieken gebruik ik die opaciteitsregel die ik eerder noemde (100%, 60%). Klinkt als een klein detail maar maakt de grafieken echt beter scanbaar. En dan heb ik in Figma ook nog knoppen, kaarten en zoekbalken als herbruikbare componenten aangemaakt. Best een tijdsinvestering op dat moment, maar later kon ik ze gewoon slepen naar nieuwe pagina's. Scheelde uiteindelijk heel veel gedoe.

### 5.2 Ontwerpiteraties overview

_Figuur 9 — De tweede iteratie van het organisatieoverzicht (Organization Overview) met KPI-kaarten._

Als je deze versie naast de wireframes legt zie je het verschil meteen. Topbalk, KPI-kaarten, grafieken, alles volgt nu dezelfde visuele taal. En dat "volgen van patronen" is niet alleen esthetiek. Ik merkte tijdens latere demo's dat gebruikers sneller snapten hoe een pagina werkte wanneer de opbouw herkenbaar was van een vorige pagina. Zeker voor beheerders die dagelijks meerdere schermen langsgaan maakt dat verschil.

### 5.3 Definitief ontwerp van de gebruikersdetailpagina

_Figuur 10 — Definitief high-fidelity ontwerp van de gebruikersdetailpagina met volledige Equans-huisstijl._

Het grootste verschil met de wireframe: ik heb de layout aangepast naar twee kolommen. Links de licenties, rechts het financiële blok. In de low-fidelity versie stond alles onder elkaar en dan moest je eindeloos scrollen. Nu zie je de kern in een oogopslag.

De licenties staan nu in kaarten met producticonen erbij. Klinkt als opsmuk, maar beheerders vinden het product dat ze zoeken er echt veel sneller mee. En de getallen Total Licenses en Monthly Cost heb ik expres groter gemaakt. Gesprekken met gebruikers wezen uit dat ze altijd eerst naar die twee cijfers kijken, dus die moeten er ook als eerste uitspringen.

### 5.4 Product details (tweede ontwerpiteratie)

_Figuur 11 toont het definitieve, high-fidelity ontwerp van de productdetailpagina, voorzien van de volledige Equans-huisstijl._

Hier heb ik de productpagina vooral aangescherpt op leesbaarheid. De KPI's zijn beter geclusterd en de grafieken hebben meer contrast gekregen. Wat me dwars zat in de eerdere versie: Cost per Active License en Potential Savings vielen te weinig op, terwijl dat juist de cijfers zijn waar je iets mee doet. Ik heb ze uiteindelijk dichter bij de kerngrafieken geplaatst zodat je meteen de link legt tussen gebruik en kosten.

Het resultaat is dat de pagina niet alleen laat zien hoe het staat, maar ook aanzet om te handelen. Dat sluit direct aan op UC-04.

### 5.5 Data-importpagina

_Figuur 12 toont het definitieve, high-fidelity ontwerp van de data-importpagina, voorzien van de volledige Equans-huisstijl._

De importpagina hoort bij UC-03 en volgt een wizard-achtige opzet (Tidwell, 2010). Centraal staat een drag-and-drop vlak waar je het bestand naartoe sleept. Daaronder staan meteen de bestandseisen: welke kolommen verplicht zijn, wat het formaat moet zijn. Denk aan velden als person_id, person_email, person_first_name en person_last_name.

Die eisen staan er niet zomaar. Van Brian heb ik gehoord dat in eerdere trajecten juist bij het importeren van data de meeste fouten ontstonden. Mensen leverden bestanden aan met verkeerde kolomnamen of ontbrekende velden, en dan kreeg je onduidelijke foutmeldingen. Door de verwachtingen vooraf op het scherm te zetten voorkom je dat grotendeels. Minder mislukte imports, minder supportvragen.

### 5.6 Organisatiedetailpagina (tweede ontwerpiteratie)

_Figuur 13 toont het definitieve, high-fidelity ontwerp van de organisatiedetailpagina, voorzien van de volledige Equans-huisstijl._

Ten opzichte van Figuur 8 is deze versie wat compacter. De KPI's hebben nu trendindicatoren: niet alleen het getal maar ook een pijltje omhoog of omlaag, zodat je de richting meteen ziet. De Cost Trend grafiek combineert kosten en actieve gebruikers in één beeld.

Ik was daar eigenlijk best huiverig voor, twee datasets in één grafiek kan zo een rommeltje worden. Maar met het goede kleurcontrast (donkerblauwe lijn voor kosten, turquoise voor actieve gebruikers) geeft het juist extra context. Je ziet in één keer of kosten stijgen terwijl het gebruik daalt. En dat is precies de info die je nodig hebt om iets te doen aan licentie-optimalisatie (UC-04).

---

## 6. Conclusie

Terugkijkend op het hele traject denk ik dat de grootste meerwaarde zat in het vroeg toetsen. Niet alleen op uiterlijk, maar ook op technische haalbaarheid. De sidebar met drill-down navigatie vonden de stakeholders werkbaar. Dat gaf een stevig fundament. En het principe om op elke pagina met KPI-kaarten te beginnen bleek echt te helpen: gebruikers vinden sneller wat ze zoeken en hoeven niet eindeloos te klikken.

Wat mij daarnaast opviel is hoe direct UX-keuzes doorwerken in de techniek. Het beheerders-scherm met grote datasets pusht je bijna automatisch naar server-side oplossingen. Daar kun je als ontwerper niet omheen.

Misschien wel het meest waardevolle inzicht: een dashboard is pas echt nuttig als het niet alleen toont maar ook aanzet tot actie. Indicatoren als Potential Savings en Utilization Rate maken dat concreet. Het verschuift van "hier is je data" naar "dit is wat je ermee zou moeten doen."

Het ontwerp is vervolgens als directe basis gebruikt voor de React-componenten en de API-endpoints in de backend. Mochten er in de toekomst onderdelen als notificaties of prognosemodules bijkomen, dan kan dat op de bestaande structuur worden aangesloten. Alles omgooien is dan niet nodig.

---

## 7. Bronnenlijst

[1] Norman, D. (z.d.). _The Design of Everyday Things._ Goodreads. https://www.goodreads.com/book/show/840.The_Design_of_Everyday_Things

[2] Equans. (z.d.). _Equans Corporate Style Guide: Kleur- en typografierichtlijnen._ https://equans.sharepoint.com/sites/nl-afd-comm/Gedeelde%20documenten/Forms/AllItems.aspx?id=%2Fsites%2Fnl%2Dafd%2Dcomm%2FGedeelde%20documenten%2FHuisstijl%202024%2FBrandguide%5FEquansNL%5F2023%5F081223%20%28002%29%2Epdf&parent=%2Fsites%2Fnl%2Dafd%2Dcomm%2FGedeelde%20documenten%2FHuisstijl%202024

[3] Garrett, J. J. (2010, 16 december). _The Elements of User Experience, Second Edition: User-Centered Design for the Web and Beyond._ O'Reilly Online Learning. https://learning.oreilly.com/library/view/the-elements-of/9780321688651/

[4] Gothelf, J., & Seiden, J. (2016, 10 oktober). _Lean UX, 2nd Edition._ O'Reilly Online Learning. https://learning.oreilly.com/library/view/lean-ux-2nd/9781491953594/

[5] Klein, V., & Veltman, B. (2026). _Persoonlijke communicatie over dashboardvereisten_ [Stakeholderinterviews, 2025 - 2026]. Equans / SLS-DP-DevOps-Forge.

[6] Krug's, S. (z.d.). _Don't make me think, revisited: a common sense approach._ Goodreads. https://www.goodreads.com/book/show/18197267-don-t-make-me-think-revisited

[7] Tidwell, J. (2010, 30 december). _Designing Interfaces, 2nd Edition._ O'Reilly Online Learning. https://learning.oreilly.com/library/view/designing-interfaces-2nd/9781449379711/
