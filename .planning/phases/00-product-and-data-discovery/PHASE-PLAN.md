# Phase 00 Plan — Product and Data Discovery

> Purpose: Ground UrbanLens in one official dataset, an honest analyst workflow, and an explicit domain model before application code is written.

## Metadata

| Field | Value |
|---|---|
| Phase | `00` |
| Name | `Product and Data Discovery` |
| Status | `blocked` — repository work complete; API application submission unconfirmed |
| Owner | `Project owner` |
| Created | `2026-06-24` |
| Last Updated | `2026-06-24` |
| Target Milestone | `MVP discovery baseline` |
| Related ADRs | `docs/adr/001-use-postgis-for-spatial-queries.md` through `docs/adr/005-use-rust-actix-web-for-api.md` (to be written in this phase) |

---

## 1. Objective

### Problem

UrbanLens cannot design a trustworthy transaction map, metric model, or ingestion pipeline until it understands what one legally usable official source actually provides. Public transaction data may omit exact coordinates, use categorical ranges, change schemas, or represent an area rather than an identifiable property. Designing from assumptions would create false precision and brittle application contracts.

### Intended Outcome

Select and document one official historical transaction-price dataset; preserve a small representative fixture; profile its source schema and limitations; define the first analyst workflow, map representation, claim boundaries, and conceptual domain model; and record the first five architectural decisions.

### Why This Matters

This phase establishes what UrbanLens can truthfully show. It gives later ingestion, PostGIS, GraphQL, and frontend work a stable vocabulary and prevents the product from implying exact property identity or location where the source does not support either.

---

## 2. Scope

### In Scope

- [ ] Evaluate and select one official historical transaction-price dataset suitable for Tokyo analysis.
- [ ] Verify publisher, source URL, license or usage terms, access method, update frequency, credential requirements, and documented limitations.
- [ ] Request required API credentials immediately; record request status without storing secrets in the repository.
- [ ] Download a small representative source-derived fixture, or create a documented source-shaped fixture when redistribution is not permitted.
- [ ] Manually profile field names, types, nulls, categorical values, geographic granularity, identifiers, and schema inconsistencies.
- [ ] Define the first analyst workflow and its filter, metric, chart, selection, and provenance behavior.
- [ ] Define location-precision semantics, map representation rules, user-facing limitations, and product claim boundaries.
- [ ] Define the conceptual entities, relationships, lifecycle states, and required domain language.
- [ ] Write the five required ADRs and the three required product/data documents.

### Out of Scope

- [ ] Application scaffolding, production ingestion code, database migrations, GraphQL schema, or frontend implementation.
- [ ] Land-price, zoning, demographic, station/railway, listing, or private commercial datasets.
- [ ] Durable `property` identity or property-level location claims.
- [ ] Authentication, deployment, saved searches, prediction, recommendations, or multi-city support.
- [ ] Production-scale downloads or imports.

### Deferred Ideas

- [ ] Enrich observations with station, ward-boundary, zoning, demographic, or land-price data in later phases.
- [ ] Introduce a durable `property` entity only after a source demonstrates stable identity and defensible location linkage.
- [ ] Validate additional source versions and schema evolution after the first ingestion path is operational.

### Required Deliverables

| Path | Required Content |
|---|---|
| `docs/product-brief.md` | First analyst workflow, map content, filters/metrics, UI states, product claims, limitations, and non-goals. |
| `docs/data-sources.md` | Selected source metadata, official evidence, access/credential status, schema observations, limitations, and deferred alternatives. |
| `docs/data-model.md` | Conceptual entities, relationships, lineage, source mapping, validation, deduplication assumptions, and location-precision rules. |
| `docs/adr/001-use-postgis-for-spatial-queries.md` | Spatial storage/query decision and tradeoffs. |
| `docs/adr/002-use-graphql-for-product-api.md` | Product API boundary, bounded queries, and tradeoffs. |
| `docs/adr/003-preserve-raw-source-payloads.md` | Raw-payload retention, lineage, and reprocessing decision. |
| `docs/adr/004-model-location-precision-explicitly.md` | Precision values, propagation, map behavior, and consequences. |
| `docs/adr/005-use-rust-actix-web-for-api.md` | API stack decision and consequences. |
| `workers/importer/fixtures/transactions/` | Small representative fixture plus provenance, legal-use, selection, format, and integrity metadata. |

---

## 3. Requirements

### Functional Requirements

