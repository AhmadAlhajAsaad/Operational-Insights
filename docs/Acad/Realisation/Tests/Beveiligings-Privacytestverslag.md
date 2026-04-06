# Beveiligings- en privacytestverslag

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
| **Referenties** | Privacy-Beveiligingsplan v1.0, Security Test Plan v1.0, MTP-001             |

---

## Inhoudsopgave

1. [Inleiding](#1-inleiding)
2. [Testomgeving en opzet](#2-testomgeving-en-opzet)
3. [PII-maskering](#3-pii-maskering-tm-04)
4. [Authenticatie en JWT-validatie](#4-authenticatie-en-jwt-validatie-m-08-m-09)
5. [Autorisatie en rolcontrole](#5-autorisatie-en-rolcontrole-tm-03)
6. [Beveiligingsheaders en CORS](#6-beveiligingsheaders-en-cors-tm-01)
7. [AVG-naleving: recht op vergetelheid](#7-avg-naleving-recht-op-vergetelheid-ts-06)
8. [Geheimenbeheer](#8-geheimenbeheer-tm-02)
9. [Samenvatting testresultaten](#9-samenvatting-testresultaten)
10. [Conclusie](#10-conclusie)
11. [Referenties](#11-referenties)

---

## 1. Inleiding

Halverwege sprint 3 liep ik tegen iets aan waar ik achteraf van schrok: de /api/persons-endpoints deden helemaal geen JWT-validatie. Iedereen kon zonder token gewoon data opvragen. Dat klopte niet, dus ik besloot om beveiliging niet langer als losse tickets te behandelen maar het structureel aan te pakken.

Dit verslag beschrijft wat ik heb getest, welke code daarvoor geschreven is en wat de uitkomsten waren. De testgevallen komen uit het Security Test Plan. Hier laat ik zien wat er daadwerkelijk werkt (en waar ik tegenaan liep).

### 1.1 Scope

| Gebied                | SRS-eisen         | Testcategorie       |
| --------------------- | ----------------- | ------------------- |
| PII-maskering in logs | TM-04             | Gegevensbescherming |
| JWT-authenticatie     | M-08, M-09, TM-03 | Authenticatie       |
| Rolgebaseerde toegang | M-09, TM-03       | Autorisatie         |
| Beveiligingsheaders   | TM-01             | Transport           |
| CORS-configuratie     | TM-01             | Transport           |
| GDPR-verwijdering     | TS-06             | Privacy             |
| Geheimenbeheer        | TM-02             | Secrets             |

---

## 2. Testomgeving en opzet

De backend draait op Rust (Edition 2021) met Axum 0.7 als webframework en SQLx 0.8 voor de database (PostgreSQL 16). Alle unit tests draai ik met cargo test --lib. Die tests werken in-memory zonder databaseverbinding, wat ze snel maakt. De hele suite van 33 tests is klaar in 0.02 seconde.

```text
running 33 tests
test auth::claims::tests::test_has_role ... ok
test auth::claims::tests::test_in_group ... ok
test auth::claims::tests::test_is_admin_via_role ... ok
test auth::claims::tests::test_is_admin_via_group ... ok
test auth::claims::tests::test_user_id_prefers_upn ... ok
test auth::jwt::tests::test_auth_config_from_env ... ok
test auth::jwt::tests::test_jwks_uri_construction ... ok
test security::masking::tests::test_mask_email ... ok
test security::masking::tests::test_mask_ip ... ok
test security::masking::tests::test_mask_token ... ok
test security::masking::tests::test_sanitize_for_logging ... ok
[... 22 overige functionele tests ...]

test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 3. PII-maskering (TM-04)

### 3.1 Waarom dit nodig was

Dit onderdeel ontstond uit frustratie. Ik was aan het debuggen en zag dat de tracing-crate bij een error het hele request dumpt, inclusief e-mailadressen in platte tekst. Dus als iemand john.doe@equans.com opvraagt en er gaat iets mis, staat dat voluit in de logs. Dat mag gewoon niet van de AVG, en SRS-eis TM-04 schrijft ook voor dat e-mails gemaskeerd moeten zijn.

Ik heb hiervoor backend/src/security/masking.rs geschreven. In eerste instantie probeerde ik simpele string-replace, maar dat werkte niet goed voor patronen die je niet van tevoren kent. Uiteindelijk gebruik ik voorgecompileerde regex-patronen via LazyLock<Regex>.

### 3.2 Hoe de maskering werkt

De mask_email-functie laat het eerste teken van het lokale deel en het domein staan, zodat je bij debugging nog kunt zien over welk adres het gaat. Bijvoorbeeld john.doe@equans.com wordt j***@e***.com.

```rust
pub fn mask_email(email: &str) -> String {
    if let Some(at_pos) = email.find('@') {
        let local = &email[..at_pos];
        let domain = &email[at_pos + 1..];

        let masked_local = if local.is_empty() {
            "***".to_string()
        } else {
            format!("{}***", &local[..1])
        };

        let masked_domain = if let Some(dot_pos) = domain.rfind('.') {
            let domain_name = &domain[..dot_pos];
            let tld = &domain[dot_pos..];
            if domain_name.is_empty() {
                format!("***{}", tld)
            } else {
                format!("{}***{}", &domain_name[..1], tld)
            }
        } else {
            "***".to_string()
        };

        format!("{}@{}", masked_local, masked_domain)
    } else {
        "***".to_string()
    }
}
```

Voor IP-adressen houd ik de eerste twee octetten (192.168.x.x), genoeg om het subnet te herkennen maar niet het apparaat. Tokens van GitHub (ghp\_) en Atlassian (ATCTT) worden herkend aan hun prefix en krijgen \*\*\* erachter. De sanitize_for_logging-functie past alle drie regels achter elkaar toe, als catch-all voor tracing-berichten.

### 3.3 Testresultaten

Ik heb bewust een paar randgevallen meegenomen. Bijvoorbeeld a@b.nl (maar twee tekens voor de @) en een logbericht met drie soorten PII tegelijk. Dat laatste was de spannendste test, want daar moeten e-mail, IP en token allemaal in een keer gemaskeerd worden zonder dat ze elkaar in de weg zitten.

```rust
#[test]
fn test_sanitize_for_logging() {
    let input = "User john.doe@equans.com from 192.168.1.100 with token ghp_abc123";
    let result = sanitize_for_logging(input);
    assert!(!result.contains("john.doe@equans.com"));
    assert!(!result.contains("192.168.1.100"));
    assert!(!result.contains("ghp_abc123"));
    assert!(result.contains("j***@e***.com"));
    assert!(result.contains("x.x"));
    assert!(result.contains("ghp_***"));
}
```

| Test                      | Wat ik controleer                                | Resultaat |
| ------------------------- | ------------------------------------------------ | --------- |
| test_mask_email           | john.doe@equans.com wordt j***@e***.com          | Geslaagd  |
| test_mask_ip              | 192.168.1.100 wordt 192.168.x.x                  | Geslaagd  |
| test_mask_token           | ghp*abc123def456 wordt ghp*\*\*\*                | Geslaagd  |
| test_sanitize_for_logging | Gecombineerd logbericht, geen PII meer zichtbaar | Geslaagd  |

---

## 4. Authenticatie en JWT-validatie (M-08, M-09)

### 4.1 Hoe het werkt

Elk request naar een beveiligd endpoint gaat eerst langs de auth-middleware (backend/src/auth/middleware.rs). Die haalt het Bearer-token uit de Authorization-header, valideert het via JwtValidator tegen de JWKS-keys van Azure AD en controleert de verloopdatum, uitgever en doelgroep.

Wat me hier lastig viel: de foutmeldingen mochten niet te specifiek zijn. Als een token ongeldig is, wil je niet terugsturen waarom precies (dat helpt een aanvaller). Dus AuthError::InvalidToken geeft alleen "Invalid authentication token" terug, ook als het probleem eigenlijk een verlopen handtekening of verkeerde audience is.

```rust
pub async fn auth_middleware(
    State(validator): State<Arc<JwtValidator>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::MissingToken)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthError::InvalidAuthHeader)?;

    let claims = validator.validate(token).await?;
    request.extensions_mut().insert(AuthenticatedUser(claims));
    Ok(next.run(request).await)
}
```

### 4.2 Tests

De eerste test controleert of AuthConfig de environment variables correct inlaadt. De tweede verifieert de JWKS-URI constructie, want als die URL niet klopt, haalt de backend de verkeerde publieke sleutels op en faalt alles.

| Test                       | Wat ik controleer                                            | Resultaat |
| -------------------------- | ------------------------------------------------------------ | --------- |
| test_auth_config_from_env  | Tenant ID, client ID, audience laden correct vanuit env vars | Geslaagd  |
| test_jwks_uri_construction | JWKS-URI bevat juiste tenant ID voor Microsoft endpoint      | Geslaagd  |

---

## 5. Autorisatie en rolcontrole (TM-03)

### 5.1 Het probleem met hoofdletters

In sprint 3 ontdekte ik een vervelend probleem: Azure AD stuurde soms "Viewer" en soms "viewer" als rol mee. Mijn oorspronkelijke vergelijking was case-sensitive, dus sommige gebruikers konden opeens niks meer. De fix was simpel (eq_ignore_ascii_case), maar het kostte me wel een uur debuggen voordat ik doorhad waar het aan lag.

```rust
impl AzureAdClaims {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r.eq_ignore_ascii_case(role))
    }

    pub fn is_admin(&self, admin_group_id: Option<&str>) -> bool {
        self.has_role("Admin") || admin_group_id.map(|g| self.in_group(g)).unwrap_or(false)
    }
}
```

Admin-detectie kan op twee manieren: via de "Admin"-rol of via een admin-groep. Die dubbele route is nodig omdat Equans voor sommige apps rollen gebruikt en voor andere groepen.

### 5.2 Tests

Vijf tests, waarvan de meest relevante checkt dat has_role("viewer") en has_role("Viewer") allebei true geven. Daarnaast test ik de admin-detectie via groepslidmaatschap, en de user_id()-methode die een fallback-volgorde hanteert (upn, preferred_username, email, sub) voor het loggen van requests.

| Test                     | Wat ik controleer                                    | Resultaat |
| ------------------------ | ---------------------------------------------------- | --------- |
| test_has_role            | Case-insensitive ("Viewer" en "viewer" werken beide) | Geslaagd  |
| test_in_group            | Groepslidmaatschap, inclusief onbekende groep        | Geslaagd  |
| test_is_admin_via_group  | Admin via groep, verkeerde groep, en None            | Geslaagd  |
| test_is_admin_via_role   | Admin via expliciete "Admin" rol                     | Geslaagd  |
| test_user_id_prefers_upn | Fallback-volgorde voor identifier in logs            | Geslaagd  |

---

## 6. Beveiligingsheaders en CORS (TM-01)

### 6.1 Headers

In backend/src/security/headers.rs zit een middleware die bij elke response zeven headers toevoegt. Ik heb die gebaseerd op de OWASP-aanbevelingen. De middleware zit als Axum-layer op de router en geldt voor alle routes, ook de ontwikkelroutes. Achteraf gezien had ik de CSP-header strikter kunnen zetten (unsafe-inline is niet ideaal), maar voor de huidige frontend met inline styles was dat nog nodig.

| Header                    | Waarde                                     | Waarom                              |
| ------------------------- | ------------------------------------------ | ----------------------------------- |
| Strict-Transport-Security | max-age=31536000; includeSubDomains        | Dwingt HTTPS af (A02)               |
| X-Content-Type-Options    | nosniff                                    | Voorkomt MIME-sniffing (A05)        |
| X-Frame-Options           | DENY                                       | Blokkeert clickjacking (A01)        |
| Content-Security-Policy   | default-src 'self'; frame-ancestors 'none' | Beperkt XSS-risico (A03)            |
| Referrer-Policy           | strict-origin-when-cross-origin            | Beperkt info-lekken                 |
| X-XSS-Protection          | 1; mode=block                              | Legacy XSS-bescherming              |
| Permissions-Policy        | camera=(), microphone=(), geolocation=()   | Blokkeert onnodige browser-features |

### 6.2 CORS

De CORS-configuratie was een ander probleem. Oorspronkelijk stond alles op Any, wat ik tijdens ontwikkeling zo had gezet en daarna vergeten was aan te passen. Dat is een risico (OWASP A01) omdat een kwaadaardige website dan API-calls kan doen namens een ingelogde gebruiker.

Nu leest de backend CORS_ALLOWED_ORIGINS uit een environment variable. Als die niet gezet is, valt hij terug op localhost:3000 en localhost:5173 (de Vite dev server). Methoden zijn beperkt tot GET, POST, PUT, DELETE en OPTIONS; headers tot Authorization, Content-Type en Accept.

```rust
let allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
    .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".to_string());
let origins: Vec<http::HeaderValue> = allowed_origins
    .split(',')
    .filter_map(|o| o.trim().parse().ok())
    .collect();
let cors = CorsLayer::new()
    .allow_origin(origins)
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
    .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
    .allow_credentials(true);
```

---

## 7. AVG-naleving: recht op vergetelheid (TS-06)

### 7.1 Waarom hard delete

Ik heb lang getwijfeld tussen soft-delete (een deleted-vlag op de record) en hard delete (daadwerkelijk weggooien). Soft-delete is makkelijker te implementeren en je kunt het ongedaan maken. Maar de AVG Art. 17 vereist dat gegevens echt verwijderd worden als iemand een verzoek indient. Een markering is niet voldoende. Dus het moest hard delete worden.

Het endpoint DELETE /api/persons/:person_id verwijdert de persoon uit de persons-tabel plus alle gerelateerde records uit github_link_audit en atlassian_link_audit. Dat gebeurt in een transactie, want als er halverwege iets fout gaat wil ik niet dat de audit trails weg zijn maar de persoon nog bestaat (of andersom).

```rust
pub async fn delete_person(&self, person_id: &str) -> Result<bool, sqlx::Error> {
    let mut tx = self.pool.begin().await?;

    sqlx::query("DELETE FROM github_link_audit WHERE person_id = ")
        .bind(person_id).execute(&mut *tx).await?;
    sqlx::query("DELETE FROM atlassian_link_audit WHERE person_id = ")
        .bind(person_id).execute(&mut *tx).await?;
    let result = sqlx::query("DELETE FROM persons WHERE person_id = ")
        .bind(person_id).execute(&mut *tx).await?;

    tx.commit().await?;
    Ok(result.rows_affected() > 0)
}
```

### 7.2 Audit logging zonder PII

Wat ik lastig vond: je wilt loggen dat een persoon verwijderd is (voor audit), maar je mag de gegevens niet in de logs zetten (want die zijn dan zelf weer PII). De oplossing was om mask_email te gebruiken in het logbericht, zodat je ziet dat de persoon met e-mail j***@e***.com verwijderd is zonder het echte adres te kennen.

---

## 8. Geheimenbeheer (TM-02)

Om te voorkomen dat API-keys of connection strings per ongeluk in Git terechtkomen, heb ik een .pre-commit-config.yaml opgezet met detect-secrets (Yelp) en custom regex-hooks voor GitHub-tokens (ghp*, gho*) en Atlassian-tokens (ATCTT, ATATT). Daarnaast detecteert een hook PostgreSQL-connection strings.

Voor de staging-omgeving heb ik scripts/anonymize*staging.sql geschreven dat alle PII vervangt: e-mails worden person* + MD5-hash + @anonymized.local, namen worden random strings en audit trails worden volledig verwijderd. Zo bevat staging geen echte persoonsgegevens, ook niet per ongeluk.

---

## 9. Samenvatting testresultaten

### 9.1 Overzicht

| Categorie                     | Tests  | Geslaagd | Gefaald |
| ----------------------------- | ------ | -------- | ------- |
| PII-maskering (TM-04)         | 4      | 4        | 0       |
| JWT-configuratie (M-08, M-09) | 2      | 2        | 0       |
| Rolcontrole (TM-03)           | 5      | 5        | 0       |
| Beveiligingsheaders (TM-01)   | 7\*    | 7        | 0       |
| CORS-beveiliging              | 1\*    | 1        | 0       |
| GDPR-verwijdering (TS-06)     | 1\*    | 1        | 0       |
| **Totaal**                    | **20** | **20**   | **0**   |

_\* Geverifieerd via code-inspectie en compilatiecontrole._

### 9.2 Traceability naar SRS-eisen

| SRS-eis | Omschrijving                      | Gedekt door                                |
| ------- | --------------------------------- | ------------------------------------------ |
| M-08    | Authenticatie via Equans SSO      | test_auth_config_from_env                  |
| M-09    | Alle endpoints vereisen JWT       | auth_middleware, test_jwks_uri             |
| TM-01   | Communicatie via HTTPS (TLS 1.2+) | security_headers_middleware (HSTS)         |
| TM-02   | Secrets niet in versiebeheer      | .pre-commit-config.yaml                    |
| TM-03   | Endpoints vereisen JWT            | auth_middleware, rolcontrole-tests         |
| TM-04   | E-mails gemaskeerd in logs        | test_mask_email, test_sanitize_for_logging |
| TS-05   | JWT-tokens max 24 uur             | Token-validatie in JwtValidator            |
| TS-06   | Persoonsgegevens verwijderbaar    | delete_person endpoint + repository        |

### 9.3 Volledige testuitvoer

```text
$ cargo test --lib

running 33 tests
test auth::claims::tests::test_has_role ... ok
test auth::claims::tests::test_in_group ... ok
test auth::claims::tests::test_is_admin_via_role ... ok
test auth::claims::tests::test_is_admin_via_group ... ok
test auth::claims::tests::test_user_id_prefers_upn ... ok
test auth::jwt::tests::test_auth_config_from_env ... ok
test auth::jwt::tests::test_jwks_uri_construction ... ok
test security::masking::tests::test_mask_email ... ok
test security::masking::tests::test_mask_ip ... ok
test security::masking::tests::test_mask_token ... ok
test security::masking::tests::test_sanitize_for_logging ... ok
[... 22 overige functionele tests ...]

test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 10. Conclusie

De beveiligingsmaatregelen uit het Privacy- en beveiligingsplan zijn geimplementeerd en getest. De PII-maskering werkt voor e-mails, IP-adressen en tokens. JWT-authenticatie draait tegen Azure AD, en de case-insensitive rolcontrole lost het probleem uit sprint 3 op.

Wat ik achteraf anders had gedaan: de beveiligingsheaders eerder toevoegen (niet pas in sprint 4) en de CORS-configuratie vanaf het begin goed instellen in plaats van met Any beginnen. Dat had me debug-tijd gescheeld. Maar de huidige stand dekt alle SRS-eisen (M-08, M-09, TM-01 t/m TM-04, TS-05, TS-06) en alle 20 tests slagen.

---

## 11. Referenties

1. Autoriteit Persoonsgegevens. (2024). _De AVG in het kort_. https://autoriteitpersoonsgegevens.nl/themas/basis-avg/avg-algemeen/de-avg-in-het-kort
2. Jones, M., Bradley, J. & Sakimura, N. (2015). _RFC 7519: JSON Web Token (JWT)_. IETF. https://datatracker.ietf.org/doc/html/rfc7519
3. Microsoft. (z.d.). _Overview of the Microsoft Authentication Library (MSAL)_. Microsoft Learn. https://learn.microsoft.com/en-us/entra/identity-platform/msal-overview
4. OWASP Foundation. (z.d.). _OWASP Top Ten Web Application Security Risks_. https://owasp.org/www-project-top-ten/
5. Verordening (EU) 2016/679 (AVG/GDPR). (z.d.). EUR-Lex. https://eur-lex.europa.eu/eli/reg/2016/679/oj
6. MDN Web Docs. (2025). _HTTP headers_. https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers
