# Phase 03 UAT — Spatial Data Model and Query Engine

> Purpose: Verify that Phase 03 makes the backend location-aware through trustworthy PostGIS behavior, not merely that spatial fields exist.

## Metadata

| Field | Value |
|---|---|
| Phase | `03` |
| Name | `Spatial Data Model and Query Engine` |
| UAT Status | `in_progress` |
| Environment | `local` |
| Tester | `Project owner` |
| Started | `not_started` |
| Completed | `not_started` |
| Build / Commit | `TBD` |
| Related Plan | `PHASE-PLAN.md` |
| Related Status | `PHASE-STATUS.md` |

---

## 1. UAT Objective

Verify that UrbanLens can load official Tokyo ward boundaries into PostGIS, use spatial indexes, run bounded database-level ward and viewport queries, expose spatial behavior through GraphQL, and make location precision visible without overstating the MLIT CSV observations.

---

## 2. Preconditions

### Required Setup

- [ ] Correct branch is checked out.
- [ ] Docker Compose is available.
- [ ] Local PostgreSQL/PostGIS stack can start through the documented Compose command.
- [x] Required migrations have been applied.
- [x] Official ward boundary fixture is available and documented.
- [ ] MLIT transaction fixtures are available under `workers/importer/fixtures/transactions/`.
- [ ] No MLIT API key is required for required Phase 03 UAT.

### Test Data

| Data Set / Fixture | Purpose | Setup Command / Location |
|---|---|---|
| Official Tokyo ward boundary fixture | Import 23 governed ward polygons into PostGIS. | `workers/importer/fixtures/boundaries/mlit-n03-tokyo-23-wards-2023.geojson`; validate with `bash scripts/validate-boundary-fixture.sh` |
| MLIT 2024 Q4 transaction fixtures | Regression data for canonical observations and ward/filter behavior. | `workers/importer/fixtures/transactions/` |
| Controlled spatial query fixture | Tests viewport/proximity behavior with defensible point geometry if needed. | `TBD during Slice 4` |

### Known Limitations

- Phase 02 CSV observations are expected to remain `location_precision=unknown` with null geometry.
- Viewport point queries may return an honest empty result until a defensible point source or controlled test fixture is present.
- Ward polygons are for area selection and aggregation; they do not represent individual transaction locations.
- Frontend map rendering is not required for Phase 03 UAT.

---

## 3. Acceptance Criteria Traceability

| UAT ID | Related Acceptance Criteria | Scenario | Required Result |
|---|---|---|---|
| UAT-01 | Product, Engineering, Documentation | Boundary source and import | Official Tokyo ward boundaries load into PostGIS with documented source lineage. |
| UAT-02 | Engineering | Spatial indexes | Required GiST/btree indexes exist and are used by query paths where practical. |
| UAT-03 | Product, Engineering | Viewport filtering | A bounded viewport query returns only matching defensible-geometry observations. |
| UAT-04 | Product, Engineering | Ward filtering/counts | Ward query and counts run through database queries and return expected ward-scoped results. |
| UAT-05 | Product, Engineering | GraphQL spatial API | GraphQL exposes bounded `areas`, `transactionObservations`, and `marketMapViewport` behavior. |
| UAT-06 | Data Integrity | Location transparency | Geometry-bearing responses expose `locationPrecision`, `locationLabel`, and `locationDisclaimer`. |
| UAT-07 | Regression | Phase 02 import/provenance | Existing fixture import, provenance, and unknown-location behavior still work. |
| UAT-08 | Documentation | Spatial strategy docs | Docs explain source, query strategy, indexes, precision limits, and local validation. |

---

## 4. UAT Test Cases

### UAT-01 — Tokyo Ward Boundaries Load

**Purpose**

Verify that the selected official boundary fixture imports into governed area tables.

**Preconditions**

- [x] Boundary source documentation is complete.
- [ ] Boundary migration has run.
- [x] Boundary importer command or script exists.

**Steps**

