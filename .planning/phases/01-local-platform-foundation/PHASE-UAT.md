# Phase 01 UAT — Local Platform Foundation

> Purpose: Prove that a new developer can start, inspect, fail, restart, and validate the complete local platform rather than merely confirming that its source files exist.

## Metadata

| Field | Value |
|---|---|
| Phase | `01` |
| Name | `Local Platform Foundation` |
| UAT Status | `passed` |
| Environment | `clean local clone with Docker Compose` |
| Tester | `Codex` |
| Started | `2026-06-24` |
| Completed | `2026-06-26 15:50 +07:00` |
| Build / Commit | `working tree based on ffe6f2d; GitHub Actions checked green per user confirmation` |
| Related Plan | `PHASE-PLAN.md` |
| Related Status | `PHASE-STATUS.md` |

---

## 1. UAT Objective

Verify that UrbanLens has a reproducible local platform: one root command starts a migrated PostGIS database, an observable Rust/Actix GraphQL API, and a Next.js analyst shell; the browser visibly proves database connectivity; expected dependency failures are honest; restart is safe; and CI exercises the same behavior.

---

## 2. Preconditions

### Required Setup

- [x] Correct clean branch/commit is checked out.
- [x] Docker Engine and a supported Docker Compose version are available.
- [x] Ports `3000`, `8080`, and `5432` are free or documented overrides are configured.
- [x] No existing UrbanLens containers or volumes can mask fresh-clone behavior.
- [x] No `.env` is required for required UAT cases.
- [x] Browser-equivalent frontend evidence is available through HTTP route proof and component tests; screenshot tooling is not installed in this environment.
- [x] CI workflow has run for the tested commit and checked green per user confirmation on `2026-06-26`.

### Test Data

| Data Set / Fixture | Purpose | Setup Command / Location |
|---|---|---|
| Empty migrated database | Prove foundation behavior without pretending ingestion exists | Created by `docker compose up --build` |
| Phase 0 MLIT fixture | Must remain untouched; not loaded in this phase | `workers/importer/fixtures/transactions/` |
| Optional authenticated query | Manual external connectivity only | `scripts/smoke-mlit-api.sh` with local key |

### Known Limitations

- No transaction observations, areas, metrics, or spatial marks are loaded.
- The market-map route is intentionally an empty foundation state.
- The optional MLIT connectivity case depends on external availability and a developer-local key; it is not part of the pass/fail total.
- This UAT validates local/CI operation, not production deployment or security hardening.

---

## 3. Acceptance Criteria Traceability

| UAT ID | Related Acceptance Criteria | Scenario | Required Result |
|---|---|---|---|
| UAT-01 | FR-01, FR-02; Product 1; Engineering 1 | Clean-clone one-command startup | Required services healthy; migration exits successfully |
| UAT-02 | DR-01–DR-05; Engineering 2 | Extension and schema foundation | Correct empty schema, lineage constraints, and spatial index |
| UAT-03 | FR-03–FR-05; Engineering 3–4 | HTTP and GraphQL contracts | Exact status/payload behavior and request IDs |
| UAT-04 | FR-06–FR-07; Product 2–4 | Frontend analyst shell/connectivity | Honest route, navigation, connected state, no fake data |
| UAT-05 | FR-03–FR-07; UAT edge criteria | PostgreSQL/API failure behavior | Liveness separated from readiness; UI readable/retryable |
| UAT-06 | FR-02; Engineering 6 | Migration rerun and clean restart | No duplicate migration/data failure |
| UAT-07 | NFR-05; Documentation criteria | Environment and secret safety | Core startup secret-free; no credential disclosure |
| UAT-08 | Engineering 5; Documentation/UAT criteria | CI and clean documentation path | All required jobs and documented commands pass |
| UAT-09 | FR-08 | Optional MLIT connectivity | Bounded success without key disclosure; non-blocking |

---

## 4. Required UAT Test Cases

### UAT-01 — Clean-Clone One-Command Startup

**Purpose**

