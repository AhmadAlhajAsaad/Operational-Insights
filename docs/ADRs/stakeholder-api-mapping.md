# Mapping Table: Stakeholder Questions ↔ API Endpoints

**By Ahmad Alhaj ASAAD**

This table provides an overview of which stakeholder questions can currently be fully, partially, or not answered based on the available APIs from Atlassian and GitHub.

**Legend**

- ✅ = Fully answerable
- ⚠️ = Partially answerable
- ❌ = Not answerable with current APIs

---

## 📊 Financial & Chargeback (Finance, PMO, SLS DP)

| **Stakeholder Question** | **Platform** | **API-endpoint(s)** | **Relevant Fields** | **Status** | **Remarks** |
|--------------------------|--------------|---------------------|---------------------|------------|-------------|
| How many licenses are in use per product? | Atlassian | `/admin/v2/orgs/{orgId}/directories/-/users` (via groups) | accountId, membershipStatus | ✅ | Implemented via license-count endpoint |
| How many active licenses are there per product? | Atlassian | `/directories/-/users?groupIds=` | account_status, membership_status | ✅ | Active = *active* + *membership active* |
| How many GitHub Enterprise licenses are consumed? | GitHub | `/enterprises/{enterprise}/consumed-licenses` | total_seats_consumed | ✅ | Directly available |
| How many licenses are available (remaining)? | GitHub | `/consumed-licenses` | total_seats_purchased | ✅ | Derived: *purchased − consumed* |
| What are the costs per product per month? | — | — | — | ❌ | Costs are not included in the API, external pricing model needed |
| What are the costs per team / BU? | — | — | — | ❌ | No cost-center mapping in APIs |

---

## 👤 Users & License Usage (Product Owners, Security)

| **Stakeholder Question** | **Platform** | **API-endpoint(s)** | **Relevant Fields** | **Status** | **Remarks** |
|--------------------------|--------------|---------------------|---------------------|------------|-------------|
| Who uses Jira / Confluence / Trello? | Atlassian | `/directories/-/users?groupIds=` | name, email, accountId | ✅ | Via product groups |
| Which users are active? | Atlassian | `/admin/v1/orgs/{orgId}/users` | account_status | ⚠️ | Activity data is limited/inconsistent |
| Which users are inactive but have a license? | Atlassian | `/directories/-/users` | account_status, membershipStatus | ⚠️ | No reliable last_active data |
| Which users have Copilot? | GitHub | `/copilot/billing/seats` | login, plan_type | ✅ | Fully available |
| When was a Copilot user last active? | GitHub | `/copilot/billing/seats` | last_activity | ✅ | Well usable |

---

## 🔐 Security & Compliance (Security, CISO)

| **Stakeholder Question** | **Platform** | **API-endpoint(s)** | **Relevant Fields** | **Status** | **Remarks** |
|--------------------------|--------------|---------------------|---------------------|------------|-------------|
| Which repositories use GHAS? | GitHub | `/settings/billing/advanced-security` | repositories[] | ✅ | Fully available |
| How many active committers are there (GHAS)? | GitHub | `/advanced-security` | committers | ⚠️ | Sometimes null |
| Can we monitor individual performance? | — | — | — | ❌ | Not allowed / GDPR risk |
| Can data be anonymized? | — | — | — | ⚠️ | Must be enforced in backend |

---

## 📈 Trends & History (Management)

| **Stakeholder Question** | **Platform** | **API-endpoint(s)** | **Relevant Fields** | **Status** | **Remarks** |
|--------------------------|--------------|---------------------|---------------------|------------|-------------|
| How does license usage develop over time? | — | — | — | ❌ | APIs do not provide historical data |
| Can we show monthly trends? | — | — | — | ⚠️ | Only possible with own storage (snapshots) |
| Forecast vs. realization possible? | — | — | — | ❌ | External financial input needed |

---

## 🔎 Summary

**Fully achievable (API-driven)**
- License counts per product (Atlassian, GitHub)
- Copilot seat usage & activity
- GHAS repository usage

**Partially achievable**
- Inactive users
- Activity at Atlassian
- Trend analysis (only with own data storage)

**Not achievable without additional resources**
- Costs per team / BU
- Historical trends from API
- Financial forecast