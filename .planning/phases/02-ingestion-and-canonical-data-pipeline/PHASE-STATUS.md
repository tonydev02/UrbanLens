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
| Last Updated | `2026-07-02 00:00 +09:00` |
| Target Completion | `TBD` |
| Current Branch | `main` |
| Current Commit | `309928c` |
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

Slices 1, 2, 3, 4, and 5 are complete. The importer has pure MLIT CSV parsing/normalization rules, canonical transaction/location schema contracts, persistence repositories for lineage-preserving database writes, a repeat-safe fixture CLI/script, and bounded GraphQL inspection for imported observations and provenance. The next work is Slice 6 documentation, regression checks, and UAT readiness.

## 3. Definition of Done

Phase 2 is done when `./scripts/import-fixture.sh` imports the committed MLIT fixture, GraphQL can inspect imported observations and provenance, raw records and validation issues are preserved, reruns create no unintended duplicates, failed imports are visible, and tests cover parsing, validation, normalization, counters, idempotency, and retry behavior.

---

## 4. Progress Snapshot

| Area | Status | Progress | Notes |
|---|---|---:|---|
| Planning | Done | 100% | Phase 2 plan/status/UAT are created from the template and ready for implementation. |
| Design / Architecture | In Progress | 95% | Parser/normalization, canonical schema, repository, CLI/script, and GraphQL inspection boundaries are implemented; final UAT/documentation closure remains. |
| Backend | In Progress | 70% | Bounded GraphQL inspection queries are implemented for observations, import runs, validation issues, data sources, and provenance summaries. |
| Database | In Progress | 45% | Canonical transaction/location migrations, lineage upsert keys, schema contract smoke assertions, and repository write tests are implemented. |
| Worker / Ingestion | In Progress | 65% | Parser/normalizer, persistence repositories, CLI import command, and fixture wrapper are implemented and repeat-verified. |
| Frontend | Not Started | 0% | No Phase 2 product UI planned beyond existing shell. |
| Tests | In Progress | 60% | Importer parser/normalization, repository tests, CLI parser tests, API GraphQL schema/pagination tests, fixture import, live GraphQL query, and isolated Compose smoke pass; formal UAT remains. |
| Documentation | In Progress | 70% | Importer, data-model, and planning docs are current for Slice 5; final Phase 02 UAT docs remain. |
| UAT | Not Started | 0% | UAT protocol drafted; no cases run. |

---

## 5. Completed Work

Record outcomes, not just activity.

| Date | Completed Outcome | Evidence / Link |
|---|---|---|
| 2026-07-02 | Completed Slice 5: bounded GraphQL inspection queries for transaction observations, import runs, validation issues, data sources, and single-observation provenance summaries, with schema/pagination tests and live isolated-stack verification. | `apps/api/src/lib.rs`, `apps/api/Cargo.toml`, `docs/importer.md`, `docs/data-model.md` |
| 2026-06-29 | Completed Slice 4: `import-transactions` CLI, Docker-backed `scripts/import-fixture.sh`, first fixture import, duplicate-safe rerun, CLI parser tests, and importer/local-development documentation. | `workers/importer/src/main.rs`, `scripts/import-fixture.sh`, `docs/importer.md`, `docs/local-development.md` |
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
| Slice 3 persistence repositories and idempotency | Complete | Continue using repositories as the only database write boundary. |
| Slice 4 importer CLI and fixture script | Complete | Use the stable fixture import path for UAT evidence. |
| Slice 5 GraphQL inspection | Complete | Use the bounded API inspection path as UAT evidence and future product-query foundation. |
| Phase 2 planning | Complete | Use the plan as the implementation guide, updating it if later slices reveal new source constraints. |
| Importer package command naming | Complete | Decision documented: keep `urbanlens-importer`; command examples use `cargo run -p urbanlens-importer -- import-transactions`. |
| Fixture path deliverable | Resolved | Keep `workers/importer/fixtures/transactions/` canonical and use `scripts/import-fixture.sh` as the stable wrapper. |

---

## 7. Exact Next Actions

Keep this short. The first action must be the exact action to take when work resumes.

