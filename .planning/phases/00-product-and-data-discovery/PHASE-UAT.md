# Phase 00 UAT — Product and Data Discovery

> Purpose: Verify that UrbanLens has a source-grounded, internally consistent, and honest product/data specification before application code begins.

## Metadata

| Field | Value |
|---|---|
| Phase | `00` |
| Name | `Product and Data Discovery` |
| UAT Status | `blocked` |
| Environment | `local repository and official MLIT documentation/download service` |
| Tester | `Codex` |
| Started | `2026-06-24` |
| Completed | `not_completed` |
| Build / Commit | `N/A — repository has no initial commit` |
| Related Plan | `PHASE-PLAN.md` |
| Related Status | `PHASE-STATUS.md` |

## 1. UAT Objective

Verify that a reviewer can identify the selected official source, reproduce and validate the fixtures, follow planned user-visible facts to source evidence, understand the analyst workflow and map semantics, and state exactly what UrbanLens will and will not claim.

The repository-controlled work passes. Final acceptance is blocked because MLIT API application submission requires the user’s identity, agreement, and attestations and has not been confirmed.

## 2. Preconditions

| Precondition | Result | Evidence |
|---|---|---|
| Required product/data documents exist | Pass | `docs/product-brief.md`, `docs/data-sources.md`, `docs/data-model.md` |
| ADRs 001–005 exist | Pass | `docs/adr/001-*.md` through `005-*.md` |
| Three source CSV fixtures exist | Pass | `workers/importer/fixtures/transactions/` |
| Fixture metadata and checksums exist | Pass | Fixture `README.md` and `SHA256SUMS` |
| Official source evidence is current | Pass | MLIT pages verified and fixtures retrieved `2026-06-24` |
| Credential application submitted | Blocked | User action required at <https://www.reinfolib.mlit.go.jp/api/request/> |
| No credential committed | Pass | Secret scan returned no match; `.env.example` contains an empty value |

### Known Limitations

- Phase 0 has no database, API, importer, or UI; UAT validates decisions and artifacts.
- Fixture coverage is three Tokyo wards for 2024 Q4 and cannot prove every historical schema variant.
- API approval may remain pending after application submission; submission confirmation is the only Phase 0 blocker.

## 3. Acceptance-Criteria Traceability

| UAT ID | Scenario | Required Result | Status |
|---|---|---|---|
| UAT-01 | Source and access decision | One official source, complete terms/access evidence, API request submitted | `blocked` |
| UAT-02 | Fixture provenance and schema profile | Legal, intact, representative, traceable source fixtures | `passed` |
| UAT-03 | First analyst workflow | Complete workflow from area/filter through provenance and states | `passed` |
| UAT-04 | Location precision and map honesty | No false exact point; all precision behaviors defined | `passed` |
| UAT-05 | Domain and lineage model | Complete lineage, consistent terms, no durable property | `passed` |
| UAT-06 | ADR review | ADRs 001–005 complete and mutually consistent | `passed` |
| UAT-07 | Product claim boundary | Supported claims and prohibited claims are explicit | `passed` |
| UAT-08 | Cross-document readiness | No remaining phase blocker | `blocked` |

## 4. UAT Results

### UAT-01 — Selected Source and Access Path

**Actual Result**

- Selected exactly one dataset: MLIT `不動産取引価格情報`.
- Excluded `成約価格情報` and documented alternatives.
- Verified publisher, terms, Public Data License 1.0 attribution rules, quarterly update model, CSV, XIT001, and XPT001.
- Verified the CSV download path by retrieving all three fixtures.
- Added empty `MLIT_REINFOLIB_API_KEY` configuration and documented key handling.
- API application submission cannot be performed by the repository agent because it requires the user’s personal identity and attestations.

**Status:** `blocked`

**Evidence:** `docs/data-sources.md`, `.env.example`, official links recorded there.

### UAT-02 — Fixture Provenance, Integrity, and Coverage

**Actual Result**

- Retrieved unmodified CP932 CSV exports for Chuo, Shinagawa, and Shibuya, 2024 Q4, transaction-price information only.
- Parsed 176, 313, and 177 records respectively; each has 30 columns.
- All 666 rows have the expected ward, quarter, price category, station name/distance, price, and area.
- Each fixture includes land, land + building, and used-condominium observations.
- Checksums pass; no source values or line endings were rewritten.

**Status:** `passed`

**Evidence:** fixture `README.md`, `SHA256SUMS`, parser output, and checksum output.

### UAT-03 — First Analyst Workflow

**Actual Result**

The product brief defines area/viewport selection, URL-backed filters, station-context aggregates, type-faceted metrics, quarterly trend, observation detail, provenance, loading, empty, warning, failure, small-sample, and unknown-location states. Unsupported listing/valuation/property workflows are explicitly excluded.

**Status:** `passed`

**Evidence:** `docs/product-brief.md`.

### UAT-04 — Location Precision and Map Representation

**Actual Result**

All five precision values have evidence and behavior rules. `exact_point` is unsupported. XPT001 alone supports `nearest_station_point`; CSV/XIT records without a defensible geometry link remain `unknown`. Ward aggregates use polygons, and district centroids remain deferred pending an authoritative source. The product disclaimer is implementation-ready.

**Status:** `passed`

**Evidence:** product brief, data model, ADR-004, XPT001 documentation.

### UAT-05 — Domain and Lineage Model

