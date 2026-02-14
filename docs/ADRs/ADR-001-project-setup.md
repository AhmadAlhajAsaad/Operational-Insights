# ADR-001: Project Setup and Repository Structure

| Metadata | Details |
|----------|---------|
| **Status** | Accepted |
| **Date** | 2025-12-04 |
| **Author** | Ahmad Alhaj Asaad |
| **Last Updated** | 2025-12-05 |

---

## Context

Equans Operational Insights is a full-stack system designed to collect, process, and visualize usage and license data from multiple platforms (Atlassian, GitHub, JFrog, and Trello).

To support scalable development and team collaboration, a clear, maintainable, and modular project structure must be established before functional development begins.

**Key concerns addressed by this ADR:**
- Repository organization and folder hierarchy
- Backend and frontend architecture separation
- Documentation structure and governance
- Foundation for continuous integration, testing, and deployment

---

## Decision

### Project Structure

We adopt the following monorepo structure with clear separation of concerns:

```
Equans-operational-insights/
├── backend/src/
│   ├── main.rs
│   ├── atlassian.rs  (combines routes, services, models)
│   ├── github.rs     (combines routes, services, models)
│   ├── health.rs     (health check handlers)
│   ├── Cargo.toml
│   └── tests/
│
├── frontend/                   # React + TypeScript dashboard
│   ├── src/
│   │   ├── components/
│   │   ├── pages/
│   │   └── App.tsx
│   ├── public/
│   ├── package.json
│   └── vite.config.ts
│
├── infra/                      # Infrastructure and deployment
│   ├── docker-compose.yml
│   ├── postgres/
│   └── scripts/
│
├── docs/
│   ├── ADRs/                   # Architecture Decision Records
│   │   ├── ADR-000-template.md
│   │   └── ADR-001-project-setup.md    
│   ├── requirements/           # Functional & non-functional specs
│   ├── architecture-overview.md
│   └── api-specs/        # API documentation and specs
│       ├── atlassian/
├── scripts/                    # Utility scripts for development
├── tests/                      # Integration tests
├── .github/
│    └── workflows/              # CI/CD pipelines (GitHub Actions)
│
├── README.md
├── .gitignore
└── package.json                # Root-level workspace config
```

### Technology Stack and Tooling Decisions

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| **Backend API** | Rust with Axum framework & SQLx | Type safety, performance, async runtime |
| **Frontend** | React 19+ with TypeScript & Vite | Modern DX, type safety, fast builds |
| **Database** | PostgreSQL | Mature, reliable, excellent querying capabilities |
| **Local Development** | Docker & Docker Compose | Reproducible environments, isolated services |
| **Deployment** | Docker containers + GitHub Actions | CI/CD automation, container orchestration ready |
| **Documentation** | Markdown + ADR structure | Version-controlled, searchable, collaborative |
| **Version Control** | GitHub (Equans DevOps Forge org) | Enterprise integration, compliance support |

### Frontend-Backend Integration

For local development, the frontend dev server (Vite) proxies `/api/*` requests to the backend:
- Frontend runs on `http://localhost:5173` (Vite default)
- Backend runs on `http://localhost:8080`
- Proxy mapping avoids CORS complexity during development
- Production builds serve static assets from backend or CDN

---

## Rationale

### Why This Structure?

**Scalability**
- Clear separation of concerns makes it easy to scale each layer independently
- Team members can work on frontend, backend, or infrastructure without conflicts
- Monorepo approach allows shared utilities and testing across layers

**Maintainability**
- ADR-based documentation provides long-term governance and decision traceability
- Consistent naming conventions and folder organization reduce cognitive load
- Standardized tech stack simplifies onboarding

**DevOps Best Practices**
- Docker containerization ensures reproducible environments across development, staging, and production
- GitHub Actions automation aligns with modern CI/CD workflows
- Markdown documentation stays version-controlled alongside code

**Developer Experience**
- Rust + Axum provides compile-time safety and excellent error messages
- React + TypeScript offers type safety and fast feedback loops
- Vite provides sub-100ms HMR during development

---

### Rust Backend

**Description:** Use Rust (Axum/SQLx) for backend API

**Why Rust:**
- Rust is the established standard for this team
- Offers superior performance characteristics for data processing
- Compile-time type safety reduces production bugs
- Better memory efficiency for long-running services

---

## Consequences

### Positive Outcomes ✓

**Clear Onboarding**
- New developers can clone the repo and follow the README for local setup
- Each folder has a clear, single responsibility
- Backend and frontend can be developed, tested, and deployed independently

**CI/CD Integration**
- GitHub Actions workflows can easily trigger tests for changed components
- Docker builds are deterministic and portable
- Deployment pipelines can target specific services

**Modular Development**
- Frontend components don't depend on backend implementation details (via API contracts)
- Backend endpoints can be stubbed for frontend development
- Each layer can use its best-fit tooling without compromise

**Documentation and Governance**
- ADRs create a searchable history of "why" decisions were made
- Future developers can understand design trade-offs without asking questions
- Technical decisions are transparent and revisable

### Negative Outcomes / Constraints ⚠

**Operational Discipline**
- Requires team commitment to maintaining ADR structure and documentation
- If documentation falls behind, confusion increases exponentially

**Developer Learning Curve**
- Rust has a steeper learning curve compared to Python or JavaScript
- SQL query optimization requires database knowledge beyond typical web dev
- Async Rust patterns take time to master

**Initial Setup Complexity**
- First-time setup involves installing Rust, Node.js, Docker, and PostgreSQL
- Some developers may need environment troubleshooting support
- Docker daemon management adds one more system dependency
---