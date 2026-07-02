# Phase 03 Plan — Spatial Data Model and Query Engine

> Purpose: Make UrbanLens genuinely location-aware by adding governed Tokyo ward boundaries, PostGIS indexes, bounded spatial queries, and transparent location-precision API behavior.

## Metadata

| Field | Value |
|---|---|
| Phase | `03` |
| Name | `Spatial Data Model and Query Engine` |
| Status | `planning` |
| Owner | `Project owner` |
| Created | `2026-07-02` |
| Last Updated | `2026-07-02` |
| Target Milestone | `MVP spatial query foundation` |
| Related ADRs | `docs/adr/001-use-postgis-for-spatial-queries.md`, `docs/adr/002-use-graphql-for-product-api.md`, `docs/adr/003-preserve-raw-source-payloads.md`, `docs/adr/004-model-location-precision-explicitly.md`, `docs/adr/006-use-postgis-for-viewport-and-proximity-queries.md` |

---

## 1. Objective

### Problem

UrbanLens can ingest official transaction observations and preserve lineage, but the backend does not yet have a trustworthy spatial query model. The Phase 02 CSV observations are intentionally `location_precision=unknown`, so map and area workflows need authoritative boundary data, spatial indexes, and query contracts that do not invent point locations.

### Intended Outcome

Add official Tokyo ward boundary ingestion, physical `areas` / `area_boundaries` support where needed, PostGIS indexes, database-level viewport and area filtering, bounded GraphQL spatial queries, and visible location precision labels/disclaimers for every geometry-bearing response.

### Why This Matters

The first analyst workflow starts from a Tokyo map. Analysts need confidence that area counts, viewport results, and future map layers are filtered by the database's spatial engine and that any displayed geometry is labeled with what it actually represents.

---

## 2. Scope

### In Scope

- [ ] Select, document, and fixture a legally usable official Tokyo ward boundary source.
- [ ] Add or adapt `areas` and `area_boundaries` schema for ward identity, polygons, source lineage, and source-record hashes.
- [ ] Import the Tokyo 23 special ward boundaries into PostGIS through a small repeat-safe importer path.
- [ ] Add GiST spatial indexes for `area_boundaries.geometry` and mappable `transaction_location_contexts.location`.
- [ ] Implement SQLx query functions for ward containment, viewport intersection, proximity to a supplied point, counts by ward, and area-level aggregate metrics.
- [ ] Add bounded GraphQL queries: `areas`, `transactionObservations`, and `marketMapViewport`.
- [ ] Expose `locationPrecision`, `locationLabel`, and `locationDisclaimer` on geometry-bearing GraphQL records/results.
- [ ] Add tests for at least one spatial query and one boundary-based query.
- [ ] Add `docs/spatial-query-strategy.md` and `docs/adr/006-use-postgis-for-viewport-and-proximity-queries.md`.

### Out of Scope

- [ ] Exact property coordinates or geocoding MLIT district labels.
- [ ] Guessing XPT001 station points onto Phase 02 CSV/XIT001 observations.
- [ ] Canonical station identity, railway lines, or station master imports.
- [ ] Frontend map rendering beyond any narrow API smoke or schema proof.
- [ ] Production-scale tile serving, vector-tile generation, or a separate GIS/search service.
- [ ] Saved searches, authentication, scheduling, or advanced analytics.

### Deferred Ideas

- [ ] XPT001 nearest-station point ingestion as a separate raw-feature path.
- [ ] District boundaries or district centroids after a defensible official source is selected.
- [ ] Materialized metric tables for expensive area comparisons.
- [ ] Query-plan monitoring and spatial performance dashboards.
- [ ] Canonical station proximity queries based on station IDs rather than user-supplied points.

---

## 3. Requirements

### Functional Requirements

