# Phase 02 Status — Ingestion and Canonical Data Pipeline

> Purpose: Record the current phase state, progress, blockers, and exact next action.

## Current State

| Field | Value |
|---|---|
| Phase | `02` |
| Name | `Ingestion and Canonical Data Pipeline` |
| Overall Status | `in_progress` |
| Health | `green` |
| Owner | `Project owner` |
| Started | `2026-06-27` |
| Last Updated | `2026-06-29 19:45 +09:00` |
| Target Completion | `TBD` |
| Current Branch | `main` |
| Current Commit | `6e099e5` |
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

Slices 1, 2, and 3 are complete. The importer has pure MLIT CSV parsing/normalization rules, canonical transaction/location schema contracts, and persistence repositories for lineage-preserving database writes. The next work is Slice 4 CLI wiring and the stable fixture import script.

## 3. Definition of Done

Phase 2 is done when `./scripts/import-fixture.sh` imports the committed MLIT fixture, GraphQL can inspect imported observations and provenance, raw records and validation issues are preserved, reruns create no unintended duplicates, failed imports are visible, and tests cover parsing, validation, normalization, counters, idempotency, and retry behavior.

---

## 4. Progress Snapshot

| Area | Status | Progress | Notes |
|---|---|---:|---|
| Planning | Done | 100% | Phase 2 plan/status/UAT are created from the template and ready for implementation. |
| Design / Architecture | In Progress | 85% | Parser/normalization boundary, canonical observation/location schema, and repository boundaries are implemented; CLI and GraphQL boundaries remain. |
| Backend | Not Started | 0% | GraphQL inspection queries are planned but not implemented. |
| Database | In Progress | 45% | Canonical transaction/location migrations, lineage upsert keys, schema contract smoke assertions, and repository write tests are implemented. |
| Worker / Ingestion | In Progress | 45% | Parser/normalizer and persistence repositories are implemented; CLI import command remains. |
| Frontend | Not Started | 0% | No Phase 2 product UI planned beyond existing shell. |
| Tests | In Progress | 35% | Importer parser/normalization and repository tests pass; schema contracts are in Compose smoke; CLI, GraphQL, and UAT tests remain. |
| Documentation | In Progress | 45% | Importer, data model, and planning docs are current for Slice 3. |
| UAT | Not Started | 0% | UAT protocol drafted; no cases run. |

---

## 5. Completed Work

Record outcomes, not just activity.

| Date | Completed Outcome | Evidence / Link |
|---|---|---|
| 2026-06-29 | Completed Slice 3: lineage upsert keys, persistence repositories, idempotent raw-record duplicate skipping, observation/location writes, validation issue storage, counters, failed-run visibility, and DB-backed repository tests. | `apps/api/migrations/202606290002_add_lineage_upsert_keys.sql`, `workers/importer/src/persistence.rs`, `docs/importer.md`, `docs/data-model.md` |
| 2026-06-29 | Completed Slice 2: canonical transaction/location migrations, observation-level validation issue link, lineage/idempotency constraints, precision/geometry constraints, and Compose schema contract assertions. | `apps/api/migrations/202606290001_create_transaction_observation_schema.sql`, `scripts/smoke-compose.sh`, `docs/data-model.md`, `docs/importer.md` |
| 2026-06-29 | Completed Slice 1: pure MLIT CSV parser/normalizer, validation issue codes, CP932 fixture parsing, raw string preservation, and edge-case tests before database writes. | `workers/importer/src/mlit.rs`, `workers/importer/src/lib.rs`, `docs/importer.md` |
| 2026-06-27 | Created the Phase 2 planning folder and drafted plan/status/UAT documents from the template, aligned with Phase 00 source/model decisions and Phase 01 handoff. | `.planning/phases/02-ingestion-and-canonical-data-pipeline/` |

---

## 6. Work In Progress

| Item | Current State | Next Step |
|---|---|---|
| Slice 1 parser and normalization | Complete | Use the parser boundary as the input to Slice 4 CLI wiring. |
| Slice 2 canonical schema and database contracts | Complete | Continue using the schema constraints as the target for repository writes. |
| Slice 3 persistence repositories and idempotency | Complete | Wire repositories into the CLI and fixture script. |
| Phase 2 planning | Complete | Use the plan as the implementation guide, updating it if later slices reveal new source constraints. |
| Importer package command naming | Open question | Decide whether to keep `urbanlens-importer` or rename/alias to `importer`. |
| Fixture path deliverable | Open question | Decide whether `fixtures/mlit/` becomes a wrapper/copy path or whether existing importer fixtures remain canonical. |

---

## 7. Exact Next Actions

