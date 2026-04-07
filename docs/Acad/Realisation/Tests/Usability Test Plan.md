# Usability Test Plan

## Equans Operational Insights Dashboard

|                 |                                                                             |
| --------------- | --------------------------------------------------------------------------- |
| **Versie**      | 1.0                                                                         |
| **Studentnaam** | Ahmad Alhaj Asaad (1035912)                                                 |
| **Project**     | Equans Operational Insights Dashboard                                       |
| **Opleiding**   | Informatica – Hogeschool Rotterdam                                          |
| **Organisatie** | Equans Nederland – SLS Digital Platforms (DevOps Forge)                     |
| **Begeleiders** | Viktor Klein (bedrijf), Brian Veltman (technisch), Jeroen Boogaard (school) |
| **Studiejaar**  | 2025 - 2026                                                                 |
| **Referentie**  | MTP-001 – Master Test Plan, sectie 4.6                                      |

---

## Inhoudsopgave

1. [Inleiding](#1-inleiding)
2. [Doel van usability tests](#2-doel-van-usability-tests)
3. [Scope](#3-scope)
4. [Testmethode](#4-testmethode)
5. [Testpersonen](#5-testpersonen)
6. [Testscenario's](#6-testscenarios)
7. [Meetpunten (Metrics)](#7-meetpunten-metrics)
8. [Acceptatiecriteria](#8-acceptatiecriteria)
9. [Output en rapportage](#9-output-en-rapportage)
10. [Referenties](#10-referenties)

---

## 1. Inleiding

### 1.1 Waarom usability testing?

Ergens halverwege sprint 4 had ik een moment dat me aan het denken zette. Ik was bezig met de GitHub vendor-pagina en had net de drie productkaarten (Copilot, GHAS, Licenses) werkend gekregen. Trots. Tot ik Viktor het scherm liet zien en hij vroeg: "En waar zie ik nu de marge?" Het stond er gewoon, onderaan de kaart. Maar hij keek er overheen. Dat was het moment dat ik dacht: ik moet dit gaan testen met echte mensen. Niet met collega-ontwikkelaars, maar met de licentiebeheerders en PMO-medewerkers die dit straks dagelijks gaan gebruiken.

Tot dat punt had ik alleen technische tests. Unit tests voor de Rust-backend, integratietests voor de API-endpoints, dat soort dingen. Allemaal nodig, maar ze vertellen je niks over hoe iemand het dashboard ervaart. Of iemand de zoekbalk vindt. Of de import-flow logisch aanvoelt. Nielsen (1993) noemt dat het verschil tussen verificatie en validatie: controleert het systeem of het technisch klopt, versus voor de persoon die erachter zit ook echt bruikbaar is.

In de ISO 9241-11-standaard (ISO, 2018) splitsen ze bruikbaarheid op in drie stukken: effectiviteit (lukt de taak?), efficiëntie (hoe snel en met hoeveel gedoe?) en tevredenheid (vindt de gebruiker het prettig?). Dat leek me een handig kader om mee te werken. Voorheen had ik het begrip "gebruiksvriendelijkheid" altijd een beetje vaag gevonden, maar met die drie pijlers wordt het concreter. Je kunt er dingen mee meten.

Nog iets: na maanden ontwikkelen was ik zelf blind geworden voor de interface. Alles voelde voor mij logisch want ik had het zelf gebouwd. Ik wist precies waar elke knop zat, hoe de filters werkten, wat de import-flow deed. Maar dat zegt niks over hoe een finance-medewerker die het dashboard voor het eerst opent ertegenaan kijkt.

### 1.2 Relatie met andere testdocumenten

In het Master Test Plan (MTP-001) had ik in sectie 4.6 al opgeschreven dat ik usability tests wilde doen. Think-aloud sessies met 2 tot 4 Equans-medewerkers, dat was het plan. Maar de details ontbraken toen nog: welke taken precies, hoe meet je het, wanneer is het goed genoeg? Dat is wat dit document invult.

| Document                                      | Relatie                                                                               |
| --------------------------------------------- | ------------------------------------------------------------------------------------- |
| **Master Test Plan (MTP-001)**                | Overkoepelend testplan; sectie 4.6 beschrijft usability testing op strategisch niveau |
| **Software Requirements Specification (SRS)** | Bron voor de functionele requirements waaraan ik de testscenario's heb gekoppeld      |
| **Unit Test Plan (UTP)**                      | Het UTP test technische correctheid; dit plan test de gebruikerservaring              |
| **Security Test Plan**                        | Complementair: security valideert beveiliging, dit plan valideert bruikbaarheid       |

Wat ik achteraf blij mee ben is dat ik de testscenario's rechtstreeks aan SRS-requirements heb gekoppeld (M-xx, S-xx nummering). Dat had ik bij het Unit Test Plan ook gedaan en het maakt het terugvinden van bevindingen per functionaliteit een stuk makkelijker. Stel dat er een probleem opduikt bij de import-flow, dan kan ik direct zien dat dat bij M-12 en S-08 hoort en het in de juiste sprint-backlog gooien.

### 1.3 Welke gebruikersflows worden getest?

Na een overleg met Viktor en Brian heb ik een lijstje gemaakt van de flows die het vaakst voorkomen en waar het meeste misgaat als ze niet goed werken. Dat tweede criterium was eigenlijk doorslaggevend. Er zijn best wat features in het dashboard, maar niet alles is even kritiek. De loginflow via Microsoft Entra ID is bijvoorbeeld ook een interactie, maar daar zit eigenlijk maar een knop in. Niet veel te testen.

Wat ik wel test: het raadplegen van het overview-dashboard (daar begint iedereen), het zoeken en filteren van personen op de `/users`-pagina, het importeren van een CSV- of Excel-bestand via `/import`, het analyseren van GitHub-kosten per product op de vendor-pagina, het openen van een organisatie met de gekoppelde personen, het handmatig triggeren van een datasync, en het exporteren van gefilterde data naar CSV. Dat zijn er zeven. Ik had eerst aan vijf gedacht (dat stond ook in het MTP), maar de export en de sync kwamen er later bij omdat Viktor specifiek aangaf dat die voor de PMO-afdeling onmisbaar zijn.

---

## 2. Doel van usability tests

### 2.1 Wat wil ik bereiken?

Eigenlijk heel simpel: ik wil weten of het dashboard werkt voor de mensen die het straks moeten gebruiken. Maar "werkt" is een rekbaar begrip. Een licentiebeheerder die een persoon kan opzoeken maar er steeds twee minuten over doet, daar schiet je niks mee op. En een finance-medewerker die de maandkosten kan vinden maar de marge over het hoofd ziet, dat leidt tot verkeerde chargeback-berekeningen.

Ik heb het daarom opgesplitst. Kan iemand de taak afronden zonder dat ik moet helpen? Dat is de effectiviteitsvraag. Hoe lang duurt het? Dat is efficiëntie. Waar gaat het mis en waarom? Dat zijn de fouten die ik wil vinden. Hoe voelt het om ermee te werken? Dat is tevredenheid. En dan nog specifiek: klopt de navigatiestructuur? De sidebar heeft best wat items (Overview, Atlassian, GitHub, Persons, Organizations, Import) en ik weet niet zeker of dat voor iedereen logisch gegroepeerd is.

### 2.2 Waar maak ik me zorgen over?

Iets waar ik van tevoren al een beetje bang voor was: de mix van technische en niet-technische gebruikers. Een IT-beheerder snapt termen als "GID-matching" en "API sync" waarschijnlijk prima. Maar de PMO-medewerker die de chargeback doet? Die wil weten wat de totale kosten zijn en welke marge erop zit. Woorden als "billable rate" zijn voor die groep niet vanzelfsprekend. Brian tipte me hier al eerder op, en het zat sindsdien in mijn achterhoofd.

Wat ik ook wil weten is of de import-flow helder genoeg is. Die flow bestaat uit drie stappen: bestand selecteren, preview bekijken, bevestigen. Klinkt simpel. Maar tijdens een informele test eerder dit semester haalde iemand "nieuw" en "bijgewerkt" door elkaar in de preview. Dat is precies het soort ding dat je wilt aftesten. De foutmeldingen zijn een ander punt. Als iemand een CSV-bestand upload met een ontbrekende kolom, krijgt hij nu een technische foutmelding terug. De vraag is of die foutmelding genoeg informatie geeft om het probleem te fixen zonder mij te hoeven bellen.

En dan de sidebar-navigatie. Ik heb daar in het prototype al wat feedback op gehad (zie het Prototype Ontwerp-document, sectie 4.3), maar of het in de daadwerkelijke implementatie net zo werkt als in Figma is een andere vraag.

### 2.3 Waarom dit ertoe doet

Als het dashboard niet fijn werkt, gaat niemand het gebruiken. Simpel. Equans heeft nu een handmatig proces voor licentiekosten, met spreadsheets die Viktor en het PMO-team bijhouden. Het hele punt van dit project is dat proces vervangen. Maar als de vervanging frustrerender is dan de spreadsheet, stappen mensen gewoon terug naar hun oude werkwijze. Dat heb ik vaker gehoord bij stage-genoten die dashboards bouwden. Technisch klopte het, maar het werd nooit geadopteerd.

Door nu met de echte gebruikers te gaan zitten voordat het live gaat, hoop ik dat soort scenario's te voorkomen. En het helpt ook bij mijn eigen blinde vlekken. Die situatie met Viktor en de marge-informatie die ik noemde in de inleiding is daar een goed voorbeeld van. Ik dacht dat het duidelijk was, en dat was het niet. Dat soort inzichten krijg je niet uit een unit test.

---

## 3. Scope

### 3.1 Wat wordt er getest?

De tests gaan over de React-frontend. Alles wat de gebruiker ziet en waar hij mee interacteert. Ik heb bewust gekozen voor de functionaliteiten die het vaakst worden gebruikt of waar de gevolgen het grootst zijn als het misgaat.

| Categorie             | Functionaliteit                                                                            | Gerelateerde requirements |
| --------------------- | ------------------------------------------------------------------------------------------ | ------------------------- |
| **Dashboard**         | Geconsolideerd overzicht met KPI-kaarten, licentiestatistieken en kosten over alle vendors | M-01, M-02, M-03          |
| **Atlassian-details** | Productkaarten met kosten per product (Jira, Confluence, Trello, JSM) inclusief marge      | M-02, M-14                |
| **GitHub-details**    | Vendor-pagina met drie productkaarten voor Copilot, GHAS en Licenses                       | M-15, M-16                |
| **Personen**          | Overzicht met zoekbalk, filters op status, organisatie en land, en paginering              | M-10, S-05, S-06, S-13    |
| **Organisaties**      | Overzicht met detailpagina's, gekoppelde personen en statistieken                          | M-11, S-07                |
| **Data-import**       | CSV/Excel-upload met preview, voortgangsindicatie en foutafhandeling                       | M-12, S-08                |
| **Navigatie**         | Sidebar-menu, pagina-routing en informatiestructuur                                        | —                         |
| **Authenticatie**     | SSO-loginflow via Microsoft Entra ID en sessieafhandeling                                  | M-08, S-04                |
| **Synchronisatie**    | Handmatige refresh-trigger en weergave van laatst gesynchroniseerd tijdstip                | S-01, S-02, S-03          |
| **Export**            | CSV-download met behoud van actieve filters                                                | S-11                      |

### 3.2 Wat wordt er niet getest, en waarom?

De backend performance test ik hier niet. Responstijden van de API worden apart gemeten met k6-scripts (staat in MTP-001, sectie 4.4). Security en autorisatie (JWT-validatie, OWASP) vallen onder het Security Test Plan; dat is een ander verhaal dan of de interface logisch is. API-correctheid gaat via Postman-collecties. Databasemigraties? Puur technisch, daar klikt geen gebruiker op.

JFrog en Trello heb ik er ook uitgelaten. Dat zijn Could Have-requirements (C-04, C-05) die nog niet gebouwd zijn. RBAC is een Won't Have (W-03), het systeem heeft alleen een admin- en user-rol en dat verschil is zo klein dat het niet zinvol is om daar apart usability op te testen.

Twee dingen waar ik even over twijfelde: mobiel en accessibility. Mobiel heb ik uiteindelijk niet meegenomen omdat het dashboard echt een desktopapplicatie is. De licentiebeheerders bij Equans zitten achter een Windows-laptop met twee schermen, die gaan dit niet op hun telefoon doen. WCAG-accessibility zou ik eigenlijk wel interessant vinden, maar een volledige audit past niet in de tijd die ik heb. Radix UI (de componentenbibliotheek die ik gebruik) heeft al basistoegankelijkheid ingebouwd, dus helemaal niets is het niet.

---

## 4. Testmethode

### 4.1 Methode-overzicht

| Onderdeel           | Beschrijving                                                       |
| ------------------- | ------------------------------------------------------------------ |
| **Methode**         | Task-based usability testing met think-aloud protocol              |
| **Type test**       | Moderated (geobserveerd door testleider)                           |
| **Duur per sessie** | 45 tot 60 minuten per testpersoon                                  |
| **Locatie**         | Op locatie bij Equans (Sliedrecht) of via Microsoft Teams          |
| **Opnametools**     | Schermopname via Teams of OBS Studio, audio-opname van think-aloud |

### 4.2 Hoe de tests verlopen

Het idee is vrij rechttoe rechtaan. De testpersoon krijgt een taak, bijvoorbeeld: "zoek persoon X op en vertel me bij welke organisatie die hoort." Diegene voert dat uit terwijl hij of zij hardop vertelt wat er door het hoofd gaat. Ondertussen zit ik erbij, observeer, en schrijf op waar het misgaat of waar twijfel ontstaat. Dat is in de kern wat task-based testing met think-aloud inhoudt.

Ik had eerst overwogen om het unmoderated te doen. Dus: de testpersoon krijgt een linkje, voert zelfstandig de taken uit, en ik bekijk achteraf de schermopnames. Scheelt een hoop agenda-gezeur. Maar toen ik erover nadacht realiseerde ik me dat je dan heel veel context mist. Stel iemand staart tien seconden naar het scherm, klikt dan op de juiste knop. In de logdata ziet dat eruit als "succes, duur: 10 seconden." Maar die tien seconden twijfel? Dat is informatie die je nodig hebt. Bij een moderated sessie zie je die aarzeling, en met think-aloud hoor je ook waarom. "Hmm, ik zoek een refresh-knop maar ik verwacht hem rechtsboven" of zoiets.

Elke sessie volgt hetzelfde protocol. Dat heb ik bewust zo gedaan, want als ik bij de ene testpersoon andere instructies geef dan bij de andere kan ik de resultaten niet naast elkaar leggen.

| Fase                 | Duur      | Wat er gebeurt                                                                                               |
| -------------------- | --------- | ------------------------------------------------------------------------------------------------------------ |
| 1. Introductie       | 5 min     | Doel uitleggen, werkwijze, toestemming opname, benadrukken dat het om het systeem gaat en niet om de persoon |
| 2. Achtergrondvragen | 5 min     | Korte vragen over rol, ervaring met softwaretools, bekendheid met licentiemanagement                         |
| 3. Taakuitvoering    | 25-35 min | De testpersoon voert vijf tot zes taken uit met hardop denken; ik observeer                                  |
| 4. Nabeschouwing     | 5-10 min  | Open vragen: wat vond je lastig, wat vond je prettig, suggesties?                                            |
| 5. SUS-vragenlijst   | 5 min     | System Usability Scale invullen, een gestandaardiseerde vragenlijst (Brooke, 1996)                           |

### 4.4 Hulpmiddelen

| Hulpmiddel                        | Waarvoor                                                             |
| --------------------------------- | -------------------------------------------------------------------- |
| **Microsoft Teams of OBS Studio** | Scherm- en audio-opname                                              |
| **Observatieformulier**           | Per taak fouten, twijfelmomenten en opmerkingen noteren              |
| **System Usability Scale (SUS)**  | Gestandaardiseerde vragenlijst met tien items op een vijfpuntsschaal |
| **Stopwatch**                     | Doorlooptijd per taak bijhouden                                      |
| **Testscript**                    | Vastgelegd stappenplan zodat elke sessie uniform verloopt            |

---

## 5. Testpersonen

### 5.1 Deelnemers

| Kenmerk       | Beschrijving                                                                                           |
| ------------- | ------------------------------------------------------------------------------------------------------ |
| **Aantal**    | 3 testpersonen                                                                                         |
| **Profiel**   | Equans-medewerkers: licentiebeheerders, finance en IT                                                  |
| **Ervaring**  | Basiskennis van webapplicaties; ervaring met vendor-adminportals is mooi meegenomen maar geen vereiste |
| **Apparaten** | Windows-laptop                                                                                         |

### 5.2 Wie doet er mee?

De keuze voor de testgroep heb ik gebaseerd op de stakeholder-analyse uit de SRS (sectie 2.1). Daar had ik drie hoofdgroepen geïdentificeerd. De licentiebeheerders van SLS Digital Platforms, die dagelijks met vendor-accounts en licenties werken. Finance en PMO, die verantwoordelijk zijn voor de maandelijkse chargeback naar Business Units. En IT-beheerders, die data importeren en accounts onderhouden.

| Nr  | Rol                 | Afdeling              | Waarom juist deze persoon?                                                                                                        |
| --- | ------------------- | --------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| 1   | Licentiebeheerder   | SLS Digital Platforms | De meest frequente gebruiker: dagelijks bezig met licenties en vendor-accounts                                                    |
| 2   | Licentiebeheerder   | SLS Digital Platforms | Tweede perspectief, zodat ik patronen kan herkennen. Als allebei op dezelfde plek vastlopen weet je dat het aan de interface ligt |
| 3   | Finance-medewerker  | PMO / Finance         | Doet de chargeback; gebruikt vooral exports en de kostenoverzichten                                                               |
| 4   | IT-beheerder        | IT Operations         | Voert data-imports uit en beheert gebruikersaccounts. Technischer profiel                                                         |
| 5   | Engineering Manager | DevOps Forge          | Kijkt incidenteel naar de GitHub-kosten per team                                                                                  |

Twee licentiebeheerders is opzettelijk. Als er maar eentje meedoet en die persoon toevallig heel technisch onderlegd is, krijg ik een vertekend beeld. Met twee kan ik beter inschatten of een probleem structureel is of persoonsgebonden.

---

## 6. Testscenario's

### 6.1 Overzicht

Zeven scenario's. Elk gebaseerd op iets dat een licentiebeheerder, finance-medewerker of IT-beheerder in het echt zou doen. Ik heb ze gekoppeld aan SRS-requirements zodat ik later precies kan terugvinden welk usability-probleem bij welke feature hoort.

| Test-ID | Scenario                                                                                                                                                      | Doel                                                                                                            | Verwacht resultaat                                                                                                                                        | SRS-koppeling          |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- |
| UX-01   | **Dashboard raadplegen:** De gebruiker opent het dashboard en moet het totaal actieve licenties, de maandkosten en het tijdstip van de laatste sync benoemen. | Kijken of het overzicht direct leesbaar is en de KPI-kaarten zonder zoeken vindbaar zijn.                       | De gebruiker herkent binnen 30 seconden minimaal drie van de vier kernmetrieken (actieve gebruikers, licentiebenutting, kosten, sync-status) zonder hulp. | M-01, M-02, M-03, S-03 |
| UX-02   | **Persoon zoeken:** De gebruiker krijgt een naam en moet de organisatie, het gekoppelde Atlassian-account en het land van die persoon achterhalen.            | Testen of de zoekfunctionaliteit op `/users` intuïtief is en de `UserDetail`-pagina overzichtelijk.             | Persoon gevonden via zoekbalk, detailpagina geopend, gevraagde info benoemd. Binnen 60 seconden.                                                          | M-10, M-13, S-05, S-06 |
| UX-03   | **CSV importeren:** De gebruiker krijgt een voorbereid CSV-bestand en moet het importeren. Upload, preview bekijken en bevestigen.                            | Checken of de hele import-flow (drie stappen) zonder verwarring doorlopen wordt.                                | Bestand geüpload, preview begrepen (hoeveel nieuw vs. bijgewerkt), import bevestigd, succesmelding gezien.                                                | M-12, S-08             |
| UX-04   | **GitHub-kosten opzoeken:** Vraag: "Hoeveel kost Copilot per maand, en wat is de marge?"                                                                      | Testen of de productkaarten op de GitHub-pagina leesbaar zijn en financiële informatie correct wordt afgelezen. | GitHub-pagina gevonden, Copilot-kaart gevonden, maandkosten en margepercentage correct afgelezen. Binnen 45 seconden.                                     | M-15, M-16, S-14       |
| UX-05   | **Organisatie bekijken:** De gebruiker zoekt een organisatie op en bekijkt de detailpagina met gekoppelde personen.                                           | Checken of de relatie organisatie-personen helder overkomt.                                                     | Organisatie gevonden, detailpagina geopend, aantal personen en kosten benoemd. Binnen 60 seconden.                                                        | M-11, S-07             |
| UX-06   | **Handmatige sync:** De gebruiker moet Atlassian-data vernieuwen en controleren wanneer de laatste sync was.                                                  | Kijken of de refresh-knop vindbaar is.                                                                          | Refresh-knop gevonden, sync getriggerd, bijgewerkt tijdstip correct afgelezen.                                                                            | S-01, S-02, S-03       |
| UX-07   | **Gefilterde export:** Filter de personenlijst op een organisatie en exporteer het resultaat naar CSV.                                                        | Testen of gebruikers begrijpen dat filters de export beinvloeden.                                               | Filter toegepast, exportknop gevonden, CSV gedownload met alleen gefilterde resultaten.                                                                   | S-06, S-11             |

### 6.2 De taakscripts

Bij elk scenario hoort een script dat ik voorlees. De scripts zijn in gewone taal geschreven, expres zonder technische termen. Het idee is dat de testpersoon zelf moet uitvogelen hoe het werkt, net als in de praktijk. Ik stuur ze niet naar de oplossing toe.

**UX-01, dashboard raadplegen**

> _"Je bent net ingelogd op het Operational Insights Dashboard. Bekijk wat je ziet en vertel me: hoeveel actieve licenties zijn er in totaal? Wat zijn de maandkosten? En wanneer zijn de gegevens voor het laatst bijgewerkt?"_

Hier wil ik zien of de KPI-kaarten bovenaan de pagina hun werk doen. Viktor had daar specifiek om gevraagd: het eerste wat een licentiebeheerder wil weten is "hoeveel licenties, hoeveel kost het, en zijn de gegevens actueel?" Die kaarten staan er nu, maar of ze ook echt als eerste opvallen is de vraag.

**UX-02, persoon zoeken**

> _"Je hebt van een collega de naam [Testnaam] gekregen. Zoek deze persoon op. Bij welke organisatie hoort die? Is er een Atlassian-account gekoppeld? En in welk land zit deze persoon?"_

Een ding waar ik benieuwd naar ben: de zoekbalk op `/users` vereist minimaal twee karakters voordat er resultaten verschijnen. Ik heb dat gedaan om de server niet te overbelasten (Equans heeft tienduizenden records in de `persons`-tabel), maar het kan verwarrend zijn als iemand een letter typt en er gebeurt niks. Misschien had ik er een placeholder-tekst bij moeten zetten. Goed, dat is precies iets dat hieruit moet komen.

**UX-03, CSV importeren**

> _"Je hebt van HR een bijgewerkt CSV-bestand ontvangen met nieuwe medewerkers. Importeer het in het systeem en controleer of de gegevens kloppen voordat je bevestigt."_

Dit is de complexste flow in het dashboard. Bestand kiezen, wachten op de preview, preview beoordelen (hoeveel records nieuw, hoeveel bijgewerkt), en dan pas op "bevestigen" klikken. Ik weet uit eerdere informele tests dat het verschil tussen "nieuw" en "bijgewerkt" in de preview niet altijd helder is. Iemand dacht dat "bijgewerkt" betekende dat er iets fout was, terwijl het gewoon betekent dat er al een record bestond dat wordt overschreven. Dat soort misverstanden zijn precies wat ik hier wil opsporen.

**UX-04, GitHub-kosten**

> _"Je manager wil weten: wat zijn de totale maandkosten voor GitHub Copilot? En welke marge berekenen we door? Zoek het op."_

De GitHub-pagina heeft drie productkaarten. Per kaart staat er: actieve gebruikers, purchase price, billable rate, margin, en total costs. Best veel informatie op een klein oppervlak. Ik verwacht dat finance-mensen de marge snel vinden (dat is hun ding), maar dat technische gebruikers eerder naar het aantal seats kijken en de financiële kolommen overslaan. Maar goed, dat is een hypothese. De test moet uitwijzen of dat klopt.

**UX-05, organisatie bekijken**

> _"Je wil weten hoeveel medewerkers bij organisatie [Testorganisatie] zitten en wat de licentiekosten voor dat team zijn. Zoek het op."_

Op de organisatiedetailpagina staan metadata (org_id, naam, land) en een tabel met gekoppelde personen. De koppeling loopt via het `org_id`-veld in de `persons`-tabel. Wat me bij eerdere demo's opviel is dat sommige stakeholders bij het woord "gekoppeld" een hiërarchische relatie verwachtten, alsof de organisatie een manager-team structuur heeft. Dat is het niet. Het is een platte koppeling. Of dat verwarrend overkomt in de UI wil ik hier testen.

**UX-06, handmatige sync**

> _"Je twijfelt of de Atlassian-gegevens actueel zijn. Vernieuw ze handmatig en kijk wanneer de laatste synchronisatie was."_

Eerlijk gezegd weet ik niet zeker of de refresh-knop op een logische plek zit. Hij staat bij de sync-status-indicator, ergens rechts op de pagina. Je moet een beetje weten waar je zoekt. Als drie van de vijf testpersonen hem niet vinden is dat al een signaal. Maar het is ook niet de meest gebruikte feature. De dagelijkse sync draait automatisch, de handmatige trigger is voor als je twijfelt. Vandaar dat ik hier een lagere lat leg (zie sectie 8.3).

**UX-07, gefilterde export**

> _"Je finance-collega heeft een overzicht nodig van alle personen in organisatie [Testorganisatie]. Filter de lijst en exporteer naar CSV."_

Het samenspel tussen filter en export is het interessante hier. De export pakt alleen de gefilterde resultaten. Ik heb dat expres niet in het taakscript vermeld. Ik wil weten of testpersonen dat zelf ontdekken. Sommigen zullen misschien de export downloaden zonder eerst te filteren, het bestand openen, en dan schrikken van de hoeveelheid data. Dat is waardevolle informatie. Misschien moet er een melding komen: "Let op: je exporteert 12.340 rijen. Wil je eerst filteren?"

---

## 7. Meetpunten (Metrics)

### 7.1 Kwantitatieve meetpunten

Ik wil niet alleen op gevoel beoordelen of het goed ging. Vandaar meetbare targets. Tegelijkertijd ken ik de valkuil: puur op cijfers afgaan kan misleidend zijn. Als de Task Success Rate 90% is maar drie van de vijf testpersonen er gefrustreerd bij zaten, dan heb je alsnog werk te doen. Daarom meet ik zowel harde cijfers als zachte observaties.

| Meetpunt              | Wat meet het?                                                                               | Doelwaarde                |
| --------------------- | ------------------------------------------------------------------------------------------- | ------------------------- |
| **Task Success Rate** | Percentage taken dat lukt zonder mijn hulp                                                  | 85% of hoger              |
| **Time on Task**      | Gemiddelde doorlooptijd per taak                                                            | Minder dan 60 seconden    |
| **Error Rate**        | Gemiddeld aantal verkeerde acties per taak (foute klik, verkeerde pagina, misinterpretatie) | Max 1 per taak            |
| **Afgebroken flows**  | Taken waarbij iemand helemaal vastloopt                                                     | Max 1 van de 7 per sessie |
| **SUS-score**         | System Usability Scale, gemiddeld over alle testpersonen (schaal 0-100)                     | 68 of hoger               |

Die 85% heb ik niet zelf bedacht. Dat is een veelgebruikte drempel in usability-literatuur (Nielsen, 1993). De SUS-score van 68 is het gemiddelde over honderden studies, berekend door Sauro (2011). Alles daarboven is bovengemiddeld. Ik had eerst 75 als doel in gedachten, maar Jeroen merkte terecht op dat bij een eerste testcyclus 68 al ambitieus genoeg is. Je wilt een realistische lat leggen, niet eentje waar je sowieso onderdoor gaat.

### 7.2 Kwalitatieve observaties

Naast de getallen schrijf ik tijdens de sessie op wat ik zie en hoor. Dat zit in het observatieformulier. Concreet gaat het om vijf dingen. De opmerkingen en suggesties die testpersonen spontaan geven, zowel positief als negatief. Verwarringspunten, dus de momenten dat iemand stopt, twijfelt of een verkeerde richting inslaat. Navigatieproblemen: klachten of vragen over de sidebar, paginaindeling of knoppen die niet zitten waar ze verwacht worden. Terminologieproblemen, want niet iedereen begrijpt woorden als "billable rate" of "margin" of "GID". En tot slot spontane positieve feedback. Dat is ook nuttig, want het vertelt me wat ik vooral niet moet veranderen.

Dat laatste punt onderschat je snel. Ik had bij een eerdere demo een chart-component aangepast omdat ik dacht dat het beter kon. Achteraf bleek dat Viktor juist die oude versie prettig vond. Als ik positieve feedback had genoteerd had ik dat kunnen voorkomen.

### 7.3 Tijdsdoelen per scenario

Niet elke taak duurt even lang, dat is logisch. Het dashboard aflezen (UX-01) is iets anders dan een CSV importeren (UX-03). Vandaar dat de Time on Task-doelen per scenario verschillen.

| Test-ID | Tijdslimiet | Waar let ik op?                                                                        |
| ------- | ----------- | -------------------------------------------------------------------------------------- |
| UX-01   | < 30 sec    | Vallen de KPI-kaarten op? Weet de gebruiker waar de sync-status staat?                 |
| UX-02   | < 60 sec    | Vindt iemand de zoekbalk meteen? Of gaat diegene eerst scrollen?                       |
| UX-03   | < 120 sec   | Snapt de gebruiker de preview? Wat gebeurt er bij een foutmelding?                     |
| UX-04   | < 45 sec    | Vindt de gebruiker de GitHub-pagina in de sidebar? Leest diegene de marge correct af?  |
| UX-05   | < 60 sec    | Vindt de gebruiker de organisatie-zoekfunctie? Begrijpt diegene de personen-koppeling? |
| UX-06   | < 45 sec    | Kan de gebruiker de refresh-knop vinden? Begrijpt diegene de sync-indicator?           |
| UX-07   | < 90 sec    | Begrijpt de gebruiker dat de filter invloed heeft op de export?                        |

De 120 seconden voor UX-03 lijkt misschien ruim, maar de import-flow heeft echt drie stappen en er zit een wachttijd bij de preview. Als iemand het onder de twee minuten haalt zonder fouten ben ik tevreden.

---

## 8. Acceptatiecriteria

### 8.1 Wanneer is het goed genoeg?

Dit was voor mij een lastige vraag. Wanneer is een dashboard "gebruiksvriendelijk genoeg"? Ik heb het uiteindelijk pragmatisch opgelost met vier drempelwaarden.

| Criterium                    | Drempelwaarde    | Toelichting                                                   |
| ---------------------------- | ---------------- | ------------------------------------------------------------- |
| Gemiddelde Task Success Rate | 85% of hoger     | Over alle scenario's en alle testpersonen                     |
| Gemiddelde SUS-score         | 68 of hoger      | Boven het gemiddelde van vergelijkbare studies (Sauro, 2011)  |
| Kritieke usability-issues    | 0 openstaand     | Geen blokkerende problemen waarbij de taak helemaal niet lukt |
| Ernstige issues              | Max 2 openstaand | Problemen die vertragen maar niet blokkeren                   |

Als het testresultaat onder de 85% Task Success Rate uitkomt, of als er nog een kritiek issue openstaat, dan is het niet geslaagd. Dan moeten die problemen eerst gefixt worden en doe ik een hertest op de scenario's die faalden. Best kans dat dat bij UX-03 (import) of UX-06 (sync) gaat gebeuren, want dat zijn de flows waar ik de meeste onduidelijkheid verwacht.

### 8.2 Issue-classificatie

Ik gebruik vier prioriteitsniveaus, vergelijkbaar met de defectclassificatie uit het Master Test Plan. Niet opnieuw het wiel uitvinden; het development-team werkt al zo.

| Prioriteit         | Wat houdt het in?                                 | Actie                                  |
| ------------------ | ------------------------------------------------- | -------------------------------------- |
| **P1, kritiek**    | Taak lukt niet. Blokkerend.                       | Moet gefixt worden voor oplevering     |
| **P2, ernstig**    | Taak lukt wel, maar met veel moeite of verwarring | Fixen of beargumenteerd accepteren     |
| **P3, gemiddeld**  | Lichte irritatie, maar de taak wordt afgerond     | Aanbevolen te fixen, mag doorgeschoven |
| **P4, cosmetisch** | Kleine visuele of tekstuele dingetjes             | Backlog, als er tijd over is           |

Het verschil tussen P1 en P2 is of de testpersoon de taak uberhaupt afkrijgt. Als iemand de refresh-knop niet vindt en uiteindelijk opgeeft, is dat P1. Als iemand er twee minuten over doet maar het uiteindelijk lukt, is dat P2. Dat onderscheid is voor de prioritering best nuttig.

### 8.3 Drempels per scenario

| Test-ID | Minimaal slagingspercentage | Toelichting                                 |
| ------- | --------------------------- | ------------------------------------------- |
| UX-01   | 80% (4 van 5)               | Startscherm, moet direct leesbaar zijn      |
| UX-02   | 80% (4 van 5)               | Dagelijkse handeling voor beheerders        |
| UX-03   | 80% (4 van 5)               | Complex maar onmisbaar                      |
| UX-04   | 80% (4 van 5)               | Fouten hier leiden tot verkeerde chargeback |
| UX-05   | 80% (4 van 5)               | Basis van het chargeback-proces             |
| UX-06   | 60% (3 van 5)               | Wordt minder vaak gebruikt                  |
| UX-07   | 80% (4 van 5)               | Dagelijks voor finance                      |

UX-06 heeft bewust een lagere lat. De handmatige sync is iets dat misschien eens per week wordt gebruikt, de dagelijkse sync draait automatisch. Als drie van de vijf de knop vinden is dat oké, mits de twee die het niet vonden na een hint wel begrijpen waar het zit. Dat noteer ik dan als P3 in plaats van P1.

---

## 9. Output en rapportage

### 9.1 Hoe leg ik de resultaten vast?

Drie niveaus. De ruwe data zit in observatieformulieren die ik per sessie invul: timestamps, fouten, opmerkingen, en alles wat de testpersoon hardop zei tijdens het think-aloud-gedeelte. Dat schrijf ik live mee, dus het is soms wat rommelig. Niet erg, want ik neem de sessies ook op via Teams of OBS. Die opnames bewaar ik zodat ik achteraf momenten kan terugkijken die me tijdens de sessie zijn ontgaan. Je mist altijd wel wat als je tegelijk observeert, noteert en op de stopwatch let.

De kwantitatieve data (Task Success Rate, Time on Task, Error Rate, SUS-scores per persoon) verwerk ik daarna in een Excel-sheet. Dat geeft me de harde cijfers om naast de drempelwaarden uit sectie 8 te leggen.

### 9.2 Wat lever ik op?

Na alle drie de sessies maak ik een usability test rapport. Dat is het hoofddocument: alle bevindingen per scenario, patronen over meerdere testpersonen, en mijn aanbevelingen. Dat gaat naar Viktor, Brian en Jeroen.

Daarnaast een issue-register. Dat is een lijst van alle usability-problemen met hun prioriteit (P1 tot P4). Die lijst dient direct als input voor de product backlog. Verder een SUS-scoreoverzicht met individuele en gemiddelde scores, vergeleken tegen de benchmark van 68. Een verbeterpuntenlijst met concrete UX-aanpassingen die ik aanbeveel. En de sessie-opnamen zelf, geanonimiseerd en alleen als de testpersoon daar toestemming voor heeft gegeven. Die sla ik intern op als onderbouwing. Mocht er discussie ontstaan over een bevinding, dan kan ik het fragment erbij pakken.

### 9.3 En dan? Hoe gaat het verder?

Een rapport schrijven is leuk, maar het gaat om wat je ermee doet. De prioritering van verbeterpunten loopt langs vier vragen. Hoe erg is het? P1 en P2 gaan als eerste. Hoe vaak kwam het voor? Een probleem dat bij vier van de vijf testpersonen optreedt weegt zwaarder dan iets dat maar bij eentje voorkwam. Zit het in een Must Have-feature of een Should Have? M-xx requirements gaan voor op S-xx. En tot slot: hoeveel werk is de fix? Als het een kwestie is van een label aanpassen kost dat een half uur. Als de hele import-flow op de schop moet is dat een sprint.

Alle gevonden issues maak ik aan als GitHub Issues in de projectrepository, met het label `usability` en een verwijzing naar het testscenario (UX-01 tot en met UX-07). P1- en P2-issues gaan de eerstvolgende sprint in. P3 en P4 in de backlog. Na het fixen van een issue voer ik het betreffende scenario opnieuw uit om te controleren of het probleem echt weg is. Want dat is iets dat ik bij unit tests heb geleerd: een fix die niet gevalideerd is, is geen fix.

---

## 10. Referenties

| Bron                                | Referentie                                                                                             |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------ |
| Brooke, J. (1996)                   | SUS: A 'quick and dirty' usability scale. _Usability Evaluation in Industry_, 189-194.                 |
| ISO 9241-11 (2018)                  | Ergonomics of human-system interaction, Part 11: Usability: Definitions and concepts.                  |
| Nielsen, J. (1993)                  | _Usability Engineering_. Academic Press.                                                               |
| Nielsen, J. & Landauer, T.K. (1993) | A mathematical model of the finding of usability problems. _Proceedings of ACM INTERCHI '93_, 206-213. |
| Olsen, T. et al. (2021)             | Referentie gebruikt in Master Test Plan (MTP-001) voor bruikbaarheidscriteria.                         |
| Sauro, J. (2011)                    | _A Practical Guide to the System Usability Scale_. Measuring Usability LLC.                            |
| Master Test Plan (MTP-001)          | Equans Operational Insights Dashboard, Master Test Plan, sectie 4.6.                                   |
| Software Requirements Specification | Equans Operational Insights Dashboard, SRS (functionele en technische requirements).                   |