1. Start a fresh local Compose stack.
2. Run the boundary import command or script.
3. Query `areas` and `area_boundaries` for ward rows.
4. Confirm geometry SRID/type and source lineage fields.

**Expected Result**

- [x] Import exits `0`.
- [x] Exactly 23 Tokyo special wards are present unless the selected official source documents a different scoped fixture.
- [x] Boundary geometries are SRID 4326 polygon/multipolygon values.
- [x] Each boundary has administrative code, Japanese/English label where available, source record hash, and source lineage.

**Actual Result**

On `2026-07-02`, isolated Compose stack
`COMPOSE_PROJECT_NAME=urbanlens_slice3_smoke` passed migrations and schema
smoke. First `./scripts/import-boundaries.sh` run imported 118 source features
into 23 ward boundaries. Second run skipped 118 duplicate raw features and
updated the same 23 boundary rows. SQL count check returned 23 `areas`, 23
`area_boundaries`, 118 `raw_records`, 23 distinct ward codes, 23 valid SRID
4326 multipolygons, and 0 transaction location geometries.

**Status:** `passed`

**Evidence**

Evidence:

```text
summary source=mlit-n03 boundary_version=2023-01-01 source_features=118 wards=23 normalization_version=mlit-n03-boundary-geojson-v1 received=118 imported=23 updated=0 duplicates_skipped=0 rejected=0 warning_records=0 status=completed
summary source=mlit-n03 boundary_version=2023-01-01 source_features=118 wards=23 normalization_version=mlit-n03-boundary-geojson-v1 received=118 imported=0 updated=23 duplicates_skipped=118 rejected=0 warning_records=0 status=completed
areas=23 boundaries=23 raw_records=118 ward_codes=23 valid_multipolygons=23 transaction_locations=0
```

---

### UAT-02 — Spatial Indexes Exist

**Purpose**

Verify that spatial storage is query-ready and not relying on sequential application-side filtering.

**Preconditions**

- [ ] UAT-01 setup has completed.
- [ ] Migrations have run on a fresh volume.

**Steps**

1. Inspect indexes for `area_boundaries.geometry`.
2. Inspect indexes for `transaction_location_contexts.location`.
3. Inspect supporting filter indexes for ward/period/asset/price/area/walk-time fields.
4. Optionally run `EXPLAIN` for representative spatial queries.

**Expected Result**

- [ ] GiST spatial index exists for area boundary geometry.
- [ ] GiST spatial index exists for transaction location geometry.
- [ ] Supporting filter indexes exist for common bounded filters.
- [ ] Representative query plans use database spatial predicates.

**Actual Result**

Fill in during UAT.

**Status:** `not_run`

**Evidence**

Add index inspection output and, if used, bounded `EXPLAIN` output.

---

### UAT-03 — Viewport Query Is Bounded and Spatial

**Purpose**

Verify that viewport filtering is performed in PostGIS and returns only matching observations with defensible geometry.

**Preconditions**

- [ ] API is healthy.
- [ ] Controlled point fixture or defensible point observations exist if the selected Phase 03 implementation includes mappable observations.

**Steps**

1. Send a `marketMapViewport` query with bounds containing at least one known mappable point.
2. Send the same query with bounds excluding that point.
3. Confirm result caps/pagination behavior.
4. Confirm unknown/null-location Phase 02 CSV observations are not returned as map points.

**Expected Result**

- [ ] Included bounds return only records or aggregates inside the viewport.
- [ ] Excluding bounds return no matching point result for the controlled point.
- [ ] Results are bounded by a strict limit or aggregate cap.
- [ ] Unknown-location observations are absent from point-map results and are explained where relevant.

**Actual Result**

Fill in during UAT.

**Status:** `not_run`

**Evidence**

Add GraphQL request/response excerpts and test output.

---

### UAT-04 — Ward Filtering and Counts Run in the Database

**Purpose**

Verify that ward selection and counts use database queries and produce ward-scoped results.

**Preconditions**

