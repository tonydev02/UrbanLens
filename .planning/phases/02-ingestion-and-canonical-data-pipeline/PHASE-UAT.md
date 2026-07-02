# Phase 02 UAT — Ingestion and Canonical Data Pipeline

> Purpose: Verify that Phase 2 proves trustworthy official-source ingestion, not merely that importer code runs.

## Metadata

| Field | Value |
|---|---|
| Phase | `02` |
| Name | `Ingestion and Canonical Data Pipeline` |
| UAT Status | `completed` |
| Environment | `local` |
| Tester | `Project owner` |
| Started | `2026-07-02 11:37 +09:00` |
| Completed | `2026-07-02 11:41 +09:00` |
| Build / Commit | `2d69474` |
| Related Plan | `PHASE-PLAN.md` |
| Related Status | `PHASE-STATUS.md` |

---

## 1. UAT Objective

Verify that the importer can load the official-source MLIT transaction fixture, preserve raw records, create normalized transaction observations, record validation warnings/rejections, expose provenance through GraphQL, and avoid unintended duplicates when run repeatedly.

---

## 2. Preconditions

### Required Setup

- [x] Correct branch is checked out.
- [x] Working tree contains no unrelated uncommitted changes that would affect ingestion.
- [x] Docker Compose is available.
- [x] Local PostgreSQL/PostGIS stack can start through `docker compose up --build`.
- [x] Required migrations have been applied.
- [x] MLIT fixture CSVs are available under `workers/importer/fixtures/transactions/`.
- [x] No MLIT API key is required for fixture UAT.

### Test Data

| Data Set / Fixture | Purpose | Setup Command / Location |
|---|---|---|
| MLIT 2024 Q4 Chuo fixture | Official-source CSV parser and import coverage | `workers/importer/fixtures/transactions/mlit-reinfolib-chuo-2024-q4.csv` |
| MLIT 2024 Q4 Shinagawa fixture | Larger ward fixture for duplicate/idempotency checks | `workers/importer/fixtures/transactions/mlit-reinfolib-shinagawa-2024-q4.csv` |
| MLIT 2024 Q4 Shibuya fixture | Additional ward fixture for parser and counter coverage | `workers/importer/fixtures/transactions/mlit-reinfolib-shibuya-2024-q4.csv` |
| Invalid/rejection fixture | Negative price, invalid date, missing required source identity, invalid geometry cases | Covered by parser and repository tests in `corepack pnpm check`; no extra committed UAT fixture was needed for the happy-path official CSV import. |

### Known Limitations

- The fixture path uses CSV rows with no defensible observation geometry; expected location precision is `unknown`.
- The Phase 2 UAT does not validate live MLIT API retrieval.
- XPT001 station-context geometry and map rendering are deferred.
- Raw payload JSON should not be exposed through default product GraphQL queries.

---

## 3. Acceptance Criteria Traceability

| UAT ID | Related Acceptance Criteria | Scenario | Required Result |
|---|---|---|---|
| UAT-01 | Product, Engineering | Fixture import command | `./scripts/import-fixture.sh` imports MLIT fixture records and completes with understandable counters. |
| UAT-02 | Product, Engineering | GraphQL inspection | Imported observations, import runs, validation issues, and provenance summaries are queryable through bounded GraphQL. |
| UAT-03 | Engineering | Raw-record preservation | Each normalized observation links to a raw record, import run, dataset, and data source. |
| UAT-04 | Product, Engineering | Validation visibility | Warnings and rejected records are stored with issue code, severity, field, message, and disposition. |
| UAT-05 | Engineering | Duplicate-safe rerun | Running the same import twice creates no unintended duplicate observations. |
| UAT-06 | Engineering | Failed import visibility and retry | A failed import run is recorded, and a later valid import can complete without manual cleanup. |
| UAT-07 | Product, Data Integrity | Honest location precision | CSV fixture observations are `unknown` location precision and do not expose exact or station-point geometry. |
| UAT-08 | Documentation | Operator/developer usability | Importer docs explain commands, validation, idempotency, counters, limitations, and troubleshooting. |

---

## 4. UAT Test Cases

### UAT-01 — Fixture Import Completes

**Purpose**

Verify that the stable local command imports the committed MLIT fixture into the local database.

**Preconditions**

- [ ] Docker Compose stack is running and healthy.
- [ ] Migrations are applied.
- [ ] `scripts/import-fixture.sh` exists and is executable.

**Steps**

1. Start or refresh the local stack with the documented Compose command.
2. Run `./scripts/import-fixture.sh`.
3. Record the importer summary output and final process exit code.
4. Query `import_runs` or GraphQL import-run inspection for the latest run.

**Expected Result**

