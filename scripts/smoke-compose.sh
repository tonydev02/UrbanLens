#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

api_port="${API_PORT:-8080}"
web_port="${WEB_PORT:-3000}"
postgres_user="${POSTGRES_USER:-urbanlens}"
postgres_db="${POSTGRES_DB:-urbanlens}"
graphql_query='{ connectivity { service status databaseReachable migrationsApplied } }'

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

log() {
  printf '[compose-smoke] %s\n' "$*"
}

fail() {
  printf '[compose-smoke] ERROR: %s\n' "$*" >&2
  exit 1
}

require_command() {
  command -v "$1" >/dev/null 2>&1 || fail "$1 is required"
}

require_command curl
require_command docker
require_command python3

wait_for_http() {
  local url="$1"
  local label="$2"

  for _ in $(seq 1 90); do
    if curl --fail --silent --show-error "$url" >/dev/null 2>&1; then
      log "$label is reachable"
      return 0
    fi
    sleep 2
  done

  fail "$label did not become reachable at $url"
}

wait_for_container_health() {
  local service="$1"

  for _ in $(seq 1 90); do
    local container_id
    container_id="$(docker compose ps --all -q "$service")"

    if [ -n "$container_id" ]; then
      local health
      health="$(docker inspect -f '{{if .State.Health}}{{.State.Health.Status}}{{else}}{{.State.Status}}{{end}}' "$container_id")"
      if [ "$health" = "healthy" ]; then
        log "$service is healthy"
        return 0
      fi
    fi

    sleep 2
  done

  fail "$service did not become healthy"
}

assert_json() {
  local path="$1"
  local expression="$2"
  local label="$3"

  python3 - "$path" "$expression" <<'PY' || fail "$label"
import json
import sys

path, expression = sys.argv[1:3]
with open(path, "r", encoding="utf-8") as handle:
    payload = json.load(handle)

if not eval(expression, {"__builtins__": {}}, {"payload": payload}):
    raise SystemExit(1)
PY
}

assert_psql_true() {
  local sql="$1"
  local label="$2"
  local result

  result="$(docker compose exec -T postgres \
    psql -U "$postgres_user" -d "$postgres_db" -v ON_ERROR_STOP=1 -Atc "$sql")"

  if [ "$result" != "t" ]; then
    fail "$label returned '$result'"
  fi

  log "$label"
}

log "rendering Compose config"
docker compose config >/dev/null

log "building and starting stack"
docker compose up --build -d

wait_for_http "http://127.0.0.1:${api_port}/health" "API liveness"
wait_for_http "http://127.0.0.1:${api_port}/ready" "API readiness"
wait_for_http "http://127.0.0.1:${web_port}/market-map" "web market map"

log "checking service health and migration result"
migrate_container="$(docker compose ps --all -q migrate)"
[ -n "$migrate_container" ] || fail "migrate container was not found"
migrate_exit_code="$(docker inspect -f '{{.State.ExitCode}}' "$migrate_container")"
[ "$migrate_exit_code" = "0" ] || fail "migrate exited with code $migrate_exit_code"

for service in postgres api web; do
  wait_for_container_health "$service"
done

log "checking HTTP contracts"
health_body="$tmp_dir/health.json"
curl --fail --silent --show-error "http://127.0.0.1:${api_port}/health" > "$health_body"
assert_json "$health_body" "payload.get('status') == 'ok'" "/health payload was not ok"

ready_body="$tmp_dir/ready.json"
curl --fail --silent --show-error "http://127.0.0.1:${api_port}/ready" > "$ready_body"
assert_json "$ready_body" "payload.get('status') == 'ready' and payload.get('database_reachable') is True and payload.get('migrations_applied') is True" "/ready payload was not ready"

graphql_body="$tmp_dir/graphql.json"
curl --fail --silent --show-error "http://127.0.0.1:${api_port}/graphql" \
  -H 'content-type: application/json' \
  --data "{\"query\":\"${graphql_query}\"}" \
  > "$graphql_body"
assert_json "$graphql_body" "'errors' not in payload and payload.get('data', {}).get('connectivity', {}).get('service') == 'urbanlens-api' and payload.get('data', {}).get('connectivity', {}).get('status') == 'ready' and payload.get('data', {}).get('connectivity', {}).get('databaseReachable') is True and payload.get('data', {}).get('connectivity', {}).get('migrationsApplied') is True" "GraphQL connectivity payload was not ready"

