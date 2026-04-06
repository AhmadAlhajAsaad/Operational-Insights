# Professionele vaardigheden reflectie

## Equans Operational Insights Dashboard

---

|                  |                                                                             |
| ---------------- | --------------------------------------------------------------------------- |
| **Studentnaam**  | Ahmad Alhaj Asaad (1035912)                                                 |
| **Opleiding**    | HBO-ICT, Informatica - Hogeschool Rotterdam                                 |
| **Projecttitel** | Equans Operational Insights Dashboard                                       |
| **Organisatie**  | Equans Nederland - SLS Digital Platforms (DevOps Forge)                     |
| **Begeleiders**  | Viktor Klein (bedrijf), Brian Veltman (technisch), Jeroen Boogaard (school) |
| **Datum**        | 27 maart 2026                                                               |
| **Versie**       | 1.0                                                                         |

---

## Inhoudsopgave

- [Inleiding](#inleiding)
- [1. Stakeholdercommunicatie, rolinvulling en presenteren](#1-stakeholdercommunicatie-rolinvulling-en-presenteren)
  - [1.1 Stakeholdercommunicatie](#11-stakeholdercommunicatie)
  - [1.2 Rolinvulling](#12-rolinvulling)
  - [1.3 Presenteren](#13-presenteren)
  - [1.4 Ontwikkeling](#14-ontwikkeling)
- [2. Feedback verwerken, testen en iteratief verbeteren](#2-feedback-verwerken-testen-en-iteratief-verbeteren)
  - [2.1 Feedback ontvangen](#21-feedback-ontvangen)
  - [2.2 Feedback verwerken](#22-feedback-verwerken)
  - [2.3 Testen als feedbackmechanisme](#23-testen-als-feedbackmechanisme)
  - [2.4 Iteratief verbeteren](#24-iteratief-verbeteren)
- [3. Reflectie op professioneel handelen](#3-reflectie-op-professioneel-handelen)
  - [3.1 Zelfstandigheid](#31-zelfstandigheid)
  - [3.2 Planning en discipline](#32-planning-en-discipline)
  - [3.3 Afwegingen maken](#33-afwegingen-maken)
  - [3.4 Kwaliteitsbewustzijn](#34-kwaliteitsbewustzijn)
- [Conclusie](#conclusie)

---

## Inleiding

Binnen dit project heb ik het Equans Operational Insights Dashboard gebouwd. Equans had op dat moment geen centraal overzicht van wie welke softwarelicentie gebruikte bij vendors als Atlassian Cloud Enterprise en GitHub Enterprise Cloud, wat dat kostte, en of er licenties ongebruikt bleven. Dat probleem moest opgelost worden. Technisch gezien draait het systeem op een Rust-backend met Axum, een React 19 frontend in TypeScript, en PostgreSQL als database. Het hele traject duurde 20 weken, opgedeeld in 8 sprints, bij het DevOps Forge-team van Equans SLS Digital Platforms.

Wat me vrij snel opviel toen ik begon: als enige ontwikkelaar op een project als dit gaat het niet alleen om code schrijven. Je moet ook praten met mensen van IT en DevOps die allemaal andere verwachtingen hebben, feedback vragen en daar iets mee doen, demo's geven, en je eigen planning bewaken. Dat soort dingen leer je niet uit een boek. Daar loop je tegenaan en dan merk je vanzelf waar je in moet groeien.

Vooraf had ik een aantal leerdoelen bedacht. Ten eerste wilde ik leren om technische informatie zo uit te leggen dat ook niet-technische mensen ermee uit de voeten konden. Daarnaast wilde ik mezelf aanleren om actief feedback te vragen bij begeleiders en gebruikers, en die feedback ook echt te verwerken in het product (niet alleen aanhoren en vergeten). Ook wilde ik het hele project zelfstandig plannen en uitvoeren binnen een Scrum-werkwijze. Tot slot wilde ik laten zien dat ik kwaliteitsbewust kon werken en onderbouwde keuzes kon maken binnen een echte bedrijfsomgeving.

In dit rapport beschrijf ik per gebied hoe dat is gegaan. En eerlijk gezegd: sommige dingen gingen beter dan verwacht, en andere dingen bleken lastiger dan ik had ingeschat.

---

## 1. Stakeholdercommunicatie, rolinvulling en presenteren

### 1.1 Stakeholdercommunicatie

Gedurende het project heb ik met heel verschillende mensen samengewerkt, en ik merkte al snel dat je niet tegen iedereen op dezelfde manier praat.

Viktor Klein was als Product Owner mijn eerste aanspreekpunt voor de functionele kant. Met hem besprak ik welke features het dashboard moest krijgen, welke KPI's voorrang hadden, en of wat ik opleverde ook echt was wat hij bedoelde. Viktor is iemand die resultaten wil zien, geen verhalen horen. Dat merkte ik in de eerste sprint review al. Ik was bezig met een uitleg over hoe mijn Rust-code de Atlassian API aanriep, en halverwege vroeg hij: "Maar wat ziet de gebruiker nu?" Vanaf dat moment leerde ik om gewoon mijn scherm te delen en te laten zien wat er werkte, in plaats van er omheen te praten.

Brian Veltman was mijn technisch begeleider, en met hem ging het over architectuur, codestructuur, foutafhandeling. Brian is een ervaren developer en geeft behoorlijk directe feedback. In het begin schrok ik daar weleens van. Hij wees me er bijvoorbeeld op dat ik businesslogica in mijn Axum route-handlers had staan, terwijl die in een aparte service-laag thuishoorde. Op dat moment dacht ik: ja maar het werkt toch? Achteraf had hij gelijk. Die directheid heb ik leren waarderen, want het bespaarde me uiteindelijk tijd en leverde schonere code op.

Het DevOps Forge-team bestond uit collega's uit meerdere landen, onder andere Frankrijk en India. Daardoor was de voertaal tijdens de ochtendvergaderingen Engels. Voor mij was dat een extra uitdaging, maar eigenlijk vooral een kans. Ik moest niet alleen mijn voortgang samenvatten en aangeven waar ik vastliep, maar dat ook in het Engels doen. Met Viktor en de Nederlandse teamleden schakelde ik over naar het Nederlands, wat hielp om mijn Nederlands op peil te houden. Die dagelijkse wissel tussen twee talen (soms drie keer op een ochtend) heeft mijn taalvaardigheid in allebei de talen verbeterd, zowel qua vaktermen als qua professionele communicatie. In het begin was ik overigens terughoudend om in de stand-ups te zeggen dat ik ergens muurvast zat. Ik wilde niet overkomen als iemand die het niet zelf kon. Na een paar keer onnodig een halve dag zoeken naar iets waar een collega direct een oplossing voor had, liet ik dat idee los. Problemen benoemen gaat sneller dan problemen verbergen.

Wat me gaandeweg opviel is dat de manier van communiceren per persoon verschilt. Met Viktor werkte visuele communicatie het beste: schermopnames, werkende demo's, Figma-wireframes. Hij reageerde daar direct op. Met Brian was het effectiever om te communiceren via code: pull requests, architecture decision records, technische stukken in Confluence. En bij eindgebruikers (tijdens de usability-tests later in het traject) werkte een scenario-aanpak: "Stel je wilt weten hoeveel de Jira-licenties kosten per team. Klik hier." Naast de inhoud speelde ook de taalkeuze mee: Engels met het internationale team, Nederlands met Viktor en Brian. Die afstemming op de ontvanger, zowel inhoudelijk als qua taal, had ik vooraf niet zo bewust bedacht. Dat groeide vanzelf naarmate het project vorderde.

### 1.2 Rolinvulling

Omdat ik de enige ontwikkelaar was op dit project, had ik automatisch meerdere petten op. Dat klinkt misschien stoer, maar in de praktijk was het soms best overweldigend.

Mijn hoofdrol was uiteraard ontwikkelaar. Ik heb de hele stack gebouwd: de Rust-backend met 12 functionele requirements (van Atlassian-integratie tot CSV-importmodule), de React 19 frontend met dashboardweergaven en KPI-kaarten, en de PostgreSQL-database met 8 migratiebestanden. De technisch lastigste onderdelen waren de vendor-integraties (de Atlassian Admin API heeft vrij strikte rate limits die je in je ontwerp moet meenemen) en de importmodule die 85.000+ rijen uit CSV-bestanden moest verwerken zonder de webserver te blokkeren.

Daarnaast was ik ook mijn eigen Scrum Master. Dat klinkt een beetje raar, want normaal is dat iemand anders. Sprint planning, dailies bijhouden, retrospectives: dat deed ik allemaal zelf. De retrospective was eerlijk gezegd het lastigst om serieus te nemen als je het in je eentje doet. Wie ga je feedback geven? Jezelf? Toch heb ik het gedaan, kort in bullets-format in Confluence. Na Sprint 3 schreef ik op dat ik te laat was begonnen met testen. Dat punt bleek achteraf behoorlijk terecht, want in Sprint 5 moest ik ineens een testachterstand inhalen.

In Sprint 1 en 2 speelde ik vooral de rol van analist. Ik heb toen stakeholderinterviews gehouden, requirements opgesteld met MoSCoW-prioritering in het SRS, en de API's verkend van Atlassian en GitHub. Die verkenning leverde een verrassend inzicht op: Atlassian heeft helemaal geen directe billing-API. Dat betekende dat ik de kostenberekening op een andere manier moest aanpakken dan ik aanvankelijk had gedacht. Goed dat ik dat in de analysefase ontdekte en niet pas tijdens het bouwen.

Tot slot was ik ook mijn eigen tester. Ik heb het Master Test Plan geschreven met een risk-based aanpak, unit tests gedraaid via `cargo test`, integratietests opgezet via PowerShell-scripts, en usability-tests uitgevoerd met echte eindgebruikers. Het feit dat dezelfde persoon de code schrijft en test is vanuit kwaliteitsperspectief niet ideaal. Dat weet ik. Hierbij heb ik geprobeerd die beperking te compenseren door de tests zoveel mogelijk te automatiseren in de CI/CD-pipeline, zodat ze onafhankelijk van mij draaiden.

Rond Sprint 5 merkte ik dat het combineren van al die rollen zwaar begon te worden. Code schrijven, testen, documenteren, feedback verwerken, sprint plannen, dat kon niet allemaal tegelijk. Wat hielp was om per sprint heel duidelijk te kiezen: wat heeft nu prioriteit, en wat schuift door? Die les had ik eigenlijk al uit Sprint 3 moeten trekken, waar ik te veel tegelijk probeerde. Beter laat geleerd dan nooit.

### 1.3 Presenteren

Gedurende het project heb ik op verschillende momenten gepresenteerd of gedemonstreerd, en elk van die momenten was weer net anders.

De sprint reviews deden we elke twee tot drie weken, en duurden zo'n 30 tot 60 minuten. Ik liet dan de werkende applicatie zien aan Viktor of Brian. Geen PowerPoint, gewoon de applicatie live. Viktor gaf al vroeg aan dat hij liever een draaiend scherm zag dan een presentatie, en dat heb ik overgenomen. Waar ik in het begin de fout maakte was dat ik begon met technische uitleg (hoe de Rust-code de Atlassian API aanriep, hoe de caching werkte) en pas daarna liet zien wat het voor de gebruiker opleverde. Na die ene sprint review waarin Viktor me onderbrak met "Maar wat ziet de gebruiker?", heb ik dat omgedraaid. Eerst het resultaat tonen, en alleen op technische details ingaan als erom gevraagd werd.

In Sprint 7 gaf ik een bredere stakeholder demo aan Viktor, Brian en Henk van het management. Dat was zenuwslopend. Het publiek was groter dan ik gewend was, en Henk kende het project vooral vanuit een managementperspectief. Ik heb die demo voorbereid met drie concrete use cases: licentieoverzicht bekijken, een CSV-import uitvoeren, en de GitHub-kostenanalyse openen. Bewust zakelijke taal gebruikt, "kostenallocatie per team" in plaats van "chargeback-query op de organizations-tabel." De structuur kreeg positieve feedback, maar er kwam ook een eerlijk punt: ik ging te snel door de schermen. Henk gaf aan dat hij sommige getallen niet eens had kunnen lezen. Dat was een concreet punt om mee te nemen.

Tijdens de usability-tests in Sprint 6 was mijn rol anders. Daar presenteerde ik niet echt, maar begeleidde ik twee eindgebruikers die het dashboard zelf moesten bedienen. Think-aloud methode: zij vertelden hardop wat ze dachten terwijl ze taken uitvoerden. Hierbij bleek dat mijn neiging om meteen te helpen zodra iemand aarzelde eigenlijk contraproductief was. Juist die aarzelmomenten leverden de waardevolste observaties op. Dat loslaten was voor mij lastig, maar nodig.

Bij de einddemo in Sprint 8 heb ik de feedback van de Sprint 7-demo bewust verwerkt. Langzamer navigeren, pauzes na elke use case, en expliciet vragen: "Zijn er vragen tot zover?" Ik had ook een setje screenshots klaarliggen als backup voor het geval de live omgeving zou haperen, maar dat bleek gelukkig niet nodig.

### 1.4 Ontwikkeling

Als ik terugkijk op hoe ik aan het begin van het project communiceerde versus aan het einde, dan zit daar een behoorlijk verschil. In het begin was ik te technisch en te gedetailleerd. Ik dacht dat ik mijn kennis moest bewijzen en ging daarom diep in op implementatiedetails die mijn publiek helemaal niet nodig had. Het omslagpunt was die sprint review in Sprint 4 waarin Viktor me onderbrak. Dat gaf me het inzicht dat communicatie niet draait om laten zien wat jij weet, maar om geven wat de ander nodig heeft.

Wat achteraf goed werkte: de live demo-aanpak. Gewoon de applicatie laten draaien en laten zien wat het doet. Dat maakte gesprekken meteen concreet en leidde tot betere feedback dan welke slide dan ook. De scenario-opzet bij de stakeholder demo werkte ook goed: door een herkenbaar verhaal neer te zetten ("Stel je bent licentiebeheerder...") snapte het publiek direct wat ik liet zien. Daarnaast merkte ik dat ik steeds makkelijker wisselde tussen technische en zakelijke taal, afhankelijk van wie er tegenover me zat.

De meertalige werkomgeving (Engels bij de stand-ups, Nederlands met Viktor en Brian) dwong me om in beide talen professioneel te functioneren. In de eerste weken moest ik soms even zoeken naar de juiste Engelse vakterm, maar na een tijdje ging dat vanzelf. Het dagelijks schakelen tussen Engels en Nederlands heeft mijn taalvaardigheid op een manier verbeterd die ik niet had bereikt als alles in een taal was geweest.

Waar ik achteraf spijt van heb: dat ik niet eerder direct contact heb gezocht met eindgebruikers. De usability-tests in Sprint 6 leverden zoveel bruikbare inzichten op dat ik denk: als ik dat al in Sprint 4 had gedaan, had ik minstens een sprint aan rework bespaard op de UI. En bij de stand-ups had ik in de eerste weken uitgebreider moeten vertellen waar ik vastliep, in plaats van alleen te noemen wat ik had afgerond. Die terughoudendheid kostte me soms een dag extra zoeken, terwijl een collega me in vijf minuten op weg had kunnen helpen.

---

## 2. Feedback verwerken, testen en iteratief verbeteren

### 2.1 Feedback ontvangen

Gedurende het project heb ik feedback gekregen van verschillende kanten, en elke bron leverde weer ander soort inzichten op.

Viktor gaf na elke sprint review functionele feedback: wat vond hij goed, wat moest anders, welke prioriteiten schoven. Die feedback was vrijwel altijd concreet en actiegericht. Zinnen als "De marge moet zichtbaarder op de GitHub-pagina" of "De import-preview moet duidelijker tonen wat er nieuw is versus wat al bestond." Ik heb Viktor ook bewust tussendoor om feedback gevraagd. Als ik een Figma-wireframe had gemaakt voor een nieuw scherm, deelde ik die via Teams om te checken of het aansloot bij zijn verwachtingen. Dat resulteerde in een presentatie en bespreking van mijn aantekeningen samen met Viktor. Die tussentijdse afstemming voorkwam dat ik twee weken lang iets bouwde waar hij hele andere verwachtingen bij had.

Van Brian kwam de technische feedback, vooral op codekwaliteit en architectuurkeuzes. Die kreeg ik tijdens de informele code reviews voordat ik een feature branch samenvoegde in main. Brian lette op dingen als scheiding van verantwoordelijkheden (zit de logica in de juiste laag?), foutafhandeling (geen `unwrap()` in productiecode), en testbaarheid. Zijn feedback kwam soms hard aan, met name als ik dacht dat mijn code al goed was. Maar hij onderbouwde het altijd, en na verwerking was de code daadwerkelijk beter. Ik maakte er een gewoonte van om na elke review zijn opmerkingen punt voor punt langs te gaan en het resultaat terug te koppelen.

Tijdens de usability-tests in Sprint 7 werkte ik voor het eerst met echte eindgebruikers: een licentiebeheerder en een finance-medewerker. De feedback hier was anders dan wat ik van Viktor en Brian gewend was. Niet zozeer in woorden, maar in gedrag. Waar aarzelt iemand? Waar klikt iemand verkeerd? Hierbij bleek dat een testpersoon in de import-preview "nieuw" en "bijgewerkt" door elkaar haalde. De reden: de visuele distinctie was te subtiel. Ik had dat zelf nooit gezien, want ik wist precies hoe het werkte. Een andere bevinding was dat de zoekbalk op de `/users`-pagina niet meteen opviel. Dat soort dingen ontdek je alleen als je iemand anders ermee laat werken.

Van mijn schoolbegeleider Jeroen Boogaard kreeg ik feedback op de academische kant: documentstructuur, onderbouwing, bronvermelding. Die feedback kwam minder vaak (vooral rond inlevermomenten), maar hielp me om mijn documenten scherper te krijgen.

### 2.2 Feedback verwerken

Feedback ontvangen is een ding. Er iets mee doen is een tweede. In de praktijk merkte ik dat je daar een bewust proces voor nodig hebt, anders verdwijnt de helft.

Ik heb alle feedback gelogd: in Jira als opmerking bij het bijbehorende issue, of in Confluence als actie-items na een review. Na de stakeholder demo in Sprint 7 had ik bijvoorbeeld vijf verbeterpunten. Zonder die vastlegging had ik er na een week hooguit twee onthouden. Niet elke suggestie was trouwens haalbaar. Toen een stakeholder in Sprint 6 aangaf dat er ook JFrog-integratie moest komen, heb ik dat besproken en we besloten gezamenlijk dat het buiten scope viel. Die keuze heb ik vastgelegd in het technisch adviesrapport als aanbeveling voor doorontwikkeling. Verwerkte feedback koppelde ik altijd terug, bij code via de volgende commit met een verwijzing naar het Jira-issue, bij functionele punten door het in de volgende sprint review te laten zien.

Een concreet voorbeeld: tijdens de usability-tests in Sprint 7 keek een finance-medewerker naar de GitHub vendor-pagina en zag de marges niet. Die stonden onderaan de kaart, in dezelfde lettergrootte als de rest. Na die observatie heb ik de marge-waarden groter gemaakt, boven de kaart geplaatst, en een kleuraccent in Equans-groen (#008163) toegegeven. In de volgende testsessie werd de marge als eerste opgemerkt. Dat was een klein ding qua code, maar het verschil voor de gebruiker was enorm.

Nog een voorbeeld: na de Sprint 7 demo gaf een van de stakeholders aan dat ik te snel door de schermen navigeerde. Bij de einddemo in Sprint 8 heb ik dat bewust aangepast. Langzamer klikken, na elke use case een pauze, en actief vragen of er nog vragen waren. De terugkoppeling achteraf: het tempo was nu goed. Zo'n klein punt maakt het verschil tussen een demo die overkomt en een demo die langs mensen heen gaat.

### 2.3 Testen als feedbackmechanisme

Naast feedback van mensen heb ik testen ingezet als een soort automatisch feedbacksysteem. In het Master Test Plan had ik een risk-based testing aanpak beschreven: het meeste testaandacht naar de onderdelen waar de meeste schade ontstaat als er iets misgaat. Dat waren de authenticatielaag (als daar een bug inzit, staan alle endpoints open), de licentieberekeningen (als `calculate_utilization` fout rekent, maakt Equans verkeerde financiele beslissingen), en de data-import (als die corrupt data wegschrijft, is de hele database vervuild).

Op het laagste niveau heb ik unit tests in Rust geschreven voor individuele functies: de GID-matcher, de CSV-parser, de merge-engine. `cargo test` draait in seconden, en dat maakte het haalbaar om continu te testen terwijl ik aan het ontwikkelen was. Niet na het bouwen, maar tijdens het bouwen. Die shift-left aanpak klinkt als theorie uit een boek, maar het werkte echt: bugs die ik in Sprint 5 ontdekte in mijn CSV-parser hadden weken oud kunnen zijn als ik niet regelmatig had getest.

Op API-niveau heb ik integratietests geschreven als PowerShell-scripts: `test_atlassian_endpoints.ps1` en `test_github_endpoints.ps1`. Die testten de volledige keten van request tot database en draaiden als onderdeel van de CI/CD-pipeline op GitHub Actions. Voor performance heb ik in Sprint 5 de API-responstijden gemeten. De P95-tijd moest onder de 200ms zitten. Dat lukte, maar de dashboard-laadtijd was aanvankelijk boven de 3 seconden. Na het toevoegen van lazy loading voor de gebruikerstabel (die bevat uiteindelijk duizenden rijen) kwam dat binnen de norm.

De usability-tests in Sprint 6 waren een heel ander soort feedback. Geen automatische PASS/FAIL, maar observaties: lukt de taak, hoe lang duurt het, waar gaat het mis. De meetpunten waren effectiviteit (taak afgerond: ja of nee), efficientie (tijd per taak), en tevredenheid op een vijfpuntsschaal. En tot slot security tests: JWT-validatie met ongeldige en verlopen tokens, GDPR-compliance controleren (worden e-mailadressen gemaskeerd in logs?), en rate-limiting valideren voor de externe API-calls.

Wat ik hiervan heb geleerd: testen is niet iets dat je na het bouwen doet. In Sprint 3 schreef ik bijna geen tests, en dat betaalde ik in Sprint 5 dubbel terug. Sindsdien schrijf ik tests terwijl ik bouw, niet erna.

### 2.4 Iteratief verbeteren

Het mooie van de Scrum-werkwijze is dat je elke sprint een kans hebt om bij te sturen. En dat heb ik volop gedaan.

De import-module is daar een goed voorbeeld van. In Sprint 3 had ik een basis CSV-import die data kon inlezen en wegschrijven. Functioneel, maar meer ook niet. Na feedback van Brian (er moest een preview-functie bij) heb ik in Sprint 5 de drie-stappen-flow gebouwd: bestand uploaden, preview van de wijzigingen bekijken, dan pas uitvoeren. Tijdens de usability-tests in Sprint 6 bleek vervolgens dat gebruikers "nieuw" en "bijgewerkt" door elkaar haalden in die preview (de visuele distinctie was te zwak). Dat heb ik aangepast. En in Sprint 7, op verzoek van Viktor, kwam de rollback-functionaliteit erbij: tot 30 dagen terug ongedaan maken, met de originele data opgeslagen als JSONB-snapshot in de `imports`-tabel.

Het dashboard zelf maakte een vergelijkbare reis. De eerste versie uit Sprint 4 was functioneel maar rommelig. Brian merkte op dat de componenthierarchie onnodig complex was, en de laadtijd was te hoog. In Sprint 5 heb ik de code opgeschoond en lazy loading geimplementeerd. Na de usability-tests in Sprint 6 heb ik de navigatie aangepast (de sidebar had teveel items die niet logisch gegroepeerd waren) en de KPI-kaarten prominenter gemaakt op het hoofdscherm. Sprint 7 was de fine-tuning: typografie, kleuren consistent met de Equans-huisstijl (#002439 en #008163), en fatsoenlijke empty states voor als er nog geen data geladen was.

Hierbij bleek dat de architectuur zelf ook een iteratie nodig had. In het begin had ik logica verspreid over route-handlers en services, zonder duidelijke grens. Na feedback van Brian heb ik in Sprint 4 een middag besteed aan refactoren naar een strikte gelaagde structuur: routes roepen services aan, services roepen repositories aan, en nergens anders. Die middag voelde als verloren tijd, maar het betaalde zich terug toen ik in Sprint 5 de repository-laag los kon testen zonder de hele API op te moeten starten.

Het iteratieve proces leerde me iets simpels maar waardevols: een eerste versie is nooit de laatste versie, en dat is prima. Liever snel iets werkends neerzetten en bijsturen op basis van wat je hoort en ziet, dan wekenlang in isolatie bouwen en achteraf ontdekken dat het niet aansluit.

---

## 3. Reflectie op professioneel handelen

### 3.1 Zelfstandigheid

Als enige ontwikkelaar op dit project moest ik alle technische beslissingen zelf nemen. Dat voelde in het begin best spannend, want er was niemand om me heen die zei: "Doe het zo." Ik moest zelf bedenken hoe de architectuur eruit moest zien, welke Rust crate ik gebruikte voor CSV-parsing, hoe de GID-matcher personen aan vendor-accounts moest koppelen.

Een paar keuzes wil ik hier uitlichten. De monolithische architectuur in plaats van microservices was een bewuste afweging. In eerste instantie heb ik microservices serieus overwogen, want dat is wat je overal hoort als "de juiste aanpak." Maar na er goed over nagedacht te hebben, concludeerde ik dat de extra complexiteit (aparte deployments, service-to-service communicatie, gecentraliseerde logging) niet opwoog tegen de voordelen voor een project met een team van een. Die overweging heb ik gedocumenteerd in Confluence, met de argumenten voor en tegen. Nog steeds blij met die keuze trouwens. Debuggen in een monoliet is gewoon makkelijker.

De cache-first strategie voor de Atlassian API was ook een eigen keuze. Tijdens de ontwikkeling merkte ik dat de API soms gewoon traag was, en af en toe een timeout gaf. Ik wilde niet dat het dashboard leeg bleef als Atlassian even niet bereikbaar was. Dus heb ik een cache gebouwd met 25 uur TTL: liever data van gisteren met een waarschuwingsmelding dan een blanco scherm. En de keuze om `unwrap()` volledig te verbannen uit productiecode kostte me per functie extra schrijfwerk (elke fout expliciet afhandelen via `Result<T, E>`), maar voorkomt dat het systeem crasht op iets onverwachts in productie.

Maar zelfstandigheid betekent niet dat ik nooit hulp heb gevraagd. Bij de Azure AD JWT-validatie liep ik vast op de JWKS-endpoint integratie. Na een hele dag zelf puzzelen heb ik Brian erbij gehaald. Die had binnen een kwartier de richting te pakken. Achteraf baal ik dat ik niet eerder had gevraagd: die dag had ik productiever kunnen besteden. Sindsdien hanteer ik de vuistregel: als ik na twee uur nog geen stap verder ben, vraag ik het iemand.

### 3.2 Planning en discipline

Het project was gepland in 8 sprints over 20 weken. Jira als issue tracker (projectcode: SDPDOFS), Confluence als kennisbank, GitHub voor versiebeheer met feature branches per Jira-issue. Die structuur heb ik aan het begin opgezet en gedurende het project bijgesteld waar nodig.

Een paar dingen werkten goed. De featuregerichte branchstrategie (elke feature een eigen branch, vernoemd naar het Jira-issue zoals SDPDOFS-546-ophalen-van-gebruikerslijst) hield de main branch altijd werkend. Ik kon op elk moment een demo geven zonder me zorgen te maken over half-afgeronde code. De koppeling van commitberichten aan Jira-issuenummers bleek achteraf ontzettend waardevol: weken later kon ik via een commit precies terugvinden waarom ik een wijziging had gemaakt en welk ticket eraan ten grondslag lag. Het opsplitsen van grote user stories in kleinere subtaken maakte de voortgang meetbaar. "Import-module bouwen" is vaag en onvoorspelbaar. "CSV-parser schrijven", "Validatieregels implementeren", "Preview-endpoint bouwen" zijn afgebakend en af te vinken.

Maar andere dingen gingen minder soepel. Sprint 4 en 5 had ik allebei 3 weken gegeven in plaats van 2, omdat ik verwachtte dat ze meer werk zouden bevatten. Zelfs dat was krap. De frontend MVP in Sprint 4 was ambitieuzer dan ingeschat (acht pagina's bouwen in drie weken, plus Figma-wireframes afstemmen met Viktor), en Sprint 5 werd zwaarder dan gepland door de testachterstand uit Sprint 3. Daar zat een les in: bij het plannen moet je ruimte inbouwen voor dingen die je niet kunt voorspellen.

Documentatie was een ander pijnpunt. Ik schreef het vaak op het laatste moment van een sprint, als de code al af was. Het resultaat: soms haastwerk. In Sprint 6 heb ik dat bijgesteld door documentatie als aparte Jira-taken aan te maken in plaats van het als bijzaak te behandelen. Die aanpak hielp, maar de spanning tussen "code schrijven" en "over code schrijven" bleef het hele project aanwezig. Als developer wil je bouwen, niet documenteren.

### 3.3 Afwegingen maken

Een afstudeerproject heeft een harde einddatum. Dat dwingt je om keuzes te maken, en soms betekent dat: nee zeggen.

Het project werd oorspronkelijk gepresenteerd als een dashboard voor Atlassian-producten, met als optionele uitbreiding GitHub als ik nog tijd over had. Vanaf het begin heb ik echter onderzocht en besloten dat het dashboard volledig compatibel moest zijn met zowel GitHub als Atlassian. Die ambitie heb ik waargemaakt, maar het betekende wel dat de scope strak bewaakt moest worden.

Toen een stakeholder in Sprint 6 aangaf dat er ook JFrog-integratie bij moest, heb ik dat afgewogen. Technisch haalbaar? Waarschijnlijk wel. Maar het zou ten koste gaan van de kwaliteit van wat er al stond, en de drie resterende sprints waren al vol gepland. Ik heb voorgesteld om JFrog als aanbeveling op te nemen in het technisch adviesrapport voor doorontwikkeling. Viktor ging akkoord, maar het was wel een moment waarop ik bewust nee zei tegen iemand met meer senioriteit. Dat vond ik niet makkelijk.

Op technisch vlak heb ik ook pragmatische keuzes gemaakt. Bij de import-module heb ik de rollback vereenvoudigd: geen complete transactie-log die elke individuele rij-wijziging bijhoudt, maar een JSONB-snapshot van de originele data in de `imports`-tabel. Minder elegant, maar het werkt betrouwbaar en was te bouwen binnen een sprint. De GID-matcher gebruikt heuristische patroonmatching met confidence scores in plaats van machine learning. ML had potentieel nauwkeuriger gekund, maar daarvoor had ik trainingsdata nodig die er niet was, en een expertise die ik binnen de projecttijd niet kon opbouwen. "Goed genoeg en onderhoudbaar" woog hier zwaarder dan "theoretisch beter maar onhaalbaar."

De keuze om geen end-to-end tests met Cypress of Playwright te schrijven was ook een afweging. Het had de testsuite completer gemaakt, maar de investering in setup en onderhoud (voor een soloproject van 20 weken) stond niet in verhouding. Ik heb dat gecompenseerd met handmatige usability-tests en geautomatiseerde integratietests op API-niveau.

### 3.4 Kwaliteitsbewustzijn

Kwaliteitsborging liep als een rode draad door het project. Niet als een los hoofdstuk op het einde, maar als iets dat ik gaandeweg steeds serieuzer ben gaan nemen.

Op documentatieniveau heb ik alle architectuurbeslissingen vastgelegd in ADRs (ADR-001 tot en met ADR-005), telkens met de overwogen alternatieven en de motivatie voor de uiteindelijke richting. Functionele requirements (FR-001 tot en met FR-012), technische requirements (TR-001 tot en met TR-012) en business requirements (BR-001 en BR-002) staan allemaal systematisch gedocumenteerd in Confluence. Dat klinkt als een hoop papierwerk, en dat is het ook. Maar het dwingt je om keuzes te expliciteren die je anders alleen in je hoofd hebt.

Op testniveau definieerde het Master Test Plan de teststrategie: risk-based, shift-left, testpyramide. Unit tests in Rust voor de snelle feedback, integratietests via PowerShell-scripts voor de API-endpoints, usability-tests met eindgebruikers voor de gebruikerservaring. De CI/CD-pipeline op GitHub Actions zorgde ervoor dat elke push automatisch werd gebouwd en getest, zodat ik niet afhankelijk was van mijn eigen discipline om tests te draaien.

En ook al was ik de enige ontwikkelaar, toch heb ik informele code reviews gedaan met Brian. Elke feature branch besprak ik inhoudelijk voordat ik samenvoegde. Soms was dat een kort gesprek van tien minuten, soms een half uur als het complexere code betrof (zoals de merge-engine in `imports/merger.rs`). Het dwong me om mijn code uit te leggen, en alleen dat proces brengt al fouten aan het licht. Als je niet helder kunt uitleggen wat een stuk code doet, is het waarschijnlijk te complex.

Wat ik hiervan meeneem: kwaliteit is geen fase die je aan het einde afraffelt. De Definition of Done, de gewoonte om tests te schrijven terwijl je bouwt (na de les uit Sprint 3), de code reviews met Brian, dat zijn geen extra taken bovenop het "echte werk." Het zijn investeringen die zich terugbetalen in minder bugs, betere leesbaarheid, en een product waar je achter kunt staan als je het opdraagt aan iemand anders.

---

## Conclusie

Als ik eerlijk terugkijk op dit project, dan is mijn grootste groei niet technisch. Rust leren, een API bouwen, een React-frontend neerzetten: dat zijn vaardigheden die ik heb opgedaan, maar die me niet het meest hebben verrast. Wat me wel verraste is hoeveel verschil het maakt hoe je communiceert.

In het begin was ik een ontwikkelaar die technisch kon uitleggen wat hij deed, maar niet goed aanvoelde wat zijn gesprekspartner eigenlijk wilde horen. Technisch detail voor Brian, functioneel resultaat voor Viktor, scenario's voor eindgebruikers: die afstemming voelde aan het begin onnatuurlijk, maar werd gaandeweg vanzelfsprekend. Na twintig weken daily stand-ups, sprint reviews, usability-sessies en stakeholder demo's durf ik te zeggen dat ik die vaardigheid nu heb. En ik denk dat die waardevoller is dan welke technische skill dan ook. Technologie verandert elk jaar. De noodzaak om goed te communiceren blijft.

Op drie vlakken ben ik gegroeid als professional. Ik heb geleerd om een volledig project zelfstandig te plannen en te bewaken, inclusief de momenten waarop je eerlijk moet zijn dat iets niet lukt of dat de scope te breed wordt. Ik heb geleerd om feedback niet te zien als kritiek maar als informatie, en om die informatie actief op te zoeken in plaats van af te wachten. En ik heb geleerd om pragmatische keuzes te maken: niet de technisch mooiste oplossing, maar de oplossing die past bij de context, de tijd die je hebt, en de mensen voor wie je bouwt.

Wat ik meeneem naar toekomstige projecten: altijd eerst begrijpen wie je publiek is voordat je begint met uitleggen. Snel iets werkends opleveren en bijsturen, want in isolatie bouwen leidt tot verkeerde aannames. Feedback actief ophalen, vastleggen, en terugkoppelen. Accepteren dat "goed genoeg en op tijd" beter is dan "perfect en te laat." En kwaliteit behandelen als een dagelijkse gewoonte, niet als een afvinklijstje aan het eind.

Dit project was niet perfect. Ik heb documentatie uitgesteld, te laat feedback gevraagd bij eindgebruikers, en de complexiteit van meerdere sprints onderschat. Maar juist die fouten hebben meer opgeleverd dan de dingen die meteen goed gingen. Dat klinkt misschien als een cliche, maar na twintig weken bouwen, testen, presenteren, en bijsturen weet ik dat het klopt.