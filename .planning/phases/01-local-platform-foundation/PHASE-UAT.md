# Phase 01 UAT — Local Platform Foundation

> Purpose: Prove that a new developer can start, inspect, fail, restart, and validate the complete local platform rather than merely confirming that its source files exist.

## Metadata

| Field | Value |
|---|---|
| Phase | `01` |
| Name | `Local Platform Foundation` |
| UAT Status | `in_progress` |
| Environment | `clean local clone with Docker Compose` |
| Tester | `Codex` |
| Started | `2026-06-24` |
| Completed | `not_completed` |
| Build / Commit | `working tree based on ffe6f2d; docs synchronization uncommitted` |
| Related Plan | `PHASE-PLAN.md` |
| Related Status | `PHASE-STATUS.md` |

---

## 1. UAT Objective

Verify that UrbanLens has a reproducible local platform: one root command starts a migrated PostGIS database, an observable Rust/Actix GraphQL API, and a Next.js analyst shell; the browser visibly proves database connectivity; expected dependency failures are honest; restart is safe; and CI exercises the same behavior.

---

## 2. Preconditions

### Required Setup

- [ ] Correct clean branch/commit is checked out.
- [ ] Docker Engine and a supported Docker Compose version are available.
- [ ] Ports `3000`, `8080`, and `5432` are free or documented overrides are configured.
- [ ] No existing UrbanLens containers or volumes can mask fresh-clone behavior.
- [ ] No `.env` is required for required UAT cases.
- [ ] Browser or equivalent screenshot capability is available for the frontend case.
- [ ] CI workflow has run for the tested commit.

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

- [ ] No `.env` exists or is sourced.
- [ ] UrbanLens Compose containers and named volumes are absent.

**Steps**

1. From the repository root, run `docker compose up --build -d`.
2. Poll until Compose reports stable service states or the documented timeout expires.
3. Run `docker compose ps` and inspect the `migrate` container exit result.
4. Open or request `http://localhost:3000/market-map`.

**Expected Result**

- [ ] `postgres`, `api`, and `web` are running and healthy.
- [ ] `migrate` ran after healthy PostgreSQL and exited with code 0.
- [ ] API/web did not become ready before migration completion.
- [ ] The market-map route returns HTTP 200.
- [ ] No secret, manual migration command, or local `.env` was required.

**Actual Result**

UAT-01 success path passed on 2026-06-26:

- `docker compose up --build -d` built `urbanlens-api`, `urbanlens-migrate`, and `urbanlens-web`.
- `postgres` became healthy before `migrate` started.
- `migrate` exited successfully before `api` started.
- `api` became healthy before `web` started.
- `web` became healthy and exposed `http://localhost:3000/market-map`.
- `curl -I http://127.0.0.1:3000/market-map` returned HTTP 200.

No secret, manual migration command, or local `.env` was required.

**Status:** `passed_success_path`

**Evidence**

Capture Compose status, migration logs without secrets, and the web HTTP result.

### UAT-02 — PostGIS and Lineage Schema Foundation

**Purpose**

Prove that migrations create the correct empty physical foundation without seed or product data.

**Preconditions**

- [ ] UAT-01 passed.
- [ ] Database inspection uses the documented local command.

**Steps**

1. Query installed extensions for `postgis` and `pgcrypto`.
2. Query catalog tables for `data_sources`, `datasets`, `import_runs`, `raw_records`, `validation_issues`, and `areas`.
3. Inspect foreign keys, check constraints, unique keys, and the partial GiST area index.
4. Confirm the area geometry type/SRID.
5. Query row counts for all six tables.
6. Execute committed migration integration tests.

**Expected Result**

- [ ] Both required extensions exist.
- [ ] All six tables exist and are empty.
- [ ] Source/dataset/import/raw foreign keys preserve the planned lineage.
- [ ] Dataset/import-run consistency and dataset/source-position uniqueness are enforced.
- [ ] Import states/counters, hashes, and severity fields have the documented constraints.
- [ ] `areas.geometry` is nullable `MultiPolygon` SRID 4326 with a non-null partial GiST index.
- [ ] No `property`, `transaction_observation`, station, metric, fixture, or area row was created.

**Actual Result**

Slice 2 database scope passed on 2026-06-25 in disposable Compose project `urbanlens_slice2_test`.

