#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

fail() {
  printf '[mlit-smoke] ERROR: %s\n' "$*" >&2
  exit 1
}

read_env_key() {
  local env_file="$repo_root/.env"

  if [ -n "${MLIT_REINFOLIB_API_KEY:-}" ] || [ ! -f "$env_file" ]; then
    return 0
  fi

  local raw
  raw="$(awk -F= '
    $1 == "MLIT_REINFOLIB_API_KEY" {
      sub(/^[^=]*=/, "")
      print
      exit
    }
  ' "$env_file")"

  raw="${raw%\"}"
  raw="${raw#\"}"
  raw="${raw%\'}"
  raw="${raw#\'}"

  if [ -n "$raw" ]; then
    MLIT_REINFOLIB_API_KEY="$raw"
    export MLIT_REINFOLIB_API_KEY
  fi
}

command -v curl >/dev/null 2>&1 || fail "curl is required"
command -v python3 >/dev/null 2>&1 || fail "python3 is required"

read_env_key

if [ -z "${MLIT_REINFOLIB_API_KEY:-}" ]; then
  fail "MLIT_REINFOLIB_API_KEY is not set; add it to the environment or local ignored .env to run this optional diagnostic"
fi

tmp_body="$(mktemp)"
trap 'rm -f "$tmp_body"' EXIT

url='https://www.reinfolib.mlit.go.jp/ex-api/external/XIT001?year=2015&quarter=2&city=13102&priceClassification=01'

http_code="$(curl --silent --show-error --compressed \
  --connect-timeout 10 \
  --max-time 30 \
  --output "$tmp_body" \
  --write-out '%{http_code}' \
  -H "Ocp-Apim-Subscription-Key: ${MLIT_REINFOLIB_API_KEY}" \
  "$url")"

if [ "$http_code" != "200" ]; then
  fail "XIT001 returned HTTP $http_code"
fi

python3 - "$tmp_body" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, "r", encoding="utf-8") as handle:
    payload = json.load(handle)

status = payload.get("status")
data = payload.get("data")
if status is None or not isinstance(data, list):
    raise SystemExit("XIT001 response did not include expected status/data fields")

print(f"[mlit-smoke] XIT001 reachable: http=200 status={status} records={len(data)}")
PY
