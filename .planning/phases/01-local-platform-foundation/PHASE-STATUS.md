# Phase 01 Status — Local Platform Foundation

> Purpose: Record the current phase state, progress, blockers, and exact next action.

## Current State

| Field | Value |
|---|---|
| Phase | `01` |
| Name | `Local Platform Foundation` |
| Overall Status | `completed` |
| Health | `green` |
| Owner | `Project owner` |
| Started | `2026-06-24` |
| Last Updated | `2026-06-26 15:50 +07:00` |
| Target Completion | `2026-06-26` |
| Current Branch | `main` |
| Current Commit | `ffe6f2d` |
| Related Plan | `PHASE-PLAN.md` |
| Related UAT | `PHASE-UAT.md` |

## 1. Current Objective

Build a clean-clone local platform in which one root Docker Compose command starts the Next.js web application, Rust/Actix GraphQL API, and PostgreSQL/PostGIS database after automatically applying the foundation migrations.

## 2. Current Focus

Phase 01 is complete. Slices 1 through 5 are implemented, locally validated with Docker, and GitHub Actions checked green per user confirmation on `2026-06-26`.

## 3. Definition of Done

The phase is done when a new developer can run `docker compose up --build`, open `/market-map`, see successful GraphQL/PostgreSQL connectivity, rerun the stack safely, and obtain passing Rust, frontend, migration, and Compose checks in CI.

---

## 4. Progress Snapshot

| Area | Status | Progress | Notes |
|---|---|---:|---|
| Planning | Done | 100% | Plan, decisions, interfaces, schema foundation, and UAT protocol are complete. |
| Design / Architecture | Complete | 100% | Workspace/package boundaries, environment contract, migration lifecycle, API runtime boundary, frontend shell boundary, and CI/smoke boundary are implemented. |
| Backend | Complete | 100% | Actix server, SQLx pool, `/health`, `/ready`, GraphQL `connectivity`, bounded CORS, request IDs, and API healthcheck are implemented and smoke-validated. |
| Database | Complete | 100% | PostGIS service, extensions, six foundation tables, constraints, indexes, SQLx ledger, and rerun lifecycle are implemented and smoke-validated. |
| Worker / Ingestion | Complete for Phase 01 | 100% | Compile-only importer crate exists; ingestion remains intentionally absent until Phase 02. |
| Frontend | Complete | 100% | App shell, root redirect, `/market-map`, empty map state, GraphQL connectivity panel, loading/error/retry states, and component tests pass. |
| Infrastructure | Complete | 100% | Root/infra Compose, healthchecks, Dockerfiles, reusable smoke script, and GitHub Actions workflow are in place and green. |
| Tests | Complete | 100% | Rust/web/build/Compose smoke/failure/restart/secret checks pass locally and in GitHub Actions. |
| Documentation | Complete | 100% | README, architecture, local-development docs, environment contract, and planning records are current for Slice 5. |
| UAT | Complete | 100% | Required UAT cases passed and GitHub Actions checked green. |

---

## 5. Completed Work