**Actual Result**

The model traces `data_source → dataset → import_run → raw_record → transaction_observation`, retains rejected records and validation issues, defines reproducible metric context, and explicitly avoids durable property identity. Exact-artifact idempotency retains legitimate identical rows by using artifact identity plus row ordinal rather than payload hash alone.

**Status:** `passed`

**Evidence:** `docs/data-model.md`.

### UAT-06 — ADR Completeness and Consistency

**Actual Result**

All five ADRs have Status, Context, Decision, Alternatives Considered, and Consequences. They agree on PostGIS spatial execution, bounded GraphQL, raw preservation, explicit precision, and the Rust/Actix/async-graphql/SQLx stack.

**Status:** `passed`

**Evidence:** ADR files; automated heading scan passed.

### UAT-07 — Product Claim Boundary

**Actual Result**

The product may claim official historical observations, reproducible indicators, and explicit station/aggregate context. It rejects complete-market, market-price, listing, appraisal, stable-property, exact-location, causal, and investment claims. Metrics require units, filters, period, source, eligible `n`, and limitations.

**Status:** `passed`

**Evidence:** `docs/product-brief.md`, `docs/data-sources.md`.

### UAT-08 — Cross-Document Readiness

**Actual Result**

- Required artifacts exist and contain no template placeholders.
- Terms, interface, domain, map, metric, and ADR decisions are internally consistent.
- No secret or source key is committed.
- API application submission remains unconfirmed, so the exit gate and move to Phase 01 cannot be approved.

**Status:** `blocked`

**Evidence:** validation commands below and `PHASE-STATUS.md` blocker `BLK-01`.

## 5. Failure and Edge-Case Validation

| Scenario | Expected Behavior | Result | Status |
|---|---|---|---|
| API approval pending after submission | CSV path keeps discovery usable; status remains explicit | Design supports this | `passed` |
| API application not submitted | Phase cannot complete | Current blocker | `blocked` |
| Raw sample redistribution | Attribute under PDL1.0; identify processing | Metadata and attribution recorded | `passed` |
| Missing numeric/source value | Raw preserved; canonical null plus issue | Model rule defined | `passed` |
| Approximate station geometry | Aggregate/labeled station context only | Product/model/ADR agree | `passed` |
| Unknown geometry | No invented point; retain for metrics | Product/model agree | `passed` |
| Unsupported universal ¥/m² | Use publisher-populated eligible values only | 599 blank fixture values documented | `passed` |
| Historical schema variant absent | List fixture boundary; validate later | Limitation recorded | `passed` |
| Misleading property/market language | Prohibited or qualified | Claim boundary recorded | `passed` |

## 6. Data Integrity Validation

| Check | Expected Result | Actual Result | Status |
|---|---|---|---|
| Source lineage | Observation concept links to raw/import/dataset/source | Complete conceptual path | `passed` |
| Raw preservation | Source values/artifact remain independent | Required by model and ADR-003 | `passed` |
| Fixture integrity | Checksums, format, encoding, counts match | All three checksums pass | `passed` |
| Idempotency | Same artifact rerun duplicates nothing; identical rows retained | Artifact checksum + row ordinal rule | `passed` |
| Validation visibility | Missing/invalid values create issues, not defaults | Defined | `passed` |
| Location precision | Evidence and map behavior explicit | Defined for all values | `passed` |
| Metric reproducibility | Unit, filters, period, source, eligible `n`, limitations | Defined | `passed` |
| Source/schema versioning | Retrieval/query/version risk recorded | Defined in fixture/source docs | `passed` |

## 7. Automated Validation Evidence

```text
shasum -a 256 -c SHA256SUMS
  mlit-reinfolib-chuo-2024-q4.csv: OK
  mlit-reinfolib-shibuya-2024-q4.csv: OK
  mlit-reinfolib-shinagawa-2024-q4.csv: OK

CSV assertions
  PASS chuo records=176 columns=30
  PASS shibuya records=177 columns=30
  PASS shinagawa records=313 columns=30
  All rows: transaction-price information, 2024 Q4, expected ward
  All fixtures: land, land + building, used condominium

Repository scans
  Required ADR headings: PASS
  Template placeholders in deliverables: none
  Committed MLIT/private key values: none
```

## 8. Defects and Blockers

| ID | Type | Severity | Description | Owner | Status |
|---|---|---|---|---|---|
| BLK-01 | External prerequisite | High | MLIT API application submission has not been confirmed. | User | Open |

No critical or high product/data defect was found in the implemented artifacts.

## 9. UAT Summary

| Metric | Count |
|---|---:|
| Total UAT Cases | 8 |
| Passed | 6 |
| Failed | 0 |
| Blocked | 2 |
| Not Run | 0 |
| Open Critical Defects | 0 |
| Open High Defects | 0 |
| Open External Blockers | 1 |

### Final UAT Decision

- [ ] `passed`
- [ ] `passed_with_accepted_exceptions`
- [ ] `failed`
- [x] `blocked` — repository work passes, but required API application submission is unconfirmed.

## 10. Next Action

1. The user submits <https://www.reinfolib.mlit.go.jp/api/request/> and confirms the submission date.
2. Record status as `requested—approval pending` in `docs/data-sources.md`.
3. Re-run UAT-01 and UAT-08; approval receipt/API key is not required.
4. If they pass, mark UAT `passed`, complete Phase 0, and activate Phase 01.
