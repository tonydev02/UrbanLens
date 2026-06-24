# AGENTS.md

## Project: UrbanLens

UrbanLens is a public-data commercial real-estate intelligence platform for Tokyo.

It ingests fragmented official datasets, normalizes them into a canonical spatial data model, and provides map-based search, area comparison, market metrics, and transparent data provenance.

The project is intentionally focused on the difficult parts of real-estate data products:

* Ingesting inconsistent external datasets reliably
* Preserving source lineage and transformation history
* Supporting geographic and spatial queries
* Making imperfect public data useful for analysts
* Building trustworthy market metrics rather than generic dashboards

This repository should be treated as a production-oriented portfolio project, not a tutorial app.

---

# Product Principles

## Build analyst tools, not a property marketplace

UrbanLens is not a listing website.

Do not optimize for:

* Apartment browsing
* Individual consumer recommendations
* Lead generation
* Property booking
* Payments
* Brokerage workflows

Optimize for:

* Market research
* Area comparison
* Transaction analysis
* Data quality visibility
* Geographic exploration
* Decision support for commercial real-estate analysts

Every major feature should answer one question:

> Does this make fragmented real-estate information easier to trust, compare, or analyze?

If not, it is probably out of scope.

---

# Target Users

Primary users:

* Commercial real-estate analysts
* Asset managers
* Developers
* Brokers
* Urban-planning researchers
* Internal data teams

Example user questions:

* Which wards have experienced the strongest increase in transaction price per square meter?
* How does Shinagawa compare with Shibuya for office-related transactions?
* Which areas have a high volume of large-floor-area transactions near major stations?
* What data sources support this market metric?
* How fresh and complete is the data shown on this screen?

---

# Technology Stack

## Frontend

* Next.js
* React
* TypeScript
* GraphQL client
* MapLibre GL or Leaflet
* TanStack Query where appropriate
* Recharts or ECharts for charts

## Backend

* Rust
* Actix Web
* async-graphql
* SQLx
* PostgreSQL
* PostGIS
* Redis

## Infrastructure

* Docker Compose for local development
* GitHub Actions for CI
* PostgreSQL migrations committed to version control
* Environment variables managed through `.env.example`
* OpenTelemetry-compatible structured logging where practical

## Optional Supporting Tools

* Python for exploratory analysis, data profiling, or one-off validation scripts
* Grafana for local metrics visualization
* Prometheus-compatible metrics endpoint
* Sentry or equivalent error tracking in later phases

Do not introduce a new major framework or service without a clear reason and an Architecture Decision Record.

---

# Repository Structure

```text
urbanlens/
├── apps/
│   ├── web/                    # Next.js frontend
│   └── api/                    # Rust Actix Web + GraphQL API
├── workers/
│   └── importer/               # Dataset ingestion and normalization jobs
├── packages/
│   └── shared-types/           # Generated GraphQL / TypeScript types
├── infra/
│   ├── docker-compose.yml
│   ├── postgres/
│   └── grafana/
├── docs/
│   ├── architecture.md
│   ├── data-model.md
│   ├── data-sources.md
│   ├── local-development.md
│   ├── adr/
│   └── screenshots/
├── scripts/
│   ├── seed-dev-data.sh
│   ├── import-mlit.sh
│   └── validate-data.sh
├── .github/
│   └── workflows/
├── AGENTS.md
├── README.md
└── .env.example
```

Keep frontend, API, workers, infrastructure, and documentation clearly separated.

Do not put temporary scripts, generated files, downloaded datasets, or secrets into the repository root.

---

# Core Architecture

```text
Official Public Data Sources
        │
        ▼
Raw Dataset Retrieval
        │
        ▼
Raw Payload Storage
        │
        ▼
Validation + Normalization
        │
        ▼
PostgreSQL + PostGIS Canonical Model
        │
        ▼
GraphQL API
        │
        ▼
Next.js Analyst Workspace
```

The frontend must not depend directly on external public-data APIs.

All external data must go through the ingestion pipeline.

This allows the system to provide:

* Consistent schemas
* Data quality validation
* Import history
* Source lineage
* Reproducible metrics
* Faster search and map queries
* Better error handling

---

# Data Sources

Use official, public, legally usable datasets only.

Preferred data categories:

* Real-estate transaction data
* Official land-price data
* Station and railway data
* Ward and district boundaries
* Urban planning or land-use data
* Public demographic or economic context data

