# ADR 005 — Use Rust and Actix Web for the API

## Status

Accepted — 2026-06-24

## Context

UrbanLens needs a production-oriented API that coordinates bounded GraphQL queries, explicit SQL/PostGIS access, ingestion status, structured errors, request IDs, and observability. The project is also intended to demonstrate reliable backend and domain engineering rather than rapid tutorial scaffolding.

## Decision

Build the API in Rust using Actix Web, async-graphql, SQLx, PostgreSQL/PostGIS, and Tokio-compatible asynchronous execution. Use SQLx parameter binding for all SQL and explicit query design for spatial workloads.

Actix Web hosts GraphQL plus health/readiness/metrics endpoints. Redis may be introduced only when a concrete cache or job-coordination need is demonstrated.

## Alternatives Considered

- Node.js/TypeScript API: strong ecosystem alignment with the frontend, but not selected for the intended backend quality/portfolio focus.
- Python API: productive for analysis and profiling, but not selected as the production API runtime.
- Axum: a credible Rust alternative; Actix Web was selected as the stated stack and offers mature production HTTP capabilities.
- ORM-first persistence: rejected in favor of SQLx because spatial SQL and aggregation behavior should remain explicit.
- Microservices: rejected as premature; one modular API and separate importer worker are sufficient.

## Consequences

- The team accepts Rust compile-time and learning overhead.
- Database queries and GraphQL boundaries can be strongly typed and reviewed explicitly.
- SQLx migrations/query validation, Clippy, formatting, and tests become CI requirements.
- Blocking work must not run on async request workers.
- Architecture should remain modular without creating operationally separate services before justified.
