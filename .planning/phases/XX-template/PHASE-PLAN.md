# Phase {{PHASE_NUMBER}} Plan — {{PHASE_NAME}}

> Purpose: Define the goal, scope, implementation approach, and completion criteria before implementation begins.

## Metadata

| Field | Value |
|---|---|
| Phase | `{{PHASE_NUMBER}}` |
| Name | `{{PHASE_NAME}}` |
| Status | `planning` |
| Owner | `{{OWNER}}` |
| Created | `YYYY-MM-DD` |
| Last Updated | `YYYY-MM-DD` |
| Target Milestone | `{{MILESTONE}}` |
| Related ADRs | `{{ADR_LINKS_OR_NA}}` |

---

## 1. Objective

### Problem

<!-- What user, product, or engineering problem does this phase solve? -->

### Intended Outcome

<!-- Describe what will be true when this phase is complete. -->

### Why This Matters

<!-- Explain why this phase matters to UrbanLens or its users. -->

---

## 2. Scope

### In Scope

- [ ]
- [ ]
- [ ]

### Out of Scope

- [ ]
- [ ]
- [ ]

### Deferred Ideas

<!-- Good ideas that are intentionally postponed to a later phase. -->

- [ ]
- [ ]

---

## 3. Requirements

### Functional Requirements

| ID | Requirement | Priority | Notes |
|---|---|---:|---|
| FR-01 |  | Must |  |
| FR-02 |  | Must |  |
| FR-03 |  | Should |  |

### Non-Functional Requirements

| ID | Requirement | Priority | Verification Method |
|---|---|---:|---|
| NFR-01 |  | Must |  |
| NFR-02 |  | Should |  |
| NFR-03 |  | Should |  |

### Data / Domain Requirements

<!-- Required when this phase changes data sources, schemas, metrics, geographic behavior, or domain concepts. -->

| ID | Requirement | Source / Assumption | Notes |
|---|---|---|---|
| DR-01 |  |  |  |
| DR-02 |  |  |  |

---

## 4. Technical Design

### Proposed Approach

<!-- Explain the planned architecture, main workflow, and important technical decisions. -->

### Components Affected

| Component | Planned Change | Reason |
|---|---|---|
| `apps/web` |  |  |
| `apps/api` |  |  |
| `workers/importer` |  |  |
| `database` |  |  |
| `infra` |  |  |
| `docs` |  |  |

### Data Flow

```text
{{TRIGGER_OR_SOURCE}}
  ↓
{{PROCESSING_STEP}}
  ↓
{{STORAGE_OR_SERVICE}}
  ↓
{{USER_OR_SYSTEM_OUTCOME}}
```

### API / Interface Changes

| Type | Name | Change | Consumers |
|---|---|---|---|
| GraphQL Query |  |  |  |
| GraphQL Mutation |  |  |  |
| CLI Command |  |  |  |
| Database Migration |  |  |  |
| Environment Variable |  |  |  |

### Data Model Changes

| Entity / Table | Change | Migration Required | Notes |
|---|---|---:|---|
|  |  | Yes / No |  |
|  |  | Yes / No |  |

### Geographic / Data Precision Notes

<!-- Required for maps, locations, transactions, stations, or public datasets. -->

- Location precision:
- User-facing disclaimer:
- Known data limitations:
- Assumptions:

---

## 5. Implementation Slices

Break the work into small, reviewable outcomes.

### Slice 1 — {{SLICE_NAME}}

**Goal**

<!-- Smallest meaningful outcome. -->

**Tasks**

- [ ]
- [ ]
- [ ]

**Expected Evidence**

- [ ]
- [ ]

---

### Slice 2 — {{SLICE_NAME}}

**Goal**

<!-- Smallest meaningful outcome. -->

**Tasks**

- [ ]
- [ ]
- [ ]

**Expected Evidence**

- [ ]
- [ ]

---

### Slice 3 — {{SLICE_NAME}}

**Goal**

<!-- Smallest meaningful outcome. -->

**Tasks**

- [ ]
- [ ]
- [ ]

**Expected Evidence**

- [ ]
- [ ]

---

## 6. Testing Strategy

### Unit Tests

| Area | Required Coverage |
|---|---|
|  |  |
|  |  |

### Integration Tests

| Flow | Expected Result |
|---|---|
|  |  |
|  |  |

### Manual Validation

| Scenario | Why Manual Validation Is Needed |
|---|---|
|  |  |
|  |  |

### Regression Risks

| Risk Area | Possible Regression | Mitigation |
|---|---|---|
|  |  |  |
|  |  |  |

---

## 7. Acceptance Criteria

### Product / User Criteria

- [ ]
- [ ]
- [ ]

### Engineering Criteria

- [ ]
- [ ]
- [ ]

### Documentation Criteria

- [ ]
- [ ]
- [ ]

### UAT Criteria

- [ ]
- [ ]
- [ ]

---

## 8. Dependencies, Risks, and Open Questions

### Dependencies

| Dependency | Owner / Source | Status | Impact if Missing |
|---|---|---|---|
|  |  |  |  |

### Risks

| Risk | Likelihood | Impact | Mitigation |
|---|---:|---:|---|
|  | Low / Medium / High | Low / Medium / High |  |
|  | Low / Medium / High | Low / Medium / High |  |

### Open Questions

- [ ]
- [ ]

---

## 9. Planning Decisions

| Decision | Rationale | ADR Required? |
|---|---|---:|
|  |  | Yes / No |
|  |  | Yes / No |

---

## 10. Completion Definition

This phase is complete when:

- [ ] All in-scope requirements are implemented.
- [ ] All required tests pass.
- [ ] Required documentation is updated.
- [ ] All UAT cases in `PHASE-UAT.md` pass or have accepted exceptions.
- [ ] No critical or high-severity defects remain open.
- [ ] `PHASE-STATUS.md` is updated to `completed`.
- [ ] `.planning/STATE.md` points to the next active phase.

---

## 11. Handoff Notes

<!-- Complete only when the phase is finished. -->

### What Is Now Available

-

### Important Constraints

-

### Deferred Work

-

### Recommended First Action for the Next Phase

-