Verified:

- `postgis` and `pgcrypto` extensions exist.
- `data_sources`, `datasets`, `import_runs`, `raw_records`, `validation_issues`, and `areas` exist and are empty.
- `areas.geometry` is nullable `MULTIPOLYGON` with SRID 4326.
- `areas_geometry_gix` exists as a partial GiST index where `geometry IS NOT NULL`.
- No seeded source, area, property, transaction observation, station, or metric rows were created.
- SQLx migration ledger contains exactly two successful migrations: `202606250001 enable postgis pgcrypto` and `202606250002 create lineage foundation`.

Committed migration integration tests for all constraints/FKs remain pending.

**Status:** `in_progress — Slice 2 database scope passed`

**Evidence**

Capture catalog query output and migration test output.

### UAT-03 — Health, Readiness, GraphQL, and Request IDs

**Purpose**

Verify all public Phase 1 API contracts and prove GraphQL reaches PostgreSQL.

**Preconditions**

- [ ] UAT-01 and UAT-02 passed.

**Steps**

1. Request `GET http://localhost:8080/health`.
2. Request `GET http://localhost:8080/ready`.
3. Send the documented `POST /graphql` connectivity query.
4. Repeat one request with a known `x-request-id`.
5. Test a configured web-origin preflight and an unconfigured-origin preflight.

**Expected Result**

- [ ] `/health` returns HTTP 200 and `{"status":"ok"}`.
- [ ] `/ready` returns HTTP 200 and `{"status":"ready","database_reachable":true,"migrations_applied":true}`.
- [ ] GraphQL returns `connectivity.status = "ready"`, `databaseReachable = true`, and `migrationsApplied = true` with no GraphQL errors.
- [ ] Responses contain request IDs; a valid supplied request ID is preserved.
- [ ] Structured logs contain request metadata/duration but no secret, database password, raw payload, or driver-detail leak.
- [ ] The configured web origin is allowed and an arbitrary origin is not.

**Actual Result**

Live success-path checks passed on 2026-06-26:

- `GET http://127.0.0.1:8080/ready` returned `{"status":"ready","database_reachable":true,"migrations_applied":true}`.
- GraphQL `connectivity` returned `service: "urbanlens-api"`, `status: "ready"`, `databaseReachable: true`, and `migrationsApplied: true`.
- API route wiring, request ID middleware, and CORS config parsing remain covered by implementation/unit evidence.
- Explicit live `/health`, request-ID preservation, and CORS preflight checks still remain for full UAT.

**Status:** `in_progress — live success path passed; request-ID/CORS checks remain`

**Evidence**

Capture response status/headers/bodies and a redacted structured-log sample.

### UAT-04 — Analyst Shell and Connectivity State

**Purpose**

Verify that the frontend communicates the platform state honestly and accessibly.

**Preconditions**

- [ ] UAT-01 and UAT-03 passed.

**Steps**

1. Open `http://localhost:3000/` and observe navigation.
2. Inspect `http://localhost:3000/market-map` at desktop width.
3. Confirm the connectivity panel after its loading transition.
4. Inspect keyboard focus, landmarks, visible units/status wording, and browser console.
5. Run frontend component tests for loading, success, error, retry, route error, and not-found states.

**Expected Result**

- [ ] `/` redirects to `/market-map`.
- [ ] The shell shows the UrbanLens title and one active Market Map navigation link.
- [ ] An honest empty map panel states that transaction geography is not yet available.
- [ ] Connectivity shows API and PostgreSQL connected after a bounded loading state.
- [ ] No fake point, transaction, metric, provenance, or market claim appears.
- [ ] The route has meaningful landmarks/focus behavior and no unexpected console error.
- [ ] Required frontend tests pass.

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

Full browser screenshot/console inspection remains pending.

**Status:** `in_progress — Slice 4 implementation and web HTTP success passed; browser inspection remains`

**Evidence**

Capture a screenshot and frontend test output.

### UAT-05 — Dependency Failure and Recovery

**Purpose**

Prove that expected database/API failures are distinguishable and user-visible rather than swallowed.

**Preconditions**

- [ ] Healthy stack from UAT-01 is running.
- [ ] Test commands do not remove the database volume.

**Steps**

