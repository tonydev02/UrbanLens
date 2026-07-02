# UrbanLens Conceptual Data Model

## Purpose and Boundary

This document defines the first source-grounded domain model. It is not a physical PostgreSQL schema and does not authorize a migration. Phase 1 and later work must preserve these meanings when choosing tables, GraphQL types, and worker interfaces.

The primary market fact is a `transaction_observation`. UrbanLens does not model a durable `property` because the selected source does not provide stable property identity or exact property geography.

## Core Language

| Term | Definition |
|---|---|
| `data_source` | Publisher-level origin and legal/provenance identity, such as MLIT Real Estate Information Library. |
| `dataset` | A bounded retrievable/versioned artifact or query result from a data source, identified by retrieval context and checksum. |
| `import_run` | One execution that reads a dataset artifact and attempts raw and normalized persistence. |
| `raw_record` | An immutable source row/feature plus its artifact position and payload hash. |
| `transaction_observation` | One published historical transaction observation normalized from one raw record; not a listing or durable property. |
| `area` | A governed geographic unit used for selection or aggregation, such as a Tokyo ward. |
| `station` | A canonical station reference only after an authoritative station source is selected; a current source station label is not yet that entity. |
| `market_metric` | A reproducible aggregate over explicitly eligible observations, filters, period, unit, and sample size. |
| `validation_issue` | A structured warning or rejection tied to a source field, record, or import run. |
| `location_precision` | Explicit evidence level describing what a stored geometry represents. |

## Conceptual Relationships

```text
data_source 1 ── * dataset 1 ── * import_run 1 ── * raw_record
                                                        │
                                                        │ 0..1 normalizes to
                                                        ▼
                                             transaction_observation
                                                │       │       │
                                                │       │       └── * validation_issue
                                                │       └────────── 0..1 area
                                                └────────────────── 0..1 station context

transaction_observation * ── eligibility/filter definition ── * market_metric
```

Every normalized observation must trace to exactly one raw record. A raw record can remain unnormalized when rejected, but it is never discarded.

## Conceptual Entities

### `data_source`

Required concepts:

- stable internal identifier;
- source name and publisher;
- source and license URLs;
- access methods;
- default attribution;
- source-level limitations; and
- metadata verification timestamp.

### `dataset`

A dataset identifies an exact artifact or API response, not the publisher in general.

Required concepts:

- data-source reference;
- source dataset/category name;
- retrieval method and query parameters;
- source version/period when available;
- retrieval timestamp;
- artifact SHA-256 checksum;
- byte and record counts;
- encoding/format; and
- source attribution and processing status.

The three ward-quarter fixture CSVs are three artifacts of the same selected source dataset category.

### `import_run`

```text
pending
running
completed
completed_with_warnings
failed
```

Required counts:

```text
records_received
records_imported
records_updated
duplicates_skipped
records_rejected
warning_records
```

An import run records start/completion timestamps, dataset reference, normalization-logic version, status, error kind, and counts. Partial failures must be explicit.

### `raw_record`

Required concepts:

- import-run and dataset references;
- source row ordinal or feature position;
- nullable source external identifier;
- exact source payload/value representation;
- payload SHA-256 hash;
- validation status/errors; and
- creation timestamp.

For CSV, preserve the decoded field map for querying and retain enough artifact metadata to recover the exact source bytes. Blank strings remain source facts and are not converted inside the raw representation.

### `transaction_observation`

The first canonical observation requires these conceptual fields; physical names and nullability are finalized with the migration design.

