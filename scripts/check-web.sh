#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

pnpm --filter @urbanlens/web lint
pnpm --filter @urbanlens/web typecheck
pnpm --filter @urbanlens/web test --run
