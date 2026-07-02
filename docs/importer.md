# UrbanLens Importer

## Current Scope

Phase 02 has completed the pure MLIT transaction CSV parser/normalizer, the
first canonical PostgreSQL schema for normalized observations, the persistence
repositories, the stable `import-transactions` CLI, and bounded GraphQL
inspection for imported records and provenance.

The parser currently targets the committed official-source fixtures under:

```text
workers/importer/fixtures/transactions/
```

## Fixture Import Command

Use the stable wrapper after the local Compose stack is running:

```bash
./scripts/import-fixture.sh
```

The wrapper runs `cargo run -p urbanlens-importer -- import-transactions` in the
pinned Rust Docker image by default, joins the `urbanlens_default` Compose
network, and connects to:

```text
postgres://urbanlens:urbanlens_dev@postgres:5432/urbanlens
```

The script is repeat-safe and accepts environment overrides:

```text
IMPORT_SOURCE=mlit
IMPORT_PREFECTURE=13
IMPORT_PERIOD=2024Q4
IMPORT_FIXTURE_DIR=workers/importer/fixtures/transactions
IMPORT_NORMALIZATION_VERSION=mlit-transaction-csv-v1
IMPORTER_DOCKER_NETWORK=urbanlens_default
DATABASE_URL=postgres://urbanlens:urbanlens_dev@postgres:5432/urbanlens
IMPORTER_RUNTIME=docker
```

Set `IMPORTER_RUNTIME=host` only when host Rust is installed; that mode defaults
to the host-mapped database URL:

```text
postgres://urbanlens:urbanlens_dev@localhost:5432/urbanlens
```

The underlying CLI can also be run directly:

```bash
cargo run -p urbanlens-importer -- import-transactions \
  --source mlit \
  --prefecture 13 \
  --period 2024Q4 \
  --fixture-dir workers/importer/fixtures/transactions \
  --normalization-version mlit-transaction-csv-v1 \
  --database-url postgres://urbanlens:urbanlens_dev@localhost:5432/urbanlens
```

### Naming Decision

The importer package remains named `urbanlens-importer`. Command examples should
use:

```bash
cargo run -p urbanlens-importer -- import-transactions
```

Do not document `cargo run -p importer` unless the Cargo package is deliberately
renamed later. Keeping `urbanlens-importer` matches the workspace naming style
used by crates such as `urbanlens-api` and avoids unnecessary churn in Phase 02.

Expected output is one line per fixture artifact plus a summary. It includes
artifact filename, import-run ID, terminal status, and counters for received,
imported, updated, duplicates skipped, rejected, and warning records. It does
not print full raw payload JSON or secrets.

## MLIT CSV Parser

Implemented in `workers/importer/src/mlit.rs`.

Current behavior:

- decodes Windows-31J / CP932 fixture bytes;
- validates the documented 30-column Japanese CSV header;
- handles quoted CSV values, blank strings, LF and CRLF row endings;
- preserves row positions as source-row ordinals;
- preserves decoded raw values in a header-keyed map before normalization;
- parses all three committed 2024 Q4 Tokyo ward fixtures, totaling 666 source rows.

## Normalization Boundary

The Slice 1 normalizer converts only conservative fields:

- source asset type into `land`, `land_and_building`, `used_condominium`, or preserved unknown;
- price category, accepting only MLIT `不動産取引価格情報`;
- transaction period into year and quarter;
- total transaction price as JPY when positive;
- MLIT-supplied source unit price as JPY/m2 only when the source field is populated;
- recorded area as m2 when non-negative;
- bounded total floor area such as `2,000㎡以上` as an at-least value, not an exact measurement;
- municipality code and source labels;
- nearest-station label and walking minutes;
- selected source context labels such as floor plan, structure, use, planning, renovation, and circumstances.

CSV fixture observations remain `location_precision=unknown`; no geometry is assigned and no station or property identity is inferred.

## Validation

Slice 1 defines warning and rejection issue codes for parser-normalizer tests. Invalid optional numeric values become `null + warning`. Invalid required observation identity or period values reject the normalized observation while preserving the source row for later raw-record persistence.

Covered examples:

- `unknown_asset_type`
- `negative_price`
- `invalid_trade_price`
- `invalid_source_unit_price`
- `invalid_area`
- `negative_area`
- `invalid_total_floor_area`
- `invalid_station_walk_minutes`
- `invalid_price_category`
- `invalid_municipality_code`
- `invalid_quarter`

## Verification

Run the repository Rust check on this MacBook with:

```bash
bash scripts/check-rust-docker.sh
```

Latest Slice 1 evidence: Docker-backed formatting, clippy, Rust tests, and doctests pass on `2026-06-29`. The importer crate has six tests covering all committed fixtures plus edge cases for invalid values, unknown asset labels, bounded display values, and rejection behavior.

Slice 2 adds migration `202606290001_create_transaction_observation_schema.sql`
with:

- `transaction_observations` linked to raw records, import runs, and datasets;
- `transaction_location_contexts` with explicit `location_precision` and SRID
  4326 geometry;
- an optional `validation_issues.transaction_observation_id` link for warning
  issues that can be tied to normalized observations;
- constraints for lineage, valid/warning observation status, asset type, price
  category, quarter format, positive money values, non-negative areas, Tokyo
  municipality codes, and location precision/geometry consistency;
- indexes for import-run lookup, raw-record lookup, ward/period filtering,
  asset/period filtering, hash lookup, validation issue lookup, and future
  spatial filtering.

The Compose smoke script is the schema contract check for this slice. It
verifies the new migration ledger, table/index/geometry metadata, rejects
`unknown` location precision with a geometry value, and rejects duplicate
observations for one raw record.

## Persistence Repositories

