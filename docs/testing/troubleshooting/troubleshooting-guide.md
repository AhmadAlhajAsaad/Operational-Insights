# Troubleshooting Guide

**Status:** Draft  
**Date:** 2026-02-11  
**Author(s):** DP-DevEx-Platform Team

---

## 1. Common Test Errors

| Error                   | Cause                          | Resolution                                              |
| ----------------------- | ------------------------------ | ------------------------------------------------------- |
| `401 Unauthorized`      | Invalid or expired token       | Verify token in environment variables, check expiration |
| `403 Forbidden`         | Insufficient permissions       | Verify API scopes and user roles                        |
| `429 Too Many Requests` | Rate limit exceeded            | Implement request throttling, add delays between tests  |
| `Connection Refused`    | Service not running            | Start required services, check port bindings            |
| `Timeout`               | Slow response or network issue | Increase timeout, check network connectivity            |

---

## 2. Error Details and Solutions

### 2.1 Authentication Errors

#### 401 Unauthorized

**Symptoms:**

- API returns `401 Unauthorized`
- Error message: "Invalid or expired token"

**Diagnosis:**

```bash
# Check if token is set
echo $TEST_AUTH_TOKEN

# Verify token format (should be valid JWT)
echo $TEST_AUTH_TOKEN | cut -d'.' -f2 | base64 -d
```

**Solutions:**

1. **Expired Token:**

   ```bash
   # Generate new token
   cargo run --bin generate-test-token
   export TEST_AUTH_TOKEN=$(cat .test-token)
   ```

2. **Missing Token:**
   - Ensure `.env.test` file exists
   - Run `source .env.test` before tests

3. **Invalid Token Format:**
   - Verify token is properly formatted JWT
   - Check for accidental whitespace or line breaks

#### 403 Forbidden

**Symptoms:**

- API returns `403 Forbidden`
- Error message: "Insufficient permissions"

**Diagnosis:**

```bash
# Check token scopes
cargo run --bin decode-token -- $TEST_AUTH_TOKEN
```

**Solutions:**

1. **Missing Scopes:**
   - Regenerate token with required scopes
   - Verify user has necessary roles

2. **Resource Not Accessible:**
   - Check if user belongs to correct team
   - Verify resource ownership

### 2.2 Rate Limiting Errors

#### 429 Too Many Requests

**Symptoms:**

- API returns `429 Too Many Requests`
- Tests fail intermittently

**Diagnosis:**

```bash
# Check rate limit headers
curl -I http://localhost:8080/api/v1/licenses
# Look for: X-RateLimit-Remaining, X-RateLimit-Reset
```

**Solutions:**

1. **Add Delays Between Requests:**

   ```javascript
   // k6 - Add sleep between requests
   import { sleep } from "k6";
   export default function () {
     http.get(url);
     sleep(1); // Wait 1 second
   }
   ```

2. **Implement Retry Logic:**

   ```rust
   async fn request_with_retry(url: &str) -> Result<Response, Error> {
       for attempt in 0..3 {
           match client.get(url).send().await {
               Ok(res) if res.status() == 429 => {
                   let wait = 2u64.pow(attempt);
                   tokio::time::sleep(Duration::from_secs(wait)).await;
               }
               result => return result,
           }
       }
       Err(Error::RateLimited)
   }
   ```

3. **Reduce Parallel Requests:**
   - Lower `vus` in k6 tests
   - Use sequential instead of parallel test execution

### 2.3 Connection Errors

#### Connection Refused

**Symptoms:**

- `Connection refused` error
- Tests cannot reach service

**Diagnosis:**

```bash
# Check if service is running
docker ps

# Check port bindings
netstat -tlnp | grep 8080

# Test connectivity
curl http://localhost:8080/health
```

**Solutions:**

1. **Start Services:**

   ```bash
   docker-compose -f docker-compose.test.yml up -d
   ```

2. **Check Port Conflicts:**

   ```bash
   # Find process using port
   lsof -i :8080

   # Kill conflicting process or use different port
   ```

3. **Verify Network:**
   - Check Docker network configuration
   - Ensure services are on same network

#### Timeout Errors

**Symptoms:**

