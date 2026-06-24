# Phase 00 Status — Product and Data Discovery

> Purpose: Record the current phase state, progress, blocker, and exact next action.

## Current State

| Field | Value |
|---|---|
| Phase | `00` |
| Name | `Product and Data Discovery` |
| Overall Status | `blocked` |
| Health | `red` |
| Owner | `Project owner` |
| Started | `2026-06-24` |
| Last Updated | `2026-06-24 07:24 +07:00` |
| Target Completion | `Immediately after API application submission is confirmed` |
| Current Branch | `main` |
| Current Commit | `N/A — repository has no initial commit` |
| Related Plan | `PHASE-PLAN.md` |
| Related UAT | `PHASE-UAT.md` |

## 1. Current Objective

Close the only remaining external prerequisite: submit the MLIT API application. All repository-controlled Phase 0 source, fixture, workflow, domain, ADR, and validation work is complete.

## 2. Current Focus

The selected source is verified, official fixtures are committed, documentation and ADRs are complete, and six of eight UAT scenarios pass. UAT-01 and UAT-08 are blocked because API application submission requires the user’s identity, agreement, and attestations and has not been confirmed.

## 3. Definition of Done

The phase is done when the user confirms API application submission, the source register records `requested—approval pending`, the two blocked UAT cases pass, and project state activates Phase 01.

## 4. Progress Snapshot

| Area | Status | Progress | Notes |
|---|---|---:|---|
| Planning | Done | 100% | Plan and traceable acceptance protocol complete. |
| Source / Access Discovery | Blocked | 90% | Source/CSV/API verified; user application submission pending. |
| Product / Workflow | Done | 100% | Workflow, metrics, states, map, and claim boundary documented. |
| Design / Architecture | Done | 100% | Conceptual model and ADRs 001–005 complete. |
| Database | Done | 100% | Conceptual model complete; physical migration correctly deferred. |
| Worker / Ingestion | Done | 100% | Three official raw fixtures and integrity metadata available. |
| Frontend | Done | 100% | Product/map behavior specified; implementation correctly deferred. |
| Tests | Done | 100% | Fixture parse/assertions, checksums, structure, placeholder, and secret scans pass. |
| Documentation | Done | 100% | All required deliverables exist. |
| UAT | Blocked | 75% | 6 passed, 2 blocked, 0 failed. |

## 5. Completed Outcomes

| Date | Outcome | Evidence |
|---|---|---|
| 2026-06-24 | Selected MLIT `不動産取引価格情報`; excluded `成約価格情報` and documented alternatives/terms/access. | `docs/data-sources.md` |
| 2026-06-24 | Retrieved and preserved 666 official 2024 Q4 observations across Chuo, Shinagawa, and Shibuya. | `workers/importer/fixtures/transactions/` |
| 2026-06-24 | Defined the analyst workflow, station-context map, metrics, UI states, and explicit claims/non-claims. | `docs/product-brief.md` |
| 2026-06-24 | Defined lineage, idempotency, validation, metric eligibility, and location precision without durable property identity. | `docs/data-model.md` |
| 2026-06-24 | Accepted PostGIS, GraphQL, raw-payload, location-precision, and Rust/Actix decisions. | `docs/adr/001-*.md` through `005-*.md` |
| 2026-06-24 | Passed all repository-controlled validation and six UAT cases. | `PHASE-UAT.md` |

## 6. Exact Next Actions

1. [ ] **User action:** Submit <https://www.reinfolib.mlit.go.jp/api/request/> and provide the submission date; do not share the issued key in chat or commit it.
2. [ ] Change API status in `docs/data-sources.md` from `action_required` to `requested—approval pending`.
3. [ ] Mark UAT-01 and UAT-08 passed, then set Phase 0 to `completed` and activate Phase 01.

## 7. Blocker and Risks

### Blocker

| ID | Blocker | Impact | Owner | Since | Resolution |
|---|---|---|---|---|---|
| BLK-01 | MLIT API application submission is unconfirmed. | Prevents required access-status gate and final UAT pass. | User | 2026-06-24 | Submit application and confirm date; approval may remain pending. |

### Residual Risks

