# Quick Start Guide - Backend API Development

## What We've Built

A complete Rust backend with Atlassian and GitHub API integrations as specified in `/docs/api/api-endpoints-license-userdata.md`.

---

## Project Structure

```
backend/
├── src/
│   ├── main.rs
│   ├── github.rs     (combines routes, services, models)
│   ├── atlassian.rs  (combines routes, services, models)
│   └── health.rs     (health check handlers)
├── Cargo.toml              # Dependencies (updated with reqwest, chrono)
├── tests/                  # Integration tests for API endpoints
│   ├── run_all_tests_fixed.ps1 
│   ├── run_all_tests.ps1
│   ├── test_atlassian_endpoints.ps1
│   └── test_github_endpoints.ps1
├── .env.example                   # Template for configuration
└── README.md                      # Full API documentation
```

---

## 🚀 How to Run

### 1. Configure API Tokens

```bash
cd backend
cp .env.example .env
```

Edit `.env` and add your tokens:
```env
ATLASSIAN_API_TOKEN=your_token_here
GITHUB_PAT_TOKEN=your_token_here
```

### 2. Start the Backend

```bash
cargo run
```

Server starts at: `http://localhost:8080`

---

## 🔌 Available API Endpoints

### Atlassian Cloud

| Endpoint | Description | Doc Reference |
|----------|-------------|---------------|
| `GET /api/atlassian/organizations` | List all organizations | A1 |
| `GET /api/atlassian/organizations/:org_id/users` | List all users | A2 |
| `GET /api/atlassian/organizations/:org_id/groups` | List groups | A3 |
| `GET /api/atlassian/organizations/:org_id/licenses/:product` | Calculate license count | A4 |

### GitHub Enterprise

| Endpoint | Description | Doc Reference |
|----------|-------------|---------------|
| `GET /api/github/validate` | Validate PAT token | G1 |
| `GET /api/github/enterprises/:enterprise/licenses` | License consumption | G2 |
| `GET /api/github/enterprises/:enterprise/copilot` | Copilot seats | G3 |
| `GET /api/github/enterprises/:enterprise/ghas` | GHAS usage | G4 |

---

## 🧪 Quick Test

### Test Health Check
```bash
curl http://localhost:8080/health
```

### Test Atlassian Organizations
```bash
curl http://localhost:8080/api/atlassian/organizations
```

### Test GitHub Token
```bash
curl http://localhost:8080/api/github/validate
```

### Test Atlassian Users
```bash
curl http://localhost:8080/api/atlassian/organizations/21959ca7-236b-11j7-k470-6dc106467633/users
```

---

## 📊 Example Usage Flow

### Calculate Jira Software Licenses

**Step 1:** Get your organization ID
```bash
curl http://localhost:8080/api/atlassian/organizations
# Returns: [{ "id": "21959ca7-...", "name": "myequans" }]
```

**Step 2:** Get license count
```bash
curl http://localhost:8080/api/atlassian/organizations/21959ca7-236b-11j7-k470-6dc106467633/licenses/jira-software
# Returns: { "product": "jira-software", "total_users": 150, "active_users": 145 }
```

### Get GitHub Enterprise Licenses

```bash
curl http://localhost:8080/api/github/enterprises/equans/licenses
# Returns: { "total_seats_consumed": 425, "total_seats_purchased": 500, ... }
```

---

## 🛠️ Key Implementation Details

### Service Layer Pattern

Each service (`AtlassianClient`, `GitHubClient`) handles:
- API authentication (Bearer tokens)
- HTTP requests with proper headers
- Error handling and logging
- Response parsing to internal models

### License Counting Logic (Atlassian)

The `calculate_license_count` function:
1. Fetches all groups for the organization
2. Filters groups matching the product name (e.g., "jira-software")
3. Retrieves users in those groups
4. Counts active users (`account_status == "active"` AND `membership_status == "active"`)

### Error Handling

All endpoints return:
- `200 OK` on success with JSON data
- `401/403` on authentication errors
- `500` on server/external API errors
- Structured error messages: `{ "error": "description" }`

---