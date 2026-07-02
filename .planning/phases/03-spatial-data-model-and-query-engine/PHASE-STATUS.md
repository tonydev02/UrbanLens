# Phase 03 Status — Spatial Data Model and Query Engine

> Purpose: Record the current phase state, progress, blockers, and exact next action.

## Current State

| Field | Value |
|---|---|
| Phase | `03` |
| Name | `Spatial Data Model and Query Engine` |
| Overall Status | `in_progress` |
| Health | `green` |
| Owner | `Project owner` |
| Started | `2026-07-02` |
| Last Updated | `2026-07-02 20:35 +09:00` |
| Target Completion | `TBD` |
| Current Branch | `main` |
| Current Commit | `5ac32c4` |
| Related Plan | `PHASE-PLAN.md` |
| Related UAT | `PHASE-UAT.md` |

### Allowed Status Values

```text
not_started
planning
ready_for_implementation
in_progress
blocked
ready_for_uat
uat_in_progress
completed
completed_with_exceptions
cancelled
```

### Allowed Health Values

```text
green
yellow
red
```

- `green`: Work is progressing normally.
- `yellow`: There is meaningful risk, uncertainty, or dependency.
- `red`: Work is blocked or the plan needs material revision.

---

## 1. Current Objective

Implement the spatial backend foundation: official Tokyo ward boundaries, area/boundary schema, spatial indexes, database-level viewport and ward filters, bounded GraphQL spatial queries, and explicit location transparency.

## 2. Current Focus

Slices 1, 2, and 3 are complete. The official Tokyo ward boundary source is selected and documented, a small source-derived 23-ward fixture is committed, the area/boundary schema plus spatial/filter indexes are implemented and smoke-verified, and the repeat-safe ward boundary importer now persists 118 raw source features into 23 governed ward boundaries. The active next step is Slice 4 SQLx spatial query functions.

## 3. Definition of Done

Phase 03 is done when Tokyo ward boundaries load into PostGIS, spatial indexes exist, viewport and ward filtering run in database queries, GraphQL exposes bounded spatial APIs with location precision/disclaimers, and UAT verifies at least one spatial query plus one boundary-based query.

---

## 4. Progress Snapshot

| Area | Status | Progress | Notes |
|---|---|---:|---|
| Planning | Done | 100% | Plan/status/UAT created and Slice 1 source decision completed. |
| Design / Architecture | In Progress | 60% | Boundary source, lineage decision, and area/boundary database contracts are documented; ADR-006 remains a deliverable. |
| Backend | Not Started | 0% | SQLx and GraphQL spatial APIs are planned but not implemented. |
| Database | Done for Slice 3 | 40% | Area/boundary schema, aggregate boundary lineage migration, geometry constraints, lineage constraints, and spatial/filter indexes are implemented and smoke-verified. |
| Worker / Ingestion | Done for Slice 3 | 45% | Boundary fixture, validator, parser, CLI, script, raw-feature persistence, ward area/boundary upserts, and rerun behavior are implemented and verified. |
| Frontend | Not Started | 0% | No Phase 03 UI implementation planned. |
| Tests | In Progress | 30% | Compose smoke asserts area/boundary schema, spatial indexes, invalid geometry rejection, and aggregate boundary lineage; importer tests cover parser, hash stability, geometry rejection, and duplicate rerun behavior. |
| Documentation | In Progress | 55% | Boundary source, fixture, lineage decision, Slice 2 physical schema, and Slice 3 importer behavior are documented; spatial strategy/ADR docs remain. |
| UAT | Not Started | 0% | UAT protocol is drafted but not executable yet. |

---

## 5. Completed Work

Record outcomes, not just activity.