| ID | Requirement | Priority | Notes |
|---|---|---:|---|
| FR-01 | Import official Tokyo ward boundaries into PostGIS with one governed `ward` area per Tokyo special ward. | Must | Source must be documented before import code is considered complete. |
| FR-02 | Store boundary lineage: `source_id`, dataset/import context where practical, `source_record_hash`, source administrative code, and labels. | Must | Boundary features are official source records, not hand-drawn application data. |
| FR-03 | Add or adapt `areas` with `id`, `name_ja`, `name_en`, `area_type`, and `administrative_code`. | Must | If an `areas` table already exists, migrate additively and preserve compatibility. |
| FR-04 | Add `area_boundaries.geometry` as SRID 4326 polygon or multipolygon geometry with a spatial index. | Must | Geometry validity and SRID must be constrained or validated on import. |
| FR-05 | Add a spatial index for mappable transaction locations. | Must | Phase 02 observations remain null/unknown, but the index prepares the query engine for defensible point sources. |
| FR-06 | Implement database-level queries for observations in a ward, within a viewport, near a selected point, counts by ward, and area metrics. | Must | Filtering must happen in PostGIS/SQL, not in Rust or the browser after unbounded fetches. |
| FR-07 | Support filters for transaction period, price, floor area, walk time, and asset type across spatial queries. | Must | Reuse Phase 02 canonical fields and units. |
| FR-08 | Add bounded GraphQL spatial queries and preserve pagination or strict limits on every list. | Must | `transactionObservations` must require `page`; viewport results must have caps. |
| FR-09 | Expose location precision label/disclaimer on every API record/result that includes geometry. | Must | Prevents ward polygons or station-context points from reading as exact property points. |
| FR-10 | Document spatial query strategy and accept ADR-006. | Must | The ADR records how PostGIS is used for viewport/proximity behavior. |

### Non-Functional Requirements

| ID | Requirement | Priority | Verification Method |
|---|---|---:|---|
| NFR-01 | Spatial filtering and aggregation run in PostgreSQL/PostGIS. | Must | SQLx tests and code review. |
| NFR-02 | Spatial queries are bounded by pagination, strict result caps, or aggregate-only return shapes. | Must | GraphQL schema tests and resolver tests. |
| NFR-03 | Boundary import is idempotent and repeat-safe. | Must | Import test or rerun smoke evidence. |
| NFR-04 | Geometry uses SRID 4326 and has appropriate GiST indexes. | Must | Migration/schema smoke assertions. |
| NFR-05 | Location precision and disclaimers are deterministic, tested, and visible in GraphQL. | Must | GraphQL tests. |
| NFR-06 | Host Cargo remains optional in this environment. | Must | Use `corepack pnpm check` or Docker-backed Rust checks. |
| NFR-07 | Fixture data stays small enough for local and CI-style validation. | Should | Fixture size review and smoke runtime. |

### Data / Domain Requirements

| ID | Requirement | Source / Assumption | Notes |
|---|---|---|---|
| DR-01 | Use official, public, legally usable ward boundary data. | `AGENTS.md`, `docs/data-sources.md` | No scraped or private geodata. |
| DR-02 | Treat `area` as a governed geographic unit, not a property or listing. | `docs/data-model.md` | Initial supported `area_type` is `ward`. |
| DR-03 | Preserve Phase 02 CSV observations as `location_precision=unknown` with null geometry. | ADR-004, Phase 02 UAT | Ward labels may support non-spatial filtering, but not point plotting. |
| DR-04 | Ward boundaries can use `location_precision=ward_polygon` for aggregate/selection results only. | ADR-004 | Do not attach ward polygons as individual observation geometry. |
| DR-05 | Administrative code is stored as text. | Phase 02 municipality-code pattern | Avoid losing leading zeros or source-code formatting. |
| DR-06 | Area-level metrics must expose sample size, units, filters, period, and source limitations. | `docs/product-brief.md` | Phase 03 may return simple aggregates; Phase 05 deepens comparison UX. |

---

## 4. Technical Design

### Proposed Approach

Build the spatial foundation in small layers. First select and document the official boundary source and create a tiny committed fixture. Then add schema/index migrations and importer parsing in isolation. Once wards are in PostGIS, implement SQLx query helpers over the canonical transaction tables and boundary tables, with tests that use controlled geometries. Finally expose bounded GraphQL fields and document the query strategy.

The key boundary is honesty: Phase 03 may filter existing CSV observations by source ward code and join to ward areas for aggregate counts, but it must not render those observations as points or pretend that the CSV row has a defensible geometry.

### Components Affected

| Component | Planned Change | Reason |
|---|---|---|
| `apps/web` | No required product UI changes. | Phase 04 owns the analyst map UI. |
| `apps/api` | Add spatial SQLx query helpers and GraphQL queries/types/filters. | Product-facing spatial access belongs behind the API. |
| `workers/importer` | Add a small ward-boundary fixture importer or import command path. | Keeps official boundary data in the ingestion pipeline. |
| `database` | Add/adapt area tables, geometry constraints, spatial indexes, and supporting filter indexes. | PostGIS owns spatial filtering and aggregation. |
| `infra` | Reuse existing Compose/PostGIS services. | No new runtime service is needed. |
| `docs` | Add spatial query strategy, ADR-006, and source/model updates. | Spatial semantics and source limitations are product-critical. |

