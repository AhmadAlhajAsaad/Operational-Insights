**Privacy- en Beveiligingsplan**

**Equans Operational Insights Dashboard**

- Versie: 1.0
- Studiejaar: 2025 - 2026 
- Studentnaam: Ahmad Alhaj Asaad 
- Opleiding: HBO-ICT Software Engineering 
- Organisatie: Equans / SLS-DP-DevOps-Forge 
- Schoolsbegeleider: Jeroen Boogaard 
- Bedrijfsbegeleider: Viktor Klein (Business Owner)
- Technisch begeleider: Brian Veltman

Inhoudsopgave

[**1\. Inleiding** 3](#_Toc224325664)

[**2\. Scope en doelstelling** 3](#_Toc224325665)

[**2.1 Scope** 3](#_Toc224325666)

[**2.2 Doelstelling** 3](#_Toc224325667)

[**3\. Wettelijk kader — Algemene Verordening Gegevensbescherming (AVG)** 4](#_Toc224325668)

[**4\. Verwerkte persoonsgegevens** 5](#_Toc224325669)

[**4.1 Overzicht van verwerkte datapunten** 5](#_Toc224325670)

[**4.2 Dataminimalisatie** 5](#_Toc224325671)

[**5\. Anonimisering en maskering van e-mailadressen** 6](#_Toc224325672)

[**5.1 E-mailmaskering (productie)** 6](#_Toc224325673)

[**5.2 Validatietests** 7](#_Toc224325674)

[**6\. Authenticatie en autorisatie** 8](#_Toc224325675)

[**6.1 Authenticatie** 8](#_Toc224325676)

[**6.2 Autorisatie (RBAC)** 8](#_Toc224325677)

[**6.3 Token-opslag** 8](#_Toc224325678)

[**7\. Datatransmissie en versleuteling** 9](#_Toc224325679)

[**7.1 In transit** 9](#_Toc224325680)

[**7.2 At rest** 9](#_Toc224325681)

[**7.3 Geheimenbeheer** 9](#_Toc224325682)

[**8\. Gegevensopslag en -retentie** 10](#_Toc224325683)

[**8.1 Retentiebeleid** 10](#_Toc224325684)

[**8.2 Databaseontwerp** 10](#_Toc224325685)

[**9\. Rechten van betrokkenen** 10](#_Toc224325686)

[**Verwijderingsprocedure (Recht op vergetelheid)** 10](#_Toc224325687)

[**10\. Verwerkersovereenkomsten (DPA's)** 11](#_Toc224325688)

[**11\. Beveiligingsmaatregelen (OWASP Top 10)** 11](#_Toc224325689)

[**12\. Logging en monitoring** 12](#_Toc224325690)

[**12.1 Logging-principes** 12](#_Toc224325691)

[**12.2 Audit trail** 12](#_Toc224325692)

[**12.3 Gezondheidscontrole** 12](#_Toc224325693)

[**13\. Risicobeoordeling** 12](#_Toc224325694)

[**14\. Testmaatregelen** 13](#_Toc224325695)

[**15\. Verantwoordelijkheden** 13](#_Toc224325696)

[**16\. Referenties** 14](#_Toc224325697)

**1\. Inleiding**

Dit document beschrijft het privacy- en beveiligingsplan voor het Equans Operational Insights Dashboard. Het systeem verzamelt, verwerkt en presenteert licentie- en gebruikersgegevens van externe platformen (Atlassian, GitHub Enterprise) ten behoeve van intern licentiebeheer en kostentoewijzing.

Omdat het systeem persoonsgegevens verwerkt — waaronder namen, e-mailadressen en accountstatussen van medewerkers — is naleving van de Algemene Verordening Gegevensbescherming (AVG/GDPR) een fundamentele eis (Regulation - 2016/679 - EN - Gdpr - EUR-Lex, z.d.). Dit plan consolideert alle privacy- en beveiligingsoverwegingen die in de projectdocumentatie zijn vastgelegd en biedt een centraal referentiepunt voor de genomen maatregelen.

**2\. Scope en doelstelling**

**2.1 Scope**

Dit plan is van toepassing op:

- De Rust backend (API, datacollectie, achtergrondtaken)
- De React/TypeScript frontend (SPA, dashboards)
- De PostgreSQL-database (persistente opslag)
- Alle integraties met externe API's (Atlassian Admin API, GitHub Enterprise API)
- De staging- en productieomgeving inclusief Docker-containers

**2.2 Doelstelling**

1.  Waarborgen van AVG-conformiteit bij de verwerking van persoonsgegevens (Regulation - 2016/679 - EN - Gdpr - EUR-Lex, z.d.)
2.  Minimaliseren van privacyrisico's door middel van privacy by design en privacy by default (Cavoukian & Information and Privacy Commissioner of Ontario, 2009/2011)
3.  Beschermen van de vertrouwelijkheid, integriteit en beschikbaarheid van gegevens
4.  Documenteren van de technische en organisatorische maatregelen conform Art. 32 AVG (Regulation - 2016/679 - EN - Gdpr - EUR-Lex, z.d.)

**3\. Wettelijk kader — Algemene Verordening Gegevensbescherming (AVG)**

Het systeem valt onder de AVG (Verordening (EU) 2016/679) aangezien het persoonsgegevens van EU-ingezetenen verwerkt (Regulation - 2016/679 - EN - Gdpr - EUR-Lex, z.d.; De AVG in het Kort, 2024). De volgende AVG-artikelen zijn direct van toepassing:

|     |     |     |
| --- | --- | --- |
| AVG-artikel | Onderwerp | Toepassing in dit systeem |
| Art. 5 | Beginselen gegevensverwerking | Dataminimalisatie, doelbinding, opslagbeperking |
| Art. 6 | Rechtmatigheid verwerking | Gerechtvaardigd belang (licentiebeheer) als grondslag |
| Art. 13/14 | Informatieplicht | Medewerkers worden geïnformeerd via intern privacybeleid |
| Art. 17 | Recht op vergetelheid | Verwijderingsworkflows voor persoonsgegevens |
| Art. 20 | Recht op overdraagbaarheid | Exportfunctionaliteit voor persoonsgebonden data |
| Art. 25 | Privacy by design/default | Maskering standaard ingeschakeld; minimale datacollectie |
| Art. 28 | Verwerkersovereenkomsten | DPA's met Atlassian, GitHub, JFrog |
| Art. 32 | Beveiliging verwerking | Versleuteling, toegangscontrole, logging |
| Art. 33/34 | Meldingsplicht datalekken | Incidentresponsprocedure bij datalekken |

**Grondslag voor verwerking**

De verwerking van persoonsgegevens vindt plaats op basis van **gerechtvaardigd belang** (Art. 6, lid 1, sub f AVG) (Regulation - 2016/679 - EN - Gdpr - EUR-Lex, z.d.): Equans heeft een legitiem belang bij het beheren van softwarelicenties, het toewijzen van kosten aan organisatie-eenheden en het identificeren van inactieve accounts om onnodige licentiekosten te voorkomen. De verwerking is proportioneel: er worden uitsluitend gegevens verzameld die noodzakelijk zijn voor licentiebeheer (De AVG in het Kort, 2024).

**4\. Verwerkte persoonsgegevens**

**4.1 Overzicht van verwerkte datapunten**

|     |     |     |     |     |
| --- | --- | --- | --- | --- |
| Gegeven | Bron | AVG-classificatie | Doel | Retentieperiode |
| Naam (displaynaam) | Atlassian, GitHub | Persoonsgegeven | Identificatie bij licentiebeheer | Account-levensduur |
| E-mailadres | Atlassian, GitHub | Persoonsgegeven | Matching personen aan vendor-accounts | Account-levensduur |
| Account-ID | Atlassian, GitHub | Pseudo-identificator | Technische koppeling | Account-levensduur |
| Accountstatus | Atlassian, GitHub | Persoonsgegeven | Actief/inactief licentiegebruik | Account-levensduur |
| Productlicentietoewijzing | Atlassian | Bedrijfsgegeven | Kostentoewijzing | 2 jaar |
| Laatste activiteitsdatum | Atlassian, GitHub | Persoonsgegeven | Detectie inactieve accounts | 2 jaar |
| Organisatorische eenheid | CSV-import | Bedrijfsgegeven | Kostentoewijzing, rapportage | Account-levensduur |
| IP-adres (in logs) | Backend-server | Persoonsgegeven | Beveiligingsmonitoring | 90 dagen |

**4.2 Dataminimalisatie**

Conform Art. 5 AVG (dataminimalisatie) worden uitsluitend gegevens verzameld die strikt noodzakelijk zijn voor de beoogde doeleinden (Regulation - 2016/679 - EN - Gdpr - EUR-Lex, z.d.). De volgende beperkingen zijn van toepassing:

- Er worden geen wachtwoorden, financiële gegevens of bijzondere persoonsgegevens verzameld
- Profielfoto-URL's worden niet lokaal opgeslagen
- API-responses worden gefilterd op relevante velden vóór opslag in de database
- Logberichten bevatten uitsluitend gemaskeerde identificatoren (zie §5)

**5\. Anonimisering en maskering van e-mailadressen**

Conform het Privacy by Design-principe (Cavoukian & Information and Privacy Commissioner of Ontario, 2009/2011) en Art. 25 AVG (Regulation - 2016/679 - EN - Gdpr - EUR-Lex, z.d.) is gegevensmaskering standaard ingeschakeld voor alle gebruikersrollen. Anonimisering in de testomgeving zorgt er bovendien voor dat productiegegevens nooit in test- of stagingomgevingen terechtkomen.

**5.1 E-mailmaskering (productie)**

E-mailadressen worden op drie niveaus gemaskeerd:

**Niveau 1: Log-output**

Alle logberichten die e-mailadressen bevatten worden automatisch gemaskeerd. Dit voorkomt dat persoonsgegevens in logbestanden terechtkomen, conform TM-04 uit de SRS en het beginsel van dataminimalisatie (Regulation - 2016/679 - EN - Gdpr - EUR-Lex, z.d., Art. 5).

|     |     |     |
| --- | --- | --- |
| Gegeven | Ongemaskeerd | Gemaskeerd in logs |
| E-mail | john.doe@equans.com | j\*\*\*@e\*\*\*.com |
| IP-adres | 192.168.1.100 | 192.168.x.x |
| API-token | ghp_xxxxxxxxxxxx | ghp_\*\*\* |
| Account-ID | user-12345 | user-\*\*\*\*\* |

**Niveau 2: Frontend-weergave (niet-beheerders)**

Voor gebruikers zonder beheerdersrol worden e-mailadressen gemaskeerd weergegeven via de mask_email-functie:

Figuur 1 Implementatie van e-mailmaskering in Rust

Deze functie toont het volledige e-mailadres alleen aan beheerders; voor alle andere gebruikers wordt het adres gemaskeerd volgens het patroon _"u\*\*\*@d\*\*\*.com"_. Dit implementeert zowel AVG-dataminimalisatie als role-based toegangscontrole.

|     |     |
| --- | --- |
| Rol | Weergave e-mailadres |
| Beheerder | john.doe@equans.com |
| Gebruiker | j\*\*\*@e\*\*\*.com |

**Niveau 3: Staging-/testomgeving**

In de staging-omgeving worden alle persoonsgegevens geanonimiseerd:

|     |     |
| --- | --- |
| Veld | Anonimiseringsmethode |
| E-mail | {hash}@anonymized.local |
| Naam | Faker-gegenereerde namen |
| IP-adres | Willekeurig privé-IP |
| Account-ID | UUID-vervanging |

**5.2 Validatietests**

De correcte werking van e-mailmaskering wordt gevalideerd door middel van geautomatiseerde tests (zie GDPR & Data Protection Testing):

|     |     |     |
| --- | --- | --- |
| Test-ID | Testcase | Verwacht resultaat |
| GDPR-001 | Controleer logs op ongemaskeerde PII | Geen ongemaskeerde e-mailadressen |
| GDPR-002 | Controleer foutmeldingen op gevoelige data | Geen gevoelige data blootgesteld |
| MIN-001 | API-response bevat alleen noodzakelijke data | Geen extra PII in responses |
| MIN-003 | Logs bevatten minimale PII | Alleen gemaskeerde identificatoren |

**6\. Authenticatie en autorisatie**

**6.1 Authenticatie**

Alle gebruikersinteractie vereist authenticatie via Equans SSO (Microsoft Azure Active Directory / Entra ID) (Cilwerner, z.d.). De backend valideert alle inkomende JWT-tokens (RFC 7519: JSON Web Token (JWT), z.d.) op:

|     |     |
| --- | --- |
| Claim | Validatie |
| exp | Token is niet verlopen |
| iss | Correcte Azure AD-tenant |
| aud | Correcte applicatie-ID |
| JWKS | Handtekeningvalidatie via Azure AD JWKS-endpoint |

Er is geen ongeauthenticeerde toegang mogelijk in productie. Het enige onbeschermde endpoint is /health (systeemgezondheidscontrole).

**6.2 Autorisatie (RBAC)**

Rolgebaseerde toegangscontrole (RBAC) wordt afgedwongen via Azure AD-groepen (Google Books, z.d.):

|     |     |
| --- | --- |
| Rol | Rechten |
| Gebruiker | Dashboard raadplegen, gemaskeerde e-mailadressen zien |
| Beheerder | Volledige e-mailadressen zien, synchronisatie triggeren, import |
| Finance | Kostendoorbelasting raadplegen, CSV-export |

**6.3 Token-opslag**

JWT-tokens worden opgeslagen conform de aanbevelingen van (Cilwerner, z.d.): de MSAL-bibliotheek slaat tokens op in sessiememory, waardoor tokens niet toegankelijk zijn voor kwaadaardige scripts (XSS-mitigatie).

|     |     |     |
| --- | --- | --- |
| Component | Opslagmethode | Reden |
| Frontend | MSAL sessiememory | Voorkomt XSS-exfiltratie (niet in localStorage) |
| Backend | Geen token-opslag | Stateless JWT-validatie per verzoek |

**7\. Datatransmissie en versleuteling**

**7.1 In transit**

|     |     |     |
| --- | --- | --- |
| Verbinding | Protocol | Verificatie |
| Browser → Frontend | TLS 1.2+ | SSL Labs-scan |
| Frontend → Backend | TLS 1.2+ | Certificaatvalidatie |
| Backend → PostgreSQL | TLS 1.2 | sslmode=require in connection string |
| Backend → Atlassian API | TLS 1.2+ | HTTPS verplicht |
| Backend → GitHub API | TLS 1.2+ | HTTPS verplicht |

HTTP-verzoeken worden automatisch omgeleid naar HTTPS. Het gebruik van minimaal TLS 1.2 is conform de richtlijnen van NIST (McKay & Cooper, 2019) en TM-01 uit de SRS.

**7.2 At rest**

|     |     |     |
| --- | --- | --- |
| Component | Versleutelingsmethode | Verificatie |
| PostgreSQL | TDE / AES-256 | Controle van pg_settings |
| Back-ups | AES-256 | Verificatie van back-upversleuteling |
| Opslag | AES-256 | Volume-encryptie gevalideerd |

**7.3 Geheimenbeheer**

|     |     |     |
| --- | --- | --- |
| Geheim | Opslaglocatie | Toegang |
| Atlassian API-key | Environment variabele / Docker secret | Alleen backend-container |
| GitHub App private key | Environment variabele / Docker secret | Alleen backend-container |
| Database-wachtwoord | Environment variabele / Docker secret | Alleen backend-container |
| Azure AD client secret | GitHub Secrets (CI/CD) | Alleen deployment pipeline |

**Harde regels:**

- API-keys worden **nooit** in broncode opgeslagen (.gitignore)
- API-keys worden **nooit** naar de frontend gestuurd
- API-keys worden **nooit** ongemaskeerd gelogd
- API-keys worden minimaal **elke 90 dagen** geroteerd
- .env.example bevat uitsluitend placeholderwaarden

**8\. Gegevensopslag en -retentie**

**8.1 Retentiebeleid**

|     |     |     |
| --- | --- | --- |
| Gegevenstype | Retentieperiode | Verwijderacte |
| Gebruikersgegeven | Account-levensduur | Verwijdering bij verwijderingsverzoek |
| Licentiegebruik | 2 jaar | Automatische purge na retentieperiode |
| Login-historie | 1 jaar | Automatische purge na retentieperiode |
| IP-adressen (logs) | 90 dagen | Automatische purge na retentieperiode |
| Audit-logs | 2 jaar | Geanonimiseerd bewaard voor compliance |

**8.2 Databaseontwerp**

- Persoonsgegevens zijn geïdentificeerd en gelabeld in het databaseschema zodat zij kunnen worden gelokaliseerd bij verwijderingsverzoeken (conform TS-06, SRS)
- SQL-queries zijn volledig geparametriseerd via SQLx — er vindt geen dynamische string-concatenatie plaats (bescherming tegen SQL-injectie)
- Migraties worden beheerd via versiegecontroleerde migratiebestanden

**9\. Rechten van betrokkenen**

De AVG geeft betrokkenen (medewerkers) specifieke rechten ten aanzien van hun persoonsgegevens (Regulation - 2016/679 - EN - Gdpr - EUR-Lex, z.d., Art. 15–21; De AVG in het Kort, 2024). Het systeem ondersteunt deze rechten als volgt:

|     |     |     |
| --- | --- | --- |
| AVG-recht | Art. | Implementatie |
| Recht op inzage | 15  | Beheerders kunnen persoonsgegevens opvragen via het persoonenoverzicht |
| Recht op rectificatie | 16  | Gegevens worden bijgewerkt bij elke dagelijkse synchronisatie met vendor-API's |
| Recht op vergetelheid | 17  | Verwijderingsworkflow: alle persoonsgebonden records worden verwijderd |
| Recht op beperking verwerking | 18  | Gegevensverwerking kan per persoon worden stopgezet |
| Recht op overdraagbaarheid | 20  | Exportfunctionaliteit genereert een machine-leesbaar pakket (JSON) |
| Recht van bezwaar | 21  | Bezwaren worden afgehandeld via de privacyfunctionaris van Equans |

**Verwijderingsprocedure (Recht op vergetelheid)**

Bij een verwijderingsverzoek worden de volgende stappen doorlopen:

1.  Verzoek wordt ontvangen via de privacyfunctionaris
2.  Alle persoonsgebonden records worden geïdentificeerd in de database
3.  Verwijdering uit de persons-tabel en gerelateerde koppelingen
4.  Licentiegeschiedenis wordt geanonimiseerd (niet verwijderd, voor financiële rapportage)
5.  Audit-logs worden bewaard met geanonimiseerde gebruikersreferentie
6.  Bevestiging van verwijdering wordt teruggemeld aan de betrokkene

**10\. Verwerkersovereenkomsten (DPA's)**

Het systeem communiceert met externe leveranciers die als verwerker optreden. Voor elke leverancier is een verwerkersovereenkomst (Data Processing Agreement) vereist conform Art. 28 AVG (Regulation - 2016/679 - EN - Gdpr - EUR-Lex, z.d.):

|     |     |     |     |
| --- | --- | --- | --- |
| Leverancier | Gedeelde gegevens | Doel | DPA-status |
| Atlassian | Account-ID's, gebruiksdata | Licentiebeheer | Ondertekend |
| GitHub | Account-ID's, gebruiksdata | Licentiebeheer | Ondertekend |
| JFrog | Gebruiksstatistieken | Licentiebeheer | Ondertekend |
| Microsoft | Azure AD-tokens | Authenticatie (SSO) | Ondertekend |

**11\. Beveiligingsmaatregelen (OWASP Top 10)**

Het systeem is ontworpen conform de OWASP Top 10 (2021) richtlijnen (OWASP Top Ten Web Application Security Risks | OWASP Foundation, z.d.):

|     |     |
| --- | --- |
| OWASP Top 10 | Maatregel |
| A01 — Broken Access Control | JWT-validatie middleware op alle endpoints; RBAC via Azure AD-groepen |
| A02 — Cryptographic Failures | TLS 1.2+ voor alle verbindingen; AES-256 at rest; geen plaintext geheimen |
| A03 — Injection | Geparametriseerde SQL via SQLx; geen dynamische query-opbouw |
| A04 — Insecure Design | Privacy by design; threat modelling; dataminimalisatie |
| A05 — Security Misconfiguration | Expliciete waarschuwing bij uitgeschakelde auth; geen debug-info in responses |
| A06 — Vulnerable Components | Bekende kwetsbaarheden gedocumenteerd (SDD §6.3); afhankelijkheden gemonitord |
| A07 — Identification & Auth Failures | Korte token-levensduur (1 uur); automatische vernieuwing via MSAL |
| A08 — Data Integrity Failures | Versiegecontroleerde migraties; gevalideerde CSV-imports |
| A09 — Security Logging & Monitoring | Structured logging met correlatie-ID's; alle requests gelogd via TraceLayer |
| A10 — Server-Side Request Forgery | Backend maakt alleen verbinding met bekende, geconfigureerde API-endpoints |

**12\. Logging en monitoring**

**12.1 Logging-principes**

- **Structured logging** met correlatie-ID's voor traceerbaarheid (TM-10, SRS)
- Logniveaus: ERROR, WARN, INFO, DEBUG
- Gevoelige gegevens worden **altijd** gemaskeerd in logs (§5.1)
- Foutmeldingen worden gelogd maar **nooit** ongewijzigd teruggestuurd naar de client

**12.2 Audit trail**

Alle beheerdersacties worden gelogd met de volgende gegevens:

|     |     |
| --- | --- |
| Veld | Beschrijving |
| Wie | Geauthenticeerde gebruiker (JWT sub) |
| Wat | Uitgevoerde actie |
| Wanneer | Tijdstempel (UTC) |
| Resultaat | Succes of fout |

**12.3 Gezondheidscontrole**

Het endpoint /health verifieert:

- Databaseconnectiviteit
- Connectiviteit met externe API's
- Systeemstatus

**13\. Risicobeoordeling**

|     |     |     |     |     |
| --- | --- | --- | --- | --- |
| ID  | Risico | Kans | Impact | Maatregel |
| P-01 | Ongemaskeerde PII in logbestanden | Laag | Hoog | Automatische e-mailmaskering; GDPR-001/002 testcases |
| P-02 | Ongeautoriseerde toegang tot persoonsgegevens | Laag | Hoog | JWT-authenticatie; RBAC; Azure AD-groepen |
| P-03 | SQL-injectie leidt tot data-exfiltratie | Laag | Hoog | Geparametriseerde queries via SQLx; geen dynamische SQL |
| P-04 | API-key blootgesteld in broncode of logs | Laag | Hoog | Environment variabelen; .gitignore; gemaskarrd in logs; rotatie elke 90d |
| P-05 | XSS-aanval exfiltreert JWT-tokens | Laag | Hoog | MSAL sessiememory (niet localStorage); Content Security Policy |
| P-06 | GDPR-verwijderingsverzoek niet volledig uitgevoerd | Middel | Hoog | Gestructureerde verwijderingsprocedure (§9); validatietests |
| P-07 | Datalek naar onbevoegde medewerkers (ontbrekende rolcontrole) | Laag | Middel | E-mailmaskering voor niet-beheerders; RBAC |
| P-08 | Vendor API stuurt onverwachte persoonsgegevens mee | Middel | Laag | Responsefiltering; alleen gedefinieerde velden worden opgeslagen |

**14\. Testmaatregelen**

Privacy- en beveiligingsmaatregelen worden gevalideerd door middel van de volgende testcategorieën:

|     |     |     |
| --- | --- | --- |
| Categorie | Testdocument | Omvat |
| GDPR-compliance | Security Testing | PII-maskering, versleuteling, recht op vergetelheid |
| Authenticatie/autorisatie | Security Testing | SSO-login, JWT-validatie, RBAC-matrix |
| Dataminimalisatie | Security Testing | Collection audit, schemacontrole |
| Consent-beheer | Security Testing | Toestemming registratie, intrekking |
| Versleutelingsvalidatie | Security Testing | TLS-controle, at-rest encryptie |
| Beveiligingsscanning | SRS | HTTPS-afdwinging, secret scanning, OWASP |

**15\. Verantwoordelijkheden**

|     |     |
| --- | --- |
| Rol | Verantwoordelijkheid |
| Ontwikkelteam | Implementatie van privacy by design; maskering; geparameteriseerde queries |
| Systeembeheerder | Geheimenbeheer; key-rotatie; toegangscontrole; monitoring |
| Privacyfunctionaris Equans | Behandeling van betrokkenenverzoeken; datalekmelding aan AP; DPA-beheer |
| Business Owner | Goedkeuring van dataverzameling; bepaling retentiebeleid |
| Technical Lead | Review van beveiligingsmaatregelen; goedkeuring van architectuurbeslissingen |

**16\. Referenties**

1.  Cavoukian, A. & Information and Privacy Commissioner of Ontario. (2011). _Privacy by Design_. Information and Privacy Commissioner of Ontario. https://www.sfu.ca/~palys/Cavoukian-2011-PrivacyByDesign-7FoundationalPrinciples.pdf (Oorspronkelijk gepubliceerd 2009)
2.  Cilwerner. (z.d.). _Overview of the Microsoft Authentication Library (MSAL) - Microsoft identity platform_. Microsoft Learn. https://learn.microsoft.com/en-us/entra/identity-platform/msal-overview
3.  _De AVG in het kort_. (2024, 23 december). Autoriteit Persoonsgegevens. Geraadpleegd op 9 januari 2026, van https://autoriteitpersoonsgegevens.nl/themas/basis-avg/avg-algemeen/de-avg-in-het-kort
4.  _Google Books_. (z.d.). https://www.google.nl/books/edition/Role_based_Access_Control/48AeIhQLWckC?hl=nl&gbpv=1&dq=Role-based+Access+Control&pg=PA283&printsec=frontcover
5.  McKay, K. A., & Cooper, D. A. (2019). _Guidelines for the selection, configuration, and use of Transport Layer Security (TLS) implementations_. https://doi.org/10.6028/nist.sp.800-52r2
6.  _OWASP Top ten web application Security Risks | OWASP Foundation_. (z.d.). https://owasp.org/www-project-top-ten/
7.  _Regulation - 2016/679 - EN - gdpr - EUR-Lex_. (z.d.). https://eur-lex.europa.eu/eli/reg/2016/679/oj
8.  _RFC 7519: JSON Web Token (JWT)_. (z.d.). IETF Datatracker. https://datatracker.ietf.org/doc/html/rfc7519