| Date | Completed Outcome | Evidence / Link |
|---|---|---|
| 2026-07-02 | Created Phase 03 planning folder and drafted plan/status/UAT documents from the template, aligned with Phase 02's unknown-location boundary. | `.planning/phases/03-spatial-data-model-and-query-engine/` |
| 2026-07-02 | Completed Slice 1: selected MLIT N03 administrative-area data as the official Tokyo ward boundary source, committed a 23-ward source-derived GeoJSON fixture, added checksum/coverage validation, and documented boundary lineage/limitations. | `docs/data-sources.md`, `workers/importer/fixtures/boundaries/`, `scripts/validate-boundary-fixture.sh` |
| 2026-07-02 | Completed Slice 2: added governed area identity fields, versioned `area_boundaries`, geometry/lineage constraints, GiST spatial indexes, transaction filter indexes, and smoke assertions for schema/index/invalid geometry behavior. Docker-backed smoke passed with isolated Compose ports. | `apps/api/migrations/202607020001_add_area_boundaries_spatial_indexes.sql`, `scripts/smoke-compose.sh`, `docs/data-model.md` |
| 2026-07-02 | Completed Slice 3: added MLIT N03 boundary parser/importer, Docker-backed import script, aggregate boundary lineage migration, raw-feature persistence, repeat-safe `areas` / `area_boundaries` upserts, importer tests, and isolated first-run/rerun evidence. | `workers/importer/src/boundaries.rs`, `workers/importer/src/persistence.rs`, `workers/importer/src/main.rs`, `scripts/import-boundaries.sh`, `apps/api/migrations/202607020002_allow_aggregate_boundary_import_lineage.sql`, `docs/importer.md` |

---

## 6. Work In Progress

| Item | Current State | Next Step |
|---|---|---|
| Phase 03 planning | Complete | Continue through Slice 4 implementation. |
| Boundary source selection | Complete | Use MLIT N03 fixture as the query-test target. |
| Area schema and spatial indexes | Complete | Use these contracts as the target for Slice 4 SQLx query functions. |
| Ward boundary importer | Complete | Use imported ward boundaries for Slice 4 database-level spatial queries. |
| Spatial query strategy | Not started | Draft `docs/spatial-query-strategy.md` during implementation. |
| ADR-006 | Not started | Write after query approach is confirmed. |

---

## 7. Exact Next Actions

Keep this short. The first action must be the exact action to take when work resumes.

1. [ ] **Next immediate action:** Implement Slice 4 SQLx spatial query functions for ward filtering, viewport filtering, proximity, counts by ward, and basic area metrics.
2. [x] Select and document the official Tokyo ward boundary source for Slice 1.
3. [x] Confirm whether Phase 03 should store boundary raw features in existing lineage tables or document a narrow exception.

---

## 8. Blockers, Risks, and Dependencies

### Blockers

| ID | Blocker | Impact | Owner | Since | Next Action |
|---|---|---|---|---|---|
| — | No active blocker. | — | — | — | — |

### Risks

| ID | Risk | Likelihood | Impact | Mitigation | Status |
|---|---|---:|---:|---|---|
| RSK-01 | Boundary source licensing, CRS, or schema details are selected too casually. | Medium | High | Make source selection/documentation the first slice and require checksum/CRS evidence. | Open |
| RSK-02 | Existing CSV observations could be plotted or filtered as if they had point geometry. | Medium | High | Preserve `unknown` precision/null geometry and test that unknown observations are absent from point-map results. | Open |
| RSK-03 | Spatial filtering could drift into application memory. | Medium | High | Require SQLx/PostGIS query tests and document query strategy. | Open |
| RSK-04 | Area counts by ward could mix source ward-code filtering and polygon containment without clear semantics. | Medium | Medium | Document the chosen method per query and expose limitations. | Open |
| RSK-05 | Host Cargo is unavailable on the main environment. | High | Low | Use `corepack pnpm check` or Docker-backed Rust checks. | Open |

### Dependencies

| Dependency | Status | Required By | Next Action |
|---|---|---|---|
| Phase 02 importer and canonical observations | Complete | Spatial query filters and regression checks | Preserve existing import behavior and unknown-location semantics. |
| PostgreSQL/PostGIS Compose foundation | Complete | All spatial schema/query work | Reuse existing Compose smoke pattern. |
| Official Tokyo ward boundary source | Selected | Slice 2 onward | Use the committed MLIT N03 fixture as the schema/importer target. |
| ADRs 001, 002, 003, 004 | Accepted | Phase 03 design | Implement consistently and add ADR-006 only for viewport/proximity specifics. |

