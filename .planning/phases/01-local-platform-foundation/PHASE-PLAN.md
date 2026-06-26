# Phase 01 Plan — Local Platform Foundation

> Purpose: Establish a reproducible local platform that starts the UrbanLens web application, API, migrations, and PostGIS database with one command.

## Metadata

| Field | Value |
|---|---|
| Phase | `01` |
| Name | `Local Platform Foundation` |
| Status | `completed` |
| Owner | `Project owner` |
| Created | `2026-06-24` |
| Last Updated | `2026-06-26` |
| Target Milestone | `MVP local platform foundation` |
| Related ADRs | `docs/adr/001-use-postgis-for-spatial-queries.md`, `002-use-graphql-for-product-api.md`, `003-preserve-raw-source-payloads.md`, `004-model-location-precision-explicitly.md`, `005-use-rust-actix-web-for-api.md` |

---

## 1. Objective

### Problem

UrbanLens has source-grounded product and domain decisions but no runnable application platform. A new developer cannot yet start a web application, API, PostGIS database, or migration workflow, and there is no automated path proving that GraphQL reaches PostgreSQL.

### Intended Outcome

Create the smallest production-oriented local foundation for later ingestion and analyst features. From a clean clone, `docker compose up --build` starts PostgreSQL/PostGIS, applies migrations once, starts the Rust API, and starts the Next.js web application. The web application visibly confirms the GraphQL API and database connection.

### Why This Matters

Every later MVP capability depends on a reliable, observable, and documented runtime boundary. Establishing it now keeps ingestion, spatial queries, and frontend work focused on product behavior instead of repeatedly revisiting local tooling and service lifecycle decisions.

---

## 2. Scope

### In Scope

- [x] Create Cargo and pnpm workspaces for `apps/api`, `apps/web`, `workers/importer`, and `crates/domain`.
- [x] Add a root Compose entrypoint backed by infrastructure definitions under `infra/`.
- [x] Start `web`, `api`, and `postgres` with `docker compose up --build`; run migrations through a one-shot `migrate` service.
- [x] Add PostGIS/pgcrypto extensions and the first lineage-compatible schema migrations.
- [x] Implement HTTP liveness/readiness endpoints and the GraphQL connectivity query.
- [x] Implement the first application shell, market-map placeholder, navigation, connectivity state, loading state, and error boundary.
- [x] Add structured request logging, request IDs, bounded local CORS, tests, CI, and local-development documentation.
- [x] Add an optional secret-safe MLIT API connectivity script that is not required for normal startup or CI.

### Out of Scope

- Production ingestion, normalization, fixture loading, import scheduling, or transaction persistence.
- Transaction, property, market-metric, provenance, or area product queries beyond the platform connectivity query.
- Real map rendering, MapLibre/Leaflet, geographic enrichment, boundaries, or seeded area data.
- Authentication, authorization, Redis, saved searches, monitoring services, deployment, or production secrets management.
- Polished visual design, responsive optimization beyond basic tolerance, or marketing content.
- Canonical station/property identity or any inference that changes Phase 0 location-precision decisions.

### Deferred Ideas

- Phase 02 implements official-source ingestion and normalized transaction observations.
- Phase 03 implements PostGIS viewport/area queries and imports authoritative geography.
- Phase 04 introduces the real analyst map and filter workflow.
- Redis, Grafana, Prometheus scraping, and error tracking remain deferred until a concrete operational need exists.

### Required Implementation Deliverables

| Area | Deliverable |
|---|---|
| Workspace | Root Cargo workspace, pnpm workspace, lockfiles, pinned toolchains, API/domain/importer crates, and web package |
| Infrastructure | Root Compose entrypoint, `infra/docker-compose.yml`, Dockerfiles, health checks, named Postgres volume, and `.dockerignore` files |
| Database | SQLx migrations for extensions and six foundation tables, plus migration tests |
| API | Actix server, SQLx pool, async-graphql schema, health/readiness routes, bounded CORS, request logging, and request IDs |
| Frontend | Next.js App Router shell, `/market-map`, root redirect, API connectivity panel, loading/error states, and component tests |
| CI | Rust, TypeScript/frontend, and Docker Compose smoke-test jobs |
| Documentation | `README.md`, `docs/local-development.md`, `docs/architecture.md`, and updated `.env.example` |
| Optional validation | `scripts/smoke-mlit-api.sh` using the local API key without printing or persisting it |