| Date | Completed Outcome | Evidence / Link |
|---|---|---|
| 2026-06-24 | Made the Phase 01 implementation plan decision-complete, including public interfaces, schema constraints, startup order, CI, and failure behavior. | `PHASE-PLAN.md` |
| 2026-06-24 | Defined traceable UAT for clean-clone startup, migrations, API/GraphQL, frontend states, dependency failure, restart, and CI. | `PHASE-UAT.md` |
| 2026-06-24 | Preserved the accepted dataset-artifact lineage by including `datasets` in the physical schema plan. | `PHASE-PLAN.md`, `docs/data-model.md` |
| 2026-06-24 | Completed Slice 1 with Cargo/pnpm workspaces, pinned toolchains, lockfiles, package scaffolds, environment/ignore contracts, shared checks, and a tested Next.js foundation. | `Cargo.toml`, `package.json`, `.env.example`, `apps/`, `crates/`, `workers/importer/`, `scripts/` |
| 2026-06-25 | Completed Slice 2 with root Compose include, `infra/docker-compose.yml`, PostGIS 17-3.5 service, named volume, health-gated SQLx migration service, embedded migration binary, extension migration, and six-table lineage foundation migration. | `compose.yaml`, `infra/`, `apps/api/migrations/`, `apps/api/src/bin/migrate.rs` |
| 2026-06-25 | Completed Slice 3 with API config, SQLx pool, Actix server, `/health`, `/ready`, GraphQL `connectivity`, bounded CORS, request IDs, API Compose service after migration success, image healthcheck, and API local-development docs. | `apps/api/src/lib.rs`, `apps/api/src/main.rs`, `apps/api/src/bin/healthcheck.rs`, `infra/docker-compose.yml`, `infra/api.Dockerfile`, `docs/local-development.md` |
| 2026-06-26 | Completed Slice 4 implementation with Next.js app shell, `/market-map`, browser GraphQL connectivity proof, loading/error/retry/not-found states, web Compose service, web Dockerfile, tests, README, architecture, and local-development docs. | `apps/web/src/app/`, `apps/web/src/components/`, `apps/web/src/lib/`, `infra/web.Dockerfile`, `infra/docker-compose.yml`, `README.md`, `docs/` |
| 2026-06-26 | Fixed the web Docker frozen-install mismatch by copying `.npmrc` before `pnpm install --frozen-lockfile`; verified `docker compose build web`. | `infra/web.Dockerfile`, `README.md`, `docs/local-development.md`, `docs/architecture.md` |
| 2026-06-26 | Verified local Compose success path: services healthy, migration complete, `/ready` ready, GraphQL `connectivity` ready, and `/market-map` HTTP 200. | `docker compose up --build -d`, `docker compose ps`, `curl` checks |
| 2026-06-26 | Completed Slice 5 implementation with GitHub Actions, reusable Compose smoke validation, optional MLIT XIT001 diagnostic, docs, fresh/existing-volume smoke, failure/recovery checks, and local CI-equivalent validation. | `.github/workflows/ci.yml`, `scripts/smoke-compose.sh`, `scripts/smoke-mlit-api.sh`, `README.md`, `docs/`, `PHASE-UAT.md` |
| 2026-06-26 | User confirmed GitHub Actions workflow checked green; Phase 01 closed. | `.github/workflows/ci.yml`, `PHASE-UAT.md` |

---

## 6. Completed Workstreams

| Item | Current State | Next Step |
|---|---|---|
| Slice 1 workspace foundation | Complete | Preserve the shared check scripts as the CI command boundary. |
| Compose/database foundation | Complete | Preserve the root/infra Compose and migration lifecycle while later services are added. |
| API foundation | Complete | Preserve `/health`, `/ready`, GraphQL `connectivity`, bounded CORS, request IDs, and API container healthcheck while adding web behavior. |
| Frontend foundation | Complete | Preserve the browser-visible analyst shell and connectivity proof while adding product data in later phases only. |
| Slice 5 CI/smoke/UAT | Complete | Preserve the workflow and smoke boundary while starting Phase 02. |

---

## 7. Exact Next Actions

1. [x] Add root Cargo/pnpm workspace manifests, `rust-toolchain.toml`, package/crate skeletons, lockfiles, shared checks, and the environment contract.
2. [x] Add the root Compose entrypoint and `infra/docker-compose.yml` with the PostGIS service, volume, safe defaults, and health check.
3. [x] Add extension/schema migrations and the dedicated embedded migration binary before implementing API routes.
4. [x] Verify the fresh-volume and migration-rerun paths before proceeding to API behavior.
5. [x] Implement Slice 3 API configuration, SQLx pool wiring, Actix health/readiness routes, GraphQL database connectivity, bounded CORS, request IDs, and API container healthcheck.
6. [x] Implement Slice 4 Next.js analyst shell and browser-visible GraphQL connectivity state.
7. [x] Implement Slice 5 CI/smoke validation and run full Docker Compose/browser UAT locally.
8. [x] GitHub Actions checked green.
9. **Next immediate action:** Create/activate Phase 02 planning documents and begin importer design.