---

## 9. Decisions and Plan Changes

### Decisions Made During Planning

| Date | Decision | Rationale | ADR / Reference |
|---|---|---|---|
| 2026-07-02 | Split Phase 03 into six slices: boundary source, schema/indexes, importer, SQLx queries, GraphQL API, docs/UAT. | Each slice teaches one spatial concept and keeps review/learning loops small. | `PHASE-PLAN.md` |
| 2026-07-02 | Keep Phase 02 CSV observations spatially `unknown` unless a defensible geometry source is ingested. | Prevents ward or station context from being misrepresented as exact transaction points. | ADR-004, Phase 02 UAT |
| 2026-07-02 | Treat frontend map rendering as out of scope for Phase 03. | The backend must be location-aware before the analyst workspace renders it. | Phase roadmap |
| 2026-07-02 | Select MLIT National Land Numerical Information `N03` administrative-area data as the Phase 03 Tokyo ward boundary source. | It is official, covers Tokyo wards, exposes administrative codes aligned with MLIT transaction municipality codes, and has a small source-derived fixture path. | `docs/data-sources.md`, fixture README |
| 2026-07-02 | Store boundary raw features through existing lineage tables in Phase 03. | Area polygons are official source records and must remain traceable to exact source artifacts/features, matching ADR-003. | `docs/data-model.md`, fixture README |
| 2026-07-02 | Allow dissolved ward boundaries to keep import-run lineage without a single raw-record reference. | One ward boundary is assembled from multiple official source polygon features; raw features remain preserved individually in `raw_records`. | `apps/api/migrations/202607020002_allow_aggregate_boundary_import_lineage.sql`, `docs/data-model.md` |

### Changes From Original Plan

| Date | Original Plan | Change | Reason | Impact |
|---|---|---|---|---|
| — | — | — | — | — |

---

## 10. Validation Status

### Automated Validation

| Check | Latest Result | Evidence |
|---|---|---|
| Rust formatting | Pass | `corepack pnpm check` passed on `2026-07-02`, using Docker-backed Rust because host Cargo is unavailable. |
| Rust lint | Pass | `corepack pnpm check` passed on `2026-07-02`. |
| Rust tests | Pass | `corepack pnpm check` passed on `2026-07-02`; importer tests include boundary parser, hash stability, invalid geometry, and DB-backed duplicate rerun behavior. |
| TypeScript lint | Pass | `corepack pnpm check` passed on `2026-07-02`. |
| TypeScript type check | Pass | `corepack pnpm check` passed on `2026-07-02`. |
| Frontend tests | Pass | `corepack pnpm check` passed on `2026-07-02`; 5 files / 8 tests passed. |
| Integration tests | Not Run | Planning-only change. |
| Boundary fixture validation | Pass | `bash scripts/validate-boundary-fixture.sh` verifies SHA-256, 118 source polygons, 23 Tokyo special-ward codes, polygon geometry, and coordinate bounds. |
| Smoke script syntax | Pass | `bash -n scripts/smoke-compose.sh` passed on `2026-07-02`. |
| Docker Compose smoke test | Pass | `PATH="/usr/local/bin:$PATH" COMPOSE_PROJECT_NAME=urbanlens_slice3_smoke API_PORT=18083 WEB_PORT=13083 POSTGRES_PORT=15435 bash scripts/smoke-compose.sh` passed on `2026-07-02`, verifying six migrations, area/boundary tables, GiST/filter indexes, aggregate boundary lineage constraint, and invalid boundary SRID/type rejection. |
| Boundary importer first run | Pass | `IMPORTER_DOCKER_NETWORK=urbanlens_slice3_smoke_default ./scripts/import-boundaries.sh` imported 118 source features into 23 ward boundaries with `status=completed`, `imported=23`, `duplicates_skipped=0`. |
| Boundary importer rerun | Pass | Second `./scripts/import-boundaries.sh` skipped 118 duplicate raw features and updated the same 23 boundary rows with no duplicate areas or boundary rows. |
| Boundary database count check | Pass | Isolated DB query returned `areas=23`, `boundaries=23`, `raw_records=118`, `ward_codes=23`, `valid_multipolygons=23`, `transaction_locations=0`. |

