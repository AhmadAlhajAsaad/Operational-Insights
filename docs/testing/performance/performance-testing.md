# Performance Testing

**Status:** Draft  
**Date:** 2026-03-28  
**Author(s):** Ahmad Alhaj Asaad

---

## 1. Performance Requirements

| Metric            | SRS Eis | Requirement      | Measurement          |
| ----------------- | ------- | ---------------- | -------------------- |
| API Response Time | TM-08   | P95 < 200ms      | Latency percentiles  |
| Dashboard Load    | TM-09   | < 3 seconds      | Time to interactive  |
| Concurrent Users  | TC-02   | 100 simultaneous | Load testing         |
| Data Collection   | M-04    | < 5 minutes      | Full vendor sync     |
| Database Queries  | TS-02   | < 50ms           | Query execution time |
| Frontend Bundle   | TM-12   | < 300 KB gzip    | Build output         |

---

## 2. Performance Test Types

### 2.1 Load Testing

| Scenario    | Users | Duration | Success Criteria        |
| ----------- | ----- | -------- | ----------------------- |
| Normal Load | 50    | 30 min   | P95 < 200ms, 0% errors |
| Peak Load   | 100   | 15 min   | P95 < 500ms, < 1% errors |
| Stress Test | 200   | 10 min   | Graceful degradation    |

### 2.2 Endurance Testing

| Scenario       | Users | Duration | Success Criteria                       |
| -------------- | ----- | -------- | -------------------------------------- |
| Sustained Load | 30    | 8 hours  | No memory leaks, stable response times |

### 2.3 Spike Testing

| Scenario      | Pattern              | Success Criteria           |
| ------------- | -------------------- | -------------------------- |
| Traffic Spike | 10 -> 150 -> 10 users | Recovery within 30 seconds |

---

## 3. Performance Testing Tools

| Tool               | Purpose                          |
| ------------------ | -------------------------------- |
| k6                 | Load testing, scripted scenarios |
| PostgreSQL EXPLAIN | Query optimization               |
| Chrome Lighthouse  | Frontend performance profiling   |
| Chrome DevTools    | Network waterfall analysis       |

---

## 4. k6 Test Scripts

Alle scripts staan in `tests/performance/`. De gedeelde configuratie (base URL, org ID, zoektermen) staat in `config.js`.

| Script              | Scenario | Beschrijving                                   |
| ------------------- | -------- | ---------------------------------------------- |
| `load-test.js`      | P1/P2/P4 | 50 VUs, normaal werkdaggebruik, 30 min         |
| `peak-load.js`      | P5       | Ramp-up naar 100 VUs, 15 min sustained         |
| `stress-test.js`    | -        | Opschalen tot 200 VUs, degradatie meten        |
| `spike-test.js`     | -        | 10 -> 150 -> 10 VUs, hersteltijd meten         |
| `endurance-test.js` | -        | 30 VUs, 8 uur, memory leaks detecteren         |
| `import-test.js`    | P3       | CSV upload -> preview -> execute, 1 VU, 3x     |
| `sync-impact.js`    | P6       | 30 VUs + handmatige sync trigger, impact meten |

### Configuratie aanpassen

Bewerk `tests/performance/config.js` voor je omgeving:

```javascript
export const BASE_URL = __ENV.BASE_URL || "http://localhost:8080";
export const ATLASSIAN_ORG_ID = __ENV.ATLASSIAN_ORG_ID || "test-org";
```

Of override via environment variables:

```bash
k6 run -e BASE_URL=http://localhost:8080 -e ATLASSIAN_ORG_ID=abc123 tests/performance/load-test.js
```

---

## 5. Running Performance Tests

### Volgorde

1. Start Docker Compose: `cd infra && docker compose up -d`
2. Wacht tot de backend klaar is: `curl http://localhost:8080/health`
3. Draai tests in deze volgorde (load-test eerst als sanity check):

