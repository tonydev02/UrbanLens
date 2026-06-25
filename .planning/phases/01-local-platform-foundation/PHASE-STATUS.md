# Phase 01 Status — Local Platform Foundation

> Purpose: Record the current phase state, progress, blockers, and exact next action.

## Current State

| Field | Value |
|---|---|
| Phase | `01` |
| Name | `Local Platform Foundation` |
| Overall Status | `in_progress` |
| Health | `green` |
| Owner | `Project owner` |
| Started | `2026-06-24` |
| Last Updated | `2026-06-25 14:05 +07:00` |
| Target Completion | `TBD` |
| Current Branch | `main` |
| Current Commit | `169f517` |
| Related Plan | `PHASE-PLAN.md` |
| Related UAT | `PHASE-UAT.md` |

## 1. Current Objective

Build a clean-clone local platform in which one root Docker Compose command starts the Next.js web application, Rust/Actix GraphQL API, and PostgreSQL/PostGIS database after automatically applying the foundation migrations.

## 2. Current Focus

Slices 1 and 2 are complete at implementation level. Current focus moves to Slice 3: observable Actix/GraphQL API behavior, SQLx pool wiring, health/readiness contracts, and database connectivity.

## 3. Definition of Done

The phase is done when a new developer can run `docker compose up --build`, open `/market-map`, see successful GraphQL/PostgreSQL connectivity, rerun the stack safely, and obtain passing Rust, frontend, migration, and Compose checks in CI.

---

## 4. Progress Snapshot

| Area | Status | Progress | Notes |
|---|---|---:|---|
| Planning | Done | 100% | Plan, decisions, interfaces, schema foundation, and UAT protocol are complete. |
| Design / Architecture | In Progress | 35% | Workspace/package boundaries, environment contract, and migration lifecycle are implemented. |
| Backend | In Progress | 20% | API crate compiles and owns the migration binary; Actix/GraphQL behavior remains Slice 3. |
| Database | In Progress | 70% | PostGIS service, extensions, six foundation tables, constraints, indexes, and SQLx rerun lifecycle are implemented and smoke-validated. |
| Worker / Ingestion | In Progress | 25% | Compile-only importer crate exists; ingestion remains intentionally absent. |
| Frontend | In Progress | 20% | Next.js 16 scaffold, strict TypeScript, CSS, and one foundation test pass. |
| Infrastructure | In Progress | 35% | Root/infra Compose database lifecycle exists; API/web services and CI smoke remain. |
| Tests | In Progress | 35% | Shared Rust/web checks pass; Slice 2 Compose/database smoke evidence exists; committed migration integration tests, API, and CI coverage remain. |
| Documentation | In Progress | 20% | Environment contract and planning records are current; runtime docs remain. |
| UAT | In Progress | 25% | Slice 1 evidence and Slice 2 database/migration evidence are recorded; full API/web UAT remains. |

---

## 5. Completed Work

| Date | Completed Outcome | Evidence / Link |
|---|---|---|
| 2026-06-24 | Made the Phase 01 implementation plan decision-complete, including public interfaces, schema constraints, startup order, CI, and failure behavior. | `PHASE-PLAN.md` |
| 2026-06-24 | Defined traceable UAT for clean-clone startup, migrations, API/GraphQL, frontend states, dependency failure, restart, and CI. | `PHASE-UAT.md` |
| 2026-06-24 | Preserved the accepted dataset-artifact lineage by including `datasets` in the physical schema plan. | `PHASE-PLAN.md`, `docs/data-model.md` |
| 2026-06-24 | Completed Slice 1 with Cargo/pnpm workspaces, pinned toolchains, lockfiles, package scaffolds, environment/ignore contracts, shared checks, and a tested Next.js foundation. | `Cargo.toml`, `package.json`, `.env.example`, `apps/`, `crates/`, `workers/importer/`, `scripts/` |
| 2026-06-25 | Completed Slice 2 with root Compose include, `infra/docker-compose.yml`, PostGIS 17-3.5 service, named volume, health-gated SQLx migration service, embedded migration binary, extension migration, and six-table lineage foundation migration. | `compose.yaml`, `infra/`, `apps/api/migrations/`, `apps/api/src/bin/migrate.rs` |

---

## 6. Work In Progress

| Item | Current State | Next Step |
|---|---|---|
| Slice 1 workspace foundation | Complete | Preserve the shared check scripts as the CI command boundary. |
| Compose/database foundation | Complete | Preserve the root/infra Compose and migration lifecycle while later services are added. |
| API/frontend foundation | Planned | Begin Slice 3 API behavior now that workspace and database lifecycle compile and start. |

---

## 7. Exact Next Actions