1. Stop only PostgreSQL while leaving the API process running long enough to inspect it.
2. Request `/health`, `/ready`, and GraphQL `connectivity`.
3. Inspect the web connectivity panel and retry state.
4. Restart PostgreSQL and wait for recovery.
5. Stop only the API and inspect the web network-error state.
6. Restart the API and retry from the web panel.

**Expected Result**

- [ ] During database outage, `/health` remains 200.
- [ ] During database outage, `/ready` returns 503 and GraphQL reports `not_ready`/false connectivity without leaking driver details.
- [ ] The web shows readable degraded/network-error copy and a working retry control.
- [ ] Database/API recovery restores connected state without rebuilding images or deleting the volume.
- [ ] Expected failures appear in structured logs with request/error kind and no secret/raw payload.

**Actual Result**

Not run.

**Status:** `not_run`

**Evidence**

Capture the three degraded API responses, frontend degraded state, and recovered state.

### UAT-06 — Migration Rerun and Existing-Volume Restart

**Purpose**

Verify that the normal developer restart path is safe and migration-aware.

**Preconditions**

- [ ] UAT-01–UAT-03 passed.

**Steps**

1. Record schema migration state and table row counts.
2. Stop the stack without deleting the database volume.
3. Run `docker compose up --build -d` again.
4. Inspect migration exit status/logs and service readiness.
5. Recheck migration state, table counts, and GraphQL connectivity.

**Expected Result**

- [ ] Migration reports no unapplied work and exits zero.
- [ ] No table, extension, migration record, or data row is duplicated.
- [ ] All required services become healthy again.
- [ ] GraphQL returns `ready` with database and migration booleans true after restart.

**Actual Result**

Slice 2 migration rerun scope passed on 2026-06-25: rerunning `docker compose -p urbanlens_slice2_test up --force-recreate --no-deps migrate` against the existing disposable volume exited with code 0, kept exactly two successful SQLx migration rows, and did not duplicate schema/data. Full UAT-06 remains incomplete because API/web health and GraphQL restart behavior are later-slice work.

**Status:** `in_progress — migration rerun scope passed`

**Evidence**

Capture before/after migration state, Compose status, and GraphQL result.

### UAT-07 — Environment and Secret Safety

**Purpose**

Confirm that the local environment contract is complete and does not leak credentials.

**Preconditions**

- [ ] Required stack has already passed without `.env`.

**Steps**

1. Compare documented environment variables, Compose substitutions, application config, and `.env.example`.
2. Confirm `.env` variants remain ignored and the optional key value in `.env.example` is empty.
3. Scan tracked repository content and built-image history/configuration for likely MLIT keys and non-development secrets.
4. Run the optional script without a key and inspect its output/exit status.
5. Inspect representative API, migration, and web logs.

**Expected Result**

- [ ] Every supported variable is documented with purpose/default/required status.
- [ ] Core startup uses development-only defaults and no private key.
- [ ] No working key, local `.env`, database URL containing a private password, or raw payload is committed/logged.
- [ ] The optional script fails clearly without a key and does not print a placeholder or header.

**Actual Result**

Partially validated for Slice 1:

- `.env.example` defines 12 unique variables covering Postgres, API, web/browser, logging, and optional MLIT access.
- Core variables have explicit development-only defaults; `MLIT_REINFOLIB_API_KEY` remains empty.
- `.env`/`.env.*`, Node modules, Next output, Rust targets, coverage, logs, and temporary files are ignored; `.env.example` remains trackable.
- Repository scanning found no populated `MLIT_REINFOLIB_API_KEY` assignment.
- Compose/application configuration and the API image now exist; API CORS and request-ID config are unit-tested. Runtime logs, web configuration, built web image, and the optional MLIT script are still not fully validated, so the full case cannot pass.

**Status:** `in_progress`

**Evidence**

```text
awk environment-key audit: 12 unique keys, no duplicates
git check-ignore: .env, apps/web/.next, apps/web/node_modules, target/debug are ignored
secret assignment scan: no populated MLIT_REINFOLIB_API_KEY found
```

### UAT-08 — CI and New-Developer Documentation Path

**Purpose**

Verify that the committed workflow, not local incidental state, satisfies the phase exit gate.

**Preconditions**