- [ ] Ward boundaries are imported.
- [ ] MLIT transaction fixture import has completed.

**Steps**

1. Run the MLIT transaction fixture import.
2. Query observations or counts for one known fixture ward, such as Chuo, Shinagawa, or Shibuya.
3. Query counts by ward.
4. Confirm SQL/resolver path uses database filters or joins rather than fetching all records into application memory.

**Expected Result**

- [ ] Selected-ward query returns only the requested ward's observations or aggregate counts.
- [ ] Counts by ward match the known fixture distribution for Chuo, Shinagawa, and Shibuya when using the Phase 02 fixture.
- [ ] Query semantics clearly state whether the result uses source ward code or polygon containment.
- [ ] No unbounded observation list is fetched to compute the result in application memory.

**Actual Result**

Fill in during UAT.

**Status:** `not_run`

**Evidence**

Add GraphQL/SQL output, known count comparison, and relevant test names.

---

### UAT-05 — GraphQL Spatial API Is Product-Ready

**Purpose**

Verify that spatial behavior is available through the API contract future UI work will use.

**Preconditions**

- [ ] API is healthy and migrations have run.
- [ ] Boundary and transaction fixtures are imported as needed.

**Steps**

1. Query `areas(filter: { areaType: WARD })`.
2. Query `transactionObservations(filter: ..., page: ...)` with period, asset-type, price, floor-area, walk-time, and ward filters.
3. Query `marketMapViewport(bounds: ..., filter: ...)`.
4. Try invalid bounds and excessive limits.

**Expected Result**

- [ ] `areas` returns governed ward data.
- [ ] `transactionObservations` requires pagination and returns a connection shape.
- [ ] `marketMapViewport` is bounded and validates coordinates.
- [ ] Invalid inputs produce readable GraphQL errors.
- [ ] Raw payload JSON is not exposed in default spatial query responses.

**Actual Result**

Fill in during UAT.

**Status:** `not_run`

**Evidence**

Add GraphQL response excerpts and schema/test evidence.

---

### UAT-06 — Location Precision Is Visible and Honest

**Purpose**

Verify that geometry-bearing API responses do not imply exact property coordinates.

**Preconditions**

- [ ] Spatial GraphQL API is implemented.
- [ ] At least one ward boundary response is available.
- [ ] Unknown-location transaction fixture data is imported.

**Steps**

1. Query an area/boundary or viewport result that includes geometry.
2. Inspect `locationPrecision`, `locationLabel`, and `locationDisclaimer`.
3. Query a Phase 02 CSV observation.
4. Confirm it remains `unknown` with no point geometry.

**Expected Result**

- [ ] Ward geometry is labeled as `ward_polygon` or equivalent aggregate selection context.
- [ ] Disclaimer states the polygon does not represent an individual transaction location.
- [ ] CSV observations remain `unknown` and explain why they are absent from point maps.
- [ ] No response labels any selected-source geometry as `exact_point`.

**Actual Result**

Fill in during UAT.

**Status:** `not_run`

**Evidence**

Add GraphQL response excerpts.

---

### UAT-07 — Phase 02 Import Regression Still Passes

**Purpose**

Verify that spatial work did not break the ingestion/provenance foundation.

**Preconditions**

- [ ] Fresh or isolated Compose stack is available.
- [ ] MLIT fixture import script exists.

**Steps**

1. Run `./scripts/import-fixture.sh`.
2. Run it again to verify idempotency.
3. Query one observation's provenance.
4. Confirm raw-record, import-run, dataset, source, and location precision behavior.

**Expected Result**

- [ ] First import loads the committed fixture.
- [ ] Second import creates no unintended duplicate observations.
- [ ] Provenance remains queryable through GraphQL.
- [ ] CSV observation location precision remains `unknown`.

**Actual Result**

Fill in during UAT.

**Status:** `not_run`

**Evidence**

Add importer summaries and GraphQL excerpts.

---

### UAT-08 — Documentation Supports the Next Phase

**Purpose**

Verify that another developer can understand and validate the spatial backend.

