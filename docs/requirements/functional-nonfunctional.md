# Functional and Non-Functional Requirements  
Equans Operational Insights

**Version:** 1.0  
**Author:** Ahmad Alhaj Asaad  
**Based on stakeholder interviews with:** Viktor Klein, Brian Veltman, Henk  

---

# 1. Functional Requirements (MoSCoW)

## MUST HAVE
1. The system must collect user and license data from:
   - Atlassian Admin API (users, groups, license allocation)
   - GitHub Enterprise API (seats, Copilot usage, org members)
   - JFrog Artifactory API (usage metrics)
   - Trello API (boards/users)
2. The system must store collected data in PostgreSQL.
3. The dashboard must visualise:
   - Active vs inactive users  
   - License utilisation per platform  
   - Trends over time  
   - Team-based usage  
4. The system must support authentication (Equans SSO / Microsoft).
5. The frontend must display:
   - Overview dashboard  
   - Atlassian usage view  
   - GitHub usage view  
   - JFrog usage view  
   - Costs & chargeback  

## SHOULD HAVE
1. Filtering by team, BU, date range.
2. Export to CSV for reporting.
3. Automated daily data refresh.

## COULD HAVE
1. Alerts for unusual license costs.
2. Predictive analytics for license forecasting.
3. Integration with Power BI.

## WON’T HAVE (for MVP)
1. Write-back operations to external APIs.
2. Role-based access control beyond basic admin/user split.

---

# 2. Non-Functional Requirements

## Performance
- API responses must return within **< 200ms** for standard queries.
- The dashboard must load in **< 3 seconds**.

## Security
- All requests must use HTTPS.
- Tokens and secrets must be stored in GitHub Secrets or Docker env files.
- Data stored must comply with GDPR (email masking where required).

## Reliability
- System must recover gracefully from API rate limits.
- Logging must capture errors with correlation IDs.

## Maintainability
- Code must follow Equans coding standards.
- All modules must include unit tests.
- ADRs must be updated when architecture changes.

## Observability
- Backend must expose health endpoints.
- CI/CD must run tests for every PR.

---

# 3. KPIs Defined
- Monthly Active Users (MAU)  
- License Utilisation %  
- Cost per Team / Cost Center  
- Inactive user ratio  
- API success/failure rates  

---

# 4. Stakeholder Priorities Summary
| Stakeholder | Priority | Notes |
|------------|----------|-------|
| Viktor     | License transparency, cost reduction | High business impact |
| Brian      | Data accuracy & API feasibility     | Technical feasibility |
| Henk       | Executive overview & KPIs           | High-level insight |

---

# 5. Approval
- [ ] Viktor Klein  
- [ ] Brian Veltman  
- [ ] Henk  