- [ ] Tested commit is pushed to a branch with GitHub Actions enabled.
- [ ] README and local-development documentation are complete.

**Steps**

1. Follow README/local-development setup from a clean clone or clean temporary worktree.
2. Run the documented Rust, frontend, and Compose validation commands.
3. Inspect GitHub Actions results for the tested commit.
4. Confirm the Compose smoke job always tears down containers/volumes, including its failure path definition.
5. Review documentation links, architecture/data-flow accuracy, ports, reset commands, and troubleshooting.

**Expected Result**

- [ ] Rust formatting, Clippy, and tests pass.
- [ ] TypeScript lint, type check, and frontend tests pass.
- [ ] Docker Compose smoke validation proves migration, API health/readiness, GraphQL database connectivity, and web HTTP success.
- [ ] CI contains no MLIT credential and performs teardown with an always-run condition.
- [ ] A developer encounters no missing command, variable, prerequisite, or architecture step.

**Actual Result**

Partially validated through Slice 4:

- `pnpm install --frozen-lockfile` completed from the generated project lockfile using Node `v24.2.0` and pnpm `10.12.1`.
- `bash scripts/check-web.sh` passed ESLint, Next route type generation, strict TypeScript, and Vitest (`5` files / `8` tests).
- Next.js `16.2.9` production build compiled and prerendered `/`, `/market-map`, and `/_not-found`.
- `rust:1.96.0-bookworm bash scripts/check-rust.sh` passed rustfmt, Clippy with warnings denied, all three workspace crates, and domain doctests.
- README, architecture, and local-development documentation are implemented and synchronized through the Slice 4 web Docker frozen-install fix.
- `docker compose config`, `docker compose build api`, and `docker compose build web` pass for the implemented API and web images.
- `docker compose up --build -d` success-path smoke passed locally with healthy `postgres`, `api`, and `web`, successful `migrate`, ready API/GraphQL, and HTTP 200 `/market-map`.
- GitHub Actions and full failure-mode Compose smoke validation remain unimplemented, so the full case cannot pass.