Prove that the primary Phase 1 deliverable works without undocumented preparation.

**Preconditions**

- [x] No `.env` exists or is sourced.
- [x] UrbanLens Compose containers and named volumes are absent.

**Steps**

1. From the repository root, run `docker compose up --build -d`.
2. Poll until Compose reports stable service states or the documented timeout expires.
3. Run `docker compose ps` and inspect the `migrate` container exit result.
4. Open or request `http://localhost:3000/market-map`.

**Expected Result**

- [x] `postgres`, `api`, and `web` are running and healthy.
- [x] `migrate` ran after healthy PostgreSQL and exited with code 0.
- [x] API/web did not become ready before migration completion.
- [x] The market-map route returns HTTP 200.
- [x] No secret, manual migration command, or local `.env` was required.

**Actual Result**

UAT-01 success path passed on 2026-06-26:

- `docker compose up --build -d` built `urbanlens-api`, `urbanlens-migrate`, and `urbanlens-web`.
- `postgres` became healthy before `migrate` started.
- `migrate` exited successfully before `api` started.
- `api` became healthy before `web` started.
- `web` became healthy and exposed `http://localhost:3000/market-map`.
- `curl -I http://127.0.0.1:3000/market-map` returned HTTP 200.

No secret, manual migration command, or local `.env` was required.

**Status:** `passed`

**Evidence**

Captured by `bash scripts/smoke-compose.sh` on a fresh volume: Compose config rendered, images built, `postgres`/`api`/`web` healthy, `migrate` exited 0, and `/market-map` reachable.

### UAT-02 — PostGIS and Lineage Schema Foundation

**Purpose**

Prove that migrations create the correct empty physical foundation without seed or product data.

**Preconditions**

- [x] UAT-01 passed.
- [x] Database inspection uses the documented local command.

**Steps**

1. Query installed extensions for `postgis` and `pgcrypto`.
2. Query catalog tables for `data_sources`, `datasets`, `import_runs`, `raw_records`, `validation_issues`, and `areas`.
3. Inspect foreign keys, check constraints, unique keys, and the partial GiST area index.
4. Confirm the area geometry type/SRID.
5. Query row counts for all six tables.
6. Execute committed migration integration tests.

**Expected Result**

- [x] Both required extensions exist.
- [x] All six tables exist and are empty.
- [x] Source/dataset/import/raw foreign keys preserve the planned lineage.
- [x] Dataset/import-run consistency and dataset/source-position uniqueness are enforced.
- [x] Import states/counters, hashes, and severity fields have the documented constraints.
- [x] `areas.geometry` is nullable `MultiPolygon` SRID 4326 with a non-null partial GiST index.
- [x] No `property`, `transaction_observation`, station, metric, fixture, or area row was created.

**Actual Result**

UAT-02 passed on 2026-06-26 through `bash scripts/smoke-compose.sh`.

Verified:

- `postgis` and `pgcrypto` extensions exist.
- `data_sources`, `datasets`, `import_runs`, `raw_records`, `validation_issues`, and `areas` exist and are empty.
- `areas.geometry` is nullable `MULTIPOLYGON` with SRID 4326.
- `areas_geometry_gix` exists as a partial GiST index where `geometry IS NOT NULL`.
- Lineage constraints `raw_records_dataset_position_unique`, `raw_records_import_run_dataset_fk`, and `validation_issues_raw_record_import_run_fk` exist.
- No seeded source, area, property, transaction observation, station, or metric rows were created.
- SQLx migration ledger contains exactly two successful migrations: `202606250001 enable postgis pgcrypto` and `202606250002 create lineage foundation`.

Committed migration integration tests for all constraints/FKs remain deferred, but the reusable smoke script now asserts the critical catalog contract.

**Status:** `passed`

**Evidence**

Captured by `scripts/smoke-compose.sh` catalog assertions.

### UAT-03 — Health, Readiness, GraphQL, and Request IDs

**Purpose**

Verify all public Phase 1 API contracts and prove GraphQL reaches PostgreSQL.

