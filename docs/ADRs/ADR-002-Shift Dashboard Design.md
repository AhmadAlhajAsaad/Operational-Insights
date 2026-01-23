# ADR-002: Shift Dashboard Design to Organization-Centric, Data-Driven Structure

**Status:** Accepted  
**Date:** 2026-01-19  
**Deciders:** Viktor Klein (Product Lead), Ahmad Alhaj Asaad (Graduate Intern)  
**Context:** Equans Operational Insights Dashboard

## Context

The initial Figma dashboard design focused primarily on presenting static metrics such as license counts and user-level data. While visually clear, the design did not sufficiently support analysis, comparison, or decision-making.

Feedback from the Product Lead (Viktor Klein) highlighted that the dashboard must better reflect how Equans uses the data in practice: decisions are made at organization and business unit level, with cost and value as the primary drivers.

The dashboard is intended to support operational and financial insights, not individual user monitoring.

## Problem Statement

The original dashboard design had the following limitations:

- It was largely static, showing numbers without supporting interaction or analysis.
- It focused too much on individual users, which is not meaningful at Equans scale.
- Screens were designed before fully analysing the underlying data model.
- There was no clear drill-down structure from overview to detail.
- Cost, value, and trends were not sufficiently emphasized.

This conflicted with the goal of an Operational Insights Dashboard that should support comparison, optimization, and decision-making.

## Decision

The dashboard design will be restructured to be **organization- and business unit–centric**, and **fully data-driven**.

### Key decisions:

#### 1. Primary aggregation level
- The dashboard starts at **Organization / Business Unit level**.
- Individual users are treated as an optional, lowest-level detail.

#### 2. Data-first design approach
The design will be derived from:
- Available datasets
- Fields and columns
- Meaningful aggregations (costs, counts, trends)
- Visual components are created after the data structure is defined.

#### 3. Interactive drill-down model
- **Overview → Product → (Optional) User**
- Each level allows users to explore deeper insights through interaction.

#### 4. Cost and value as core metrics
Dashboards must answer:
- What does this organization / BU cost?
- Which products drive the highest costs?
- How do costs and usage change over time?
- Comparisons and trends are mandatory.

#### 5. User detail is secondary
User-level views may exist, but:
- Not as the main entry point
- Not as a primary use case
- Only as the final drill-down layer

## Consequences

### Positive
- The dashboard aligns with real Equans decision-making processes.
- Supports financial insights, chargeback, and optimization.
- Encourages interactive analysis instead of static reporting.
- Provides a scalable model suitable for large user volumes.
- Improves stakeholder relevance (finance, product owners, platform teams).

### Trade-offs
- Requires additional design effort to model drill-down flows.
- User-level insights become less prominent.
- Historical and aggregated data storage becomes more important.

## Implementation Actions

1. **Update Figma designs** so that:
   - Organization / Business Unit is the main entry point.
   - Costs, license counts, and product distribution are immediately visible.

2. **Visually represent interaction:**
   - Clickable rows and cards
   - Clear navigation hints (e.g. "Click to view details")
   - Logical drill-down hierarchy

3. **Add visualizations focused on:**
   - Cost per BU (bar charts)
   - Cost and usage trends over time (line charts)
   - Product cost distribution

4. Prepare a revised design for a follow-up session with the Product Lead, using real or realistic data examples.

## Summary

This decision shifts the dashboard from a static, user-focused view to an interactive, data-driven design where organizations and business units are central, enabling cost-based insights, comparisons, and drill-downs that support operational and financial decision-making.