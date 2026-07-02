# Phase 02 Plan — Ingestion and Canonical Data Pipeline

> Purpose: Build the first official-source ingestion path while preserving raw source evidence, validation history, and idempotent canonical transaction observations.

## Metadata

| Field | Value |
|---|---|
| Phase | `02` |
| Name | `Ingestion and Canonical Data Pipeline` |
| Status | `in_progress` |
| Owner | `Project owner` |
| Created | `2026-06-27` |
| Last Updated | `2026-07-02` |
| Target Milestone | `MVP ingestion foundation` |
| Related ADRs | `docs/adr/001-use-postgis-for-spatial-queries.md`, `002-use-graphql-for-product-api.md`, `003-preserve-raw-source-payloads.md`, `004-model-location-precision-explicitly.md`, `005-use-rust-actix-web-for-api.md` |

---

## 1. Objective

### Problem

UrbanLens has a local platform and lineage foundation, but it cannot yet ingest an official dataset, preserve source rows, normalize useful transaction fields, or prove that repeated imports are safe. Without this phase, later map, metrics, and provenance screens would either depend on fake data or bypass the trust model.

### Intended Outcome

Create a Rust importer that loads the committed MLIT transaction CSV fixtures, records dataset/import-run/raw-record lineage, validates and normalizes records into canonical transaction observations, stores validation issues, and can be rerun without unintended duplicates. The imported records must be inspectable through bounded GraphQL queries.

### Why This Matters

This is the first end-to-end proof that UrbanLens can turn imperfect public data into analyst-usable records without hiding uncertainty. It establishes the working pattern for every later dataset: raw evidence first, explicit validation, conservative normalization, observable import status, and reproducible retry behavior.

---

## 2. Scope

### In Scope

- [x] Implement the first real `workers/importer` Rust CLI command for MLIT transaction fixture imports.
- [ ] Parse the committed CP932/Windows-31J MLIT CSV fixtures under `workers/importer/fixtures/transactions/`.
- [ ] Add migrations for `transaction_observations` and `transaction_location_contexts`, plus any narrowly required schema adjustments to Phase 01 lineage tables.
- [ ] Preserve raw rows, source positions, payload hashes, dataset artifact metadata, and normalization version.
- [ ] Validate source records into `valid`, `valid_with_warnings`, or `rejected` states.
- [ ] Normalize price, area, quarter, ward, station label/time, asset type, source unit price, and selected source context fields.
- [ ] Use idempotent upserts based on exact dataset artifact identity plus source row position, with deterministic source-record hashes where no stable external ID exists.
- [ ] Record import counters for received, imported, updated, duplicates skipped, rejected, and warning records.
- [x] Expose bounded GraphQL inspection for imported observations, validation issues, provenance, and import-run status.
- [x] Add `docs/importer.md`, `scripts/import-fixture.sh`, and a stable fixture path or documented wrapper for `fixtures/mlit/`.

### Out of Scope

- [ ] Live authenticated MLIT API ingestion as the required path.
- [ ] XPT001 station-point ingestion or guessed joins from CSV/XIT001 rows to station geometry.
- [ ] Exact property coordinates, durable `property` identity, canonical `station` identity, or ward boundary imports.
- [ ] Market metric materialization, area comparison, saved searches, authentication, or scheduling.
- [ ] Production-scale downloads, long-running job orchestration, Redis queues, or admin UI.
- [ ] Deriving universal unit price per square metre for records where the source does not supply it.

### Deferred Ideas

- [ ] Live XIT001 ingestion after the fixture path proves parsing, validation, idempotency, and persistence.
- [ ] XPT001 station-context geometry as its own defensibly linked source-record path.
- [ ] Reprocessing tools for new normalization versions.
- [ ] Admin-only raw payload view with intentional access controls and audit logging.
- [ ] Materialized market metrics and spatial aggregation after authoritative geography is imported.

---

## 3. Requirements

### Functional Requirements

