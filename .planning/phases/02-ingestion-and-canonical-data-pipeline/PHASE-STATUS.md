# Phase 02 Status — Ingestion and Canonical Data Pipeline

> Purpose: Record the current phase state, progress, blockers, and exact next action.

## Current State

| Field | Value |
|---|---|
| Phase | `02` |
| Name | `Ingestion and Canonical Data Pipeline` |
| Overall Status | `ready_for_implementation` |
| Health | `green` |
| Owner | `Project owner` |
| Started | `2026-06-27` |
| Last Updated | `2026-06-27 13:00 +07:00` |
| Target Completion | `TBD` |
| Current Branch | `main` |
| Current Commit | `f72e09e` |
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

Build the first real ingestion path for the MLIT transaction fixture: preserve raw records and lineage, validate source rows, normalize useful transaction fields, persist canonical observations, and prove repeat imports are idempotent.

## 2. Current Focus

Phase planning is complete and aligned with Phase 00/01 decisions. Implementation has not started; the next work is Slice 1 parser and normalization tests.

## 3. Definition of Done

Phase 2 is done when `./scripts/import-fixture.sh` imports the committed MLIT fixture, GraphQL can inspect imported observations and provenance, raw records and validation issues are preserved, reruns create no unintended duplicates, failed imports are visible, and tests cover parsing, validation, normalization, counters, idempotency, and retry behavior.

---

## 4. Progress Snapshot

| Area | Status | Progress | Notes |
|---|---|---:|---|
| Planning | Done | 100% | Phase 2 plan/status/UAT are created from the template and ready for implementation. |
| Design / Architecture | In Progress | 60% | Slice boundaries and data-flow design are drafted; schema details remain for Slice 2. |
| Backend | Not Started | 0% | GraphQL inspection queries are planned but not implemented. |
| Database | Not Started | 0% | Canonical transaction/location migrations are planned but not implemented. |
| Worker / Ingestion | Not Started | 0% | Compile-only importer exists from Phase 01. |
| Frontend | Not Started | 0% | No Phase 2 product UI planned beyond existing shell. |
| Tests | Not Started | 0% | Parser, persistence, importer, and GraphQL tests are planned. |
| Documentation | In Progress | 20% | Planning docs created; `docs/importer.md` is a later deliverable. |
| UAT | Not Started | 0% | UAT protocol drafted; no cases run. |

---

## 5. Completed Work

Record outcomes, not just activity.

| Date | Completed Outcome | Evidence / Link |
|---|---|---|
| 2026-06-27 | Created the Phase 2 planning folder and drafted plan/status/UAT documents from the template, aligned with Phase 00 source/model decisions and Phase 01 handoff. | `.planning/phases/02-ingestion-and-canonical-data-pipeline/` |

---

## 6. Work In Progress

| Item | Current State | Next Step |
|---|---|---|
| Phase 2 planning | Complete | Use the plan as the implementation guide, updating it if Slice 1 reveals new source constraints. |
| Importer package command naming | Open question | Decide whether to keep `urbanlens-importer` or rename/alias to `importer`. |
| Fixture path deliverable | Open question | Decide whether `fixtures/mlit/` becomes a wrapper/copy path or whether existing importer fixtures remain canonical. |

---

## 7. Exact Next Actions

Keep this short. The first action must be the exact action to take when work resumes.

1. [ ] **Next immediate action:** Begin Slice 1 by adding pure MLIT CSV parsing, source-row structs, normalization structs, validation issue codes, and fixture tests.
2. [ ] Begin Slice 1 by adding pure MLIT CSV parsing, source-row structs, normalization structs, validation issue codes, and fixture tests.
3. [ ] Decide and document importer package naming before publishing the first command examples.

---

## 8. Blockers, Risks, and Dependencies

### Blockers

| ID | Blocker | Impact | Owner | Since | Next Action |
|---|---|---|---|---|---|
| — | No active blocker. | — | — | — | — |

### Risks

