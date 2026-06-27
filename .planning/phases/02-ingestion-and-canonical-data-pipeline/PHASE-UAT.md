# Phase 02 UAT — Ingestion and Canonical Data Pipeline

> Purpose: Verify that Phase 2 proves trustworthy official-source ingestion, not merely that importer code runs.

## Metadata

| Field | Value |
|---|---|
| Phase | `02` |
| Name | `Ingestion and Canonical Data Pipeline` |
| UAT Status | `not_started` |
| Environment | `local` |
| Tester | `Project owner` |
| Started | `not_started` |
| Completed | `not_started` |
| Build / Commit | `f72e09e` |
| Related Plan | `PHASE-PLAN.md` |
| Related Status | `PHASE-STATUS.md` |

---

## 1. UAT Objective

Verify that the importer can load the official-source MLIT transaction fixture, preserve raw records, create normalized transaction observations, record validation warnings/rejections, expose provenance through GraphQL, and avoid unintended duplicates when run repeatedly.

---

## 2. Preconditions

### Required Setup

- [ ] Correct branch is checked out.
- [ ] Working tree contains no unrelated uncommitted changes that would affect ingestion.
- [ ] Docker Compose is available.
- [ ] Local PostgreSQL/PostGIS stack can start through `docker compose up --build`.
- [ ] Required migrations have been applied.
- [ ] MLIT fixture CSVs are available under `workers/importer/fixtures/transactions/`.
- [ ] No MLIT API key is required for fixture UAT.

### Test Data

| Data Set / Fixture | Purpose | Setup Command / Location |
|---|---|---|
| MLIT 2024 Q4 Chuo fixture | Official-source CSV parser and import coverage | `workers/importer/fixtures/transactions/mlit-reinfolib-chuo-2024-q4.csv` |
| MLIT 2024 Q4 Shinagawa fixture | Larger ward fixture for duplicate/idempotency checks | `workers/importer/fixtures/transactions/mlit-reinfolib-shinagawa-2024-q4.csv` |
| MLIT 2024 Q4 Shibuya fixture | Additional ward fixture for parser and counter coverage | `workers/importer/fixtures/transactions/mlit-reinfolib-shibuya-2024-q4.csv` |
| Invalid/rejection fixture | Negative price, invalid date, missing required source identity, invalid geometry cases | To be added during implementation as a small committed test fixture or generated in tests |

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

- [ ] Command exits `0`.
- [ ] Latest import run is `completed` or `completed_with_warnings`.
- [ ] Counters include received, imported/updated/skipped, rejected, and warning records.
- [ ] No full raw payloads or secrets are printed to the terminal.

**Actual Result**

Not run.

**Status:** `not_run`

**Evidence**

Add command output excerpt and GraphQL/database query result.

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

- [ ] Observation results include ID, period, asset type, units, ward code, station context fields, validation state, location precision, and provenance IDs.
- [ ] Import-run results include status, timestamps, normalization version, and counters.
- [ ] Validation issue results include code, severity, field, message, and disposition.
- [ ] Queries require a strict limit or pagination.
- [ ] Raw payload JSON is not returned by default.

**Actual Result**

Not run.

**Status:** `not_run`

**Evidence**

Add GraphQL request/response excerpts.

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

- [ ] Observation has exactly one originating raw record.
- [ ] Raw record links to the same import run and dataset lineage.
- [ ] Dataset links to the MLIT data source.
- [ ] Raw payload is durably stored in the database but not exposed by default GraphQL product queries.

**Actual Result**

Not run.

**Status:** `not_run`

**Evidence**

Add database or GraphQL provenance excerpts.

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

- [ ] Warning cases create observations when safe and store warning issues.
- [ ] Rejected cases preserve raw records and store rejection issues, but create no observations.
- [ ] Issue records include code, severity, field, raw value summary, message, and disposition.
- [ ] Import-run counters reflect warning and rejected records.

**Actual Result**

Not run.

**Status:** `not_run`

**Evidence**

Add validation issue query excerpts.

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