- [x] Command exits `0`.
- [x] Latest import run is `completed` or `completed_with_warnings`.
- [x] Counters include received, imported/updated/skipped, rejected, and warning records.
- [x] No full raw payloads or secrets are printed to the terminal.

**Actual Result**

Passed. Isolated stack fixture import completed with:

```text
summary source=mlit prefecture=13 period=2024Q4 artifacts=3 normalization_version=mlit-transaction-csv-v1 received=666 imported=666 updated=0 duplicates_skipped=0 rejected=0 warning_records=0 status=completed
```

**Status:** `passed`

**Evidence**

EV-01.

---

### UAT-02 — Imported Records Are Inspectable Through GraphQL

**Purpose**

Verify that imported data is available through the intended API boundary rather than direct database-only inspection.

**Preconditions**

- [ ] UAT-01 has completed successfully.
- [ ] API is healthy and ready.

**Steps**

1. Send a bounded GraphQL query for transaction observations.
2. Send a bounded GraphQL query for import runs and counters.
3. Send a bounded GraphQL query for validation issues.
4. Inspect one observation's provenance summary.

**Expected Result**

- [x] Observation results include ID, period, asset type, units, ward code, station context fields, validation state, location precision, and provenance IDs.
- [x] Import-run results include status, timestamps, normalization version, and counters.
- [x] Validation issue results include code, severity, field, message, and disposition.
- [x] Queries require a strict limit or pagination.
- [x] Raw payload JSON is not returned by default.

**Actual Result**

Passed. Bounded GraphQL query returned observations, import-run counters,
`validationIssues: []`, and MLIT data-source metadata. Sample observations
included `transactionYear: 2024`, `transactionQuarter: 4`, explicit JPY and m2
fields, lineage IDs, `validationStatus: "valid"`, and
`locationPrecision: "unknown"`.

**Status:** `passed`

**Evidence**

EV-03.

---

### UAT-03 — Raw Records and Lineage Are Preserved

**Purpose**

Verify that normalized observations remain traceable to source evidence.

**Preconditions**

- [ ] UAT-01 has completed successfully.

**Steps**

1. Select one imported observation.
2. Follow its provenance to raw record, import run, dataset, and data source.
3. Confirm the raw record has source position, payload hash, external ID if available, and validation status.
4. Confirm the dataset has fixture artifact metadata such as source/query/checksum/format.

**Expected Result**

- [x] Observation has exactly one originating raw record.
- [x] Raw record links to the same import run and dataset lineage.
- [x] Dataset links to the MLIT data source.
- [x] Raw payload is durably stored in the database but not exposed by default GraphQL product queries.

**Actual Result**

Passed. `transactionObservationProvenance` returned observation ID, raw-record
ID, source position, payload SHA-256, completed import-run ID/status, dataset
ID/name/retrieval method/source version/artifact SHA-256, and data-source name.
No `payload_json` field was exposed.

**Status:** `passed`

**Evidence**

EV-03, EV-05.

---

### UAT-04 — Validation Warnings and Rejections Are Visible

**Purpose**

Verify that imperfect source data is handled explicitly.

**Preconditions**

- [ ] Fixture import supports at least one warning case from the real fixture or a committed edge fixture.
- [ ] Rejection behavior is testable through a committed invalid fixture or controlled test command.

**Steps**

1. Import fixture data containing warning cases.
2. Import or test fixture data containing rejection cases.
3. Query validation issues by latest import run.
4. Inspect affected raw-record and observation linkage.

**Expected Result**

- [x] Warning cases create observations when safe and store warning issues.
- [x] Rejected cases preserve raw records and store rejection issues, but create no observations.
- [x] Issue records include code, severity, field, raw value summary, message, and disposition.
- [x] Import-run counters reflect warning and rejected records.

**Actual Result**

Passed through automated validation evidence. The committed official fixtures
had no warnings or rejections, and GraphQL returned `validationIssues: []` for
the imported stack. `corepack pnpm check` covered warning/rejection behavior in
parser and persistence tests, including invalid values, unknown asset labels,
rejected required identity/period rows, issue storage fields, counters, and
failed-run visibility.

**Status:** `passed`

**Evidence**

EV-04, EV-06.

---

### UAT-05 — Repeated Import Is Idempotent

**Purpose**

Verify that repeat fixture imports do not create duplicate observations.

**Preconditions**

- [ ] UAT-01 has completed successfully once.

**Steps**

1. Record current counts for datasets, import runs, raw records, observations, and validation issues.
2. Run `./scripts/import-fixture.sh` again with the same fixture bytes and options.
3. Record counts again.
4. Inspect the second import-run counters.

**Expected Result**

