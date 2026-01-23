# 📘 API Data Structure and Response Format Analysis  

## ⚙️ 1. Atlassian Cloud API – Users, Groups, and Licenses

### 1.1 Endpoint: Get Organizations

**URL:**  
`GET https://api.atlassian.com/admin/v1/orgs`  
**Source:** [Atlassian Cloud Admin REST API – Organizations](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-orgs/?utm_source=chatgpt.com)

**Example response (simplified):**
```json
{
  "data": [
    {
      "id": "21959ca7-236b-11j7-k470-6dc106467633",
      "attributes": {
        "name": "myequans"
      },
      "links": {
        "self": "https://api.atlassian.com/admin/v1/orgs/21959ca7-236b-11j7-k470-6dc106467633"
      }
    }
  ]
}
```

**Important fields:**

| Field                     | Type   | Description                                 |
|---------------------------|--------|---------------------------------------------|
| `data[].id`               | String | Unique organization ID (orgId)              |
| `data[].attributes.name`  | String | Name of the organization                    |
| `links.self`              | String | Link to the specific organization           |

**Usage:**  
This endpoint provides basic organization data. The `orgId` is required for all subsequent queries (such as users and groups).

---

### 1.2 Endpoint: Get Users (Managed Accounts)

**URL:**  
`GET https://api.atlassian.com/admin/v1/orgs/{orgId}/users`  
**Source:** [Atlassian REST API – Users](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-users/?utm_source=chatgpt.com)

**Example response:**
```json
{
  "data": [
    {
      "account_id": "557058:1368c49b-068b-45a8-9d9f-bb94661d0175",
      "name": "Stephane LOBJOIS",
      "email": "ich168@equans.com",
      "account_status": "active",
      "last_active": "2025-12-03T13:10:59.759Z",
      "product_access": [
        {
          "name": "Jira Software",
          "key": "jira-software",
          "url": "tinea-data.atlassian.net"
        }
      ]
    }
  ]
}
```

**Important fields:**

| Field                     | Type   | Description                                 |
|---------------------------|--------|---------------------------------------------|
| `account_id`              | String | Unique Atlassian user ID                    |
| `name`                    | String | Full name of the user                       |
| `email`                   | String | Email address (GDPR-sensitive)              |
| `account_status`          | String | Account status (active / inactive / suspended) |
| `last_active`             | String | Last active timestamp                       |
| `product_access[].name`   | String | Name of product (Jira, Confluence, Trello, etc.) |
| `product_access[].url`    | String | URL of site or workspace                    |

**Usage:**  
The `product_access` array shows which products a user actively uses.  
This data is crucial for usage reports and license audits.

---

### 1.3 Endpoint: Get Groups

**URL:**  
`GET https://api.atlassian.com/admin/v2/orgs/{orgId}/directories/-/groups`  
**Source:** [Atlassian Directory API – Groups](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-groups/?utm_source=chatgpt.com)

**Example response:**
```json
{
  "data": [
    {
      "id": "1a33bac2-fd80-4a92-8e21-8e018ae83ead",
      "name": "AARSEN-admins",
      "directoryId": "5c930cbb-1905-4b15-838d-935f1d69d4f5",
      "externalSynced": false
    }
  ]
}
```

**Important fields:**

| Field           | Type    | Description                                         |
|-----------------|---------|-----------------------------------------------------|
| `id`            | String  | Unique group ID                                     |
| `name`          | String  | Name of the group                                   |
| `directoryId`   | String  | Directory where the group resides                   |
| `externalSynced`| Boolean | True = external source (e.g., Azure AD), False = internal |

**Usage:**  
Group names often contain product names (e.g., _jira-software-users_),  
so this endpoint is used to identify license groups.

---

### 1.4 Endpoint: Get Users per Group

**URL:**  
`GET https://api.atlassian.com/admin/v2/orgs/{orgId}/directories/-/users?groupIds={groupId}`  
**Source:** Atlassian Directory API – Users per Group

**Example response:**
```json
{
  "data": [
    {
      "accountId": "64143e682cee496759eb0cf8",
      "name": "Edwin ZWANENBURG",
      "email": "dl6021@equans.com",
      "status": "active",
      "membershipStatus": "active"
    }
  ]
}
```

**Important fields:**

