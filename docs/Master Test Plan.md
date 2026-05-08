**Master Test Plan**

**Equans Operational Insights Dashboard**

- Studentnaam: Ahmad Alhaj Asaad (1035912)
- Project: Equans Operational Insights Dashboard
- Opleiding: Informatica – Hogeschool Rotterdam
- Organisatie: Equans Nederland – SLS Digital Platforms (DevOps Forge)

Inhoudsopgave

[**1\. Inleiding** 4](#_Toc224417266)

[**1.1** **Achtergrond van het project** 4](#_Toc224417267)

[**1.2** **Doel van dit document** 4](#_Toc224417268)

[**2\. Testscope** 5](#_Toc224417269)

[**2.1 In scope** 5](#_Toc224417270)

[**2.2 Out of scope** 6](#_Toc224417271)

[**3\. Teststrategie** 7](#_Toc224417272)

[**3.1 Risk-based testing** 7](#_Toc224417273)

[**3.2 Shift-left testing** 7](#_Toc224417274)

[**3.3 Testpyramide** 7](#_Toc224417275)

[**4\. Testsoorten** 8](#_Toc224417276)

[**4.1 Unit testing** 8](#_Toc224417277)

[**4.2 Integratietesting** 8](#_Toc224417278)

[**4.3 Systeemtesting** 8](#_Toc224417279)

[**4.4 Performance testing** 8](#_Toc224417280)

[**4.5 Security testing** 9](#_Toc224417281)

[**4.6 Usability testing** 9](#_Toc224417282)

[**5\. Testomgeving** 10](#_Toc224417283)

[**6\. Entry- en exitcriteria** 10](#_Toc224417284)

[**7\. Defectmanagement** 11](#_Toc224417285)

[**7.1 Defect lifecycle** 11](#_Toc224417286)

[**7.2 Defectclassificatie** 11](#_Toc224417287)

[**8\. Deliverables** 12](#_Toc224417288)

[**9\. Kritische Reflectie en Leerpunten** 13](#_Toc224417289)

[**9.1 Beperkingen van de testaanpak** 13](#_Toc224417290)

[**9.2 Evaluatie van testactiviteiten** 13](#_Toc224417291)

[**9.3 Lessen voor toekomstige projecten** 13](#_Toc224417292)

[**10\. Conclusie** 14](#_Toc224417293)

[**10.1 Realisatie** 14](#_Toc224417294)

[**10.2 Kwaliteitsvalidatie** 14](#_Toc224417295)

[**10.3 Beperkingen** 14](#_Toc224417296)

[**10.4 Projectbijdrage** 14](#_Toc224417297)

[**10.5 Eindoordeel** 14](#_Toc224417298)

[**11\. Referenties** 15](#_Toc224417299)

**Samenvatting**

Binnen moderne IT-organisaties is het verkrijgen van inzicht in softwarelicentiegebruik essentieel om kosten te beheersen en efficiënt gebruik van ontwikkelplatformen te stimuleren. Het Operational Insights Dashboard is ontwikkeld voor Equans om licentiegebruik en gebruikersactiviteit binnen Atlassian Cloud en GitHub Enterprise Cloud te analyseren en te visualiseren. Het systeem bestaat uit een full-stack webapplicatie met een Rust-gebaseerde backend, een React-frontend en een PostgreSQL-database.

Dit document beschrijft het Master Test Plan dat is opgesteld om de kwaliteit van het systeem systematisch te waarborgen. De teststrategie is gebaseerd op risk-based testing (Olsen et al., 2021), waarbij testactiviteiten worden afgestemd op de potentiële risico’s van het systeem. Daarnaast wordt een shift-left benadering toegepast om testactiviteiten vroeg in het ontwikkelproces te integreren. De verdeling van testactiviteiten volgt de testpyramide (Myers et al., 2012), waarbij de nadruk ligt op unit- en integratietests.

Binnen het project worden verschillende testsoorten toegepast, waaronder unit testing, integratietesting, systeemtesting, security testing, performance testing en usability testing. Deze tests richten zich zowel op de functionele correctheid van het systeem als op niet-functionele kwaliteitsaspecten zoals beveiliging, prestaties en gebruiksvriendelijkheid (Olsen et al., 2021).

De resultaten van de uitgevoerde tests vormen de basis voor het beoordelen van de betrouwbaarheid en stabiliteit van het Operational Insights Dashboard en bepalen of het systeem gereed is voor oplevering binnen de IT-omgeving van Equans.

**1\. Inleiding**

- 1.  **Achtergrond van het project**

Binnen Equans wordt gebruikgemaakt van verschillende ontwikkelplatformen, waaronder Atlassian Cloud Enterprise en GitHub Enterprise Cloud. Deze platformen worden gebruikt voor softwareontwikkeling, projectbeheer en samenwerking binnen ontwikkelteams. Ondanks het intensieve gebruik van deze systemen bestaat er beperkt inzicht in het daadwerkelijke gebruik van licenties en de bijbehorende kostenstructuren.

Het ontbreken van een geïntegreerd overzicht maakt het moeilijk om inefficiënt gebruik van licenties te identificeren. Hierdoor kunnen organisaties te maken krijgen met onnodige kosten door inactieve accounts, overallocatie van licenties of onvoldoende inzicht in gebruik per team of business unit.

Om dit probleem te adresseren is het Operational Insights Dashboard ontwikkeld. Dit dashboard verzamelt gegevens via API-integraties met Atlassian Cloud en GitHub Enterprise en visualiseert deze gegevens in een centraal dashboard. Hierdoor kunnen organisaties beter inzicht krijgen in licentiegebruik, gebruikersactiviteit en potentiële optimalisatiemogelijkheden.

- 1.  **Doel van dit document**

Het doel van dit document is het beschrijven van de teststrategie en testaanpak voor het Operational Insights Dashboard. Het Master Test Plan definieert de scope van het testproces, de gebruikte testmethoden en de criteria voor het succesvol afronden van testactiviteiten.

Dit document vervult meerdere functies binnen het project. Ten eerste dient het als leidraad voor het plannen en uitvoeren van testactiviteiten tijdens de ontwikkeling van het systeem. Ten tweede biedt het een formele kwaliteitsverantwoording binnen het kader van het afstudeeronderzoek. Ten derde geeft het stakeholders binnen Equans inzicht in de manier waarop de kwaliteit van het systeem wordt gecontroleerd.

De structuur van dit document is gebaseerd op richtlijnen voor softwaretestdocumentatie zoals beschreven in de IEEE 829-standaard voor software- en systeemtestdocumentatie (Client Challenge, z.d.).

**2\. Testscope**

**2.1 In scope**

De testscope van dit project omvat alle belangrijke componenten van het Operational Insights Dashboard. Hierbij worden zowel functionele als niet-functionele aspecten van het systeem getest.

De backend-architectuur vormt een belangrijk onderdeel van de testscope. Hierbij worden alle REST-API-endpoints getest die worden gebruikt voor het ophalen en verwerken van gegevens uit externe systemen. Deze endpoints zijn onder andere verantwoordelijk voor het ophalen van gebruikersgegevens, licentiegegevens en organisatorische informatie.

Daarnaast wordt de businesslogica van het systeem getest. Deze logica omvat onder andere algoritmen voor licentieanalyse, kostenberekening en identificatie van inactieve accounts.

Ook de frontendcomponenten van het dashboard vallen binnen de testscope. Hierbij wordt getest of gebruikersinformatie correct wordt weergegeven en of interactieve functionaliteiten zoals filters, tabellen en grafieken correct functioneren.

Verder wordt de integratie met externe API’s getest. Hierbij gaat het specifiek om de Atlassian Cloud Admin API en de GitHub Enterprise API.

Naast functionele tests wordt ook aandacht besteed aan niet-functionele kwaliteitsaspecten, zoals beveiliging, prestaties en gebruiksvriendelijkheid.

|     |     |     |
| --- | --- | --- |
| Categorie | Concreet onderdeel | Testsoort |
| Backend — API | Alle REST-endpoints (/api/v1/licenses, /persons, /orgs, /github, etc.) | Unit + Integratietest |
| Backend — Business logic | Licentieberekening, chargeback-logica, CSV-importverwerking | Unit tests (Rust #\[test\]) |
| Backend — Auth | JWT-validatie, SSO-integratie (Microsoft Entra ID) | Security tests |
| Backend — Jobs | Dagelijkse synchronisatietaak, rate-limit handling | Integratietest |
| Frontend — Componenten | Dashboard-widgets, filtertabellen, persoons- en organisatiepagina's | Component tests (Jest/RTL) |
| Frontend — Flows | Login-flow, CSV-uploadflow, handmatige refresh | E2E tests |
| Database | Migratiebestanden (001–009), query-performance | Integratietest + performance |
| Integraties | Atlassian Admin API, GitHub Enterprise API | Integratietest (mock/stub) |
| Security | OWASP Top 10 (A01, A02, A07), AVG-conformiteit | Security tests |
| Performance | API P95 < 200ms, dashboard load < 3s, 100 gelijktijdige gebruikers | Load testing (k6) |
| Usability | Kernflows voor licentiebeheerders/finance | Usability test (think-aloud) |

**2.2 Out of scope**

Hoewel het doel van het testproces is om een zo volledig mogelijk beeld te krijgen van de kwaliteit van het systeem, zijn bepaalde testactiviteiten buiten de scope van dit project geplaatst.

Zo worden volledige penetratietests niet uitgevoerd, omdat deze gespecialiseerde securitytools en expertise vereisen die buiten het tijdsbestek van het project vallen. Daarnaast wordt de infrastructuur van de productieomgeving niet getest, aangezien deze verantwoordelijkheid ligt bij het operations-team van Equans.

Ook bepaalde uitbreidingen van het systeem, zoals integraties met aanvullende platformen, vallen buiten de scope van dit testplan.

|     |     |
| --- | --- |
| Buiten scope | Reden |
| JFrog Artifactory API-integratie | Niet opgenomen in MVP (Won't Have W-01 scope); API beschikbaarheid niet gegarandeerd |
| Volledige penetratietest | Vereist gespecialiseerde tooling en externe expertise; buiten tijdbudget student (Myers et al., 2012) |
| Infrastructuurtest (Docker/infra) | Verantwoordelijkheid van Equans ops-team; buiten projectscope |
| Trello-integratie | Could Have (C-05); niet geïmplementeerd in huidige sprintplanning |
| Real-time streaming | Won't Have (W-02); architectuur is batch-based |
| SCIM-integratie | Won't Have (W-05) |
| Browser-overgrijpende E2E | Tijdsbeperking solo-project; Chrome als primaire browser getest |

**3\. Teststrategie**

**3.1 Risk-based testing**

De teststrategie van dit project is gebaseerd op het principe van risk-based testing, zoals gedefinieerd in het ISTQB Foundation Level syllabus (Olsen et al., 2021). Bij deze aanpak wordt de prioriteit van testactiviteiten bepaald door de combinatie van de kans dat een fout optreedt en de impact van deze fout op het systeem. Concreet betekent dit dat componenten met een hoog risicoprofiel, zoals authenticatie en licentieberekeningen, intensiever worden getest dan componenten met een lager risico.

Door deze aanpak kunnen testactiviteiten worden gericht op de onderdelen van het systeem die het grootste risico vormen voor de betrouwbaarheid of veiligheid van de applicatie.

**3.2 Shift-left testing**

Naast risk-based testing wordt een shift-left benadering toegepast (Olsen et al., 2021). Hierbij worden testactiviteiten zo vroeg mogelijk in het ontwikkelproces uitgevoerd, zodat defecten worden gedetecteerd in de fase waarin de correctiekosten het laagst zijn (Myers et al., 2012). Binnen dit project wordt shift-left gerealiseerd door het schrijven van unit tests parallel aan de implementatie en het uitvoeren van code reviews voorafgaand aan integratie.

**3.3 Testpyramide**

De verdeling van testactiviteiten volgt het concept van de testpyramide (Myers et al., 2012), waarbij het grootste deel van de tests bestaat uit unit tests, gevolgd door integratietests en een kleiner aantal end-to-end tests. Deze verdeling zorgt voor snelle feedback op componentniveau, terwijl hogere testniveaus de integratie en het systeemgedrag valideren (Olsen et al., 2021).

**4\. Testsoorten**

**4.1 Unit testing**

Unit tests worden gebruikt om individuele softwarecomponenten geïsoleerd te verifiëren (Myers et al., 2012). Binnen het Operational Insights Dashboard worden unit tests voornamelijk toegepast op backendlogica in Rust en frontendcomponenten in React.

|     |     |
| --- | --- |
| Aspect | Invulling |
| Framework | Rust ingebouwde #\[test\] macro's, cargo test, cargo tarpaulin voor coverage |
| Framework FE | Jest + React Testing Library |
| Scope backend | Businesslogica: licentieberekening (calculate_utilization), CSV-parsing, person-matching logica, rate-limit backoff |
| Scope frontend | React-componenten: LicenseDashboard, PersonTable, OrgFilter, CsvUpload |
| Coverdoelen | Backend: min. 70%, doel 85% · Frontend: min. 60%, doel 75% |
| Techniek | White-box (beslissingsafdekking voor Result&lt;T,E&gt; branches in Rust) + black-box equivalentieklassen |

**4.2 Integratietesting**

Integratietests controleren de interactie tussen verschillende systeemcomponenten, zoals de backend, database en externe API's (Olsen et al., 2021). Hierbij wordt gevalideerd of de interfaces tussen modules correct functioneren.

|     |     |
| --- | --- |
| Aspect | Invulling |
| Aanpak | Bottom-up integratietesting (eerst database ↔ Rust service, dan service ↔ API) |
| Scope | Atlassian API ↔ cache-laag, GitHub API ↔ sync-job, PostgreSQL ↔ migratiescripts, JWT-middleware ↔ endpoints |
| Externe API's | Gebruik **mock servers** (Wiremock of Rust mockito) voor Atlassian/GitHub API's — geen dependency op live third-party services |
| Testdata | Seed-data via SQL-fixtures; representatieve datasets van ~500 gebruikers |

**4.3 Systeemtesting**

Systeemtests richten zich op het testen van het volledige systeem vanuit het perspectief van de eindgebruiker (Olsen et al., 2021). Hierbij wordt het systeem als geheel gevalideerd tegen de functionele en niet-functionele requirements.

- Use case-gebaseerde E2E scenario's (bijv. UC-01: licentiebeheerder logt in en exporteert CSV-rapport)
- Exploratory testing sessies voor niet-gescripteerde flows
- Scenario's gebaseerd op FR

**4.4 Performance testing**

Performance tests controleren of het systeem voldoet aan prestatienormen zoals responstijd en stabiliteit bij meerdere gebruikers. Prestatietesting is een essentieel onderdeel van niet-functioneel testen (Olsen et al., 2021).

|     |     |     |     |
| --- | --- | --- | --- |
| KPI | Norm | Bron | Tool |
| API P95 response time | < 200ms | TR  | k6  |
| Dashboard load time | < 3 seconden | TR  | Chrome DevTools / k6 |
| Gelijktijdige gebruikers | 100 | TR  | k6  |
| Volledige vendor sync | < 5 minuten | TR  | k6 + logging |
| Database queries | < 50ms | TR  | PostgreSQL EXPLAIN |

**4.5 Security testing**

Security tests worden uitgevoerd om kwetsbaarheden in authenticatie en autorisatie te identificeren. De security-testaanpak is gebaseerd op de OWASP Top 10 (OWASP Top 10:2021, z.d.), die de meest kritieke beveiligingsrisico's voor webapplicaties categoriseert.

|     |     |     |
| --- | --- | --- |
| Testsoort | Aanpak | Tool / Referentie |
| Authenticatietests | AUTH (SSO, JWT, MFA) | Microsoft Entra ID + Postman |
| Token-beveiliging | Expiry, revocation, secure cookie (niet localStorage) | OWASP ASVS §3 |
| AVG-compliance | E-mailmaskering in logs, recht op vergetelheid, data-minimalisatie | GDPR Art. 5, Auth-test doc |
| Security headers | HSTS, CSP, X-Frame-Options, SameSite cookies | OWASP Secure Headers Project |
| Secrets management | Geen secrets in Git, gebruik van .env | TR-001 + GitHub Actions secrets |

**4.6 Usability testing**

Usability tests analyseren de gebruiksvriendelijkheid van het dashboard, als onderdeel van niet-functionele kwaliteitsvalidatie (Olsen et al., 2021).

|     |     |
| --- | --- |
| Aspect | Invulling |
| Methode | Think-aloud protocol in Sprint 6 |
| Doelgroep | 3–5 Equans-medewerkers: 1–2 licentiebeheerders, 1 finance medewerker, 1 IT-beheerder |
| Kernflows | Login-flow, licentieopvraag per team, CSV-export, handmatige refresh |
| Evaluatie | Heuristische evaluatie op basis van bruikbaarheidscriteria (Olsen et al., 2021) |
| Opzet | Gestructureerd observatiescript met think-aloud; 45 min per sessie |

**5\. Testomgeving**

De testomgeving is gebaseerd op een containergebaseerde ontwikkelomgeving met Docker, waarin backend, frontend en database worden uitgevoerd.

|     |     |
| --- | --- |
| Omgeving | Beschrijving |
| Ontwikkelomgeving | Dev container (Debian GNU/Linux 12 Bookworm) in VS Code |
| Backend runtime | Rust 1.7x + Axum 0.7 |
| Database | PostgreSQL (Docker Compose via infra/docker-compose.yml) |
| Frontend runtime | Node.js + Vite + React 19 + TypeScript 5 |
| Testbrowser | Google Chrome (primair), Firefox (secundair) |
| Netwerk | Lokale Docker-netwerk voor geïsoleerde tests; staging voor integratie |

**6\. Entry- en exitcriteria**

De entry- en exitcriteria voor testactiviteiten zijn opgesteld conform de IEEE 829-standaard voor testdocumentatie (Client Challenge, z.d.). Testactiviteiten starten wanneer de applicatie compileert, de testomgeving operationeel is en testdata beschikbaar is. Het testproces wordt afgerond wanneer alle kritieke functionaliteiten zijn getest en geen kritieke defects meer aanwezig zijn (Olsen et al., 2021).

- \[ \] Code compileert zonder errors (cargo build, npm run build)
- \[ \] Functionele requirements zijn vastgesteld en goedgekeurd
- \[ \] Testomgeving (Docker Compose) is operationeel
- \[ \] Seed-data en databasemigraties zijn uitgevoerd
- \[ \] Code review is afgerond voor het te testen component
- \[ \] Unit tests slagen lokaal (cargo test, npm test)

**7\. Defectmanagement**

Het defectmanagementproces is ingericht conform de richtlijnen van het ISTQB Foundation Level syllabus (Olsen et al., 2021). Alle gevonden fouten worden geregistreerd in het Jira-systeem van Equans. Elk defect wordt voorzien van een beschrijving, ernstniveau en reproduceerstappen.

**7.1 Defect lifecycle**

Beschrijf de lifecycle op basis van het Jira-workflow:

New → In Progress → Fixed → In Review → Verified → Closed

↘ Won't Fix (gemotiveerd)

**7.2 Defectclassificatie**

|     |     |     |
| --- | --- | --- |
| Ernst | Definitie | Voorbeeld in dit project |
| Critical | Systeem niet bruikbaar; data-integriteitsrisico of security-breuk | Ongeautoriseerde API-toegang mogelijk |
| High | Kernfunctionaliteit werkt niet; Must Have requirement gefaald | Licentieberekening geeft incorrect resultaat |
| Medium | Functionaliteit werkt deels; workaround beschikbaar | Filter werkt niet op alle browsers |
| Low | Cosmetisch of minor UX-issue | Afkapping van lange organisatienamen |

**7.3 Defectrapportage**

Een defectrapport in Jira bevat:

- Uniek ID (Jira-nummer)
- Samenvatting (1 zin)
- Ernst (Critical/High/Medium/Low)
- Stappen om te reproduceren
- Verwacht vs. werkelijk gedrag
- Omgeving (OS, browser, versie)
- Screenshot / logfragment
- Koppeling aan requirements (FR-/TR-nummer)

**8\. Deliverables**

Testactiviteiten worden uitgevoerd in meerdere sprints waarbij unit tests, integratietests en systeemtests gefaseerd plaatsvinden. De belangrijkste testdeliverables worden gedocumenteerd.

1.  Master Test Plan (dit document)
2.  Test Strategy
3.  Testcases per testsoort
4.  Performance testrapport (k6)
5.  Security testrapporten (AUTH + GDPR)
6.  Traceability Matrix (requirements ↔ testcases)
7.  Defectrapporten / Jira issue log
8.  Usability testrapport
9.  UAT-acceptatierapport
10. Testafsluitrapport

**9\. Kritische Reflectie en Leerpunten**

**9.1 Beperkingen van de testaanpak**

Het is van belang de beperkingen van de gevolgde testaanpak te erkennen. Myers et al. (2011) benadrukken dat het onmogelijk is om software uitputtend te testen; selectie van testgevallen is daarom altijd een afweging.

1.  **Solo-ontwikkeling en testobjectiviteit:** Omdat het project door één ontwikkelaar wordt uitgevoerd, bestaat er een risico op beperkte testonafhankelijkheid (Olsen et al., 2021). Dit is gecompenseerd door geautomatiseerde tests, code reviews door de technisch begeleider en acceptatietests door stakeholders.

1.  **Externe API's niet volledig testbaar:** De Atlassian Cloud en GitHub Enterprise API's konden niet in een volledig geïsoleerde omgeving worden getest. Mock servers bieden een benadering van de werkelijkheid, maar garanderen geen volledige representativiteit van het API-gedrag.

1.  **Beperkte penetratietesting:** Volledige penetratietesting vereist gespecialiseerde expertise en tooling die buiten het projectbudget vallen. In plaats daarvan zijn gerichte tests uitgevoerd op basis van de OWASP Top 10 (OWASP Top 10:2021, z.d.).

1.  **Tijdsdruk en testdekking:** Door de sprintplanning konden niet alle Could Have-testcases worden uitgevoerd. De focus is gelegd op Must Have- en Should Have-requirements conform de risk-based teststrategie (Olsen et al., 2021).

**9.2 Evaluatie van testactiviteiten**

|     |     |
| --- | --- |
| Aspect | Evaluatie |
| Unit tests Rust | Effectief voor vroege detectie van logicafouten in licentieberekening |
| Security tests | Goede dekking dankzij gestructureerde aanpak op basis van OWASP Top 10 |
| Performance tests | k6-scripts leverden concrete, meetbare resultaten |
| Integratietests | Mock-gebaseerde aanpak functioneel, maar beperkt in representativiteit |

**9.3 Lessen voor toekomstige projecten**

Op basis van de ervaringen binnen dit project zijn de volgende aanbevelingen geformuleerd:

1.  Begin met een traceability matrix vóór de eerste sprint, zodat de relatie tussen requirements en testgevallen vroeg inzichtelijk is (Client Challenge, z.d.).
2.  Implementeer integratietestscripts met mock servers vanaf het begin van de ontwikkeling, in lijn met het shift-left principe (Olsen et al., 2021).
3.  Plan User Acceptance Testing in de sprint review van elke sprint, niet alleen aan het einde van het project.
4.  Investeer vroeg in testdata-management — reproduceerbare seed-scripts voorkomen tijdverlies bij het opzetten van testomgevingen.

**10\. Conclusie**

Dit Master Test Plan beschrijft een gestructureerde teststrategie voor het Operational Insights Dashboard, gebaseerd op risk-based testing (Olsen et al., 2021), de testpyramide (Myers et al., 2012) en IEEE 829 (Client Challenge, z.d.). De combinatie van unit-, integratie-, systeem-, security- en performancetests dekt zowel functionele als niet-functionele kwaliteitsaspecten af.

**10.1 Realisatie**

Dankzij een traceability matrix zijn alle FR's en TR's herleidbaar naar testgevallen. Shift-left testing (Olsen et al., 2021) zorgde voor vroege detectie van defecten, wat de correctiekosten laag hield (Myers et al., 2012).

**10.2 Kwaliteitsvalidatie**

- Functionele correctheid: Alle Must Have-requirements getest
- Performance: API P95 < 200ms, dashboard < 3s
- Security: OWASP-gebaseerde tests; geen kritieke kwetsbaarheden
- Onderhoudbaarheid: Backend coverage > 75%, frontend > 60%
- Usability: Positieve feedback op kernflows

**10.3 Beperkingen**

- Solo-ontwikkeling: Beperkte testonafhankelijkheid, gecompenseerd door automatisering en code reviews
- Geen volledige pentest: Vervangen door gerichte OWASP-tests
- Externe API's: Getest met mocks, niet volledig representatief
- Tijdsdruk: Focus op Must Have/Should Have (risk-based)

**10.4 Projectbijdrage**

Het testproces heeft gevalideerd dat het dashboard:

- Betrouwbare licentie-inzichten levert
- Accurate kostenberekeningen uitvoert
- Voldoet aan beveiligingseisen
- Aansluit bij gebruikersbehoeften (usability tests)

**10.5 Eindoordeel**

Het Operational Insights Dashboard voldoet aan de gestelde kwaliteitscriteria en is gereed voor oplevering binnen Equans. Het testproces combineert academische onderbouwing met praktische toepasbaarheid en toont aan dat kwaliteitsborging ook binnen een individueel afstudeerproject professioneel kan worden uitgevoerd.

**11\. Referenties**

1.  _Client challenge_. (z.d.). https://www.scribd.com/document/531867110/IEEE-Std-829-2008-IEEE-Standard-for-Software-and-System-Test-Documentation-1423058832-HCSCIRY
2.  Myers, G. J., Badgett, T., & Sandler, C. (2012). _THE ART OF SOFTWARE TESTING_ (Third Edition) \[Book\]. John Wiley & Sons, Inc. https://malenezi.github.io/malenezi/SE401/Books/114-the-art-of-software-testing-3-edition.pdf
3.  Olsen, K., Posthuma, M., Ulrich, S., Olsen, K., Parveen, T., Black, R., Friedenberg, D., Hamburg, M., McKay, J., Posthuma, M., Schaefer, H., Smilgin, R., Smith, M., Toms, S., Ulrich, S., Walsh, M., Zakaria, E., Müller, T., Friedenberg, D., . . . Veenendaal, E. V. (2021). Certified Tester Foundation Level Syllabus. In International Software Testing Qualifications Board, _International Software Testing Qualifications Board_. https://istqb-main-web-prod.s3.amazonaws.com/media/documents/ISTQB-CTFL_Syllabus_2018_v3.1.1.pdf
4.  _OWASP Top 10:2021_. (z.d.). https://owasp.org/Top10/2021/