### Data Flow

```text
Official Tokyo ward boundary fixture
  ↓
Boundary importer with source lineage and geometry validation
  ↓
areas and area_boundaries in PostgreSQL/PostGIS
  ↓
SQLx spatial query functions with canonical transaction filters
  ↓
Bounded GraphQL spatial results with precision labels and disclaimers
```

### API / Interface Changes

| Type | Name | Change | Consumers |
|---|---|---|---|
| GraphQL Query | `areas(filter: AreaFilter): [Area!]!` | Return bounded/listed governed areas, initially Tokyo wards. | Future map filters and area comparison |
| GraphQL Query | `transactionObservations(filter: TransactionObservationFilter!, page: PageInput!): TransactionObservationConnection!` | Extend existing inspection query into the product spatial/filter contract. | Future transaction explorer and map side panel |
| GraphQL Query | `marketMapViewport(bounds: MapBoundsInput!, filter: TransactionObservationFilter!): MarketMapViewportResult!` | Return bounded viewport results and/or aggregate summaries with spatial filtering in PostGIS. | Future market map |
| GraphQL Input | `AreaFilter` | Filter by area type, administrative code, and text label where useful. | Area picker |
| GraphQL Input | `TransactionObservationFilter` | Add ward, viewport/proximity, period, price, floor area, walk time, and asset-type filters. | Product queries |
| GraphQL Input | `MapBoundsInput` | Validate north/south/east/west bounds. | Viewport query |
| CLI Command | `import-ward-boundaries` or importer subcommand | Import official ward boundary fixture into area tables. | Developer/UAT |
| Database Migration | `areas` / `area_boundaries` | Add/adapt ward identity, geometry, lineage fields, constraints, and indexes. | Importer/API |
| Documentation | `docs/spatial-query-strategy.md` | Explain query patterns, precision semantics, and limitations. | Developers/reviewers |

### Data Model Changes

| Entity / Table | Change | Migration Required | Notes |
|---|---|---:|---|
| `areas` | Add or adapt fields for `name_ja`, `name_en`, `area_type`, `administrative_code`, source lineage, and current boundary reference if useful. | Yes | Phase 01 may already contain an `areas` foundation table; migrate additively. |
| `area_boundaries` | New table if not already present. Store boundary geometry, source lineage, source record hash, effective/version context, and timestamps. | Yes | Keep boundary versions separate from area identity. |
| `transaction_location_contexts` | Add/verify GiST index on `location` for mappable observations. | Maybe | Existing unknown/null rows remain valid. |
| `transaction_observations` | Add/verify supporting btree indexes for ward code, period, asset type, price, area, and station walk filters. | Maybe | Keep filters database-backed. |
| `data_sources` / `datasets` / `raw_records` | Reuse lineage tables for boundary source artifacts if practical. | Maybe | Preserve raw boundary features where implementation scope allows. |

### Geographic / Data Precision Notes

- Location precision: existing CSV observations stay `unknown`; ward boundaries are `ward_polygon`; future XPT001 point features may become `nearest_station_point`.
- User-facing disclaimer for ward aggregate geometry: "Ward boundary shown for area selection and aggregation. It does not represent an individual transaction location."
- User-facing disclaimer for unknown observations: "This observation has ward/district labels but no defensible map geometry, so it is not shown as a point."
- Known data limitations: ward boundaries can change over time; source observation ward codes may not be sufficient for exact geometry; MLIT observations are incomplete survey-derived records.
- Assumptions: Initial Phase 03 metrics/counts may use source ward code for counts by ward while viewport point queries only include observations with non-null defensible geometry.

---

## 5. Implementation Slices

Each slice is deliberately small so spatial work teaches one concept at a time.

### Slice 1 — Boundary Source Decision and Fixture

**Goal**

Learn the boundary source before writing schema code: choose an official Tokyo ward boundary artifact, document its license and limitations, and create a small repeatable fixture path.

**Tasks**

- [ ] Identify the official boundary source and usage terms.
- [ ] Update `docs/data-sources.md` with source URL, publisher, license, retrieval method, date, version, update frequency, and limitations.
- [ ] Add a small committed fixture or documented fixture-generation path that covers the Tokyo 23 wards.
- [ ] Record fixture checksum, feature count, geometry type, encoding/format, CRS/SRID, and administrative-code field.
- [ ] Decide whether raw boundary features are stored in `raw_records` in Phase 03 or deferred with a documented reason.

