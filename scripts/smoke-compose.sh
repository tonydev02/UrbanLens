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
assert_psql_true "SELECT COUNT(*) FILTER (WHERE success) = 2 AND COUNT(*) FILTER (WHERE NOT success) = 0 FROM _sqlx_migrations;" "SQLx migration ledger is successful"
assert_psql_true "SELECT COUNT(*) = 6 FROM information_schema.tables WHERE table_schema = 'public' AND table_name IN ('data_sources', 'datasets', 'import_runs', 'raw_records', 'validation_issues', 'areas');" "six foundation tables exist"
assert_psql_true "SELECT COALESCE(SUM(row_count), 0) = 0 FROM (SELECT COUNT(*) AS row_count FROM data_sources UNION ALL SELECT COUNT(*) FROM datasets UNION ALL SELECT COUNT(*) FROM import_runs UNION ALL SELECT COUNT(*) FROM raw_records UNION ALL SELECT COUNT(*) FROM validation_issues UNION ALL SELECT COUNT(*) FROM areas) counts;" "foundation tables are empty"
assert_psql_true "SELECT EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname = 'public' AND tablename = 'areas' AND indexname = 'areas_geometry_gix' AND indexdef ILIKE '%WHERE (geometry IS NOT NULL)%');" "partial GiST index exists"
assert_psql_true "SELECT EXISTS (SELECT 1 FROM geometry_columns WHERE f_table_schema = 'public' AND f_table_name = 'areas' AND f_geometry_column = 'geometry' AND type = 'MULTIPOLYGON' AND srid = 4326);" "areas geometry is MultiPolygon SRID 4326"
assert_psql_true "SELECT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'raw_records_dataset_position_unique') AND EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'raw_records_import_run_dataset_fk') AND EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'validation_issues_raw_record_import_run_fk');" "lineage constraints exist"

log "Compose smoke validation passed"
