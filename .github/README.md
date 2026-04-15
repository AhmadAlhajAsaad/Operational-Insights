# GitHub Configuration

Dit document beschrijft de GitHub-specifieke configuratie voor het Equans Operational Insights project.

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

### Workflow resultaten bekijken

1. Ga naar de **Actions** tab in GitHub
2. Selecteer de relevante workflow run
3. Bekijk de **Summary** voor een overzicht
4. Download artifacts voor gedetailleerde rapporten

## Onderhoud

- **Workflow updates**: Bij wijzigingen in dependencies, security tools of CI/CD requirements moeten workflows worden bijgewerkt.

### Workflow updates

Workflows moeten bijgewerkt worden wanneer:

- GitHub Actions versies updaten
- Nieuwe quality tools worden geïntroduceerd
- CI/CD requirements veranderen

## Gerelateerde Documentatie

- [ADR-005: AI Agent Configuratie](../docs/ADRs/ADR-005-ai-agent-configuratie.md)
- [Copilot Instructions](../docs/ADRs/copilot-instructions.md)
- [Security Testing](../docs/testing/security/)