request_headers="$tmp_dir/request-headers.txt"
curl --fail --silent --show-error --dump-header "$request_headers" \
  -H 'x-request-id: urbanlens-smoke-request' \
  "http://127.0.0.1:${api_port}/health" \
  >/dev/null
grep -qi '^x-request-id: urbanlens-smoke-request' "$request_headers" \
  || fail "x-request-id was not preserved"
log "request ID preservation works"

allowed_cors="$tmp_dir/allowed-cors.txt"
curl --silent --show-error --dump-header "$allowed_cors" \
  -X OPTIONS "http://127.0.0.1:${api_port}/graphql" \
  -H 'origin: http://localhost:3000' \
  -H 'access-control-request-method: POST' \
  -H 'access-control-request-headers: content-type,x-request-id' \
  >/dev/null
grep -qi '^access-control-allow-origin: http://localhost:3000' "$allowed_cors" \
  || fail "configured CORS origin was not allowed"
log "configured CORS origin is allowed"

blocked_cors="$tmp_dir/blocked-cors.txt"
curl --silent --show-error --dump-header "$blocked_cors" \
  -X OPTIONS "http://127.0.0.1:${api_port}/graphql" \
  -H 'origin: http://example.invalid' \
  -H 'access-control-request-method: POST' \
  >/dev/null
if grep -qi '^access-control-allow-origin: http://example.invalid' "$blocked_cors"; then
  fail "unconfigured CORS origin was allowed"
fi
log "unconfigured CORS origin is not granted"

log "checking database schema foundation"
assert_psql_true "SELECT COUNT(*) = 2 FROM pg_extension WHERE extname IN ('postgis', 'pgcrypto');" "required extensions exist"
assert_psql_true "SELECT COUNT(*) FILTER (WHERE success) = 6 AND COUNT(*) FILTER (WHERE NOT success) = 0 FROM _sqlx_migrations;" "SQLx migration ledger is successful"
assert_psql_true "SELECT COUNT(*) = 9 FROM information_schema.tables WHERE table_schema = 'public' AND table_name IN ('data_sources', 'datasets', 'import_runs', 'raw_records', 'validation_issues', 'areas', 'area_boundaries', 'transaction_observations', 'transaction_location_contexts');" "lineage, area, and transaction tables exist"
assert_psql_true "SELECT COALESCE(SUM(row_count), 0) = 0 FROM (SELECT COUNT(*) AS row_count FROM data_sources UNION ALL SELECT COUNT(*) FROM datasets UNION ALL SELECT COUNT(*) FROM import_runs UNION ALL SELECT COUNT(*) FROM raw_records UNION ALL SELECT COUNT(*) FROM validation_issues UNION ALL SELECT COUNT(*) FROM areas UNION ALL SELECT COUNT(*) FROM area_boundaries UNION ALL SELECT COUNT(*) FROM transaction_observations UNION ALL SELECT COUNT(*) FROM transaction_location_contexts) counts;" "lineage, area, and transaction tables are empty"
assert_psql_true "SELECT EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname = 'public' AND tablename = 'areas' AND indexname = 'areas_geometry_gix' AND indexdef ILIKE '%WHERE (geometry IS NOT NULL)%');" "partial GiST index exists"
assert_psql_true "SELECT EXISTS (SELECT 1 FROM geometry_columns WHERE f_table_schema = 'public' AND f_table_name = 'areas' AND f_geometry_column = 'geometry' AND type = 'MULTIPOLYGON' AND srid = 4326);" "areas geometry is MultiPolygon SRID 4326"
assert_psql_true "SELECT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'raw_records_dataset_position_unique') AND EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'raw_records_import_run_dataset_fk') AND EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'validation_issues_raw_record_import_run_fk') AND EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'transaction_observations_raw_record_unique') AND EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'transaction_location_contexts_precision_geometry_consistent');" "lineage and transaction constraints exist"
assert_psql_true "SELECT EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname = 'public' AND tablename = 'transaction_location_contexts' AND indexname = 'transaction_location_contexts_location_gix' AND indexdef ILIKE '%WHERE (location IS NOT NULL)%');" "transaction location partial GiST index exists"
assert_psql_true "SELECT EXISTS (SELECT 1 FROM geometry_columns WHERE f_table_schema = 'public' AND f_table_name = 'transaction_location_contexts' AND f_geometry_column = 'location' AND srid = 4326);" "transaction location geometry is SRID 4326"
assert_psql_true "SELECT EXISTS (SELECT 1 FROM geometry_columns WHERE f_table_schema = 'public' AND f_table_name = 'area_boundaries' AND f_geometry_column = 'geometry' AND type = 'MULTIPOLYGON' AND srid = 4326);" "area boundary geometry is MultiPolygon SRID 4326"
assert_psql_true "SELECT EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname = 'public' AND tablename = 'area_boundaries' AND indexname = 'area_boundaries_geometry_gix') AND EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname = 'public' AND tablename = 'areas' AND indexname = 'areas_type_administrative_code_unique') AND EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname = 'public' AND tablename = 'transaction_observations' AND indexname = 'transaction_observations_ward_asset_period_idx') AND EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname = 'public' AND tablename = 'transaction_observations' AND indexname = 'transaction_observations_trade_price_idx') AND EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname = 'public' AND tablename = 'transaction_observations' AND indexname = 'transaction_observations_area_m2_idx') AND EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname = 'public' AND tablename = 'transaction_observations' AND indexname = 'transaction_observations_station_walk_minutes_idx');" "area boundary and spatial-filter indexes exist"
assert_psql_true "SELECT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'areas_area_type_known') AND EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'area_boundaries_geometry_valid') AND EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'area_boundaries_location_precision_known') AND EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'area_boundaries_raw_lineage_consistent');" "area and boundary constraints exist"