- [x] No unintended duplicate observations are created.
- [x] Raw-record behavior matches the implemented idempotency contract: reused/skipped/updated without duplicate source-position rows for the same dataset artifact.
- [x] Second import-run counters clearly show duplicates skipped and/or records updated.
- [x] Legitimate identical payloads at different source positions remain distinct.

**Actual Result**

Passed. The second fixture import completed with:

```text
summary source=mlit prefecture=13 period=2024Q4 artifacts=3 normalization_version=mlit-transaction-csv-v1 received=666 imported=0 updated=0 duplicates_skipped=666 rejected=0 warning_records=0 status=completed
```

Counts after first import plus duplicate rerun were 3 datasets, 6 import runs,
666 raw records, and 666 transaction observations.

**Status:** `passed`

**Evidence**

EV-02, EV-05.

---

### UAT-06 — Failed Import Is Visible and Retryable

**Purpose**

Verify that importer failure is observable and does not poison later imports.

**Preconditions**

- [ ] There is a documented way to force a controlled importer failure without damaging unrelated data.

**Steps**

1. Run the importer with an invalid fixture path, invalid fixture, or controlled failure option.
2. Confirm the command exits non-zero.
3. Query latest import run.
4. Run `./scripts/import-fixture.sh` with the valid fixture.
5. Query latest successful import run.

**Expected Result**

- [x] Failed run is stored with `status='failed'`, completed timestamp, error kind, and any counters known before failure.
- [x] Failure output is readable and does not expose internals or raw payload dumps.
- [x] Later valid import completes without manual database cleanup.

**Actual Result**

Passed. A temporary isolated-database trigger forced raw-record insert failure.
The importer exited non-zero with `import failed: import persistence failed`.
The database recorded:

```text
failed|persistence_error|0|0|0|0|0|2024Q4-UAT-FAIL
```

After removing the trigger, the normal fixture import completed successfully
with 666 duplicates skipped.

**Status:** `passed`

**Evidence**

EV-07.

---

### UAT-07 — Location Precision Is Honest

**Purpose**

Verify that Phase 2 does not imply false exact locations.

**Preconditions**

- [ ] UAT-01 has completed successfully.

**Steps**

1. Query imported observations' location precision values.
2. Query any stored location geometry/context fields.
3. Inspect GraphQL observation output for location representation.

**Expected Result**

- [x] CSV fixture observations have `location_precision=unknown`.
- [x] No exact property point is stored or returned.
- [x] No nearest-station point is assigned from CSV fixture rows.
- [x] Observations remain eligible for non-spatial inspection despite unknown geometry.

**Actual Result**

Passed. SQL count check returned `unknown_location_contexts=666`, where every
counted location context had `location_precision='unknown'` and `location IS
NULL`. GraphQL sample observations returned `locationPrecision: "unknown"`.

**Status:** `passed`

**Evidence**

EV-03, EV-05.

---

### UAT-08 — Importer Documentation Supports the Workflow

**Purpose**

Verify that another developer can understand and repeat the importer workflow.

**Preconditions**

- [ ] `docs/importer.md` exists.
- [ ] Any README/local-development updates are complete.

**Steps**

1. Read `docs/importer.md` from top to bottom.
2. Follow the documented setup and fixture import commands.
3. Confirm docs explain validation states, issue codes, counters, idempotency, fixture provenance, and known limitations.

**Expected Result**

- [x] Docs are sufficient to run the fixture import locally.
- [x] Docs explain what the importer does and does not claim.
- [x] Docs do not require or reveal an MLIT API key for fixture import.
- [x] Docs point to source/data-model limitations for location precision and unit-price behavior.

**Actual Result**

Passed. `docs/importer.md`, `docs/data-model.md`, `docs/data-sources.md`,
`docs/local-development.md`, and `README.md` document the command path,
fixture boundary, validation/idempotency model, counters, GraphQL inspection,
and unknown-location limitation without requiring an MLIT API key.

**Status:** `passed`

**Evidence**

EV-01, EV-02, EV-03, EV-06.

---

## 5. Failure and Edge-Case Validation

| UAT ID | Scenario | Expected Behavior | Actual Result | Status |
|---|---|---|---|---|
| UAT-E01 | Controlled persistence failure | Import run fails visibly with error kind; no partial observations are silently marked complete. | Temporary trigger forced `persistence_error`; failed run was stored and later normal retry completed. | `passed` |
| UAT-E02 | Negative price | Raw record is preserved, rejection issue is stored, and no observation is created. | Covered by parser/repository tests in `corepack pnpm check`. | `passed` |
| UAT-E03 | Invalid transaction quarter | Raw record is preserved, rejection issue is stored, and no invented date is used. | Covered by parser tests in `corepack pnpm check`. | `passed` |
| UAT-E04 | Unknown asset type | Source label is preserved, warning is stored, and behavior follows the validation rule. | Covered by parser tests in `corepack pnpm check`. | `passed` |
| UAT-E05 | Missing floor area | Observation remains valid with warning when otherwise safe; area is null, not defaulted. | Covered by parser tests in `corepack pnpm check`. | `passed` |
| UAT-E06 | Duplicate source payload at different row positions | Rows remain distinct observations because source position differs. | Covered by repository tests and exact-artifact/source-position idempotency model. | `passed` |