| Concept | Type Intent | Source / Rule |
|---|---|---|
| identity | internal UUID | Never presented as source property identity. |
| raw lineage | required raw-record reference | One originating record. |
| asset type | governed enum plus raw label | Map source `種類`; preserve unknown labels with warning. |
| price category | governed enum | MVP accepts transaction-price information only. |
| transaction quarter | year + quarter | Parse source quarter; no invented exact date. |
| total transaction price | nullable positive JPY integer | Parse `取引価格（総額）`; invalid becomes null + issue. |
| source unit price | nullable positive JPY/m² | Only source `取引価格（㎡単価）`; do not populate universally. |
| recorded area | nullable non-negative decimal m² | Meaning remains asset-type contextual. |
| total floor area | nullable numeric/bounded value | Preserve display bounds such as `以上`; do not coerce to an exact measurement. |
| floor plan | nullable source category | Preserve raw label. |
| building year | nullable year/era interpretation | Record transformation and raw label. |
| structure | nullable source category | Preserve unrecognized values. |
| current/source use | nullable multi-valued label set | Discovery aid, not verified commercial identity. |
| intended use | nullable source category | Survey response. |
| municipality | required code plus source label | Validate expected Tokyo code in MVP imports. |
| district label | nullable source label | Not a durable area identity. |
| nearest-station label | nullable source label | Not yet a canonical `station`. |
| station walking time | nullable non-negative minutes | Validate reasonable range; never default. |
| location precision | required governed enum | Assignment rules below. |
| geometry | nullable SRID 4326 geometry/geography | Must agree with location precision. |
| normalization version | required version string | Makes transformations reproducible. |

Road, planning, coverage-ratio, renovation, and circumstances fields remain traceable canonical/source attributes where useful; none grants property identity.

### `area`

An area is an authoritative geographic unit with stable internal identity, source code, label, type, source/version lineage, and optional polygon geometry. Phase 0 recognizes Tokyo wards as the first intended area type but does not select or ingest a boundary dataset.

### `station`

A canonical station eventually requires an authoritative station code, name variants, point geometry, source/version lineage, and railway relationships. The current CSV station name and XPT001 point are source context. They must not create or merge a canonical station by label alone.

### `validation_issue`

Required concepts:

- import-run and raw-record/observation reference;
- issue code and severity (`warning` or `rejection` initially);
- affected source/canonical field;
- raw value summary safe for logging;
- human-readable explanation; and
- transformation or disposition (`set_null`, `preserved_unknown`, `record_rejected`, etc.).

### `market_metric`

A metric definition/result must include:

- qualified metric identifier and display name;
- value and unit;
- calculation version;
- eligible asset type(s);
- filters and area context;
- start/end quarter;
- eligible sample size;
- source/dataset lineage;
- limitation text; and
- small-sample flag.

Metric results may be cached/materialized later, but the eligibility and formula remain reproducible.

## Location Precision

```text
exact_point
nearest_station_point
district_centroid
ward_polygon
unknown
```

| Value | Evidence Required | Geometry / Query Behavior | User Meaning |
|---|---|---|---|
| `exact_point` | Source explicitly supplies an exact observation coordinate with permitted use. | Point geometry. | Exact observation point. Unsupported for the selected source. |
| `nearest_station_point` | Geometry obtained directly from XPT001 for the source feature being modeled. | Aggregate colocated features; distance/view queries use station context. | Nearest-station context, never property location. |
| `district_centroid` | Authoritative district boundary/centroid source and documented transformation. | Aggregate/context point only. | Approximate district context. Deferred. |
| `ward_polygon` | Authoritative ward boundary and ward-level aggregate. | Polygon selection/aggregation; never attach as individual point. | Ward aggregate. |
| `unknown` | No defensible geometry link. | No map point; observation remains eligible for non-spatial metrics. | Location cannot be mapped honestly. |

`location_precision` is mandatory even when geometry is null. Geometry and precision must pass a consistency constraint in the later physical schema.

## Source-to-Domain Mapping and Uncertainty

| Source Pattern | Canonical Rule | Validation / Limitation |
|---|---|---|
| Empty CSV field | canonical null | Preserve empty raw string; warning only when the field is expected for that asset type. |
| Numeric string | parsed numeric | Reject non-finite/negative values according to domain rule; retain raw. |
| `2,000㎡以上` or another bound | structured lower bound when supported, otherwise null | Never store as exact 2,000 m². |
| Unknown asset/use label | preserved unknown label | Add validation warning; do not coerce to “other” without retaining the source value. |
| Quarter label | year + quarter | Reject impossible/out-of-range quarter. |
| CSV station label/time | station context attributes | Does not establish station identity or point geometry. |
| XPT001 feature geometry | nearest-station point | Applies to that raw XPT feature only. |
| District name/code | source locality context | District codes may change and names are not durable identity. |

