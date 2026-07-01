# UrbanLens Importer

## Current Scope

Phase 02 has completed the pure MLIT transaction CSV parser/normalizer, the
first canonical PostgreSQL schema for normalized observations, and the Slice 3
persistence repositories. Slice 4 adds the stable `import-transactions` CLI and
`scripts/import-fixture.sh` local entrypoint. GraphQL inspection remains
deferred to Slice 5.

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

## Next Slice

Slice 5 should expose bounded GraphQL inspection for imported observations,
import runs, validation issues, and provenance summaries without exposing raw
payload JSON by default.