| ID | Requirement | Priority | Notes |
|---|---|---:|---|
| FR-01 | Add an importer command equivalent to `cargo run -p urbanlens-importer -- import-transactions --source mlit --prefecture 13 --period 2024Q4 --fixture-dir workers/importer/fixtures/transactions`. | Must | Resolved in Slice 4: preserve the existing package name `urbanlens-importer` and document command examples with that package name. |
| FR-02 | Create or reuse a `data_sources` row for MLIT and create one dataset artifact record per imported source fixture artifact/query. | Must | Preserve retrieval query, artifact checksum, format, encoding, record count, and attribution context. |
| FR-03 | Every import execution creates an `import_run`, starts as `running`, and ends as `completed`, `completed_with_warnings`, or `failed`. | Must | Failed imports must remain visible with error kind and counts reached before failure. |
| FR-04 | Every received source row is stored as a `raw_record` before or alongside normalization. | Must | Preserve blank strings and source display values in JSONB. |
| FR-05 | Valid records create or update exactly one `transaction_observations` row linked to the originating raw record. | Must | Rejected raw records must not create observations. |
| FR-06 | Validation issues are stored for warnings and rejections with code, severity, field, raw value summary, message, and disposition. | Must | Examples include `missing_floor_area`, `unknown_asset_type`, `negative_price`, and `invalid_date`. |
| FR-07 | Re-running the same fixture import produces zero unintended duplicate raw records or observations. | Must | Counters must distinguish inserted, updated, duplicate/skipped, rejected, and warning records. |
| FR-08 | GraphQL exposes bounded inspection queries for transactions, import runs, validation issues, and provenance links. | Must | No unbounded list query and no raw payload JSON by default. |
| FR-09 | `./scripts/import-fixture.sh` starts/uses the local platform path and imports the committed fixture. | Must | The script must be safe for repeat local runs. |
| FR-10 | Importer documentation explains setup, command options, fixture source, idempotency, validation, counters, and troubleshooting. | Should | Keep MLIT API keys out of required fixture flow. |

### Non-Functional Requirements

| ID | Requirement | Priority | Verification Method |
|---|---|---:|---|
| NFR-01 | Import behavior is deterministic for the same fixture bytes and normalization version. | Must | Unit tests plus repeated integration import. |
| NFR-02 | Importer is retry-safe: failed runs are visible and a subsequent run can complete without manual database cleanup. | Must | Failure/retry integration test and UAT case. |
| NFR-03 | Logs include useful import context but never print secrets or full raw payloads. | Must | Code review and smoke log inspection. |
| NFR-04 | Validation prefers `null + warning` over invented defaults for missing or ambiguous values. | Must | Normalization tests and fixture inspection. |
| NFR-05 | All source and normalized units are explicit: JPY, JPY/m2, m2, minutes, quarter. | Must | GraphQL/API tests and docs review. |
| NFR-06 | The importer and API remain compatible with Docker-backed Rust checks when host Cargo is unavailable. | Must | `corepack pnpm check` or `bash scripts/check-rust-docker.sh`. |
| NFR-07 | Fixture import remains small and fast enough for local and CI-style checks. | Should | Smoke test runtime and fixture size review. |

### Data / Domain Requirements

| ID | Requirement | Source / Assumption | Notes |
|---|---|---|---|
| DR-01 | Keep `transaction_observation` as an observed historical publication, not a listing or durable property. | `docs/data-model.md` | No `properties` table in this phase. |
| DR-02 | Set CSV fixture observations to `location_precision=unknown` and `location=NULL`. | ADR-004, `docs/data-sources.md` | CSV rows must not be guessed onto XPT001 station points. |
| DR-03 | Model `transaction_period` as year plus quarter, not an exact transaction date. | MLIT schema | Reject impossible or unparseable quarter labels. |
| DR-04 | Preserve source unit price only when MLIT supplies `取引価格（㎡単価）`; do not derive a universal replacement. | Fixture profile | A separately calculated helper may be tested only if explicitly named as derived and not used as source unit price. |
| DR-05 | Store `ward_code` as text and validate MVP imports against Tokyo prefecture/ward codes. | Fixture profile | Do not infer authoritative ward geometry. |
| DR-06 | Preserve unknown asset labels with warnings instead of coercing them into misleading categories. | Ingestion rules | Rejections are reserved for records that cannot safely become observations. |
| DR-07 | Use exact-artifact idempotency: dataset identity by source/query/checksum and raw record identity by dataset plus source position. | Phase 0 decision | Identical rows at different positions remain distinct. |
| DR-08 | Raw payload JSON remains an intentional provenance/admin concern and is not returned by default GraphQL product queries. | ADR-003 | GraphQL can expose raw-record ID, hash, source position, and summary metadata. |

