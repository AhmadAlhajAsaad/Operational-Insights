# API Endpoints for License & User Data

## Atlassian Cloud & GitHub Enterprise

| Property | Details |
|----------|---------|
| **Author** | Ahmad Alhaj Asaad |
| **Last Updated** | 2025-12-05 |
| **Status** | Active Development |
| **Scope** | Atlassian Cloud (myequans) + GitHub Enterprise Cloud (equans) |

---

## Table of Contents

1. [Introduction](#introduction)
2. [Atlassian Cloud APIs](#atlassian-cloud-apis)
3. [GitHub Enterprise APIs](#github-enterprise-apis)
4. [Use Case Mapping](#use-case-mapping)
5. [Authentication & Configuration](#authentication--configuration)
6. [References](#references)

---

## Introduction

This document describes the REST API endpoints used to retrieve operational insights data:

**Data Categories:**

- License consumption per product (Jira, Confluence, JSM, GitHub Enterprise, Copilot, GHAS)
- User information and product access
- Activity and engagement metrics
- Chargeback and cost allocation data

**Primary Consumers:**

- Dashboard visualizations
- Reporting and analytics
- Chargeback calculations
- Team-level cost attribution

**Key Scope:**

| Organization | Products | API Provider |
|--------------|----------|--------------|
| **myequans** | Jira Software, Confluence, JSM, Atlas, Trello | Atlassian Cloud Admin API |
| **equans** | GitHub Enterprise, Copilot Business/Enterprise, GHAS | GitHub Enterprise Cloud REST API |

---

## Atlassian Cloud APIs

### Atlassian License Model

Atlassian license counting follows this hierarchy:

```
Organization (orgId)
  ↓
Groups (product-specific: jira-software-users, confluence-users, etc.)
  ↓
Users (with active/inactive status)
  ↓
License Count (active users in product groups)
```

**License Determination:**
A user consumes a license if:

1. They are a member of the product group (e.g., `jira-software-users`)
2. Their account status is `active`
3. Their membership status is `active`

---

### A1: Get Organizations

**Purpose:** Retrieve organization metadata and determine `orgId` for subsequent calls.

**Endpoint:**

```
GET https://api.atlassian.com/admin/v1/orgs
```

**Authentication:** Bearer Token (Admin API Key)

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `limit` | integer | No | Items per page (default: 50, max: 100) |
| `cursor` | string | No | Pagination cursor |

**Response Fields:**

```json
{
  "data": [
    {
      "id": "21959ca7-236b-11e7-k470-6dc106467633",
      "name": "myequans",
      "display_name": "Equans Cloud Organization",
      "url": "https://myequans.atlassian.net",
      "domains": ["myequans.atlassian.net"],
      "links": {
        "self": "https://api.atlassian.com/admin/v1/orgs/..."
      }
    }
  ]
}
```

**Used For:** Determining the `orgId` for all subsequent Atlassian API calls.

**Example Usage:**

```bash
curl -X GET "https://api.atlassian.com/admin/v1/orgs" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Accept: application/json"
```

---

### A2: Get Managed Accounts (Users)

**Purpose:** Retrieve user directory with product access information.

**Endpoint:**

```
GET https://api.atlassian.com/admin/v1/orgs/{orgId}/users
```

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `orgId` | string | Yes | Organization ID from A1 |

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `limit` | integer | No | Items per page (default: 50, max: 100) |
| `cursor` | string | No | Pagination cursor for large datasets |

**Response Fields:**

```json
{
  "data": [
    {
      "account_id": "557058:12345678-1234-1234-1234-123456789012",
      "name": "John Smith",
      "email": "john.smith@equans.com",
      "account_status": "active",
      "member_since": "2023-01-15T10:30:00Z",
      "product_access": [
        {
          "product_name": "jira-software",
          "access_type": "licensed"
        },
        {
          "product_name": "confluence",
          "access_type": "licensed"
        }
      ]
    }
  ]
}
```

**Used For:**

- Activity insights and user engagement
- Supporting datasets (not primary license counting - use A4 instead)
- User deprovisioning tracking

**Example Usage:**

```bash
curl -X GET "https://api.atlassian.com/admin/v1/orgs/21959ca7-236b-11e7-k470-6dc106467633/users?limit=100" \
  -H "Authorization: Bearer YOUR_API_KEY"
```

---

### A3: Get Groups in Organization

**Purpose:** Identify product-specific groups that determine license membership.

**Endpoint:**

```
GET https://api.atlassian.com/admin/v2/orgs/{orgId}/directories/-/groups
```

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `orgId` | string | Yes | Organization ID from A1 |

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `limit` | integer | No | Items per page (default: 50, max: 100) |
| `cursor` | string | No | Pagination cursor |

**Response Fields:**

```json
{
  "data": [
    {
      "id": "d3b07384-d9a5-11e9-98d0-037ffccadce9",
      "name": "jira-software-users",
      "member_count": 150,
      "directory_id": "c3b07384-d9a5-11e9-98d0-037ffccadce9",
      "external_synced": false,
      "links": {
        "self": "https://api.atlassian.com/..."
      }
    },
    {
      "id": "e4c18495-e9a5-11e9-98d0-037ffccadce9",
      "name": "confluence-users",
      "member_count": 120,
      "directory_id": "c3b07384-d9a5-11e9-98d0-037ffccadce9",
      "external_synced": false
    }
  ]
}
```

**Group Naming Convention:**

- `{product}-users` (e.g., `jira-software-users`, `confluence-users`)
- `team-{name}` (e.g., `team-platform`, `team-security`)

**Used For:**

- Identifying product groups for license counting
- Mapping users to products
- Understanding organizational structure

**Example Usage:**

```bash
curl -X GET "https://api.atlassian.com/admin/v2/orgs/21959ca7-236b-11e7-k470-6dc106467633/directories/-/groups" \
  -H "Authorization: Bearer YOUR_API_KEY"
```

---

### A4: Get Users in Group(s)

**Purpose:** Final license count - retrieve users in product-specific groups.

**Endpoint:**

```
GET https://api.atlassian.com/admin/v2/orgs/{orgId}/directories/-/users
```

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `orgId` | string | Yes | Organization ID from A1 |

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `group_ids` | string | Yes | Comma-separated group IDs from A3 |
| `limit` | integer | No | Items per page (default: 50, max: 100) |
| `cursor` | string | No | Pagination cursor |

**Response Fields:**

```json
{
  "data": [
    {
      "account_id": "557058:12345678-1234-1234-1234-123456789012",
      "name": "Alice Johnson",
      "email": "alice.johnson@equans.com",
      "status": "active",
      "membership_status": "active",
      "last_active": "2025-12-05T14:22:00Z",
      "groups": [
        {
          "group_id": "d3b07384-d9a5-11e9-98d0-037ffccadce9",
          "group_name": "jira-software-users"
        }
      ]
    }
  ]
}
```

**License Counting Logic:**

- Include users where `status == "active"` AND `membership_status == "active"`
- One user per product group = one license
- Same user in multiple groups = multiple licenses

**Used For:** ⭐ **Primary license consumption counting** ⭐

**Example Usage:**

```bash
curl -X GET "https://api.atlassian.com/admin/v2/orgs/21959ca7-236b-11e7-k470-6dc106467633/directories/-/users?group_ids=d3b07384-d9a5-11e9-98d0-037ffccadce9,e4c18495-e9a5-11e9-98d0-037ffccadce9" \
  -H "Authorization: Bearer YOUR_API_KEY"
```

---

### Atlassian Data Flow Example

**Scenario:** Count active Jira Software licenses

```
1. Call A1 → Get orgId = "21959ca7-236b-11e7-k470-6dc106467633"

2. Call A3 → List groups → Find "jira-software-users"
   Group ID: "d3b07384-d9a5-11e9-98d0-037ffccadce9"

3. Call A4 → Get users in jira-software-users group
   Result: 150 active users

4. Dashboard Display:
   ✓ Jira Software: 150 licenses consumed / 200 purchased
```

---

## GitHub Enterprise APIs

### GitHub License Model

GitHub license tracking uses enterprise-level endpoints with different authentication scopes per endpoint.

```
Enterprise (equans)
  ↓
Products:
  - GitHub Enterprise (seat licensing)
  - Copilot Business/Enterprise
  - Advanced Security (GHAS)
  ↓
License Metrics (seats consumed, features active)
```

**Authentication:** Personal Access Token (PAT) with `admin:enterprise` scope

---

### G1: Validate GitHub Token

**Purpose:** Verify PAT authentication and identify token owner.

**Endpoint:**

```
GET https://api.github.com/user
```

**Authentication:** Bearer Token (GitHub PAT)

**Response Fields:**

```json
{
  "login": "api-user",
  "id": 12345678,
  "email": "api-user@equans.com",
  "name": "API Service Account",
  "type": "User",
  "site_admin": true
}
```

**Used For:**

- Verifying PAT is valid and unexpired
- Confirming service account permissions
- Initial connection validation

**Example Usage:**

```bash
curl -X GET "https://api.github.com/user" \
  -H "Authorization: Bearer YOUR_PAT"
```

---

### G2: Enterprise License Consumption

**Purpose:** Retrieve overall GitHub Enterprise seat licensing metrics.

**Endpoint:**

```
GET https://api.github.com/enterprises/{enterprise}/consumed-licenses
```

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `enterprise` | string | Yes | Enterprise name (e.g., "equans") |

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `per_page` | integer | No | Items per page (default: 30, max: 100) |
| `page` | integer | No | Page number for pagination |

**Response Fields:**

```json
{
  "total_seats_consumed": 425,
  "total_seats_purchased": 500,
  "seats_available": 75,
  "users": [
    {
      "login": "john.smith",
      "id": 98765432,
      "email": "john.smith@equans.com",
      "name": "John Smith",
      "avatar_url": "https://avatars.githubusercontent.com/u/98765432",
      "profile_url": "https://github.com/john.smith",
      "created_at": "2023-01-15T08:00:00Z",
      "last_login": "2025-12-05T14:20:00Z"
    }
  ]
}
```

**Key Metrics:**

| Metric | Meaning | Use Case |
|--------|---------|----------|
| `total_seats_consumed` | Active GitHub Enterprise seats | License cost calculation |
| `total_seats_purchased` | License allocation | Budget vs. actual reporting |
| `seats_available` | Unused seats | Capacity planning |

**Used For:** ⭐ **Primary GitHub Enterprise license metrics** ⭐

**Example Usage:**

```bash
curl -X GET "https://api.github.com/enterprises/equans/consumed-licenses" \
  -H "Authorization: Bearer YOUR_PAT"
```

---

### G3: Copilot Seat Usage & Activity

**Purpose:** Track Copilot Business/Enterprise adoption and team activity.

**Endpoint:**

```
GET https://api.github.com/enterprises/{enterprise}/copilot/billing/seats
```

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `enterprise` | string | Yes | Enterprise name (e.g., "equans") |

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `per_page` | integer | No | Items per page (default: 50, max: 100) |
| `page` | integer | No | Page number |

**Response Fields:**

```json
{
  "total_seats": 150,
  "total_seats_used": 145,
  "seats": [
    {
      "login": "alice.johnson",
      "id": 54321098,
      "email": "alice.johnson@equans.com",
      "plan_type": "enterprise",
      "state": "active",
      "last_activity_at": "2025-12-05T13:45:00Z",
      "created_at": "2024-03-20T10:15:00Z",
      "last_activity": "pull_request_review"
    }
  ]
}
```

**Activity States:**

| State | Meaning |
|-------|---------|
| `active` | License currently used and actively generating activity |
| `pending_invitation` | User invited but hasn't accepted |
| `pending_cancellation` | License scheduled for removal |
| `inactive` | Assigned but no recent activity |

**Used For:**

- Copilot adoption tracking
- Team productivity insights
- Cost per active user analysis
- Idle license identification

**Example Usage:**

```bash
curl -X GET "https://api.github.com/enterprises/equans/copilot/billing/seats" \
  -H "Authorization: Bearer YOUR_PAT"
```

---

### G4: Advanced Security (GHAS) Usage

**Purpose:** Track GitHub Advanced Security consumption and repository-level security metrics.

**Endpoint:**

```
GET https://api.github.com/enterprises/{enterprise}/settings/billing/advanced-security
```

**Path Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `enterprise` | string | Yes | Enterprise name (e.g., "equans") |

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `per_page` | integer | No | Items per page (default: 100, max: 500) |
| `page` | integer | No | Page number |

**Response Fields:**

```json
{
  "total_committers": 320,
  "total_committers_used": 295,
  "repositories": [
    {
      "name": "equans-operational-insights",
      "url": "https://github.com/SLS-DP-DevOps-Forge/equans-operational-insights",
      "advanced_security_committers": 45,
      "advanced_security_committers_breakdown": {
        "pull_requests": 35,
        "issues": 40,
        "commits": 42
      }
    }
  ]
}
```

**Committer Counting:**

- Counted by unique committer across all activity types
- Aggregated monthly for billing purposes
- Used for cost attribution per team/repository

**Used For:**

- Security compliance reporting
- Repository-level cost allocation
- Team security investment tracking

**Example Usage:**

```bash
curl -X GET "https://api.github.com/enterprises/equans/settings/billing/advanced-security" \
  -H "Authorization: Bearer YOUR_PAT"
```

---

## Use Case Mapping

### Dashboard Requirements & API Coverage

| Use Case | Requirement | Atlassian APIs | GitHub APIs | Data Freshness |
|----------|-------------|----------------|-------------|-----------------|
| **License Dashboard** | Total seats consumed per product | A1, A3, A4 | G2 | Daily |
| **Team-Level Costs** | Breakdown by business unit | A4 (group mapping) | G3 (org mapping) | Daily |
| **Active vs Inactive Users** | User engagement metrics | A2, A4 | G2 (last_login) | Daily |
| **Copilot Adoption** | Who uses Copilot + activity | - | G3 | Daily |
| **Security Metrics** | GHAS committer tracking | - | G4 | Monthly |
| **Chargeback Calculations** | Cost per team/person | A1-A4 | G2-G4 | Daily |
| **Capacity Planning** | Headroom analysis | A1, A3 | G2 | Weekly |

### Data Collection Sequence

```
Daily Sync (Scheduled 02:00 UTC):

1. Atlassian Flow:
   - Call A1 → Fetch orgId
   - Call A3 → Fetch all groups
   - Call A4 → Fetch users per group (paginated)
   - Store license counts by product
   - Aggregate by team/BU

2. GitHub Flow:
   - Call G1 → Validate token
   - Call G2 → Fetch enterprise licenses
   - Call G3 → Fetch Copilot seats
   - Call G4 → Fetch GHAS metrics
   - Store in time-series database

3. Transform & Persist:
   - Deduplicate user records
   - Calculate derived metrics (% utilization, trend)
   - Insert into PostgreSQL
   - Update dashboard cache
```

---

## Authentication & Configuration

### Atlassian Admin API

**Prerequisites:**

1. Atlassian organization administrator role
2. API key generated in Admin Settings

**Getting Your API Key:**

```
1. Navigate to: https://myequans.atlassian.net/admin
2. Settings → Security → API tokens
3. Create token with "Read Users" and "Read Groups" scopes
4. Copy token and store securely
```

**Bearer Token Format:**

```
Authorization: Bearer YOUR_API_TOKEN
```

**Request Headers:**

```
GET /admin/v1/orgs HTTP/1.1
Host: api.atlassian.com
Authorization: Bearer YOUR_API_TOKEN
Accept: application/json
Content-Type: application/json
```

---

### GitHub Enterprise API

**Prerequisites:**

1. GitHub organization owner or enterprise admin role
2. Personal Access Token (PAT) with appropriate scopes

**Creating Your PAT:**

```
1. Navigate to: https://github.com/settings/tokens/new
2. Select scopes:
   - admin:enterprise (for enterprise endpoints)
   - read:org (for org member endpoints)
3. Set expiration: 90 days recommended
4. Copy token and store in secrets manager
```

**Bearer Token Format:**

```
Authorization: Bearer YOUR_PAT_TOKEN
```

**Request Headers:**

```
GET /enterprises/equans/consumed-licenses HTTP/1.1
Host: api.github.com
Authorization: Bearer YOUR_PAT_TOKEN
Accept: application/json
```

---

### Postman Configuration Example

**Atlassian Collection:**

```json
{
  "variable": [
    {
      "key": "atlassian_api_key",
      "value": "YOUR_API_KEY",
      "type": "string"
    },
    {
      "key": "org_id",
      "value": "21959ca7-236b-11e7-k470-6dc106467633",
      "type": "string"
    }
  ]
}
```

**GitHub Collection:**

```json
{
  "variable": [
    {
      "key": "github_pat",
      "value": "YOUR_PAT_TOKEN",
      "type": "string"
    },
    {
      "key": "enterprise",
      "value": "equans",
      "type": "string"
    }
  ]
}
```

---

## References

### Official Documentation

| API | Documentation Link | Key Sections |
|-----|------------------|--------------|
| **Atlassian Admin API** | [Admin API Users](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-users/) | Users, Groups, Directory |
| **Atlassian Admin API v2** | [Admin API Groups](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-groups/) | Group Management |
| **GitHub Enterprise API** | [Enterprise Licensing](https://docs.github.com/en/enterprise-cloud@latest/rest/enterprise-admin/licensing) | License Management |
| **GitHub Copilot API** | [Copilot User Management](https://docs.github.com/en/enterprise-cloud@latest/rest/copilot/copilot-user-management) | Seat Usage, Activity |
| **GitHub Advanced Security** | [GHAS Billing](https://docs.github.com/en/enterprise-cloud@latest/rest/billing/advanced-security) | Committer Tracking |

### Related Documents

- **[Architecture Overview](../architecture-overview.md)** - System design and data flows
- **[ADR-001: Project Setup](../ADRs/ADR-001-project-setup.md)** - Repository structure
- **[Requirements](./functional-nonfunctional.md)** - Feature specifications

---

## Troubleshooting

### Common Issues

**Issue:** 401 Unauthorized

**Causes:**

- Invalid or expired API token
- Token missing from Authorization header
- Wrong token for endpoint type

**Solution:**

```bash
# Verify token validity
curl -X GET "https://api.github.com/user" \
  -H "Authorization: Bearer YOUR_TOKEN"

# Should return user profile without 401 error
```

---

**Issue:** 403 Forbidden

**Causes:**

- Insufficient permissions for token scope
- Organization policy restrictions
- Enterprise admin requirements

**Solution:**

- Verify token has `admin:enterprise` scope (GitHub)
- Check organization/admin settings
- Request elevated permissions if needed

---

**Issue:** 429 Rate Limit Exceeded

**Causes:**

- Too many API calls in short timeframe
- Pagination not implemented correctly

**Solution:**

```bash
# Check rate limit headers
curl -X GET "https://api.github.com/rate_limit" \
  -H "Authorization: Bearer YOUR_PAT"

# Implement exponential backoff for retries
```

---

**Updated:** 2025-12-05  
