# UrbanLens Project State

> Purpose: This is the project-wide resume point. Read it before starting work.

## Project Snapshot

| Field | Value |
|---|---|
| Project | `UrbanLens` |
| Product | Tokyo commercial real-estate intelligence platform using official public data |
| Current Milestone | `MVP local platform foundation` |
| Current Phase | `01 — Local Platform Foundation` |
| Current Phase Status | `ready_for_implementation` |
| Project Health | `green` |
| Last Updated | `2026-06-24` |
| Primary Owner | `Project owner` |
| Current Branch | `main` |
| Latest Commit | `8b66ed9` |

---

## 1. Current Objective

Build the decision-complete Phase 01 local platform foundation so one root Docker Compose command starts a migrated PostGIS database, observable Rust/Actix GraphQL API, and Next.js analyst shell without violating Phase 0 source, lineage, metric, or location-precision decisions.

---

## 2. Active Phase

| Field | Value |
|---|---|
| Active Phase | `01 — Local Platform Foundation` |
| Phase Folder | `.planning/phases/01-local-platform-foundation/` |
| Plan | `.planning/phases/01-local-platform-foundation/PHASE-PLAN.md` |
| Status | `.planning/phases/01-local-platform-foundation/PHASE-STATUS.md` |
| UAT | `.planning/phases/01-local-platform-foundation/PHASE-UAT.md` |
| Phase Status | `ready_for_implementation` |
| Phase Health | `green` |

### Why This Is the Active Phase

Phase 0 passed all eight UAT cases, and Phase 01 now has decision-complete plan/status/UAT documents. The runnable local platform is the next dependency for ingestion, spatial storage, GraphQL product queries, and the analyst workspace.

---

## 3. Exact Next Action

> **Do this first when resuming work:**

```text
Add root Cargo/pnpm workspace manifests, pinned toolchain/version files, package/crate skeletons, and the shared safe environment contract. Then create the root/infra Compose entrypoint, PostGIS health check, and dedicated migration binary.
```

### Resume Sequence

1. Read the Phase 01 plan/status and the accepted ADRs.
2. Confirm the branch, working tree, and latest commit.
3. Implement Slice 1: root workspaces, pinned toolchains, crate/package skeletons, lockfiles, and environment contract.
4. Implement the root/infra Compose entrypoint, PostGIS service, migrations, and one-shot migration binary.
5. Validate the fresh-volume migration path before beginning API or frontend behavior.

---

## 4. Project Workstream Status

| Workstream | State | Active Phase | Notes |
|---|---|---|---|
| Product / Domain | Stable | Phase 00 | Workflow, claims, metrics, precision, and conceptual model complete. |
| Architecture | Stable | Phase 00 | ADRs 001–005 accepted. |
| Backend API | Ready | Phase 01 | Actix/GraphQL contracts, failure behavior, logging, and tests are planned. |
| Database / PostGIS | Ready | Phase 01 | Six-table lineage schema, extensions, indexes, and migration lifecycle are planned. |
| Ingestion Pipeline | Stable | Phase 00 | Three official fixtures and approved API access are available. |
| Frontend Workspace | Ready | Phase 01 | App shell, empty market-map route, connectivity, and failure states are planned. |
| Testing | Ready | Phase 01 | Eight required Phase 01 UAT cases and automated coverage are specified. |
| Infrastructure / CI | Ready | Phase 01 | Compose lifecycle and Rust/frontend/smoke CI jobs are specified. |
| Documentation | Ready | Phase 01 | README, architecture, local-development, and environment updates are required. |

---

## 5. Phase Roadmap

| Phase | Name | Status | Health | Exit Condition |
|---:|---|---|---|---|
| 00 | Product and Data Discovery | Completed | Green | First source, fixtures, workflow, model, access, and ADRs passed UAT |
| 01 | Local Platform Foundation | Ready for Implementation | Green | Web, API, and PostGIS run locally |
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
| 2026-06-24 | Completed decision-ready Phase 01 plan, status, and UAT documents with interfaces, schema constraints, implementation slices, CI, and failure cases. | 01 | `.planning/phases/01-local-platform-foundation/` |
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
| RSK-01 | Browser and container API URLs/origins may diverge. | Medium | High | Use explicit browser/service URLs and cover browser-visible connectivity in smoke/UAT. |
| RSK-02 | Source revisions can change historical query results. | Medium | Medium | Version each retrieval by query, timestamp, and artifact checksum. |
| RSK-03 | XPT station points cannot be guessed onto CSV/XIT rows. | High | High | Treat XPT features as their own source records or keep other observations spatially unknown. |
| RSK-04 | Optional authenticated API diagnostics could disclose a local key if implemented carelessly. | Low | High | Keep the check out of CI, disable tracing/header output, and never persist the response. |

---

## 8. Project-Level Decisions

| Date | Decision | Why It Matters | Reference |
|---|---|---|---|
| 2026-06-24 | Include `datasets` in the Phase 01 physical foundation. | Preserves exact-artifact lineage between source and import run. | Phase 01 plan, ADR-003 |
| 2026-06-24 | Run migrations through a one-shot Compose service. | Makes migration order and failure explicit while retaining one-command startup. | Phase 01 plan |
| 2026-06-24 | Keep authenticated MLIT connectivity optional in Phase 01. | Core startup and CI remain secret-free and independent of external availability. | Phase 01 plan |
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
# Inspect active phase and the working tree
git status --short
sed -n '1,260p' .planning/phases/01-local-platform-foundation/PHASE-STATUS.md

# Read the implementation design and UAT contract
sed -n '1,320p' .planning/phases/01-local-platform-foundation/PHASE-PLAN.md
sed -n '1,260p' .planning/phases/01-local-platform-foundation/PHASE-UAT.md
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

Completed the Phase 01 planning package. The implementation is split into workspace/tooling, PostGIS/migrations, API, frontend, and CI/documentation slices. Public interfaces, six-table lineage schema, one-shot migration lifecycle, failure behavior, tests, and eight required UAT cases are decision-complete.

### Where Work Stopped

Phase 00 is closed. Phase 01 is `ready_for_implementation`; no application or infrastructure code has been added yet.

### First File to Read Next Time

```text
.planning/STATE.md
```

### First Action Next Time

```text
Implement Phase 01 Slice 1: add root Cargo/pnpm workspaces, pinned toolchains, package/crate skeletons, lockfiles, and the safe environment contract; then begin the Compose/PostGIS/migration foundation.
```
