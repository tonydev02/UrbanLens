# UrbanLens Project State

> Purpose: This is the project-wide resume point. Read it before starting work.

## Project Snapshot

| Field | Value |
|---|---|
| Project | `UrbanLens` |
| Product | Tokyo commercial real-estate intelligence platform using official public data |
| Current Milestone | `MVP local platform foundation` |
| Current Phase | `01 — Local Platform Foundation` |
| Current Phase Status | `in_progress` |
| Project Health | `green` |
| Last Updated | `2026-06-25` |
| Primary Owner | `Project owner` |
| Current Branch | `main` |
| Latest Commit | `169f517` |

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
| Phase Status | `in_progress` |
| Phase Health | `green` |

### Why This Is the Active Phase

Phase 0 passed all eight UAT cases, and Phase 01 now has decision-complete plan/status/UAT documents. The runnable local platform is the next dependency for ingestion, spatial storage, GraphQL product queries, and the analyst workspace.

---

## 3. Exact Next Action

> **Do this first when resuming work:**

```text
Begin Slice 3: add the observable Rust/Actix API foundation, SQLx pool wiring, health/readiness routes, and GraphQL connectivity contracts on top of the migrated PostGIS foundation.
```

### Resume Sequence

1. Read the Phase 01 plan/status and the accepted ADRs.
2. Confirm the branch, working tree, and latest commit.
3. Preserve the completed Slice 1 shared check/environment boundaries.
4. Preserve the completed Slice 2 Compose/PostGIS/migration lifecycle and rerun evidence.
5. Begin Slice 3 API behavior only after confirming the working tree still contains the migration foundation.

---

## 4. Project Workstream Status

| Workstream | State | Active Phase | Notes |
|---|---|---|---|
| Product / Domain | Stable | Phase 00 | Workflow, claims, metrics, precision, and conceptual model complete. |
| Architecture | Stable | Phase 00 | ADRs 001–005 accepted. |
| Backend API | In Progress | Phase 01 | Rust API crate compiles; Actix/GraphQL behavior remains Slice 3. |
| Database / PostGIS | Implemented | Phase 01 | PostGIS service, SQLx migrations, six-table lineage schema, extensions, indexes, and rerun lifecycle are in place. |
| Ingestion Pipeline | Stable | Phase 00 | Three official fixtures and approved API access are available. |
| Frontend Workspace | In Progress | Phase 01 | Next.js scaffold, strict TypeScript, initial test, and production build pass. |
| Testing | In Progress | Phase 01 | Shared Rust/web checks pass; Slice 2 Compose/database smoke evidence exists; API/web/CI checks remain. |
| Infrastructure / CI | In Progress | Phase 01 | Root/infra Compose database lifecycle exists; API/web services and CI remain. |
| Documentation | In Progress | Phase 01 | Environment contract and phase records are current; runtime docs remain. |

---

## 5. Phase Roadmap

| Phase | Name | Status | Health | Exit Condition |
|---:|---|---|---|---|
| 00 | Product and Data Discovery | Completed | Green | First source, fixtures, workflow, model, access, and ADRs passed UAT |
| 01 | Local Platform Foundation | In Progress | Green | Web, API, and PostGIS run locally |
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
| 2026-06-24 | Completed Slice 1: Cargo/pnpm workspaces, pinned tools, lockfiles, API/domain/importer/web scaffolds, environment contract, shared checks, and passing Rust/frontend validation. | 01 | Root manifests, `apps/`, `crates/`, `workers/importer/`, `scripts/` |
| 2026-06-25 | Completed Slice 2 database foundation: root Compose include, `infra/docker-compose.yml`, `postgis/postgis:17-3.5`, named volume, health-gated one-shot SQLx migration service, embedded migration binary, and lineage schema migrations. | 01 | `compose.yaml`, `infra/`, `apps/api/migrations/`, `apps/api/src/bin/migrate.rs` |
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
| 2026-06-25 | Pin the PostGIS Compose service to `linux/amd64`. | Docker reported no native arm64 manifest for `postgis/postgis:17-3.5`; the selected image still runs locally under emulation on Apple Silicon. | `infra/docker-compose.yml` |
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

Completed Phase 01 Slice 2. Root Compose delegates to `infra/docker-compose.yml`; PostGIS 17-3.5 runs with a named volume and health check; the `migrate` service waits for healthy Postgres and runs the embedded SQLx migrations; the schema creates PostGIS/pgcrypto plus the six empty lineage foundation tables.

### Where Work Stopped

Phase 00 is closed. Phase 01 is `in_progress`; Slices 1 and 2 are complete at implementation level. API/web Compose services, health/readiness routes, GraphQL connectivity, and browser-visible `/market-map` proof remain for Slices 3 and 4.

### First File to Read Next Time

```text
.planning/STATE.md
```

### First Action Next Time

```text
Begin Phase 01 Slice 3: implement the observable Actix/GraphQL API with SQLx pool configuration, `/health`, `/readyz`, GraphQL database connectivity, and API container readiness after successful migrations.
```
