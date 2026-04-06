# Security Test Plan

## Equans Operational Insights Dashboard

| | |
|---|---|
| **Versie** | 1.0 |
| **Studentnaam** | Ahmad Alhaj Asaad (1035912) |
| **Project** | Equans Operational Insights Dashboard |
| **Opleiding** | Informatica -- Hogeschool Rotterdam |
| **Organisatie** | Equans Nederland -- SLS Digital Platforms (DevOps Forge) |
| **Begeleiders** | Viktor Klein (bedrijf), Brian Veltman (technisch), Jeroen Boogaard (school) |
| **Studiejaar** | 2025 - 2026 |
| **Referentie** | MTP-001 -- Master Test Plan, sectie 4.5 |

---

## Inhoudsopgave

1. [Inleiding](#1-inleiding)
2. [Authenticatietests](#2-authenticatietests)
3. [Autorisatietests](#3-autorisatietests)
4. [Token-beveiliging](#4-token-beveiliging)
5. [Sessiebeheer](#5-sessiebeheer)
6. [Beveiligingsheaders](#6-beveiligingsheaders)
7. [AVG-naleving en gegevensbescherming](#7-avg-naleving-en-gegevensbescherming)
8. [Traceability matrix](#8-traceability-matrix)
9. [Referenties](#9-referenties)

---

## 1. Inleiding

### 1.1 Waarom security testing?

Ergens in sprint 3 kwam ik erachter dat de backend helemaal geen JWT-validatie deed op de `/api/persons`-endpoints. Iedereen kon gewoon data opvragen zonder token. Dat was het moment dat ik dacht: ik moet dit structureel gaan testen in plaats van ad hoc dingen fixen als ik ze toevallig tegenkom. De Atlassian- en GitHub-data die in het dashboard zitten bevatten e-mailadressen, licentie-informatie en organisatiegegevens van Equans-medewerkers. Dat zijn persoonsgegevens onder de AVG en daar moet je zorgvuldig mee omgaan.

Dit document beschrijft de testgevallen voor authenticatie, autorisatie, tokenbeveiliging en AVG-conformiteit binnen het Operational Insights Dashboard. De tests zijn gebaseerd op de OWASP Top 10 (specifiek A01 Broken Access Control en A07 Identification and Authentication Failures) en de relevante AVG-artikelen (OWASP Foundation, z.d.; Verordening 2016/679, z.d.).

### 1.2 Relatie met andere testdocumenten

In het Master Test Plan (MTP-001, sectie 4.5) had ik security testing als apart testniveau opgenomen. De unit tests in het Unit Test Plan dekken al de technische correctheid van de JWT-claims parsing en de `AuthConfig`, maar die testen niet of het hele authenticatie- en autorisatieproces end-to-end werkt. Dat is wat dit document invult.

### 1.3 Welke eisen worden getest?

De tests in dit document raken aan de volgende eisen uit de Software Requirements Specification:

| Eis-ID | Omschrijving | Categorie |
|---|---|---|
| M-08 | Gebruikers authenticeren via Equans SSO (Microsoft Entra ID) | Authenticatie |
| M-09 | Alle API-endpoints vereisen JWT-authenticatie | Authenticatie |
| S-04 | Gebruikerssessies worden automatisch verlengd bij actief gebruik | Sessies |
| TM-01 | Alle communicatie via HTTPS (TLS 1.2+) | Transport |
| TM-02 | API-tokens en geheimen nooit in versiebeheer | Secrets |
| TM-03 | Alle gebruikersgerichte endpoints vereisen JWT | Autorisatie |
| TM-04 | E-mailadressen gemaskeerd in logberichten (AVG) | Privacy |
| TS-05 | JWT-tokens met passende verloopdatum (max 24 uur) | Tokens |
| TS-06 | Persoonsgegevens identificeerbaar voor verwijderingsverzoeken (AVG) | Privacy |

---

## 2. Authenticatietests

De authenticatie van het dashboard loopt via Microsoft Entra ID (voorheen Azure AD), dat OpenID Connect implementeert bovenop OAuth 2.0 (Cilwerner, z.d.; OpenID Foundation, z.d.). In de frontend wordt MSAL gebruikt om de gebruiker te authenticeren en een JWT-token op te halen. De backend valideert dat token bij elk request.

Tijdens het opzetten van deze tests viel op dat de redirect-flow na een verlopen sessie niet altijd netjes werkte. De frontend stuurde de gebruiker naar de loginpagina, maar de URL-state ging verloren. Dat soort dingen vind je alleen als je het expliciet test.

| Test-ID | Scenario | Verwacht resultaat | Eisen |
|---|---|---|---|
| AUTH-001 | Geldige SSO-login via Entra ID | Gebruiker geauthenticeerd, JWT uitgegeven, redirect naar dashboard | M-08 |
| AUTH-002 | Ongeldige inloggegevens | 401 Unauthorized, geen token uitgegeven | M-08, TM-03 |
| AUTH-003 | Verlopen sessie | Redirect naar loginpagina | M-08, S-04 |
| AUTH-004 | Token verlenging bij actief gebruik | Nieuw token uitgegeven voor verloopdatum | S-04, TS-05 |
| AUTH-005 | Uitloggen | Sessie ongeldig, token ingetrokken | M-08 |

**Detailscenario AUTH-001: Geldige SSO-login**

Voorwaarden: gebruiker bestaat in de Equans Microsoft-directory en de SSO-service is beschikbaar.

Stappen: (1) navigeer naar de loginpagina, (2) klik op "Sign in with SSO", (3) voer geldige credentials in, (4) voltooi MFA indien vereist.

Verwacht: gebruiker wordt doorgestuurd naar het dashboard, JWT-token wordt opgeslagen (niet in localStorage, conform OWASP-aanbeveling), en de gebruikerssessie is actief.

---

## 3. Autorisatietests

Het dashboard gebruikt Role-Based Access Control met twee rollen: admin en user. Dat is bewust simpel gehouden (W-03 in de SRS geeft aan dat geavanceerdere RBAC buiten scope valt). In de praktijk betekent dat: admins zien alles, users zien de dashboards en exports maar geen systeeminstellingen. Anonieme toegang is volledig geblokkeerd.

Hierbij bleek dat de rolcontrole in de `AzureAdClaims` struct case-insensitive moest zijn. Een gebruiker met de rol "viewer" (lowercase) uit Azure AD werd eerst niet herkend omdat de backend op "Viewer" (met hoofdletter) controleerde. Dat soort subtiliteiten test je met de unit tests in het UTP, maar hier test ik of het end-to-end werkt.

| Test-ID | Rol | Resource | Actie | Verwacht | Eisen |
|---|---|---|---|---|---|
| AUTHZ-001 | Admin | Alle dashboards | Bekijken | Toegestaan | M-09, TM-03 |
| AUTHZ-002 | User | Eigen team-dashboard | Bekijken | Toegestaan | M-09, TM-03 |
| AUTHZ-003 | User | Systeeminstellingen | Wijzigen | 403 Forbidden | M-09, TM-03 |
| AUTHZ-004 | Anoniem | Elk endpoint | Elke actie | 401 Unauthorized | M-09, TM-03 |

**Rollenmatrix**

| Resource | Admin | User | Anoniem |
|---|---|---|---|
| Overview Dashboard | Ja | Ja | Nee |
| Kostenrapporten | Ja | Ja | Nee |
| Data exporteren | Ja | Ja | Nee |
| Systeeminstellingen | Ja | Nee | Nee |
| Handmatige sync triggeren | Ja | Nee | Nee |

---

## 4. Token-beveiliging

JWT-tokens worden gebruikt om claims te verzenden tussen Entra ID en de Rust-backend (RFC 7519, z.d.). De backend valideert bij elk request de tokenhandtekening, de verloopdatum (`exp`), de uitgever (`iss`) en de doelgroep (`aud`). In de code zit dat in `backend/src/auth/jwt.rs` waar de `JwtValidator` struct de JWKS-keys ophaalt van Microsoft en het token verifieert.

Een uitdaging hierbij was dat de JWKS-endpoint van Microsoft af en toe traag reageerde. Hierdoor moest ik een cache inbouwen voor de signing keys zodat niet bij elk request de keys opnieuw worden opgehaald.

| Test-ID | Scenario | Verwacht resultaat | Eisen |
|---|---|---|---|
| TOK-001 | JWT met geldige handtekening | Request verwerkt | M-09, TM-03 |
| TOK-002 | JWT met ongeldige handtekening | 401 Unauthorized | M-09, TM-03 |
| TOK-003 | Verlopen token | 401 Unauthorized | TS-05 |
| TOK-004 | Token van andere uitgever | 401 Unauthorized | TM-03 |
| TOK-005 | Ongeldig geformateerd token | 400 Bad Request | TM-03 |
| TOK-006 | Token in URL-parameter | Token wordt geweigerd | TM-03 |

---

## 5. Sessiebeheer

Correct sessiebeheer voorkomt session hijacking en fixation-aanvallen (OWASP Foundation, z.d.). Na een succesvolle authenticatie via Entra ID moet er een nieuwe sessie-identifier worden aangemaakt. Sessies verlopen na inactiviteit, en bij uitloggen wordt de sessie ongeldig verklaard.

In eerste instantie had ik geen harde sessie-timeout. Viktor gaf aan dat de licentiebeheerders soms uren met het dashboard werken zonder het te sluiten. De SRS-eis S-04 schrijft voor dat sessies automatisch worden verlengd bij actief gebruik, met een maximale sessieduur van 24 uur (TS-05).

| Test-ID | Scenario | Verwacht resultaat | Eisen |
|---|---|---|---|
| SESS-001 | Sessie-timeout na inactiviteit | Gebruiker uitgelogd | S-04, TS-05 |
| SESS-002 | Sessie-verlenging bij actief gebruik | Sessie verlengd, geen onderbreking | S-04 |
| SESS-003 | Session fixation poging | Nieuwe sessie-ID na login | M-08 |
| SESS-004 | Session hijacking poging | Sessie ongeldig verklaard | M-08, TM-03 |

---

## 6. Beveiligingsheaders

HTTP-beveiligingsheaders zijn een extra verdedigingslaag die de browser instrueert om bescherming af te dwingen tegen XSS, clickjacking en MIME-type-sniffing (OWASP Foundation, z.d.; MDN, 2025). Deze headers moeten aanwezig zijn in alle responses van de applicatie.

| Header | Verwachte waarde | Doel |
|---|---|---|
| Strict-Transport-Security | `max-age=31536000; includeSubDomains` | HTTPS afdwingen (TM-01) |
| X-Content-Type-Options | `nosniff` | MIME-sniffing voorkomen |
| X-Frame-Options | `DENY` | Clickjacking voorkomen |
| Content-Security-Policy | Passende CSP-directieven | XSS-bescherming |

---

## 7. AVG-naleving en gegevensbescherming

### 7.1 Relevante AVG-artikelen

Het dashboard verwerkt persoonsgegevens van Equans-medewerkers: e-mailadressen, namen, organisatiekoppeling en licentie-informatie. Dat valt onder de AVG en daar hoort een aantal verplichtingen bij. De SRS-eisen TM-04 (e-mailmaskering in logs) en TS-06 (persoonsgegevens identificeerbaar voor verwijdering) zijn hier rechtstreeks aan gekoppeld.

| AVG-artikel | Verplichting | Test-aanpak | Eisen |
|---|---|---|---|
| Art. 5 | Dataminimalisatie | Controleren of alleen noodzakelijke data wordt verzameld | TM-04 |
| Art. 17 | Recht op vergetelheid | Testen of persoonsgegevens volledig verwijderd kunnen worden | TS-06 |
| Art. 20 | Dataportabiliteit | Testen of data exporteerbaar is in machineleesbaar formaat | S-11 |
| Art. 25 | Privacy by design | Code-review op dataverwerking | TM-04 |
| Art. 32 | Beveiliging van verwerking | Versleutelingsvalidatie, penetratietests | TM-01, TM-02 |

### 7.2 Gegevensbeschermingstests

Tijdens het ontwikkelen viel op dat de standaard Rust-logbibliotheek (`tracing`) bij een fout gewoon het hele request-object dumpt, inclusief e-mailadressen. Hierdoor moest ik een custom maskeringsfunctie schrijven die e-mails omzet naar het formaat `j***@e***.com` voordat ze in de logs terechtkomen. Dat is nu getest met een unit test (staat in het UTP), maar hier controleer ik of het in het hele systeem werkt.

| Test-ID | Categorie | Testgeval | Verwacht resultaat | Eisen |
|---|---|---|---|---|
| GDPR-001 | Logging | Controleer logs op leesbare e-mailadressen | Geen ongemaskeerde PII in logs | TM-04 |
| GDPR-002 | Foutmeldingen | Controleer error-responses op gevoelige data | Geen interne details zichtbaar | TM-04 |
| GDPR-003 | Opslag | Verifieer versleuteling in de database | PostgreSQL encryptie actief | TM-01 |
| GDPR-004 | Transport | Verifieer HTTPS op alle verbindingen | TLS 1.2+ overal | TM-01 |
| GDPR-005 | Verwijdering | Gebruikersgegevens wissen op verzoek | Alle persoonsgegevens verwijderd uit `persons`-tabel en gerelateerde tabellen | TS-06 |
| GDPR-006 | Export | Data-exportverzoek verwerken | Complete dataset in JSON/CSV-formaat | S-11 |
| GDPR-007 | Minimalisatie | API-responses bevatten alleen noodzakelijke velden | Geen overbodige PII in responses | TM-04 |

### 7.3 Datamaskering

De volgende maskeringsregels zijn geimplementeerd en worden getest:

| Datatype | Ongemaskeerd | Verwacht gemaskeerd |
|---|---|---|
| E-mailadres | john.doe@equans.com | j***@e***.com |
| IP-adres | 192.168.1.100 | 192.168.x.x |
| API-token | ghp_xxxxxxxxxxxx | ghp_*** |

### 7.4 Versleuteling

| Verbinding | Protocol | Verificatie | Eisen |
|---|---|---|---|
| Client naar frontend | TLS 1.2+ | SSL Labs scan | TM-01 |
| Frontend naar backend | TLS 1.2+ | Certificaatvalidatie | TM-01 |
| Backend naar database | TLS 1.2 | `sslmode=require` in connectiestring | TM-01 |

### 7.5 Gegevensdeling met derden

Het dashboard haalt data op bij twee externe vendors. Beide treden op als verwerkers onder de AVG, waarvoor een verwerkersovereenkomst (DPA) vereist is op grond van Art. 28 (Verordening 2016/679, z.d.).

| Vendor | Gedeelde data | Doel | DPA-status |
|---|---|---|---|
| Atlassian | User-ID`s, gebruiksdata | Licentiebeheer | Getekend |
| GitHub | User-ID`s, gebruiksdata | Licentiebeheer | Getekend |

---

## 8. Traceability matrix

Deze matrix koppelt alle security-testgevallen aan de functionele en technische eisen uit de SRS. Op basis van deze analyse kan ik concluderen dat de security-kritische eisen (M-08, M-09, TM-01, TM-03, TM-04) allemaal meervoudig gedekt zijn.

| Test-ID | Testonderwerp | Functionele eisen | Technische eisen |
|---|---|---|---|
| AUTH-001 | Geldige SSO-login | M-08 | -- |
| AUTH-002 | Ongeldige credentials | M-08 | TM-03 |
| AUTH-003 | Verlopen sessie | M-08, S-04 | -- |
| AUTH-004 | Token verlenging | S-04 | TS-05 |
| AUTH-005 | Uitloggen | M-08 | -- |
| AUTHZ-001 t/m 004 | Rolgebaseerde toegang | M-09 | TM-03 |
| TOK-001 t/m 006 | Token-validatie | M-09 | TM-03, TS-05 |
| SESS-001 t/m 004 | Sessiebeheer | M-08, S-04 | TS-05 |
| GDPR-001 t/m 002 | Logging en maskering | -- | TM-04 |
| GDPR-003 t/m 004 | Versleuteling | -- | TM-01 |
| GDPR-005 | Recht op vergetelheid | -- | TS-06 |
| GDPR-006 | Data-export | S-11 | -- |
| GDPR-007 | Dataminimalisatie | -- | TM-04 |

---

## 9. Referenties

1. Cavoukian, A. & Information and Privacy Commissioner of Ontario. (2011). *Privacy by Design*. Information and Privacy Commissioner of Ontario. (Oorspronkelijk gepubliceerd 2009)
2. Cilwerner. (z.d.). *Overview of the Microsoft Authentication Library (MSAL)*. Microsoft Learn. https://learn.microsoft.com/en-us/entra/identity-platform/msal-overview
3. *De AVG in het kort*. (2024, 23 december). Autoriteit Persoonsgegevens. https://autoriteitpersoonsgegevens.nl/themas/basis-avg/avg-algemeen/de-avg-in-het-kort
4. *Final: OpenID Connect Core 1.0 incorporating errata set 2*. (z.d.). https://openid.net/specs/openid-connect-core-1_0.html
5. *HTTP headers -- HTTP | MDN*. (2025). https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers
6. Myers, G. J., Badgett, T., & Sandler, C. (2012). *The Art of Software Testing* (Third Edition). John Wiley & Sons, Inc.
7. *OWASP Top Ten Web Application Security Risks | OWASP Foundation*. (z.d.). https://owasp.org/www-project-top-ten/
8. *Regulation 2016/679 -- General Data Protection Regulation (GDPR)*. (z.d.). EUR-Lex. https://eur-lex.europa.eu/eli/reg/2016/679/oj
9. *RFC 7519: JSON Web Token (JWT)*. (z.d.). IETF Datatracker. https://datatracker.ietf.org/doc/html/rfc7519