**Expected Evidence**

- [ ] Source documentation is complete enough for a reviewer to verify legality and provenance.
- [ ] A test or script can read the fixture and confirm 23 ward features with SRID 4326-compatible geometry.

---

### Slice 2 — Area Schema and Spatial Indexes

**Goal**

Add the minimum database contracts for governed ward identity and spatial lookup.

**Tasks**

- [ ] Inspect the current `areas` table from Phase 01 and choose additive migrations.
- [ ] Add or adapt `areas` and `area_boundaries` fields for ward labels, administrative codes, source lineage, source record hash, and geometry.
- [ ] Add geometry validity, SRID, type, uniqueness, and lineage constraints where practical.
- [ ] Add GiST indexes for `area_boundaries.geometry` and `transaction_location_contexts.location`.
- [ ] Add btree indexes needed for spatial-filter combinations: ward code, period, asset type, price, floor area, and station walk time.
- [ ] Extend `scripts/smoke-compose.sh` schema assertions for the new tables/indexes.

**Expected Evidence**

- [ ] Fresh and existing databases migrate successfully.
- [ ] Smoke checks prove area tables and spatial indexes exist.
- [ ] Constraint checks reject invalid SRID or geometry type.

---

### Slice 3 — Ward Boundary Importer

**Goal**

Persist official ward boundaries repeatably before exposing any product queries.

**Tasks**

- [ ] Add parser/importer code for the selected boundary fixture.
- [ ] Normalize ward labels, area type, administrative code, geometry, source record hash, and source metadata.
- [ ] Upsert `areas` and `area_boundaries` idempotently.
- [ ] Preserve source lineage through `data_sources` / `datasets` / `import_runs` / `raw_records` where selected in Slice 1.
- [ ] Add importer tests for feature count, geometry validation, duplicate rerun behavior, and source-record hash stability.
- [ ] Add a stable local script if the command is not covered by `scripts/import-fixture.sh`.

**Expected Evidence**

- [ ] First run imports all Tokyo special wards.
- [ ] Second run does not create duplicate areas or boundary rows.
- [ ] Imported geometries can be queried by administrative code and label.

---

### Slice 4 — SQLx Spatial Query Functions

**Goal**

Prove the query engine in the database before adding GraphQL surface area.

**Tasks**

- [ ] Implement SQLx functions for observations in a ward.
- [ ] Implement viewport filtering with validated bounds and a strict result cap.
- [ ] Implement proximity filtering around a supplied point and radius.
- [ ] Implement counts by ward using boundary joins or source ward-code mapping, with the chosen method documented.
- [ ] Implement area-level aggregate metrics: count, median price, median area, median source price per m2 where eligible, and sample size.
- [ ] Compose period, price, floor area, walk time, and asset-type filters in SQL.
- [ ] Add tests with controlled points/polygons and imported fixtures.

**Expected Evidence**

- [ ] At least one viewport query returns only matching mappable observations.
- [ ] At least one ward query returns only observations in the selected ward.
- [ ] Tests demonstrate unknown/null-location observations are not returned as map points.

---

### Slice 5 — GraphQL Spatial API

**Goal**

Expose bounded spatial behavior through the product API with location transparency.

**Tasks**

- [ ] Add GraphQL types for `Area`, `AreaBoundarySummary`, `MarketMapViewportResult`, and updated `TransactionObservationConnection`.
- [ ] Add input types for `AreaFilter`, `TransactionObservationFilter`, `MapBoundsInput`, and page/limit validation where missing.
- [ ] Implement `areas`, `transactionObservations`, and `marketMapViewport` resolvers.
- [ ] Include `locationPrecision`, `locationLabel`, and `locationDisclaimer` anywhere geometry is included.
- [ ] Prevent raw payload JSON exposure in default spatial queries.
- [ ] Add GraphQL schema/resolver tests for bounded results, filters, and precision fields.

**Expected Evidence**

- [ ] GraphQL introspection/tests show required queries and no unbounded list paths.
- [ ] API tests prove precision labels/disclaimers are returned with geometry-bearing results.
- [ ] Invalid bounds, excessive limits, and malformed filters produce readable errors.

---

### Slice 6 — Spatial Docs, ADR, and UAT Closure

**Goal**

Close the phase with evidence a future map can trust.

**Tasks**

