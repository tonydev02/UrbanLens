# Architecture

UrbanLens is organized as a local platform with explicit service boundaries:
Next.js renders the analyst workspace, Rust/Actix owns the product API, and
PostgreSQL/PostGIS owns persistence and spatial query execution.

## Local Runtime

```text
docker compose up --build
        |
        v
postgres --healthy--> migrate --success--> api --healthy--> web
```

- `postgres` uses `postgis/postgis:17-3.5` with a named local volume.
- `migrate` runs committed SQLx migrations once and exits.
- `api` starts only after migrations succeed and exposes `/health`, `/ready`,
  and `/graphql`.
- `web` starts after API readiness and serves the analyst shell at
  `/market-map`.
- The web image performs a frozen pnpm install using the committed `.npmrc`,
  workspace manifest, package manifest, and lockfile before copying application
  source.

## API Boundary

The frontend talks to the backend through GraphQL only. The first exposed query
is the platform proof:

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

The browser sends this query to `NEXT_PUBLIC_GRAPHQL_URL`, which defaults to
`http://localhost:8080/graphql`. CORS is bounded to the configured local web
origin.

## Data Flow Target

```text
Official public data
        |
        v
raw dataset retrieval
        |
        v
validation and normalization
        |
        v
PostgreSQL + PostGIS canonical model
        |
        v
GraphQL API
        |
        v
Next.js analyst workspace
```

The current foundation creates the lineage storage backbone but does not import
fixtures or expose raw payload JSON through GraphQL.

## Tradeoffs

- A one-shot migration service keeps startup order explicit and prevents the API
  from hiding schema failures.
- The browser-facing GraphQL URL is separate from container networking because
  the connectivity proof is intentionally made by the browser.
- The market map shell avoids fake data and fake map points until official data,
  location precision, and spatial queries are implemented.

## CI and Smoke Validation

GitHub Actions mirrors the committed developer commands:

- `scripts/check-rust.sh` for Rust formatting, Clippy, and tests.
- `scripts/check-web.sh` plus a production web build for frontend validation.
- `scripts/smoke-compose.sh` for Compose startup, service health, migration
  success, HTTP/GraphQL contracts, CORS behavior, and PostGIS schema assertions.

The Compose smoke job always runs `docker compose down --volumes --remove-orphans`
after validation so CI does not leave containers or volumes behind. The optional
MLIT XIT001 diagnostic is deliberately local-only and excluded from CI because
it requires a developer-owned key and an external service.

## Scaling Considerations

Later phases should keep geographic filtering and aggregation inside PostGIS,
add bounded GraphQL pagination/complexity controls, and preserve source lineage
from every normalized record back to its dataset, import run, and raw record.
