# GitHub Copilot Instructions  
## Equans Operational Insights

These instructions define how GitHub Copilot should assist in this repository.
Copilot must follow the architectural, security, and coding standards described below.

---

## 1. Project Context

**Project name:** Equans Operational Insights  
**Goal:**  
Build an operational insights dashboard that collects and exposes license and usage data from:
- Atlassian Cloud (Jira, Confluence, Trello)
- GitHub Enterprise Cloud

**Architecture overview:**
- Backend: Rust (API-only backend)
- Frontend: React + TypeScript
- Database: PostgreSQL
- Deployment: Docker + docker-compose
- CI/CD: GitHub Actions

This project is **NOT** a classic MVC application.

---

## 2. Architectural Principles

Copilot MUST follow these principles:

- ❌ No MVC pattern (no controllers/views/models split)
- ✅ API-first backend
- ✅ Clear separation of concerns:
  - `routes` → HTTP endpoints
  - `services` → business logic
  - `clients` → external API integrations
  - `models` → domain data structures
  - `config` → environment & settings


Copilot should **not** introduce unnecessary abstraction layers.

---

## 3. Security Rules (VERY IMPORTANT)

Copilot MUST NEVER:

- Commit API tokens, secrets, passwords, or credentials
- Hardcode secrets in Rust, TypeScript, or config files
- Generate `.env` files with real values

Copilot MUST ALWAYS:

- Read secrets from environment variables
- Use `.env.example` with placeholder values only
- Assume secrets are injected via Docker, GitHub Actions, or runtime environment

## 4. Rust Backend Guidelines
**Copilot should:**

- Prefer explicit, readable Rust over overly clever code
- Use Result<T, E> and proper error handling
- Avoid unwrap() in production code
- Use async/await consistently
- Keep files reasonably small and focused

**Testing:**

- Prefer Rust test frameworks where possible

- PowerShell scripts may exist for integration testing, but are not the default

## 5. API Design Rules

**All APIs must:**

- Be RESTful

- Return JSON

- Use clear, predictable response structures
- Include meaningful HTTP status codes

- Avoid exposing personal data unless explicitly required

**Example endpoint:**
`GET /api/atlassian/organizations/{org_id}/licenses/jira-software`


## 6. Frontend (React) Guidelines

**Copilot should:**

- Use TypeScript strictly (no any)
- Prefer functional components
- Follow existing component library and design system
- Keep UI components dumb; data logic belongs in services/hooks
- Assume data comes from backend APIs, not direct external APIs


## 7. Documentation Expectations

**Copilot should help generate:**

- ADRs (Architectural Decision Records) 
- Markdown documentation

- Clear comments explaining why, not just what

**Any architectural change must be reflected in:**
- docs/ADRs/ADR-002-project-structure.md
- A new ADR (e.g. ADR-003-project-Newstructure.md)

**8. What Copilot Should Optimize For**
- Maintainability
- Security  
- Clarity
- Alignment with Equans enterprise standards
- Ease of review by senior engineers

**9. What Copilot Should NOT Do**
- Introduce new frameworks without justification
- Change architecture implicitly
- Generate placeholder secrets that look real 
- Assume MVC patterns
- Over-engineer solutions