| ID | Requirement | Priority | Notes |
|---|---|---:|---|
| FR-01 | Select exactly one official historical transaction-price source for the MVP. | Must | Record why it was selected and why plausible alternatives were deferred. |
| FR-02 | Document source ownership, URLs, terms, retrieval method, credential needs, version, frequency, and limitations. | Must | Use `docs/data-sources.md`. |
| FR-03 | Request any required credentials as soon as the need is confirmed. | Must | Store only request status and setup instructions; never store a key or token. |
| FR-04 | Commit a small, representative, legally redistributable fixture and fixture metadata. | Must | Include normal, missing, approximate-location, and invalid/edge-shaped records when the source contains them. |
| FR-05 | Produce a manual source-schema profile and source-to-domain field mapping. | Must | Distinguish observed facts from proposed normalization. |
| FR-06 | Specify the first end-to-end analyst workflow. | Must | Area selection → filtering → metrics/trend → selection → provenance and limitations. |
| FR-07 | Decide what map marks or aggregates represent at every supported precision. | Must | Never imply a property coordinate when the source provides only station, district, or ward context. |
| FR-08 | Define explicit product claims and non-claims for the first release. | Must | Include metric wording, identity limitations, coverage, and completeness. |
| FR-09 | Write ADRs 001–005 using Context, Decision, Alternatives considered, Consequences, and Status. | Must | Decisions must agree with the discovery documents. |

### Non-Functional Requirements

| ID | Requirement | Priority | Verification Method |
|---|---|---:|---|
| NFR-01 | Every factual source claim is traceable to an official publisher page, specification, terms page, or downloaded artifact. | Must | Documentation review and link check. |
| NFR-02 | Fixture acquisition or construction is reproducible and legally defensible. | Must | Fixture README includes source, retrieval date, version/query, license, transformations, and checksum. |
| NFR-03 | No credential, `.env` file, large dataset, or personal/local download is committed. | Must | Repository scan and `git status` review. |
| NFR-04 | Location and metric language is conservative and understandable to an analyst. | Must | Claim-boundary review and UAT. |
| NFR-05 | Domain terms are used consistently across all Phase 0 documents. | Should | Terminology audit. |
| NFR-06 | Planning outputs are specific enough that Phase 1 can scaffold interfaces without reopening core product questions. | Should | Exit review against open questions. |

### Data / Domain Requirements

| ID | Requirement | Source / Assumption | Notes |
|---|---|---|---|
| DR-01 | Use the terms `data_source`, `dataset`, `import_run`, `raw_record`, `transaction_observation`, `area`, `station`, `market_metric`, `validation_issue`, and `location_precision`. | Phase requirement | Define each term in `docs/data-model.md`. |
| DR-02 | Do not create a durable `property` entity in the initial model. | Source identity is not yet proven stable. | Revisit through an ADR if future data supports it. |
| DR-03 | Model `location_precision` as `exact_point`, `nearest_station_point`, `district_centroid`, `ward_polygon`, or `unknown`. | Phase requirement | Define assignment evidence, map behavior, and disclaimer for every value. |
| DR-04 | Preserve lineage from a `transaction_observation` to its `raw_record`, `import_run`, `dataset`, and `data_source`. | Product trust principle | Include normalization-logic version and validation issues conceptually. |
| DR-05 | Preserve source values in `raw_record`; normalization must not invent replacements for missing or invalid values. | Ingestion policy | Prefer `null` plus a `validation_issue`. |
| DR-06 | Define import states and counts before implementation. | Repository rules | States: pending, running, completed, completed_with_warnings, failed; counts cover received, imported, updated, duplicates, rejected, warnings. |
| DR-07 | Define initial market metrics using observed transactions, explicit filters/time range, units, sample size, and limitations. | Product rules | Prefer median to mean for skewed price data. |
| DR-08 | Distinguish source schema, canonical concept, and UI label. | Discovery finding discipline | Avoid treating a source label as a durable domain model by default. |

---

## 4. Technical Design

### Proposed Approach

Begin with a source decision record rather than code. Assess candidate official transaction datasets against authority, legal use, Tokyo coverage, historical depth, access stability, geographic precision, schema clarity, identifiers, and fixture reproducibility. The leading candidate should be verified against current official documentation before selection.

After selection, preserve a minimal source-derived fixture under `workers/importer/fixtures/transactions/`. Add metadata explaining exactly how it was obtained or constructed and profile it manually. Use those observations to define the conceptual lineage model, validation boundaries, location-precision mapping, metrics, first workflow, and user-visible disclaimers. Complete the required ADRs last so their consequences reflect the actual source constraints rather than generic architecture preferences.