---

## 4. Technical Design

### Proposed Approach

Build the importer in small layers. First isolate MLIT CSV parsing and normalization as pure Rust modules with fixture tests. Then add the physical canonical schema and persistence repository functions. After those pieces are independently testable, wire them into a CLI command that manages import-run lifecycle and counters transactionally where practical.

The importer should use the existing PostgreSQL/PostGIS foundation and SQLx migrations. It should not require a live MLIT key for the default path. The first fixture import should be repeatable against local Docker Compose and should make imported observations visible through the API GraphQL boundary.

### Components Affected

| Component | Planned Change | Reason |
|---|---|---|
| `apps/web` | No major product UI; possibly retain connectivity shell only. | Phase 04 owns real map UI. |
| `apps/api` | Add bounded GraphQL read queries for transaction observations, import runs, validation issues, and provenance summaries. | Lets users inspect imported records after `scripts/import-fixture.sh`. |
| `workers/importer` | Add parser, validator, normalizer, persistence layer, CLI options, tests, and fixture command. | Core phase deliverable. |
| `database` | Add canonical transaction/location tables and constraints/indexes; adjust lineage tables only where implementation requires it. | Persist normalized observations with lineage and idempotency. |
| `infra` | Reuse Compose database/API; optionally add no new service. | Fixture import can run as a one-shot local command. |
| `docs` | Add importer docs and update data model/source/local-development docs where the physical schema or command contract changes. | Documentation is part of the product trust surface. |

### Data Flow

```text
Committed MLIT CSV fixture
  ↓
CSV decoding and source-row parsing
  ↓
raw_records with dataset/import_run lineage and payload hash
  ↓
validation issues plus normalized transaction observation fields
  ↓
transaction_observations and transaction_location_contexts
  ↓
bounded GraphQL inspection for imported records and provenance
```

### API / Interface Changes

| Type | Name | Change | Consumers |
|---|---|---|---|
| GraphQL Query | `transactionObservations` | Bounded list with limit/offset or cursor, basic filters, and source/provenance fields. | Local inspection, future market map |
| GraphQL Query | `importRuns` / `importRun` | Bounded import status and counters. | Local inspection, future operations page |
| GraphQL Query | `validationIssues` | Bounded list by import run, raw record, observation, severity, or code. | Provenance and QA |
| GraphQL Query | `dataSources` | Read source/dataset metadata needed to explain imported data. | Provenance views |
| CLI Command | `import-transactions` | Imports MLIT fixture rows for a prefecture/period/source combination. | Developer, CI-style smoke, future ops |
| Database Migration | `transaction_observations` | Store canonical normalized transaction observations. | Importer, API |
| Database Migration | `transaction_location_contexts` | Store explicit location precision/context separate from observed transaction facts. | Importer, future spatial queries |
| Environment Variable | `DATABASE_URL` | Required by importer when run outside Compose. | Importer |
| Environment Variable | `MLIT_REINFOLIB_API_KEY` | Not required for fixture import; keep optional diagnostic/live path only. | Future live importer |

### Data Model Changes

| Entity / Table | Change | Migration Required | Notes |
|---|---|---:|---|
| `transaction_observations` | New canonical table linked to `raw_records`, `import_runs`, and `datasets` either directly or through raw lineage. | Yes | Stores parsed period, asset type, price/area/station context, ward code, source hash, timestamps, and normalization version. |
| `transaction_location_contexts` | New table for precision, geometry, and context labels. | Yes | CSV fixtures should store `unknown` precision with null geometry. |
| `validation_issues` | May need optional `transaction_observation_id` or stricter issue enums. | Maybe | Preserve Phase 01 semantics while enabling observation-linked warnings. |
| `raw_records` | May need importer-friendly uniqueness/upsert support. | Maybe | Keep dataset plus source position identity. |
| `import_runs` | May need clearer updated/duplicate counters or status transitions. | Maybe | Prefer additive/compatible changes. |
| `data_sources` / `datasets` | May need seed/upsert helpers rather than schema changes. | Maybe | Store MLIT source metadata and fixture artifact records. |

### Geographic / Data Precision Notes

