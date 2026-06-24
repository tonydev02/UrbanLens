# Phase {{PHASE_NUMBER}} Status — {{PHASE_NAME}}

> Purpose: Record the current phase state, progress, blockers, and exact next action.

## Current State

| Field | Value |
|---|---|
| Phase | `{{PHASE_NUMBER}}` |
| Name | `{{PHASE_NAME}}` |
| Overall Status | `not_started` |
| Health | `green` |
| Owner | `{{OWNER}}` |
| Started | `not_started` |
| Last Updated | `YYYY-MM-DD HH:MM` |
| Target Completion | `{{DATE_OR_TBD}}` |
| Current Branch | `{{BRANCH_NAME}}` |
| Current Commit | `{{COMMIT_SHA_OR_NA}}` |
| Related Plan | `PHASE-PLAN.md` |
| Related UAT | `PHASE-UAT.md` |

### Allowed Status Values

```text
not_started
planning
ready_for_implementation
in_progress
blocked
ready_for_uat
uat_in_progress
completed
completed_with_exceptions
cancelled
```

### Allowed Health Values

```text
green
yellow
red
```

- `green`: Work is progressing normally.
- `yellow`: There is meaningful risk, uncertainty, or dependency.
- `red`: Work is blocked or the plan needs material revision.

---

## 1. Current Objective

<!-- One or two sentences describing the current phase goal. -->

## 2. Current Focus

<!-- What is actively being worked on right now? -->

## 3. Definition of Done

<!-- Short version of the completion criteria. Full definition remains in PHASE-PLAN.md. -->

---

## 4. Progress Snapshot

| Area | Status | Progress | Notes |
|---|---|---:|---|
| Planning | Not Started / In Progress / Done | 0% |  |
| Design / Architecture | Not Started / In Progress / Done | 0% |  |
| Backend | Not Started / In Progress / Done | 0% |  |
| Database | Not Started / In Progress / Done | 0% |  |
| Worker / Ingestion | Not Started / In Progress / Done | 0% |  |
| Frontend | Not Started / In Progress / Done | 0% |  |
| Tests | Not Started / In Progress / Done | 0% |  |
| Documentation | Not Started / In Progress / Done | 0% |  |
| UAT | Not Started / In Progress / Done | 0% |  |

---

## 5. Completed Work

Record outcomes, not just activity.

| Date | Completed Outcome | Evidence / Link |
|---|---|---|
| YYYY-MM-DD |  |  |
| YYYY-MM-DD |  |  |

---

## 6. Work In Progress

| Item | Current State | Next Step |
|---|---|---|
|  |  |  |
|  |  |  |

---

## 7. Exact Next Actions

Keep this short. The first action must be the exact action to take when work resumes.

1. [ ] **Next immediate action:**
2. [ ]
3. [ ]

---

## 8. Blockers, Risks, and Dependencies

### Blockers

| ID | Blocker | Impact | Owner | Since | Next Action |
|---|---|---|---|---|---|
| BLK-01 |  |  |  |  |  |

### Risks

| ID | Risk | Likelihood | Impact | Mitigation | Status |
|---|---|---:|---:|---|---|
| RSK-01 |  | Low / Medium / High | Low / Medium / High |  | Open |

### Dependencies

| Dependency | Status | Required By | Next Action |
|---|---|---|---|
|  |  |  |  |

---

## 9. Decisions and Plan Changes

### Decisions Made During Implementation

| Date | Decision | Rationale | ADR / Reference |
|---|---|---|---|
| YYYY-MM-DD |  |  |  |

### Changes From Original Plan

| Date | Original Plan | Change | Reason | Impact |
|---|---|---|---|---|
| YYYY-MM-DD |  |  |  |  |

---

## 10. Validation Status

### Automated Validation

| Check | Latest Result | Evidence |
|---|---|---|
| Rust formatting | Not Run / Pass / Fail |  |
| Rust lint | Not Run / Pass / Fail |  |
| Rust tests | Not Run / Pass / Fail |  |
| TypeScript lint | Not Run / Pass / Fail |  |
| TypeScript type check | Not Run / Pass / Fail |  |
| Frontend tests | Not Run / Pass / Fail |  |
| Integration tests | Not Run / Pass / Fail |  |
| Docker Compose smoke test | Not Run / Pass / Fail |  |

### UAT Status

| Field | Value |
|---|---|
| UAT Readiness | `not_ready` |
| UAT Result | `not_started` |
| Blocking Defects | `0` |
| UAT File | `PHASE-UAT.md` |

---

## 11. Resume Context

### Last Meaningful Change

<!-- What was changed most recently? -->

### Current Working Assumption

<!-- Important assumption currently guiding implementation. -->

### Important Files

```text
{{FILE_PATH_1}} — {{WHY_IT_MATTERS}}
{{FILE_PATH_2}} — {{WHY_IT_MATTERS}}
{{FILE_PATH_3}} — {{WHY_IT_MATTERS}}
```

### Recommended Resume Command

```bash
{{COMMAND_TO_RUN_FIRST}}
```

### Exact Next Technical Step

<!-- Be concrete.
Example:
"Add the SQLx migration for import_runs and raw_records, then write the fixture-import integration test before implementing the CLI command."
-->

---

## 12. Exit Checklist

- [ ] All in-scope phase requirements are complete.
- [ ] Required automated tests pass.
- [ ] Required documentation is complete.
- [ ] UAT cases have been executed.
- [ ] UAT result is `passed` or `passed_with_accepted_exceptions`.
- [ ] No critical or high defects remain open.
- [ ] Handoff notes are completed in `PHASE-PLAN.md`.
- [ ] `.planning/STATE.md` is updated.
- [ ] Overall status is `completed` or `completed_with_exceptions`.

---

## 13. Update Log

Append one concise row whenever the phase changes meaningfully.

| Timestamp | Status | Update |
|---|---|---|
| YYYY-MM-DD HH:MM | `not_started` | Phase created from template. |