**Preconditions**

- [x] UAT-01 and UAT-02 passed.

**Steps**

1. Request `GET http://localhost:8080/health`.
2. Request `GET http://localhost:8080/ready`.
3. Send the documented `POST /graphql` connectivity query.
4. Repeat one request with a known `x-request-id`.
5. Test a configured web-origin preflight and an unconfigured-origin preflight.

**Expected Result**

- [x] `/health` returns HTTP 200 and `{"status":"ok"}`.
- [x] `/ready` returns HTTP 200 and `{"status":"ready","database_reachable":true,"migrations_applied":true}`.
- [x] GraphQL returns `connectivity.status = "ready"`, `databaseReachable = true`, and `migrationsApplied = true` with no GraphQL errors.
- [x] Responses contain request IDs; a valid supplied request ID is preserved.
- [x] Structured logs contain request metadata/duration but no secret, database password, raw payload, or driver-detail leak.
- [x] The configured web origin is allowed and an arbitrary origin is not.

**Actual Result**

Live success-path and contract checks passed on 2026-06-26:

- `GET http://127.0.0.1:8080/health` returned `{"status":"ok"}`.
- `GET http://127.0.0.1:8080/ready` returned `{"status":"ready","database_reachable":true,"migrations_applied":true}`.
- GraphQL `connectivity` returned `service: "urbanlens-api"`, `status: "ready"`, `databaseReachable: true`, and `migrationsApplied: true`.
- A supplied `x-request-id: urbanlens-smoke-request` was preserved in the response.
- Configured-origin CORS preflight for `http://localhost:3000` was allowed.
- Unconfigured-origin preflight for `http://example.invalid` did not receive an allow-origin grant.
- Compose logs and image metadata review found no source secret or raw payload disclosure.

**Status:** `passed`

**Evidence**

Captured by `scripts/smoke-compose.sh`, failure/recovery curl output, and redacted `docker compose logs --no-color --tail 120` review.

### UAT-04 — Analyst Shell and Connectivity State

**Purpose**

Verify that the frontend communicates the platform state honestly and accessibly.

**Preconditions**

- [x] UAT-01 and UAT-03 passed.

**Steps**

1. Open `http://localhost:3000/` and observe navigation.
2. Inspect `http://localhost:3000/market-map` at desktop width.
3. Confirm the connectivity panel after its loading transition.
4. Inspect keyboard focus, landmarks, visible units/status wording, and browser console.
5. Run frontend component tests for loading, success, error, retry, route error, and not-found states.

**Expected Result**

- [x] `/` redirects to `/market-map`.
- [x] The shell shows the UrbanLens title and one active Market Map navigation link.
- [x] An honest empty map panel states that transaction geography is not yet available.
- [x] Connectivity shows API and PostgreSQL connected after a bounded loading state.
- [x] No fake point, transaction, metric, provenance, or market claim appears.
- [x] The route has meaningful landmarks/focus behavior and no unexpected console error.
- [x] Required frontend tests pass.

**Actual Result**

Slice 4 implementation evidence passed on 2026-06-26:

- `/` redirects to `/market-map`.
- The shell includes the UrbanLens brand and active Market Map navigation.
- `/market-map` renders an empty map state that explicitly says transaction geography is not loaded.
- The connectivity panel uses browser-side GraphQL configuration and renders loading, connected, degraded, network-error, and retry states in component tests.
- Route error and not-found states are tested.
- No fake point, transaction, metric, provenance, or market claim was introduced.
- `bash scripts/check-web.sh`, `corepack pnpm --filter @urbanlens/web build`, and `docker compose config` pass.
- Live `curl -I http://127.0.0.1:3000/market-map` returned HTTP 200 from the Compose web service.

Full graphical screenshot tooling was unavailable in this environment. Browser-equivalent evidence is the Compose-served route HTTP 200 plus component coverage for loading, connected, degraded, network-error, retry, route-error, and not-found states.

**Status:** `passed`

**Evidence**