Potential source types:

* MLIT real-estate transaction datasets
* MLIT land-price datasets
* Tokyo Metropolitan Government Open Data
* e-Stat datasets
* Public railway or geographic boundary datasets

Never scrape:

* Suumo
* AtHome
* Real estate brokerage websites
* Private property databases
* Login-protected sites
* Sources that prohibit automated extraction

Every data source must be documented in `docs/data-sources.md`.

For each source, record:

```text
- Source name
- Publisher
- Original source URL
- License or usage terms
- Retrieval method
- Retrieval date
- Dataset version, if available
- Update frequency
- Important limitations
- Known schema issues
```

---

# Data Lineage Requirements

Data provenance is a first-class feature.

Every normalized record shown to users must be traceable to:

1. The source dataset
2. The import run
3. The original raw record
4. The normalization logic version
5. Any validation warnings or transformations applied

At minimum, maintain these conceptual entities:

```text
data_sources
import_runs
raw_records
properties
transactions
areas
area_metrics
validation_issues
```

Recommended fields:

```text
data_sources
- id
- name
- publisher
- source_url
- license_url
- retrieved_at
- dataset_version

import_runs
- id
- source_id
- started_at
- completed_at
- status
- records_received
- records_imported
- records_rejected

raw_records
- id
- import_run_id
- external_id
- payload_json
- payload_hash
- validation_status
- validation_errors
- created_at
```

Never discard the raw payload after normalization.

Normalized records should reference their originating raw record whenever possible.

---

# Ingestion Rules

All ingestion jobs must be:

* Idempotent
* Retry-safe
* Observable
* Deterministic where possible
* Safe to rerun after failure
* Explicit about partial failures

## Idempotency

Use stable external identifiers where available.

When no stable identifier exists, use a deterministic content hash based on normalized source fields.

Do not create duplicate records when the same dataset is imported twice.

## Validation

Validate external data before writing normalized records.

Examples:

* Transaction price must be positive.
* Floor area must be non-negative.
* Coordinates must be within expected geographic bounds.
* Dates must be parseable.
* Station walking time should be within a reasonable range.
* Currency must be explicitly identified.
* Unknown asset types must be preserved but flagged.

Do not silently convert invalid values into fake defaults.

Prefer:

```text
null + validation warning
```

over:

```text
invented or misleading value
```

## Import Status

Every import run should end in one of:

```text
pending
running
completed
completed_with_warnings
failed
```

Record counts for:

* Received records
* Imported records
* Updated records
* Duplicates skipped
* Rejected records
* Warning records

---

# Geographic Data Rules

Use PostGIS for all spatial storage and queries.

Prefer geographic types and proper indexes.

Examples:

```sql
geometry(Point, 4326)
geometry(Polygon, 4326)
geography(Point, 4326)
```

Use spatial indexes for map and proximity queries.

Examples of expected query capabilities:

* Transactions inside a ward boundary
* Transactions within a map viewport
* Properties within a distance from a station
* Aggregation by ward or district
* Spatial joins between transactions and zoning boundaries

Avoid doing large geographic filtering in application memory.

Spatial filtering belongs in PostgreSQL/PostGIS.

---

# Market Metric Rules

Metrics must be transparent, reproducible, and conservative.

Do not present a metric without documenting:

* Its source data
* Calculation method
* Time period
* Filters applied
* Sample size
* Important limitations

Examples of useful metrics:

* Transaction count
* Median transaction price
* Median price per square meter
* Median floor area
* Median station walking time
* Year-over-year change
* Price distribution
* Data completeness score

Prefer median over average for skewed price data.

Always show sample size beside market metrics where possible.

Example:

```text
Median transaction price per m²:
¥1,240,000

Based on:
83 transactions
January 2024 – December 2025
```

Never describe a metric as a “market price” if it is calculated from incomplete or biased public data.

Use wording such as:

```text
Observed transaction median
Public-data market indicator
Historical transaction metric
```

---

# Data Quality Rules

The UI should make data limitations visible rather than hiding them.

Each normalized record should have a data-quality state.

Suggested levels:

```text
high
medium
low
unknown
```

Possible quality signals:

* Presence of geographic coordinates
* Geographic precision level
* Completeness of price and floor area
* Presence of station information
* Source freshness
* Validation warnings
* Duplicate confidence
* Classification confidence