### Components Affected

| Component | Planned Change | Reason |
|---|---|---|
| `apps/web` | No code change; document future map/workflow states. | Phase 0 defines behavior before UI implementation. |
| `apps/api` | No code change; document future bounded query concepts. | Prevent premature GraphQL contracts. |
| `workers/importer` | Add a small source-derived fixture and fixture metadata only. | Make later parser and integration work reproducible. |
| `database` | Define conceptual entities and relationships only; no migration. | The source profile must precede physical schema design. |
| `infra` | No change. | Local services begin in the next phase. |
| `docs` | Add product brief, source register, data model, and ADRs 001–005. | These are the primary Phase 0 outputs. |

### Data Flow

```text
Official historical transaction-price source
  ↓
Small, documented source-derived fixture
  ↓
Manual schema, quality, identity, and location analysis
  ↓
Conceptual lineage model + location-precision and metric rules
  ↓
Analyst workflow, product claim boundary, and ADRs
```

### API / Interface Changes

| Type | Name | Change | Consumers |
|---|---|---|---|
| GraphQL Query | Conceptual only | Identify future needs for area/viewport observations, metrics, and provenance; do not commit a schema. | Phase 1+ API design |
| GraphQL Mutation | None | No mutation is required in discovery. | N/A |
| CLI Command | None | Record likely retrieval/import needs without implementing them. | Phase 2+ importer |
| Database Migration | None | Document the conceptual model only. | Phase 1+ database design |
| Environment Variable | Source credential placeholder, if required | Add only a proposed variable name to setup documentation; do not create or commit a secret. | Future importer |

### Data Model Changes

| Entity / Table | Change | Migration Required | Notes |
|---|---|---:|---|
| `data_source` / `dataset` | Define publisher-level source separately from a versioned/retrievable dataset. | No | Prevent license metadata and dataset versions from being conflated. |
| `import_run` / `raw_record` | Define acquisition history and immutable source payload lineage. | No | Include statuses, counts, hash, and validation state conceptually. |
| `transaction_observation` | Define an observed historical transaction, not a listing or durable property. | No | Link to the originating raw record. |
| `area` / `station` | Define geographic context concepts only where supported by source evidence. | No | Do not add external enrichment in this phase. |
| `market_metric` | Define reproducible aggregate semantics and required context. | No | Include unit, time range, filters, sample size, and limitations. |
| `validation_issue` | Define record-level warnings/rejections without overwriting raw facts. | No | Severity and affected field should remain explainable. |

### Geographic / Data Precision Notes

- Location precision: every mappable observation must have one explicit enum value backed by source evidence or a documented transformation.
- `exact_point`: allowed only when the source supplies or legally supports an exact observation coordinate; never infer it from an address label alone.
- `nearest_station_point`: marker represents station context associated with the observation, not the transacted property.
- `district_centroid`: marker represents an area centroid; colocated observations must be aggregated or visually disclosed as approximate.
- `ward_polygon`: use polygon selection or ward-level aggregation rather than a property-like point.
- `unknown`: do not place a point marker; retain the observation for non-spatial analysis when otherwise valid.
- User-facing disclaimer: “Map locations may represent a station, district, or ward context rather than the exact transacted property. Open an observation to see its location precision and source.”
- Known data limitations to verify: geographic generalization, field suppression, categorical ranges, missing values, update lag, source revisions, duplicates, and incomplete market coverage.
- Assumption: early public transaction records are observations from an official dataset, not a complete market census and not proof of stable property identity.

---

## 5. Implementation Slices

### Slice 1 — Select the Source and Secure Access

**Goal**

Choose exactly one legally usable official historical transaction-price dataset and remove access uncertainty early.

**Tasks**

- [ ] Create a short candidate matrix covering publisher authority, Tokyo/history coverage, license, access, credentials, schema documentation, location granularity, stable identifiers, update frequency, and reproducible fixtures.
- [ ] Verify all material facts against current official pages or source artifacts.
- [ ] Select one source, document the decision and deferred alternatives in `docs/data-sources.md`.
- [ ] Request credentials immediately if required and record request date/status and local variable name without storing the credential.
- [ ] Record retrieval date, dataset/API version, endpoint or download workflow, rate limits, and terms.

**Expected Evidence**

- [ ] One source is marked `selected for MVP` with complete metadata and official references.
- [ ] Credential status is `not required`, `requested`, or `available locally`; it is not ambiguous.

---

### Slice 2 — Preserve and Profile a Representative Fixture