---

## 3. Requirements

### Functional Requirements

| ID | Requirement | Priority | Notes |
|---|---|---:|---|
| FR-01 | `docker compose up --build` starts the complete local platform from the repository root without requiring a local `.env`. | Must | Safe development defaults live in Compose; `.env` overrides them. |
| FR-02 | PostgreSQL is healthy before migrations run; the API starts only after migrations succeed; the web starts only after API readiness. | Must | A successful one-shot `migrate` container may remain exited. |
| FR-03 | `GET /health` reports process liveness without querying PostgreSQL. | Must | A database outage must not make liveness fail. |
| FR-04 | `GET /ready` runs a database and SQLx migration-ledger readiness check, returns `200` when ready, and returns `503` when PostgreSQL or successful migrations are unavailable. | Must | Response remains structured and readable. |
| FR-05 | `POST /graphql` exposes the typed `connectivity` query and reports actual database/migration connectivity. | Must | No raw or product data query is added. |
| FR-06 | `/` redirects to `/market-map`, which renders the analyst shell, navigation, empty map placeholder, and API/database status. | Must | The route must not imply that transaction data is loaded. |
| FR-07 | Frontend connectivity shows explicit loading, connected, degraded/error, and retry states. | Must | The page must remain understandable if the API is unavailable. |
| FR-08 | A bounded MLIT API smoke script can be run when `MLIT_REINFOLIB_API_KEY` exists. | Should | It must not echo the key or run in CI. |

### Non-Functional Requirements

| ID | Requirement | Priority | Verification Method |
|---|---|---:|---|
| NFR-01 | Toolchains and dependencies are reproducible enough for clean-clone setup. | Must | Pin Node/Rust/PostGIS lines and commit Cargo/pnpm lockfiles. |
| NFR-02 | API logs are structured and include a request ID, method, path, status, and duration without secrets or raw payloads. | Must | Integration test and manual log inspection. |
| NFR-03 | CORS permits only the configured local web origin. | Must | API integration tests cover allowed and disallowed origins. |
| NFR-04 | Service startup and failure behavior are deterministic and observable. | Must | Compose health checks and smoke-test assertions. |
| NFR-05 | No secret is required for the core platform and no real secret is committed, logged, or embedded in an image. | Must | Repository scan and optional-script test. |
| NFR-06 | CI uses the same committed commands and Compose definition documented for developers. | Should | Workflow review and clean-branch CI run. |

### Data / Domain Requirements

| ID | Requirement | Source / Assumption | Notes |
|---|---|---|---|
| DR-01 | Preserve `data_source → dataset → import_run → raw_record` as the physical lineage backbone. | Phase 0 conceptual model and ADR-003 | `datasets` is included even though the original Phase 1 shortlist named five tables. |
| DR-02 | Preserve exact-artifact/record-position identity without collapsing equal source payloads. | Phase 0 idempotency decision | Raw-record uniqueness is based on dataset plus source position, not payload hash. |
| DR-03 | Create `areas` with nullable `geometry(MultiPolygon,4326)` and a GiST index, but insert no area or geometry data. | ADR-001 and Phase 0 deferral | A later authoritative boundary dataset must supply lineage. |
| DR-04 | Use explicit import states and non-negative count fields. | Phase 0 data model | States are pending, running, completed, completed_with_warnings, and failed. |
| DR-05 | Raw payloads use JSONB and remain unavailable through the initial GraphQL schema and production logs. | ADR-003 | This phase creates storage structure only. |
| DR-06 | Do not add `property`, `transaction_observation`, metrics, station identity, or location-precision data. | Phase 0 boundaries | Those require later source-aware implementation. |

---

## 4. Technical Design

### Toolchain and Workspace

- Use Node.js 24 LTS, pnpm 10, Next.js 16 App Router, React, strict TypeScript, ESLint, Vitest, React Testing Library, and jsdom.
- Use Rust 1.96 with edition 2024, a root Cargo workspace, and committed `Cargo.lock`/`rust-toolchain.toml`.
- Use `postgis/postgis:17-3.5` and a named database volume.
- Keep styling in ordinary CSS/CSS modules; do not add Tailwind or another UI framework.
- Use `graphql-request` with TanStack Query for the connectivity request. Do not add Apollo or generated shared types in this phase.
- `workers/importer` is a workspace binary that compiles and reports that ingestion is not implemented; it is not a Compose service.
- `crates/domain` is an intentionally small library crate and must not invent transaction/property fields.

