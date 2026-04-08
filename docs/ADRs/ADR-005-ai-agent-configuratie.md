# ADR-005: AI Agent Configuratie

**Status:** Accepted
**Datum:** 2026-02-16
**Auteur(s):** Solutions Architect Agent

---

## 1. Context

Het Equans Operational Insights project groeit in complexiteit en vereist consistente code kwaliteit, security standaarden, en architectuur beslissingen. Handmatige code reviews zijn tijdrovend en kunnen inconsistent zijn tussen verschillende reviewers.

GitHub Copilot ondersteunt custom agents die gespecialiseerde assistentie kunnen bieden voor specifieke taken. We willen deze mogelijkheid benutten om:
- Security reviews te automatiseren en standaardiseren
- Code quality checks consistent te maken
- Architectuur beslissingen te ondersteunen met gespecialiseerde kennis

---

## 2. Beslissing

We introduceren drie gespecialiseerde AI agents:

| Agent | Locatie | Verantwoordelijkheid |
|-------|---------|---------------------|
| **Security Reviewer** | `.github/agents/security-reviewer.agent.md` | OWASP/CWE compliance, vulnerability scanning, secrets detection |
| **Code Reviewer** | `.github/agents/code-reviewer.agent.md` | Rust/TypeScript best practices, code quality, testing |
| **Solutions Architect** | `.github/agents/solutions-architect.agent.md` | Architectuur beslissingen, ADR generatie, tech stack keuzes |

Aanvullend worden GitHub Actions workflows geïntroduceerd:
- `security-scan.yml` - Automatische security scans bij PRs en pushes
- `code-review.yml` - Automatische code quality checks bij PRs

---

## 3. Rationale

### Waarom AI Agents?
- **Consistentie:** Elke review volgt dezelfde standaarden
- **Snelheid:** Directe feedback tijdens ontwikkeling
- **Kennis:** Gespecialiseerde expertise altijd beschikbaar
- **Schaalbaarheid:** Geen bottleneck door beperkte reviewer capaciteit

### Waarom deze drie agents?
1. **Security Reviewer** - Security is kritiek voor enterprise software. Een dedicated agent met OWASP/CWE kennis zorgt voor continue security awareness.
2. **Code Reviewer** - Rust en TypeScript hebben specifieke idiomen en best practices die niet iedereen kent.
3. **Solutions Architect** - Architectuur beslissingen hebben langdurige impact en vereisen holistische kennis.

### Waarom GitHub Actions workflows?
- Automatische checks bij elke PR
- Blokkeren van merges bij kritieke issues
- Audit trail van security scans
- Integratie met GitHub Security tab

---

## 4. Alternatieven Overwogen

### Alternatief 1: Alleen documentatie met coding guidelines
- **Nadeel:** Vereist handmatige naleving, geen automatische controle
- **Nadeel:** Inconsistente toepassing tussen teamleden

### Alternatief 2: Externe tools (SonarQube, Snyk)
- **Nadeel:** Extra infrastructuur en kosten
- **Nadeel:** Minder integratie met development workflow
- **Voordeel:** Meer mature tooling - kan later toegevoegd worden als complement

### Alternatief 3: Single "super" agent
- **Nadeel:** Te breed, minder diepgaande expertise per domein
- **Nadeel:** Moeilijker te onderhouden

---

## 5. Consequenties

### Positief
- Consistente code kwaliteit en security standaarden
- Snellere feedback loops voor developers
- Lagere drempel voor security kennis
- Gedocumenteerde best practices in agent definities
- Automatische CI/CD security gates

### Negatief
- Onderhoud van agent definities vereist
- Agents kunnen verouderde advies geven als niet bijgewerkt
- Developers kunnen teveel vertrouwen op agents zonder kritisch denken
- GitHub Actions verbruiken CI/CD minuten

### Mitigaties
- Kwartaal review van agent definities
- Agents geven suggesties, finale beslissing blijft bij developer
- Workflow runs geoptimaliseerd met caching

---

## 6. Referenties

- [GitHub Copilot Agents Documentation](https://docs.github.com/en/copilot)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Guidelines](https://rustsec.org/)
- Bestaande agents: `.github/agents/backend-engineer.agent.md`
- Project conventies: `docs/ADRs/copilot-instructions.md`
