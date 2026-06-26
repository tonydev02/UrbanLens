#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

if command -v pnpm >/dev/null 2>&1; then
  pnpm_cmd=(pnpm)
else
  pnpm_cmd=(corepack pnpm)
fi

"${pnpm_cmd[@]}" --filter @urbanlens/web lint
"${pnpm_cmd[@]}" --filter @urbanlens/web typecheck
"${pnpm_cmd[@]}" --filter @urbanlens/web test --run