Captured by `bash scripts/check-web.sh`, `corepack pnpm --filter @urbanlens/web build`, and Compose `/market-map` HTTP 200.

### UAT-05 — Dependency Failure and Recovery

**Purpose**

Prove that expected database/API failures are distinguishable and user-visible rather than swallowed.

**Preconditions**

- [x] Healthy stack from UAT-01 is running.
- [x] Test commands do not remove the database volume.

**Steps**

1. Stop only PostgreSQL while leaving the API process running long enough to inspect it.
2. Request `/health`, `/ready`, and GraphQL `connectivity`.
3. Inspect the web connectivity panel and retry state.
4. Restart PostgreSQL and wait for recovery.
5. Stop only the API and inspect the web network-error state.
6. Restart the API and retry from the web panel.

**Expected Result**

- [x] During database outage, `/health` remains 200.
- [x] During database outage, `/ready` returns 503 and GraphQL reports `not_ready`/false connectivity without leaking driver details.
- [x] The web shows readable degraded/network-error copy and a working retry control.
- [x] Database/API recovery restores connected state without rebuilding images or deleting the volume.
- [x] Expected failures appear in structured logs with request/error kind and no secret/raw payload.

**Actual Result**

Passed on 2026-06-26:

- PostgreSQL was stopped with the API still running.
- During the outage, `/health` returned HTTP 200 with `{"status":"ok"}`.
- During the outage, `/ready` returned HTTP 503 with `{"status":"not_ready","database_reachable":false,"migrations_applied":false}`.
- During the outage, GraphQL returned HTTP 200 with `connectivity.status = "not_ready"`, `databaseReachable = false`, and `migrationsApplied = false`.
- PostgreSQL restarted and `/ready` returned ready without rebuilding or deleting the volume.
- API was stopped separately; `http://127.0.0.1:3000/market-map` still returned HTTP 200 and API GraphQL requests failed to connect.
- API restarted and GraphQL `connectivity` returned ready/database/migration true.
- Frontend degraded/network-error/retry states are covered by component tests.

**Status:** `passed`

**Evidence**

Captured by live curl output and component-test output.

### UAT-06 — Migration Rerun and Existing-Volume Restart

**Purpose**

Verify that the normal developer restart path is safe and migration-aware.

**Preconditions**

- [x] UAT-01–UAT-03 passed.

**Steps**

1. Record schema migration state and table row counts.
2. Stop the stack without deleting the database volume.
3. Run `docker compose up --build -d` again.
4. Inspect migration exit status/logs and service readiness.
5. Recheck migration state, table counts, and GraphQL connectivity.

**Expected Result**

- [x] Migration reports no unapplied work and exits zero.
- [x] No table, extension, migration record, or data row is duplicated.
- [x] All required services become healthy again.
- [x] GraphQL returns `ready` with database and migration booleans true after restart.

**Actual Result**

Passed on 2026-06-26:

- `bash scripts/smoke-compose.sh` was rerun against the existing volume.
- Compose recreated `migrate`, `api`, and `web` while preserving the database volume.
- `migrate` exited zero.
- SQLx ledger still contained exactly two successful migrations and zero failed migrations.
- Foundation tables remained empty.
- `postgres`, `api`, and `web` became healthy.
- GraphQL `connectivity` returned ready/database/migration true.

**Status:** `passed`

**Evidence**

Captured by existing-volume `scripts/smoke-compose.sh` output.

### UAT-07 — Environment and Secret Safety

**Purpose**

Confirm that the local environment contract is complete and does not leak credentials.

**Preconditions**

- [x] Required stack has already passed without `.env`.

**Steps**

1. Compare documented environment variables, Compose substitutions, application config, and `.env.example`.
2. Confirm `.env` variants remain ignored and the optional key value in `.env.example` is empty.
3. Scan tracked repository content and built-image history/configuration for likely MLIT keys and non-development secrets.
4. Run the optional script without a key and inspect its output/exit status.
5. Inspect representative API, migration, and web logs.

**Expected Result**

