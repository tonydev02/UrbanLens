#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

source="${IMPORT_BOUNDARY_SOURCE:-mlit-n03}"
fixture_path="${IMPORT_BOUNDARY_FIXTURE_PATH:-workers/importer/fixtures/boundaries/mlit-n03-tokyo-23-wards-2023.geojson}"
normalization_version="${IMPORT_BOUNDARY_NORMALIZATION_VERSION:-mlit-n03-boundary-geojson-v1}"
boundary_version="${IMPORT_BOUNDARY_VERSION:-2023-01-01}"
database_url="${DATABASE_URL:-postgres://urbanlens:urbanlens_dev@postgres:5432/urbanlens}"
compose_network="${IMPORTER_DOCKER_NETWORK:-urbanlens_default}"

if [[ "${IMPORTER_RUNTIME:-docker}" == "host" ]]; then
  exec cargo run -p urbanlens-importer -- import-ward-boundaries \
    --source "$source" \
    --fixture-path "$fixture_path" \
    --normalization-version "$normalization_version" \
    --boundary-version "$boundary_version" \
    --database-url "${DATABASE_URL:-postgres://urbanlens:urbanlens_dev@localhost:5432/urbanlens}"
fi

exec docker run --rm \
  --network "$compose_network" \
  -e CARGO_TARGET_DIR=/tmp/urbanlens-target \
  -v urbanlens-cargo-registry:/usr/local/cargo/registry \
  -v urbanlens-cargo-git:/usr/local/cargo/git \
  -v urbanlens-importer-target:/tmp/urbanlens-target \
  -v "$repo_root":/workspace \
  -w /workspace \
  rust:1.96.0-bookworm \
  cargo run -p urbanlens-importer -- import-ward-boundaries \
    --source "$source" \
    --fixture-path "$fixture_path" \
    --normalization-version "$normalization_version" \
    --boundary-version "$boundary_version" \
    --database-url "$database_url"
