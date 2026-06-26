#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

docker run --rm \
  -e CARGO_TARGET_DIR=/tmp/urbanlens-target \
  -v "$repo_root":/workspace \
  -w /workspace \
  rust:1.96.0-bookworm \
  bash scripts/check-rust.sh