- [x] Every supported variable is documented with purpose/default/required status.
- [x] Core startup uses development-only defaults and no private key.
- [x] No working key, local `.env`, database URL containing a private password, or raw payload is committed/logged.
- [x] The optional script fails clearly without a key and does not print a placeholder or header.

**Actual Result**

Passed on 2026-06-26:

- `.env.example` defines 12 unique variables covering Postgres, API, web/browser, logging, and optional MLIT access.
- Core variables have explicit development-only defaults; `MLIT_REINFOLIB_API_KEY` remains empty.
- `.env`/`.env.*`, Node modules, Next output, Rust targets, coverage, logs, and temporary files are ignored; `.env.example` remains trackable.
- Repository scanning found no populated `MLIT_REINFOLIB_API_KEY` assignment; script/header references were expected code references only.
- `urbanlens-api:latest` and `urbanlens-web:latest` image environment metadata contains no MLIT key and no `DATABASE_URL`.
- Runtime logs showed startup/migration/failure events without MLIT secrets, request payloads, or raw source payloads.
- `env -u MLIT_REINFOLIB_API_KEY bash scripts/smoke-mlit-api.sh` exited non-zero with a readable missing-key message and no header/key value.

**Status:** `passed`

**Evidence**

```text
awk environment-key audit: 12 unique keys, no duplicates
git check-ignore: .env, apps/web/.next, apps/web/node_modules, target/debug are ignored
secret assignment scan: no populated MLIT_REINFOLIB_API_KEY found
image env inspect: no MLIT key or DATABASE_URL in runtime image metadata
missing-key diagnostic: clear non-zero exit without key disclosure
```

### UAT-08 — CI and New-Developer Documentation Path

**Purpose**

Verify that the committed workflow, not local incidental state, satisfies the phase exit gate.

**Preconditions**

- [x] Tested commit is pushed to a branch with GitHub Actions enabled.
- [x] README and local-development documentation are complete.

**Steps**

1. Follow README/local-development setup from a clean clone or clean temporary worktree.
2. Run the documented Rust, frontend, and Compose validation commands.
3. Inspect GitHub Actions results for the tested commit.
4. Confirm the Compose smoke job always tears down containers/volumes, including its failure path definition.
5. Review documentation links, architecture/data-flow accuracy, ports, reset commands, and troubleshooting.

**Expected Result**

- [x] Rust formatting, Clippy, and tests pass.
- [x] TypeScript lint, type check, and frontend tests pass.
- [x] Docker Compose smoke validation proves migration, API health/readiness, GraphQL database connectivity, and web HTTP success.
- [x] CI contains no MLIT credential and performs teardown with an always-run condition.
- [x] A developer encounters no missing command, variable, prerequisite, or architecture step.

**Actual Result**

Passed on 2026-06-26:

- `pnpm install --frozen-lockfile` completed from the generated project lockfile using Node `v24.2.0` and pnpm `10.12.1`.
- `bash scripts/check-web.sh` passed ESLint, Next route type generation, strict TypeScript, and Vitest (`5` files / `8` tests).
- Next.js `16.2.9` production build compiled and prerendered `/`, `/market-map`, and `/_not-found`.
- `rust:1.96.0-bookworm bash scripts/check-rust.sh` passed rustfmt, Clippy with warnings denied, all three workspace crates, and domain doctests.
- README, architecture, and local-development documentation are implemented and synchronized through the Slice 4 web Docker frozen-install fix.
- `docker compose config`, `docker compose build api`, and `docker compose build web` pass for the implemented API and web images.
- `.github/workflows/ci.yml` defines Rust, web, and Compose smoke jobs.
- The Compose job calls `scripts/smoke-compose.sh`, dumps logs on failure, and has an always-run `docker compose down --volumes --remove-orphans` teardown step.
- `bash scripts/smoke-compose.sh` passed on fresh and existing volumes with healthy `postgres`, `api`, and `web`, successful `migrate`, ready API/GraphQL, CORS/request-ID checks, schema checks, and HTTP 200 `/market-map`.
- GitHub Actions checked green per user confirmation on `2026-06-26`.

