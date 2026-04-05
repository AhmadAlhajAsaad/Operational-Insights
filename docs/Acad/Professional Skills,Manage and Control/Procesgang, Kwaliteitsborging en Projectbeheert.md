**Procesgang, Kwaliteitsborging en Projectbeheer**

**Equans Operational Insights Dashboard**

- Versie: 1.0
- Schoolsbegeleider: Jeroen Boogaard
- Bedrijfsbegeleider: Viktor Klein (De Product Owner)
- Technisch begeleider: Brian Veltman
- Studentnaam: Ahmad Alhaj Asaad
- Instelling: Equans, SLS Digital Platforms / DevOps Forge
- Studiejaar: 2025 - 2026

---

## 1. Inleiding

In dit hoofdstuk beschrijf ik hoe ik tijdens de uitvoeringsfase van het afstudeerproject te werk ben gegaan. Het gaat dan om het wijzigingsbeheer, de kwaliteitsborging van de code en de manier waarop ik het project heb georganiseerd. Wat hier staat is gebaseerd op hoe ik het daadwerkelijk heb aangepakt, niet op hoe het in theorie zou moeten. De concrete bewijsstukken liggen vast in Jira en Confluence, dus alles is terug te vinden.

## 2. Ontwikkelwerkwijze en wijzigingsbeheer

Het project heb ik grotendeels zelfstandig uitgevoerd, maar dat betekent niet dat ik zonder structuur heb gewerkt. Juist bij een soloproject merk je hoe snel het een rommeltje wordt als je geen duidelijke werkwijze hanteert. Daarom heb ik vanaf het begin gekozen voor een featuregerichte branchstrategie: elke functionaliteit kreeg een eigen feature branch, vernoemd naar het bijbehorende Jira-issue (bijvoorbeeld SDPDOFS-546-ophalen-van-gebruikerslijst). Hierdoor bleef de main branch altijd in een werkende staat.

Elke commit bevatte een afgebakende functionele toevoeging of technische verbetering. Bij het schrijven van commitberichten heb ik er bewust voor gezorgd dat het Jira-issuenummer (PAN-nummer) erin stond. Dit klinkt misschien als een klein detail, maar in de praktijk bleek dit enorm waardevol. Als ik weken later terugkeek naar een bepaalde wijziging, kon ik via het commitbericht direct terugvinden waarom ik die aanpassing had gemaakt en wat het oorspronkelijke Jira-ticket was. Zonder die koppeling was dat een stuk lastiger geweest.

Figuur 1 in de projectdocumentatie toont een overzicht van afgeronde taken in Confluence.

Voltooide taken en documenten heb ik bijgehouden in Confluence, dat als centrale kennisbank voor het project diende. Afgeronde werkitems werden voorzien van reviewcommentaar voordat ik ze samenvoegde in de main branch. Confluence bleek hierbij een goed middel om niet alleen code, maar ook beslissingen en overwegingen vast te leggen.

Figuur 2 geeft een gedetailleerde beschrijving van taken in Jira inclusief bijbehorende acceptatiecriteria.

## 3. Kwaliteitsbewaking en code review

Tijdens het ontwikkelen heb ik informele code reviews gedaan samen met mijn technisch begeleider Brian Veltman. Voordat ik nieuwe functionaliteiten samenvoegde in de main branch, besprak ik ze eerst inhoudelijk. Hierbij lette ik op een aantal dingen die ik gaandeweg als belangrijk ben gaan beschouwen.

Ten eerste de architectuurconformiteit: houdt de code zich aan de drielegige REST API-architectuur en is de scheiding tussen frontend en backend strikt? Dit was iets waar ik in het begin niet altijd scherp genoeg op was. Soms schreef ik logica in de frontend die eigenlijk in de backend thuishoorde, en dat viel bij de reviews op. Ten tweede de foutafhandeling: in de Rust-backend heb ik consequent `Result<T, E>` gebruikt en `unwrap()` vermeden in productiecode. Dat kost meer moeite bij het schrijven, maar voorkomt dat het systeem onverwacht crasht. Daarnaast lette ik op UI-prestaties (lazy loading voor lijstweergaven, ondersteuning van lege toestanden) en gebruikersfeedback (optimistic UI met pull-to-refresh als fallback).

