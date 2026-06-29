# UrbanLens Data Sources

## Selected MVP Source

| Field | Value |
|---|---|
| Status | `selected_for_mvp` |
| Source name | Real Estate Information Library — Real Estate Transaction Price Information (`不動産情報ライブラリ — 不動産取引価格情報`) |
| Publisher | Ministry of Land, Infrastructure, Transport and Tourism (MLIT), Japan |
| Source URL | <https://www.reinfolib.mlit.go.jp/realEstatePrices/> |
| Dataset documentation | <https://www.reinfolib.mlit.go.jp/realEstatePrices/about/> |
| CSV manual | <https://www.reinfolib.mlit.go.jp/realEstatePrices/manual/> |
| XIT001 API | <https://www.reinfolib.mlit.go.jp/help/apiManual/xit001/> |
| XPT001 point API | <https://www.reinfolib.mlit.go.jp/help/apiManual/xpt001/> |
| Terms | <https://www.reinfolib.mlit.go.jp/help/termsOfUse/> |
| License | Public Data License 1.0 unless MLIT identifies an exception; attribution and processed-content disclosure required |
| Retrieval methods | Official CSV download; authenticated XIT001 JSON; authenticated XPT001 GeoJSON/PBF |
| Earliest coverage | 2005 Q3 for transaction-price information |
| Update frequency | Quarterly |
| Verified | `2026-06-24` |
| Fixture retrieval | `2026-06-24` |

### Why It Was Selected

It is an official, nationwide historical transaction dataset with Tokyo coverage, documented access routes, reproducible quarter/area filters, commercial reuse, fields useful for price/area/station analysis, and a publisher-provided station-context geometry interface. It directly supports the first analyst workflow without scraping private listing sites.

Only `不動産取引価格情報` is selected. `成約価格情報` is excluded because it is a separate price-information category with different provenance and the publisher notes that the displayed categories can overlap.

## Access and Credential Status

| Interface | Authentication | Phase 0 Status | Intended Use |
|---|---|---|---|
| Web CSV download | None through the official web workflow | Verified; fixtures retrieved | Source-shaped parser fixtures and manual recovery path |
| XIT001 | MLIT-issued API key in `Ocp-Apim-Subscription-Key` | `approved` — user confirmed approval and local `.env` configuration on `2026-06-24` | Deterministic record retrieval by year/quarter and area/city/station |
| XPT001 | Same MLIT-issued API key | `approved` — same locally configured key | GeoJSON/PBF nearest-station context for the first map |

The application is at <https://www.reinfolib.mlit.go.jp/api/request/>. The user confirmed that MLIT approved the application on `2026-06-24` and placed the key in a local `.env` file. The local variable name is `MLIT_REINFOLIB_API_KEY`; the issued value must never enter version control, documentation, chat, fixtures, or logs.

The local `.env` is ignored by Git. An authenticated API smoke test remains a Phase 1 setup check because requests from the current execution environment timed out without returning a response body; this does not change the confirmed credential status.

## Source Acquisition and Disclosure Process

MLIT describes the data as based on questionnaires sent to parties involved in registered real-estate transfers. It processes results so a transacted property cannot be easily identified and publishes the information quarterly. Published price values are rounded but otherwise are not corrected for property-specific or transaction-specific circumstances.

Consequences for UrbanLens:

- the dataset is an incomplete observed sample, not a market census;
- absence of a record does not mean no transaction occurred;
- source revisions can change counts for previously published quarters, especially recent periods;
- district/address labels and station context are intentionally not exact property identity;
- source price variation cannot be interpreted without asset and transaction context.

## Interfaces to the Selected Dataset

### CSV

The CSV download is the Phase 0 fixture source. The observed export is Windows-31J/CP932, includes a 30-column Japanese header, and retains blank strings and display-oriented values. The fixture exports include station name and walking time, which XIT001 does not expose in its documented output.

### XIT001

XIT001 returns gzip-encoded JSON with top-level `status` and `data`. Query inputs include year, optional quarter, price category, and at least one of prefecture, city, or station. All documented output values are strings, including numeric concepts.

### XPT001

XPT001 returns GeoJSON or PBF by XYZ tile and period. MLIT explicitly states that its point is the nearest-station point for the observation; multiple observations can share a coordinate. UrbanLens therefore assigns `nearest_station_point`, aggregates colocated features, and never presents this geometry as a property point.

XPT001 features must not be guessed onto CSV/XIT001 rows. Until the source offers a defensible shared identifier or the ingestion design treats the XPT feature itself as the raw record, CSV/XIT001 observations remain spatially `unknown`.

## Observed CSV Schema

The committed fixtures contain these exact source columns in order:

