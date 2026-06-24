# UrbanLens Project State

> Purpose: This is the project-wide resume point. Read it before starting work.

## Project Snapshot

| Field | Value |
|---|---|
| Project | `UrbanLens` |
| Product | Tokyo commercial real-estate intelligence platform using official public data |
| Current Milestone | `MVP local platform foundation` |
| Current Phase | `01 — Local Platform Foundation` |
| Current Phase Status | `not_started` |
| Project Health | `green` |
| Last Updated | `2026-06-24` |
| Primary Owner | `Project owner` |
| Current Branch | `main` |
| Latest Commit | `be6ff47` |

---

## 1. Current Objective

Plan and build the smallest local platform foundation that runs Next.js, Rust/Actix, PostgreSQL/PostGIS, and supporting health checks without violating the completed Phase 0 source, lineage, metric, or location-precision decisions.

---

## 2. Active Phase

| Field | Value |
|---|---|
| Active Phase | `01 — Local Platform Foundation` |
| Phase Folder | `.planning/phases/01-local-platform-foundation/` (to create) |
| Plan | `.planning/phases/01-local-platform-foundation/PHASE-PLAN.md` (to create) |
| Status | `.planning/phases/01-local-platform-foundation/PHASE-STATUS.md` (to create) |
| UAT | `.planning/phases/01-local-platform-foundation/PHASE-UAT.md` (to create) |
| Phase Status | `not_started` |
| Phase Health | `green` |

### Why This Is the Active Phase

Phase 0 passed all eight UAT cases. A runnable local platform is now the next dependency for implementing ingestion, spatial storage, GraphQL, and the analyst workspace.

---

## 3. Exact Next Action

> **Do this first when resuming work:**

```text
Create Phase 01 planning documents from the templates, including a bounded authenticated MLIT API smoke test that never prints or stores the key.
```

### Resume Sequence

1. Read the completed Phase 00 status/UAT and accepted ADRs.
2. Confirm the branch, working tree, and latest commit.
3. Create the Phase 01 plan, status, and UAT files from the templates.
4. Include local API-key loading and a bounded authenticated smoke test without key disclosure.
5. Update this file after the Phase 01 plan is decision-complete.

---

## 4. Project Workstream Status

| Workstream | State | Active Phase | Notes |
|---|---|---|---|
| Product / Domain | Stable | Phase 00 | Workflow, claims, metrics, precision, and conceptual model complete. |
| Architecture | Stable | Phase 00 | ADRs 001–005 accepted. |
| Backend API | Not Started | Phase 01 | Local Actix/GraphQL service foundation is next. |
| Database / PostGIS | Not Started | Phase 01 | Local PostgreSQL/PostGIS service and migration mechanism are next. |
| Ingestion Pipeline | Stable | Phase 00 | Three official fixtures and approved API access are available. |
| Frontend Workspace | Not Started | Phase 01 | Local Next.js shell is next; analyst features remain later. |
| Testing | Stable | Phase 00 | All eight discovery UAT cases pass. |
| Infrastructure / CI | Not Started | Phase 01 | Docker Compose and baseline checks are next. |
| Documentation | Stable | Phase 00 | Required discovery documents are complete. |

---

## 5. Phase Roadmap

| Phase | Name | Status | Health | Exit Condition |
|---:|---|---|---|---|
| 00 | Product and Data Discovery | Completed | Green | First source, fixtures, workflow, model, access, and ADRs passed UAT |
| 01 | Local Platform Foundation | Not Started | Green | Web, API, and PostGIS run locally |
| 02 | Ingestion and Canonical Data Pipeline | Not Started | Green | Official data imports safely and repeatedly |
| 03 | Spatial Data Model and Query Engine | Not Started | Green | Viewport and area filtering work in PostGIS |
| 04 | Analyst Workspace v0.1 | Not Started | Green | Market map works with filters and source details |
| 05 | Market Metrics and Area Comparison | Not Started | Green | Two areas can be compared transparently |
| 06 | Provenance, Data Quality, and Import Operations | Not Started | Green | Data lineage and import status are visible |
| 07 | Operational Hardening and Deployment | Not Started | Green | Public portfolio deployment is ready |
| 08 | Advanced Data and Product Expansion | Not Started | Green | One justified advanced capability is complete |

---

## 6. Recently Completed Work

