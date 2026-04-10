# GitHub Configuration

Dit document beschrijft de GitHub-specifieke configuratie voor het Equans Operational Insights project.

### Locaties

```
.github/
├── agents/
│   ├── backend-engineer.agent.md      # Rust backend development
│   ├── business-requirement.agent.md  # Business requirements
│   ├── code-reviewer.agent.md         # Code quality reviews
│   ├── security-reviewer.agent.md     # Security reviews
│   └── solutions-architect.agent.md   # Architecture decisions
├── prompts/
│   └── docs-agent.prompt.md           # Documentation assistance
└── workflows/
    ├── code-review.yml                # Automated code quality checks
    └── security-scan.yml              # Automated security scanning
```

## GitHub Actions Workflows

### Security Scan (`security-scan.yml`)

Automatische security scanning bij elke push naar `main` en bij PRs.

**Triggers:**

- Push naar `main`
- Pull request naar `main`
- Wekelijks (maandag 9:00 UTC)

**Jobs:**
| Job | Beschrijving | Blokkerend |
|-----|--------------|------------|
| `rust-security` | `cargo audit` voor Rust dependencies | Ja (bij critical/high) |
| `frontend-security` | `npm audit` voor frontend dependencies | Ja (bij critical) |
| `secret-scanning` | Scan op gelekte secrets | Ja |
| `dependency-review` | License en vulnerability check | Ja (bij high severity) |

### Code Review (`code-review.yml`)

Automatische code quality checks bij pull requests.

**Triggers:**

- Pull request naar `main` (opened, synchronize, reopened)

**Jobs:**
| Job | Beschrijving | Blokkerend |
|-----|--------------|------------|
| `rust-quality` | `cargo fmt`, `cargo clippy`, tests | Ja |
| `frontend-quality` | ESLint, TypeScript check, tests | Ja |
| `code-analysis` | TODO check, documentatie coverage | Nee (informatief) |

## Gebruik

### Agent aanroepen

In VS Code met GitHub Copilot Chat:

```
@security-reviewer Review deze code op security issues

@code-reviewer Analyseer dit endpoint voor best practices

@solutions-architect Ik wil caching toevoegen, wat zijn de opties?
```

### Workflow resultaten bekijken

1. Ga naar de **Actions** tab in GitHub
2. Selecteer de relevante workflow run
3. Bekijk de **Summary** voor een overzicht
4. Download artifacts voor gedetailleerde rapporten

## Onderhoud

### Agent updates

Agents moeten bijgewerkt worden wanneer:

- Nieuwe dependencies worden toegevoegd
- Project structuur verandert
- Security standaarden updaten (bijv. nieuwe OWASP versie)
- Best practices evolueren

**Verantwoordelijke:** Team lead of security officer
**Frequentie:** Minimaal elk kwartaal reviewen

### Workflow updates

Workflows moeten bijgewerkt worden wanneer:

- GitHub Actions versies updaten
- Nieuwe quality tools worden geïntroduceerd
- CI/CD requirements veranderen

## Gerelateerde Documentatie

- [ADR-005: AI Agent Configuratie](../docs/ADRs/ADR-005-ai-agent-configuratie.md)
- [Copilot Instructions](../docs/ADRs/copilot-instructions.md)
- [Security Testing](../docs/testing/security/)