| # | CSV Header | Canonical Concept | Notes |
|---:|---|---|---|
| 1 | `種類` | asset type | Land, land + building, used condominium, and other source categories. |
| 2 | `価格情報区分` | price category | Fixtures contain only `不動産取引価格情報`. |
| 3 | `地域` | region classification | Often blank for used condominiums. |
| 4 | `市区町村コード` | municipality code | Five-digit code stored as text. |
| 5 | `都道府県名` | prefecture name | Source label. |
| 6 | `市区町村名` | municipality name | Source label. |
| 7 | `地区名` | district name | Generalized locality label. |
| 8 | `最寄駅：名称` | nearest-station label | Context label, not property identity. |
| 9 | `最寄駅：距離（分）` | station walking time | Minutes; source string/numeric must be validated. |
| 10 | `取引価格（総額）` | total transaction price | JPY; string/numeric source value. |
| 11 | `坪単価` | price per tsubo | Often blank outside eligible asset types. |
| 12 | `間取り` | floor plan | Source category; frequently blank. |
| 13 | `面積（㎡）` | recorded area | Meaning varies by asset type. |
| 14 | `取引価格（㎡単価）` | source price per m² | Populated for only 44 of 666 fixture records. |
| 15 | `土地の形状` | land shape | Source category. |
| 16 | `間口` | frontage | Metres where numeric; blank otherwise. |
| 17 | `延床面積（㎡）` | total floor area | Can contain bounded display text such as `2,000㎡以上`; treat as a lower bound, not an exact area. |
| 18 | `建築年` | building year | Japanese display string; blank for land. |
| 19 | `建物の構造` | structure | Source category. |
| 20 | `用途` | use | Multi-valued label or blank; not a verified commercial class. |
| 21 | `今後の利用目的` | intended future use | Survey response or blank. |
| 22 | `前面道路：方位` | road direction | Source category. |
| 23 | `前面道路：種類` | road type | Source category. |
| 24 | `前面道路：幅員（ｍ）` | road width | Metres where numeric. |
| 25 | `都市計画` | planning category | Source label; this is not a separate zoning dataset. |
| 26 | `建ぺい率（％）` | building coverage ratio | Percent where numeric. |
| 27 | `容積率（％）` | floor-area ratio | Percent where numeric. |
| 28 | `取引時期` | transaction quarter | Quarter only, not an exact date. |
| 29 | `改装` | renovation | Used-condominium category or blank. |
| 30 | `取引の事情等` | transaction circumstances | Source note/category or blank. |

### XIT001 Differences

XIT001 documents equivalent fields named `Type`, `PriceCategory`, `Region`, `MunicipalityCode`, `Prefecture`, `Municipality`, `DistrictName`, `TradePrice`, `PricePerUnit`, `FloorPlan`, `Area`, `UnitPrice`, `LandShape`, `Frontage`, `TotalFloorArea`, `BuildingYear`, `Structure`, `Use`, `Purpose`, `Direction`, `Classification`, `Breadth`, `CityPlanning`, `CoverageRatio`, `FloorAreaRatio`, `Period`, `Renovation`, and `Remarks`.

It also exposes `DistrictCode`, whose continuity MLIT does not guarantee across updates. The documented XIT001 output does not contain CSV station name or station walking time.

## Fixture Profile

| Ward | Records | Land | Land + Building | Used Condominium | Source ¥/m² Present | Use Blank | Office/Store Use |
|---|---:|---:|---:|---:|---:|---:|---:|
| Chuo | 176 | 4 | 28 | 144 | 4 | 39 | 21 |
| Shinagawa | 313 | 40 | 67 | 206 | 40 | 86 | 8 |
| Shibuya | 177 | 23 | 41 | 113 | 23 | 42 | 10 |
| **Total** | **666** | **67** | **136** | **463** | **67** | **167** | **39** |

The source-provided ¥/m² counts correspond to the land rows in these fixtures. This supports an eligibility-limited metric; it does not support a universal cross-type unit-price calculation.

## Known Limitations and Schema Risks

- Survey response and registration scope create incomplete, non-random coverage.
- Publisher processing prevents reliable exact-property identification.
- Values are rounded and not adjusted for transaction circumstances.
- Recent historical quarters can be revised.
- There is no documented stable transaction identifier in CSV or XIT001.
- `DistrictCode` can change and is not a cross-release identity key.
- Numeric concepts are strings or display text and can be blank, bounded, or categorical.
- Asset types use different area semantics, making mixed-type price/area aggregates misleading.
- XPT001 coordinates are nearest-station points; multiple observations can colocate.
- The three Phase 0 files are representative ward-quarter fixtures, not full schema-history coverage.

## Candidate Decision Matrix

| Candidate | Decision | Reason |
|---|---|---|
| MLIT `不動産取引価格情報` | Selected | Official historical transaction observations, Tokyo coverage, CSV/API access, useful price/area/station fields, documented reuse. |
| MLIT `成約価格情報` | Deferred | Different provenance, later historical coverage, and possible overlap with transaction-price displays. |
| Official land-price data | Deferred | It is an appraisal/reference-price domain, not the first historical transaction workflow. |
| Zoning and urban-planning data | Deferred | Valuable enrichment after reliable transaction ingestion and geography exist. |
| Demographic/economic data | Deferred | Contextual enrichment outside the first source boundary. |
| Railway/station master data | Deferred | Needed later for canonical station identity; XPT001 alone supplies map context for the first design. |
| Private listing/brokerage sources | Rejected | Outside scope, licensing risk, and not official public transaction evidence. |

## Attribution in UrbanLens

Use:

```text
Source: Ministry of Land, Infrastructure, Transport and Tourism,
Real Estate Information Library (https://www.reinfolib.mlit.go.jp/)
```

For normalized, filtered, or aggregated displays, add:

```text
Processed by UrbanLens from MLIT Real Estate Information Library content.
```

Never imply that an UrbanLens metric or transformation was produced or endorsed by MLIT.