- [ ] Add `docs/spatial-query-strategy.md`.
- [ ] Add `docs/adr/006-use-postgis-for-viewport-and-proximity-queries.md`.
- [ ] Update `docs/data-model.md`, `docs/local-development.md`, `README.md`, and planning files as needed.
- [ ] Run `corepack pnpm check` or Docker-backed Rust checks.
- [ ] Run Compose smoke, boundary import, fixture import, GraphQL spatial queries, and UAT cases.
- [ ] Update `.planning/STATE.md` when the phase is ready for UAT or complete.

**Expected Evidence**

- [ ] UAT proves ward boundaries load, indexes exist, viewport filtering works, ward filtering runs in the database, and API responses show location precision.
- [ ] Documentation explains limitations, precision semantics, and how to validate locally.

---

## 6. Testing Strategy

### Unit Tests

| Area | Required Coverage |
|---|---|
| Boundary parser | Feature count, code/name normalization, geometry type/SRID validation, source hash stability. |
| Filter builder | Period, price, area, walk time, asset type, bounds, proximity radius, pagination limits. |
| Location transparency | Deterministic label and disclaimer for `unknown`, `ward_polygon`, and future point precision. |

### Integration Tests

| Flow | Expected Result |
|---|---|
| Boundary import rerun | First run imports 23 wards; second run creates no duplicates. |
| Viewport query | Only observations with defensible non-null geometry inside the viewport are returned. |
| Ward query | Filtering by selected ward runs in SQL/PostGIS and returns only matching observations or aggregates. |
| Counts by ward | Counts are grouped by ward with explicit sample size and no application-memory spatial filtering. |
| GraphQL spatial query | Queries are bounded and return precision labels/disclaimers. |

### Manual Validation

| Scenario | Why Manual Validation Is Needed |
|---|---|
| Inspect a ward boundary visually or through coordinate extents. | Catches swapped coordinates, wrong CRS, or non-Tokyo geometry. |
| Run local Compose plus imports from a fresh volume. | Confirms the developer workflow is still understandable. |
| Query a viewport with no matching point geometry. | Confirms honest empty behavior for Phase 02 unknown-location observations. |

### Regression Risks

| Risk Area | Possible Regression | Mitigation |
|---|---|---|
| Location precision | CSV observations accidentally become map points. | Constraints, tests, and GraphQL disclaimers. |
| Query performance | Spatial filters fetch too much data into Rust. | SQL review, result caps, and integration tests. |
| Lineage | Boundary source data becomes untraceable fixture data. | Source docs and importer lineage requirements. |
| Existing GraphQL inspection | Phase 02 query behavior breaks while adding filters. | Schema compatibility tests and bounded connection tests. |
| Schema migration | Existing databases fail additive area migration. | Fresh and existing-volume smoke checks. |

---

## 7. Acceptance Criteria

### Product / User Criteria

- [ ] A developer can import official Tokyo ward boundaries locally.
- [ ] An analyst-facing API can list Tokyo wards and use a selected ward as a filter.
- [ ] A viewport query returns only observations with defensible geometry inside the requested bounds.
- [ ] Area counts and metrics include sample size, units, filters, and source/precision limitations.
- [ ] Geometry-bearing API results include `locationPrecision`, `locationLabel`, and `locationDisclaimer`.

### Engineering Criteria

- [ ] Spatial filtering runs in PostGIS through SQLx, not application-memory filtering.
- [ ] `area_boundaries.geometry` and transaction location geometry have spatial indexes.
- [ ] All list queries are paginated or strictly bounded.
- [ ] Tests verify at least one spatial query and one boundary-based query.
- [ ] Phase 02 CSV observations remain `location_precision=unknown` with null geometry unless a defensible geometry source is ingested.

### Documentation Criteria

- [ ] `docs/spatial-query-strategy.md` explains query patterns, precision semantics, indexes, and limitations.
- [ ] `docs/adr/006-use-postgis-for-viewport-and-proximity-queries.md` is accepted.
- [ ] `docs/data-sources.md` documents the official ward boundary source.
- [ ] `docs/data-model.md` reflects the physical area/boundary model and precision behavior.
- [ ] README/local-development docs explain the local boundary import and validation path.

### UAT Criteria

- [ ] UAT-01: Tokyo ward boundaries load into PostGIS.
- [ ] UAT-02: Spatial indexes exist and are validated.
- [ ] UAT-03: Viewport filtering returns only matching mappable observations.
- [ ] UAT-04: Ward filtering/counts run through database queries.
- [ ] UAT-05: GraphQL spatial queries are bounded and expose location transparency.
- [ ] UAT-06: Existing Phase 02 import/provenance behavior still works.

