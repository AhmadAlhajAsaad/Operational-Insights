**Procesgang, Kwaliteitsborging en Projectbeheer**

**Equans Operational Insights Dashboard**

&nbsp;

- Versie: 1.0 
- Schoolsbegeleider: Jeroen Boogaard 
- Bedrijfsbegeleider: Viktor Klein (De Product Owner)
- Studentnaam: Ahmad Alhaj Asaad 
- Instelling: Equans — SLS Digital Platforms / DevOps Forge 
- Studiejaar: 2025 - 2026 

Inhoud

[**1.** **Inleiding** 3](#_Toc224328046)

[**2.** **Ontwikkelwerkwijze en Wijzigingsbeheer** 3](#_Toc224328047)

[**3.** **Kwaliteitsbewaking en Code Review** 4](#_Toc224328048)

[**4.** **Issue Tracking en Traceerbaarheid** 5](#_Toc224328049)

[**5.** **Teamcommunicatie en Samenwerking** 7](#_Toc224328050)

[**6.** **Bronnen** 10](#_Toc224328051)

1.  **Inleiding**

Dit hoofdstuk beschrijft de werkwijze die is gehanteerd gedurende de uitvoeringsfase van het afstudeerproject, met specifieke aandacht voor het wijzigingsbeheer, de kwaliteitsborging van de ontwikkelde software en de gebruikte projectbeheermethoden. De beschreven praktijken zijn gebaseerd op de werkelijk gehanteerde aanpak en worden ondersteund door concrete artefacten die zijn vastgelegd in Jira en Confluence.

1.  **Ontwikkelwerkwijze en Wijzigingsbeheer**

Hoewel het afstudeerproject in hoofdzaak zelfstandig is uitgevoerd, is er gedurende het gehele traject expliciet aandacht besteed aan verantwoord wijzigingsbeheer en een gestructureerde ontwikkelwerkwijze. Wijzigingen zijn incrementeel doorgevoerd met behulp van een featuregerichte branchstrategie, waarbij elke functionaliteit werd ontwikkeld op een afzonderlijke feature branch, vernoemd naar het corresponderende Jira-issue (bijv. Figuur 9: SDPDOFS-546-ophalen-van-gebruikerslijst).

Elke commit omvatte een afgebakende functionele toevoeging of technische verbetering en was voorzien van een helder en herleidbaar commitbericht dat expliciet verwees naar het bijbehorende Jira-issuenummer (PAN-nummer). Dankzij de directe koppeling tussen Jira-issues en GitHub-commits is per geïmplementeerde functionaliteit inzichtelijk wat de aanleiding was voor de wijziging en waar de technische realisatie heeft plaatsgevonden. Deze werkwijze borgt zowel de reproduceerbaarheid als de controleerbaarheid van het ontwikkelproces.

Figuur 1 in de projectdocumentatie toont een overzicht van afgeronde taken in Confluence.

Voltooide taken en documenten zijn bijgehouden in Confluence, dat fungeerde als centrale kennisbank voor het project. Afgeronde werkitems werden voorzien van reviewcommentaar voordat ze werden samengevoegd in de hoofdcodebase (main branch).

Figuur 2 geeft een gedetailleerde beschrijving van taken in Jira inclusief bijbehorende acceptatiecriteria.

1.  **Kwaliteitsbewaking en Code Review**

Gedurende het ontwikkeltraject is informele code review toegepast in nauwe samenwerking met de technisch begeleider, Brian Veltman. Nieuwe functionaliteiten werden besproken en inhoudelijk beoordeeld voordat zij werden samengevoegd in de main branch. De nadruk lag daarbij op de volgende kwaliteitsaspecten:

- Architectuurconformiteit: naleving van de gekozen drielegige REST API-architectuur en strikte scheiding tussen frontend en backend;
- Robuuste foutafhandeling: consequent gebruik van Result in de Rust-backend; afwezigheid van unwrap() in productiecode;
- UI-prestaties: optimalisatie van lijstweergave door middel van lazy loading en ondersteuning van lege toestanden (_empty states_);
- Gebruikersfeedback: implementatie van optimistic UI met pull-to-refresh als fallbackmechanisme.

De verkregen feedback is systematisch verwerkt in opeenvolgende commits. Voorafgaand aan elke push naar GitHub werden duidelijke opmerkingen toegevoegd aan de voltooide taak in Jira, zodat de status en motivering van elke wijziging volledig gedocumenteerd zijn.

Figuur 3 illustreert hoe duidelijke opmerkingen worden toegevoegd aan voltooide taken voordat ze naar GitHub worden gepusht.

1.  **Issue Tracking en Traceerbaarheid**

Voor de planning, bewaking en traceerbaarheid van de werkzaamheden is Jira ingezet als primaire issue tracker (projectcode: SDPDOFS). Alle taken, user stories en technische verbeteringen zijn als afzonderlijke issues vastgelegd en verdeeld over sprints. Grote user stories zijn systematisch opgesplitst in kleinere, beter beheersbare subtaken, conform de in Figuur 4 geïllustreerde structuur. Dit maakte het mogelijk om voortgang nauwkeurig te bewaken en na afronding gerichte opmerkingen of beoordelingen toe te voegen.

Figuur 4 laat zien hoe Jira grote user stories en taken opsplitst in kleinere, beter beheersbare onderdelen.

Figuur 5 bevat een lijst van taken die nog moeten worden afgerond, waarvan een deel reeds is voltooid.

De traceerbaarheid van het projectwerk is aantoonbaar via de volgende artefacten:

|                       |                                                                          |
| --------------------- | ------------------------------------------------------------------------ |
| Artefact              | Beschrijving                                                             |
| Jira-issues           | Status Done of Closed per afgeronde taak                                 |
| GitHub-commits        | Commitberichten met expliciete verwijzing naar Jira PAN-nummer           |
| Feature branches      | Eén branch per functionaliteit, direct gekoppeld aan Jira-issue          |
| Confluence-documenten | Alle opgestelde documenten zijn geverifieerd en vastgelegd in Confluence |
| Openstaande taken     | Figuur 5 toont resterende taken; een deel hiervan is inmiddels voltooid  |

Figuur 6 toont een volledig overzicht van door de auteur opgestelde en in Confluence geverifieerde documenten.

Figuur 7 biedt een voorbeeld sprintoverzicht in Jira met alle opgenomen en afgeronde issues.

1.  **Teamcommunicatie en Samenwerking**

Het project is uitgevoerd binnen het bredere DevOps Forge-team bij Equans SLS Digital Platforms. De dagelijkse afstemming vond plaats via een stand-up overleg elke ochtend, waarin de voortgang, eventuele blokkades en de planning voor de dag werden besproken. Deze structuur conformeert aan de Scrum-methodiek die binnen het team wordt gehanteerd.

Figuur 8 DevOpsForge Kanban Board voor teams

Figuur 8 toont het DevOpsForge Kanban-bord, waarop alle teamtaken per teamlid zichtbaar zijn. De bijdrage van de auteur aan het project was op dit bord inzichtelijk en werd dagelijks bijgewerkt. Aan elke taak of user story die code-implementatie vereiste, werd een afzonderlijke branch en een directe koppeling naar het corresponderende GitHub-repository toegekend (zie Figuur 9), waardoor de gehele ontwikkelcyclus — van taakdefinitie tot code-integratie — volledig herleidbaar is.

Figuur 9 een branch en een link naar GitHub toegewezen

De gehanteerde werkwijze borgt hiermee niet alleen de kwaliteit van het ontwikkelde product, maar demonstreert tevens de toepassing van professionele software-ontwikkelpraktijken conform de standaarden van de ICT-beroepspraktijk.

1.  **Bronnen**
2.  Atlassian. (z.d.-a). Confluence | Your Remote-Friendly Team workspace | Atlassian. https://www.atlassian.com/software/confluence
3.  Atlassian. (z.d.). Jira | Issue & Project Tracking Software | Atlassian. https://www.atlassian.com/software/jira
