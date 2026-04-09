# API Testing

**Status:** Draft  
**Date:** 2026-02-11  
**Author(s):** DP-DevEx-Platform Team

---

## 1. API Test Strategy

All REST API endpoints must be tested for:

| Aspect               | Test Cases                                          |
| -------------------- | --------------------------------------------------- |
| **Functionality**    | Happy path, edge cases, error conditions            |
| **Input Validation** | Invalid data types, missing fields, boundary values |
| **Response Format**  | Correct JSON structure, HTTP status codes           |
| **Authentication**   | Valid/invalid/expired tokens                        |
| **Authorization**    | Role-based access enforcement                       |

---

## 2. Postman Collections

### Collection Structure

| Collection     | Purpose                                | Variables                              |
| -------------- | -------------------------------------- | -------------------------------------- |
| Atlassian APIs | Test Atlassian Admin API integration   | `baseUrl`, `orgId`, `atlassianToken`   |
| GitHub APIs    | Test GitHub Enterprise API integration | `baseUrl`, `enterprise`, `githubToken` |
| JFrog APIs     | Test JFrog Artifactory integration     | `baseUrl`, `jfrogToken`                |
| Internal APIs  | Test backend endpoints                 | `baseUrl`, `authToken`                 |

### Environment Variables

| Variable     | Description               | Example                        |
| ------------ | ------------------------- | ------------------------------ |
| `baseUrl`    | API base URL              | `http://localhost:8080/api/v1` |
| `orgId`      | Atlassian organization ID | `org-12345`                    |
| `enterprise` | GitHub Enterprise name    | `equans-enterprise`            |
| `authToken`  | JWT authentication token  | `{{TEST_AUTH_TOKEN}}`          |

### Environment Configuration

```json
{
  "id": "equans-test-env",
  "name": "Test Environment",
  "values": [
    { "key": "baseUrl", "value": "http://localhost:8080/api/v1" },
    { "key": "orgId", "value": "{{TEST_ORG_ID}}" },
    { "key": "enterprise", "value": "{{TEST_ENTERPRISE}}" },
    { "key": "authToken", "value": "{{TEST_AUTH_TOKEN}}" }
  ]
}
```

> ⚠️ **Security:** Never commit real tokens. Use environment variables injected at runtime.

---

## 3. API Test Cases

### License Endpoint Testing

| Test ID | Endpoint                | Method | Test Case               | Expected Result       |
| ------- | ----------------------- | ------ | ----------------------- | --------------------- |
| API-001 | `/api/v1/licenses`      | GET    | Valid request with auth | 200 OK, license data  |
| API-002 | `/api/v1/licenses`      | GET    | Missing auth token      | 401 Unauthorized      |
| API-003 | `/api/v1/licenses`      | GET    | Expired token           | 401 Unauthorized      |
| API-004 | `/api/v1/licenses/{id}` | GET    | Invalid license ID      | 404 Not Found         |
| API-005 | `/api/v1/licenses`      | GET    | Rate limit exceeded     | 429 Too Many Requests |

### Atlassian API Testing

| Test ID | Endpoint                       | Method | Test Case                | Expected Result      |
| ------- | ------------------------------ | ------ | ------------------------ | -------------------- |
| ATL-001 | `/api/v1/atlassian/users`      | GET    | Fetch all users          | 200 OK, user list    |
| ATL-002 | `/api/v1/atlassian/groups`     | GET    | Fetch all groups         | 200 OK, group list   |
| ATL-003 | `/api/v1/atlassian/licenses`   | GET    | Fetch license allocation | 200 OK, license data |
| ATL-004 | `/api/v1/atlassian/users/{id}` | GET    | Invalid user ID          | 404 Not Found        |

### GitHub API Testing

| Test ID | Endpoint                       | Method | Test Case             | Expected Result       |
| ------- | ------------------------------ | ------ | --------------------- | --------------------- |
| GH-001  | `/api/v1/github/seats`         | GET    | Fetch seat allocation | 200 OK, seat data     |
| GH-002  | `/api/v1/github/copilot/usage` | GET    | Fetch Copilot usage   | 200 OK, usage metrics |
| GH-003  | `/api/v1/github/members`       | GET    | Fetch org members     | 200 OK, member list   |
| GH-004  | `/api/v1/github/seats`         | GET    | Invalid enterprise    | 404 Not Found         |

---

## 4. Running API Tests

### Using Postman CLI (Newman)

```bash
# Run all collections
newman run collections/atlassian-apis.json -e environments/test.json
newman run collections/github-apis.json -e environments/test.json
newman run collections/internal-apis.json -e environments/test.json

# Run with HTML report
newman run collections/internal-apis.json -e environments/test.json -r html
```

### Using Cargo Tests

```bash
# Run API integration tests
cargo test --test api_tests

# Run with verbose output
cargo test --test api_tests -- --nocapture
```

---

## 5. Response Validation

### Expected Response Structure

```json
{
  "success": true,
  "data": { ... },
  "meta": {
    "page": 1,
    "total": 100,
    "timestamp": "2026-02-11T10:00:00Z"
  }
}
```

### Error Response Structure

```json
{
  "success": false,
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Invalid or expired token"
  }
}
```

---

## Related Documents

- [Auth Testing](../security/auth-testing.md)
- [Test Strategy](../strategy/test-strategy.md)
- [Troubleshooting](../troubleshooting/troubleshooting-guide.md)