```bash
# 1. Load test (30 min)
k6 run --out json=results/load-test.json tests/performance/load-test.js

# 2. Peak load (25 min)
k6 run --out json=results/peak-load.json tests/performance/peak-load.js

# 3. Stress test (13 min)
k6 run --out json=results/stress-test.json tests/performance/stress-test.js

# 4. Spike test (8 min)
k6 run --out json=results/spike-test.json tests/performance/spike-test.js

# 5. Import test (5 min) - vereist testdata/persons-1000.csv
k6 run --out json=results/import-test.json tests/performance/import-test.js

# 6. Sync impact (10 min) - trigger sync apart
k6 run --out json=results/sync-impact.json tests/performance/sync-impact.js
# In apart terminal: curl -X POST http://localhost:8080/api/atlassian/users/sync

# 7. Endurance test (8 uur) - draai als laatste
k6 run --out json=results/endurance-test.json tests/performance/endurance-test.js
```

### Database Query Performance

```sql
-- Personen zoeken (scenario P2)
EXPLAIN ANALYZE
SELECT * FROM persons
WHERE (first_name ILIKE '%jan%' OR last_name ILIKE '%jan%' OR email ILIKE '%jan%')
AND status = 'active'
ORDER BY last_name
LIMIT 25 OFFSET 0;

-- Atlassian users ophalen (scenario P4)
EXPLAIN ANALYZE
SELECT * FROM atlassian_users_cache
WHERE expires_at > NOW()
ORDER BY display_name
LIMIT 25 OFFSET 0;

-- Check of indexes gebruikt worden
SELECT schemaname, tablename, indexname, idx_scan
FROM pg_stat_user_indexes
WHERE idx_scan = 0;
```

### Frontend Performance (scenario P1)

```bash
# Bundle size checken (TM-12: < 300 KB gzip)
cd frontend
npm run build
# Kijk naar de Vite output voor gzip sizes

# Lighthouse via Chrome CLI (optioneel)
npx lighthouse http://localhost:3000 --output=json --output-path=results/lighthouse.json
```

---

## 6. Acceptatiecriteria Mapping

| AC    | Criterium                    | Test Script         | Drempelwaarde              |
| ----- | ---------------------------- | ------------------- | -------------------------- |
| AC-01 | API bij 50 VUs               | `load-test.js`      | P95 < 200 ms               |
| AC-02 | Dashboard laden              | Lighthouse          | TTI < 3s, FCP < 1,5s       |
| AC-03 | Frontend bundel              | `npm run build`     | < 300 KB gzip              |
| AC-04 | Database queries             | `EXPLAIN ANALYZE`   | < 50 ms                    |
| AC-05 | Errors bij 50 VUs            | `load-test.js`      | 0%                         |
| AC-06 | Errors bij 100 VUs           | `peak-load.js`      | < 1%                       |
| AC-07 | Vendor-sync duur             | `sync-impact.js`    | < 5 minuten                |
| AC-08 | API bij 100 VUs              | `peak-load.js`      | P95 < 500 ms               |
| AC-09 | Geheugen na 8 uur            | `endurance-test.js` | Max 10% toename            |
| AC-10 | Herstel na spike             | `spike-test.js`     | Binnen 30 sec              |
| AC-11 | CSV-import 1.000 records     | `import-test.js`    | < 20 sec totaal            |
| AC-12 | Zoeken personen              | `load-test.js`      | < 200 ms                   |

---

## 7. Performance Optimization Checklist

### Backend

- [ ] Database queries geoptimaliseerd met indexes (migratie 007)
- [ ] Connection pool geconfigureerd (max 50, timeout 30s)
- [ ] Caching met TTL voor Atlassian/GitHub data
- [ ] Async verwerking voor imports via tokio
- [ ] Batch-inserts bij import execute

### Frontend

- [ ] Productie-build met minificatie
- [ ] Lazy loading voor routes
- [ ] Paginering server-side (niet client-side)
- [ ] Bundle size < 300 KB gzip

### Database

- [ ] Indexes op persons (email, org_id, status, full-text search)
- [ ] Indexes op atlassian_users_cache (expires_at)
- [ ] EXPLAIN ANALYZE op alle kritieke queries
- [ ] Connection pool tuning

---

## Related Documents

- [Performance Test Plan](../../Acad/Realisation/Tests/Performance%20Test%20Plan.md)
- [TR-001: Performance and Security Standards](../../Technical-Requirements/TR-001-Performance-Security-Standards.md)
- [SRS-001: Software Requirements Specification](../../Acad/Analysis/Software%20Requirements%20Specification%20(SRS).md)
