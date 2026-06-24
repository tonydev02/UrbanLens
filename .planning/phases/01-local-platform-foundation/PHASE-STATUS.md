# Phase 01 Status — Local Platform Foundation

> Purpose: Record the current phase state, progress, blockers, and exact next action.

## Current State

| Field | Value |
|---|---|
| Phase | `01` |
| Name | `Local Platform Foundation` |
| Overall Status | `ready_for_implementation` |
| Health | `green` |
| Owner | `Project owner` |
| Started | `not_started` |
| Last Updated | `2026-06-24 17:55 +07:00` |
| Target Completion | `TBD` |
| Current Branch | `main` |
| Current Commit | `8b66ed9` |
| Related Plan | `PHASE-PLAN.md` |
| Related UAT | `PHASE-UAT.md` |

## 1. Current Objective

Build a clean-clone local platform in which one root Docker Compose command starts the Next.js web application, Rust/Actix GraphQL API, and PostgreSQL/PostGIS database after automatically applying the foundation migrations.

## 2. Current Focus

Planning is decision-complete. Implementation should begin with workspace manifests, the shared environment contract, the root/infra Compose entrypoint, the PostgreSQL service, and the dedicated migration binary.

## 3. Definition of Done

The phase is done when a new developer can run `docker compose up --build`, open `/market-map`, see successful GraphQL/PostgreSQL connectivity, rerun the stack safely, and obtain passing Rust, frontend, migration, and Compose checks in CI.

---

## 4. Progress Snapshot

| Area | Status | Progress | Notes |
|---|---|---:|---|
| Planning | Done | 100% | Plan, decisions, interfaces, schema foundation, and UAT protocol are complete. |
| Design / Architecture | Not Started | 0% | Accepted ADRs constrain implementation; no new ADR currently required. |
| Backend | Not Started | 0% | Actix/GraphQL health foundation is planned. |
| Database | Not Started | 0% | Six tables, extensions, and migration lifecycle are specified. |
| Worker / Ingestion | Not Started | 0% | Only a compile-only importer workspace placeholder is in scope. |
| Frontend | Not Started | 0% | Shell, placeholder route, and connectivity states are planned. |
| Infrastructure | Not Started | 0% | Root Compose entrypoint and four-service lifecycle are planned. |
| Tests | Not Started | 0% | Unit, integration, frontend, and smoke coverage are specified. |
| Documentation | Not Started | 0% | README, architecture, local-development, and environment updates are required. |
| UAT | Not Started | 0% | Eight required cases plus one optional source-connectivity case are defined. |

---

## 5. Completed Work

| Date | Completed Outcome | Evidence / Link |
|---|---|---|
| 2026-06-24 | Made the Phase 01 implementation plan decision-complete, including public interfaces, schema constraints, startup order, CI, and failure behavior. | `PHASE-PLAN.md` |
| 2026-06-24 | Defined traceable UAT for clean-clone startup, migrations, API/GraphQL, frontend states, dependency failure, restart, and CI. | `PHASE-UAT.md` |
| 2026-06-24 | Preserved the accepted dataset-artifact lineage by including `datasets` in the physical schema plan. | `PHASE-PLAN.md`, `docs/data-model.md` |

---

## 6. Work In Progress

| Item | Current State | Next Step |
|---|---|---|
| Phase 01 implementation | Ready to start | Create root workspace/toolchain manifests and safe environment defaults. |
| Compose/database foundation | Planned | Add root/infra Compose definitions, PostGIS health check, migrations, and migration binary. |
| API/frontend foundation | Planned | Begin only after workspace and database lifecycle compile and start. |

---

## 7. Exact Next Actions

1. [ ] **Next immediate action:** Add root Cargo/pnpm workspace manifests, `rust-toolchain.toml`, package/crate skeletons, and lockfiles.
2. [ ] Define the shared `.env.example` contract and safe Compose defaults, then add the root Compose entrypoint and PostGIS service.
3. [ ] Add extension/schema migrations and the dedicated embedded migration binary before implementing API routes.
4. [ ] Verify the fresh-volume migration path before proceeding to the API and frontend slices.

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
| RSK-04 | Host Cargo is unavailable. | High | Low | Make Docker/CI the authoritative pinned Rust environment; document optional host setup. | Open |
| RSK-05 | Optional MLIT diagnostic exposes the key. | Low | High | No shell tracing/header output, no CI credential, and output disclosure test. | Open |

### Dependencies

| Dependency | Status | Required By | Next Action |
|---|---|---|---|
| Docker Compose | Available locally (`v5.0.2`) | Slice 2 and UAT | Validate minimum supported version in documentation/CI. |
| Node.js 24 / pnpm 10 | Available locally | Slice 1/4 | Pin supported version lines and commit lockfile. |
| Rust 1.96 | Host unavailable; container planned | Slice 1/3 | Add toolchain file and Docker build environment. |
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
| Rust formatting | Not Run | No Rust workspace exists yet. |
| Rust lint | Not Run | No Rust workspace exists yet. |
| Rust tests | Not Run | No Rust workspace exists yet. |
| TypeScript lint | Not Run | No web workspace exists yet. |
| TypeScript type check | Not Run | No web workspace exists yet. |
| Frontend tests | Not Run | No web workspace exists yet. |
| Database/migration tests | Not Run | No migrations exist yet. |
| Integration tests | Not Run | Platform is not implemented. |
| Docker Compose smoke test | Not Run | Compose definition is not implemented. |

### UAT Status

| Field | Value |
|---|---|
| UAT Readiness | `not_ready` |
| UAT Result | `not_started` |
| Blocking Defects | `0` |
| Required Cases | `8` |
| Optional Cases | `1` |
| UAT File | `PHASE-UAT.md` |

---

## 11. Resume Context

### Last Meaningful Change

Phase 01 planning documents were created from the repository templates and aligned with completed Phase 0 decisions.

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

Add the root Cargo and pnpm workspace manifests, pinned toolchain/version files, package/crate skeletons, and safe environment-variable contract. Then create the root/infra Compose entrypoint, PostGIS health check, and dedicated migration binary before adding API behavior.

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
