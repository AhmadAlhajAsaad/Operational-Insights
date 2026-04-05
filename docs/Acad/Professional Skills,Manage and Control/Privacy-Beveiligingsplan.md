# Privacy- en beveiligingsplan

**Equans Operational Insights Dashboard**

|                 |                                                                             |
| --------------- | --------------------------------------------------------------------------- |
| **Versie**      | 1.0                                                                         |
| **Studentnaam** | Ahmad Alhaj Asaad (1035912)                                                 |
| **Project**     | Equans Operational Insights Dashboard                                       |
| **Opleiding**   | Informatica, Hogeschool Rotterdam                                           |
| **Organisatie** | Equans Nederland, SLS Digital Platforms (DevOps Forge)                      |
| **Begeleiders** | Viktor Klein (bedrijf), Brian Veltman (technisch), Jeroen Boogaard (school) |
| **Studiejaar**  | 2025 - 2026                                                                 |

---

## Inhoudsopgave

1. [Inleiding](#1-inleiding)
2. [Wettelijk kader](#2-wettelijk-kader)
3. [Verwerkte persoonsgegevens](#3-verwerkte-persoonsgegevens)
4. [Privacymaatregelen](#4-privacymaatregelen)
5. [Authenticatie en autorisatie](#5-authenticatie-en-autorisatie)
6. [Token-beveiliging en sessiebeheer](#6-token-beveiliging-en-sessiebeheer)
7. [Versleuteling en geheimenbeheer](#7-versleuteling-en-geheimenbeheer)
8. [AVG-naleving en beveiligingstests](#8-avg-naleving-en-beveiligingstests)
9. [Traceability matrix](#9-traceability-matrix)
10. [Verantwoordelijkheden](#10-verantwoordelijkheden)
11. [Referenties](#11-referenties)

---

## 1. Inleiding

Dit document beschrijft hoe binnen het Equans Operational Insights Dashboard wordt omgegaan met privacy en beveiliging, en welke tests ik daarvoor heb opgezet. Het systeem haalt gebruikers- en licentiegegevens op bij Atlassian en GitHub Enterprise, slaat die op in een PostgreSQL-database en toont ze in een React-dashboard. Omdat het hier gaat om persoonsgegevens (namen, e-mailadressen, accountstatussen) is AVG-conformiteit een harde eis.

Ergens in sprint 3 kwam ik erachter dat de backend helemaal geen JWT-validatie deed op de `/api/persons`-endpoints. Iedereen kon gewoon data opvragen zonder token. Dat was het moment dat ik dacht: dit moet ik structureel aanpakken in plaats van ad hoc dingen fixen als ik ze toevallig tegenkom. In het Master Test Plan (MTP-001, sectie 4.5) had ik security testing als apart testniveau opgenomen. De unit tests dekken de technische correctheid van de JWT-claims parsing en de `AuthConfig`, maar die testen niet of het hele authenticatie- en autorisatieproces end-to-end werkt. Dat is wat dit document invult.

Hierbij is gekozen voor een aanpak waarbij privacy by design centraal staat. Dat klinkt misschien als een buzzword, maar in de praktijk betekent het dat ik bij elke ontwerpbeslissing eerst nadenk over welke gegevens echt nodig zijn en hoe ik die bescherm. Niet achteraf een beveiligingslaagje eroverheen, maar meteen goed.

### 1.1 Scope

Dit plan geldt voor de Rust-backend (API en achtergrondtaken), de React/TypeScript-frontend, de PostgreSQL-database, alle integraties met externe API's (Atlassian Admin API, GitHub Enterprise API) en de Docker-containers in staging en productie.

### 1.2 Koppeling met SRS-eisen

De tests en maatregelen in dit document raken aan de volgende eisen uit de Software Requirements Specification:

| Eis-ID | Omschrijving                                                       |
| ------ | ------------------------------------------------------------------ |
| M-08   | Authenticatie uitsluitend via Equans SSO (Microsoft Entra ID)      |
| M-09   | Alle API-endpoints vereisen JWT-authenticatie                      |
| TM-01  | Alle communicatie via HTTPS (TLS 1.2+)                             |
| TM-02  | API-tokens en secrets via environment variabelen of Docker secrets |
| TM-03  | Alle gebruikersgerichte endpoints vereisen JWT-authenticatie       |
| TM-04  | E-mailadressen gemaskeerd in logberichten (AVG)                    |
| TM-07  | PostgreSQL 16 met versiegecontroleerde migraties (SQLx)            |
| TS-05  | JWT-tokens met passende verloopduur (max sessie 24 uur)            |
| TS-06  | Persoonsgegevens identificeerbaar voor AVG-verwijderingsverzoeken  |

## 2. Wettelijk kader

Omdat het dashboard namen, e-mailadressen en accountstatussen van Equans-medewerkers verwerkt, valt het onder de AVG (Verordening (EU) 2016/679). Dat was me eerlijk gezegd niet meteen duidelijk toen ik begon met ontwikkelen. Ik dacht eerst: we slaan toch alleen wat licentiedata op? Maar toen ik de velden in de persons-tabel op een rijtje zette (e-mail, naam, organisatie-koppeling) bleek al snel dat dit gewoon persoonsgegevens zijn waarvoor de AVG geldt.

Na het doorlezen van de verordening kwamen een paar artikelen steeds terug. Art. 5 schrijft dataminimalisatie voor, wat betekent dat ik niet meer mag opslaan dan strikt nodig is. Art. 6 gaat over de grondslag, dus waarom je uberhaupt gegevens mag verwerken. Art. 17 is het recht op vergetelheid, dat verplicht me om persoonsgegevens op verzoek te kunnen wissen. Art. 25 gaat over privacy by design en Art. 32 over beveiliging van de verwerking zelf. Niet elk artikel leidde tot evenveel werk, maar samen vormen ze wel het kader waar ik me aan moet houden (Autoriteit Persoonsgegevens, 2024).

Voor de grondslag heb ik gekozen voor gerechtvaardigd belang (Art. 6, lid 1, sub f). Equans wil weten welke medewerkers welke licenties gebruiken, wat dat kost per organisatie-eenheid en welke accounts al maanden inactief zijn. Dat is een concreet bedrijfsbelang. Hierbij was de afweging: weegt dat belang zwaarder dan de privacy van de medewerkers? Aangezien het systeem alleen werkgerelateerde gegevens verzamelt (geen prive-adressen, geen financiele data, geen bijzondere persoonsgegevens) en die data bovendien al bij Atlassian en GitHub beschikbaar is, vond ik dat proportioneel. Het dashboard brengt eigenlijk alleen samen wat al verspreid over meerdere vendor-portals staat.

## 3. Verwerkte persoonsgegevens

| Gegeven                   | Bron              | Doel                             | Retentie           |
| ------------------------- | ----------------- | -------------------------------- | ------------------ |
| Naam (displaynaam)        | Atlassian, GitHub | Identificatie bij licentiebeheer | Account-levensduur |
| E-mailadres               | Atlassian, GitHub | Matching aan vendor-accounts     | Account-levensduur |
| Account-ID                | Atlassian, GitHub | Technische koppeling             | Account-levensduur |
| Accountstatus             | Atlassian, GitHub | Actief/inactief licentiegebruik  | Account-levensduur |
| Productlicentietoewijzing | Atlassian         | Kostentoewijzing                 | 2 jaar             |
| Laatste activiteitsdatum  | Atlassian, GitHub | Detectie inactieve accounts      | 2 jaar             |
| Organisatorische eenheid  | CSV-import        | Kostentoewijzing, rapportage     | Account-levensduur |
| IP-adres (in logs)        | Backend-server    | Beveiligingsmonitoring           | 90 dagen           |

Conform Art. 5 AVG verzamelt het systeem alleen gegevens die strikt noodzakelijk zijn. In de praktijk betekent dit dat het systeem geen wachtwoorden, financiele gegevens of bijzondere persoonsgegevens opslaat. De backend filtert API-responses op relevante velden voordat ze in de database terechtkomen, en logberichten bevatten uitsluitend gemaskeerde identificatoren (TM-04). Tijdens het ontwikkelen viel op dat vendor-API's veel meer data teruggeven dan nodig is. De Atlassian Admin API stuurt bijvoorbeeld profielfoto-URL's, taalvoorkeuren en tijdzones mee die het systeem helemaal niet nodig heeft. Daarom filtert de backend alle velden eruit die niet in de `persons`-tabel thuishoren. SQL-queries draaien volledig geparametriseerd via SQLx en migraties lopen via versiegecontroleerde migratiebestanden (TM-07).

## 4. Privacymaatregelen

### 4.1 E-mailmaskering in logberichten

Alle logberichten die e-mailadressen bevatten, maskeert de backend automatisch (TM-04). Een e-mailadres als `john.doe@equans.com` verschijnt in logs als `j***@e***.com`, IP-adressen worden afgekapt tot `192.168.x.x` en API-tokens tot `ghp_***`. In eerste instantie had ik dit niet ingebouwd, maar na een review van de logoutput bleek dat de standaard Rust-logbibliotheek (`tracing`) bij een fout het hele request-object dumpt, inclusief e-mailadressen. Dat was een behoorlijke schrik. Hierdoor moest ik een custom maskeringsfunctie schrijven die e-mails omzet voordat ze in de logs terechtkomen.

### 4.2 Maskering in de frontend

Voor gebruikers zonder beheerdersrol maskeert de backend e-mailadressen via de `mask_email`-functie. Beheerders zien het volledige adres, alle andere gebruikers zien de gemaskeerde variant.

| Rol       | Weergave            |
| --------- | ------------------- |
| Beheerder | john.doe@equans.com |
| Gebruiker | j\*\*\*@e\*\*\*.com |

### 4.3 Anonimisering in staging

In de staging-omgeving anonimiseer ik alle persoonsgegevens. E-mailadressen krijgen een hash, namen vervang ik door Faker-gegenereerde waarden en account-ID's krijgen een UUID-vervanging. Hiermee voorkom ik dat productiegegevens in testomgevingen terechtkomen. Dit was overigens een bewuste keuze na overleg met Viktor, die aangaf dat de staging-omgeving soms door meerdere teams tegelijk wordt gebruikt.

## 5. Authenticatie en autorisatie

### 5.1 Authenticatie

Alle gebruikersinteractie vereist authenticatie via Equans SSO (Microsoft Entra ID), conform M-08. De backend valideert elk inkomend JWT-token op vier claims: `exp` (token niet verlopen), `iss` (correcte Azure AD-tenant), `aud` (correcte applicatie-ID) en de JWKS-handtekening. Op het `/health`-endpoint na is geen ongeauthenticeerde toegang mogelijk.

Hierbij bleek dat het valideren van tokens tegen het JWKS-endpoint een netwerkverzoek vereist bij elke request. Om de prestaties niet in gevaar te brengen cachet de backend de publieke sleutels. Tijdens het opzetten van de tests viel ook op dat de redirect-flow na een verlopen sessie niet altijd netjes werkte: de frontend stuurde de gebruiker naar de loginpagina, maar de URL-state ging verloren. Dat soort dingen vind je alleen als je het expliciet test.

### 5.2 Autorisatie (RBAC)

Het systeem kent twee rollen: admin en user. Dat heb ik bewust simpel gehouden. Admins zien alles (inclusief systeeminstellingen en synchronisatietriggers), users zien de dashboards en exports maar geen systeeminstellingen. Anonieme toegang is volledig geblokkeerd. De frontend slaat JWT-tokens op via de MSAL-bibliotheek in sessiememory (TS-05), bewust niet in localStorage vanwege XSS-kwetsbaarheid (Microsoft, z.d.).

Een uitdaging hierbij was dat de rolcontrole in de `AzureAdClaims` struct case-insensitive moest zijn. Een gebruiker met de rol "viewer" (lowercase) uit Azure AD werd eerst niet herkend omdat de backend op "Viewer" (met hoofdletter) controleerde. Dat soort subtiliteiten test je met unit tests, maar hier test ik of het end-to-end werkt.

### 5.3 Authenticatie- en autorisatietests

| Test-ID   | Scenario                                | Verwacht resultaat                                 | Eisen       |
| --------- | --------------------------------------- | -------------------------------------------------- | ----------- |
| AUTH-001  | Geldige SSO-login via Entra ID          | Gebruiker geauthenticeerd, redirect naar dashboard | M-08        |
| AUTH-002  | Ongeldige inloggegevens                 | 401 Unauthorized                                   | M-08, TM-03 |
| AUTH-003  | Verlopen sessie                         | Redirect naar loginpagina                          | M-08, S-04  |
| AUTHZ-001 | Admin benadert alle dashboards          | Toegestaan                                         | M-09, TM-03 |
| AUTHZ-002 | User wijzigt systeeminstellingen        | 403 Forbidden                                      | M-09, TM-03 |
| AUTHZ-003 | Anoniem verzoek op willekeurig endpoint | 401 Unauthorized                                   | M-09, TM-03 |

## 6. Token-beveiliging en sessiebeheer

### 6.1 Token-validatie

JWT-tokens verzenden claims tussen Entra ID en de Rust-backend (Jones et al., 2015). De backend valideert bij elk request de tokenhandtekening, de verloopdatum, de uitgever en de doelgroep. In de code zit dat in `backend/src/auth/jwt.rs` waar de `JwtValidator` struct de JWKS-keys ophaalt.

Een uitdaging hierbij was dat de JWKS-endpoint van Microsoft af en toe traag reageerde. Hierdoor moest ik een cache inbouwen voor de signing keys, zodat de backend niet bij elk request de keys opnieuw ophaalt. Dat was trouwens ook nodig om de prestatie-eis van P95 < 200ms te halen.

| Test-ID | Scenario                       | Verwacht resultaat | Eisen       |
| ------- | ------------------------------ | ------------------ | ----------- |
| TOK-001 | JWT met geldige handtekening   | Request verwerkt   | M-09, TM-03 |
| TOK-002 | JWT met ongeldige handtekening | 401 Unauthorized   | M-09, TM-03 |
| TOK-003 | Verlopen token                 | 401 Unauthorized   | TS-05       |
| TOK-004 | Ongeldig geformateerd token    | 400 Bad Request    | TM-03       |

### 6.2 Sessiebeheer

Sessies verlopen na inactiviteit en de MSAL-bibliotheek verlengt ze automatisch bij actief gebruik (S-04), met een maximale sessieduur van 24 uur (TS-05). Bij uitloggen maakt het systeem de sessie ongeldig. In eerste instantie had ik geen harde sessie-timeout ingesteld. Viktor gaf aan dat licentiebeheerders soms uren met het dashboard werken zonder het te sluiten, vandaar de eis voor automatische verlenging. Dat was een goed punt, want zonder die verlenging zouden gebruikers midden in hun werk uitgelogd raken.

| Test-ID  | Scenario                             | Verwacht resultaat                 | Eisen       |
| -------- | ------------------------------------ | ---------------------------------- | ----------- |
| SESS-001 | Sessie-timeout na inactiviteit       | Gebruiker uitgelogd                | S-04, TS-05 |
| SESS-002 | Sessie-verlenging bij actief gebruik | Sessie verlengd, geen onderbreking | S-04        |

## 7. Versleuteling en geheimenbeheer

Alle communicatie verloopt via TLS 1.2+ (TM-01). Dat geldt voor de verbinding tussen browser en frontend, frontend en backend, backend en PostgreSQL (`sslmode=require`) en backend naar de Atlassian en GitHub API's. De PostgreSQL-database draait met AES-256 versleuteling en back-ups zijn eveneens versleuteld.

API-keys en secrets slaat het systeem op als environment variabelen of Docker secrets (TM-02). In de CI/CD-pipeline beheert GitHub Secrets de Azure AD client secrets. API-keys staan nooit in broncode, gaan nooit naar de frontend en verschijnen nooit ongemaskeerd in logs. Het bestand `.env.example` bevat uitsluitend placeholderwaarden.

Tijdens het ontwikkelen viel op dat het verrassend makkelijk is om per ongeluk een API-key in een commitbericht of logbestand te laten staan. Daarom heb ik een pre-commit check ingesteld die controleert op patronen die lijken op API-tokens. Dat bespaart een hoop stress achteraf.

De backend stuurt ook beveiligingsheaders mee bij elke response:

| Header                    | Verwachte waarde                      | Doel                    |
| ------------------------- | ------------------------------------- | ----------------------- |
| Strict-Transport-Security | `max-age=31536000; includeSubDomains` | HTTPS afdwingen (TM-01) |
| X-Content-Type-Options    | `nosniff`                             | MIME-sniffing voorkomen |
| X-Frame-Options           | `DENY`                                | Clickjacking voorkomen  |
| Content-Security-Policy   | Passende CSP-directieven              | XSS-bescherming         |

## 8. AVG-naleving en beveiligingstests

### 8.1 Gegevensbeschermingstests

Tijdens het ontwikkelen bleek dat de standaard `tracing`-crate bij een fout gewoon het hele request-object dumpt, inclusief e-mailadressen. Hierdoor schreef ik een custom maskeringsfunctie die e-mails omzet naar het formaat `j***@e***.com`. Dat heb ik al getest met een unit test, maar hier controleer ik of de maskering in het hele systeem werkt.

| Test-ID  | Testgeval                                          | Verwacht resultaat                                               | Eisen |
| -------- | -------------------------------------------------- | ---------------------------------------------------------------- | ----- |
| GDPR-001 | Controleer logs op leesbare e-mailadressen         | Geen ongemaskeerde PII in logs                                   | TM-04 |
| GDPR-002 | Controleer error-responses op gevoelige data       | Geen interne details zichtbaar                                   | TM-04 |
| GDPR-003 | Verifieer HTTPS op alle verbindingen               | TLS 1.2+ overal                                                  | TM-01 |
| GDPR-004 | Gebruikersgegevens wissen op verzoek               | Alle PII verwijderd uit `persons`-tabel en gerelateerde tabellen | TS-06 |
| GDPR-005 | API-responses bevatten alleen noodzakelijke velden | Geen overbodige PII                                              | TM-04 |

### 8.2 OWASP Top 10

Het systeem is ontwikkeld conform de OWASP Top 10 (OWASP Foundation, z.d.). Hieronder de maatregelen die ik voor de belangrijkste risico's heb genomen:

| OWASP                              | Maatregel                                                         | Eisen             |
| ---------------------------------- | ----------------------------------------------------------------- | ----------------- |
| A01 Broken Access Control          | JWT-validatie op alle endpoints; RBAC via Azure AD-groepen        | M-08, M-09, TM-03 |
| A02 Cryptographic Failures         | TLS 1.2+ voor alle verbindingen; AES-256 at rest                  | TM-01             |
| A03 Injection                      | Geparametriseerde SQL via SQLx; geen dynamische query-opbouw      | TM-07             |
| A07 Identification & Auth Failures | Korte token-levensduur (1 uur); automatische vernieuwing via MSAL | TS-05             |
| A09 Logging & Monitoring           | Structured logging met correlatie-ID's; gemaskeerde PII           | TM-04             |

### 8.3 Gegevensdeling met derden

Het dashboard haalt data op bij twee externe vendors. Beide treden op als verwerkers onder de AVG, waarvoor een verwerkersovereenkomst (DPA) is getekend op grond van Art. 28 (Verordening 2016/679).

| Vendor    | Gedeelde data           | DPA-status |
| --------- | ----------------------- | ---------- |
| Atlassian | User-ID's, gebruiksdata | Getekend   |
| GitHub    | User-ID's, gebruiksdata | Getekend   |

## 9. Traceability matrix

Op basis van deze analyse concludeer ik dat de security-kritische eisen (M-08, M-09, TM-01, TM-03, TM-04) allemaal meervoudig gedekt zijn. Onderstaande matrix koppelt alle onderdelen en testgevallen aan de SRS-eisen.

| Onderwerp / Test-ID     | SRS-eisen           |
| ----------------------- | ------------------- |
| Authenticatie (SSO)     | M-08, TM-03         |
| JWT-validatie           | M-09, TS-05         |
| HTTPS/TLS               | TM-01               |
| Geheimenbeheer          | TM-02               |
| E-mailmaskering         | TM-04               |
| Database en migraties   | TM-07               |
| AUTH-001 t/m AUTH-003   | M-08, S-04, TM-03   |
| AUTHZ-001 t/m AUTHZ-003 | M-09, TM-03         |
| TOK-001 t/m TOK-004     | M-09, TM-03, TS-05  |
| SESS-001 t/m SESS-002   | S-04, TS-05         |
| GDPR-001 t/m GDPR-005   | TM-01, TM-04, TS-06 |

## 10. Verantwoordelijkheden

| Rol                        | Verantwoordelijkheid                                                  |
| -------------------------- | --------------------------------------------------------------------- |
| Ontwikkelteam              | Implementatie privacy by design, maskering, geparametriseerde queries |
| Systeembeheerder           | Geheimenbeheer, key-rotatie, toegangscontrole, monitoring             |
| Privacyfunctionaris Equans | Behandeling betrokkenenverzoeken, datalekmelding aan AP               |
| Business Owner             | Goedkeuring dataverzameling, bepaling retentiebeleid                  |
| Technical Lead             | Review beveiligingsmaatregelen, goedkeuring architectuurbeslissingen  |

## 11. Referenties

1. Autoriteit Persoonsgegevens. (2024). _De AVG in het kort_. https://autoriteitpersoonsgegevens.nl/themas/basis-avg/avg-algemeen/de-avg-in-het-kort
2. Jones, M., Bradley, J. & Sakimura, N. (2015). _RFC 7519: JSON Web Token (JWT)_. IETF. https://datatracker.ietf.org/doc/html/rfc7519
3. Microsoft. (z.d.). _Overview of the Microsoft Authentication Library (MSAL)_. Microsoft Learn. https://learn.microsoft.com/en-us/entra/identity-platform/msal-overview
4. OWASP Foundation. (z.d.). _OWASP Top Ten Web Application Security Risks_. https://owasp.org/www-project-top-ten/
5. Verordening (EU) 2016/679 (AVG/GDPR). (z.d.). EUR-Lex. https://eur-lex.europa.eu/eli/reg/2016/679/oj