---

## 6. Data Integrity Validation

| Check | Expected Result | Actual Result | Status |
|---|---|---|---|
| Source lineage | Normalized data links to source, dataset, import run, and raw record. | Provenance GraphQL returned raw-record, import-run, dataset, artifact checksum, and source identity for sampled observation. | `passed` |
| Idempotency | Re-running the same import creates no unintended duplicate observations. | Duplicate rerun skipped 666 rows; counts remained 666 raw records and 666 observations. | `passed` |
| Validation visibility | Warnings and rejected records are stored and visible. | GraphQL exposes bounded validation issues; test suite covered warning/rejection issue persistence. | `passed` |
| Location precision | CSV fixture observations remain `unknown` and unmapped. | SQL returned `unknown_location_contexts=666`; GraphQL samples returned `locationPrecision: "unknown"`. | `passed` |
| Unit preservation | MLIT source unit price is stored only when supplied by the source. | Parser tests covered source-only unit price preservation; GraphQL sample showed one land unit price and one null condominium unit price. | `passed` |
| Raw preservation | Raw payload JSON and payload hash are stored for received rows. | Counts returned 666 raw records; provenance exposed payload SHA-256 but not raw payload JSON. | `passed` |
| Counter consistency | Import-run counters match raw/observation/issue counts. | First import 666 imported; rerun 666 skipped; SQL counts matched expected records and observations. | `passed` |

---

## 7. Evidence Register

| Evidence ID | Type | Description | Location |
|---|---|---|---|
| EV-01 | Command output | First fixture import summary: 3 artifacts, 666 received, 666 imported, 0 rejected, 0 warning records, status completed. | Terminal output from isolated `urbanlens_slice6_uat` run. |
| EV-02 | Command output | Duplicate rerun summary: 666 received, 0 imported, 666 duplicates skipped, status completed. | Terminal output from isolated `urbanlens_slice6_uat` run. |
| EV-03 | GraphQL response | Bounded observation/import-run/source query and provenance query returned expected lineage and no raw payload JSON. | `http://127.0.0.1:18081/graphql` during isolated UAT. |
| EV-04 | GraphQL or SQL response | Validation issue inspection returned an empty list for clean official fixtures; warning/rejection behavior covered by automated tests. | GraphQL plus `corepack pnpm check`. |
| EV-05 | SQL response | Count check returned `data_sources=1`, `datasets=3`, `import_runs=6`, `raw_records=666`, `transaction_observations=666`, `validation_issues=0`, `unknown_location_contexts=666`. | Isolated Compose PostgreSQL. |
| EV-06 | Test output | `corepack pnpm check` passed Rust format/lint/tests, importer parser/repository/CLI tests, API GraphQL tests, web lint/typecheck, and Vitest. Web production build also passed. | Local terminal output on `2026-07-02`. |
| EV-07 | Log excerpt | Controlled failed import recorded `failed|persistence_error|0|0|0|0|0|2024Q4-UAT-FAIL`; retry after trigger removal completed with 666 duplicates skipped. | Isolated Compose PostgreSQL and importer output. |

---

## 8. Defects Found

| Defect ID | Severity | Description | Reproduction Steps | Owner | Status |
|---|---|---|---|---|---|
| — | — | No defects recorded yet. | — | — | — |

### Severity Guide

| Severity | Meaning |
|---|---|
| Critical | Core phase outcome cannot be used, data integrity is at risk, or a security issue exists. |
| High | Major workflow is broken or misleading with no reasonable workaround. |
| Medium | Important issue exists but there is a reasonable workaround. |
| Low | Cosmetic, minor usability, or non-blocking issue. |

---

## 9. UAT Summary

| Metric | Count |
|---|---:|
| Total UAT Cases | 8 |
| Passed | 8 |
| Failed | 0 |
| Blocked | 0 |
| Not Run | 0 |
| Edge Cases | 6 |

### Final Result

`passed`

### Summary Notes

Phase 02 UAT passed on `2026-07-02`. The committed MLIT fixture import is
repeat-safe, inspectable through bounded GraphQL, traceable to raw/source
lineage, honest about unknown location precision, and retryable after a
controlled persistence failure. No blocking defects were found.
