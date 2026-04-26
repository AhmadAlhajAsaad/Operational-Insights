# Backend API Endpoints - Implementation Guide

## Overview

This backend implements the API endpoints documented in `/docs/api/api-endpoints-license-userdata.md`.

**Base URL:** `http://localhost:8080`

---

## Configuration

### Environment Variables

Create a `.env` file in the `backend/` directory:

```bash
cp .env.example .env
```

Then configure your API tokens:

```env
ATLASSIAN_API_TOKEN=your_atlassian_admin_api_token
GITHUB_PAT_TOKEN=your_github_personal_access_token
```

### Getting API Tokens

**Atlassian Admin API Token:**
1. Navigate to `https://myequans.atlassian.net/admin`
2. Go to Settings → Security → API tokens
3. Create token with "Read Users" and "Read Groups" scopes
4. Copy token to `.env` file

**GitHub Personal Access Token:**
1. Navigate to `https://github.com/settings/tokens/new`
2. Select scopes: `admin:enterprise`, `read:org`
3. Set expiration: 90 days recommended
4. Copy token to `.env` file

---

## Running the Backend

```bash
cd backend
cargo run
```

The server will start on `http://localhost:8080`

---

## Available Endpoints

### Health Check

**GET** `/health`

Check if the backend is running.

**Response:**
```json
{
  "status": "ok",
  "service": "equans-operational-insights-backend"
}
```

---

## Atlassian Cloud API Endpoints

### 1. Get Organizations

**GET** `/api/atlassian/organizations`

Retrieve all Atlassian organizations (implements A1 from docs).

**Example:**
```bash
curl http://localhost:8080/api/atlassian/organizations
```

**Response:**
```json
[
  {
    "id": "21959ca7-236b-11j7-k470-6dc106467633",
    "name": "myequans",
    "display_name": "Equans Cloud Organization"
  }
]
```

---

### 2. Get Groups

**GET** `/api/atlassian/organizations/:org_id/groups`

Retrieve all groups for an organization (implements A3 from docs).

**Parameters:**
- `org_id` (path): Organization ID from endpoint #1

**Example:**
```bash
curl http://localhost:8080/api/atlassian/organizations/21959ca7-236b-11j7-k470-6dc106467633/groups
```

**Response:**
```json
[
  {
    "id": "1a33bac2-fd80-4a92-8e21-8e018ae83ead",
    "name": "AARSEN-admins",
    "member_count": 5
  },
  {
    "id": "01804d5a-f720-4b24-964e-2bd37303a02d",
    "name": "AARSEN-users",
    "member_count": 120
  },
  {
    "id": "d3b07384-d9a5-11e9-98d0-037ffccadce9",
    "name": "jira-software-users",
    "member_count": 150,
    "directory_id": "c3b07384-d9a5-11e9-98d0-037ffccadce9"
  }
]
```

**Note:** This endpoint attempts to fetch member counts by making an additional API call for each group to `/admin/v2/orgs/{orgId}/directories/-/users?group_ids={groupId}`. 

**Important**: The `member_count` field will only be populated if your API token has **"Manage Groups"** or **"Manage Users"** permissions in Atlassian. If your token only has "Read Users" and "Read Groups" permissions, the `member_count` will be `null` for all groups. This is a limitation of the Atlassian Admin API permissions model.

To enable member counts:
1. Go to `https://myequans.atlassian.net/admin`
2. Navigate to Settings → API tokens  
3. Update your API token permissions to include "Manage Groups"
4. Update the token in your `.env` file

---

### 3. Get Users

**GET** `/api/atlassian/organizations/:org_id/users`

Retrieve all users in an Atlassian organization with pagination support (implements A2 from docs).

**Parameters:**
- `org_id` (path): Organization ID from endpoint #1

**Example:**
```bash
curl http://localhost:8080/api/atlassian/organizations/21959ca7-236b-11j7-k470-6dc106467633/users
```