- Location precision: all CSV fixture observations use `unknown`.
- Geometry: `location` stays null for CSV fixture observations.
- User-facing disclaimer: "CSV fixture observations contain ward, district, and station labels but no defensible property or station-context geometry. They are eligible for non-spatial inspection and future metrics, but should not render as map points."
- Known data limitations: survey coverage is incomplete, values are rounded, exact property identity is intentionally obscured, source rows have no stable transaction ID, and source display values can be blank or bounded.
- Assumptions: Phase 2 imports records for trustworthy persistence and inspection; Phase 3+ supplies authoritative spatial behavior.

---

## 5. Implementation Slices

Each slice is deliberately small so the implementation teaches one ingestion concept at a time.

### Slice 1 — Source Parser and Normalization Rules

**Goal**

Learn the source shape without touching the database: decode MLIT CSV rows, preserve raw values, and convert only the safest canonical fields.

**Tasks**

- [x] Add MLIT CSV parser modules and fixture tests for the committed files.
- [x] Handle CP932/Windows-31J decoding, headers, blank strings, row positions, and line-ending quirks.
- [x] Parse transaction quarter, JPY price, source unit price, area, total floor area display values, ward code, station label, walk minutes, and source asset type.
- [x] Define normalization structs and validation issue codes in importer modules.
- [x] Test numeric normalization, unit-price handling, invalid values, bounded display values, and unknown asset labels.

**Expected Evidence**

- [x] Unit tests parse all three fixture files and confirm 666 source records.
- [x] Tests prove raw strings survive and invalid/ambiguous values become warnings or rejections instead of fake defaults.

**Completion Notes — 2026-06-29**

- Implemented pure parser/normalizer code in `workers/importer/src/mlit.rs`.
- Added CP932/Windows-31J fixture decoding, documented 30-column header validation, source row positions, raw-value preservation, canonical normalization structs, and validation issue codes.
- Fixture tests parse Chuo, Shinagawa, and Shibuya 2024 Q4 files, confirm 666 source rows, preserve blank raw strings, preserve MLIT-supplied source unit price only, and keep CSV fixture observations at `location_precision=unknown`.
- Edge tests cover unknown asset labels, negative prices, invalid numeric values, invalid station walking time, bounded total floor-area display values, unsupported price categories, invalid municipality codes, and invalid quarters.

---

### Slice 2 — Canonical Schema and Database Contracts

**Goal**

Add the minimum physical schema needed to persist normalized observations with lineage and honest location precision.

**Tasks**

- [x] Add SQLx migrations for `transaction_observations` and `transaction_location_contexts`.
- [x] Add constraints for lineage, validation state, positive numeric fields, quarter format, location precision, and geometry/precision consistency.
- [x] Add uniqueness for idempotent observation upsert by source-record hash or raw-record lineage.
- [x] Add indexes for import-run lookup, raw-record lookup, ward/period filtering, and future spatial filtering.
- [x] Add migration/schema tests or smoke assertions.

**Expected Evidence**

- [x] Fresh and existing databases migrate successfully.
- [x] Schema checks prove location cannot be stored as exact when the precision is `unknown`, and duplicates are constrained.

**Completion Notes — 2026-06-29**

- Added migration `202606290001_create_transaction_observation_schema.sql`.
- Created `transaction_observations` with required raw/import/dataset lineage, governed asset/price/validation fields, year/quarter period, explicit unit-bearing numeric fields, station/ward/source context fields, and normalization version.
- Created `transaction_location_contexts` with mandatory `location_precision`, nullable SRID 4326 geometry, a one-to-one observation link, and precision/geometry consistency checks.
- Added optional `validation_issues.transaction_observation_id` so warning issues can later link to normalized observations without breaking raw-record/import-run issue scope.
- Anchored observation idempotency in raw-record lineage: one observation per raw record. `source_record_hash` remains indexed evidence rather than a uniqueness key, preserving distinct identical rows at different source positions.
- Extended `scripts/smoke-compose.sh` to verify the new tables/indexes/geometry metadata, reject geometry for `unknown` precision, and reject duplicate observations for one raw record. Slice 3 later raised the successful migration count to four.

---

### Slice 3 — Persistence Repositories and Idempotency

**Goal**

Persist source artifacts, raw records, validation issues, and observations safely before wiring the CLI.

**Tasks**