**Status:** `in_progress`

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
```

---

## 5. Optional UAT Case

### UAT-09 — Bounded Authenticated MLIT Connectivity

**Purpose**

Carry forward Phase 0’s approved API access as a safe local diagnostic without turning it into ingestion or a CI dependency.

**Preconditions**

- [ ] The user trusts the local environment and has configured `MLIT_REINFOLIB_API_KEY` outside tracked files.
- [ ] Shell tracing is disabled.

**Steps**

1. Run `scripts/smoke-mlit-api.sh` using the documented local environment-loading method.
2. Observe only bounded connectivity/result metadata.
3. Review terminal output and repository changes.

**Expected Result**

- [ ] The script makes one time-bounded authenticated request and exits successfully when MLIT is available.
- [ ] It does not echo the key, authorization header, full payload, or personal application data.
- [ ] It creates no tracked/untracked response dataset and modifies no database row.
- [ ] Failure is readable and does not expose request secrets.

**Actual Result**

Not run; optional and excluded from the required UAT count.

**Status:** `not_run_optional`

**Evidence**

Record pass/fail metadata only; never paste a key or authenticated request header.

---

## 6. Failure and Edge-Case Validation

| UAT ID | Scenario | Expected Behavior | Actual Result | Status |
|---|---|---|---|---|
| UAT-E01 | PostgreSQL unavailable after startup | Liveness 200; readiness 503; GraphQL `not_ready`; readable UI | Not run | `not_run` |
| UAT-E02 | API unavailable | Frontend shows network error and retry without losing shell | Not run | `not_run` |
| UAT-E03 | Migration fails | API/web do not become ready; failure is visible in migration logs | Not run | `not_run` |
| UAT-E04 | Existing migrated volume | Migration exits zero without duplicate schema/data | Slice 2 migration rerun passed; full API/web restart not run | `in_progress` |
| UAT-E05 | Disallowed browser origin | CORS headers do not grant access | Not run | `not_run` |
| UAT-E06 | Optional MLIT key absent | Diagnostic exits clearly without exposing values | Not run | `not_run` |

---

## 7. Data Integrity Validation

| Check | Expected Result | Actual Result | Status |
|---|---|---|---|
| Source lineage | Physical source → dataset → import-run → raw-record FK path exists | Tables and FK path created by migration; full constraint insertion tests pending | `in_progress` |
| Artifact identity | Dataset captures retrieval/artifact checksum metadata | `datasets` table includes retrieval method/query, source version/time, artifact SHA-256, format, and record count fields | `pass` |
| Record identity | Dataset + source position is unique; equal hashes at distinct positions are allowed | Unique `(dataset_id, source_position)` exists; payload hash is indexed but not unique | `pass` |
| Validation visibility | Validation issues link to runs/optional raw records with warning/rejection severity | `validation_issues` table and warning/rejection severity check exist; insertion tests pending | `in_progress` |
| Location precision | No observation geometry or exact property location is introduced | No property, transaction observation, station, metric, or seeded observation geometry tables/rows introduced | `pass` |
| Area spatial shape | Nullable MultiPolygon 4326 column and GiST index exist without seeded boundaries | Verified via `geometry_columns` and `pg_indexes`; `areas` row count is 0 | `pass` |
| Metric reproducibility | No metric table/result is introduced prematurely | No metric table/result introduced in Slice 2 migration | `pass` |

---

## 8. Evidence Register

| Evidence ID | Type | Description | Location |
|---|---|---|---|
| EV-01 | Command output | Slice 2 Compose build/start/status and migration exit: PostGIS healthy, migrate exited 0, rerun exited 0 | Current session output; disposable project `urbanlens_slice2_test` |
| EV-02 | Database output | Extensions, schema/indexes, empty row counts, geometry metadata, and SQLx migration ledger | Current session output; disposable project `urbanlens_slice2_test` |
| EV-03 | API output | `/ready` returns ready and GraphQL `connectivity` returns ready/database/migration true; config/request-ID unit tests pass; CORS live preflight remains TBD. | Current session output |
| EV-04 | Web output | `/market-map` returns HTTP 200 from Compose web service; browser screenshot remains TBD. | Current session output |
| EV-05 | Screenshot/output | Degraded and recovered frontend/API states | TBD |
| EV-06 | Test output | Slice 1 Rust/frontend checks pass; Slice 3 `cargo fmt --all --check`, Clippy with warnings denied, and workspace tests pass; database/live integration pending | Current session output; UAT-08 actual result |
| EV-07 | CI run | Passing workflow and always-run teardown | TBD |
| EV-08 | Documentation review | README, architecture, and local-development docs synchronized with web shell, browser GraphQL URL, Compose web service, and `.npmrc` Docker install contract | Current session output |

---

## 9. Defects Found

| Defect ID | Severity | Description | Reproduction Steps | Owner | Status |
|---|---|---|---|---|---|
| — | — | No defects recorded; UAT is in progress with Slice 1, Slice 2, and Slice 3 partial evidence. | — | — | — |

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
| Passed | 0 |
| Failed | 0 |
| Blocked | 0 |
| In Progress | 5 |
| Required Not Run | 3 |
| Open Critical Defects | 0 |
| Open High Defects | 0 |

### Final UAT Decision

- [ ] `passed` — All eight required cases pass. No critical or high defects remain.
- [ ] `passed_with_accepted_exceptions` — Remaining issues are documented and accepted.
- [ ] `failed` — Required behavior is incomplete or blocking defects remain.
- [ ] `blocked` — UAT cannot continue because prerequisites are unavailable.

### Accepted Exceptions

| Exception | Reason | Follow-Up Phase / Issue |
|---|---|---|
| None | Pre-UAT validation is in progress; no exception has been requested or accepted. | — |

---

## 11. Next Action

### Before UAT

- Implement all five slices in `PHASE-PLAN.md`.
- Pass automated checks locally.
- Start UAT from a clean clone/volume state and record evidence in this file.

### If Passed

- Complete the Phase 01 handoff notes.
- Update `PHASE-STATUS.md` to `completed`.
- Update `.planning/STATE.md` to Phase 02 — Ingestion and Canonical Data Pipeline.

### If Passed With Exceptions

- Document each exception and its explicit owner/follow-up phase.
- Update status to `completed_with_exceptions` only after acceptance.

### If Failed

- Add defects and exact remediation actions to `PHASE-STATUS.md`.
- Set phase status to `in_progress` or `blocked` as appropriate.
