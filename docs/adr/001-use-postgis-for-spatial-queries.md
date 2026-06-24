# ADR 001 — Use PostGIS for Spatial Queries

## Status

Accepted — 2026-06-24

## Context

UrbanLens must support viewport searches, ward aggregation, station-context points, future distance queries, and transparent location precision. Performing these operations in Rust or the browser would move large datasets through application memory, weaken spatial-index use, and create inconsistent query semantics.

## Decision

Use PostgreSQL with PostGIS as the canonical spatial store and query engine. Store geometries with SRID 4326, choose geometry or geography per query semantics, and create spatial indexes for viewport, containment, join, and proximity workloads.

Application code supplies validated parameters and composes bounded queries through SQLx. Large geographic filtering and aggregation must remain in PostGIS.

## Alternatives Considered

- Application-memory filtering: rejected because it scales poorly and bypasses spatial indexes.
- Browser-only filtering: rejected because it requires unbounded data transfer and creates inconsistent results.
- Separate search/GIS service: deferred because PostgreSQL/PostGIS covers the MVP without another operational system.
- Non-spatial latitude/longitude columns: rejected because they cannot express polygon joins and indexed spatial predicates safely.

## Consequences

- Local and deployed PostgreSQL must include PostGIS.
- Migrations must define SRID, constraints, and spatial indexes explicitly.
- Spatial integration tests need representative points/polygons.
- Query plans and timeouts require monitoring as data grows.
- Location precision remains separate from geometry; a point does not imply an exact property.