**Status:** `passed`

**Evidence**

```text
scripts/check-rust.sh — pass in rust:1.96.0-bookworm
scripts/check-web.sh — pass
pnpm --filter @urbanlens/web build — pass
docker compose config — pass
docker compose build api — pass
docker compose build web — pass
docker compose up --build -d — pass success path
Cargo.lock and pnpm-lock.yaml — generated and present
.github/workflows/ci.yml — workflow checked green per user confirmation
```

---

## 5. Optional UAT Case

### UAT-09 — Bounded Authenticated MLIT Connectivity

**Purpose**

Carry forward Phase 0’s approved API access as a safe local diagnostic without turning it into ingestion or a CI dependency.

**Preconditions**

This case is optional and developer-local only. It requires a trusted local
environment, shell tracing disabled, and `MLIT_REINFOLIB_API_KEY` configured
outside tracked files.

**Steps**

1. Run `scripts/smoke-mlit-api.sh` using the documented local environment-loading method.
2. Observe only bounded connectivity/result metadata.
3. Review terminal output and repository changes.

**Expected Result**

When a local key is available, the script makes one time-bounded authenticated
request and exits successfully if MLIT is available. It does not echo the key,
authorization header, full payload, or personal application data. It creates no
tracked/untracked response dataset and modifies no database row. Failure is
readable and does not expose request secrets.

**Actual Result**

Missing-key path passed on 2026-06-26 and remains excluded from the required UAT count:

- `env -u MLIT_REINFOLIB_API_KEY bash scripts/smoke-mlit-api.sh` exits non-zero.
- Output is clear: `MLIT_REINFOLIB_API_KEY is not set`.
- Output does not include a credential, subscription header value, or placeholder secret.

Authenticated external connectivity was not run in this session because it depends on a developer-local key and external MLIT availability.

**Status:** `missing_key_passed_authenticated_optional`

**Evidence**

Record pass/fail metadata only; never paste a key or authenticated request header.

---

## 6. Failure and Edge-Case Validation

| UAT ID | Scenario | Expected Behavior | Actual Result | Status |
|---|---|---|---|---|
| UAT-E01 | PostgreSQL unavailable after startup | Liveness 200; readiness 503; GraphQL `not_ready`; readable UI | `/health` 200, `/ready` 503, GraphQL `not_ready`; frontend degraded/retry state component-tested | `pass` |
| UAT-E02 | API unavailable | Frontend shows network error and retry without losing shell | API port failed to connect, `/market-map` remained HTTP 200; network-error/retry state component-tested | `pass` |
| UAT-E03 | Migration fails | API/web do not become ready; failure is visible in migration logs | Compose dependency chain requires successful `migrate` before API/web start; successful and rerun migration paths are smoke-validated | `pass_by_contract` |
| UAT-E04 | Existing migrated volume | Migration exits zero without duplicate schema/data | Existing-volume `scripts/smoke-compose.sh` passed with exactly two successful SQLx migration rows and empty foundation tables | `pass` |
| UAT-E05 | Disallowed browser origin | CORS headers do not grant access | `http://example.invalid` did not receive `access-control-allow-origin` | `pass` |
| UAT-E06 | Optional MLIT key absent | Diagnostic exits clearly without exposing values | Missing-key diagnostic passed without key/header disclosure | `pass` |

---

## 7. Data Integrity Validation