**Response:**
```json
[
  {
    "account_id": "557058:09bea08c-fbb8-48a9-be32-a73b2c4fe857",
    "name": "Ben Veuchelen",
    "email": "maht77@equans.com",
    "account_status": "active",
    "product_access": [
      {
        "name": "Jira Service Management",
        "key": "jira-service-management"
      }
    ]
  },
  {
    "account_id": "557058:1368c49b-068b-45a8-9d9f-bb94661d0175",
    "name": "Stephane LOBJOIS",
    "email": "ich168@equans.com",
    "account_status": "active",
    "product_access": [
      {
        "name": "Confluence",
        "key": "confluence"
      },
      {
        "name": "Jira Software",
        "key": "jira-software"
      }
    ]
  }
]
```

**Note:** This endpoint automatically handles pagination and fetches all users across multiple API pages. For organizations with thousands of users, this may take 30-60 seconds. The response includes the `product_access` field which shows which Atlassian products each user has access to.

---

### 4. Get License Count

**GET** `/api/atlassian/organizations/:org_id/licenses/:product`

Calculate license consumption for a specific product (implements A4 logic from docs).

**Parameters:**
- `org_id` (path): Organization ID
- `product` (path): Product name (e.g., `jira-software`, `confluence`, `trello`, `jira-service-management`)

**Example:**
```bash
curl http://localhost:8080/api/atlassian/organizations/21959ca7-236b-11j7-k470-6dc106467633/licenses/jira-software
```

**Response:**
```json
{
  "product": "jira-software",
  "total_users": 1689,
  "active_users": 1689
}
```

**Logic:**
1. Fetches all users with their product_access data
2. Filters users who have access to the specified product
3. Counts total users and active users (where `account_status == "active"`)

---

### 5. Get License Count with User Details

**GET** `/api/atlassian/organizations/:org_id/licenses/:product/details`

Get detailed license information including the names, emails, and IDs of all users with access to a product.

**Parameters:**
- `org_id` (path): Organization ID
- `product` (path): Product name (e.g., `jira-software`, `confluence`, `trello`, `jira-service-management`)

**Example:**
```bash
curl http://localhost:8080/api/atlassian/organizations/21959ca7-236b-11j7-k470-6dc106467633/licenses/jira-software/details
```

**Response:**
```json
{
  "product": "jira-software",
  "total_users_count": 1689,
  "active_users_count": 1689,
  "total_users": [
    {
      "account_id": "557058:1368c49b-068b-45a8-9d9f-bb94661d0175",
      "name": "Stephane LOBJOIS",
      "email": "ich168@equans.com",
      "account_status": "active"
    },
    {
      "account_id": "557058:c95f3efa-eb3b-4e6e-b0cd-4c0c3e7e4831",
      "name": "Hedi AJAM",
      "email": "lx1253@equans.com",
      "account_status": "active"
    }
  ],
  "active_users": [
    {
      "account_id": "557058:1368c49b-068b-45a8-9d9f-bb94661d0175",
      "name": "Stephane LOBJOIS",
      "email": "ich168@equans.com",
      "account_status": "active"
    }
  ]
}
```

**Fields:**
- `total_users_count`: Number of users with product access
- `active_users_count`: Number of active users with product access
- `total_users`: Array of all users (active and inactive) with full details
- `active_users`: Array of only active users with full details