Keep this short. The first action must be the exact action to take when work resumes.

1. [ ] **Next immediate action:** Begin Slice 4 by wiring `import-transactions` CLI options to the parser and persistence repositories, then add `scripts/import-fixture.sh`.
2. [x] Decide whether validation issue storage needs an observation-level foreign key or can remain raw-record/import-run scoped for the first persistence slice.
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
| 2026-06-29 | Keep observation idempotency anchored in raw-record lineage and treat `source_record_hash` as indexed evidence, not a uniqueness key. | Prevents distinct identical source rows at different ordinals from being collapsed. | `docs/data-model.md`, ADR-003 |
| 2026-06-29 | Add nullable `validation_issues.transaction_observation_id`. | Warning issues can link to normalized observations while rejected rows remain raw-record/import-run scoped. | `PHASE-PLAN.md` |
| 2026-06-29 | Add unique upsert keys for `data_sources` and exact artifact/query `datasets`. | Repository upserts need durable database-level identity rather than best-effort select/update behavior. | `apps/api/migrations/202606290002_add_lineage_upsert_keys.sql` |

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
| Rust formatting | Pass | `bash scripts/check-rust-docker.sh` on `2026-06-29`. |
| Rust lint | Pass | Docker-backed clippy workspace/all-target/all-feature check with warnings denied on `2026-06-29`. |
| Rust tests | Pass | Docker-backed Rust tests pass: API 4 tests, importer 9 parser/normalization/repository tests, workspace doctests on `2026-06-29`. |
| TypeScript lint | Not Run | No frontend code changed in Slice 3. |
| TypeScript type check | Not Run | No frontend code changed in Slice 3. |
| Frontend tests | Not Run | No frontend code changed in Slice 3. |
| Integration tests | Pass | `docker run ... cargo test -p urbanlens-importer persistence::tests` passed against Compose Postgres on `2026-06-29`. |
| Docker Compose config | Pass | Covered by `bash scripts/smoke-compose.sh` on `2026-06-29`. |
| Docker Compose smoke test | Pass | `bash scripts/smoke-compose.sh` passes on a fresh volume on `2026-06-29`; asserts four migrations, new transaction tables, precision/geometry constraints, and duplicate-observation rejection. |

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

Slice 3 persistence repositories and DB-backed repository tests were implemented.

### Current Working Assumption

The first importer should use the committed MLIT CSV fixtures and local PostgreSQL/PostGIS. It should not require an MLIT API key, live external requests, XPT001 geometry, or any property/station identity inference. Slices 1 through 3 keep CSV observations spatially `unknown`; Slice 4 should call the existing parser and persistence repositories rather than reimplementing import behavior in the CLI.

### Important Files

```text
.planning/STATE.md — project-wide resume point and active phase handoff
.planning/phases/02-ingestion-and-canonical-data-pipeline/PHASE-PLAN.md — Phase 2 implementation slices and acceptance criteria
docs/data-sources.md — MLIT fixture/source profile and source limitations
docs/data-model.md — conceptual lineage, idempotency, validation, and location-precision rules
workers/importer/fixtures/transactions/README.md — committed fixture acquisition, profile, and checksums
workers/importer/src/mlit.rs — pure MLIT parser, normalization structs, validation issues, and Slice 1 tests
apps/api/migrations/202606290001_create_transaction_observation_schema.sql — canonical observation/location schema
apps/api/migrations/202606290002_add_lineage_upsert_keys.sql — source/dataset upsert keys
scripts/smoke-compose.sh — Compose and schema contract smoke validation
workers/importer/src/persistence.rs — Slice 3 repository writes and DB-backed tests
workers/importer/src/main.rs — current compile-only importer entrypoint
```

### Recommended Resume Command

```bash
sed -n '1,260p' .planning/phases/02-ingestion-and-canonical-data-pipeline/PHASE-PLAN.md
```

### Exact Next Technical Step

Begin Slice 4: implement the `import-transactions` CLI options and `scripts/import-fixture.sh` wrapper around the Slice 1 parser and Slice 3 repositories.

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
| 2026-06-29 00:00 +09:00 | `in_progress` | Slice 1 parser/normalizer completed; Docker-backed Rust formatting, clippy, and tests pass. |
| 2026-06-29 12:00 +09:00 | `in_progress` | Slice 2 canonical transaction/location schema and Compose schema contract assertions implemented; focus moves to Slice 3 persistence repositories. |
| 2026-06-29 19:45 +09:00 | `in_progress` | Slice 3 persistence repositories, idempotency keys, DB-backed repository tests, and fresh-volume Compose smoke completed; focus moves to Slice 4 CLI and fixture script. |
