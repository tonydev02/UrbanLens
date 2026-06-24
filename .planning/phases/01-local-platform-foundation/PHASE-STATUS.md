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
| Last Updated | `2026-06-24 18:27 +07:00` |
| Target Completion | `TBD` |
| Current Branch | `main` |
| Current Commit | `169f517` |
| Related Plan | `PHASE-PLAN.md` |
| Related UAT | `PHASE-UAT.md` |

## 1. Current Objective

Build a clean-clone local platform in which one root Docker Compose command starts the Next.js web application, Rust/Actix GraphQL API, and PostgreSQL/PostGIS database after automatically applying the foundation migrations.

## 2. Current Focus

Slice 1 is complete and validated. Current focus moves to Slice 2: the root/infra Compose lifecycle, PostGIS service, SQLx migrations, and dedicated migration binary.

## 3. Definition of Done

The phase is done when a new developer can run `docker compose up --build`, open `/market-map`, see successful GraphQL/PostgreSQL connectivity, rerun the stack safely, and obtain passing Rust, frontend, migration, and Compose checks in CI.

---

## 4. Progress Snapshot

| Area | Status | Progress | Notes |
|---|---|---:|---|
| Planning | Done | 100% | Plan, decisions, interfaces, schema foundation, and UAT protocol are complete. |
| Design / Architecture | In Progress | 20% | Workspace/package boundaries and environment contract are implemented. |
| Backend | In Progress | 10% | API crate compiles; Actix/GraphQL behavior remains Slice 3. |
| Database | Not Started | 0% | Six tables, extensions, and migration lifecycle are specified. |
| Worker / Ingestion | In Progress | 25% | Compile-only importer crate exists; ingestion remains intentionally absent. |
| Frontend | In Progress | 20% | Next.js 16 scaffold, strict TypeScript, CSS, and one foundation test pass. |
| Infrastructure | In Progress | 10% | Ignore rules and environment contract exist; Compose is next. |
| Tests | In Progress | 20% | Shared Rust/web checks pass; database/API/Compose coverage remains. |
| Documentation | In Progress | 20% | Environment contract and planning records are current; runtime docs remain. |
| UAT | In Progress | 10% | UAT-07 and UAT-08 have partial Slice 1 evidence; neither is complete. |

---

## 5. Completed Work

| Date | Completed Outcome | Evidence / Link |
|---|---|---|
| 2026-06-24 | Made the Phase 01 implementation plan decision-complete, including public interfaces, schema constraints, startup order, CI, and failure behavior. | `PHASE-PLAN.md` |
| 2026-06-24 | Defined traceable UAT for clean-clone startup, migrations, API/GraphQL, frontend states, dependency failure, restart, and CI. | `PHASE-UAT.md` |
| 2026-06-24 | Preserved the accepted dataset-artifact lineage by including `datasets` in the physical schema plan. | `PHASE-PLAN.md`, `docs/data-model.md` |
| 2026-06-24 | Completed Slice 1 with Cargo/pnpm workspaces, pinned toolchains, lockfiles, package scaffolds, environment/ignore contracts, shared checks, and a tested Next.js foundation. | `Cargo.toml`, `package.json`, `.env.example`, `apps/`, `crates/`, `workers/importer/`, `scripts/` |

---

## 6. Work In Progress

| Item | Current State | Next Step |
|---|---|---|
| Slice 1 workspace foundation | Complete | Preserve the shared check scripts as the CI command boundary. |
| Compose/database foundation | Ready to start | Add root/infra Compose definitions, PostGIS health check, migrations, and migration binary. |
| API/frontend foundation | Planned | Begin only after workspace and database lifecycle compile and start. |

---

## 7. Exact Next Actions

1. [x] Add root Cargo/pnpm workspace manifests, `rust-toolchain.toml`, package/crate skeletons, lockfiles, shared checks, and the environment contract.
2. [ ] **Next immediate action:** Add the root Compose entrypoint and `infra/docker-compose.yml` with the PostGIS service, volume, safe defaults, and health check.
3. [ ] Add extension/schema migrations and the dedicated embedded migration binary before implementing API routes.
4. [ ] Verify the fresh-volume and migration-rerun paths before proceeding to API behavior.

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
| RSK-02 | Compose startup races migrations or readiness. | Medium | High | Use health/service-completion dependencies and smoke-test ordering. | Open |
| RSK-03 | Foundational schema weakens exact-artifact lineage. | Low | High | Enforce dataset-position identity and consistent dataset/import-run references. | Open |
| RSK-04 | Host Cargo is unavailable. | High | Low | Pinned Rust 1.96 container runs the authoritative shared check successfully. | Mitigated |
| RSK-05 | Optional MLIT diagnostic exposes the key. | Low | High | No shell tracing/header output, no CI credential, and output disclosure test. | Open |

### Dependencies

| Dependency | Status | Required By | Next Action |
|---|---|---|---|
| Docker Compose | Available locally (`v5.0.2`) | Slice 2 and UAT | Validate minimum supported version in documentation/CI. |
| Node.js 24 / pnpm 10 | Validated (`v24.2.0` / `10.12.1`) | Slice 1/4 | Keep declarations and lockfile aligned. |
| Rust 1.96 | Validated in pinned container; host unavailable | Slice 1/3 | Reuse `scripts/check-rust.sh` in CI. |
| PostGIS 17-3.5 image | Selected | Slice 2 | Pin image line in Compose. |
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
| TypeScript lint | Pass | `pnpm check:web` using ESLint 9 / Next.js rules |
| TypeScript type check | Pass | `next typegen && tsc --noEmit` |
| Frontend tests | Pass | Vitest 4.1.9: 1 file, 1 test passed |
| Frontend production build | Pass | Next.js 16.2.9 compiled and prerendered `/` and `/_not-found` |
| Database/migration tests | Not Run | No migrations exist yet. |
| Integration tests | Not Run | Platform is not implemented. |
| Docker Compose smoke test | Not Run | Compose definition is not implemented. |

### UAT Status

| Field | Value |
|---|---|
| UAT Readiness | `not_ready` |
| UAT Result | `in_progress — partial Slice 1 evidence only` |
| Blocking Defects | `0` |
| Required Cases | `8` |
| Optional Cases | `1` |
| UAT File | `PHASE-UAT.md` |

---

## 11. Resume Context

### Last Meaningful Change

Slice 1 was implemented and validated: workspaces, pinned tools, lockfiles, package scaffolds, ignores/environment contract, and shared Rust/web checks are available.

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

Implement Slice 2: add the root/infra Compose entrypoint, `postgis/postgis:17-3.5` service and health check, SQLx extension/schema migrations, and the dedicated migration binary. Validate fresh-volume and rerun behavior before adding API routes.

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
