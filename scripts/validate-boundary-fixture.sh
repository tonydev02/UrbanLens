#!/usr/bin/env bash
set -euo pipefail

fixture="${1:-workers/importer/fixtures/boundaries/mlit-n03-tokyo-23-wards-2023.geojson}"
expected_hash="42d78e7f93cb63c6d2afdf388e06bab454962fb55a3f9b366148cfdb701595a7"

if [[ ! -f "$fixture" ]]; then
  echo "Boundary fixture not found: $fixture" >&2
  exit 1
fi

actual_hash="$(shasum -a 256 "$fixture" | awk '{print $1}')"
if [[ "$actual_hash" != "$expected_hash" ]]; then
  echo "Boundary fixture checksum mismatch: $actual_hash" >&2
  exit 1
fi

jq -e '
  def coord_pairs:
    [.features[].geometry.coordinates
      | ..
      | arrays
      | select(length == 2 and (.[0] | type) == "number" and (.[1] | type) == "number")];

  .type == "FeatureCollection"
  and (.features | length == 118)
  and ([.features[].properties.N03_007] | unique | length == 23)
  and ([.features[].properties.N03_007] | unique == [
    "13101", "13102", "13103", "13104", "13105", "13106", "13107", "13108",
    "13109", "13110", "13111", "13112", "13113", "13114", "13115", "13116",
    "13117", "13118", "13119", "13120", "13121", "13122", "13123"
  ])
  and all(.features[]; .properties.N03_001 == "東京都")
  and all(.features[]; .geometry.type == "Polygon")
  and ((coord_pairs | length) > 0)
  and all(coord_pairs[]; .[0] >= 138 and .[0] <= 141 and .[1] >= 34 and .[1] <= 37)
' "$fixture" >/dev/null

echo "Boundary fixture ok: 118 source polygons across 23 Tokyo special wards."
