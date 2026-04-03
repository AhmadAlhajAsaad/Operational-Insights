# Onderzoek naar gebruikersbehoeften

- **Project:** Equans Operational Insights Dashboard
- **Studentnaam:** Ahmad Alhaj Asaad
- **Datum:** 15–12-2025
- **Studentnummer:** 1035912
- **Opleiding:** HBO-ICT Informatica, Hogeschool Rotterdam
- **Opdrachtgever:** Equans Nederland — SLS Digital Platforms
- **Bedrijfsbegeleider:** Viktor Klein
- **Technisch begeleider:** Brian Veltman
- **Schoolsbegeleider:** Jeroen Boogaard

---

## Inhoudsopgave

- [Onderzoek naar gebruikersbehoeften](#onderzoek-naar-gebruikersbehoeften)
- [1. Inleiding](#1-inleiding)
- [2. Onderzoeksmethode](#2-onderzoeksmethode)
- [3. Vragenlijst voor Operational Insights Dashboard Requirements](#3-vragenlijst-voor-operational-insights-dashboard-requirements)
- [4. Interviewverslag met Viktor Klein](#4-interviewverslag-met-viktor-klein)
- [5. Interviewverslag – Brian Veltman](#5-interviewverslag--brian-veltman)
- [6. Interviewverslag – Henk Soppe](#6-interviewverslag--henk-soppe)
- [7. Conclusie](#7-conclusie)

---

## 1. Inleiding

Dit onderzoek naar gebruikersbehoeften is uitgevoerd om inzicht te krijgen in de eisen, wensen en verwachtingen rondom het te ontwikkelen Operational Insights Dashboard voor Equans. Dit dashboard moet inzichten bieden in het gebruik van DevOps-tools (zoals Jira, Confluence, GitHub, Trello en JFrog), de bijbehorende licentiekosten en de interne doorbelasting (chargeback). Daarnaast moet het dashboard inzicht geven in de waarde die deze tools opleveren, zodat Equans betere financiële beslissingen kan nemen en accurater kan factureren.

In dit onderzoek zijn drie stakeholders geïnterviewd: de Product Lead, een externe developer van het DevOps Experience Platform en de Director SLS. De inzichten uit deze interviews vormen de basis voor de functionele en technische eisen van het POC-dashboard.

---

## 2. Onderzoeksmethode

Voor het verzamelen van gebruikersbehoeften is gebruikgemaakt van kwalitatief onderzoek in de vorm van semigestructureerde interviews. Deze methode is gekozen omdat een dashboard als dit sterk afhankelijk is van de behoeften van verschillende rollen binnen de organisatie en omdat inzicht nodig is in de huidige problemen en processen.

**Methode:** Semigestructureerde interviews (1-op-1), gebaseerd op vooraf opgestelde vragenlijsten gericht op:

- Businesscontext
- Functionele requirements
- Technische requirements
- Gebruiksscenario's en flows
- KPI- en databehoeften

**Aantal deelnemers:** 3

**Doelgroep:** Stakeholders binnen SLS Digital Platforms die direct betrokken zijn bij financiële processen, licentiebeheer en product ownership:

- Product Lead DevOps Forge
- Extern Developer DevOps Experience Platform
- Director SLS

**Duur:** ±30 minuten per deelnemer

**Doel:** Het doel van dit onderzoek is het verzamelen van duidelijke, bruikbare requirements voor de Proof of Concept van het Operational Insights Dashboard. De interviews moeten inzicht geven in:

- Welke informatie het dashboard moet tonen
- Welke KPI's belangrijk zijn voor correcte chargeback
- Welke data nodig is en op welk detailniveau
- Hoe gebruikers willen dat informatie wordt weergegeven
- Welke problemen het dashboard moet oplossen
- Hoe de gebruikersflow eruit moet zien

---

## 3. Vragenlijst voor Operational Insights Dashboard Requirements

### A. Vragen om de business context te begrijpen

1. Wat is het belangrijkste doel van dit dashboard voor jullie afdeling?
2. Waarom hebben jullie dit dashboard nodig? Welk probleem lost het op?
3. Wie gaat dit dashboard het meest gebruiken?
4. Hoe worden deze inzichten momenteel verzameld?

### B. Vragen voor het verzamelen van functionele requirements

1. Welke vragen willen jullie dat het dashboard kan beantwoorden?
2. Welke (Key Performance Indicators) KPI's zijn het belangrijkst voor jullie?
   - Active users
   - License utilization
   - Kosten per team
3. Welke data moeten we uit elke tool kunnen ophalen (Jira, Confluence, GitHub, Trello, JFrog)?
4. Op welk detailniveau hebben jullie data nodig?
   - Per user
   - Per maand/week/dag
5. Hoe vaak moet de data geüpdatet worden (real-time, dagelijks, wekelijks)?
6. Hoe willen jullie dat deze informatie wordt weergegeven? Grafieken? Tabellen? Alerts?
7. Welke grafieken zijn het meest logisch voor jullie?
8. Wat moet direct zichtbaar zijn in één oogopslag?
9. Op welke manieren moeten gebruikers data kunnen filteren? Team, periode, project?
10. Moeten gebruikers zelf kolommen kunnen kiezen?
11. Willen jullie exportfuncties? Zo ja, in welke formaten? CSV, Excel, PDF?
12. Moet er een optie komen voor geplande rapportages?

### C. Vragen om technische requirements te verzamelen

1. Wat is een acceptabele laadtijd voor dashboards en grafieken?
2. Hoeveel gebruikers verwachten jullie dat tegelijkertijd gebruik maken van het dashboard?
3. Moet data geanonimiseerd worden?
4. Welke authenticatiemethode moet worden gebruikt?
5. Moet de interface mobiel/tabletvriendelijk zijn?
6. Zijn er bepaalde UI/UX richtlijnen die ik moet volgen van SLS Digital Platforms?

### D. Vragen voor het maken van Use Cases

1. Welke acties moet een gebruiker kunnen uitvoeren in het dashboard?
2. Wat is voor een gebruiker het belangrijkste scenario?
3. Wat is de flow die een gebruiker doorloopt van begin tot eind?

### E. Vragen voor User Stories (Agile stijl)

1. Als we een user story schrijven, wie is de "gebruiker"?
2. Wat wil deze gebruiker bereiken in het dashboard?

---

## 4. Interviewverslag met Viktor Klein

**Datum interview:** 24-11-2025
**Interviewpartner:** Viktor Klein (Product Lead DevOps Forge, Equans)
**Interviewer:** Ahmad Alhaj Asaad

### Doel van het dashboard

Het Operational Insights Dashboard wordt ontwikkeld om financiële inzichten en chargeback-data te leveren over de DevOps-tools binnen Equans (Jira, Confluence, GitHub, JFrog, Trello, enz.).

De afdeling moet elke drie maanden een chargeback uitvoeren: kosten worden doorbelast aan business units (BU's) op basis van hun licentiegebruik. Het dashboard moet helpen om:

- beter te voorspellen hoeveel kosten eraan komen,
- historische en actuele licentiekosten te vergelijken, en
- verwachte inkomsten vs. uitgaven te berekenen voor toekomstige periodes (forecast).

> **Kort:** Het dashboard is primair een financieel hulpmiddel, niet een gebruikersanalyse-tool.

### Gebruikers van het dashboard

| Rol                             | Gebruik                                        |
| ------------------------------- | ---------------------------------------------- |
| Product Owner & Director (SLS)  | Startfase: beslissingen en controle            |
| PMO (Project Management Office) | Eindverantwoordelijke chargeback-administratie |
| Business Units (indirect)       | Ontvangen rapportages                          |

> **Opmerking van Viktor:** PMO moet uiteindelijk zelfstandig met het dashboard kunnen werken.

### Informatie & KPI's

#### Belangrijkste vragen die het dashboard moet beantwoorden

- Wat zijn de totale licentiekosten per team en per gebruiker?
- Wat zijn de actuele en historische kosten?
- Hoe verhouden werkelijke kosten zich tot voorspelde kosten?
- Welke gebruikers zijn billable, en welke niet actief maar wel facturabel?

#### Belangrijkste KPI's volgens Viktor

| KPI                            | Omschrijving                                              |
| ------------------------------ | --------------------------------------------------------- |
| License Utilization            | Hoeveel licenties worden daadwerkelijk gebruikt?          |
| Kosten per gebruiker           | Kosten op basis van billable user per maand               |
| Kosten per team/BU             | Chargeback per afdeling                                   |
| Kosten historisch vs. forecast | Vergelijking tussen daadwerkelijke kosten en voorspelling |

#### Databehoefte per tool

| Tool                               | Informatie                                                      |
| ---------------------------------- | --------------------------------------------------------------- |
| Atlassian (Jira/Confluence/Trello) | Actieve/billable users, licentieprijzen, historische activiteit |
| GitHub                             | Copilot-verbruik → heeft mogelijk extra kosten per gebruiker    |
| JFrog                              | Niet per user maar per site/storage, dus andere berekening      |
| Andere                             | Moet bekeken worden of data via API te verkrijgen is            |

> **Let op:** Sommige data is niet op gebruikersniveau (zoals JFrog-storage of Copilot-metered usage).

#### Gewenst detailniveau

| Aspect     | Detail                                           |
| ---------- | ------------------------------------------------ |
| Per user   | Ja                                               |
| Per maand  | Ja, want licenties worden per maand gefactureerd |
| Activiteit | Alleen "actief in de maand" is genoeg            |

> **Voorbeeld van Viktor:** ook als iemand slechts 1 dag actief is, moet er voor de hele maand worden betaald.

**Frequentie van updates:** Eén keer per maand is voldoende, maar tijdens analyse wil men soms "live inzicht".

> **Conclusie:** maandelijkse refresh + mogelijkheid tot handmatige refresh.

### Visualisatie & UI-behoeften

#### Wat moet zichtbaar zijn?

| Element                                      | Waarom?                  |
| -------------------------------------------- | ------------------------ |
| Totale licentiekosten (actueel & historisch) | Kern van dashboard       |
| Vergelijking: forecast vs. realisatie        | Voorspelling + correctie |
| Grafieken + waardes in één oogopslag         | High-level beslissingen  |

> Aantal gebruikers is minder belangrijk → kosten zijn belangrijker.

#### Gewenste grafieken

- **Lijngrafiek:** historische vs. actuele kosten
- **Staafgrafiek:** kosten per team/BU
- **Alerts:** bij overschrijding van licentiecapaciteit

#### Filtering

| Filter  | Opmerking       |
| ------- | --------------- |
| Product | Belangrijkste   |
| Periode | Maand           |
| Team    | Chargeback BU's |

### Export & Rapportages

- CSV verplicht, Excel optioneel, PDF niet nodig
- Moet gepland kunnen worden via API → naar PMO-systeem

### Technische eisen

| Aspect         | Eisen                                                 |
| -------------- | ----------------------------------------------------- |
| Performance    | < 2 seconden laadtijd                                 |
| Gebruikers     | ~2 tegelijk                                           |
| Privacy        | GDPR: geen persoonlijke data tonen                    |
| Authenticatie  | Nog te bepalen, maar achter login                     |
| Device         | Desktop only, mobiel/tablet niet noodzakelijk         |
| UI-richtlijnen | SLS Design Guidelines volgen (Confluence beschikbaar) |

### Use Case / Scenario

> Als PMO-medewerker wil ik kosten per maand kunnen vergelijken met de forecast, zodat ik de chargeback kan uitvoeren en afwijkingen kan rapporteren.

#### Flow

1. Inloggen
2. Dashboard opent direct met huidige kosten
3. Filteren op periode/product/team
4. Vergelijking historisch ↔ forecast
5. Export als CSV
6. CSV wordt doorgestuurd naar PMO

### Samenvatting

Het dashboard moet een financieel hulpmiddel worden dat licentiekosten per maand, per product en per team inzichtelijk maakt en historische kosten vergelijkt met toekomstige voorspellingen.

---

## 5. Interviewverslag – Brian Veltman

**Interviewpartner:** Brian Veltman – Extern Developer DevOps Experience Platform
**Datum interview:** 25-11-2025
**Interviewer:** Ahmad Alhaj Asaad
**Project:** Proof of Concept – Operational Insights Dashboard (Equans, SLS Digital Platforms)

### Doel van het dashboard

Brian beschrijft dat het belangrijkste doel van het dashboard is om financieel inzicht te krijgen in de DevOps-tools die Equans levert (o.a. Jira, Confluence, GitHub, Trello, JFrog). Het dashboard moet duidelijk maken:

- Wat kosten de producten die Equans afneemt?
- Wat leveren deze producten op aan interne facturatie?
- Wordt er winst of verlies gemaakt per product?

### Waarom is dit dashboard nodig?

Het dashboard moet inzicht geven in kosten, omzet en winst, op basis van licenties vs. gebruik.

Momenteel worden inzichten handmatig verzameld via Excel-bestanden en exports uit Palantir. Dit zorgt voor problemen:

| Huidige situatie                                  | Probleem                    |
| ------------------------------------------------- | --------------------------- |
| Handmatig Excel werk                              | Tijdrovend & foutgevoelig   |
| Data wordt maar eens per 3 maanden bijgewerkt     | Geen actuele inzichten      |
| Licenties worden niet altijd correct gefactureerd | Financieel risico & verlies |

Een geautomatiseerd dashboard lost deze problemen op en levert real-time of dagelijkse inzichten.

### Voor wie is het dashboard bedoeld?

**Belangrijkste gebruikers:**

- Business stakeholders: Viktor (Product Lead) - Henk (Director SLS Digital Platforms)
- Secundaire gebruikers: Supportteam in India, om sneller te zien welke gebruikers welke tools gebruiken.

### Belangrijkste vragen die het dashboard moet beantwoorden

Brian noemt vooral financiële en facturatie gerelateerde vragen:

- Verdienen wij geld met deze tools of maken we verlies?
- Hoeveel licenties nemen we af vs. hoeveel worden er verkocht?
- Zijn de gebruikers die wij factureren ook daadwerkelijk actief?
- Zijn er licenties die we betalen maar niemand gebruikt?

> **Kernvraag:** Zijn onze gebruikers "billable"? Dat betekent: hebben we genoeg gegevens om ze te factureren?

### Belangrijkste KPI's (Key Performance Indicators)

| KPI                                 | Betekenis                                                         |
| ----------------------------------- | ----------------------------------------------------------------- |
| License Utilization %               | Hoeveel % van de gekochte licenties is in gebruik?                |
| Actieve gebruikers (per facturatie) | Hoeveel licenties zijn verkocht én worden daadwerkelijk gebruikt? |
| Kosten vs. opbrengsten per tool     | Wordt er winst gemaakt?                                           |
| Billable users %                    | Hoeveel gebruikers kunnen echt worden gefactureerd?               |

> **Brian benadrukt** dat minimaal 90–95% licentiegebruik noodzakelijk is om winstgevend te blijven.

### Welke data moet worden opgehaald?

**Per gebruiker:**

- Tot welke tools heeft de gebruiker toegang? (Jira/Confluence/GitHub/Trello/JFrog)
- Wanneer heeft de gebruiker toegang gekregen?
- Is de gebruiker actief?
- Tot welke interne organisatie behoort de gebruiker? _(Deze informatie komt uit Palantir, niet uit Atlassian/GitHub.)_

> **Toegang + organisatie + activiteit = billable.**

### Detailniveau & frequentie

| Aspect           | Specificatie                                            |
| ---------------- | ------------------------------------------------------- |
| Detailniveau     | Per gebruiker                                           |
| Tijdseenheid     | Per maand (gebruik = facturatie)                        |
| Updatefrequentie | Dagelijks is voldoende (real-time mag, maar hoeft niet) |

### Visualisatie & UI-wensen

- Tabellen zijn verplicht, omdat business gewend is aan Excel.
- Grafieken, alerts en progress bars zijn optioneel maar gewenst, bijvoorbeeld:
  - Bar/progress indicator: actieve vs. inactieve gebruikers
  - Alert: "Overbodige licenties gedetecteerd"
  - Trendgrafiek: licentiegebruik over tijd

> **Belangrijkste eis:** Één oogopslag moet laten zien of facturatie klopt.

### Security & privacy

- Data moet geanonimiseerd worden (namen, e-mails, IDs waar mogelijk).
- Authenticatie voor dashboard: SSO via Entra ID (Microsoft).

### Data & systeem

| Onderwerp               | Antwoord                                                    |
| ----------------------- | ----------------------------------------------------------- |
| Data ophalen via?       | REST API's (OAuth of API-tokens)                            |
| Centrale datawarehouse? | Nee, data moeten rechtstreeks uit de tools worden opgehaald |
| Datavolume              | ~20.000 records (<5 MB)                                     |
| Exports                 | Liefst: Excel/CSV én via API terug naar Palantir            |

### Conclusie van het interview

Het Operational Insights Dashboard moet financiële en gebruikersinzichten automatiseren binnen Equans. De grootste uitdaging ligt in:

- Business definities verduidelijken
- Gegevens combineren uit meerdere bronnen
- Factureerbaarheid bepalen ("billable users")

De technische ontwikkeling (React UI) is pas haalbaar nadat de requirements en datadefinities volledig zijn opgehelderd.

---

## 6. Interviewverslag – Henk Soppe

**Onderwerp:** Requirements voor het financieel/licentie-dashboard (Operational Insights Dashboard)
**Geïnterviewde:** Henk Soppe – Director SLS
**Interviewer:** Ahmad Alhaj Asaad
**Datum:** 27-11-2025

### Doel en context van het dashboard

Het dashboard is primair een financieel dashboard. SLS levert diensten via verschillende platformen (Jira, Confluence, Trello, GitHub, etc.). Gebruiksorganisaties betalen op basis van licenties en functionaliteiten. Daarom is het cruciaal dat wij:

- Accurate en tijdige financiële informatie leveren over licenties en kosten
- Deze informatie gebruiken voor interne doorbelasting (chargeback)
- Klanten in staat stellen hun eigen klanten correct door te belasten

> Het dashboard is een fundament: als "stromend water" moet het altijd beschikbaar, betrouwbaar en actueel zijn.

### Waarom is dit dashboard nodig? (Probleem/huidige situatie)

Op dit moment gebeurt veel handmatig:

- Licenties worden bijgehouden op basis van uitgifte
- Veel handwerk leidt tot fouten
- Fouten zorgen voor extra gesprekken, correcties en kostbare tijd

Daarnaast:

- Uitputting van contracten monitoren (lopen we uit onze licenties?)
- Kosten goed kunnen doorbelasten naar interne klanten
- Discussies over facturen verminderen door transparante en controleerbare data

> **Kortom:** het dashboard moet inzicht geven in kosten, opbrengsten, licentiegebruik en facturatie, op een betrouwbare manier.

### Gebruikers van het dashboard

Er zijn twee hoofdgroepen:

1. **Interne klanten/business units:** Voor doorbelasting aan hun eigen eindklanten
2. **SLS zelf:** Voor contractbeheer, licentie-uitputting, gebruikstrends en verbetervoorstellen

Het dashboard is dus voor operationeel financieel beheer én optimalisatie van platformgebruik.

### Gewenste inzichten, data en KPI's

#### Belangrijke inzichten

- Hoeveel licenties zijn er per product in gebruik?
- Wie gebruikt welke licentie?
- Hoe vaak worden tools gebruikt? (dagelijks, wekelijks, maandelijks)
- Wat zijn de kosten per systeem en per product?
- Benutten we licenties efficiënt?

#### Belangrijke KPI's

- Kosten per systeem/product
- Totaal aantal licenties per product
- Licentiegebruik per organisatie/business unit
- Trend in gebruik over de tijd

> **Kosten per product en trends** zijn nu het belangrijkst.

### Gewenste data per tool

Per tool (Jira, Confluence, Trello, GitHub, etc.):

- Wie een licentie heeft (actieve gebruikers)
- Of en hoe vaak het product wordt gebruikt (dagelijks, wekelijks, maandelijks)

Kosten zijn gekoppeld aan het hebben van een licentie, niet aan daadwerkelijk gebruik.

Gebruiksdata is belangrijk voor:

- Efficiëntie van licentie-inzet beoordelen
- Zien of iemand een licentie heeft maar het product nauwelijks gebruikt

### Niveaus van detail & updatefrequentie

**Detailniveau:**

- Minimaal per gebruiker
- Daarnaast per organisatie/business unit, project, tijdseenheid (dag, week, maand)

**Update-frequentie:**

- Ideaal: real-time
- Praktisch: dagelijks of wekelijks is voldoende
- Belangrijk: data moet actueel en betrouwbaar zijn bij facturatie/analyses

### Visualisatie en presentatie van de data

- Grafische weergave voor snel overzicht (bijv. staafdiagrammen per product/organisatie)
- Tabellen als onderliggende laag voor analyses
- Per systeem een duidelijke grafiek van: Aantal licenties in gebruik - bijbehorende kosten

Bij binnenkomst direct overzicht van:

- Aantal licenties per product
- Kosten per product
- Gefilterd op business unit, organisatie-ID of project

### Filtering, export en historische data

**Filters:** Team/business unit - periode (maand, kwartaal, jaar) - gebruiker – project - actief/inactief

**Export:** Export naar Excel gewenst

**Historische data:** Bewaard worden en trends over meerdere jaren zijn relevant

### Technische eisen (Performance, Security, Privacy)

**Performance:**

- Laadtijd: Binnen ±1 seconde een zichtbare reactie
- Aantal gelijktijdige gebruikers: Tot 25 bij piekmomenten

**Privacy & anonimiteit (AVG):**

- Algemene dashboards: geanonimiseerde data
- Doorbelasting: persoonsgebonden data alleen zichtbaar voor bevoegden

**Authenticatie:**

- Single Sign-On (SSO) via Microsoft/Equans accounts
- Voldoen aan Equans security policy

### UI-richtlijnen, flow en hergebruik

- Geen strikte UI-richtlijnen
- Wel kijken naar bestaande dashboards en stijlen

**Gebruikersflow:**

1. Inloggen via SSO
2. Eerst hoog-over overzicht (per business unit, aantal licenties, totale kosten)
3. Daarna doorklikken naar projecten, sub-entiteiten, detailniveau

### Aanvullende opmerkingen van Henk

- Kijk wat al beschikbaar is aan dashboards
- Hergebruik waar mogelijk, voeg waarde toe waar nodig
- Maak ook trend-dashboards: niet alleen "wie heeft een licentie?", maar ook "hoe vaak wordt het gebruikt?"
- Dit zegt veel over de efficiëntie van licentiegebruik

---

## 7. Conclusie

Uit het onderzoek blijkt duidelijk dat het Operational Insights Dashboard een financieel en operationeel hulpmiddel moet worden dat Equans ondersteunt bij het beheren van licenties, het doorbelasten van kosten en het monitoren van gebruikstrends. Alle drie de geïnterviewden benadrukken dat het dashboard moet helpen om fouten te verminderen, processen te automatiseren en betrouwbare inzichten te leveren over kosten, opbrengsten en licentiegebruik.

**Belangrijke conclusies zijn:**

- **Primair doel:** financieel inzicht en correcte interne doorbelasting (chargeback).
- **Kernbehoefte:** inzicht in licentiekosten, gebruiksactiviteit en factureerbare gebruikers ("billable users").
- **Data:** moet per gebruiker, per product en per maand beschikbaar zijn.
- **Visualisatie:** combinatie van grafieken en tabellen, met directe inzichten in één oogopslag.
- **Technische eisen:** snelle laadtijd (<2 sec), SSO-authenticatie, GDPR-proof dataverwerking, en betrouwbare maandelijkse/dagelijkse updates.
- **Businesswaarde:** minder fouten, minder handmatig werk, actuelere data en beter inzicht in winst/verlies per DevOps-tool.

Het dashboard moet daarmee een centraal instrument worden dat de financiële processen binnen SLS ondersteunt en optimaliseert, en dat de basis legt voor een schaalbaar en toekomstbestendig licentie- en kostenbeheer.