---

## 8. Blockers, Risks, and Dependencies

### Blockers

| ID | Blocker | Impact | Owner | Since | Next Action |
|---|---|---|---|---|---|
| — | No active blocker. | — | — | — | — |

### Risks

| ID | Risk | Likelihood | Impact | Mitigation | Status |
|---|---|---:|---:|---|---|
| RSK-01 | Browser and container API URLs/origins diverge. | Medium | High | `NEXT_PUBLIC_GRAPHQL_URL` is documented as browser-facing, component-tested, and exercised through Compose web/API smoke. | Mitigated |
| RSK-02 | Compose startup races migrations or readiness. | Medium | High | Postgres health gates migration; API waits for migration success; web waits for API health; reusable smoke validates the chain. | Mitigated |
| RSK-03 | Foundational schema weakens exact-artifact lineage. | Low | High | Enforced dataset-position identity and consistent dataset/import-run references in migration; smoke asserts lineage constraints and schema state. | Mitigated |
| RSK-04 | Host Cargo is unavailable. | High | Low | Pinned Rust 1.96 container runs the authoritative shared check successfully. | Mitigated |
| RSK-05 | Optional MLIT diagnostic exposes the key. | Low | High | Diagnostic avoids shell tracing, prints only bounded metadata, is excluded from CI, and missing-key output was tested. | Mitigated |
| RSK-06 | Remote CI may differ from local checks. | Low | Medium | Workflow uses the same committed scripts and always-run teardown; GitHub Actions checked green. | Mitigated |

### Dependencies

| Dependency | Status | Required By | Next Action |
|---|---|---|---|
| Docker Compose | Available locally (`v5.0.2`) | Slice 2 and UAT | Validate minimum supported version in documentation/CI. |
| Node.js 24 / pnpm 10 | Validated (`v24.2.0` / `10.12.1`) | Slice 1/4 | Keep declarations and lockfile aligned. |
| Rust 1.96 | Validated in pinned container; host unavailable | Slice 1/3 | Reuse `scripts/check-rust.sh` in CI. |
| PostGIS 17-3.5 image | Validated locally | Slice 2 | Pinned image line in Compose; service uses `platform: linux/amd64` because Docker reported no native arm64 manifest for the selected tag. |
| MLIT API key | Available locally; optional | Optional UAT-09 | Never add it to CI or required startup. |

---

## 9. Decisions and Plan Changes

### Decisions Made During Planning

| Date | Decision | Rationale | ADR / Reference |
|---|---|---|---|
| 2026-06-24 | Include `datasets` as a sixth foundation table. | Preserves source → exact artifact → import run lineage from Phase 0. | ADR-003, `docs/data-model.md` |
| 2026-06-24 | Run migrations in a one-shot Compose service. | Makes ordering/failure visible and keeps API startup separate. | `PHASE-PLAN.md` |
| 2026-06-24 | Make the MLIT smoke check optional. | Platform startup and CI must not depend on a private key or external availability. | `PHASE-PLAN.md` |
| 2026-06-24 | Use the accepted stack without a new ADR. | The phase implements ADRs 001–005 rather than changing architecture. | ADRs 001–005 |
| 2026-06-25 | Pin PostGIS Compose platform to `linux/amd64`. | Preserves the selected `postgis/postgis:17-3.5` image after Docker reported no arm64 manifest on this machine. | `infra/docker-compose.yml` |
| 2026-06-25 | Use `/ready` and GraphQL `connectivity` for Slice 3. | Matches the implemented API and reports service/database/migration state before product data exists. | `apps/api/src/lib.rs`, `docs/local-development.md` |