- [x] Add SQLx repository functions for data-source/dataset upsert, import-run lifecycle, raw-record upsert, validation issue insert, and observation upsert.
- [x] Implement counters for received, imported, updated, duplicates skipped, rejected, and warning records.
- [x] Make duplicate fixture imports reuse or skip existing rows without double-counting observations.
- [x] Preserve failed import runs and support retry in a later run.
- [x] Add integration tests against PostgreSQL/PostGIS through existing Docker-backed check patterns.

**Expected Evidence**

- [x] Import persistence tests prove raw-record preservation, observation linkage, warning/rejection storage, counters, and duplicate behavior.
- [x] A forced failure leaves an `import_runs.status='failed'` row with an error kind.

**Completion Notes — 2026-06-29**

- Added migration `202606290002_add_lineage_upsert_keys.sql` with durable
  upsert keys for `data_sources` and exact artifact/query `datasets`.
- Added `workers/importer/src/persistence.rs` with repository functions for
  source/dataset upsert, import-run start/complete/fail, raw-record insert with
  duplicate skip, validation issue insert, transaction observation upsert,
  unknown-location context upsert, and import counters.
- Preserved exact-artifact idempotency by skipping duplicate
  `(dataset_id, source_position)` rows while keeping the original raw-record
  import-run lineage intact.
- Added DB-backed repository tests for stable source/dataset reuse,
  raw-record/observation linkage, duplicate source-position skipping,
  warning/rejection issue storage, import counters, and failed-run visibility.
- Updated Compose smoke to expect four successful migrations.

---

### Slice 4 — Importer CLI and Fixture Script

**Goal**

Give the user one repeatable command to import the committed official-source fixture.

**Tasks**

- [x] Implement `import-transactions` CLI options for source, prefecture, period, fixture directory, normalization version, and database URL.
- [x] Create `scripts/import-fixture.sh` as the stable local entrypoint.
- [x] Ensure the command creates an import run, loads fixture artifacts, stores raw rows, validates, normalizes, upserts, records issues, updates counters, and marks final status.
- [x] Add clear terminal output that summarizes counts without printing full raw payloads.
- [x] Document that the Cargo package remains `urbanlens-importer` and command examples use `cargo run -p urbanlens-importer -- import-transactions`.

**Expected Evidence**

- [x] Running `./scripts/import-fixture.sh` imports the fixture into a local database.
- [x] Running it twice reports no unintended duplicate observations.

**Completion Notes — 2026-06-29**

- Replaced the compile-only importer entrypoint with `import-transactions` in
  `workers/importer/src/main.rs`.
- Added CLI options for `--source`, `--prefecture`, `--period`,
  `--fixture-dir`, `--normalization-version`, and `--database-url`.
- The CLI discovers CSV artifacts, computes artifact SHA-256 checksums, upserts
  MLIT data source/dataset rows, starts one import run per artifact, normalizes
  rows through the Slice 1 parser, persists through the Slice 3 repositories,
  records counters, and marks terminal status.
- Added `scripts/import-fixture.sh` as the stable Docker-backed local entrypoint.
  It joins the `urbanlens_default` Compose network and uses the existing
  package name `urbanlens-importer`.
- Verified first import against local Compose: 3 artifacts, 666 received, 666
  imported, 0 rejected, 0 warning records.
- Verified duplicate rerun: 3 artifacts, 666 received, 0 imported, 666
  duplicates skipped, 0 rejected, 0 warning records.
- Count check after both runs: 3 datasets, 6 import runs, 666 raw records, 666
  transaction observations, and 0 validation issues.

---

### Slice 5 — GraphQL Inspection Path

**Goal**

Make imported records visible through the product API boundary without exposing raw payload JSON by default.

**Tasks**

- [x] Add bounded GraphQL types and queries for transaction observations, import runs, validation issues, data sources, and provenance summaries.
- [x] Include units, period, source names, import-run IDs, raw-record IDs/hashes, validation states, and location precision.
- [x] Add pagination or strict limits to every list query.
- [x] Add resolver tests for bounded results and raw-payload exclusion from the schema.
- [x] Update `/ready` or connectivity only if new migration expectations require it.

**Expected Evidence**

- [x] After fixture import, a GraphQL query can inspect imported observations and import-run counters.
- [x] Tests prove raw payload JSON is not exposed through default product queries.

