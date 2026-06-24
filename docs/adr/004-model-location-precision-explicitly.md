# ADR 004 — Model Location Precision Explicitly

## Status

Accepted — 2026-06-24

## Context

MLIT transaction disclosures intentionally avoid easy identification of individual properties. CSV/XIT001 provide locality and station labels without exact property coordinates. XPT001 states that its geometry is the nearest-station point and that multiple observations can share it. A generic point column would invite false exactness.

## Decision

Every mappable or non-mappable observation/aggregate carries one explicit precision value:

```text
exact_point
nearest_station_point
district_centroid
ward_polygon
unknown
```

The precision controls storage constraints, GraphQL output, map symbol/aggregation, filtering interpretation, tooltips, detail copy, and disclaimers. The selected source does not support `exact_point`. XPT001 geometry is `nearest_station_point`. CSV/XIT001 observations without defensible geometry are `unknown`; they are not guessed onto XPT001 features.

## Alternatives Considered

- Treat all points as property locations: rejected as misleading.
- Store a boolean `is_approximate`: rejected because station, centroid, polygon, and unknown contexts require different behavior.
- Omit imprecise records from the product: rejected because they remain useful for non-spatial analysis and would bias metrics silently.
- Geocode district/address labels automatically: rejected until an authoritative source and documented precision rule exist.

## Consequences

- UI and API must display/explain precision, not merely store it.
- Spatial queries distinguish observation geometry from aggregate/context geometry.
- Some valid observations will appear in metrics but not as map points.
- Future geography enrichment must assign precision through documented evidence and lineage.
- Tests must prove that unknown or approximate locations cannot render as exact property points.
