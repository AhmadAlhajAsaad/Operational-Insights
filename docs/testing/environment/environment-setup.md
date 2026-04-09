# Test Environment Setup

**Status:** Draft  
**Date:** 2026-02-11  
**Author(s):** DP-DevEx-Platform Team

---

## 1. Environment Matrix

| Environment    | Purpose           | Data                       | Refresh   |
| -------------- | ----------------- | -------------------------- | --------- |
| **Local**      | Developer testing | Mock/seed data             | On demand |
| **CI/CD**      | Automated testing | Test containers            | Per build |
| **Staging**    | Integration & UAT | Anonymized production data | Weekly    |
| **Production** | Live system       | Real data                  | N/A       |

---

## 2. Infrastructure Architecture

```mermaid
graph LR
    subgraph Test Environment
        FE[Frontend<br/>React]
        BE[Backend<br/>Rust API]
        DB[(PostgreSQL<br/>Test DB)]
        MOCK[Mock APIs<br/>WireMock]
    end

    FE --> BE
    BE --> DB
    BE --> MOCK
```

---

## 3. Local Development Setup

### Prerequisites

| Tool       | Version | Purpose                 |
| ---------- | ------- | ----------------------- |
| Docker     | 20+     | Container runtime       |
| Rust       | Stable  | Backend development     |
| Node.js    | LTS     | Frontend development    |
| PostgreSQL | 14+     | Database (or container) |

### Quick Start

```bash
# Clone repository
git clone https://github.com/SLS-DP-DevOps-Forge/Equans-operational-insights.git
cd Equans-operational-insights

# Start test environment with Docker Compose
docker-compose -f docker-compose.test.yml up -d

# Run database migrations
cargo run --bin migrate

# Seed test data
cargo run --bin seed-test-data

# Execute test suite
cargo test --workspace
```

### Docker Compose Configuration

```yaml
# docker-compose.test.yml
version: "3.8"

services:
  postgres-test:
    image: postgres:14
    environment:
      POSTGRES_USER: test_user
      POSTGRES_PASSWORD: test_password
      POSTGRES_DB: equans_test
    ports:
      - "5433:5432"
    volumes:
      - postgres_test_data:/var/lib/postgresql/data

  wiremock:
    image: wiremock/wiremock:2.35.0
    ports:
      - "8089:8080"
    volumes:
      - ./tests/mocks:/home/wiremock

volumes:
  postgres_test_data:
```

---

## 4. Test Data Management

### Data Sources

| Data Type     | Source               | Handling               |
| ------------- | -------------------- | ---------------------- |
| User data     | Anonymized export    | GDPR-compliant masking |
| License data  | Synthetic generation | Realistic patterns     |
| API responses | Recorded fixtures    | Version controlled     |
| Edge cases    | Manual creation      | Documented scenarios   |

### Seed Data Script

```bash
# Generate seed data for testing
cargo run --bin seed-test-data -- \
  --users 100 \
  --teams 10 \
  --licenses 500

# Reset database to clean state
cargo run --bin reset-test-db
```

### Test Fixtures

```
tests/
├── fixtures/
│   ├── users.json           # Sample user data
│   ├── licenses.json        # Sample license data
│   ├── atlassian/
│   │   ├── users.json       # Mock Atlassian user response
│   │   └── licenses.json    # Mock Atlassian license response
│   └── github/
│       ├── seats.json       # Mock GitHub seats response
│       └── copilot.json     # Mock Copilot usage response
└── mocks/
    └── mappings/            # WireMock stub mappings
```

---

## 5. Mock API Configuration

### WireMock Stubs

```json
// tests/mocks/mappings/atlassian-users.json
{
  "request": {
    "method": "GET",
    "urlPattern": "/atlassian/users.*"
  },
  "response": {
    "status": 200,
    "headers": {
      "Content-Type": "application/json"
    },
    "bodyFileName": "atlassian/users.json"
  }
}
```

### Environment Variables

```bash
# .env.test
DATABASE_URL=postgres://test_user:test_password@localhost:5433/equans_test
ATLASSIAN_API_URL=http://localhost:8089/atlassian
GITHUB_API_URL=http://localhost:8089/github
JFROG_API_URL=http://localhost:8089/jfrog
JWT_SECRET=test-secret-key-for-testing-only
LOG_LEVEL=debug
```

> ⚠️ **Security:** Never use test credentials in production. Use `.env.example` for templates.

---

## 6. CI/CD Environment

### GitHub Actions Configuration

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    services:
      postgres:
        image: postgres:14
        env:
          POSTGRES_USER: test_user
          POSTGRES_PASSWORD: test_password
          POSTGRES_DB: equans_test
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
      - uses: actions/checkout@v4

      - name: Setup Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Run migrations
        run: cargo run --bin migrate
        env:
          DATABASE_URL: postgres://test_user:test_password@localhost:5432/equans_test

      - name: Run tests
        run: cargo test --workspace
        env:
          DATABASE_URL: postgres://test_user:test_password@localhost:5432/equans_test
```

---

## 7. Staging Environment

### Access

| Component | URL                                     | Authentication |
| --------- | --------------------------------------- | -------------- |
| Frontend  | https://staging.equans-insights.com     | SSO            |
| API       | https://api-staging.equans-insights.com | JWT            |
| Database  | Internal only                           | Credentials    |

### Data Refresh Schedule

- **Weekly:** Full anonymized production data sync
- **Daily:** Incremental updates
- **On-demand:** Manual refresh for specific testing

### Data Anonymization Rules

| Field Type | Anonymization Method      |
| ---------- | ------------------------- |
| Email      | `{hash}@anonymized.local` |
| Name       | Faker-generated names     |
| IP Address | Random private IP         |
| User ID    | UUID replacement          |

---

## 8. Troubleshooting Environment Issues

| Issue                    | Cause                 | Resolution                           |
| ------------------------ | --------------------- | ------------------------------------ |
| Database connection fail | Container not running | `docker-compose up -d postgres-test` |
| Port already in use      | Conflicting service   | Change port in docker-compose        |
| Mock API not responding  | WireMock not started  | `docker-compose up -d wiremock`      |
| Stale test data          | Cache not cleared     | `cargo run --bin reset-test-db`      |

---

## Related Documents

- [Test Strategy](../strategy/test-strategy.md)
- [Performance Testing](../performance/performance-testing.md)
- [Troubleshooting](../troubleshooting/troubleshooting-guide.md)