| ID | Risk | Likelihood | Impact | Mitigation | Status |
|---|---|---:|---:|---|---|
| RSK-01 | CSV rows could be accidentally assigned map geometry or exact-property meaning. | Medium | High | Require `location_precision=unknown` for CSV fixture observations and test geometry/precision constraints. | Open |
| RSK-02 | Source unit price could be confused with derived price per square metre. | Medium | High | Preserve only MLIT-supplied unit price in the canonical source field; name any derived helper explicitly if added later. | Open |
| RSK-03 | Import idempotency could collapse legitimate identical rows. | Medium | High | Use exact artifact plus source row position for raw identity; keep payload hash as evidence, not sole identity. | Open |
| RSK-04 | Importer may overgrow into live API ingestion before fixture behavior is reliable. | Medium | Medium | Keep live MLIT API out of required Phase 2 path and close fixture import first. | Open |
| RSK-05 | Host Cargo is unavailable on the main environment. | High | Low | Use `corepack pnpm check` or Docker-backed Rust checks per project context. | Open |

### Dependencies

| Dependency | Status | Required By | Next Action |
|---|---|---|---|
| Phase 01 local platform | Complete | All Phase 2 slices | Reuse Compose, migrations, API, and checks. |
| MLIT fixture CSVs | Available | Slice 1 onward | Use `workers/importer/fixtures/transactions/`. |
| PostgreSQL/PostGIS | Available through Compose | Slice 2 onward | Use existing migration and smoke patterns. |
| MLIT API key | Optional/local only | Deferred live ingestion | Do not require for fixture import or CI. |

---

## 9. Decisions and Plan Changes

### Decisions Made During Planning

| Date | Decision | Rationale | ADR / Reference |
|---|---|---|---|
| 2026-06-27 | Make fixture import the required Phase 2 path; keep live MLIT API ingestion deferred. | The committed official-source fixture is enough to prove parsing, validation, lineage, persistence, and idempotency without external availability or secrets. | `docs/data-sources.md`, ADR-003 |
| 2026-06-27 | Keep CSV fixture observations at `location_precision=unknown`. | CSV rows do not include defensible observation geometry and must not be guessed onto XPT001 station points. | ADR-004 |
| 2026-06-27 | Split implementation into six small slices: parser, schema, persistence, CLI, GraphQL, docs/UAT. | Each slice teaches one ingestion concept and keeps review/learning loops small. | `PHASE-PLAN.md` |

### Changes From Original Plan

| Date | Original Plan | Change | Reason | Impact |
|---|---|---|---|---|
| 2026-06-27 | User-requested command used period `2024Q1`. | Plan points first at existing `2024Q4` committed fixtures while leaving period configurable. | The repo's official fixtures are 2024 Q4 for three Tokyo wards. | Avoids inventing or downloading new fixture data during planning. |
| 2026-06-27 | Deliverable listed `fixtures/mlit/`. | Plan records an open decision about whether to create that as a wrapper/copy path or keep `workers/importer/fixtures/transactions/` canonical. | Existing Phase 00 fixture path is already documented and populated. | Prevents duplicate fixture churn until implementation chooses a stable boundary. |

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

Phase 2 planning documents were created from the template and tailored to the MLIT fixture importer objective.

### Current Working Assumption

The first importer should use the committed MLIT CSV fixtures and local PostgreSQL/PostGIS. It should not require an MLIT API key, live external requests, XPT001 geometry, or any property/station identity inference.

### Important Files

```text
.planning/STATE.md — project-wide resume point and active phase handoff
.planning/phases/02-ingestion-and-canonical-data-pipeline/PHASE-PLAN.md — Phase 2 implementation slices and acceptance criteria
docs/data-sources.md — MLIT fixture/source profile and source limitations
docs/data-model.md — conceptual lineage, idempotency, validation, and location-precision rules
workers/importer/fixtures/transactions/README.md — committed fixture acquisition, profile, and checksums
workers/importer/src/main.rs — current compile-only importer entrypoint
```

### Recommended Resume Command

```bash
sed -n '1,260p' .planning/phases/02-ingestion-and-canonical-data-pipeline/PHASE-PLAN.md
```

### Exact Next Technical Step

Begin Slice 1: add pure MLIT CSV parser and normalization tests before any database writes.

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
| 2026-06-27 13:00 +07:00 | `ready_for_implementation` | Phase 2 planning docs created from template with small implementation slices. |
