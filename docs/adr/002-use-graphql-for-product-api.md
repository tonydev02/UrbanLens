# ADR 002 — Use GraphQL for the Product API

## Status

Accepted — 2026-06-24

## Context

The analyst workspace combines map observations, filters, area summaries, metric definitions, provenance, and data-quality details. Screens need different projections of the same governed domain without direct access to external sources or database tables.

## Decision

Use async-graphql as the product-facing API in the Rust service. Queries must be bounded by cursor pagination or strict limits, use explicit filter inputs, and enforce query depth/complexity limits.

GraphQL will cover product domains such as transactions, areas, market metrics, provenance, import status, and later saved searches. Health, readiness, and metrics endpoints remain conventional HTTP endpoints. Raw payloads are excluded from normal product types and require an intentional provenance/admin boundary.

## Alternatives Considered

- REST-only product endpoints: viable, but rejected for the primary product surface because map, metric, detail, and comparison screens require many related projections.
- Direct frontend database/source access: rejected because it bypasses validation, lineage, authorization boundaries, and stable schemas.
- Unbounded GraphQL list queries: rejected because map and analytics queries could exhaust API/database resources.
- Multiple service-specific APIs: rejected as premature for the MVP.

## Consequences

- Schema evolution and backward compatibility require discipline.
- Resolver behavior, pagination, authorization when introduced, and query complexity need tests.
- N+1 access must be controlled with query design/batching.
- Product errors should remain readable while preserving structured server context.
- The frontend can request screen-specific projections without depending on MLIT interfaces.
