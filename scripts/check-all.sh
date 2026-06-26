#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

if command -v cargo >/dev/null 2>&1; then
  ./scripts/check-rust.sh
else
  ./scripts/check-rust-docker.sh
fi

./scripts/check-web.sh
