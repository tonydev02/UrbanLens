# UrbanLens

UrbanLens is a public-data commercial real-estate intelligence platform for
Tokyo. It focuses on trustworthy ingestion, lineage, spatial search, and
analyst-facing market exploration rather than property marketplace workflows.

## Current MVP Foundation

- Rust/Actix GraphQL API with `/health`, `/ready`, request IDs, and bounded CORS.
- PostgreSQL/PostGIS database with SQLx migrations for the lineage foundation.
- Next.js analyst shell at `/market-map`.
- Browser-visible GraphQL `connectivity` proof showing API, PostgreSQL, and
  migration readiness.
- Docker Compose lifecycle for `postgres`, `migrate`, `api`, and `web`.
- GitHub Actions workflow and reusable Compose smoke validation.

The map route intentionally contains no fake transaction points or metrics.
Official transaction data arrives through later ingestion slices.

## Architecture

```text
Official public data sources
        |
        v
Raw retrieval and validation
        |
        v
PostgreSQL + PostGIS canonical model
        |
        v
Rust Actix + async-graphql API
        |
        v
Next.js analyst workspace
```

See [docs/architecture.md](docs/architecture.md) and
[docs/data-model.md](docs/data-model.md) for current decisions.

## Local Setup

```bash
docker compose up --build
```

Open `http://localhost:3000/market-map`.

No local `.env` or secret is required for the core platform. Use
[.env.example](.env.example) only when overriding ports, origins, or optional
source diagnostics.

## Smoke Proof

```bash
bash scripts/smoke-compose.sh
```

The smoke script builds and starts the stack, checks service health, verifies
`migrate` exits successfully, calls `/health`, `/ready`, GraphQL
`connectivity`, CORS preflights, `/market-map`, and inspects the PostGIS
lineage schema. It leaves the stack running for inspection.

## Checks

```bash
bash scripts/check-rust-docker.sh
bash scripts/check-web.sh
corepack pnpm --filter @urbanlens/web build
docker compose config
bash scripts/smoke-compose.sh
```

The web Docker build uses the committed `.npmrc` before
`pnpm install --frozen-lockfile`, so container installs use the same peer
dependency settings as the lockfile.

If host Rust is installed, `bash scripts/check-rust.sh` runs the same Rust
formatting, lint, and test checks without Docker.

To stop and reset the local stack after smoke validation:

```bash
docker compose down --volumes --remove-orphans
```

Optional MLIT connectivity can be checked only on a developer machine with an
ignored local key:

```bash
bash scripts/smoke-mlit-api.sh
```

That diagnostic is intentionally excluded from CI and does not import data.

## Data Sources

The first planned source is MLIT real-estate transaction data. Source metadata,
license notes, retrieval method, and limitations are documented in
[docs/data-sources.md](docs/data-sources.md).

## Known Limitations

- No transaction observations are imported yet.
- No live map library or spatial product query is implemented yet.
- Area metrics, provenance drawers, saved searches, and import operations are
  planned for later MVP phases.
- Docker Compose smoke validation requires a running Docker daemon.