### Compose Lifecycle

```text
postgres starts
  ↓ health check: pg_isready
migrate starts and applies embedded SQLx migrations
  ↓ exits successfully
api starts
  ↓ /ready confirms database and migration readiness
web starts
  ↓ /market-map queries POST /graphql from the browser
```

- The root `compose.yaml` delegates to or includes `infra/docker-compose.yml`, keeping substantive infrastructure configuration under `infra/` while preserving the required root command.
- Compose supplies safe local defaults for database name, user, password, ports, `DATABASE_URL`, `WEB_ORIGIN`, and `NEXT_PUBLIC_GRAPHQL_URL`.
- `.env.example` documents every override and retains an empty `MLIT_REINFOLIB_API_KEY`.
- Default ports are web `3000`, API `8080`, and PostgreSQL `5432`.
- `migrate` and `api` use separate binaries built from the API workspace/package; the migration binary embeds the committed migrations with `sqlx::migrate!()`.
- API startup must not apply migrations itself. A failed migration prevents the API and web from becoming ready.

### Database Migrations

Migration 1 enables:

```sql
CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS pgcrypto;
```

Migration 2 creates the following empty schema:

| Table | Required Foundation |
|---|---|
| `data_sources` | UUID primary key generated by `gen_random_uuid()`, name, publisher, source/license URLs, metadata verification time, created/updated timestamps |
| `datasets` | UUID key, required source FK, source dataset name, retrieval method/query JSONB, optional source version, retrieval time, SHA-256 artifact checksum, format, record count, timestamps |
| `import_runs` | UUID key, required dataset FK, lifecycle timestamps/status, normalization version, six non-negative count fields, nullable error kind, timestamps |
| `raw_records` | UUID key, required dataset/import-run pair, source position, optional external ID, JSONB payload, SHA-256 payload hash, validation status/errors, created timestamp |
| `validation_issues` | UUID key, required import-run FK, optional raw-record FK, issue code, warning/rejection severity, optional field/value summary, message, disposition, created timestamp |
| `areas` | UUID key, required dataset FK, source code, name, area type, nullable `geometry(MultiPolygon,4326)`, timestamps |

Constraints and indexes:

- Foreign keys use restrictive deletion by default so lineage cannot disappear silently.
- `import_runs` has a unique `(id, dataset_id)` key; `raw_records` uses a composite FK so its dataset and import run cannot disagree.
- `raw_records` is unique on `(dataset_id, source_position)`; identical payload hashes at different positions remain distinct.
- SHA-256 fields are lowercase 64-character hexadecimal text with checks.
- Import counters and dataset record counts are non-negative.
- Completed/failed states require `completed_at`; pending/running states do not.
- `areas` is unique on `(dataset_id, area_type, source_code)` and has a partial GiST index for non-null geometry.
- Migrations contain no fixture, source, area, or transaction seed rows and can be rerun safely by SQLx.

### API Foundation

- Actix Web owns `/health`, `/ready`, and `/graphql`; async-graphql owns the GraphQL schema.
- SQLx creates one PostgreSQL pool from `DATABASE_URL` with bounded connection/time-out settings.
- `GET /health` always returns `200` while the process is running:

```json
{"status":"ok"}
```

- `GET /ready` executes `SELECT 1` and verifies successful SQLx migration metadata with no failed migration rows. Success returns `200`:

```json
{"status":"ready","database_reachable":true,"migrations_applied":true}
```

- Database failure returns `503` without leaking driver details:

```json
{"status":"not_ready","database_reachable":false,"migrations_applied":false}
```

- `POST /graphql` exposes exactly:

```graphql
type Query {
  connectivity: Connectivity!
}

type Connectivity {
  service: String!
  status: String!
  databaseReachable: Boolean!
  migrationsApplied: Boolean!
}
```

- The resolver runs its own bounded connectivity check. A database or migration-ledger error returns `connectivity { status: "not_ready", databaseReachable: false, migrationsApplied: false }` or the nearest true/false state rather than exposing an internal GraphQL error.
- Configure a conservative GraphQL depth/complexity limit even though the initial schema is small.
- Accept CORS from `WEB_ORIGIN` only, include `content-type` and request-ID headers, and reject arbitrary origins.
- Generate or preserve `x-request-id`, return it in the response, and include it in request logging output.

### Frontend Foundation