### Changes From Original Phase Outline

| Date | Original Plan | Change | Reason | Impact |
|---|---|---|---|---|
| 2026-06-24 | Five named foundation tables | Added `datasets` | Required by the completed Phase 0 conceptual lineage model | One additional empty migration table and FK layer |
| 2026-06-24 | Migrations automatic or one documented command | Selected one-shot migration service | Guarantees the root one-command workflow while exposing migration failure | Adds an exited-success Compose service |
| 2026-06-24 | No source connectivity requirement stated | Added optional bounded MLIT smoke script | Carries forward Phase 0 handoff without expanding ingestion scope | Non-blocking manual diagnostic only |

---

## 10. Validation Status

### Automated Validation

| Check | Latest Result | Evidence |
|---|---|---|
| Rust formatting | Pass | `rust:1.96.0-bookworm bash scripts/check-rust.sh` |
| Rust lint | Pass | Clippy workspace/all-target/all-feature check with warnings denied |
| Rust tests | Pass | Three workspace crates and domain doctests pass |
| Compose config | Pass | `docker compose config` renders the root include, PostGIS service, named volume, and migration service. |
| API unit tests | Pass | Config parsing rejects wildcard CORS and request-ID middleware generates/preserves `x-request-id`. |
| API image build | Pass | `docker compose build api` builds `urbanlens-api:latest` with `urbanlens-api`, `urbanlens-migrate`, and `urbanlens-healthcheck` binaries. |
| Database/migration smoke | Pass | Disposable `urbanlens_slice2_test` stack: Postgres healthy, `migrate` exited 0, SQLx ledger contains exactly two successful migrations after rerun. |
| Schema inspection | Pass | `postgis`/`pgcrypto`, six empty foundation tables, `areas.geometry` MultiPolygon SRID 4326, and partial GiST index verified. |
| TypeScript lint | Pass | `pnpm check:web` using ESLint 9 / Next.js rules |
| TypeScript type check | Pass | `next typegen && tsc --noEmit` |
| Frontend tests | Pass | Vitest 4.1.9: 5 files, 8 tests passed |
| Frontend production build | Pass | Next.js 16.2.9 compiled and prerendered `/`, `/market-map`, and `/_not-found` |
| Web Compose image build | Pass | `docker compose build web` builds `urbanlens-web:latest`; `.npmrc` is copied before `pnpm install --frozen-lockfile` so lockfile settings match inside Docker. |
| Database/migration tests | Pass for Phase 01 | Reusable smoke asserts extensions, SQLx ledger, six empty foundation tables, lineage constraints, and area spatial index; deeper insertion tests are deferred to ingestion behavior. |
| Integration tests | Pass for Phase 01 | Fresh/existing-volume Compose smoke, API readiness/GraphQL, CORS, request IDs, web HTTP success, PostgreSQL outage, API outage, and recovery pass locally. |
| Docker Compose smoke test | Pass | `postgres`, `api`, and `web` healthy; `migrate` exited; `/ready`, GraphQL `connectivity`, `/market-map`, CORS, request IDs, and schema assertions succeed. |
| Reusable Compose smoke | Pass | `bash scripts/smoke-compose.sh` passes on fresh and existing volumes; asserts service health, migration exit, request IDs, CORS, API/web endpoints, SQLx ledger, extensions, empty foundation tables, lineage constraints, and area spatial index. |
| Failure/recovery UAT | Pass | PostgreSQL outage: `/health` 200, `/ready` 503, GraphQL `not_ready`; recovery returns ready. API outage: web route 200 and API connection fails; API restart returns GraphQL ready. |
| Optional MLIT diagnostic missing key | Pass | `env -u MLIT_REINFOLIB_API_KEY bash scripts/smoke-mlit-api.sh` exits non-zero with a clear message and no key/header value. |
| GitHub Actions workflow | Pass | `.github/workflows/ci.yml` contains Rust, web, and Compose smoke jobs plus failure logs and always-run teardown; user confirmed workflow checked green. |