Do not expose a fake single “AI confidence score” without explaining how it was calculated.

If a score is shown, make the factors visible.

---

# API Rules

Use GraphQL for product-facing queries.

GraphQL should support:

* Map viewport search
* Area summaries
* Area comparison
* Transaction filters
* Data provenance details
* Import-run status
* Saved searches
* Pagination
* Sorting
* Explicit filtering

Avoid GraphQL queries that return unbounded datasets.

Every list query must support pagination or a strict limit.

Example query domains:

```text
areas
transactions
properties
marketMetrics
dataSources
importRuns
savedSearches
```

Do not expose raw payload JSON by default.

Raw data should only be available through a clearly intentional provenance or admin-oriented query.

---

# Frontend Rules

The frontend is an analyst workspace, not a marketing site.

Prioritize:

* Dense but readable data presentation
* Fast filtering
* Shareable URLs
* Map interaction
* Transparent metric definitions
* Clear loading and error states
* Mobile tolerance, but desktop-first layout

Important screens:

1. Market map
2. Area comparison
3. Transaction explorer
4. Transaction or property detail
5. Data provenance drawer
6. Import-run status page
7. Saved searches or watchlists

All filters should be reflected in URL query parameters when practical.

A user should be able to copy a URL and share the exact market view with another analyst.

---

# UI Requirements

Every chart must include:

* Clear title
* Unit labels
* Time range
* Sample size where relevant
* Empty state
* Loading state
* Error state
* Tooltip or explanation for metric definitions

Every map layer must include:

* Layer name
* Data source
* Data freshness
* Visibility toggle
* Legend where appropriate

Every user-visible data point should have a clear unit.

Examples:

```text
¥
¥/m²
m²
minutes
transactions
percent
```

Do not use ambiguous labels such as:

```text
Price
Value
Score
Growth
```

without context.

---

# Error Handling

Expected failures must be handled intentionally.

Examples:

* Source API unavailable
* Dataset schema changed
* Rate limit exceeded
* Partial import failure
* Invalid record data
* PostGIS query timeout
* Empty map viewport
* No transactions matching filters

Do not swallow errors.

Use structured logging with useful fields:

```text
request_id
import_run_id
source_id
external_record_id
area_id
user_filter_context
error_kind
```

User-facing errors should be readable and actionable.

Bad:

```text
Internal Server Error
```

Better:

```text
We could not load transaction data for this area. Please retry shortly.
```

For data-import failures:

```text
Import completed with warnings. 1,238 records were processed and 42 were rejected due to missing or invalid location data.
```

---

# Observability

Add observability early.

At minimum, provide:

* Structured application logs
* Request IDs
* Health endpoint
* Readiness endpoint
* Import-run status tracking
* Basic metrics

Recommended metrics:

```text
http_request_duration_seconds
http_requests_total
import_records_received_total
import_records_imported_total
import_records_rejected_total
import_run_duration_seconds
graphql_query_duration_seconds
database_query_duration_seconds
```

Do not log:

* Secrets
* API keys
* Raw user credentials
* Full personal addresses if not necessary
* Sensitive tokens
* Full external payloads in production logs

---

# Security Rules

This is a public-data research platform, but basic security still matters.

Requirements:

* Never commit `.env` files.
* Maintain `.env.example`.
* Validate all user input.
* Use parameterized SQL through SQLx.
* Apply rate limits to public endpoints where appropriate.
* Limit expensive GraphQL query depth and complexity.
* Use secure cookie/session practices if authentication is added.
* Treat imported external data as untrusted.
* Sanitize user-generated notes or saved-search labels.

Do not add authentication until the core research workflow works.

When authentication is introduced, keep the initial scope simple.

---

# Testing Requirements

Every meaningful feature should include tests.

## Backend

Write tests for:

* Dataset parsing
* Normalization logic
* Validation rules
* Idempotent imports
* Duplicate detection
* Market metric calculations
* GraphQL resolver behavior
* Spatial query behavior where practical

## Frontend

Write tests for:

* Filter state behavior
* URL query synchronization
* Metric rendering
* Empty states
* Error states
* Important chart transformations

## Integration

At minimum, support one integration test flow:

```text
1. Start local PostgreSQL + PostGIS
2. Run an import using a fixture dataset
3. Query imported records through GraphQL
4. Confirm map or area metrics return expected values
```