- Use the Next.js App Router with `/` redirecting to `/market-map`.
- The root layout provides the UrbanLens title, one active `Market Map` navigation link, main-content landmark, and a restrained desktop-first shell.
- `/market-map` renders an honest empty map panel stating that transaction geography arrives in a later phase.
- A client-side status panel uses TanStack Query and `graphql-request` to run the exact GraphQL connectivity query.
- Loading shows a bounded status skeleton; success identifies API and PostgreSQL connectivity; degraded/network error shows readable copy and a retry action.
- Add route-level `loading.tsx`, `error.tsx`, and a not-found state. Errors must not collapse the application shell.
- Do not install a map library, draw fake points, show market metrics, or use sample transaction values.

### Optional MLIT Smoke Check

- Add `scripts/smoke-mlit-api.sh` as a manual developer diagnostic.
- Require `MLIT_REINFOLIB_API_KEY`; exit with a clear message when it is absent.
- Make one bounded request against a documented MLIT transaction-information endpoint/query with timeouts and failure handling.
- Pass the secret only through the required request header; never enable shell tracing, echo headers, persist a response containing credentials, or print the key.
- Report only HTTP/result success metadata needed for connectivity. Do not import or modify data.
- Exclude the script from CI and Phase 1 exit criteria because CI must not require a private credential.

---

## 5. Implementation Slices

### Slice 1 — Workspace, Toolchains, and Environment Contract

**Goal**

Create compiling Rust/TypeScript workspace foundations and a single documented environment contract.

**Tasks**

- [x] Add root Cargo/pnpm workspace manifests, lockfiles, Rust toolchain, Node/pnpm version declarations, and shared scripts.
- [x] Scaffold the API, domain, importer, and Next.js packages without domain or ingestion behavior.
- [x] Expand `.gitignore`, `.dockerignore`, and `.env.example` for generated output, local environments, ports, database configuration, origins, and optional MLIT access.
- [x] Add root and package-level check/test scripts used identically by CI.

**Expected Evidence**

- [x] Rust workspace formatting, lint, and tests pass in the pinned Rust 1.96.0 container.
- [x] pnpm frozen-lockfile install, lint, type check, initial test, and Next.js production build pass.

### Slice 2 — PostGIS, Migrations, and Compose Lifecycle

**Goal**

Make one command initialize a healthy PostGIS database and apply the lineage schema before application startup.

**Tasks**

- [x] Add root/infra Compose definitions, PostGIS service, volume, health checks, and safe defaults.
- [x] Add extension/schema migrations and a dedicated embedded migration binary.
- [x] Add database smoke coverage for extensions, tables, constraints, indexes, foreign keys, and migration reruns.
- [x] Wire `migrate` to healthy Postgres and API startup to successful migration completion.

**Expected Evidence**

- [x] Fresh and existing volumes both reach a migrated state without manual intervention.
- [x] Schema inspection proves six empty tables, both extensions, and the area spatial index exist.

**Slice 2 Implementation Note — 2026-06-25**

- [x] Root/infra Compose definitions, PostGIS service, named volume, safe defaults, and Postgres health check were added.
- [x] SQLx extension/schema migrations and the dedicated embedded migration binary were added.
- [x] Reusable Compose smoke asserts the critical database catalog contract; deeper insertion tests are deferred to ingestion behavior.
- [x] API startup is gated on `migrate` completing successfully; `migrate` remains gated on healthy Postgres.
- [x] Disposable Compose validation proved fresh-volume migration, migration rerun, the two SQLx migration ledger rows, both extensions, six empty foundation tables, `areas.geometry` as `MultiPolygon` SRID 4326, and the partial GiST index.

### Slice 3 — Observable Actix and GraphQL API

**Goal**

Expose stable health contracts proving that the API can reach PostgreSQL.

**Tasks**

- [x] Build API configuration, SQLx pool, Actix routes, async-graphql schema, and startup wiring.
- [x] Implement liveness, readiness, GraphQL connectivity, bounded CORS, request IDs, and request logging.
- [x] Add integration coverage for database failure, GraphQL response shape, and CORS through reusable Compose smoke and failure/recovery UAT.
- [x] Add an API container health check against `/ready`.

**Expected Evidence**

- [x] Endpoint response contracts compile and request-ID behavior is unit-tested; full live endpoint smoke remains for UAT.
- [x] Database unavailability leaves `/health` live while `/ready` and GraphQL report degraded connectivity.

