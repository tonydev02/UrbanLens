# MLIT N03 Tokyo Ward Boundary Fixture

This directory contains a small source-derived GeoJSON fixture for Phase 03
spatial data model work. It is filtered from the official MLIT National Land
Numerical Information administrative-area artifact for Tokyo.

## Acquisition

| Item | Value |
|---|---|
| Source | National Land Numerical Information administrative-area data (`国土数値情報 行政区域データ`, identifier `N03`) |
| Publisher | Ministry of Land, Infrastructure, Transport and Tourism (MLIT), Japan |
| Source page | <https://nlftp.mlit.go.jp/ksj/gml/datalist/KsjTmplt-N03-v3_1.html> |
| Source artifact | `N03-20230101_13_GML.zip` |
| Artifact URL | <https://nlftp.mlit.go.jp/ksj/gml/data/N03/N03-2023/N03-20230101_13_GML.zip> |
| Artifact SHA-256 | `5430e29c82e5fa485a63e2b8979f17f2cb6bf95aa5eb8df8cd85a86248bcde50` |
| Retrieved | `2026-07-02 12:03 +09:00` |
| Dataset version | Product specification 3.1; data reference date `2023-01-01` |
| Source CRS | JGD2011 longitude/latitude (`JGD2011 / (B, L)`) |
| Fixture CRS | GeoJSON longitude/latitude, SRID 4326 compatible for PostGIS ingestion |
| Source format | MLIT zip containing GeoJSON, Shapefile, GML/JPGIS2014, metadata XML, and PRJ |
| Fixture format | Filtered GeoJSON FeatureCollection |
| Geometry type | Source polygons; importer may dissolve by ward code into multipolygon boundaries |
| Administrative-code field | `N03_007` |

## Fixture Generation

The committed fixture preserves the source feature granularity for Tokyo special
ward codes `13101` through `13123`.

```bash
mkdir -p /tmp/urbanlens-n03 workers/importer/fixtures/boundaries
curl -L \
  https://nlftp.mlit.go.jp/ksj/gml/data/N03/N03-2023/N03-20230101_13_GML.zip \
  -o /tmp/urbanlens-n03/N03-20230101_13_GML.zip
unzip -p /tmp/urbanlens-n03/N03-20230101_13_GML.zip N03-23_13_230101.geojson \
  | jq -c '{type:.type, name:"MLIT N03 2023 Tokyo special wards source polygons", features:[.features[] | select(.properties.N03_007 >= "13101" and .properties.N03_007 <= "13123")]}' \
  > workers/importer/fixtures/boundaries/mlit-n03-tokyo-23-wards-2023.geojson
```

Run the fixture check from the repository root:

```bash
bash scripts/validate-boundary-fixture.sh
```

## Files

| File | Source Features | Unique Ward Codes | Geometry | Bytes | SHA-256 |
|---|---:|---:|---|---:|---|
| `mlit-n03-tokyo-23-wards-2023.geojson` | 118 | 23 | Polygon | 1,292,135 | `42d78e7f93cb63c6d2afdf388e06bab454962fb55a3f9b366148cfdb701595a7` |

The 118 features are source polygon parts across 23 Tokyo special wards. Slice 3
can import each source feature as a raw record and upsert one governed `ward`
area per `N03_007` code.

## Attribute Mapping

| Source Attribute | Meaning | UrbanLens Use |
|---|---|---|
| `N03_001` | Prefecture name | Must be `東京都`. |
| `N03_002` | Subprefecture / branch office label | Usually null for Tokyo wards. |
| `N03_003` | County or ordinance-designated city label | Usually null for Tokyo wards. |
| `N03_004` | Municipality / ward name | `areas.name_ja`. |
| `N03_007` | Administrative-area code | `areas.administrative_code`; stored as text. |

## License and Attribution

MLIT documents the 2018-and-later N03 administrative-area data as governed by
the National Land Numerical Information download-service terms for open data.
This artifact is derived from Geospatial Information Authority of Japan base map
administrative-boundary data, so downstream use must preserve the MLIT/GSI
attribution and any reproduction notices documented with the source artifact.

Use this attribution with the fixture:

```text
Source: Ministry of Land, Infrastructure, Transport and Tourism,
National Land Numerical Information administrative-area data (N03).
Original source material includes Geospatial Information Authority of Japan
Digital Map (Basic Geospatial Information) administrative-boundary data.
```

Any normalized or aggregated UrbanLens output must additionally state that it
was processed by UrbanLens from MLIT content.

## Limitations

- The fixture is a bounded development artifact, not a replacement for full
  source metadata preservation.
- MLIT notes that some municipal boundaries are unsettled and provisional; its
  page specifically cautions about Tokyo Chiyoda, Chuo, and Minato boundaries.
- Ward polygons are area-selection and aggregation geometry. They must never be
  attached to individual MLIT transaction observations as property points.
- Phase 03 should store raw boundary features in the existing lineage tables so
  every imported boundary can be traced back to this exact fixture/artifact.
