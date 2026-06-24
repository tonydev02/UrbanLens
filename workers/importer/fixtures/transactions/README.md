# MLIT Transaction Fixture

This directory contains unmodified CSV exports from the Ministry of Land, Infrastructure, Transport and Tourism (MLIT) Real Estate Information Library (`不動産情報ライブラリ`). They are development fixtures for the first UrbanLens ingestion path.

## Acquisition

| Item | Value |
|---|---|
| Source | `不動産取引価格情報` |
| Publisher | Ministry of Land, Infrastructure, Transport and Tourism (MLIT), Japan |
| Download interface | <https://www.reinfolib.mlit.go.jp/realEstatePrices/> |
| Download instructions | <https://www.reinfolib.mlit.go.jp/realEstatePrices/manual/> |
| Retrieved | `2026-06-24 07:15 +07:00` |
| Price category | `不動産取引価格情報` only; `成約価格情報` excluded |
| Period | `2024年第4四半期` (`20244`) |
| Asset type | `all` in the source query; fixture observations contain land, land with building, and used condominium categories |
| Encoding | Windows-31J / CP932 |
| Format | Source CSV, 30 columns, header included |
| Transformation | None; filenames were normalized for the repository, but file bytes were not changed |

The files were retrieved from the same public CSV service used by the official web download screen with these source query fields:

```text
language=ja
areaCondition=address
prefecture=13
transactionPrice=true
kind=all
seasonFrom=20244
seasonTo=20244
city=<ward code>
```

## Files

| File | Ward Code | Source Ward | Records | Bytes | Asset-Type Counts | Office/Store Use |
|---|---:|---|---:|---:|---|---:|
| `mlit-reinfolib-chuo-2024-q4.csv` | `13102` | 中央区 | 176 | 38,048 | land 4; land + building 28; used condominium 144 | 21 |
| `mlit-reinfolib-shinagawa-2024-q4.csv` | `13109` | 品川区 | 313 | 67,290 | land 40; land + building 67; used condominium 206 | 8 |
| `mlit-reinfolib-shibuya-2024-q4.csv` | `13113` | 渋谷区 | 177 | 38,602 | land 23; land + building 41; used condominium 113 | 10 |

Record counts exclude the header. “Office/Store Use” counts records whose source `用途` contains `事務所` or `店舗`; this is a profiling signal, not a durable commercial-property classification.

All expected MVP asset categories are present. Agricultural and forest transactions are not present in these ward-quarter exports and remain outside the initial workflow.

## Observed Quality Characteristics

- All 666 records have transaction price, area, nearest-station name, station walking time, ward, and transaction quarter.
- All records are categorized as `不動産取引価格情報`; no `成約価格情報` records are mixed in.
- Source-provided `取引価格（㎡単価）` is populated for 67 records and blank for 599 records. UrbanLens must not manufacture a cross-asset-type replacement.
- `用途` is blank in 167 records. A blank remains unknown and is not converted to a default use.
- The source uses display values such as `2,000㎡以上`, Japanese era/domain labels, and blank strings. Import parsing must preserve the original raw value before normalization.
- The final row uses CRLF while preceding rows use LF in each downloaded file. This source quirk is preserved; parsers must use universal-newline handling.

## Integrity

Run from this directory:

```bash
shasum -a 256 -c SHA256SUMS
```

## License and Attribution

Unless otherwise noted by MLIT, the content is provided under the [Public Data License 1.0](https://www.digital.go.jp/resources/open_data/public_data_license_v1.0). The [Real Estate Information Library terms](https://www.reinfolib.mlit.go.jp/help/termsOfUse/) require source attribution and require processed content to be identified as processed rather than represented as government-created output.

Use this attribution with the fixture:

```text
Source: Ministry of Land, Infrastructure, Transport and Tourism,
Real Estate Information Library (https://www.reinfolib.mlit.go.jp/)
```

These fixture files are unmodified source exports. Any normalized or aggregated UrbanLens output must additionally state that it was created by UrbanLens from MLIT content.

## Limitations

- Records come from an official survey-based disclosure process and are not a complete market census.
- Locations and values are processed so individual transacted properties are not easily identifiable.
- Prices are rounded by the publisher and are not adjusted for property-specific transaction circumstances.
- A fixture is a bounded development sample, not evidence of every historical schema variant.
- CSV rows contain no exact property coordinate. They must use `location_precision=unknown` unless a defensible XPT001 station-context geometry is obtained without guessing a row-level join.