**Goal**

Create a small legal fixture that exposes the real source shape and its difficult cases.

**Tasks**

- [ ] Retrieve a bounded sample for Tokyo using a recorded query/download procedure.
- [ ] Confirm redistribution terms; if raw redistribution is restricted, create a minimal source-shaped fixture and document every synthetic transformation.
- [ ] Store the sample and metadata under `workers/importer/fixtures/transactions/` rather than the repository root.
- [ ] Record checksum, encoding, format, retrieval date, version/query, row/record count, and selection method.
- [ ] Profile fields, types, nullability, categorical values, date/price/area units, identifiers, geographic fields, and edge cases.
- [ ] Document observed schema issues without silently repairing the fixture.

**Expected Evidence**

- [ ] The fixture parses with an appropriate standard parser and is small enough for source control/tests.
- [ ] A reviewer can reproduce or explain every fixture record and distinguish source values from annotations.

---

### Slice 3 — Define the Analyst Workflow and Claim Boundary

**Goal**

Specify exactly what the first product view helps an analyst do and what it refuses to imply.

**Tasks**

- [ ] Write the first workflow in `docs/product-brief.md`, including entry state, area/viewport selection, filters, summary metrics, trend chart, point/aggregate selection, provenance, empty/loading/error states, and shareable URL state.
- [ ] Decide the first map representation for each supported `location_precision` value.
- [ ] Define initial metric names, units, calculation intent, sample-size display, time context, and limitation copy.
- [ ] Document product claims and non-claims, including incomplete public-data coverage and approximate geography.
- [ ] Confirm that the workflow remains useful without station enrichment, exact properties, listings, or additional datasets.

**Expected Evidence**

- [ ] The workflow can be narrated from area selection through provenance without an unresolved product step.
- [ ] Every map mark, metric, and detail value has a defined meaning and honest limitation.

---

### Slice 4 — Define the Conceptual Domain and Lineage Model

**Goal**

Create a source-grounded language and relationship model for later schema and API design.

**Tasks**

- [ ] Define every required domain term and its boundaries in `docs/data-model.md`.
- [ ] Draw the conceptual relationship path from source/dataset through import/raw record to observation, validation issues, geography, and metrics.
- [ ] Add a source-to-domain mapping table with transformation and uncertainty notes.
- [ ] Define `location_precision` assignment, fallbacks, prohibited inferences, and map/API implications.
- [ ] Define deduplication inputs, import lifecycle/counts, null/validation behavior, and normalization versioning conceptually.
- [ ] Explicitly record why `property` is absent.

**Expected Evidence**

- [ ] Every user-visible observation can conceptually trace back to source and raw record.
- [ ] No durable field or entity depends on unverified source identity or location assumptions.

---

### Slice 5 — Record Architecture Decisions and Close Discovery

**Goal**

Turn the source-grounded product constraints into reviewable architectural decisions and pass the exit gate.

**Tasks**

- [ ] Write ADR-001 for PostGIS spatial queries.
- [ ] Write ADR-002 for the GraphQL product API.
- [ ] Write ADR-003 for preserving raw source payloads.
- [ ] Write ADR-004 for explicit location precision.
- [ ] Write ADR-005 for Rust, Actix Web, async-graphql, and SQLx at the API boundary.
- [ ] Cross-check ADR consequences against the product brief, source profile, and data model.
- [ ] Resolve or explicitly defer all open questions that would block the next phase.
- [ ] Execute `PHASE-UAT.md` and update phase/project status.

**Expected Evidence**

- [ ] All required deliverables exist and contain no unresolved placeholders.
- [ ] UAT passes with no critical/high defect or undocumented exception.

---

## 6. Testing Strategy

### Unit Tests

| Area | Required Coverage |
|---|---|
| Fixture syntax | Parse the committed CSV/JSON/other source format with a standard parser; verify expected bounded record count and encoding. |
| Fixture integrity | Verify the recorded checksum and required representative cases/fields. |
| Documentation structure | Check required files, ADR headings, links, and absence of template placeholders. |
| Vocabulary | Search for prohibited durable `property` modeling and inconsistent core terms. |

### Integration Tests

| Flow | Expected Result |
|---|---|
| Source evidence → fixture metadata → schema mapping | A reviewer can trace the fixture to the selected official source and understand any transformations. |
| Raw source field → conceptual observation → workflow value | Each planned user-visible value has source evidence, normalization intent, unit, quality/precision behavior, and provenance. |
| Location precision → map representation → disclaimer | No supported precision is rendered or worded as more exact than its evidence. |

