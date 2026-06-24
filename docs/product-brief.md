# UrbanLens Product Brief

## Product Purpose

UrbanLens is a Tokyo commercial real-estate research workspace built from official public data. Its first release helps an analyst explore historical transaction observations, compare like-for-like groups, and inspect the provenance and limitations behind every displayed indicator.

It is not a property marketplace, listing browser, valuation service, or investment recommendation engine.

## Primary User and Question

The primary user is a commercial real-estate analyst, asset manager, developer, broker, urban-planning researcher, or internal data specialist.

The first workflow answers:

> What historical transaction activity and public-data market indicators are visible in this Tokyo area, and where did the numbers come from?

## First Analyst Workflow

```text
Open the Tokyo market map
  → select a ward or map viewport
  → set period and comparable asset-type filters
  → optionally filter total price, area, and station walking time
  → inspect station-context aggregates and type-faceted summary metrics
  → inspect the quarterly observation-count trend
  → select an aggregate, then an observation
  → review the source, import date, raw-record lineage, quality warnings,
    and geographic-accuracy limitation
  → share the same view through URL query parameters
```

### URL State

The future analyst workspace must represent practical filters in the URL: ward or viewport, period, asset type, total-price range, area range, station-distance range, selected aggregate, page, and sort. Opening a copied URL must reconstruct the same research view.

## Initial Dataset Boundary

The only MVP dataset is MLIT `不動産取引価格情報`. UrbanLens excludes `成約価格情報` from the initial dataset because it has different provenance and the publisher warns that the two price-information categories can contain duplicate displayed data.

The default record mix includes:

- `宅地(土地)` — land
- `宅地(土地と建物)` — land and building
- `中古マンション等` — used condominium and related strata observations

Agricultural and forest transactions are outside the initial workflow. Source `用途` values containing office or store language may be highlighted for discovery, but they do not establish a durable commercial-property classification.

## First Map

### Meaning

The first map represents station context, not property coordinates. XPT001 explicitly returns the nearest-station point associated with price observations; multiple observations can share one point. The UI groups colocated observations into one station-context aggregate.

An aggregate shows:

- observed transaction count;
- asset-type composition;
- selected period;
- source freshness;
- `nearest_station_point` precision label; and
- a direct disclaimer that the point is not the transacted property.

### Precision Behavior

| Precision | Initial Behavior |
|---|---|
| `exact_point` | Unsupported by the selected source; never assigned. |
| `nearest_station_point` | Display a station-context aggregate only when the geometry comes directly from XPT001. |
| `district_centroid` | Deferred until an authoritative centroid source is selected. |
| `ward_polygon` | Use for ward-level aggregate selection, never as an observation point. |
| `unknown` | Keep the observation eligible for non-spatial metrics; do not plot a point. |

CSV or XIT001 records must not be joined to XPT001 features using district, price, row order, or other guessed combinations. Until a defensible link exists, CSV/XIT001 observations have `location_precision=unknown`.

### Required Disclaimer

> Map points represent nearest-station context and may contain multiple observations. They are not exact transacted-property locations. Open an observation to see its location precision and source.

## Filters

| Filter | Unit / Values | Rule |
|---|---|---|
| Area | ward or viewport | Apply in PostGIS; never filter a production result set in application memory. |
| Period | calendar quarter range | Use the published transaction quarter, not an invented exact date. |
| Asset type | land; land + building; used condominium | Keep types distinct in price and area summaries. |
| Total transaction price | JPY | Positive normalized values only; invalid values produce validation issues. |
| Recorded area | m² | Meaning varies by asset type and must be shown with its type context. |
| Station walking time | minutes | Supported by the selected CSV schema; blank/invalid values remain unknown. |
| Source use | source labels | Optional discovery filter; do not market it as a verified commercial-property class. |

## Metrics and Trend

### Summary Metrics

| Metric | Definition | Eligibility |
|---|---|---|
| Observation count | Count of eligible `transaction_observation` records. | Valid lineage and current filters. |
| Asset-type distribution | Count and share by source asset type. | Same as observation count. |
| Median total transaction price | Median `trade_price_jpy`, faceted by asset type. | Positive parsed price; never combine asset types into one price claim. |
| Median recorded area | Median `area_m2`, faceted by asset type. | Non-negative parsed area with the source meaning visible. |
| Median source price per m² | Median source-provided `unit_price_jpy_per_m2`, faceted by asset type. | Use only records where MLIT supplies the field; do not derive a universal replacement. |
| Quarterly observation trend | Count per published quarter, faceted by asset type. | Eligible observations in the selected period. |

Every metric must display:

- qualified metric name, such as “Observed transaction median”;
- unit;
- selected period and filters;
- eligible sample size (`n`);
- data source and freshness; and
- a definition/limitations affordance.

For `n < 5`, show the value with a “small sample” warning. For `n = 0`, show an honest unavailable state rather than zero or a fabricated value.

## Observation and Provenance Detail

The detail view must provide:

- source values and normalized display values side by side where transformation matters;
- dataset and source name;
- retrieval/import date and import-run status;
- source quarter and ward/district labels;
- raw-record lineage identifier;
- normalization-logic version;
- validation warnings;
- location-precision label and explanation; and
- attribution to MLIT.

Raw payload JSON is not exposed by default. It is an intentional provenance/admin action.

## Required UI States

- Loading: preserve filter context and show bounded skeletons rather than a blank map.
- Empty: state that no observations match the active filters and offer filter reset.
- Source/import warning: show stale or partial data and affected counts.
- Error: provide a retry and readable context without exposing internals.
- Small sample: retain the metric, display `n`, and qualify interpretation.
- Unknown location: include the record in non-spatial results and explain why it is absent from the map.

## Product Claims

UrbanLens may claim:

- it displays historical observations published by MLIT;
- metrics are reproducible from documented eligible observations and filters;
- map points represent stated station or aggregate context; and
- every normalized observation is designed to retain source lineage.

UrbanLens must not claim:

- complete transaction-market coverage;
- current listings, asking prices, appraisal values, or “the market price”;
- an exact or identified transacted property;
- stable property identity across records;
- causal explanations or investment recommendations; or
- an office/commercial classification beyond what a source label supports.

## MVP Success Criteria

An analyst can define an area and comparable asset-type scope, understand observed transaction activity, inspect type-specific metrics and trends, and reach source and accuracy information without being led to infer exact properties or complete market coverage.
