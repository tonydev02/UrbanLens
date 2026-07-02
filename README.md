# UrbanLens

UrbanLens is a public-data commercial real-estate intelligence platform for
Tokyo. It focuses on trustworthy ingestion, lineage, spatial search, and
analyst-facing market exploration rather than property marketplace workflows.

## Current MVP Foundation

- Rust/Actix GraphQL API with `/health`, `/ready`, request IDs, and bounded CORS.
- PostgreSQL/PostGIS database with SQLx migrations for lineage plus canonical
  transaction/location schema contracts.
- Next.js analyst shell at `/market-map`.
- Browser-visible GraphQL `connectivity` proof showing API, PostgreSQL, and
  migration readiness.
- Docker Compose lifecycle for `postgres`, `migrate`, `api`, and `web`.
- GitHub Actions workflow and reusable Compose smoke validation.
- Repeat-safe MLIT transaction fixture importer with raw-record lineage,
  normalized observations, validation issue storage, and bounded GraphQL
  inspection.

The map route intentionally contains no fake transaction points or metrics. The
first official transaction fixture can be imported locally, but Phase 03 still
owns spatial query behavior because CSV observations do not include defensible
map geometry.

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
lineage and transaction schema contracts. It leaves the stack running for
inspection.

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

## Import The MLIT Fixture

After the Compose stack is healthy, import the committed official MLIT
transaction CSV fixtures:

```bash
./scripts/import-fixture.sh
```

The importer writes one import run per CSV artifact, preserves raw records and
payload hashes, normalizes transaction observations, and is safe to rerun. The
default fixture path is `workers/importer/fixtures/transactions/`; see
[docs/importer.md](docs/importer.md) for counters, GraphQL inspection examples,
and troubleshooting.

## Data Sources

The first planned source is MLIT real-estate transaction data. Source metadata,
license notes, retrieval method, and limitations are documented in
[docs/data-sources.md](docs/data-sources.md).

## Known Limitations

- CSV transaction observations import with `location_precision=unknown` and no
  geometry, so they must not be rendered as property or station points.
- No live map library or spatial product query is implemented yet.
- Area metrics, provenance drawers, saved searches, and import operations are
  planned for later MVP phases.
- Docker Compose smoke validation requires a running Docker daemon.