**Preconditions**

- [ ] Phase 03 implementation and tests are complete.

**Steps**

1. Review `docs/spatial-query-strategy.md`.
2. Review `docs/adr/006-use-postgis-for-viewport-and-proximity-queries.md`.
3. Review updated source/model/local-development docs.
4. Follow the documented local validation commands.

**Expected Result**

- [ ] Docs identify the boundary source, license, retrieval method, and limitations.
- [ ] Docs explain viewport, proximity, ward, and metric query semantics.
- [ ] Docs explain precision labels and why Phase 02 CSV observations are not points.
- [ ] Local validation commands are accurate.

**Actual Result**

Fill in during UAT.

**Status:** `not_run`

**Evidence**

Add reviewed file paths and command outputs.

---

## 5. Failure and Edge-Case Validation

| UAT ID | Scenario | Expected Behavior | Actual Result | Status |
|---|---|---|---|---|
| UAT-E01 | Invalid map bounds, such as north below south or longitude outside range. | GraphQL rejects the request with a readable validation error. |  | `not_run` |
| UAT-E02 | Empty viewport or ward filter with no matching observations. | API returns an honest empty result with no fabricated metric value. |  | `not_run` |
| UAT-E03 | Boundary fixture rerun. | Importer skips or updates deterministically without duplicate areas/boundaries. |  | `not_run` |
| UAT-E04 | Invalid or wrong-SRID boundary geometry in a test fixture. | Import fails or records a clear validation issue; invalid geometry is not persisted as valid. |  | `not_run` |
| UAT-E05 | Excessive page size or viewport limit. | API clamps or rejects according to documented limit behavior. |  | `not_run` |
| UAT-E06 | Unknown-location observation requested in viewport point query. | Observation is excluded from point results and remains available through non-spatial/ward-code queries where appropriate. |  | `not_run` |

---

## 6. Data Integrity Validation

| Check | Expected Result | Actual Result | Status |
|---|---|---|---|
| Boundary source lineage | Ward boundaries link to documented official source metadata and source-record hash. |  | `not_run` |
| Idempotency | Re-running boundary import creates no duplicate areas or boundaries. |  | `not_run` |
| Geometry validity | Boundary geometries have SRID 4326, valid polygon/multipolygon type, and spatial index coverage. |  | `not_run` |
| Spatial filtering | Viewport and ward filters run in PostGIS/database queries. |  | `not_run` |
| Location precision | API does not imply exact location for ward polygons, nearest-station context, or unknown CSV observations. |  | `not_run` |
| Metric reproducibility | Area metrics match filters, time range, units, and sample size. |  | `not_run` |

---

## 7. Evidence Register

| Evidence ID | Type | Description | Location |
|---|---|---|---|
| EV-01 | Source docs | Official ward boundary source, license, and checksum. | `docs/data-sources.md` |
| EV-02 | Test output | Spatial schema/import/query tests. | `TBD` |
| EV-03 | Command output | Boundary import and MLIT fixture import summaries. | `TBD` |
| EV-04 | GraphQL output | `areas`, `transactionObservations`, and `marketMapViewport` examples. | `TBD` |
| EV-05 | SQL output | Index and geometry validation checks. | `TBD` |
| EV-06 | Documentation | Spatial strategy and ADR-006. | `docs/spatial-query-strategy.md`, `docs/adr/006-use-postgis-for-viewport-and-proximity-queries.md` |

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
| Open Critical Defects | 0 |
| Open High Defects | 0 |

### Final UAT Decision

- [ ] `passed` — All required UAT cases pass. No critical or high defects remain.
- [ ] `passed_with_accepted_exceptions` — Remaining issues are documented and accepted.
- [ ] `failed` — Required behavior is incomplete or blocking defects remain.
- [ ] `blocked` — UAT cannot continue because prerequisites are unavailable.

### Accepted Exceptions

| Exception | Reason | Follow-Up Phase / Issue |
|---|---|---|
| — | — | — |

---
