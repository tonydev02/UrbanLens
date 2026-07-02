# UrbanLens Project State

> Purpose: This is the project-wide resume point. Read it before starting work.

## Project Snapshot

| Field | Value |
|---|---|
| Project | `UrbanLens` |
| Product | Tokyo commercial real-estate intelligence platform using official public data |
| Current Milestone | `MVP ingestion foundation` |
| Current Phase | `02 — Ingestion and Canonical Data Pipeline` |
| Current Phase Status | `in_progress` |
| Project Health | `green` |
| Last Updated | `2026-07-02` |
| Primary Owner | `Project owner` |
| Current Branch | `main` |
| Latest Commit | `309928c` |

---

## 1. Current Objective

Implement the first official MLIT fixture import in small slices without
violating Phase 0 source, lineage, metric, or location-precision decisions.
Slices 1, 2, 3, 4, and 5 are complete; the active next step is Phase 02
documentation, regression-check evidence, and UAT readiness.

---

## 2. Active Phase

| Field | Value |
|---|---|
| Active Phase | `02 — Ingestion and Canonical Data Pipeline` |
| Phase Folder | `.planning/phases/02-ingestion-and-canonical-data-pipeline/` |
| Plan | `.planning/phases/02-ingestion-and-canonical-data-pipeline/PHASE-PLAN.md` |
| Status | `.planning/phases/02-ingestion-and-canonical-data-pipeline/PHASE-STATUS.md` |
| UAT | `.planning/phases/02-ingestion-and-canonical-data-pipeline/PHASE-UAT.md` |
| Phase Status | `in_progress` |
| Phase Health | `green` |

### Why This Is the Active Phase

Phase 0 passed all eight UAT cases. Phase 01 is complete: the local platform
runs with Docker Compose, Docker-backed UAT passed, reusable smoke validation
exists, and GitHub Actions checked green per user confirmation on
`2026-06-26`. Phase 02 can now focus on reliable official-source ingestion and
canonical persistence.

---

## 3. Exact Next Action

> **Do this first when resuming work:**

```text
Begin Slice 6 by completing Phase 02 documentation, regression-check evidence, and UAT readiness.
```

### Resume Sequence

1. Read Phase 00 source/product docs, Phase 01 handoff, and accepted ADRs.
2. Confirm the branch, working tree, and latest commit.
3. Read the Phase 02 plan, status, and UAT files.
4. Confirm the smallest useful ingestion scope remains the committed MLIT transaction fixtures.
5. Preserve raw payloads and exact source-artifact lineage.
6. Keep CSV/XIT observations spatially `unknown` unless a defensible source geometry link exists.
7. Preserve the completed Slice 1 parser/normalizer boundary, Slice 2 schema constraints, Slice 3 repository behavior, Slice 4 CLI/script behavior, and Slice 5 GraphQL inspection while preparing Phase 02 UAT evidence.

---

## 4. Project Workstream Status

| Workstream | State | Active Phase | Notes |
|---|---|---|---|
| Product / Domain | Stable | Phase 00 | Workflow, claims, metrics, precision, and conceptual model complete. |
| Architecture | Stable | Phase 00 | ADRs 001–005 accepted. |
| Backend API | Implemented | Phase 01 | Actix API, SQLx pool, `/health`, `/ready`, GraphQL `connectivity`, request IDs, bounded CORS, and API image healthcheck are in place. |
| Database / PostGIS | Implemented | Phase 01 | PostGIS service, SQLx migrations, six-table lineage schema, extensions, indexes, and rerun lifecycle are in place. |
| Ingestion Pipeline | In Progress | Phase 02 | Slice 1 parser/normalizer, Slice 2 schema/database contracts, Slice 3 persistence repositories, Slice 4 CLI/script, and Slice 5 GraphQL inspection are complete; Slice 6 UAT/docs closure is next. |
| Frontend Workspace | Implemented | Phase 01 | Next.js analyst shell, `/market-map`, root redirect, loading/error/not-found states, and browser-visible GraphQL connectivity panel are implemented and tested. |
| Testing | Complete | Phase 01 | Rust/web/build checks, fresh/existing-volume Compose smoke, failure/recovery UAT, secret checks, and GitHub Actions pass. |
| Infrastructure / CI | Complete | Phase 01 | Root/infra Compose starts PostGIS, migrate, API, and web; GitHub Actions workflow and reusable smoke script are green. |
| Documentation | Complete | Phase 01 | README, architecture, local development, environment contract, smoke scripts, CI, and optional MLIT diagnostic docs are current. |

---

