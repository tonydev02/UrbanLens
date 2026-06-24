# Phase {{PHASE_NUMBER}} UAT — {{PHASE_NAME}}

> Purpose: Verify that the phase is complete from a user and system perspective, not merely that implementation tasks were checked off.

## Metadata

| Field | Value |
|---|---|
| Phase | `{{PHASE_NUMBER}}` |
| Name | `{{PHASE_NAME}}` |
| UAT Status | `not_started` |
| Environment | `local / staging / production-like` |
| Tester | `{{TESTER}}` |
| Started | `YYYY-MM-DD` |
| Completed | `YYYY-MM-DD` |
| Build / Commit | `{{COMMIT_SHA_OR_TAG}}` |
| Related Plan | `PHASE-PLAN.md` |
| Related Status | `PHASE-STATUS.md` |

---

## 1. UAT Objective

<!-- Describe what this UAT proves in plain language. -->

Example:

> Verify that the importer can load an official transaction dataset, preserve raw records, create normalized observations, record validation warnings, and avoid duplicates when run twice.

---

## 2. Preconditions

### Required Setup

- [ ] Correct branch is checked out.
- [ ] Required environment variables are configured.
- [ ] Docker or local dependencies are running.
- [ ] Required migrations have been applied.
- [ ] Required fixture or source data is available.
- [ ] Test accounts or permissions are available, if needed.

### Test Data

| Data Set / Fixture | Purpose | Setup Command / Location |
|---|---|---|
|  |  |  |
|  |  |  |

### Known Limitations

- [ ]
- [ ]

---

## 3. Acceptance Criteria Traceability

| UAT ID | Related Acceptance Criteria | Scenario | Required Result |
|---|---|---|---|
| UAT-01 |  |  |  |
| UAT-02 |  |  |  |
| UAT-03 |  |  |  |

---

## 4. UAT Test Cases

### UAT-01 — {{SCENARIO_NAME}}

**Purpose**

<!-- What user or system outcome does this verify? -->

**Preconditions**

- [ ]
- [ ]

**Steps**

1.
2.
3.

**Expected Result**

- [ ]
- [ ]

**Actual Result**

<!-- Fill in during UAT. -->

**Status:** `not_run`

**Evidence**

<!-- Add screenshots, logs, command output, URLs, commit references, or file paths. -->

---

### UAT-02 — {{SCENARIO_NAME}}

**Purpose**

<!-- What user or system outcome does this verify? -->

**Preconditions**

- [ ]
- [ ]

**Steps**

1.
2.
3.

**Expected Result**

- [ ]
- [ ]

**Actual Result**

<!-- Fill in during UAT. -->

**Status:** `not_run`

**Evidence**

<!-- Add screenshots, logs, command output, URLs, commit references, or file paths. -->

---

### UAT-03 — {{SCENARIO_NAME}}

**Purpose**

<!-- What user or system outcome does this verify? -->

**Preconditions**

- [ ]
- [ ]

**Steps**

1.
2.
3.

**Expected Result**

- [ ]
- [ ]

**Actual Result**

<!-- Fill in during UAT. -->

**Status:** `not_run`

**Evidence**

<!-- Add screenshots, logs, command output, URLs, commit references, or file paths. -->

---

## 5. Failure and Edge-Case Validation

| UAT ID | Scenario | Expected Behavior | Actual Result | Status |
|---|---|---|---|---|
| UAT-E01 |  |  |  | `not_run` |
| UAT-E02 |  |  |  | `not_run` |
| UAT-E03 |  |  |  | `not_run` |

Suggested cases:

- Invalid user input is rejected clearly.
- Empty search results show an honest empty state.
- A failed dependency produces a readable error.
- Repeated import does not create duplicate data.
- Invalid source records are recorded as warnings or rejections.
- Location precision is not overstated.
- A metric displays no result when the sample size is insufficient.

---

## 6. Data Integrity Validation

<!-- Use this when the phase changes ingestion, database structure, geographic data, or market metrics. -->

| Check | Expected Result | Actual Result | Status |
|---|---|---|---|
| Source lineage | Normalized data links to source and raw record |  | `not_run` |
| Idempotency | Re-running the same import creates no duplicates |  | `not_run` |
| Validation visibility | Warnings and rejected records are stored and visible |  | `not_run` |
| Location precision | UI/API does not imply false exact location |  | `not_run` |
| Metric reproducibility | Metrics match filters, time range, and source data |  | `not_run` |

---

## 7. Evidence Register

| Evidence ID | Type | Description | Location |
|---|---|---|---|
| EV-01 | Screenshot |  |  |
| EV-02 | Test output |  |  |
| EV-03 | Log excerpt |  |  |
| EV-04 | Video / GIF |  |  |

---

## 8. Defects Found

| Defect ID | Severity | Description | Reproduction Steps | Owner | Status |
|---|---|---|---|---|---|
| DEF-01 | Critical / High / Medium / Low |  |  |  | Open |
| DEF-02 | Critical / High / Medium / Low |  |  |  | Open |

### Severity Guide

| Severity | Meaning |
|---|---|
| Critical | Core phase outcome cannot be used, data integrity is at risk, or a security issue exists. |
| High | Major workflow is broken or misleading with no reasonable workaround. |
| Medium | Important issue exists but there is a reasonable workaround. |
| Low | Cosmetic, minor usability, or non-blocking issue. |

---

## 9. UAT Summary

| Metric | Count |
|---|---:|
| Total UAT Cases | 0 |
| Passed | 0 |
| Failed | 0 |
| Blocked | 0 |
| Not Run | 0 |
| Open Critical Defects | 0 |
| Open High Defects | 0 |

### Final UAT Decision

- [ ] `passed` — All required UAT cases pass. No critical or high defects remain.
- [ ] `passed_with_accepted_exceptions` — Remaining issues are documented and accepted.
- [ ] `failed` — Required behavior is incomplete or blocking defects remain.
- [ ] `blocked` — UAT cannot continue because prerequisites are unavailable.

### Accepted Exceptions

| Exception | Reason | Follow-Up Phase / Issue |
|---|---|---|
|  |  |  |

---

## 10. Next Action

<!-- Complete after UAT ends. -->

### If Passed

- Update `PHASE-STATUS.md` to `completed`.
- Update `.planning/STATE.md` to the next active phase.

### If Passed With Exceptions

- Document accepted exceptions.
- Create follow-up work in the appropriate future phase.
- Update `PHASE-STATUS.md` to `completed_with_exceptions`.

### If Failed

- Add blocking defects to `PHASE-STATUS.md`.
- Set phase status to `in_progress` or `blocked`.
- Define the exact next remediation action.