## Idempotency and Duplicate Handling

The selected source supplies no documented stable transaction identifier. A content hash alone is unsafe because two distinct anonymized transactions can have identical published fields.

Phase 0 therefore adopts exact-artifact idempotency:

```text
dataset identity = source + retrieval query + artifact SHA-256
raw record identity = dataset identity + source row ordinal
payload hash = SHA-256 of the canonical raw field representation
```

- Rerunning the same artifact reuses/skips the same raw-record identities.
- Identical rows at different ordinals remain separate observations.
- A revised source artifact is a new dataset artifact/import, even for the same query period.
- Cross-artifact or cross-release transaction identity is not claimed or automatically collapsed.
- Future deduplication requires source-supported identity or an explicit probabilistic decision with visible confidence and an ADR.

## Metric Eligibility

- Counts use observations that passed record-level normalization and current filters.
- Price medians require positive parsed total price and remain faceted by asset type.
- Area medians require a non-negative parsed area and remain faceted by asset type.
- Source ¥/m² medians use only publisher-populated unit-price values; the eligible `n` is distinct from total observation `n`.
- Quarterly trends use published quarters and show observation counts by asset type.
- `n < 5` sets a small-sample flag; `n = 0` yields unavailable, not numeric zero.

## Data Quality State

Later implementation may expose `high`, `medium`, `low`, or `unknown`, but it must be rule-based and explainable. Candidate factors are geographic precision, required price/area completeness, station context, source freshness, and validation issues. Phase 0 does not define a fake aggregate confidence score.

## Physical-Design Requirements for Later Phases

- PostgreSQL/PostGIS owns spatial storage, joins, viewport filtering, proximity, and area aggregation.
- Use SRID 4326 and GiST/SP-GiST indexes appropriate to the chosen geometry/geography columns.
- Foreign keys enforce lineage from observation to raw record/import/dataset/source.
- Raw payload storage is not exposed in normal product queries.
- All list queries are bounded by pagination or strict limits.
- Physical schema changes require migrations and tests; they must preserve this conceptual boundary or update documentation/ADR explicitly.

## Current Physical Schema

Phase 02 Slices 2 and 3 add the first physical observation schema and the
repository layer that writes through it:

- `transaction_observations` stores one normalized historical observation per
  originating raw record.
- `transaction_location_contexts` stores the observation's explicit
  `location_precision` and optional SRID 4326 geometry.
- `validation_issues.transaction_observation_id` can link warning issues to a
  normalized observation.
- Slice 5 adds bounded GraphQL inspection for `transaction_observations`,
  `import_runs`, `validation_issues`, `data_sources`, and single-observation
  provenance summaries. These queries expose lineage identifiers, source
  positions, payload hashes, dataset metadata, and source identity, but do not
  expose raw payload JSON by default.

Additional Slice 3 idempotency keys:

- `data_sources.name` is unique so publisher-level source metadata can be
  upserted deterministically.
- `datasets` are unique by source, source dataset name, retrieval method,
  retrieval query, and artifact checksum so exact source artifacts are reused
  on repeated imports.

Important constraints:

- each observation references one `(raw_record_id, import_run_id, dataset_id)`
  lineage tuple;
- each raw record can produce at most one normalized observation;
- asset type, price category, validation state, Tokyo municipality code, and
  quarter values are constrained;
- money values must be positive when present, and area values must be
  non-negative when present;
- `unknown` location precision requires `location IS NULL`;
- point precision values require point geometry, and ward precision requires
  polygon or multipolygon geometry.

The schema intentionally does not make `source_record_hash` globally unique.
Exact-artifact idempotency remains anchored in raw-record lineage
`(dataset_id, source_position)`, so two identical source payloads at different
row ordinals are still allowed to become distinct observations.

The importer repository preserves the first raw-record lineage on duplicate
reruns. A later import run that sees the same dataset artifact/source position
counts that row as a duplicate skip instead of rewriting the raw record's
original `import_run_id`.
