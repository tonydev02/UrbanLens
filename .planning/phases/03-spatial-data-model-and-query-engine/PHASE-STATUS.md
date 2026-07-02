# Phase 03 Status — Spatial Data Model and Query Engine

> Purpose: Record the current phase state, progress, blockers, and exact next action.

## Current State

| Field | Value |
|---|---|
| Phase | `03` |
| Name | `Spatial Data Model and Query Engine` |
| Overall Status | `planning` |
| Health | `green` |
| Owner | `Project owner` |
| Started | `2026-07-02` |
| Last Updated | `2026-07-02 12:00 +09:00` |
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

Plan the spatial backend foundation: official Tokyo ward boundaries, area/boundary schema, spatial indexes, database-level viewport and ward filters, bounded GraphQL spatial queries, and explicit location transparency.

## 2. Current Focus

Phase 03 planning is being created from the template and Phase 02 handoff. Implementation has not started.

## 3. Definition of Done

Phase 03 is done when Tokyo ward boundaries load into PostGIS, spatial indexes exist, viewport and ward filtering run in database queries, GraphQL exposes bounded spatial APIs with location precision/disclaimers, and UAT verifies at least one spatial query plus one boundary-based query.

---

## 4. Progress Snapshot

| Area | Status | Progress | Notes |
|---|---|---:|---|
| Planning | In Progress | 80% | Plan/status/UAT created; source selection remains the first implementation slice. |
| Design / Architecture | In Progress | 40% | Spatial scope and slices are drafted; ADR-006 remains a deliverable. |
| Backend | Not Started | 0% | SQLx and GraphQL spatial APIs are planned but not implemented. |
| Database | Not Started | 0% | Area/boundary migrations and indexes are planned but not implemented. |
| Worker / Ingestion | Not Started | 0% | Ward boundary importer is planned but not implemented. |
| Frontend | Not Started | 0% | No Phase 03 UI implementation planned. |
| Tests | Not Started | 0% | Spatial and boundary tests are planned. |
| Documentation | In Progress | 20% | Planning docs created; spatial strategy/source/ADR docs remain. |
| UAT | Not Started | 0% | UAT protocol is drafted but not executable yet. |

---

## 5. Completed Work

Record outcomes, not just activity.

| Date | Completed Outcome | Evidence / Link |
|---|---|---|
| 2026-07-02 | Created Phase 03 planning folder and drafted plan/status/UAT documents from the template, aligned with Phase 02's unknown-location boundary. | `.planning/phases/03-spatial-data-model-and-query-engine/` |

---

## 6. Work In Progress

| Item | Current State | Next Step |
|---|---|---|
| Phase 03 planning | Drafted | Review with owner, then mark ready for implementation. |
| Boundary source selection | Not started | Select and document an official Tokyo ward boundary source. |
| Spatial query strategy | Not started | Draft `docs/spatial-query-strategy.md` during implementation. |
| ADR-006 | Not started | Write after query approach is confirmed. |

---

## 7. Exact Next Actions

Keep this short. The first action must be the exact action to take when work resumes.

1. [ ] **Next immediate action:** Select and document the official Tokyo ward boundary source for Slice 1.
2. [ ] Confirm whether Phase 03 should store boundary raw features in existing lineage tables or document a narrow exception.
3. [ ] Inspect the current `areas` table and draft the additive migration for `areas` / `area_boundaries`.

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
| Official Tokyo ward boundary source | Not selected | Slice 1 onward | Select, document, and fixture before migrations/importer are final. |
| ADRs 001, 002, 003, 004 | Accepted | Phase 03 design | Implement consistently and add ADR-006 only for viewport/proximity specifics. |

---

## 9. Decisions and Plan Changes

### Decisions Made During Planning

| Date | Decision | Rationale | ADR / Reference |
|---|---|---|---|
| 2026-07-02 | Split Phase 03 into six slices: boundary source, schema/indexes, importer, SQLx queries, GraphQL API, docs/UAT. | Each slice teaches one spatial concept and keeps review/learning loops small. | `PHASE-PLAN.md` |
| 2026-07-02 | Keep Phase 02 CSV observations spatially `unknown` unless a defensible geometry source is ingested. | Prevents ward or station context from being misrepresented as exact transaction points. | ADR-004, Phase 02 UAT |
| 2026-07-02 | Treat frontend map rendering as out of scope for Phase 03. | The backend must be location-aware before the analyst workspace renders it. | Phase roadmap |

### Changes From Original Plan

| Date | Original Plan | Change | Reason | Impact |
|---|---|---|---|---|
| — | — | — | — | — |

---

## 10. Validation Status

### Automated Validation

| Check | Latest Result | Evidence |
|---|---|---|
| Rust formatting | Not Run | Planning-only change. |
| Rust lint | Not Run | Planning-only change. |
| Rust tests | Not Run | Planning-only change. |
| TypeScript lint | Not Run | Planning-only change. |
| TypeScript type check | Not Run | Planning-only change. |
| Frontend tests | Not Run | Planning-only change. |
| Integration tests | Not Run | Planning-only change. |
| Docker Compose smoke test | Not Run | Planning-only change. |

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

Phase 03 planning documents were created from the template and aligned with Phase 02 closure.

### Current Working Assumption

UrbanLens will add official Tokyo ward polygons and database-level spatial query behavior without assigning point geometry to the existing MLIT CSV observations. Viewport point queries only return records with defensible non-null geometry; ward-level filtering/aggregation may use official ward identity and must disclose its semantics.

### Important Files

```text
.planning/STATE.md — project-wide resume point and phase roadmap
.planning/phases/03-spatial-data-model-and-query-engine/PHASE-PLAN.md — Phase 03 scope, slices, and acceptance criteria
.planning/phases/02-ingestion-and-canonical-data-pipeline/PHASE-STATUS.md — completed ingestion handoff and unknown-location rule
docs/data-sources.md — selected MLIT source and future boundary source documentation target
docs/data-model.md — area, location precision, idempotency, and physical schema boundaries
docs/product-brief.md — first map, filters, metrics, and precision disclaimers
docs/adr/001-use-postgis-for-spatial-queries.md — accepted PostGIS decision
docs/adr/004-model-location-precision-explicitly.md — accepted precision decision
```

### Recommended Resume Command

```bash
sed -n '1,260p' .planning/phases/03-spatial-data-model-and-query-engine/PHASE-PLAN.md
```

### Exact Next Technical Step

Select and document the official Tokyo ward boundary source, then inspect the current `areas` table before drafting additive area/boundary migrations.

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

