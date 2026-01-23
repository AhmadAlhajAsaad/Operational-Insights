# Equans Operational Insights - Architecture Overview



---

## Table of Contents

1. [System Overview](#system-overview)
2. [High-Level Architecture](#high-level-architecture)
3. [Component Architecture](#component-architecture)
4. [Data Flow](#data-flow)
5. [API Integration Layer](#api-integration-layer)
6. [Security Considerations](#security-considerations)
7. [Deployment Architecture](#deployment-architecture)
8. [Future Extensions](#future-extensions)

---

## System Overview

Equans Operational Insights is a full-stack system designed to collect, process, and visualize usage and license data from multiple third-party platforms (Atlassian, GitHub, JFrog, and Trello).

**Core Purpose:** Enable IT operations teams to monitor, analyze, and optimize software licensing and usage across their organization.

**MVP Scope:**
- License usage aggregation from 4 major platforms
- Real-time dashboard visualization
- Historical trend analysis
- Automated data collection via scheduled tasks

---

## High-Level Architecture

### System Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     User Browsers                            │
└──────────────────────────┬──────────────────────────────────┘
                           │
                    HTTP/HTTPS (Vite Dev Proxy)
                           │
      ┌────────────────────┴─────────────────────┐
      │                                           │
┌─────▼─────────────────┐            ┌──────────▼──────────┐
│   React Frontend      │            │  Rust Backend API   │
│  (TypeScript, Vite)   │            │   (Axum v0.7)      │
│                       │            │                    │
│  - Dashboards        │◄──────────►│  - REST Endpoints  │
│  - Charts/Tables     │   /api/*   │  - Data Processing │
│  - Visualizations    │            │  - Scheduling       │
└───────────────────────┘            └──────────┬──────────┘
                                                 │
                    ┌────────────────────────────┼────────────────────────────┐
                    │                            │                            │
         ┌──────────▼──────────┐    ┌───────────▼────────┐    ┌─────────────▼────┐
         │    PostgreSQL       │    │  External APIs     │    │   Scheduler      │
         │     Database        │    │                    │    │  (Background)    │
         │                     │    │  - Atlassian       │    │                  │
         │  - Users            │    │  - GitHub          │    │  - Cron Jobs     │
         │  - License Usage    │    │  - JFrog           │    │  - Task Queue    │
         │  - Activity Logs    │    │  - Trello          │    └──────────────────┘
         │  - Teams            │    └────────────────────┘
         └─────────────────────┘
```

### Architecture Pattern

This project follows a **three-tier layered architecture**:

| Tier | Component | Technology | Responsibility |
|------|-----------|-----------|-----------------|
| **Presentation** | React Frontend | TypeScript, Vite, React 19 | User interface, visualization, client-side state |
| **Application** | Rust Backend | Axum, Tokio, SQLx | API endpoints, business logic, data transformation |
| **Data** | PostgreSQL | Relational Database | Persistent storage, transactions, queries |

---

## Component Architecture

### 2.1 Frontend Layer (React + TypeScript + Vite)

**Purpose:** Provide interactive dashboards and visualizations for end users.

**Key Responsibilities:**
- Display real-time license usage metrics
- Render interactive charts and tables
- Enable filtering and date range selection
- Handle user navigation and state management

**Architecture Pattern:** Component-based with reusable UI components

**Core Components:**
- `Dashboard` - Main landing page with overview metrics
- `BackendStatus` - Health check indicator for backend connectivity
- `UsageChart` - Time-series visualization component
- `LicenseTable` - Tabular display of license inventory
- `Filter` - Date range and category filters

**Development Setup:**
- Dev Server: `npm start` → Vite on `http://localhost:5173`
- Proxy Configuration: `/api/*` routes forward to `http://localhost:8080`
- Build Output: Optimized static assets in `dist/` directory

**Dependencies:**
- React 19.2 with TypeScript strict mode
- Vite 7.2 for rapid HMR (hot module replacement)
- ESLint for code quality enforcement

### 2.2 Backend Layer (Rust + Axum)

**Purpose:** Handle API requests, orchestrate data collection, and manage business logic.

**Key Responsibilities:**
- Expose RESTful API endpoints for frontend consumption
- Fetch data from external platform APIs
- Transform and normalize data into internal domain models
- Persist processed data to PostgreSQL
- Schedule periodic data collection tasks

**Architecture Pattern:** Modular services with clear separation of concerns

**Core Modules:**

| Module | Purpose | Files |
|--------|---------|-------|
| `routes` | HTTP endpoint definitions | `routes.rs` |
| `handlers` | Endpoint request handlers | `handlers/` |
| `services` | Business logic & API clients | `services/` |
| `models` | Domain entities & data structures | `models/` |
| `db` | Database queries & SQLx configuration | `db/` |
| `scheduler` | Background task execution | `scheduler/` |
| `error` | Error handling & response types | `error.rs` |

**API Endpoints:**

| Endpoint | Method | Purpose | Status |
|----------|--------|---------|--------|
| `/health` | GET | Backend health check | ✅ Implemented |
| `/api/license-usage` | GET | Fetch aggregated license data | ⏳ Planned |
| `/api/teams` | GET | List teams and members | ⏳ Planned |
| `/api/sync` | POST | Trigger manual data sync | ⏳ Planned |

**Runtime Configuration:**
- Server: Runs on `http://localhost:8080` (development)
- Async Runtime: Tokio with full feature set
- Logging: `tracing` crate for structured logging

### 2.3 Data Layer (PostgreSQL)

**Purpose:** Provide reliable, transactional storage for all operational data.

**Database Schema (MVP):**

```sql
-- Core tables
users              -- User accounts and profiles
teams              -- Organizational teams
products           -- Software products tracked
license_usage      -- Time-series license consumption data
activity_logs      -- Audit trail of system actions
sync_status        -- Track last successful API sync
```

**Key Design Decisions:**
- Timestamps use UTC timezone
- Soft deletes enabled for audit compliance
- Indexes on frequently queried columns (user_id, team_id, date)
- Connection pooling configured for concurrent access

### 2.4 External Services Integration

**Integration Layer:** The backend acts as the sole consumer of external APIs, protecting the frontend from API changes and managing rate limiting.

**Supported Platforms:**

1. **Atlassian Admin API**
   - Authentication: OAuth 2.0
   - Rate Limit: 60 req/min
   - Data: Organization users, groups, activity

2. **GitHub Enterprise API**
   - Authentication: GitHub App or PAT
   - Rate Limit: 5,000 req/hour
   - Data: License consumption, seat usage, organization members

3. **JFrog Artifactory API**
   - Authentication: API Key
   - Rate Limit: 300 req/min
   - Data: Storage usage, user management, artifact counts

4. **Trello API**
   - Authentication: API Key + Token
   - Rate Limit: 180 req/min
   - Data: Board membership, organization structure

---

## Data Flow

### End-to-End Request Flow

```
1. User opens dashboard
   ↓
2. Frontend loads on localhost:5173
   ↓
3. React component mounts, calls GET /api/health
   ↓
4. Vite proxy forwards to http://localhost:8080/health
   ↓
5. Rust backend returns { status: "ok" }
   ↓
6. Frontend displays "Backend connected" status
```

### Data Collection Flow (Scheduled)

```
1. Scheduler wakes up (e.g., midnight daily)
   ↓
2. Iterates through configured API sources
   ↓
3. Calls Atlassian → GitHub → JFrog → Trello APIs
   ↓
4. Collects raw JSON responses
   ↓
5. Transforms data into internal models (validation, normalization)
   ↓
6. Inserts batch into PostgreSQL via SQLx transactions
   ↓
7. Records sync completion timestamp in sync_status table
   ↓
8. Logs summary (records processed, errors, duration)
```

### Database Write Pattern

- **Transactions:** All data insertions wrapped in transactions for atomicity
- **Idempotency:** Duplicate API records identified by unique external ID
- **Error Handling:** Failed syncs logged but don't block subsequent collections

---

## API Integration Layer

### Request Flow Architecture

Each external API call follows this pattern:

```
Frontend Request
   ↓
Rust Backend Handler
   ↓
Service Layer (Authentication + Request Building)
   ↓
External API Call (with retry logic & timeout)
   ↓
Response Parsing & Validation
   ↓
Data Transformation to Internal Models
   ↓
Persistence to PostgreSQL
   ↓
Response to Frontend
```

### API Endpoint Specifications

#### Atlassian Admin API

**Base URL:** `https://api.atlassian.com`

**Endpoints Used:**

| Endpoint | Purpose | Rate Limit Impact |
|----------|---------|------------------|
| `GET /v1/orgs` | List organizations | Low |
| `GET /v1/orgs/{orgId}/users` | Get organization users | High |
| `GET /v2/orgs/{orgId}/directories/-/groups` | List groups | Medium |
| `GET /v2/orgs/{orgId}/directories/{directoryId}/users?groupIds=` | Get users in group | High |

#### GitHub Enterprise API

**Base URL:** `https://api.github.com`

**Endpoints Used:**

| Endpoint | Purpose | Rate Limit Impact |
|----------|---------|------------------|
| `GET /enterprises/{enterprise}/consumed-licenses` | License consumption | Low |
| `GET /enterprises/{enterprise}/copilot/billing/seats` | Copilot seat usage | Low |
| `GET /orgs/{org}/members` | Organization members | Medium |

#### JFrog Artifactory API

**Base URL:** `https://{instance}.jfrog.io`

**Endpoints Used:**

| Endpoint | Purpose | Rate Limit Impact |
|----------|---------|------------------|
| `GET /artifactory/api/security/users` | User inventory | Medium |
| `GET /artifactory/api/storageinfo` | Storage usage statistics | Low |

#### Trello API

**Base URL:** `https://api.trello.com`

**Endpoints Used:**

| Endpoint | Purpose | Rate Limit Impact |
|----------|---------|------------------|
| `GET /1/members/me/boards` | Personal board access | Low |
| `GET /1/organizations/{id}/members` | Organization members | Medium |

---

## Security Considerations

### Authentication & Authorization

**Current State (MVP):**
- No user authentication implemented (local development)
- API keys stored in environment variables
- Vite dev proxy simplifies local CORS handling

**Future State (Production):**
- Implement OAuth 2.0 or SAML for user authentication
- RBAC (Role-Based Access Control) with fine-grained permissions
- API token management with expiration and rotation
- Multi-factor authentication for sensitive operations

### Data Protection

**In Transit:**
- ✅ HTTPS enforced for all external API calls
- ✅ Environment variables for secrets (not hardcoded)
- ⏳ TLS 1.3 minimum version required (production)

**At Rest:**
- ✅ Database encryption via PostgreSQL extensions (future)
- ✅ PII (Personally Identifiable Information) anonymization strategy
- ⏳ Secrets vault integration (HashiCorp Vault)

### Logging & Audit

**Logged Events:**
- ✅ API sync operations with timestamps
- ✅ Errors and exceptions with stack traces
- ✅ User actions for compliance auditing
- ❌ Personal data excluded from logs (strict filtering)

**Log Retention:**
- Development: No retention policy
- Production: 90 days minimum (per compliance)

---

## Deployment Architecture

### Local Development Environment

```
Docker Compose Stack:
├── backend:8080       (Rust Axum server)
├── frontend:5173      (Vite dev server)
└── postgres:5432      (Database)
```

**Start Development:**
```bash
# Terminal 1: Backend
cd backend && cargo run

# Terminal 2: Frontend  
cd frontend && npm start
```

### Production Deployment (Future)

**Containerization Strategy:**
- Backend: Multi-stage Docker build for minimal image size
- Frontend: Static asset generation + CDN distribution
- Database: Managed PostgreSQL service (AWS RDS, Azure Database, etc.)

**Scaling Approach:**
- Horizontal scaling: Multiple backend instances behind load balancer
- Caching: Redis for session management and query results
- Monitoring: Prometheus metrics + Grafana dashboards

---

## Future Extensions

### Q1 2026 Features

| Feature | Description | Impact |
|---------|-------------|--------|
| **RBAC System** | Role-based access control for multi-team environments | Moderate complexity |
| **Advanced Reporting** | Export to CSV/PDF, scheduled email reports | Low complexity |
| **Predictive Analytics** | ML-based forecasting of license needs | High complexity |

### Q2 2026+ Roadmap

- **Power BI Integration** - Direct connection for enterprise reporting
- **Multi-Tenant Support** - Isolate data per organizational unit
- **Webhook Support** - Real-time event notifications to external systems
- **API Rate Limiting** - Client-side quota enforcement
- **Dark Mode** - UI theme support for evening operations teams

### Architecture Evolution

**Monorepo → Microservices Migration:**
- Current: Single backend handling all concerns
- Future: Separate services for data collection, aggregation, reporting
- Trigger: When throughput or latency becomes limiting factor

---

## Related Documentation

- **[ADR-001: Project Setup](./ADRs/ADR-001-project-setup.md)** - Repository structure decisions
- **[Requirements](./requirements/functional-nonfunctional.md)** - Detailed functional specifications
- **[README](../README.md)** - Project setup and quick start guide

---

## Glossary

| Term | Definition |
|------|-----------|
| **RBAC** | Role-Based Access Control - permission system based on user roles |
| **PII** | Personally Identifiable Information - sensitive user data |
| **HMR** | Hot Module Replacement - Vite feature for instant code updates |
| **OAuth 2.0** | Open standard for secure delegated access |
| **SQLx** | Rust SQL toolkit with compile-time query verification |
| **Scheduled Tasks** | Background jobs that run on a timer (cron-like) |