### Manual Validation

| Scenario | Why Manual Validation Is Needed |
|---|---|
| Official terms and access review | Legal wording, API access, and redistribution constraints require human interpretation and current official evidence. |
| Schema inspection | Inconsistent labels, suppressed values, geographic meaning, and categorical ranges require domain judgment. |
| Workflow walkthrough | Analyst usefulness and misleading claims cannot be proven by syntax checks alone. |
| ADR consistency review | Tradeoffs and consequences must reflect this source and MVP rather than generic templates. |

### Regression Risks

| Risk Area | Possible Regression | Mitigation |
|---|---|---|
| Source facts | URLs, access rules, schema, or terms become stale before implementation. | Record verification/retrieval date and recheck before building the importer. |
| Location claims | Later code treats approximate context as an exact property point. | Make precision non-optional conceptually and trace it through ADR, model, API, and UI requirements. |
| Domain drift | Later phases reintroduce `property` or source-specific labels without evidence. | Treat `docs/data-model.md` as the vocabulary baseline and require an ADR for material changes. |
| Fixture bias | Sample omits difficult records and makes parsing appear easier. | Select representative missing, approximate, categorical, and invalid/edge cases deliberately. |

---

## 7. Acceptance Criteria

### Product / User Criteria

- [ ] The first analyst workflow is written from area selection through provenance and accuracy limitations.
- [ ] Initial filters, metrics, trend chart, selection behavior, and empty/loading/error expectations are defined.
- [ ] The first map representation is explicit for each supported location precision.
- [ ] Product claims and non-claims clearly distinguish observed public transaction indicators from a complete market or exact property map.

### Engineering Criteria

- [ ] Exactly one official source is selected with current evidence, legal/access metadata, and alternative rationale.
- [ ] Credential needs are resolved to a clear status without committing secrets.
- [ ] A representative, bounded, documented fixture exists under `workers/importer/fixtures/transactions/`.
- [ ] The source schema and conceptual source-to-domain mapping are documented.
- [ ] The conceptual model preserves lineage and validation issues and does not include a durable `property` entity.
- [ ] All five location-precision values have assignment and behavior rules.
- [ ] ADRs 001–005 exist and are internally consistent.

### Documentation Criteria

- [ ] `docs/product-brief.md` defines the user, workflow, map, metrics, states, claim boundary, and non-goals.
- [ ] `docs/data-sources.md` contains all required source metadata and limitations.
- [ ] `docs/data-model.md` defines entities, relationships, source mapping, lineage, deduplication assumptions, and geographic strategy.
- [ ] ADRs use the required structure and have a clear status.
- [ ] Fixture metadata includes provenance, terms, retrieval/construction method, and integrity information.

### UAT Criteria

- [ ] Every UAT case passes or has an explicitly accepted, non-critical exception with a follow-up owner/phase.
- [ ] No unresolved question can materially change the first source, workflow, map meaning, domain identity, or claim boundary.
- [ ] A reviewer can explain exactly what UrbanLens will and will not claim from the selected data.

---

## 8. Dependencies, Risks, and Open Questions

### Dependencies

| Dependency | Owner / Source | Status | Impact if Missing |
|---|---|---|---|
| Current official source documentation and terms | Dataset publisher | To verify | Cannot select or document the first source defensibly. |
| Dataset/API access and credentials, if required | Dataset publisher / project owner | Unknown | May prevent representative fixture retrieval. |
| Legally redistributable bounded sample or permission to create source-shaped fixture | Dataset terms | To verify | Cannot commit a reliable fixture until resolved. |
| Tokyo coverage and geographic semantics | Source specification and sample | To verify | Cannot define map behavior or precision assignments. |

### Risks

| Risk | Likelihood | Impact | Mitigation |
|---|---:|---:|---|
| Credentials or access approval take longer than the phase. | Medium | High | Request immediately; use official downloadable artifacts or a documented source-shaped fixture only if terms permit and record the dependency. |
| The source does not support property-level coordinates or stable identity. | High | Medium | Model observations, not properties; require explicit precision and aggregate/approximate map behavior. |
| Published terms do not permit redistribution of raw samples. | Medium | Medium | Commit minimal derived/source-shaped fixture data with documented transformations rather than redistributed payloads. |
| The sample hides schema variants or invalid records. | Medium | High | Use documented stratified selection and record known unrepresented cases. |
| Metric fields are categorical or suppressed rather than numeric. | Medium | High | Do not invent values; narrow metrics/workflow and document unavailable calculations. |
| Source access or schema changes after discovery. | Medium | Medium | Record dates/versions and make Phase 1 re-verification an explicit prerequisite. |