Implemented in `workers/importer/src/persistence.rs`.

Current behavior:

- upserts publisher-level `data_sources` rows by source name;
- upserts exact artifact/query `datasets` rows by source, dataset name,
  retrieval method, retrieval query, and artifact checksum;
- creates visible `import_runs` in `running` state and marks runs as
  `completed`, `completed_with_warnings`, or `failed`;
- inserts raw records with deterministic JSON payload SHA-256 hashes;
- preserves raw-record idempotency by `(dataset_id, source_position)`;
- stores warning and rejection issues with code, severity, field, safe raw
  value summary, message, and disposition;
- writes one canonical `transaction_observations` row per inserted raw record;
- writes one `transaction_location_contexts` row with
  `location_precision=unknown` and no geometry for CSV rows;
- reports counters for received, imported, updated, duplicate skipped,
  rejected, and warning records.

Duplicate fixture rows from the same dataset artifact and source position are
reported as skipped. The original raw-record/import-run lineage is preserved
rather than reassigned to a later retry run.

Slice 3 also adds migration
`202606290002_add_lineage_upsert_keys.sql`, which gives the repository durable
upsert keys for `data_sources` and `datasets`.

## Slice 4 Verification

On `2026-06-29`, with the Compose stack healthy and four migrations applied:

```text
./scripts/import-fixture.sh
summary source=mlit prefecture=13 period=2024Q4 artifacts=3 normalization_version=mlit-transaction-csv-v1 received=666 imported=666 updated=0 duplicates_skipped=0 rejected=0 warning_records=0 status=completed

./scripts/import-fixture.sh
summary source=mlit prefecture=13 period=2024Q4 artifacts=3 normalization_version=mlit-transaction-csv-v1 received=666 imported=0 updated=0 duplicates_skipped=666 rejected=0 warning_records=0 status=completed
```

Database count check after the two runs:

```text
datasets=3
import_runs=6
raw_records=666
transaction_observations=666
validation_issues=0
```

## GraphQL Inspection

Implemented in `apps/api/src/lib.rs`.

The API exposes bounded inspection queries for local verification and later
analyst-facing workflows:

```graphql
query Slice5Check {
  transactionObservations(limit: 25) {
    id
    rawRecordId
    importRunId
    datasetId
    transactionYear
    transactionQuarter
    assetType
    tradePriceJpy
    sourceUnitPriceJpyPerM2
    areaM2
    municipalityCode
    nearestStationName
    stationWalkMinutes
    validationStatus
    locationPrecision
  }
  importRuns(limit: 10) {
    id
    status
    recordsReceived
    recordsImported
    duplicatesSkipped
    recordsRejected
    warningRecords
    dataSourceName
  }
  validationIssues(limit: 25) {
    id
    issueCode
    severity
    fieldName
    message
    disposition
  }
  dataSources(limit: 10) {
    id
    name
    publisher
    datasetCount
  }
}
```

Use `transactionObservationProvenance(observationId: "...")` to inspect the
lineage summary for one normalized observation. It returns raw-record ID,
source position, payload hash, import-run status, dataset retrieval metadata,
artifact checksum, and source identity. It does not return `payload_json`.

List queries are capped server-side at 100 rows and default to 25 rows.
`transactionObservations`, `importRuns`, and `validationIssues` support narrow
filters for import/dataset/record inspection.

## Slice 6 Verification

On `2026-07-02`, Slice 6 was verified against an isolated Compose project:

```text
COMPOSE_PROJECT_NAME=urbanlens_slice6_uat
API_PORT=18081
WEB_PORT=13081
POSTGRES_PORT=15433
```

Regression checks:

```text
corepack pnpm check
corepack pnpm --filter @urbanlens/web build
COMPOSE_PROJECT_NAME=urbanlens_slice6_uat API_PORT=18081 WEB_PORT=13081 POSTGRES_PORT=15433 bash scripts/smoke-compose.sh
```

The full check passed Rust formatting, Clippy, Rust tests, API/importer tests,
web lint/typecheck, and Vitest. The web production build passed. The isolated
Compose smoke passed with four successful migrations, healthy services, empty
lineage/transaction tables before import, and transaction schema contract
checks.

First fixture import into the isolated stack:

```text
summary source=mlit prefecture=13 period=2024Q4 artifacts=3 normalization_version=mlit-transaction-csv-v1 received=666 imported=666 updated=0 duplicates_skipped=0 rejected=0 warning_records=0 status=completed
```

Repeat import against the same fixture bytes:

```text
summary source=mlit prefecture=13 period=2024Q4 artifacts=3 normalization_version=mlit-transaction-csv-v1 received=666 imported=0 updated=0 duplicates_skipped=666 rejected=0 warning_records=0 status=completed
```

Count and precision evidence after first import plus duplicate rerun:

```text
data_sources=1
datasets=3
import_runs=6
raw_records=666
transaction_observations=666
validation_issues=0
unknown_location_contexts=666
```

Bounded GraphQL inspection returned transaction observations, import-run
counters, data-source metadata, and an empty validation issue list. The sampled
observations included `locationPrecision: "unknown"` and no raw payload JSON.
`transactionObservationProvenance` returned raw-record ID, source position,
payload SHA-256, import-run status, dataset metadata, artifact checksum, and
data-source name for the sampled observation.

Failed-run visibility was validated with a temporary isolated-database trigger
that rejected `raw_records` inserts. The importer exited non-zero with
`import failed: import persistence failed`, and the database recorded:

```text
failed|persistence_error|0|0|0|0|0|2024Q4-UAT-FAIL
```

After the trigger was removed, a normal fixture rerun completed successfully
with 666 duplicates skipped, proving retry after a controlled persistence
failure does not require manual data cleanup.