**Completion Notes — 2026-07-02**

- Added bounded GraphQL queries in `apps/api/src/lib.rs`:
  `transactionObservations`, `importRuns`, `validationIssues`,
  `dataSources`, and `transactionObservationProvenance`.
- List queries default to 25 rows and clamp to 100 rows. Validation issue
  filters are grouped in `ValidationIssueFilter`; UUID filters are validated
  before SQL execution.
- Observation results include period, unit-bearing numeric fields, ward/station
  context, validation state, lineage IDs, and `locationPrecision`. Provenance
  summaries expose raw-record ID, source position, payload hash, import-run
  status, dataset retrieval metadata, artifact checksum, and data-source
  identity, without exposing `payload_json`.
- Added API schema/pagination tests and enabled SQLx `uuid` support for the API
  release image path.
- Verified with Docker-backed Rust checks, isolated Compose smoke, fixture
  import into the isolated stack, and live GraphQL observation/import-run/source
  plus provenance queries.

---

### Slice 6 — Documentation, Regression Checks, and UAT Readiness

**Goal**

Close the phase as a teachable, repeatable workflow with clear evidence.

**Tasks**

- [x] Add `docs/importer.md` with command usage, fixture provenance, validation rules, idempotency model, counters, and troubleshooting.
- [x] Update `docs/data-model.md`, `docs/data-sources.md`, `docs/local-development.md`, and README where behavior changes.
- [x] Add or document `fixtures/mlit/` as the stable fixture boundary if implementation creates it; otherwise explain why the canonical fixture path remains `workers/importer/fixtures/transactions/`.
- [x] Run Rust, web, importer, migration, GraphQL, and Compose smoke checks.
- [x] Execute Phase 2 UAT and update planning status/state.

**Expected Evidence**

- [x] UAT shows fixture import, raw preservation, normalized observation persistence, validation visibility, duplicate-free rerun, failed-run visibility, and GraphQL inspection.
- [x] Planning docs and `.planning/STATE.md` record the Phase 2 completion or exact next action.

**Completion Notes — 2026-07-02**

- Updated importer, README, data-source, data-model, Phase 02 status, UAT, and
  project state documentation for the completed fixture ingestion workflow.
- Verified `corepack pnpm check`, `corepack pnpm --filter @urbanlens/web build`,
  and isolated Compose smoke with `COMPOSE_PROJECT_NAME=urbanlens_slice6_uat`.
- Imported the committed MLIT fixtures into the isolated stack: 3 artifacts, 666
  received, 666 imported, 0 rejected, and 0 warning records.
- Re-ran the same fixture import: 666 received, 0 imported, and 666 duplicates
  skipped.
- Verified database counts after the duplicate rerun: 1 data source, 3 datasets,
  6 import runs, 666 raw records, 666 observations, 0 validation issues, and 666
  unknown/null location contexts.
- Verified bounded GraphQL inspection and single-observation provenance without
  exposing raw payload JSON.
- Verified failed-run visibility with a temporary isolated database trigger that
  forced a `persistence_error`, then removed the trigger and confirmed a normal
  retry completed successfully.

---

## 6. Testing Strategy

### Unit Tests

| Area | Required Coverage |
|---|---|
| MLIT CSV parsing | Header validation, CP932 decoding, fixture row counts, blank-string preservation, row positions. |
| Numeric normalization | JPY price, source JPY/m2, m2 area, walk minutes, bounded display values, non-finite/negative values. |
| Period parsing | Japanese/display quarter parsing, invalid quarter rejection, no invented exact dates. |
| Asset type mapping | Known MLIT labels, unknown labels preserved with warnings. |
| Validation rules | `valid`, `valid_with_warnings`, `rejected`; issue code/severity/disposition behavior. |
| Hashing/idempotency | Deterministic payload hash and source-record hash for the same normalized raw representation. |

### Integration Tests

| Flow | Expected Result |
|---|---|
| Fresh fixture import | Creates MLIT source/dataset/import-run/raw-records/observations/issues with correct counters. |
| Duplicate fixture import | Creates no unintended duplicate observations; counters expose updates/skips. |
| Rejected-record fixture | Preserves raw record, stores rejection issue, creates no observation. |
| Forced failure | Leaves failed import run visible with error kind and supports later successful retry. |
| GraphQL inspection | Returns bounded imported observations, import runs, validation issues, and provenance metadata. |
| Migration rerun | Existing volumes migrate safely and retain data. |