- Requests time out
- Inconsistent test failures

**Solutions:**

1. **Increase Timeout:**

   ```rust
   let client = reqwest::Client::builder()
       .timeout(Duration::from_secs(30))
       .build()?;
   ```

2. **Check Resource Usage:**

   ```bash
   # Monitor CPU/Memory
   docker stats

   # Check database performance
   docker exec -it postgres-test pg_stat_activity
   ```

---

## 3. Database Issues

### Connection Pool Exhausted

**Symptoms:**

- `too many connections` error
- Tests hang waiting for connection

**Solutions:**

1. **Increase Pool Size:**

   ```rust
   let pool = PgPoolOptions::new()
       .max_connections(20)
       .connect(&database_url).await?;
   ```

2. **Ensure Connections Released:**
   - Check for connection leaks in tests
   - Use connection pooling correctly

### Migration Failures

**Symptoms:**

- `relation does not exist` errors
- Schema mismatch

**Solutions:**

```bash
# Reset and re-run migrations
cargo run --bin reset-test-db
cargo run --bin migrate
```

---

## 4. Test Flakiness

### Identifying Flaky Tests

| Symptom                 | Likely Cause                 |
| ----------------------- | ---------------------------- |
| Random failures         | Race conditions, timing      |
| Fails only in CI        | Environment differences      |
| Fails with other tests  | Test pollution, shared state |
| Fails on first run only | Missing setup                |

### Mitigation Strategies

| Issue              | Mitigation                                     |
| ------------------ | ---------------------------------------------- |
| Flaky tests        | Add retry logic, improve test isolation        |
| Slow tests         | Parallelize execution, optimize setup/teardown |
| Environment issues | Use containers, document dependencies          |
| Data dependencies  | Use factories, reset state between tests       |

### Test Isolation Best Practices

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Good: Each test creates its own data
    #[tokio::test]
    async fn test_user_creation() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await;

        // Test logic...

        cleanup_test_user(&pool, user.id).await;
    }

    // Bad: Relies on external state
    #[tokio::test]
    async fn test_user_query() {
        // This test assumes user "test-user" exists
        let user = get_user("test-user").await;
    }
}
```

---

## 5. Mock API Issues

### WireMock Not Responding

**Diagnosis:**

```bash
# Check WireMock status
curl http://localhost:8089/__admin/mappings

# Check logs
docker logs wiremock
```

**Solutions:**

1. **Restart WireMock:**

   ```bash
   docker-compose restart wiremock
   ```

2. **Verify Mappings:**
   - Check `tests/mocks/mappings/` directory
   - Ensure JSON is valid

### Stale Mock Responses

**Solution:**

```bash
# Clear and reload mappings
curl -X POST http://localhost:8089/__admin/mappings/reset
```

---

## 6. CI/CD Specific Issues

### Tests Pass Locally, Fail in CI

**Common Causes:**

1. **Environment Differences:**
   - Check Node/Rust versions match
   - Verify environment variables are set

2. **Timing Issues:**
   - CI runners may be slower
   - Add appropriate waits/retries

3. **Resource Constraints:**
   - CI runners have limited resources
   - Reduce parallelization if needed

**Debugging CI Failures:**

```yaml
# Add verbose logging
- name: Run tests
  run: cargo test --workspace -- --nocapture
  env:
    RUST_BACKTRACE: 1
    RUST_LOG: debug
```

---

## 7. Quick Reference

### Diagnostic Commands

```bash
# Check all services
docker-compose ps

# View logs
docker-compose logs -f backend

# Database connectivity
psql $DATABASE_URL -c "SELECT 1"

# API health
curl http://localhost:8080/health

# Test token validity
curl -H "Authorization: Bearer $TEST_AUTH_TOKEN" http://localhost:8080/api/v1/me
```

### Reset Everything

```bash
# Nuclear option - reset entire test environment
docker-compose -f docker-compose.test.yml down -v
docker-compose -f docker-compose.test.yml up -d
cargo run --bin migrate
cargo run --bin seed-test-data
```

---

## Related Documents

- [Environment Setup](../environment/environment-setup.md)
- [API Testing](../api/api-testing.md)
- [Test Management](../management/test-management.md)
