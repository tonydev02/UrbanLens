# UrbanLens Importer

## Current Scope

Phase 02 has completed the pure MLIT transaction CSV parser/normalizer and the
first canonical PostgreSQL schema for normalized observations. It does not yet
write fixture rows to PostgreSQL, create import runs from the CLI, upsert raw
records, or expose GraphQL inspection.

The parser currently targets the committed official-source fixtures under:

```text
workers/importer/fixtures/transactions/
```

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

## Next Slice

Slice 3 should add persistence repositories for data source/dataset upsert,
import-run lifecycle, raw-record upsert, validation issue insert, and
observation/location upsert. CLI commands, idempotent import counters, and
GraphQL inspection remain intentionally deferred.