### Manual Validation

| Scenario | Why Manual Validation Is Needed |
|---|---|
| `./scripts/import-fixture.sh` on a local Compose database | Confirms the intended developer workflow and command output. |
| GraphQL query after import | Confirms imported data is inspectable through the public API boundary. |
| Importer log review | Confirms useful observability without secrets or full raw payload dumps. |
| Duplicate rerun inspection | Confirms counters and row counts are understandable to a human reviewer. |

### Regression Risks

| Risk Area | Possible Regression | Mitigation |
|---|---|---|
| Lineage schema | Observation points to the wrong raw record/import/dataset. | Foreign keys, repository tests, UAT provenance checks. |
| Idempotency | Re-running imports duplicates observations or collapses legitimate duplicate rows. | Dataset+source-position identity tests and fixture duplicate scenarios. |
| Location precision | CSV observations appear as mappable/exact points. | DB constraints, GraphQL tests, UAT precision check. |
| Unit handling | Source unit price is silently derived or mixed across asset types. | Normalization tests and field naming that distinguishes source value from derived helpers. |
| Failed imports | Import failures disappear or leave misleading completed states. | Failure/retry tests and import-run status UAT. |
| API bounds | New GraphQL lists return unbounded datasets. | Resolver limits and tests. |

---

## 7. Acceptance Criteria

### Product / User Criteria

- [ ] A developer can run `./scripts/import-fixture.sh` and import a realistic official MLIT transaction fixture.
- [ ] Imported observations can be inspected through GraphQL with source, period, units, validation state, and provenance metadata.
- [ ] The system honestly represents CSV fixture locations as `unknown`, not property points or station points.
- [ ] Validation warnings and rejected records are visible and understandable.

### Engineering Criteria

- [ ] Raw payloads are stored and linked to import runs, datasets, and normalized observations.
- [ ] Running the same import twice creates zero unintended duplicates.
- [ ] Failed imports are visible in `import_runs` and do not require manual cleanup before retry.
- [ ] Tests cover parsing, validation, normalization, unit-price behavior, idempotency, counters, raw preservation, failure, and retry.
- [ ] Every new list query is bounded by pagination or a strict limit.
- [ ] Existing Phase 01 startup, health, readiness, and smoke behavior remain intact.

### Documentation Criteria

- [ ] `docs/importer.md` exists and explains command usage, fixture source, validation, idempotency, counters, and troubleshooting.
- [ ] Source/data-model/local-development docs are updated for physical ingestion behavior.
- [ ] Fixture location and checksum/provenance remain documented.
- [ ] Planning status, UAT, and project state are updated when the phase reaches implementation, UAT, and completion milestones.

### UAT Criteria

- [x] Phase 2 UAT proves fixture import, GraphQL inspection, raw-record preservation, validation issue visibility, duplicate-free rerun, failed-run visibility, and honest location precision.

---

## 8. Handoff Notes

### Implementation Starting Point

Phase 02 is complete. Resume by starting Phase 03 planning for spatial data
model and query engine, preserving the Phase 02 boundary that CSV transaction
observations remain `location_precision=unknown` until a defensible geometry
source is ingested.

### Resolved Questions

- Resolved during Slice 4: keep the Cargo package name as
  `urbanlens-importer`; docs and direct command examples use
  `cargo run -p urbanlens-importer -- import-transactions`.
- Resolved during Slice 4: keep `workers/importer/fixtures/transactions/` as
  the canonical fixture path and use `scripts/import-fixture.sh` as the stable
  local wrapper.
- Resolved during Slice 2: normalized transaction facts, source context labels, period, units, and raw/import/dataset lineage belong on `transaction_observations`; precision and geometry belong on `transaction_location_contexts`.
- Resolved during Slice 2: validation issue storage gains nullable `transaction_observation_id` while preserving existing import-run/raw-record scope.

### Do Not Reopen Without New Evidence

- Do not add a durable `property` table.
- Do not infer exact property points.
- Do not join CSV rows to XPT001 geometries by guesswork.
- Do not require a live MLIT API key for fixture import.