| ID | Risk | Likelihood | Impact | Mitigation |
|---|---|---:|---:|---|
| RSK-01 | API approval is delayed/denied. | Medium | Medium | CSV path supports discovery/import fixture; track approval separately. |
| RSK-02 | Source revisions change historical rows. | Medium | Medium | Treat every checksum/query retrieval as a distinct dataset artifact. |
| RSK-03 | XPT features cannot be safely linked to CSV/XIT records. | High | High | Treat XPT feature as its own raw record or leave CSV/XIT location unknown; never guess joins. |
| RSK-04 | Older source variants differ from 2024 fixtures. | Medium | Medium | Expand fixture coverage in ingestion phase before claiming historical schema completeness. |

## 8. Decisions Made

| Date | Decision | Rationale | Reference |
|---|---|---|---|
| 2026-06-24 | Select only MLIT transaction-price information for MVP. | Narrow, official, source-grounded workflow without mixed provenance. | `docs/data-sources.md` |
| 2026-06-24 | Use station-context aggregates for the first map. | XPT001 points represent nearest stations, not properties. | ADR-004 |
| 2026-06-24 | Keep CSV/XIT observations spatially unknown unless a defensible geometry link exists. | Prevents false row-level joining and exactness. | `docs/data-model.md` |
| 2026-06-24 | Preserve identical published rows using artifact identity + row ordinal. | Content-hash dedup could collapse legitimate transactions. | `docs/data-model.md` |
| 2026-06-24 | Keep ¥/m² eligibility limited to publisher-populated values. | 599 of 666 fixture records omit the field and asset area semantics differ. | `docs/product-brief.md` |

## 9. Validation Status

| Check | Result | Evidence |
|---|---|---|
| Required files | Pass | All product/data/ADR/fixture artifacts exist. |
| Fixture checksum | Pass | All three SHA-256 checks return `OK`. |
| CSV parsing | Pass | CP932, 30 columns; 176/313/177 records. |
| Query invariants | Pass | Correct wards, 2024 Q4, transaction-price only. |
| Representative types | Pass | All three MVP types in each fixture. |
| Raw preservation | Pass | Source bytes, blank values, encoding, and mixed final line ending retained. |
| Template placeholders | Pass | None in deliverables. |
| ADR structure | Pass | Required five headings in every ADR. |
| Secret scan | Pass | No MLIT/private key committed. |
| Rust/TypeScript/Docker tests | Not Applicable | No application code or infrastructure is in Phase 0. |

### UAT Status

| Field | Value |
|---|---|
| UAT Readiness | `blocked_on_external_prerequisite` |
| UAT Result | `blocked` |
| Passed / Blocked / Failed | `6 / 2 / 0` |
| Blocking Defects | `0` |
| External Blockers | `1` |
| UAT File | `PHASE-UAT.md` |

## 10. Resume Context

### Important Files

```text
docs/data-sources.md — selected source, schema, access status, fixture profile
docs/product-brief.md — first analyst workflow and claim boundary
docs/data-model.md — conceptual lineage, idempotency, precision, metrics
.planning/phases/00-product-and-data-discovery/PHASE-UAT.md — exact blocked acceptance cases
```

### Recommended Resume Command

```bash
sed -n '1,220p' .planning/phases/00-product-and-data-discovery/PHASE-STATUS.md
```

### Exact Next Technical Step

After the user confirms the MLIT application submission date, update one access-status row in `docs/data-sources.md`, rerun UAT-01/UAT-08, and close Phase 0 without waiting for key approval.

## 11. Exit Checklist

- [x] One official historical transaction-price source is selected and documented.
- [ ] Required API access is requested without committing secrets.
- [x] Documented representative fixtures exist.
- [x] First analyst workflow and product claim boundary are complete.
- [x] Geographic accuracy and all location-precision rules are documented.
- [x] Conceptual domain model and source mapping are agreed.
- [x] ADRs 001–005 exist and agree with discovery evidence.
- [x] Required repository validation checks pass.
- [x] All UAT cases were executed.
- [ ] UAT result is `passed` or `passed_with_accepted_exceptions`.
- [x] No critical or high product/data defects remain open.
- [x] Handoff notes are completed in `PHASE-PLAN.md`.
- [ ] `.planning/STATE.md` is updated to the next active phase.
- [ ] Overall status is `completed` or `completed_with_exceptions`.

## 12. Update Log

| Timestamp | Status | Update |
|---|---|---|
| 2026-06-24 06:50 +07:00 | `ready_for_implementation` | Planning documents created. |
| 2026-06-24 07:24 +07:00 | `blocked` | Repository work and validation complete; user API application submission is the only remaining exit prerequisite. |