**Note:** This endpoint returns the complete user information for each license. For products with many users, the response may be large. Use the regular `/licenses/:product` endpoint if you only need the counts.
```

**Logic:**
1. Fetches all groups for the organization
2. Filters groups matching the product name (e.g., "jira-software-users")
3. Retrieves all users in matching groups
4. Counts users where `account_status == "active"` AND `membership_status == "active"`

---

## GitHub Enterprise API Endpoints

### 1. Validate Token

**GET** `/api/github/validate`

Validate GitHub PAT token (implements G1 from docs).

**Example:**
```bash
curl http://localhost:8080/api/github/validate
```

**Response:**
```json
{
  "login": "api-user",
  "id": 12345678,
  "email": "api-user@equans.com",
  "name": "API Service Account"
}
```

---

### 2. Get License Consumption

**GET** `/api/github/enterprises/:enterprise/licenses`

Get GitHub Enterprise license consumption (implements G2 from docs).

**Parameters:**
- `enterprise` (path): Enterprise name (e.g., `equans`)

**Example:**
```bash
curl http://localhost:8080/api/github/enterprises/equans/licenses
```

**Response:**
```json
{
  "total_seats_consumed": 481,
  "total_seats_purchased": 500,
  "seats_available": 19,
  "user_count": 30
}
```

**Note:** `user_count` indicates the number of users included in the GitHub API response page (typically 30), not the total consumed seats.

---

### 3. Get Copilot Seats

**GET** `/api/github/enterprises/:enterprise/copilot`

Get GitHub Copilot seat usage (implements G3 from docs).

**Parameters:**
- `enterprise` (path): Enterprise name (e.g., `equans`)

**Example:**
```bash
curl http://localhost:8080/api/github/enterprises/equans/copilot
```

**Response:**
```json
{
  "total_seats": 129,
  "total_seats_used": null,
  "seats": [
    {
      "login": null,
      "id": null,
      "email": null,
      "plan_type": "business",
      "state": null,
      "last_activity_at": "2025-12-09T14:53:25Z"
    }
  ]
}
```

**Note:** The GitHub Copilot Billing API does not return detailed user information (login, id, email, state) or `total_seats_used` for enterprise accounts. These fields will be `null` in the response. Only `total_seats`, `plan_type`, and `last_activity_at` are available. This is a limitation of the GitHub Enterprise API.

---

### 4. Get GHAS Usage

**GET** `/api/github/enterprises/:enterprise/ghas`

Get GitHub Advanced Security usage (implements G4 from docs).

**Parameters:**
- `enterprise` (path): Enterprise name (e.g., `equans`)

**Example:**
```bash
curl http://localhost:8080/api/github/enterprises/equans/ghas
```

**Response:**
```json
{
  "total_committers": null,
  "total_committers_used": null,
  "repositories": [
    {
      "name": "Equans-Indicon/job-sorter",
      "advanced_security_committers": 1
    },
    {
      "name": "EquansCorporate/FinancialCockpitCore",
      "advanced_security_committers": 3
    }
  ]
}
```

**Note:** The GitHub Enterprise Advanced Security Billing API may not return `total_committers` or `total_committers_used` fields for all enterprise configurations. These fields will be `null` if not provided by the API. The `repositories` array contains all repositories with Advanced Security enabled and their individual committer counts.
```

---

## Error Handling

All endpoints return appropriate HTTP status codes:

- `200 OK`: Success
- `401 Unauthorized`: Invalid or expired API token
- `500 Internal Server Error`: API token not configured or external API error

**Error Response Format:**
```json
{
  "error": "Error message description"
}
```

---

## Architecture

### Project Structure

```
backend/src/
├── main.rs
├── atlassian.rs  (combines routes, services, models)
├── github.rs     (combines routes, services, models)
└── health.rs     (health check handlers)
```

### Data Flow

```
HTTP Request
    ↓
Route Handler (atlassian.rs or github.rs)
    ↓
Service Client (services/atlassian.rs or services/github.rs)
    ↓
External API Call (Atlassian/GitHub)
    ↓
Response Parsing (models/atlassian.rs or models/github.rs)
    ↓
JSON Response to Frontend
```

---

## Testing

### Manual Testing with curl

**Test Atlassian Organizations:**
```bash
curl -v http://localhost:8080/api/atlassian/organizations
```

**Test GitHub Token Validation:**
```bash
curl -v http://localhost:8080/api/github/validate
```

**Test License Count:**
```bash
# Replace with your actual org_id
curl http://localhost:8080/api/atlassian/organizations/YOUR_ORG_ID/licenses/jira-software
```

---


## Troubleshooting

### Error: "ATLASSIAN_API_TOKEN not configured"

**Solution:** Create a `.env` file with your API token:
```bash
cp .env.example .env
# Edit .env and add your token
```

### Error: "API request failed with status 401"

**Solution:** Your API token is invalid or expired. Generate a new token and update `.env`.

### Error: "API request failed with status 403"

**Solution:** Your token doesn't have sufficient permissions. Verify scopes:
- Atlassian: "Read Users", "Read Groups"
- GitHub: "admin:enterprise", "read:org"