- [ ] No unintended duplicate observations are created.
- [ ] Raw-record behavior matches the implemented idempotency contract: reused/skipped/updated without duplicate source-position rows for the same dataset artifact.
- [ ] Second import-run counters clearly show duplicates skipped and/or records updated.
- [ ] Legitimate identical payloads at different source positions remain distinct.

**Actual Result**

Not run.

**Status:** `not_run`

**Evidence**

Add before/after count table and second-run summary output.

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

- [ ] Failed run is stored with `status='failed'`, completed timestamp, error kind, and any counters known before failure.
- [ ] Failure output is readable and does not expose internals or raw payload dumps.
- [ ] Later valid import completes without manual database cleanup.

**Actual Result**

Not run.

**Status:** `not_run`

**Evidence**

Add failed-run and retry-run excerpts.

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

- [ ] CSV fixture observations have `location_precision=unknown`.
- [ ] No exact property point is stored or returned.
- [ ] No nearest-station point is assigned from CSV fixture rows.
- [ ] Observations remain eligible for non-spatial inspection despite unknown geometry.

**Actual Result**

Not run.

**Status:** `not_run`

**Evidence**

Add GraphQL/database precision excerpts.

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

- [ ] Docs are sufficient to run the fixture import locally.
- [ ] Docs explain what the importer does and does not claim.
- [ ] Docs do not require or reveal an MLIT API key for fixture import.
- [ ] Docs point to source/data-model limitations for location precision and unit-price behavior.

**Actual Result**

Not run.

**Status:** `not_run`

**Evidence**

Add doc review notes and command evidence.

---

## 5. Failure and Edge-Case Validation

| UAT ID | Scenario | Expected Behavior | Actual Result | Status |
|---|---|---|---|---|
| UAT-E01 | Invalid fixture path | Import run fails visibly with error kind; no partial observations are silently marked complete. |  | `not_run` |
| UAT-E02 | Negative price | Raw record is preserved, rejection issue is stored, and no observation is created. |  | `not_run` |
| UAT-E03 | Invalid transaction quarter | Raw record is preserved, rejection issue is stored, and no invented date is used. |  | `not_run` |
| UAT-E04 | Unknown asset type | Source label is preserved, warning is stored, and behavior follows the validation rule. |  | `not_run` |
| UAT-E05 | Missing floor area | Observation remains valid with warning when otherwise safe; area is null, not defaulted. |  | `not_run` |
| UAT-E06 | Duplicate source payload at different row positions | Rows remain distinct observations because source position differs. |  | `not_run` |

---

## 6. Data Integrity Validation

| Check | Expected Result | Actual Result | Status |
|---|---|---|---|
| Source lineage | Normalized data links to source, dataset, import run, and raw record. |  | `not_run` |
| Idempotency | Re-running the same import creates no unintended duplicate observations. |  | `not_run` |
| Validation visibility | Warnings and rejected records are stored and visible. |  | `not_run` |
| Location precision | CSV fixture observations remain `unknown` and unmapped. |  | `not_run` |
| Unit preservation | MLIT source unit price is stored only when supplied by the source. |  | `not_run` |
| Raw preservation | Raw payload JSON and payload hash are stored for received rows. |  | `not_run` |
| Counter consistency | Import-run counters match raw/observation/issue counts. |  | `not_run` |

---

## 7. Evidence Register

| Evidence ID | Type | Description | Location |
|---|---|---|---|
| EV-01 | Command output | First fixture import summary | TBD |
| EV-02 | Command output | Duplicate rerun summary | TBD |
| EV-03 | GraphQL response | Observation/provenance inspection | TBD |
| EV-04 | GraphQL or SQL response | Validation issue inspection | TBD |
| EV-05 | SQL response | Raw-record and lineage count checks | TBD |
| EV-06 | Test output | Automated parser/validation/idempotency tests | TBD |
| EV-07 | Log excerpt | Controlled failed import and retry | TBD |

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
| Passed | 0 |
| Failed | 0 |
| Blocked | 0 |
| Not Run | 8 |
| Edge Cases | 6 |

### Final Result

`not_started`

### Summary Notes

UAT protocol is ready as a target. Execute only after Slice 6 marks the phase `ready_for_uat`.