log "checking transaction schema contracts"
docker compose exec -T postgres psql -U "$postgres_user" -d "$postgres_db" -v ON_ERROR_STOP=1 <<'SQL'
DO $$
DECLARE
    v_source_id uuid;
    v_dataset_id uuid;
    v_import_run_id uuid;
    v_raw_record_id uuid;
    v_observation_id uuid;
    v_area_id uuid;
    v_boundary_id uuid;
    invalid_boundary_accepted boolean;
BEGIN
    INSERT INTO data_sources (
        name,
        publisher,
        source_url,
        license_url,
        metadata_verified_at
    )
    VALUES (
        'MLIT contract test',
        'MLIT',
        'https://www.reinfolib.mlit.go.jp/',
        'https://www.reinfolib.mlit.go.jp/help/termsOfUse/',
        now()
    )
    RETURNING id INTO v_source_id;

    INSERT INTO datasets (
        source_id,
        source_dataset_name,
        retrieval_method,
        retrieval_query,
        retrieved_at,
        artifact_sha256,
        format,
        record_count
    )
    VALUES (
        v_source_id,
        'MLIT transaction fixture contract test',
        'fixture',
        '{"prefecture":"13","period":"2024Q4"}'::jsonb,
        now(),
        repeat('a', 64),
        'csv',
        1
    )
    RETURNING id INTO v_dataset_id;

    INSERT INTO import_runs (
        dataset_id,
        status,
        normalization_version,
        completed_at,
        records_received,
        records_imported
    )
    VALUES (
        v_dataset_id,
        'completed',
        'contract-test',
        now(),
        1,
        1
    )
    RETURNING id INTO v_import_run_id;

    INSERT INTO raw_records (
        dataset_id,
        import_run_id,
        source_position,
        payload_json,
        payload_sha256,
        validation_status
    )
    VALUES (
        v_dataset_id,
        v_import_run_id,
        1,
        '{"row":"contract-test"}'::jsonb,
        repeat('b', 64),
        'valid'
    )
    RETURNING id INTO v_raw_record_id;

    INSERT INTO areas (
        dataset_id,
        source_id,
        source_code,
        administrative_code,
        name,
        name_ja,
        name_en,
        area_type
    )
    VALUES (
        v_dataset_id,
        v_source_id,
        '13109',
        '13109',
        '品川区',
        '品川区',
        'Shinagawa',
        'ward'
    )
    RETURNING id INTO v_area_id;

    INSERT INTO area_boundaries (
        area_id,
        source_id,
        dataset_id,
        import_run_id,
        raw_record_id,
        administrative_code,
        name_ja,
        name_en,
        source_record_hash,
        source_feature_position,
        boundary_version,
        geometry
    )
    VALUES (
        v_area_id,
        v_source_id,
        v_dataset_id,
        v_import_run_id,
        v_raw_record_id,
        '13109',
        '品川区',
        'Shinagawa',
        repeat('e', 64),
        1,
        'contract-test',
        ST_Multi(ST_GeomFromText(
            'POLYGON((139.70 35.60,139.71 35.60,139.71 35.61,139.70 35.61,139.70 35.60))',
            4326
        ))
    )
    RETURNING id INTO v_boundary_id;

    UPDATE areas
    SET current_boundary_id = v_boundary_id
    WHERE id = v_area_id;

    invalid_boundary_accepted := false;
    BEGIN
        INSERT INTO area_boundaries (
            area_id,
            source_id,
            dataset_id,
            administrative_code,
            name_ja,
            source_record_hash,
            boundary_version,
            geometry
        )
        VALUES (
            v_area_id,
            v_source_id,
            v_dataset_id,
            '13109',
            '品川区',
            repeat('f', 64),
            'invalid-srid',
            ST_Multi(ST_GeomFromText(
                'POLYGON((139.70 35.60,139.71 35.60,139.71 35.61,139.70 35.61,139.70 35.60))',
                3857
            ))
        );
        invalid_boundary_accepted := true;
    EXCEPTION
        WHEN others THEN
            NULL;
    END;
    IF invalid_boundary_accepted THEN
        RAISE EXCEPTION 'invalid boundary SRID was accepted';
    END IF;

    invalid_boundary_accepted := false;
    BEGIN
        INSERT INTO area_boundaries (
            area_id,
            source_id,
            dataset_id,
            administrative_code,
            name_ja,
            source_record_hash,
            boundary_version,
            geometry
        )
        VALUES (
            v_area_id,
            v_source_id,
            v_dataset_id,
            '13109',
            '品川区',
            repeat('0', 64),
            'invalid-type',
            ST_SetSRID(ST_MakePoint(139.70, 35.60), 4326)
        );
        invalid_boundary_accepted := true;
    EXCEPTION
        WHEN others THEN
            NULL;
    END;
    IF invalid_boundary_accepted THEN
        RAISE EXCEPTION 'invalid boundary geometry type was accepted';
    END IF;

    INSERT INTO transaction_observations (
        raw_record_id,
        import_run_id,
        dataset_id,
        source_record_hash,
        normalization_version,
        validation_status,
        asset_type,
        raw_asset_type,
        price_category,
        transaction_year,
        transaction_quarter,
        trade_price_jpy,
        source_unit_price_jpy_per_m2,
        area_m2,
        municipality_code,
        prefecture_name,
        municipality_name
    )
    VALUES (
        v_raw_record_id,
        v_import_run_id,
        v_dataset_id,
        repeat('c', 64),
        'contract-test',
        'valid',
        'land',
        '宅地(土地)',
        'transaction_price_information',
        2024,
        4,
        1000000,
        500000,
        20,
        '13109',
        '東京都',
        '品川区'
    )
    RETURNING id INTO v_observation_id;

    BEGIN
        INSERT INTO transaction_location_contexts (
            transaction_observation_id,
            location_precision,
            location
        )
        VALUES (
            v_observation_id,
            'unknown',
            ST_SetSRID(ST_MakePoint(139.7, 35.6), 4326)
        );
        RAISE EXCEPTION 'unknown precision accepted geometry';
    EXCEPTION
        WHEN check_violation THEN
            NULL;
    END;

    INSERT INTO transaction_location_contexts (
        transaction_observation_id,
        location_precision
    )
    VALUES (v_observation_id, 'unknown');

    BEGIN
        INSERT INTO transaction_observations (
            raw_record_id,
            import_run_id,
            dataset_id,
            source_record_hash,
            normalization_version,
            validation_status,
            asset_type,
            raw_asset_type,
            price_category,
            transaction_year,
            transaction_quarter,
            municipality_code,
            prefecture_name,
            municipality_name
        )
        VALUES (
            v_raw_record_id,
            v_import_run_id,
            v_dataset_id,
            repeat('d', 64),
            'contract-test',
            'valid',
            'land',
            '宅地(土地)',
            'transaction_price_information',
            2024,
            4,
            '13109',
            '東京都',
            '品川区'
        );
        RAISE EXCEPTION 'duplicate raw-record observation was accepted';
    EXCEPTION
        WHEN unique_violation THEN
            NULL;
    END;

    DELETE FROM transaction_location_contexts WHERE transaction_observation_id = v_observation_id;
    DELETE FROM transaction_observations WHERE id = v_observation_id;
    UPDATE areas SET current_boundary_id = NULL WHERE id = v_area_id;
    DELETE FROM area_boundaries WHERE id = v_boundary_id;
    DELETE FROM areas WHERE id = v_area_id;
    DELETE FROM raw_records WHERE id = v_raw_record_id;
    DELETE FROM import_runs WHERE id = v_import_run_id;
    DELETE FROM datasets WHERE id = v_dataset_id;
    DELETE FROM data_sources WHERE id = v_source_id;
END $$;
SQL
log "transaction schema contracts hold"

log "Compose smoke validation passed"