Use small fixture datasets committed to the repository.

Do not require downloading large production datasets to run tests.

---

# Documentation Requirements

Documentation is part of the product.

Maintain:

```text
README.md
docs/architecture.md
docs/data-model.md
docs/data-sources.md
docs/local-development.md
docs/adr/
```

## README Must Include

* Project purpose
* Screenshots or demo GIF
* Architecture diagram
* Tech stack
* Main features
* Local setup
* Test commands
* Data sources
* Known limitations
* Future improvements

## Architecture Document Must Include

* System diagram
* Data flow
* API boundaries
* Background-job behavior
* Main tradeoffs
* Scaling considerations

## Data Model Document Must Include

* Entity relationship overview
* Important indexes
* Spatial query strategy
* Data lineage strategy
* Deduplication strategy

## Architecture Decision Records

Create an ADR for non-trivial decisions.

Examples:

```text
001-use-postgis-for-spatial-queries.md
002-use-graphql-for-product-api.md
003-preserve-raw-source-payloads.md
004-use-idempotent-import-runs.md
005-use-rust-actix-web-for-api.md
```

Each ADR should include:

```text
Context
Decision
Alternatives considered
Consequences
Status
```

---

# Development Workflow

Before starting a feature:

1. Identify the user problem.
2. Define the smallest useful scope.
3. Check whether it affects data lineage, metrics, or spatial queries.
4. Write or update an ADR if necessary.
5. Add tests.
6. Update documentation.
7. Verify local development setup still works.

Prefer small, reviewable changes.

Avoid large mixed commits that include:

* Refactors
* New features
* Schema changes
* Formatting changes
* Dependency upgrades

in a single pull request.

---

# Definition of Done

A feature is done only when:

* The core behavior works.
* Tests cover critical logic.
* Error states are handled.
* Loading and empty states are handled.
* Relevant metrics include units and definitions.
* Data lineage is preserved.
* Documentation is updated.
* No secrets or local data files are committed.
* The feature is usable through the intended product workflow.

A feature is not done merely because it renders in the browser.

---

# Scope Guardrails

Do not build these before the core product works:

* Full property marketplace
* Listing scraping
* Chatbot-first interface
* AI-generated investment recommendations
* Prediction models with weak or incomplete data
* Payment systems
* Complex organization permissions
* Mobile-native apps
* Multiple cities
* Multi-language support beyond Japanese and English labels
* Over-engineered microservices

Prioritize this order:

```text
1. Reliable ingestion
2. Canonical data model
3. Spatial search
4. Analyst-facing market map
5. Area comparison
6. Provenance and data quality visibility
7. Saved searches
8. Advanced analytics
```

---

# Engineering Quality Bar

This is a portfolio project intended to demonstrate readiness for a product engineering role.

Code should communicate:

* Thoughtful domain modeling
* Reliable backend design
* Practical handling of imperfect data
* Strong API design
* Appropriate use of geographic databases
* Good frontend product judgment
* Clear documentation
* Healthy engineering tradeoffs

Favor clarity over cleverness.

Favor correctness over premature optimization.

Favor a complete, trustworthy small product over an ambitious but unfinished platform.

---

# Current MVP Goal

The first release should support:

1. Importing one official real-estate transaction dataset.
2. Persisting raw records and normalized transaction records.
3. Displaying transactions on a Tokyo map.
4. Filtering by area, time period, price, floor area, and station distance.
5. Calculating basic area metrics.
6. Showing data-source and import-run provenance.
7. Running locally with one command through Docker Compose.

The MVP is successful when an analyst can answer:

> “What historical transaction activity and public-data market indicators are visible in this Tokyo area, and where did the numbers come from?”

---

# Agent Behavior

When working in this repository:

* Read relevant documentation before changing architecture.
* Do not invent data fields without documenting assumptions.
* Do not hide data-quality problems.
* Do not bypass the ingestion pipeline.
* Do not replace official-source data with scraped private data.
* Preserve backward compatibility for existing API queries where possible.
* Prefer database-level filtering and aggregation over application-memory processing.
* Write migrations for schema changes.
* Add or update tests with behavior changes.
* Update documentation in the same change set.
* Keep the project focused on commercial real-estate intelligence.

When uncertain, choose the implementation that improves:

```text
trustworthiness
traceability
reproducibility
query performance
maintainability
analyst usefulness
```