### Open Questions

- [ ] Which official transaction dataset best satisfies legal use, Tokyo coverage, historical depth, schema clarity, and fixture reproducibility?
- [ ] Does access require credentials, and may a small raw sample be committed?
- [ ] What geographic evidence does each record actually contain, and which precision values are supportable without enrichment?
- [ ] Which price, floor-area, date, asset/use, and station-distance fields are numeric versus categorical/ranged/suppressed?
- [ ] Is any external identifier stable across downloads or revisions, or must deduplication rely on a deterministic normalized content hash?
- [ ] What minimum sample-size rule, if any, should suppress or qualify a metric in the first workflow?

---

## 9. Planning Decisions

| Decision | Rationale | ADR Required? |
|---|---|---:|
| Phase 0 selects one historical transaction-price source only. | A narrow source boundary makes claims and ingestion behavior testable. | No |
| The core record is a `transaction_observation`, not a `property`. | Stable property identity and exact location have not been established. | No; document in data model |
| Location precision is explicit and controls map behavior. | Approximate public data must not appear as exact property geography. | Yes — ADR-004 |
| Raw source payloads remain distinct from normalized records. | Enables audit, reprocessing, and reproducibility. | Yes — ADR-003 |
| Spatial filtering belongs in PostGIS. | Later viewport, distance, polygon, and aggregation queries need indexed spatial operations. | Yes — ADR-001 |
| Product-facing access uses bounded GraphQL queries. | Supports map, metric, comparison, and provenance compositions without exposing unbounded data. | Yes — ADR-002 |
| The API stack is Rust + Actix Web + async-graphql + SQLx. | Matches the intended production portfolio architecture and explicit SQL/spatial control. | Yes — ADR-005 |
| Phase 0 changes documentation and fixtures, not application code or physical schemas. | Discovery evidence should drive later implementation contracts. | No |

---

## 10. Completion Definition

This phase is complete when:

- [x] One source is selected, verified, and fully documented.
- [ ] Credential/access status is clear and secrets remain outside version control.
- [x] A small representative fixture exists with reproducible provenance and legal-use notes.
- [x] The first user workflow, map behavior, geographic accuracy rules, and product claim boundary are documented.
- [x] The first conceptual domain model and source mapping are agreed, including the absence of a durable `property`.
- [x] All five required ADRs exist and agree with discovery evidence.
- [x] Required artifact, integrity, terminology, and manual reviews pass.
- [ ] All UAT cases pass or have accepted exceptions.
- [x] No critical or high-severity product/data defects remain open.
- [ ] `PHASE-STATUS.md` is updated to `completed` or `completed_with_exceptions`.
- [ ] `.planning/STATE.md` points to the next active phase.

---

## 11. Handoff Notes

The repository-controlled handoff is complete. Final phase closure awaits user confirmation that the MLIT API application was submitted.

### What Is Now Available

- MLIT `不動産取引価格情報` is the sole selected dataset; `成約価格情報` is excluded.
- Three unmodified CP932 CSV fixtures contain 666 observations across Chuo, Shinagawa, and Shibuya for 2024 Q4, with checksums and acquisition metadata.
- The product brief defines station-context aggregates, type-faceted metrics, URL-backed filters, UI states, provenance, and explicit claims/non-claims.
- The conceptual model defines lineage, exact-artifact idempotency, validation, location precision, metric eligibility, and the absence of durable property identity.
- ADRs 001–005 are accepted.

### Important Constraints

- The map must communicate observation location precision and must not imply exact transacted-property coordinates without source evidence.
- The first release describes public-data transaction observations and indicators, not complete market coverage or appraised market price.
- A durable `property` entity remains prohibited until supported by stable source identity and geography.
- CSV/XIT001 observations remain spatially `unknown` unless a defensible source link to geometry exists; do not guess joins to XPT001.
- Source-provided ¥/m² is sparse and eligible only where MLIT supplies it.

### Deferred Work

- Application scaffold, physical database schema, importer, GraphQL API, and analyst UI.
- Additional datasets and geographic enrichment.
- Authenticated XIT001/XPT001 fixture capture after the user receives an API key.

### Recommended First Action for the Next Phase

- After Phase 0 is closed, re-verify source access/schema dates and scaffold the smallest local web/API/PostGIS platform around the agreed conceptual boundaries.