## 5. Phase Roadmap

| Phase | Name | Status | Health | Exit Condition |
|---:|---|---|---|---|
| 00 | Product and Data Discovery | Completed | Green | First source, fixtures, workflow, model, access, and ADRs passed UAT |
| 01 | Local Platform Foundation | Completed | Green | Web, API, and PostGIS run locally; CI checks are green |
| 02 | Ingestion and Canonical Data Pipeline | Ready for Implementation | Green | Official data imports safely and repeatedly |
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
| 2026-07-02 | Completed Phase 02 Slice 5: bounded GraphQL inspection for imported observations, import runs, validation issues, data sources, and provenance summaries; verified with Docker-backed Rust checks, isolated Compose smoke, fixture import, and live GraphQL queries. | 02 | `apps/api/src/lib.rs`, `apps/api/Cargo.toml`, `docs/importer.md`, `PHASE-STATUS.md` |
| 2026-06-29 | Completed Phase 02 Slice 4: `import-transactions` CLI, Docker-backed `scripts/import-fixture.sh`, first official fixture import, duplicate-safe rerun, CLI tests, and docs/planning sync. | 02 | `workers/importer/src/main.rs`, `scripts/import-fixture.sh`, `docs/importer.md`, `PHASE-STATUS.md` |
| 2026-06-29 | Completed Phase 02 Slice 3: source/dataset upsert keys, persistence repositories, raw-record duplicate skipping, observation/location writes, validation issue storage, counters, failed-run visibility, DB-backed repository tests, and fresh-volume Compose smoke. | 02 | `apps/api/migrations/202606290002_add_lineage_upsert_keys.sql`, `workers/importer/src/persistence.rs`, `docs/importer.md`, `PHASE-STATUS.md` |
| 2026-06-29 | Completed Phase 02 Slice 2: canonical transaction/location schema, observation-level validation issue link, lineage/idempotency constraints, precision/geometry constraints, and fresh/existing-volume Compose schema contract validation. | 02 | `apps/api/migrations/202606290001_create_transaction_observation_schema.sql`, `scripts/smoke-compose.sh`, `docs/data-model.md`, `PHASE-STATUS.md` |
| 2026-06-29 | Completed Phase 02 Slice 1: CP932 MLIT CSV parser, source-row/raw-value preservation, normalization structs, validation issue codes, and six importer tests over all 666 committed fixture rows and edge cases. | 02 | `workers/importer/src/mlit.rs`, `docs/importer.md`, `PHASE-STATUS.md` |
| 2026-06-27 | Created Phase 02 plan/status/UAT documents with six small implementation slices for the MLIT fixture importer. | 02 | `.planning/phases/02-ingestion-and-canonical-data-pipeline/` |
| 2026-06-26 | User confirmed GitHub Actions checked green; Phase 01 closed and project resume point moved to Phase 02. | 01 | `.github/workflows/ci.yml`, `.planning/phases/01-local-platform-foundation/`, `.planning/STATE.md` |
| 2026-06-26 | Completed Slice 5: GitHub Actions workflow, reusable Compose smoke, optional MLIT XIT001 diagnostic, fresh/existing-volume Docker UAT, failure/recovery checks, Rust/web/build checks, and documentation updates. | 01 | `.github/workflows/ci.yml`, `scripts/smoke-compose.sh`, `scripts/smoke-mlit-api.sh`, `PHASE-UAT.md` |
| 2026-06-26 | Completed Slice 4 implementation: Next.js analyst shell, `/market-map`, root redirect, QueryClient/GraphQL browser connectivity panel, loading/error/retry/not-found states, web Compose service, and README/architecture/local-development updates. | 01 | `apps/web/src/app/`, `apps/web/src/components/`, `apps/web/src/lib/`, `infra/web.Dockerfile`, `infra/docker-compose.yml`, `docs/`, `README.md` |
| 2026-06-25 | Completed Slice 3 API foundation: Actix server, API config, SQLx pool, bounded CORS, generated/preserved request IDs, `/health`, `/ready`, GraphQL `connectivity`, API service after migration success, and container healthcheck. | 01 | `apps/api/src/lib.rs`, `apps/api/src/main.rs`, `apps/api/src/bin/healthcheck.rs`, `infra/docker-compose.yml`, `infra/api.Dockerfile`, `docs/local-development.md` |
| 2026-06-25 | Completed Slice 2 database foundation: root Compose include, `infra/docker-compose.yml`, `postgis/postgis:17-3.5`, named volume, health-gated one-shot SQLx migration service, embedded migration binary, and lineage schema migrations. | 01 | `compose.yaml`, `infra/`, `apps/api/migrations/`, `apps/api/src/bin/migrate.rs` |
| 2026-06-24 | Completed Slice 1: Cargo/pnpm workspaces, pinned tools, lockfiles, API/domain/importer/web scaffolds, environment contract, shared checks, and passing Rust/frontend validation. | 01 | Root manifests, `apps/`, `crates/`, `workers/importer/`, `scripts/` |
| 2026-06-24 | Confirmed MLIT API approval/local ignored `.env`; passed UAT-01/UAT-08 and completed Phase 0. | 00 | `docs/data-sources.md`, Phase 00 UAT |
| 2026-06-24 | Selected MLIT transaction-price information and completed source/access/schema/limitation documentation. | 00 | `docs/data-sources.md` |
| 2026-06-24 | Retrieved, profiled, and checksum-validated 666 official source observations across three Tokyo wards. | 00 | `workers/importer/fixtures/transactions/` |
| 2026-06-24 | Completed product brief, conceptual model, and accepted ADRs 001–005. | 00 | `docs/` |