| Field             | Type   | Description                                 |
|-------------------|--------|---------------------------------------------|
| `accountId`       | String | User ID (links to managed account)          |
| `name`            | String | Name of user                                |
| `email`           | String | Email address                               |
| `status`          | String | Active status of user                       |
| `membershipStatus`| String | Status of group membership                  |

**Usage:**  
Used for calculating active licenses per product.  
Only users with `status = active` and `membershipStatus = active` are counted.

---

## 💻 2. GitHub Enterprise Cloud API – Licenses, Copilot, and GHAS

### 2.1 Endpoint: Get License Consumption

**URL:**  
`GET https://api.github.com/enterprises/equans/consumed-licenses`  
**Source:** [GitHub REST API – Enterprise Licensing](https://docs.github.com/en/enterprise-cloud@latest/rest/enterprise-admin/licensing)

**Example response:**
```json
{
  "total_seats_consumed": 480,
  "total_seats_purchased": 500,
  "users": [
    {
      "github_com_login": "PL5999_equans",
      "github_com_name": "MERCIER Daniel (EQUANS)",
      "license_type": "Enterprise"
    }
  ]
}
```

**Important fields:**

| Field                   | Type    | Description                                 |
|-------------------------|---------|---------------------------------------------|
| `total_seats_consumed`  | Integer | Number of licenses used                     |
| `total_seats_purchased` | Integer | Total number of available licenses          |
| `users[].github_com_login` | String | GitHub login name                        |
| `users[].github_com_name`  | String | Full name of user                        |
| `users[].license_type`     | String | License type (Enterprise, Business, etc.) |

**Usage:**  
Calculates license consumption and supports chargeback analysis per business unit.

---

### 2.2 Endpoint: Get Copilot Seats

**URL:**  
`GET https://api.github.com/enterprises/equans/copilot/billing/seats`  
**Source:** [GitHub REST API – Copilot User Management](https://docs.github.com/en/enterprise-cloud@latest/rest/copilot/copilot-user-management)

**Example response:**
```json
{
  "total_seats": 125,
  "seats": [
    {
      "plan_type": "business",
      "last_activity_at": "2025-12-03T15:00:56+01:00",
      "assignee": {
        "login": "BM8226_equans",
        "id": 185944009
      }
    }
  ]
}
```

**Important fields:**

| Field                   | Type    | Description                                 |
|-------------------------|---------|---------------------------------------------|
| `total_seats`           | Integer | Total number of Copilot licenses            |
| `seats[].plan_type`     | String  | License type (business / enterprise)        |
| `seats[].last_activity_at` | String | Last activity of user                    |
| `seats[].assignee.login`   | String | Username with Copilot seat                |

**Usage:**  
Measures Copilot usage per user and detects inactive licenses for optimization.

---

### 2.3 Endpoint: GHAS (GitHub Advanced Security) Usage

**URL:**  
`GET https://api.github.com/enterprises/equans/settings/billing/advanced-security`  
**Source:** GitHub REST API – Advanced Security Active Committers

**Example response:**
```json
{
  "repositories": [
    {
      "name": "EquansCorporate/FinancialCockpitFront",
      "advanced_security_committers": 3,
      "advanced_security_committers_breakdown": [
        { "user_login": "GD8090_equans" }
      ]
    }
  ]
}
```

**Important fields:**

| Field                                         | Type    | Description                                 |
|-----------------------------------------------|---------|---------------------------------------------|
| `repositories[].name`                         | String  | Repository name                             |
| `repositories[].advanced_security_committers` | Integer | Number of active committers                 |
| `repositories[].advanced_security_committers_breakdown[].user_login` | String | Usernames of committers |

**Usage:**  
Supports reporting on GHAS usage per repository or team.

---

## 📊 Summary

| Platform                | Main APIs                        | Data Type                | Used for                                 |
|-------------------------|----------------------------------|--------------------------|------------------------------------------|
| **Atlassian Cloud**     | Organizations, Users, Groups, Directory Users | User and group data         | License analysis, activity, cost tracking |
| **GitHub Enterprise Cloud** | Licensing, Copilot, GHAS      | License and user activity | Copilot usage, GHAS audit, chargeback    |

---

## 📚 Sources

1. [Atlassian Cloud REST API – Users & Groups](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-users/?utm_source=chatgpt.com)
2. [GitHub REST API – Licensing & Security](https://docs.github.com/en/enterprise-cloud@latest/rest)
3. Own test results from Postman & PowerShell tests (`run_all_tests.ps1` log)

---