### UAT Status

| Field | Value |
|---|---|
| UAT Readiness | `not_ready` |
| UAT Result | `not_started` |
| Blocking Defects | `0` |
| UAT File | `PHASE-UAT.md` |

---

## 11. Resume Context

### Last Meaningful Change

Slice 3 added and verified the repeat-safe MLIT N03 ward boundary importer.

### Current Working Assumption

UrbanLens has imported official Tokyo ward polygons from the MLIT N03 fixture into `areas` and `area_boundaries` without assigning point geometry to existing MLIT CSV observations. Viewport point queries should only return records with defensible non-null geometry; ward-level filtering/aggregation may use official ward identity and must disclose its semantics.

### Important Files

```text
.planning/STATE.md — project-wide resume point and phase roadmap
.planning/phases/03-spatial-data-model-and-query-engine/PHASE-PLAN.md — Phase 03 scope, slices, and acceptance criteria
.planning/phases/02-ingestion-and-canonical-data-pipeline/PHASE-STATUS.md — completed ingestion handoff and unknown-location rule
docs/data-sources.md — selected MLIT source and future boundary source documentation target
docs/data-model.md — area, boundary, location precision, idempotency, and physical schema boundaries
docs/product-brief.md — first map, filters, metrics, and precision disclaimers
docs/adr/001-use-postgis-for-spatial-queries.md — accepted PostGIS decision
docs/adr/004-model-location-precision-explicitly.md — accepted precision decision
```

### Recommended Resume Command

```bash
sed -n '1,260p' .planning/phases/03-spatial-data-model-and-query-engine/PHASE-PLAN.md
```

### Exact Next Technical Step

Implement Slice 4 SQLx spatial query functions against the imported ward boundaries and canonical transaction observations.

---

## 12. Exit Checklist

- [ ] All in-scope phase requirements are complete.
- [ ] Required automated tests pass.
- [ ] Required documentation is complete.
- [ ] UAT cases have been executed.
- [ ] UAT result is `passed` or `passed_with_accepted_exceptions`.
- [ ] No critical or high defects remain open.
- [ ] Handoff notes are completed in `PHASE-PLAN.md`.
- [ ] `.planning/STATE.md` is updated.
- [ ] Overall status is `completed` or `completed_with_exceptions`.

---

## 13. Update Log

Append one concise row whenever the phase changes meaningfully.

| Timestamp | Status | Update |
|---|---|---|
| 2026-07-02 12:00 +09:00 | `planning` | Phase 03 plan/status/UAT documents created from template. |
| 2026-07-02 12:05 +09:00 | `in_progress` | Slice 1 completed: MLIT N03 boundary source selected, documented, fixture committed, and validator added. |
| 2026-07-02 13:20 +09:00 | `in_progress` | Slice 2 implementation complete: area/boundary schema, spatial indexes, filter indexes, smoke schema assertions, and docs sync added; Docker-backed validation pending because Docker is unavailable in this shell. |
| 2026-07-02 20:09 +09:00 | `in_progress` | Slice 2 validation passed after rerunning Compose smoke with `/usr/local/bin` on PATH; focus moves to Slice 3 ward boundary importer. |
| 2026-07-02 20:35 +09:00 | `in_progress` | Slice 3 complete: boundary parser/importer/script, aggregate lineage migration, tests, docs sync, full check, isolated Compose smoke, first import, rerun, and DB count validation passed; focus moves to Slice 4 SQLx spatial query functions. |
