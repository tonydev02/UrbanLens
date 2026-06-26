# Local Development

UrbanLens runs its local platform through Docker Compose. The required path does
not need a local `.env`; `.env.example` only documents overrides.

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

## Frontend Checks

```bash
bash scripts/check-web.sh
corepack pnpm --filter @urbanlens/web build
```

`scripts/check-web.sh` runs ESLint, Next type generation plus TypeScript, and
Vitest. It uses `pnpm` when it is on `PATH` and falls back to `corepack pnpm`.

## Troubleshooting

- If `/market-map` shows a connection error, verify `docker compose ps` reports
  `api` as healthy and `migrate` as exited successfully.
- If the API is reachable from the host but the browser panel fails, check that
  `NEXT_PUBLIC_GRAPHQL_URL` points to a browser-visible URL and that
  `CORS_ALLOWED_ORIGINS` includes the web origin.
- If `docker compose build` cannot access `~/.docker` or the Docker socket,
  start Docker Desktop and rerun the command outside restricted sandboxing.
