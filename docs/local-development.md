# Local Development

UrbanLens runs its local platform through Docker Compose. The required path does
not need a local `.env`; `.env.example` only documents overrides.

## Prerequisites

- Docker Engine with Docker Compose.
- Node.js 24 and pnpm 10 for host-side frontend checks.
- Rust 1.96.0 for host-side Rust checks, or the pinned Rust Docker image for
  equivalent local validation.

## Services

- `postgres`: PostGIS-backed PostgreSQL database.
- `migrate`: one-shot SQLx migration runner.
- `api`: Actix Web API. It starts only after PostgreSQL is healthy and `migrate` exits successfully.
- `web`: Next.js analyst shell. It starts only after the API health check is ready.

## Start The Stack

```bash
docker compose up --build
```

Open:

- Web workspace: `http://localhost:3000/market-map`
- API readiness: `http://localhost:8080/ready`
- GraphQL endpoint: `http://localhost:8080/graphql`

The root route redirects to `/market-map`. The market-map route is intentionally
an empty analyst shell until official transaction geography is imported.

## API Configuration

The API reads these environment variables:

- `DATABASE_URL`: PostgreSQL connection string. Required.
- `API_HOST`: bind host. Defaults to `0.0.0.0`.
- `API_PORT`: bind port. Defaults to `8080`.
- `API_DATABASE_MAX_CONNECTIONS`: SQLx pool size. Defaults to `5`.
- `CORS_ALLOWED_ORIGINS`: comma-separated HTTP or HTTPS origins. Wildcards are rejected.
- `RUST_LOG`: logging filter.

## Web Configuration

The browser-visible GraphQL proof uses:

- `WEB_PORT`: web port. Defaults to `3000`.
- `WEB_ORIGIN`: expected local web origin. Defaults to `http://localhost:3000`.
- `NEXT_PUBLIC_GRAPHQL_URL`: browser-facing GraphQL URL. Defaults to `http://localhost:8080/graphql`.

Keep `NEXT_PUBLIC_GRAPHQL_URL` browser-reachable. In Docker Compose this is
still `localhost:8080` because the request is made by the browser, not by the
web container.

## Dependency Install Contract

The web image copies `.npmrc`, `package.json`, `pnpm-lock.yaml`, and
`pnpm-workspace.yaml` before running `pnpm install --frozen-lockfile`. Keep
`.npmrc` committed with the lockfile so Docker uses the same pnpm settings as
local development.

## API Checks

- `GET /health`: process liveness only.
- `GET /ready`: readiness. Returns HTTP 200 only when the database is reachable and SQLx migration metadata exists.
- `POST /graphql`: product GraphQL endpoint. The initial connectivity query is:

```graphql
{
  connectivity {
    service
    status
    databaseReachable
    migrationsApplied
  }
}
```

Responses include an `x-request-id` header. If the request already has `x-request-id`, the API preserves it; otherwise it generates one.

## Local Smoke Checks

The reusable smoke script is the preferred local and CI validation path:

```bash
bash scripts/smoke-compose.sh
```

Expected results:

- `postgres`, `api`, and `web` are healthy.
- `migrate` exited successfully.
- `/ready` returns `status: "ready"` with database and migration booleans true.
- GraphQL `connectivity` returns `urbanlens-api`, `ready`, and true database and migration booleans.
- `/market-map` returns HTTP 200.
- CORS allows only the configured local web origin.
- The six foundation tables are empty, PostGIS/pgcrypto are installed, SQLx has
  exactly two successful migration rows, and the nullable `areas.geometry`
  column is `MultiPolygon` SRID 4326 with its partial GiST index.

The script leaves services running for inspection. Stop and reset the stack
with:

```bash
docker compose down --volumes --remove-orphans
```

## Frontend Checks

```bash
bash scripts/check-web.sh
corepack pnpm --filter @urbanlens/web build
```

`scripts/check-web.sh` runs ESLint, Next type generation plus TypeScript, and
Vitest. It uses `pnpm` when it is on `PATH` and falls back to `corepack pnpm`.

## Rust Checks

```bash
bash scripts/check-rust-docker.sh
```

This MacBook-friendly wrapper runs the same script in the pinned Rust
toolchain container. If host Rust is installed, run the checks directly with
`bash scripts/check-rust.sh`.

```bash
bash scripts/check-rust.sh
```

## Optional MLIT Diagnostic

The MLIT diagnostic is manual, secret-safe, and not part of normal startup or
CI. Set `MLIT_REINFOLIB_API_KEY` in the shell or in a local ignored `.env`, then
run:

```bash
bash scripts/smoke-mlit-api.sh
```

The script makes one bounded authenticated XIT001 request using the documented
`Ocp-Apim-Subscription-Key` header. It prints only HTTP/status/count metadata
and does not persist the response or modify the database.

Without a key, it exits non-zero with a readable message.

## CI

GitHub Actions runs three jobs:

- Rust formatting, Clippy, and tests via `scripts/check-rust.sh`.
- Web lint, type check, Vitest, and production build.
- Docker Compose smoke validation via `scripts/smoke-compose.sh`, with
  `docker compose down --volumes --remove-orphans` registered as an always-run
  teardown step.

## Troubleshooting

- If `/market-map` shows a connection error, verify `docker compose ps` reports
  `api` as healthy and `migrate` as exited successfully.
- If the API is reachable from the host but the browser panel fails, check that
  `NEXT_PUBLIC_GRAPHQL_URL` points to a browser-visible URL and that
  `CORS_ALLOWED_ORIGINS` includes the web origin.
- If `docker compose build` cannot access `~/.docker` or the Docker socket,
  start Docker Desktop and rerun the command outside restricted sandboxing.
- If Docker reports `ERR_PNPM_LOCKFILE_CONFIG_MISMATCH`, make sure the web
  Dockerfile copies `.npmrc` before `pnpm install --frozen-lockfile` and rebuild
  with the latest `main`.
- If `scripts/smoke-compose.sh` fails after a partial startup, inspect
  `docker compose logs --no-color`, then reset with
  `docker compose down --volumes --remove-orphans`.