### UAT Status

| Field | Value |
|---|---|
| UAT Readiness | `complete` |
| UAT Result | `passed` |
| Blocking Defects | `0` |
| Required Cases | `8` |
| Optional Cases | `1` |
| UAT File | `PHASE-UAT.md` |

---

## 11. Resume Context

### Last Meaningful Change

Slice 5 was implemented and validated locally: GitHub Actions workflow, reusable Compose smoke, optional MLIT diagnostic, fresh/existing-volume Docker UAT, PostgreSQL/API failure and recovery checks, Rust/web/build checks, and documentation/planning updates.

### Current Working Assumption

Core startup must require no secret or local `.env`; safe Compose defaults are authoritative for development, while `.env.example` documents overrides and the optional MLIT key.

### Important Files

```text
.planning/phases/01-local-platform-foundation/PHASE-PLAN.md — decision-complete implementation design
.planning/phases/01-local-platform-foundation/PHASE-UAT.md — executable phase acceptance protocol
docs/data-model.md — lineage/idempotency boundaries that migrations must preserve
```

### Recommended Resume Command

```bash
sed -n '1,260p' .planning/phases/01-local-platform-foundation/PHASE-PLAN.md
```

### Exact Next Technical Step

Create/activate Phase 02 planning documents and begin ingestion and canonical data pipeline design.

---

## 12. Exit Checklist

- [x] All in-scope phase requirements are complete.
- [x] Required automated tests and CI jobs pass.
- [x] Required documentation is complete.
- [x] All eight required UAT cases have been executed locally.
- [x] UAT result is `passed`.
- [x] No critical or high defects remain open.
- [x] Handoff notes are completed in `PHASE-PLAN.md`.
- [x] `.planning/STATE.md` is updated with Phase 02 as the next resume action.
- [x] Overall status is `completed`.

---

## 13. Update Log

| Timestamp | Status | Update |
|---|---|---|
| 2026-06-24 17:55 +07:00 | `ready_for_implementation` | Decision-complete plan, status, and UAT documents created; implementation has not started. |
| 2026-06-24 18:27 +07:00 | `in_progress` | Slice 1 completed; pinned Rust checks, frozen pnpm install, web lint/typecheck/test, and Next production build pass. |
| 2026-06-25 14:05 +07:00 | `in_progress` | Slice 2 completed; Compose/PostGIS/migration lifecycle and schema smoke validation pass; focus moves to Slice 3 API behavior. |
| 2026-06-25 15:25 +07:00 | `in_progress` | Slice 3 completed; Actix/GraphQL API, SQLx pool, `/health`, `/ready`, GraphQL `connectivity`, request IDs, bounded CORS, API healthcheck, Compose API dependency gating, docs, tests, Clippy, Compose config, and API image build pass; focus moves to Slice 4 frontend connectivity. |
| 2026-06-26 14:20 +07:00 | `in_progress` | Slice 4 completed at implementation level; Next.js analyst shell, `/market-map`, browser GraphQL connectivity panel, loading/error/retry/not-found states, web Dockerfile, Compose web service, README, architecture, local-development docs, lint, typecheck, tests, and web production build pass. Initial Docker validation was deferred and later superseded by the 15:00 and 15:15 evidence below. |
| 2026-06-26 15:00 +07:00 | `in_progress` | Documentation synchronized after web Docker fix; `docker compose build web` now passes with `.npmrc` copied into the build context before frozen pnpm install. |
| 2026-06-26 15:15 +07:00 | `in_progress` | Local Compose success-path smoke passed: all runtime services healthy, migration completed, API readiness and GraphQL connectivity report ready, and `/market-map` returns HTTP 200. |
| 2026-06-26 15:50 +07:00 | `completed` | Slice 5 local UAT completed; user confirmed GitHub Actions workflow checked green; Phase 01 closed. |