**Slice 3 Implementation Note — 2026-06-25**

- [x] Added `ApiConfig` with `API_HOST`, `API_PORT`, `DATABASE_URL`, `API_DATABASE_MAX_CONNECTIONS`, `CORS_ALLOWED_ORIGINS`, and `WEB_ORIGIN` fallback.
- [x] Added SQLx pool creation with bounded connection count and acquire timeout.
- [x] Added Actix `/health`, `/ready`, and `/graphql` routes.
- [x] Added GraphQL `connectivity` response with service, status, database reachability, and migration-applied state.
- [x] Added generated/preserved `x-request-id` response headers and request logging.
- [x] Added bounded CORS that rejects wildcard/non-HTTP origins during config parsing.
- [x] Added `urbanlens-healthcheck` binary and Docker image copy.
- [x] Added Compose `api` service that depends on healthy Postgres and successful `migrate`, and uses the API healthcheck.
- [x] Added `docs/local-development.md` for API config and health/connectivity checks.
- [x] Verified `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `docker compose config`, and `docker compose build api`.

### Slice 4 — Analyst Shell and Connectivity States

**Goal**

Give developers a truthful browser-visible proof that web, API, and database are connected.

**Tasks**

- [x] Build the App Router layout, navigation, root redirect, and empty `/market-map` route.
- [x] Configure QueryClient and GraphQL request boundaries using the public API URL.
- [x] Implement loading, connected, degraded/error, retry, route-error, and not-found states.
- [x] Add accessible component tests without introducing fake market data or a map library.

**Expected Evidence**

- [x] A browser-equivalent request to `http://localhost:3000/market-map` succeeds and frontend component tests cover connected/degraded states.
- [x] Simulated loading/failure tests prove the application remains readable and retryable.

**Slice 4 Implementation Note — 2026-06-26**

- [x] Added `Providers` with TanStack Query and a browser-side `graphql-request` connectivity client using `NEXT_PUBLIC_GRAPHQL_URL`.
- [x] Added the App Router shell, active Market Map navigation, `/` redirect, `/market-map`, route loading/error states, and global not-found state.
- [x] Added an honest empty map panel that states transaction geography is not loaded and avoids fake points, metrics, or provenance claims.
- [x] Added component tests for root redirect, market-map copy, connectivity loading/success/degraded/error/retry, route error, and not-found behavior.
- [x] Added `infra/web.Dockerfile` and Compose `web` service gated on API health.
- [x] Updated README, architecture, and local-development docs for the browser-facing GraphQL proof.
- [x] Local Compose success-path smoke passed after the Dockerfile `.npmrc` fix.
- [x] Failure-mode Compose UAT passed; screenshot tooling was unavailable, so route HTTP proof and component tests are the recorded frontend evidence.

### Slice 5 — CI, Smoke Test, Documentation, and UAT Readiness

**Goal**

Make the clean-clone workflow repeatable for developers and CI.

**Tasks**

- [x] Add GitHub Actions jobs for Rust checks, frontend checks, and Docker Compose smoke validation.
- [x] Make the Compose job build, start detached, poll readiness, call GraphQL, check the web route, inspect migration success, and always run `docker compose down --volumes`.
- [x] Add the optional MLIT connectivity script and test its missing-key/secret-safe behavior locally.
- [x] Write the README, architecture, and local-development instructions with exact commands and troubleshooting.
- [x] Execute every required case in `PHASE-UAT.md` on a clean working tree.

**Expected Evidence**

- [x] All required CI jobs pass; the user confirmed GitHub Actions checked green on `2026-06-26`.
- [x] A new developer can follow the documented one-command path without undocumented setup.

**Slice 5 Implementation Note — 2026-06-26**

- [x] Added `.github/workflows/ci.yml` with Rust, web, and Docker Compose smoke jobs.
- [x] Added `scripts/smoke-compose.sh`, which builds/starts the stack, polls liveness/readiness, checks GraphQL, CORS, request IDs, web HTTP 200, migration exit status, SQLx ledger state, extensions, empty foundation tables, lineage constraints, and the area spatial index.
- [x] Added `scripts/smoke-mlit-api.sh`, a manual XIT001 diagnostic that reads `MLIT_REINFOLIB_API_KEY` from the environment or ignored `.env`, passes it only in the required header, prints only bounded metadata, and exits clearly when the key is absent.
- [x] Verified fresh-volume Compose smoke, existing-volume rerun smoke, database outage behavior, API outage behavior, web route availability, Rust checks, web checks, production build, and missing-key diagnostic behavior locally.
- [x] Remote GitHub Actions checked green per user confirmation on `2026-06-26`.

