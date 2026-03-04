# BR-001: Multi-Vendor License Insights Platform

**Status:** Draft  
**Date:** 2026-02-11  
**Author(s):** Ahmad Alhaj Asaad  
**Stakeholders:** Viktor Klein, Brian Veltman, Henk

---

## Problem Statement

Organizations using multiple software platforms (Atlassian, GitHub Enterprise, JFrog Artifactory, Trello) lack centralized visibility into:

- License utilization across vendors
- Active vs. inactive user counts
- Per-team and per-cost-center software expenses
- Usage trends over time

This fragmentation leads to:

- Overpaying for unused licenses
- Difficulty in budget allocation and chargeback processes
- Limited executive oversight of software spend

---

## Business Value

| Benefit              | Impact                                             |
| -------------------- | -------------------------------------------------- |
| License transparency | Identify unused licenses for cost recovery         |
| Cost reduction       | Optimize license allocation based on actual usage  |
| Chargeback accuracy  | Attribute costs to teams/business units            |
| Executive oversight  | Provide high-level KPIs for decision-making        |
| Time savings         | Eliminate manual data aggregation across platforms |

---

## Stakeholders

| Stakeholder   | Role              | Priority Focus                       |
| ------------- | ----------------- | ------------------------------------ |
| Viktor Klein  | Business Owner    | License transparency, cost reduction |
| Brian Veltman | Technical Lead    | Data accuracy, API feasibility       |
| Henk          | Executive Sponsor | Executive overview, KPIs             |

---

## Success Criteria

The platform will be considered successful when the following KPIs are measurable:

- [ ] **Monthly Active Users (MAU)** — Trackable per platform
- [ ] **License Utilization %** — Visible per vendor and team
- [ ] **Cost per Team / Cost Center** — Attributable and reportable
- [ ] **Inactive User Ratio** — Identifiable for license optimization
- [ ] **API Success/Failure Rates** — Monitored for data reliability

---

## Scope

### In Scope (MVP)

- Data collection from Atlassian Admin API, GitHub Enterprise API, JFrog Artifactory API, Trello API
- Centralized data storage in PostgreSQL
- Dashboard visualization of license and usage metrics
- Authentication via Equans SSO / Microsoft
- Basic admin/user access control

### Out of Scope (MVP)

- Write-back operations to external vendor APIs
- Advanced role-based access control beyond admin/user
- Real-time streaming data (batch refresh only)

---

## Dependencies

| Dependency             | Type           | Notes                             |
| ---------------------- | -------------- | --------------------------------- |
| Atlassian Admin API    | External       | Users, groups, license allocation |
| GitHub Enterprise API  | External       | Seats, Copilot usage, org members |
| JFrog Artifactory API  | External       | Usage metrics                     |
| Trello API             | External       | Boards, users                     |
| PostgreSQL             | Infrastructure | Data storage                      |
| Equans SSO / Microsoft | Infrastructure | Authentication                    |

---

## Related Documents

- Functional Requirement: [FR-001-License-Dashboard](../Functional-Requirements/FR-001-License-Dashboard.md)
- Functional Requirement: [FR-002-Vendor-Data-Collection](../Functional-Requirements/FR-002-Vendor-Data-Collection.md)
- Technical Requirement: [TR-001-Performance-Security-Standards](../Technical-Requirements/TR-001-Performance-Security-Standards.md)