De feedback die ik kreeg heb ik steeds verwerkt in de volgende commits. Voorafgaand aan elke push naar GitHub voegde ik opmerkingen toe aan de voltooide taak in Jira, zodat duidelijk was wat er was gewijzigd en waarom.

Figuur 3 illustreert hoe duidelijke opmerkingen worden toegevoegd aan voltooide taken voordat ze naar GitHub worden gepusht.

## 4. Issue tracking en traceerbaarheid

Voor de planning en bewaking van mijn werkzaamheden heb ik Jira gebruikt als issue tracker (projectcode: SDPDOFS). Alle taken, user stories en technische verbeteringen staan als losse issues geregistreerd en zijn verdeeld over sprints. Wat ik al vrij snel merkte is dat grote user stories lastig te overzien zijn als je ze als een geheel probeert op te pakken. Daarom heb ik ze systematisch opgesplitst in kleinere subtaken. Dat maakte het niet alleen makkelijker om de voortgang bij te houden, maar ook om na afronding gerichte opmerkingen of beoordelingen toe te voegen.

Figuur 4 laat zien hoe Jira grote user stories en taken opsplitst in kleinere, beter beheersbare onderdelen.

Figuur 5 bevat een lijst van taken die nog moeten worden afgerond, waarvan een deel reeds is voltooid.

De traceerbaarheid van het werk is via meerdere artefacten aantoonbaar:

| Artefact | Beschrijving |
| --- | --- |
| Jira-issues | Status Done of Closed per afgeronde taak |
| GitHub-commits | Commitberichten met verwijzing naar Jira PAN-nummer |
| Feature branches | Een branch per functionaliteit, gekoppeld aan het Jira-issue |
| Confluence-documenten | Alle documenten zijn geverifieerd en vastgelegd in Confluence |
| Openstaande taken | Figuur 5 toont resterende taken; een deel hiervan is inmiddels voltooid |

Figuur 6 toont een volledig overzicht van door de auteur opgestelde en in Confluence geverifieerde documenten.

Figuur 7 biedt een voorbeeld sprintoverzicht in Jira met alle opgenomen en afgeronde issues.

## 5. Teamcommunicatie en samenwerking

Het project vond plaats binnen het DevOps Forge-team bij Equans SLS Digital Platforms. Elke ochtend was er een stand-up waarin we de voortgang, blokkades en de planning voor de dag bespraken. Bij deze stand-ups waren onder anderen Viktor Klein, de product owner en mijn bedrijfsbegeleider, en Brian Veltman, mijn technisch begeleider, aanwezig. Samen bespraken we de voortgang van het project en eventuele obstakels. In het begin voelde dat voor mij wat onwennig, omdat ik als stagiair tussen ervaren developers zat. Maar gaandeweg merkte ik dat juist die dagelijkse afstemming hielp om op koers te blijven. Als ik ergens vastliep kon ik dat direct benoemen, en vaak had iemand uit het team al een idee voor een oplossing.

Figuur 8 DevOpsForge Kanban Board voor teams

Op het DevOpsForge Kanban-bord (Figuur 8) waren alle teamtaken per teamlid zichtbaar. Mijn bijdrage aan het project stond daar ook op en werd dagelijks bijgewerkt. Aan elke taak die code-implementatie vereiste heb ik een aparte branch en een koppeling naar het GitHub-repository toegekend (zie Figuur 9). Hierdoor was de hele ontwikkelcyclus, van taakdefinitie tot code-integratie, volledig herleidbaar.

Figuur 9 een branch en een link naar GitHub toegewezen

Op basis van deze werkwijze heb ik niet alleen geprobeerd om een kwalitatief goed product op te leveren, maar ook om te laten zien dat ik professionele software-ontwikkelpraktijken kan toepassen in de praktijk. Het was een leerproces: niet alles ging meteen goed, maar door de structuur die ik heb aangehouden kon ik fouten snel terugvinden en corrigeren.

## 6. Bronnen

1. Atlassian. (z.d.). Confluence | Your Remote-Friendly Team workspace | Atlassian. https://www.atlassian.com/software/confluence
2. Atlassian. (z.d.). Jira | Issue & Project Tracking Software | Atlassian. https://www.atlassian.com/software/jira
