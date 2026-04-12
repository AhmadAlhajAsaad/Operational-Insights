**Security Testing**

**Authenticatie, Autorisatie & AVG-gegevensbescherming**

- Projecttitel: Equans Operational Insights Dashboard 
- Studentnaam: Ahmad Alhaj Asaad 
- Opleiding: HBO-ICT Software Engineering 
- Organisatie: Equans / SLS-DP-DevOps-Forge 
- Technisch begeleider: Brian Veltman
- Studiejaar: 2025 - 2026 
- Versie: 1.0 

Inhoudsopgave

[**1\. Overzicht** 3](#_Toc224420117)

[**2\. Authenticatietestgevallen** 4](#_Toc224420118)

[**3\. Autorisatietestgevallen** 6](#_Toc224420119)

[**4\. Token-beveiligingstesten** 7](#_Toc224420120)

[**5\. Sessiebeheer-testen** 8](#_Toc224420121)

[**6\. Validatie van beveiligingsheaders** 9](#_Toc224420122)

[**7\. AVG-nalevingsvereisten** 10](#_Toc224420123)

[**8\. Testgevallen gegevensbescherming** 11](#_Toc224420124)

[**9\. Validatie van datamaskering** 12](#_Toc224420125)

[**10\. Testen van het recht op vergetelheid** 13](#_Toc224420126)

[**11\. Testen van dataportabiliteit** 14](#_Toc224420127)

[**12\. Versleutelingsvalidatie** 15](#_Toc224420128)

[**13\. Testen van dataminimalisatie** 16](#_Toc224420129)

[**14\. Testen van toestemmingsbeheer** 17](#_Toc224420130)

[**15\. Gegevensdeling met derden** 18](#_Toc224420131)

[**16\. Referenties** 19](#_Toc224420132)

**1\. Overzicht**

Dit document beschrijft testgevallen voor het verifiëren van authenticatie- en autorisatiemechanismen, inclusief SSO-integratie, JWT-validatie en rolgebaseerde toegangscontrole, alsmede voor het waarborgen van AVG-conformiteit en gegevensbescherming binnen het Equans Operational Insights platform.

Het systematisch testen van authenticatiecontroles is een erkende beveiligingsengineeringpraktijk en is vereist om naleving aan te tonen van OWASP Top 10-categorieën A01 (Broken Access Control) en A07 (Identification and Authentication Failures) (_OWASP Top Ten Web Application Security Risks | OWASP Foundation_, z.d.). JWT-gebaseerde authenticatie en SSO via OAuth 2.0 / OpenID Connect zijn industriestandaarden voor het beveiligen van API-gedreven applicaties (_RFC 7519: JSON Web Token (JWT)_, z.d.; _Final: OpenID Connect Core 1.0 Incorporating Errata Set 2_, z.d.). De Algemene Verordening Gegevensbescherming (AVG) legt bovendien strikte verplichtingen op aan organisaties die persoonsgegevens van EU-ingezetenen verwerken (_Regulation - 2016/679 - EN - Gdpr - EUR-Lex_, z.d.). Het systematisch testen van gegevensbeschermingsmaatregelen is een erkende best practice voor het aantonen van naleving en het beperken van het risico op datalekken (Cavoukian & Information and Privacy Commissioner of Ontario, 2009/2011; _OWASP Top Ten Web Application Security Risks | OWASP Foundation_, z.d.).

**2\. Authenticatietestgevallen**

Authenticatie verifieert de identiteit van een gebruiker of systeem voordat toegang wordt verleend. De onderstaande testgevallen valideren de SSO-integratie via Microsoft Azure Active Directory (Entra ID), dat het OpenID Connect-protocol implementeert bovenop OAuth 2.0 (Cilwerner, z.d.; _Final: OpenID Connect Core 1.0 Incorporating Errata Set 2_, z.d.). Door het gehele document wordt een gestructureerde testontwerpaanpak gehanteerd (Myers et al., 2012).

|          |                     |                                    |
| -------- | ------------------- | ---------------------------------- |
| Test ID  | Scenario            | Expected Behavior                  |
| AUTH-001 | Valid SSO login     | User authenticated, JWT issued     |
| AUTH-002 | Invalid credentials | 401 Unauthorized, error message    |
| AUTH-003 | Expired session     | Redirect to login                  |
| AUTH-004 | Token refresh       | New token issued before expiry     |
| AUTH-005 | Logout              | Session invalidated, token revoked |

**Gedetailleerde testscenario's**

**AUTH-001: Valid SSO Login**

**Preconditions:**

- User exists in Equans SSO / Microsoft directory
- SSO service is available

**Steps:**

1.  Navigate to application login page
2.  Click "Sign in with SSO"
3.  Enter valid credentials
4.  Complete MFA if required

**Expected Result:**

- User redirected to dashboard
- JWT token stored in secure cookie (not in localStorage, per OWASP recommendation — _OWASP Top Ten Web Application Security Risks | OWASP Foundation_, z.d.)
- User session created

**AUTH-002: Invalid Credentials**

**Steps:**

1.  Attempt login with invalid credentials

**Expected Result:**

- 401 Unauthorized response
- Error message: "Invalid credentials"
- No token issued
- Failed attempt logged

**3\. Autorisatietestgevallen**

Autorisatie bepaalt welke acties een geauthenticeerde identiteit mag uitvoeren. Het platform implementeert Role-Based Access Control (RBAC), waarbij machtigingen worden toegewezen aan rollen in plaats van direct aan individuele gebruikers (Hu et al., 2014). Dit model is consistent met NIST SP 800-162 en vermindert het risico op privilege-escalatie (Hu et al., 2014). Schendingen moeten resulteren in HTTP 403 Forbidden-reacties, zoals gespecificeerd in RFC 9110 (Ed, 2022).

|           |           |                      |        |                 |
| --------- | --------- | -------------------- | ------ | --------------- |
| Test ID   | Role      | Resource             | Action | Expected Result |
| AUTHZ-001 | Admin     | All dashboards       | View   | Allowed         |
| AUTHZ-003 | User      | Own team dashboard   | View   | Allowed         |
| AUTHZ-004 | User      | Other team dashboard | View   | Denied (403)    |
| AUTHZ-005 | User      | Admin settings       | Modify | Denied (403)    |
| AUTHZ-006 | Anonymous | Any resource         | Any    | Denied (401)    |

**Rollenrechtenmatrix**

|                        |       |      |           |
| ---------------------- | ----- | ---- | --------- |
| Resource               | Admin | User | Anonymous |
| Overview Dashboard     | ✓     | ✓    | ✗         |
| Team Dashboard (own)   | ✓     | ✓    | ✗         |
| Team Dashboard (other) | ✓     | ✗    | ✗         |
| Cost Reports           | ✓     | ✓    | ✗         |
| Export Data            | ✓     | ✓    | ✗         |
| System Settings        | ✓     | ✗    | ✗         |

**4\. Token-beveiligingstesten**

JSON Web Tokens (JWT) worden gebruikt om claims te verzenden tussen de identiteitsprovider (Azure AD) en de backend-API (_RFC 7519: JSON Web Token (JWT)_, z.d.). De backend moet op elk verzoek de tokenhandtekening, vervaldatum (exp), uitgever (iss) en doelgroep (aud) valideren om tokenvervalsing en replay-aanvallen te voorkomen (_RFC 7519: JSON Web Token (JWT)_, z.d.; _OWASP Top Ten Web Application Security Risks | OWASP Foundation_, z.d.).

**Security Checklist**

- \[ \] JWT signature validation (_RFC 7519: JSON Web Token (JWT)_, z.d.)
- \[ \] Token expiration enforcement
- \[ \] Token not exposed in URLs or logs
- \[ \] Refresh token rotation
- \[ \] Cross-site request forgery (CSRF) protection

**Token Validation Tests**

|         |                             |                        |
| ------- | --------------------------- | ---------------------- |
| Test ID | Scenario                    | Expected Result        |
| TOK-001 | Valid JWT signature         | Request processed      |
| TOK-002 | Invalid JWT signature       | 401 Unauthorized       |
| TOK-003 | Expired token               | 401 Unauthorized       |
| TOK-004 | Token from different issuer | 401 Unauthorized       |
| TOK-005 | Malformed token             | 400 Bad Request        |
| TOK-006 | Token in URL parameter      | Token must be rejected |

**JWT Security Test Example**

Figuur Voorbeeld van JWT-beveiligingstests in Rust

Deze tests waarborgen dat ongeldige en verlopen JWT-tokens correct worden afgewezen door de Rust-backend.

**5\. Sessiebeheer-testen**

Correct sessiebeheer is cruciaal om session hijacking en fixation-aanvallen te voorkomen (_OWASP Top Ten Web Application Security Risks | OWASP Foundation_, z.d.). Na authenticatie moet een nieuwe sessie-identifier worden uitgegeven om session fixation te voorkomen (SESS-003). Sessietokens moeten gekoppeld zijn aan de geauthenticeerde gebruiker en worden ongeldig verklaard bij uitloggen of time-out, in overeenstemming met richtlijnen voor veilig sessiebeheer (Myers et al., 2012; _OWASP Top Ten Web Application Security Risks | OWASP Foundation_, z.d.).

|          |                           |                                  |
| -------- | ------------------------- | -------------------------------- |
| Test ID  | Scenario                  | Expected Result                  |
| SESS-001 | Session timeout           | User logged out after inactivity |
| SESS-002 | Concurrent sessions       | Policy enforced (allow/deny)     |
| SESS-003 | Session fixation          | New session ID after login       |
| SESS-004 | Session hijacking attempt | Session invalidated              |

**6\. Validatie van beveiligingsheaders**

HTTP-beveiligingsheaders zijn een defence-in-depth-maatregel die de browser instrueert aanvullende bescherming af te dwingen tegen veelvoorkomende aanvallen zoals cross-site scripting (XSS), clickjacking en MIME-type sniffing (_OWASP Top Ten Web Application Security Risks | OWASP Foundation_, z.d.; _HTTP Headers - HTTP | MDN_, 2025). De onderstaande headers moeten aanwezig zijn in alle reacties van de applicatie.

|                           |                                     |
| ------------------------- | ----------------------------------- |
| Header                    | Expected Value                      |
| Strict-Transport-Security | max-age=31536000; includeSubDomains |
| X-Content-Type-Options    | nosniff                             |
| X-Frame-Options           | DENY                                |
| Content-Security-Policy   | Appropriate CSP directives          |
| X-XSS-Protection          | 1; mode=block                       |

**7\. AVG-nalevingsvereisten**

De volgende AVG-artikelen worden direct behandeld door dit testplan (_Regulation - 2016/679 - EN - Gdpr - EUR-Lex_, z.d.):

|              |                        |                                            |
| ------------ | ---------------------- | ------------------------------------------ |
| GDPR Article | Requirement            | Test Approach                              |
| Art. 5       | Data minimization      | Verify only necessary data collected       |
| Art. 17      | Right to erasure       | Test data deletion workflows               |
| Art. 20      | Data portability       | Test data export functionality             |
| Art. 25      | Privacy by design      | Review data handling in code               |
| Art. 32      | Security of processing | Penetration testing, encryption validation |

Art. 25 (Privacy by Design) weerspiegelt het principe van Cavoukian en Information and Privacy Commissioner of Ontario (2009/2011) dat privacybescherming standaard in technologie moet zijn ingebouwd, in plaats van achteraf te worden toegevoegd. Art. 32 vereist dat organisaties passende technische maatregelen implementeren zoals versleuteling en pseudonimisering om een beveiligingsniveau te waarborgen dat passend is bij het risico (_Regulation - 2016/679 - EN - Gdpr - EUR-Lex_, z.d.).

**8\. Testgevallen gegevensbescherming**

De onderstaande testgevallen zijn afgeleid van de AVG-nalevingsvereisten en volgen een gestructureerde testontwerpaanpak (Myers et al., 2012). Elk testgeval heeft betrekking op één of meer AVG-verplichtingen.

|          |           |                                  |                                 |
| -------- | --------- | -------------------------------- | ------------------------------- |
| Test ID  | Category  | Test Case                        | Expected Result                 |
| GDPR-001 | Logging   | Check logs for PII               | No unmasked email addresses     |
| GDPR-002 | Logging   | Check error messages             | No sensitive data exposed       |
| GDPR-003 | Storage   | Verify encryption at rest        | Database encryption enabled     |
| GDPR-004 | Transit   | Verify encryption in transit     | HTTPS only, TLS 1.2+            |
| GDPR-005 | Retention | Data older than retention period | Automatically purged            |
| GDPR-006 | Export    | User data export request         | Complete data package generated |
| GDPR-007 | Deletion  | User deletion request            | All user data removed           |

**9\. Validatie van datamaskering**

Pseudonimisering en maskering van persoonsgegevens in logs en niet-geprivilegieerde weergaven zijn vereist op grond van Art. 32 AVG en aanbevolen als belangrijke technische maatregel (_Regulation - 2016/679 - EN - Gdpr - EUR-Lex_, z.d.; _De AVG in het Kort_, 2024). De volgende testen valideren dat geen Personally Identifiable Information (PII) wordt blootgesteld aan onbevoegde partijen.

**Email Masking Test**

Figuur Unit test voor e-mailmaskering

Deze test verifieert dat de maskeringsfunctie e-mailadressen correct anonimiseert, zoals vereist voor AVG-compliance.

**Log Output Validation**

|            |                     |                        |
| ---------- | ------------------- | ---------------------- |
| Data Type  | Raw Value           | Expected Masked Output |
| Email      | john.doe@equans.com | j\*\*\*@e\*\*\*.com    |
| IP Address | 192.168.1.100       | 192.168.x.x            |
| User ID    | user-12345          | user-\*\*\*\*\*        |
| API Token  | ghp_xxxxxxxxxxxx    | ghp\_\*\*\*            |

**10\. Testen van het recht op vergetelheid**

Het recht op vergetelheid ("right to be forgotten") is vastgelegd in Art. 17 AVG (_Regulation - 2016/679 - EN - Gdpr - EUR-Lex_, z.d.) Organisaties moeten persoonsgegevens zonder onredelijke vertraging kunnen wissen op verzoek. Testen moeten verifiëren dat verwijdering volledig en onomkeerbaar is in alle gegevensopslag (_De AVG in het Kort_, 2024).

**Testscenario: Complete User Data Deletion**

**Preconditions:**

- User account exists with associated data
- User has license allocation history

**Steps:**

1.  Submit deletion request for user
2.  Wait for processing (async operation)
3.  Verify data removal

**Verification Checklist:**

- \[ \] User record removed from users table
- \[ \] User removed from team_members table
- \[ \] License history anonymized or deleted
- \[ \] Audit logs preserved (with anonymized user reference)
- \[ \] External systems notified (if applicable)

**11\. Testen van dataportabiliteit**

Het recht op dataportabiliteit (Art. 20 AVG) vereist dat persoonsgegevens worden verstrekt in een gestructureerd, gangbaar en machineleesbaar formaat (_Regulation - 2016/679 - EN - Gdpr - EUR-Lex_, z.d.). Het JSON-formaat dat wordt gebruikt voor data-exports voldoet aan deze vereiste (_RFC 8259: The JavaScript Object Notation (JSON) Data Interchange Format_, z.d.).

**Testscenario: User Data Export**

**Steps:**

1.  User requests data export
2.  System generates export package

**Expected Export Contents:**

Figuur JSON-exportvoorbeeld voor recht op overdraagbaarheid (Art. 20 AVG)

Dit voorbeeld toont hoe persoonsgegevens worden geëxporteerd in een machine-leesbaar JSON-formaat, waarmee wordt voldaan aan het recht op overdraagbaarheid van de AVG.

**Validation:**

- \[ \] All user data included
- \[ \] Format is machine-readable (JSON)
- \[ \] Export generated within 30 days of request (Art. 12 AVG response deadline)
- \[ \] Download link expires after use

**12\. Versleutelingsvalidatie**

Versleuteling is een primaire technische maatregel die verplicht is op grond van Art. 32 AVG (_Regulation - 2016/679 - EN - Gdpr - EUR-Lex_, z.d.). NIST beveelt minimaal AES-256 aan voor data-at-rest en TLS 1.2 of hoger voor data-in-transit (McKay & Cooper, 2019). Deze standaarden zijn overgenomen in het gehele platform.

**At-Rest Encryption**

|              |                   |                                   |
| ------------ | ----------------- | --------------------------------- |
| Component    | Encryption Method | Verification                      |
| PostgreSQL   | TDE/AES-256       | Check pg_settings for encryption  |
| Backups      | AES-256           | Verify backup encryption settings |
| File Storage | AES-256           | Check volume encryption           |

**In-Transit Encryption**

|                    |          |                               |
| ------------------ | -------- | ----------------------------- |
| Connection         | Protocol | Verification                  |
| Client → Frontend  | TLS 1.2+ | SSL Labs scan                 |
| Frontend → Backend | TLS 1.2+ | Certificate validation        |
| Backend → Database | TLS 1.2  | sslmode=require in connection |

**13\. Testen van dataminimalisatie**

Het principe van dataminimalisatie (Art. 5, lid 1, sub c AVG) vereist dat persoonsgegevens toereikend, ter zake dienend en beperkt zijn tot wat noodzakelijk is voor de doeleinden waarvoor zij worden verwerkt (_Regulation - 2016/679 - EN - Gdpr - EUR-Lex_, z.d.). De onderstaande collectie-audit verifieert dat elk datapunt een gedocumenteerde zakelijke rechtvaardiging en een gedefinieerde retentieperiode heeft (_De AVG in het Kort_, 2024).

**Collection Audit**

|                      |                         |                  |
| -------------------- | ----------------------- | ---------------- |
| Data Point Collected | Business Justification  | Retention Period |
| User email           | Identity, notifications | Account lifetime |
| License usage        | Billing, analytics      | 2 years          |
| Login history        | Security audit          | 1 year           |
| IP addresses         | Security monitoring     | 90 days          |

**Validation Tests**

|         |                                        |                           |
| ------- | -------------------------------------- | ------------------------- |
| Test ID | Scenario                               | Expected Result           |
| MIN-001 | API response contains only needed data | No extra PII in responses |
| MIN-002 | Database stores only required fields   | Schema matches spec       |
| MIN-003 | Logs contain minimal PII               | Only masked identifiers   |

**14\. Testen van toestemmingsbeheer**

Waar toestemming wordt gebruikt als wettelijke grondslag voor gegevensverwerking, vereist Art. 7 AVG dat toestemming vrij, specifiek, geïnformeerd en ondubbelzinnig wordt gegeven (_Regulation - 2016/679 - EN - Gdpr - EUR-Lex_, z.d.). Toestemming moet even gemakkelijk intrekbaar zijn als te verlenen. De volgende testen verifiëren de correcte implementatie van toestemmingsbeheer (_De AVG in het Kort_, 2024).

|         |                            |                                           |
| ------- | -------------------------- | ----------------------------------------- |
| Test ID | Scenario                   | Expected Result                           |
| CON-001 | First login consent prompt | User must accept before proceeding        |
| CON-002 | Consent withdrawal         | Data processing stopped                   |
| CON-003 | Consent audit trail        | All consent changes logged with timestamp |

**15\. Gegevensdeling met derden**

Waar persoonsgegevens worden gedeeld met externe leveranciers die optreden als verwerkers, is een Data Processing Agreement (DPA) wettelijk vereist op grond van Art. 28 AVG (_Regulation - 2016/679 - EN - Gdpr - EUR-Lex_, z.d.). Elke DPA moet het onderwerp, de duur, de aard en het doel van de verwerking specificeren. Onderstaande tabel documenteert de huidige DPA-status per leverancier.

**Vendor Data Handling**

|           |                 |                    |            |
| --------- | --------------- | ------------------ | ---------- |
| Vendor    | Data Shared     | Purpose            | DPA Status |
| Atlassian | User IDs, usage | License management | Signed     |
| GitHub    | User IDs, usage | License management | Signed     |
| JFrog     | Usage metrics   | License management | Signed     |

**DPA = Data Processing Agreement** (Art. 28 AVG — _Regulation - 2016/679 - EN - Gdpr - EUR-Lex_, z.d.)

**16\. Referenties**

1.  Cavoukian, A. & Information and Privacy Commissioner of Ontario. (2011). _Privacy by Design_. Information and Privacy Commissioner of Ontario. https://www.sfu.ca/~palys/Cavoukian-2011-PrivacyByDesign-7FoundationalPrinciples.pdf (Oorspronkelijk gepubliceerd 2009)
2.  Cilwerner. (z.d.). _Overview of the Microsoft Authentication Library (MSAL) - Microsoft identity platform_. Microsoft Learn. https://learn.microsoft.com/en-us/entra/identity-platform/msal-overview
3.  _De AVG in het kort_. (2024, 23 december). Autoriteit Persoonsgegevens. Geraadpleegd op 9 januari 2026, van https://autoriteitpersoonsgegevens.nl/themas/basis-avg/avg-algemeen/de-avg-in-het-kort
4.  Ed, R. F. (2022, 1 juni). _RFC 9110: HTTP Semantics_. IETF Datatracker. https://datatracker.ietf.org/doc/html/rfc9110
5.  _Final: OpenID Connect Core 1.0 incorporating errata set 2_. (z.d.). https://openid.net/specs/openid-connect-core-1_0.html
6.  _Google Books_. (z.d.). https://www.google.nl/books/edition/Role_based_Access_Control/48AeIhQLWckC?hl=nl&gbpv=1&dq=Role-based+Access+Control&pg=PA283&printsec=frontcover
7.  _HTTP headers - HTTP | MDN_. (2025, 21 december). https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers#security
8.  Hu, V. C., Ferraiolo, D., Kuhn, R., Schnitzer, A., Sandlin, K., Miller, R., & Scarfone, K. (2014). _Guide to Attribute Based Access Control (ABAC) Definition and Considerations_. https://doi.org/10.6028/nist.sp.800-162
9.  McKay, K. A., & Cooper, D. A. (2019). _Guidelines for the selection, configuration, and use of Transport Layer Security (TLS) implementations_. https://doi.org/10.6028/nist.sp.800-52r2
10. Myers, G. J., Badgett, T., & Sandler, C. (2012). _THE ART OF SOFTWARE TESTING_ (Third Edition) \[Book\]. John Wiley & Sons, Inc. https://malenezi.github.io/malenezi/SE401/Books/114-the-art-of-software-testing-3-edition.pdf
11. _OWASP Top ten web application Security Risks | OWASP Foundation_. (z.d.). https://owasp.org/www-project-top-ten/
12. _Regulation - 2016/679 - EN - gdpr - EUR-Lex_. (z.d.). https://eur-lex.europa.eu/eli/reg/2016/679/oj
13. _RFC 7519: JSON Web Token (JWT)_. (z.d.). IETF Datatracker. https://datatracker.ietf.org/doc/html/rfc7519
14. _RFC 8259: The JavaScript Object Notation (JSON) Data Interchange Format_. (z.d.). IETF Datatracker. https://datatracker.ietf.org/doc/html/rfc8259