1. [ ] **Next immediate action:** Begin Slice 6 by completing Phase 02 documentation, regression-check evidence, and UAT readiness.
2. [x] Decide whether validation issue storage needs an observation-level foreign key or can remain raw-record/import-run scoped for the first persistence slice.
3. [x] Decide and document importer package naming before publishing the first command examples.

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
| 2026-06-29 | Keep the importer package name as `urbanlens-importer`. | This matches existing workspace naming and avoids churn; command examples now use `cargo run -p urbanlens-importer -- import-transactions`. | `docs/importer.md` |
| 2026-06-29 | Use `scripts/import-fixture.sh` as the stable fixture boundary rather than copying fixtures to `fixtures/mlit/`. | The committed Phase 0 fixture path is already documented and populated; the script provides the stable user entrypoint. | `docs/importer.md` |

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
| Rust formatting | Pass | `bash scripts/check-rust-docker.sh` on `2026-07-02`. |
| Rust lint | Pass | Docker-backed clippy workspace/all-target/all-feature check with warnings denied on `2026-07-02`. |
| Rust tests | Pass | Docker-backed Rust tests pass: API 6 tests, importer 9 parser/normalization/repository tests, importer CLI 2 tests, and workspace doctests on `2026-07-02`. |
| Importer CLI tests | Pass | Docker-backed Rust tests include 2 `urbanlens-importer` binary CLI parser tests on `2026-07-02`. |
| TypeScript lint | Not Run | No frontend code changed in Slice 3. |
| TypeScript type check | Not Run | No frontend code changed in Slice 3. |
| Frontend tests | Not Run | No frontend code changed in Slice 3. |
| Integration tests | Pass | `docker run ... cargo test -p urbanlens-importer persistence::tests` passed against Compose Postgres on `2026-06-29`. |
| Fixture import script | Pass | First `./scripts/import-fixture.sh`: 3 artifacts, 666 received, 666 imported, 0 rejected, 0 warnings. Second run: 666 received, 0 imported, 666 duplicates skipped. |
| Imported row counts | Pass | After final fixture-script validation: 3 datasets, 9 import runs, 666 raw records, 666 transaction observations, 0 validation issues. |
| GraphQL schema tests | Pass | API tests prove bounded pagination, Slice 5 query presence, `locationPrecision` and `payloadSha256` exposure, and no `payloadJson` field in the default schema. |
| Live GraphQL inspection | Pass | Isolated stack fixture import returned 666 imported rows; live GraphQL returned observations, import-run counters, data source metadata, empty validation issues, and a provenance summary with raw-record/dataset/source lineage. |
| Docker Compose config | Pass | Covered by isolated `COMPOSE_PROJECT_NAME=urbanlens_slice5_smoke API_PORT=18080 WEB_PORT=13080 POSTGRES_PORT=15432 bash scripts/smoke-compose.sh` on `2026-07-02`. |
| Docker Compose smoke test | Pass | Isolated fresh-volume Compose smoke passed on `2026-07-02`; default-project smoke was not used as evidence because its local volume already contained imported fixture data. |

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

Slice 5 bounded GraphQL inspection was implemented and verified against an
isolated Compose stack with imported MLIT fixture rows.

### Current Working Assumption

The first importer uses the committed MLIT CSV fixtures and local PostgreSQL/PostGIS. It does not require an MLIT API key, live external requests, XPT001 geometry, or any property/station identity inference. Slices 1 through 5 keep CSV observations spatially `unknown` and expose imported observations through bounded GraphQL inspection without returning raw payload JSON by default.

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
apps/api/src/lib.rs — bounded GraphQL inspection queries and API schema tests
apps/api/Cargo.toml — API SQLx feature set, including UUID support for GraphQL filters
scripts/smoke-compose.sh — Compose and schema contract smoke validation
workers/importer/src/persistence.rs — Slice 3 repository writes and DB-backed tests
workers/importer/src/main.rs — `import-transactions` CLI entrypoint
scripts/import-fixture.sh — stable Docker-backed fixture import wrapper
```

### Recommended Resume Command

```bash
sed -n '1,260p' .planning/phases/02-ingestion-and-canonical-data-pipeline/PHASE-PLAN.md
```

### Exact Next Technical Step

Begin Slice 6: complete Phase 02 documentation, regression-check evidence, and UAT readiness.

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
| 2026-06-29 22:30 +09:00 | `in_progress` | Slice 4 importer CLI and Docker-backed fixture script completed; first import and duplicate rerun verified 666 official fixture rows without unintended duplicate observations; focus moves to Slice 5 GraphQL inspection. |
| 2026-07-02 00:00 +09:00 | `in_progress` | Slice 5 bounded GraphQL inspection completed and verified with Docker-backed Rust checks, isolated Compose smoke, fixture import, live observation/import-run/source query, and live provenance query; focus moves to Slice 6 UAT/docs closure. |