| Date | Completed Outcome | Phase | Evidence |
|---|---|---|---|
| 2026-06-24 | Confirmed MLIT API approval/local ignored `.env`; passed UAT-01/UAT-08 and completed Phase 0. | 00 | `docs/data-sources.md`, Phase 00 UAT |
| 2026-06-24 | Selected MLIT transaction-price information and completed source/access/schema/limitation documentation. | 00 | `docs/data-sources.md` |
| 2026-06-24 | Retrieved, profiled, and checksum-validated 666 official source observations across three Tokyo wards. | 00 | `workers/importer/fixtures/transactions/` |
| 2026-06-24 | Completed product brief, conceptual model, and accepted ADRs 001–005. | 00 | `docs/` |
| 2026-06-24 | Passed all repository-controlled validation and six of eight UAT cases. | 00 | `.planning/phases/00-product-and-data-discovery/PHASE-UAT.md` |
| 2026-06-24 | Created the Phase 0 plan, execution status, and source/product/data-focused UAT protocol. | 00 | `.planning/phases/00-product-and-data-discovery/` |
| 2026-06-24 | Aligned the project roadmap and resume point to Product and Data Discovery as Phase 00. | Project | `.planning/STATE.md` |

---

## 7. Active Blockers and Risks

### Blockers

| ID | Blocker | Impact | Owner | Next Action |
|---|---|---|---|---|
| — | No active blocker. | — | — | — |

### Risks

| ID | Risk | Likelihood | Impact | Mitigation |
|---|---|---:|---:|---|
| RSK-01 | Authenticated API connectivity has not completed from this execution environment. | Medium | Medium | Add a bounded, secret-safe smoke test to Phase 01 local setup. |
| RSK-02 | Source revisions can change historical query results. | Medium | Medium | Version each retrieval by query, timestamp, and artifact checksum. |
| RSK-03 | XPT station points cannot be guessed onto CSV/XIT rows. | High | High | Treat XPT features as their own source records or keep other observations spatially unknown. |

---

## 8. Project-Level Decisions

| Date | Decision | Why It Matters | Reference |
|---|---|---|---|
| 2026-06-24 | Use official public datasets only. | Avoids scraping, licensing uncertainty, and unsupported claims. | `AGENTS.md` |
| 2026-06-24 | Begin with one historical transaction-price source. | Keeps discovery, ingestion, and product claims narrow and testable. | Phase 00 plan |
| 2026-06-24 | Use `transaction_observation`, not a durable `property`, in the initial model. | Stable identity and exact location are not established. | Phase 00 plan |
| 2026-06-24 | Preserve explicit location precision. | Prevents approximate public geography from appearing as exact property coordinates. | ADR-004 |
| 2026-06-24 | Preserve raw source payloads and lineage. | Enables audit, reprocessing, and reproducible metrics. | ADR-003 |
| 2026-06-24 | Use GraphQL for the product API and PostGIS for spatial queries. | Supports bounded map/filter/metric/provenance workflows with database-level geography. | ADR-001/002 |

---

## 9. Technical Context

### Main Stack

```text
Frontend: Next.js + React + TypeScript
API: Rust + Actix Web + async-graphql
Database: PostgreSQL + PostGIS
Cache / Job Coordination: Redis only when justified
Local Infrastructure: Docker Compose
CI: GitHub Actions
```

### Important Product Rules

- UrbanLens is an analyst tool, not a property marketplace.
- Use official, public, legally usable datasets only.
- Never scrape private listing sites.
- Never imply false geographic precision.
- Preserve source lineage from normalized observations to raw source records.
- Prefer database-level filtering and aggregation.
- Every metric must show units, time range, sample size, and limitations.
- Do not create a durable `property` until stable source identity and location support it.
- Do not advance to the next phase before the current phase passes UAT.

---

## 10. Resume Commands

```bash
# Inspect completed discovery and the working tree
git status --short
sed -n '1,240p' .planning/phases/00-product-and-data-discovery/PHASE-STATUS.md

# Start Phase 01 planning from the templates
ls .planning/phases/XX-template
```

---

## 11. Update Rules

Update this file when:

- The selected source or credential status changes.
- The exact next action changes.
- A blocker appears, changes, or is resolved.
- A major project-level decision is made.
- The phase reaches `ready_for_uat`, `completed`, or `completed_with_exceptions`.
- The active phase changes.

Detailed discovery notes and progress belong in the active phase documents and `docs/` deliverables.

---

## 12. Last Session Handoff

### Last Session Summary

Completed Phase 0. MLIT API access is approved and configured in an ignored local `.env`; all discovery artifacts and all eight UAT cases pass. The key was neither printed nor stored in repository files.

### Where Work Stopped

Phase 00 is closed. Phase 01 planning documents do not yet exist.

### First File to Read Next Time

```text
.planning/STATE.md
```

### First Action Next Time

```text
Create .planning/phases/01-local-platform-foundation/ from the template and make authenticated API connectivity a secret-safe local smoke test.
```
