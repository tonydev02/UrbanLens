#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

source="${IMPORT_SOURCE:-mlit}"
prefecture="${IMPORT_PREFECTURE:-13}"
period="${IMPORT_PERIOD:-2024Q4}"
fixture_dir="${IMPORT_FIXTURE_DIR:-workers/importer/fixtures/transactions}"
normalization_version="${IMPORT_NORMALIZATION_VERSION:-mlit-transaction-csv-v1}"
database_url="${DATABASE_URL:-postgres://urbanlens:urbanlens_dev@postgres:5432/urbanlens}"
compose_network="${IMPORTER_DOCKER_NETWORK:-urbanlens_default}"

if [[ "${IMPORTER_RUNTIME:-docker}" == "host" ]]; then
  exec cargo run -p urbanlens-importer -- import-transactions \
    --source "$source" \
    --prefecture "$prefecture" \
    --period "$period" \
    --fixture-dir "$fixture_dir" \
    --normalization-version "$normalization_version" \
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
  cargo run -p urbanlens-importer -- import-transactions \
    --source "$source" \
    --prefecture "$prefecture" \
    --period "$period" \
    --fixture-dir "$fixture_dir" \
    --normalization-version "$normalization_version" \
    --database-url "$database_url"
