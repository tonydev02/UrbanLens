# UrbanLens Project State

> Purpose: This is the project-wide resume point. Read it before starting work.

## Project Snapshot

| Field | Value |
|---|---|
| Project | `UrbanLens` |
| Product | Tokyo commercial real-estate intelligence platform using official public data |
| Current Milestone | `MVP discovery baseline` |
| Current Phase | `00 — Product and Data Discovery` |
| Current Phase Status | `blocked` |
| Project Health | `red` |
| Last Updated | `2026-06-24 07:24 +07:00` |
| Primary Owner | `Project owner` |
| Current Branch | `main` |
| Latest Commit | `N/A — repository has no initial commit` |

---

## 1. Current Objective

Close Phase 0 by confirming submission of the MLIT API application. All repository-controlled discovery outputs—selected source, official fixtures, analyst workflow, claim boundary, conceptual model, ADRs, and validation—are complete.

---

## 2. Active Phase

| Field | Value |
|---|---|
| Active Phase | `00 — Product and Data Discovery` |
| Phase Folder | `.planning/phases/00-product-and-data-discovery/` |
| Plan | `.planning/phases/00-product-and-data-discovery/PHASE-PLAN.md` |
| Status | `.planning/phases/00-product-and-data-discovery/PHASE-STATUS.md` |
| UAT | `.planning/phases/00-product-and-data-discovery/PHASE-UAT.md` |
| Phase Status | `blocked` |
| Phase Health | `red` |

### Why This Is the Active Phase

Source, schema, identity, geography, legal use, fixtures, and product boundaries are resolved. The phase remains active only because required API application submission needs the user’s identity and attestations.

---

## 3. Exact Next Action

> **Do this first when resuming work:**

```text
Submit the MLIT API application at https://www.reinfolib.mlit.go.jp/api/request/ and confirm the submission date; do not share the issued key.
```

### Resume Sequence

1. Read the active phase `PHASE-STATUS.md`.
2. Confirm the branch and working tree; this repository currently has no initial commit.
3. Confirm the user’s MLIT API application submission date.
4. Update the access-status row and rerun UAT-01/UAT-08.
5. Update `PHASE-STATUS.md` and this file after meaningful progress.

---

## 4. Project Workstream Status

| Workstream | State | Active Phase | Notes |
|---|---|---|---|
| Product / Domain | Stable | Phase 00 | Workflow, claims, metrics, precision, and conceptual model complete. |
| Architecture | Stable | Phase 00 | ADRs 001–005 accepted. |
| Backend API | Not Started | Future phase | Phase 0 documents needs but writes no API code. |
| Database / PostGIS | Not Started | Future phase | Conceptual model only in Phase 0. |
| Ingestion Pipeline | In Progress | Phase 00 | Three source fixtures ready; production importer remains a later phase. |
| Frontend Workspace | Not Started | Future phase | Workflow and map semantics precede implementation. |
| Testing | In Progress | Phase 00 | Repository checks pass; UAT blocked 6 passed / 2 blocked. |
| Infrastructure / CI | Not Started | Future phase | Out of Phase 0 scope. |
| Documentation | Stable | Phase 00 | Required source, product, model, ADR, fixture, and planning docs complete. |

---

## 5. Phase Roadmap

| Phase | Name | Status | Health | Exit Condition |
|---:|---|---|---|---|
| 00 | Product and Data Discovery | Blocked | Red | User confirms API application submission; final two UAT cases pass |
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
| BLK-01 | MLIT API application submission is unconfirmed. | Blocks final access gate and UAT closure. | User | Submit application and confirm date; approval may remain pending. |

### Risks

| ID | Risk | Likelihood | Impact | Mitigation |
|---|---|---:|---:|---|
| RSK-01 | Source requires credentials or delayed approval. | Medium | High | Verify and request access as the first discovery action. |
| RSK-02 | Fixture redistribution is restricted. | Medium | Medium | Document terms and use a minimal source-shaped fixture only when permitted. |
| RSK-03 | Source geography or identity is less precise than desired. | High | Medium | Model observations and explicit location precision; narrow map behavior. |
| RSK-04 | Desired filters/metrics are unsupported or categorical. | Medium | High | Narrow the workflow and avoid invented conversions. |

---

## 8. Project-Level Decisions

| Date | Decision | Why It Matters | Reference |
|---|---|---|---|
| 2026-06-24 | Use official public datasets only. | Avoids scraping, licensing uncertainty, and unsupported claims. | `AGENTS.md` |
| 2026-06-24 | Begin with one historical transaction-price source. | Keeps discovery, ingestion, and product claims narrow and testable. | Phase 00 plan |
| 2026-06-24 | Use `transaction_observation`, not a durable `property`, in the initial model. | Stable identity and exact location are not established. | Phase 00 plan |
| 2026-06-24 | Preserve explicit location precision. | Prevents approximate public geography from appearing as exact property coordinates. | Planned ADR-004 |
| 2026-06-24 | Preserve raw source payloads and lineage. | Enables audit, reprocessing, and reproducible metrics. | Planned ADR-003 |
| 2026-06-24 | Use GraphQL for the product API and PostGIS for spatial queries. | Supports bounded map/filter/metric/provenance workflows with database-level geography. | Planned ADR-001/002 |

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
# Inspect the active phase and working tree
git status --short
sed -n '1,240p' .planning/phases/00-product-and-data-discovery/PHASE-STATUS.md

# Locate current planning and documentation artifacts
rg --files .planning docs workers/importer/fixtures 2>/dev/null | sort

# Phase 0 source/fixture validation commands are added after format selection
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

Implemented all repository-controlled Phase 0 outputs: official MLIT fixtures, source/product/model documentation, ADRs 001–005, environment placeholder, integrity checks, and UAT evidence. Six UAT cases pass; two are blocked by the unconfirmed user API application.

### Where Work Stopped

Only MLIT API application submission remains. Approval/key receipt is not required for Phase 0; submission confirmation is.

### First File to Read Next Time

```text
.planning/phases/00-product-and-data-discovery/PHASE-STATUS.md
```

### First Action Next Time

```text
After the user confirms the MLIT application submission date, update docs/data-sources.md to requested—approval pending, rerun UAT-01/UAT-08, close Phase 0, and activate Phase 01.
```