---

## 7. Active Blockers and Risks

### Blockers

| ID | Blocker | Impact | Owner | Next Action |
|---|---|---|---|---|
| — | No active blocker. | — | — | — |

### Risks

| ID | Risk | Likelihood | Impact | Mitigation |
|---|---|---:|---:|---|
| RSK-01 | Source revisions can change historical query results. | Medium | Medium | Version each retrieval by query, timestamp, and artifact checksum. |
| RSK-02 | XPT station points cannot be guessed onto CSV/XIT rows. | High | High | Treat XPT features as their own source records or keep other observations spatially unknown. |
| RSK-03 | Optional authenticated API diagnostics could disclose a local key if implemented carelessly. | Low | High | Keep the check out of CI, disable tracing/header output, and never persist the response. |
| RSK-04 | Ingestion may accidentally invent defaults for bad source values. | Medium | High | Prefer `null + validation warning` and preserve raw payloads. |

---

## 8. Project-Level Decisions

| Date | Decision | Why It Matters | Reference |
|---|---|---|---|
| 2026-06-26 | Keep the MLIT diagnostic local-only and keep CI secret-free. | The diagnostic validates developer-local access without coupling startup or CI to a private credential or external availability. | `scripts/smoke-mlit-api.sh`, `.github/workflows/ci.yml` |
| 2026-06-25 | Pin the PostGIS Compose service to `linux/amd64`. | Docker reported no native arm64 manifest for `postgis/postgis:17-3.5`; the selected image still runs locally under emulation on Apple Silicon. | `infra/docker-compose.yml` |
| 2026-06-25 | Use `/ready` and GraphQL `connectivity` as the initial API readiness/connectivity contract. | Keeps Phase 01 focused on service/database/migration state before product data exists. | `apps/api/src/lib.rs`, `docs/local-development.md` |
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
- Do not advance a phase before it passes UAT.

---

## 10. Resume Commands

```bash
# Inspect project state and working tree
git status --short
sed -n '1,260p' .planning/STATE.md

# Read the completed Phase 01 handoff and source/domain decisions
sed -n '1,260p' .planning/phases/01-local-platform-foundation/PHASE-STATUS.md
sed -n '1,220p' docs/data-sources.md
sed -n '1,240p' docs/data-model.md
```

---

## 11. Update Rules

Update this file when:

- The selected source or credential status changes.
- The exact next action changes.
- A blocker appears, changes, or is resolved.
- A major project-level decision is made.
- A phase reaches `ready_for_uat`, `completed`, or `completed_with_exceptions`.
- The active phase changes.

Detailed discovery notes and progress belong in the active phase documents and
`docs/` deliverables.

---

## 12. Last Session Handoff

### Last Session Summary

Closed Phase 01. The local platform starts through Docker Compose, exposes
health/readiness/GraphQL connectivity, serves the Next.js analyst shell, applies
the foundation PostGIS/lineage migrations, has reusable smoke validation, and
has a green GitHub Actions workflow. Disposable runtime artifacts were cleaned
from the local MacBook environment while keeping source files and installed
dependencies useful for continued development.

### Where Work Stopped

Phase 02 is the active next phase, but its planning files still need to be
created from the templates.

### First File to Read Next Time

```text
.planning/STATE.md
```

### First Action Next Time

```text
Create Phase 02 plan/status/UAT files from the template, then design the MLIT fixture importer around the committed official fixtures and the Phase 01 lineage schema.
```