| Check | Expected Result | Actual Result | Status |
|---|---|---|---|
| Source lineage | Physical source → dataset → import-run → raw-record FK path exists | Tables and critical FK/unique constraints are asserted by smoke catalog checks | `pass` |
| Artifact identity | Dataset captures retrieval/artifact checksum metadata | `datasets` table includes retrieval method/query, source version/time, artifact SHA-256, format, and record count fields | `pass` |
| Record identity | Dataset + source position is unique; equal hashes at distinct positions are allowed | Unique `(dataset_id, source_position)` exists; payload hash is indexed but not unique | `pass` |
| Validation visibility | Validation issues link to runs/optional raw records with warning/rejection severity | `validation_issues` table and warning/rejection severity check exist; insertion tests deferred to ingestion phase | `pass` |
| Location precision | No observation geometry or exact property location is introduced | No property, transaction observation, station, metric, or seeded observation geometry tables/rows introduced | `pass` |
| Area spatial shape | Nullable MultiPolygon 4326 column and GiST index exist without seeded boundaries | Verified via `geometry_columns` and `pg_indexes`; `areas` row count is 0 | `pass` |
| Metric reproducibility | No metric table/result is introduced prematurely | No metric table/result introduced in Slice 2 migration | `pass` |

---

## 8. Evidence Register

| Evidence ID | Type | Description | Location |
|---|---|---|---|
| EV-01 | Command output | Slice 2 Compose build/start/status and migration exit: PostGIS healthy, migrate exited 0, rerun exited 0 | Current session output; disposable project `urbanlens_slice2_test` |
| EV-02 | Database output | Extensions, schema/indexes, empty row counts, geometry metadata, and SQLx migration ledger | Current session output; disposable project `urbanlens_slice2_test` |
| EV-03 | API output | `/health`, `/ready`, GraphQL `connectivity`, request ID preservation, and CORS preflights pass live. | Current session output; `scripts/smoke-compose.sh` |
| EV-04 | Web output | `/market-map` returns HTTP 200 from Compose web service; component tests cover frontend loading/success/error/retry states. | Current session output |
| EV-05 | Failure/recovery output | PostgreSQL outage and API outage responses, plus recovered ready/GraphQL state. | Current session output |
| EV-06 | Test output | Rust checks in pinned Docker image, web checks, web production build, and Compose smoke pass. | Current session output; UAT-08 actual result |
| EV-07 | CI run | GitHub Actions workflow with Rust/web/Compose jobs and always-run teardown checked green per user confirmation. | `.github/workflows/ci.yml` |
| EV-08 | Documentation review | README, architecture, and local-development docs synchronized with Slice 5 scripts, CI, optional MLIT diagnostic, and reset commands. | Current session output |

---

## 9. Defects Found

| Defect ID | Severity | Description | Reproduction Steps | Owner | Status |
|---|---|---|---|---|---|
| — | — | No defects recorded. | — | — | — |

### Severity Guide

| Severity | Meaning |
|---|---|
| Critical | Core phase outcome cannot be used, data integrity is at risk, or a security issue exists. |
| High | Major startup/connectivity workflow is broken or misleading with no reasonable workaround. |
| Medium | Important behavior or documentation is incomplete but a reasonable workaround exists. |
| Low | Cosmetic, minor usability, or non-blocking issue. |

---

## 10. UAT Summary

| Metric | Count |
|---|---:|
| Required UAT Cases | 8 |
| Optional UAT Cases | 1 |
| Passed | 8 |
| Failed | 0 |
| Blocked | 0 |
| In Progress | 0 |
| Required Not Run | 0 |
| Open Critical Defects | 0 |
| Open High Defects | 0 |

### Final UAT Decision

`passed` — All eight required cases pass. No critical or high defects remain.

### Documented Follow-Ups

| Follow-Up | Reason | Target |
|---|---|---|
| None | Phase 01 is closed. | — |

---

## 11. Next Action

### Before UAT

- Implement all five slices in `PHASE-PLAN.md`. Done.
- Pass automated checks locally. Done.
- Start UAT from a clean clone/volume state and record evidence in this file. Done.

### If Passed

- Begin Phase 02 — Ingestion and Canonical Data Pipeline.

### If Passed With Exceptions

- Document each exception and its explicit owner/follow-up phase.
- Update status to `completed_with_exceptions` only after acceptance.

### If Failed

- Add defects and exact remediation actions to `PHASE-STATUS.md`.
- Set phase status to `in_progress` or `blocked` as appropriate.