1. [x] Add root Cargo/pnpm workspace manifests, `rust-toolchain.toml`, package/crate skeletons, lockfiles, shared checks, and the environment contract.
2. [x] Add the root Compose entrypoint and `infra/docker-compose.yml` with the PostGIS service, volume, safe defaults, and health check.
3. [x] Add extension/schema migrations and the dedicated embedded migration binary before implementing API routes.
4. [x] Verify the fresh-volume and migration-rerun paths before proceeding to API behavior.
5. [ ] **Next immediate action:** Implement Slice 3 API configuration, SQLx pool wiring, Actix health/readiness routes, and GraphQL database connectivity.

---

## 8. Blockers, Risks, and Dependencies

### Blockers

| ID | Blocker | Impact | Owner | Since | Next Action |
|---|---|---|---|---|---|
| — | No active blocker. | — | — | — | — |

### Risks

| ID | Risk | Likelihood | Impact | Mitigation | Status |
|---|---|---:|---:|---|---|
| RSK-01 | Browser and container API URLs/origins diverge. | Medium | High | Define separate explicit URLs/origin variables and test browser connectivity. | Open |
| RSK-02 | Compose startup races migrations or readiness. | Medium | High | Postgres health gates the one-shot migration service; API/web health gates still need Slice 3/4 implementation. | Partially mitigated |
| RSK-03 | Foundational schema weakens exact-artifact lineage. | Low | High | Enforced dataset-position identity and consistent dataset/import-run references in migration; committed constraint tests still remain. | Partially mitigated |
| RSK-04 | Host Cargo is unavailable. | High | Low | Pinned Rust 1.96 container runs the authoritative shared check successfully. | Mitigated |
| RSK-05 | Optional MLIT diagnostic exposes the key. | Low | High | No shell tracing/header output, no CI credential, and output disclosure test. | Open |

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
| Database/migration smoke | Pass | Disposable `urbanlens_slice2_test` stack: Postgres healthy, `migrate` exited 0, SQLx ledger contains exactly two successful migrations after rerun. |
| Schema inspection | Pass | `postgis`/`pgcrypto`, six empty foundation tables, `areas.geometry` MultiPolygon SRID 4326, and partial GiST index verified. |
| TypeScript lint | Pass | `pnpm check:web` using ESLint 9 / Next.js rules |
| TypeScript type check | Pass | `next typegen && tsc --noEmit` |
| Frontend tests | Pass | Vitest 4.1.9: 1 file, 1 test passed |
| Frontend production build | Pass | Next.js 16.2.9 compiled and prerendered `/` and `/_not-found` |
| Database/migration tests | Partial | Manual catalog and rerun smoke passed; committed migration integration tests are not yet added. |
| Integration tests | Partial | PostGIS + migration lifecycle passed; API/web integration remains unimplemented. |
| Docker Compose smoke test | Partial | Database/migration services validated in disposable Compose project; full API/web smoke remains for later slices. |

### UAT Status

| Field | Value |
|---|---|
| UAT Readiness | `not_ready` |
| UAT Result | `in_progress — Slice 1 evidence plus Slice 2 database/migration evidence` |
| Blocking Defects | `0` |
| Required Cases | `8` |
| Optional Cases | `1` |
| UAT File | `PHASE-UAT.md` |

---

## 11. Resume Context

### Last Meaningful Change

Slice 2 was implemented and validated at database scope: root/infra Compose exists, PostGIS starts healthy, SQLx migrations run through a one-shot service, the six-table lineage schema is created empty, and migration reruns are safe.

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

Implement Slice 3: add API configuration, SQLx pool setup, Actix health/readiness endpoints, async-graphql connectivity response, bounded CORS, request IDs, and an API container health check after successful migration completion.

---

## 12. Exit Checklist

- [ ] All in-scope phase requirements are complete.
- [ ] Required automated tests and CI jobs pass.
- [ ] Required documentation is complete.
- [ ] All eight required UAT cases have been executed.
- [ ] UAT result is `passed` or `passed_with_accepted_exceptions`.
- [ ] No critical or high defects remain open.
- [ ] Handoff notes are completed in `PHASE-PLAN.md`.
- [ ] `.planning/STATE.md` is updated to Phase 02.
- [ ] Overall status is `completed` or `completed_with_exceptions`.

---

## 13. Update Log

| Timestamp | Status | Update |
|---|---|---|
| 2026-06-24 17:55 +07:00 | `ready_for_implementation` | Decision-complete plan, status, and UAT documents created; implementation has not started. |
| 2026-06-24 18:27 +07:00 | `in_progress` | Slice 1 completed; pinned Rust checks, frozen pnpm install, web lint/typecheck/test, and Next production build pass. |
| 2026-06-25 14:05 +07:00 | `in_progress` | Slice 2 completed; Compose/PostGIS/migration lifecycle and schema smoke validation pass; focus moves to Slice 3 API behavior. |