---

## 6. Testing Strategy

### Unit and Component Tests

| Area | Required Coverage |
|---|---|
| API response mapping | Liveness payload, readiness success/failure, GraphQL `ready`/`not_ready`, and safe errors |
| Request middleware | Generated/preserved request IDs and structured response metadata |
| Frontend shell | Root navigation, empty map copy, landmarks, and active route |
| Frontend connectivity | Loading, connected, degraded/network error, and retry behavior |
| Frontend boundaries | Route error state and not-found state remain readable |
| Optional script | Missing-key failure and absence of secret disclosure in output |

### Integration Tests

| Flow | Expected Result |
|---|---|
| Fresh migration | Both extensions, all six tables, constraints, and indexes are created |
| Migration rerun | SQLx reports no pending migration and schema remains unchanged |
| API with PostgreSQL | `/ready` is 200 and GraphQL returns `connectivity.status = "ready"` with database/migration booleans true |
| API without PostgreSQL | `/health` remains 200, `/ready` is 503, GraphQL returns `connectivity.status = "not_ready"` without leaking driver details |
| CORS | Configured web origin is allowed; another origin is not |
| Compose dependency chain | Migration completes before API/web become ready |

### CI Checks

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
pnpm --filter web lint
pnpm --filter web typecheck
pnpm --filter web test --run
docker compose up --build -d
HTTP/GraphQL/web/migration assertions
docker compose down --volumes
```

The teardown command must be registered as an always-run CI step, including after failed assertions.

### Regression Risks

| Risk Area | Possible Regression | Mitigation |
|---|---|---|
| Compose ordering | API races migrations or web races API | Health-gated dependencies plus end-to-end smoke test |
| Browser networking | Container-internal API URL is exposed to the browser | Separate explicit browser URL and Compose service URL where needed |
| Database health | Liveness incorrectly depends on PostgreSQL | Separate route tests with database unavailable |
| Lineage schema | Equal rows are accidentally collapsed | Unique dataset-position test with equal payload hashes at distinct positions |
| Spatial schema | Wrong SRID/type or missing index | Catalog assertions and PostGIS geometry test |
| Secrets | MLIT key leaks through logs or CI | No CI credential, no shell tracing, output scan, empty example value |

---

## 7. Acceptance Criteria

### Product / User Criteria

- [x] From a clean clone, `docker compose up --build` starts the required platform without a private credential or pre-created `.env`.
- [x] Visiting `/market-map` shows the application shell and an honest empty foundation state.
- [x] The page visibly confirms when GraphQL and PostgreSQL are connected and gives a readable retryable state when they are not.
- [x] No screen suggests that transaction, map, metric, or property data is available.

### Engineering Criteria

- [x] `web`, `api`, and `postgres` remain healthy; `migrate` exits zero after applying all migrations.
- [x] Both extensions, all six tables, lineage constraints, raw-record uniqueness, and the area GiST index exist.
- [x] All three endpoint contracts match the plan, and GraphQL reaches PostgreSQL.
- [x] Structured logs and request IDs work without exposing secrets or raw payloads.
- [x] Rust/frontend checks and Docker Compose smoke tests pass through the CI command boundaries; GitHub Actions checked green.
- [x] Restarting against an existing volume does not fail or reapply migrations incorrectly.

### Documentation Criteria

- [x] README explains purpose, architecture, stack, one-command setup, checks, source boundary, known limitations, and future phases.
- [x] `docs/local-development.md` documents prerequisites, ports, environment overrides, start/stop/reset/test commands, health checks, and troubleshooting.
- [x] `docs/architecture.md` documents service/data flow, migration lifecycle, API boundary, lineage foundation, tradeoffs, and deferred scaling concerns.
- [x] `.env.example` contains every supported variable with empty optional secrets and no working credential.
- [x] No ADR is added unless implementation materially departs from accepted ADRs 001–005.

### UAT Criteria

- [x] All required UAT cases pass on the committed Compose configuration.
- [x] Failure/edge cases include PostgreSQL unavailability, API unavailability, migration rerun, and clean restart.
- [x] Evidence includes endpoint output, GraphQL output, schema inspection, frontend route output/component states, Compose status, and local CI-equivalent command results.
- [x] No critical or high-severity defect remains open.

---

## 8. Dependencies, Risks, and Open Questions

### Dependencies

| Dependency | Owner / Source | Status | Impact if Missing |
|---|---|---|---|
| Docker Engine with Compose | Developer / CI | Required; unavailable in the current implementation environment | One-command platform and smoke UAT cannot run |
| Node 24 and pnpm 10 | Container/CI toolchain | Implemented locally through Corepack/pnpm and the web Dockerfile | Frontend local checks cannot run outside containers |
| Rust 1.96 | Container/CI toolchain | Planned; host Cargo is absent | Rust checks must initially run in Docker/CI |
| `postgis/postgis:17-3.5` | Docker PostGIS project | Selected | Database/spatial foundation cannot start |
| MLIT key | Developer-local only | Available, optional | Only optional connectivity check is skipped |

### Risks

| Risk | Likelihood | Impact | Mitigation |
|---|---:|---:|---|
| Cross-platform Compose include/paths behave differently | Medium | Medium | Document minimum Compose version and validate root command in CI |
| Rust image builds are slow for new contributors | Medium | Medium | Layer dependency builds and use CI caches without adding another build service |
| Browser/API origin configuration differs inside/outside Compose | Medium | High | Separate explicit URLs/origin variables, document browser-facing `NEXT_PUBLIC_GRAPHQL_URL`, and test browser-visible connectivity |
| Foundational schema overcommits before ingestion evidence | Low | High | Limit tables to Phase 0 concepts and allow no domain/product rows |
| Health checks mask database outages | Low | High | Separate liveness/readiness contracts and failure tests |
| Optional source test leaks its key | Low | High | No shell tracing/header output, bounded script, no CI secret |

### Open Questions

No implementation-blocking questions remain. If a selected dependency cannot support Node 24, Rust 1.96, Next.js 16, or PostgreSQL 17, record the compatibility evidence and update this plan before substituting a different version line.

---

## 9. Planning Decisions

| Decision | Rationale | ADR Required? |
|---|---|---:|
| Add a physical `datasets` table now | Preserves the accepted exact-artifact lineage between source and import run | No — implements ADR-003/data model |
| Use a one-shot Compose migration service | Keeps API startup focused and makes migration failure/order visible | No |
| Allow core startup without `.env` | Meets the clean-clone one-command exit gate; overrides remain documented | No |
| Use Next.js App Router and ordinary CSS | Provides native layouts/loading/errors without a premature UI framework | No |
| Use `graphql-request` plus TanStack Query | Small typed-enough connectivity layer without committing to a heavy GraphQL cache | No |
| Keep the MLIT check optional and local | Validates approved access without coupling platform startup or CI to a private credential/external service | No |
| Keep PostgreSQL 17/PostGIS 3.5 for Phase 1 | Stable supported image line with conventional data-volume behavior | No |

---

## 10. Completion Definition

This phase is complete when:

- [x] All in-scope requirements are implemented.
- [x] `docker compose up --build` satisfies the clean-clone workflow.
- [x] All required automated checks and CI jobs pass.
- [x] Required documentation is complete and accurate.
- [x] All required UAT cases in `PHASE-UAT.md` pass.
- [x] No critical or high-severity defect remains open.
- [x] `PHASE-STATUS.md` is updated to `completed`.
- [x] `.planning/STATE.md` points to Phase 02 with a concrete resume action.

---

## 11. Handoff Notes

Complete this section when Phase 01 closes.

### What Is Now Available

- Root Docker Compose starts PostGIS, migration, API, and web with no secret or local `.env`.
- `scripts/smoke-compose.sh` provides reusable local/CI validation for startup, readiness, GraphQL, CORS, request IDs, migration state, and schema foundation.
- GitHub Actions CI defines Rust, web, and Compose smoke jobs with always-run Compose teardown.
- `scripts/smoke-mlit-api.sh` provides a local-only, secret-safe XIT001 connectivity diagnostic.

### Important Constraints

- Preserve Phase 0 lineage, source, metric, and location-precision decisions.
- Do not treat a healthy platform as evidence that ingestion or analyst functionality exists.

### Deferred Work

- Real ingestion, transaction observations, area data, spatial search, and market UI remain in later phases.

### Recommended First Action for the Next Phase

- Create/activate the Phase 02 planning documents, then design the importer around the committed official fixture and the physical source/dataset/import/raw lineage.
