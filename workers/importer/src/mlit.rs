use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use encoding_rs::Encoding;
use thiserror::Error;

const HEADER: [&str; 30] = [
    "種類",
    "価格情報区分",
    "地域",
    "市区町村コード",
    "都道府県名",
    "市区町村名",
    "地区名",
    "最寄駅：名称",
    "最寄駅：距離（分）",
    "取引価格（総額）",
    "坪単価",
    "間取り",
    "面積（㎡）",
    "取引価格（㎡単価）",
    "土地の形状",
    "間口",
    "延床面積（㎡）",
    "建築年",
    "建物の構造",
    "用途",
    "今後の利用目的",
    "前面道路：方位",
    "前面道路：種類",
    "前面道路：幅員（ｍ）",
    "都市計画",
    "建ぺい率（％）",
    "容積率（％）",
    "取引時期",
    "改装",
    "取引の事情等",
];

#[derive(Debug, Error)]
pub enum MlitParseError {
    #[error("failed to read MLIT CSV fixture `{path}`")]
    ReadFile {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("MLIT CSV fixture is not valid Windows-31J / CP932 text")]
    InvalidEncoding,
    #[error("CSV row {row_number} has an unterminated quoted field")]
    UnterminatedQuote { row_number: usize },
    #[error("CSV row {row_number} has {actual} columns; expected {expected}")]
    WrongColumnCount {
        row_number: usize,
        expected: usize,
        actual: usize,
    },
    #[error("MLIT CSV header does not match the documented 30-column fixture schema")]
    UnexpectedHeader,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MlitSourceRow {
    pub source_position: usize,
    pub raw_values: BTreeMap<String, String>,
    pub fields: MlitTransactionFields,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MlitTransactionFields {
    pub asset_type: String,
    pub price_category: String,
    pub region: String,
    pub municipality_code: String,
    pub prefecture_name: String,
    pub municipality_name: String,
    pub district_name: String,
    pub nearest_station_name: String,
    pub station_walk_minutes: String,
    pub trade_price: String,
    pub price_per_tsubo: String,
    pub floor_plan: String,
    pub area: String,
    pub unit_price: String,
    pub land_shape: String,
    pub frontage: String,
    pub total_floor_area: String,
    pub building_year: String,
    pub structure: String,
    pub use_label: String,
    pub future_use: String,
    pub road_direction: String,
    pub road_type: String,
    pub road_width: String,
    pub city_planning: String,
    pub building_coverage_ratio: String,
    pub floor_area_ratio: String,
    pub transaction_period: String,
    pub renovation: String,
    pub transaction_circumstances: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetType {
    Land,
    LandAndBuilding,
    UsedCondominium,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriceCategory {
    TransactionPriceInformation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationPrecision {
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransactionQuarter {
    pub year: u16,
    pub quarter: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AreaValue {
    Exact(f64),
    AtLeast(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationSeverity {
    Warning,
    Rejection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationIssueCode {
    InvalidQuarter,
    InvalidMunicipalityCode,
    InvalidPriceCategory,
    UnknownAssetType,
    InvalidTradePrice,
    NegativePrice,
    InvalidSourceUnitPrice,
    InvalidArea,
    NegativeArea,
    InvalidTotalFloorArea,
    InvalidStationWalkMinutes,
}

impl ValidationIssueCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidQuarter => "invalid_quarter",
            Self::InvalidMunicipalityCode => "invalid_municipality_code",
            Self::InvalidPriceCategory => "invalid_price_category",
            Self::UnknownAssetType => "unknown_asset_type",
            Self::InvalidTradePrice => "invalid_trade_price",
            Self::NegativePrice => "negative_price",
            Self::InvalidSourceUnitPrice => "invalid_source_unit_price",
            Self::InvalidArea => "invalid_area",
            Self::NegativeArea => "negative_area",
            Self::InvalidTotalFloorArea => "invalid_total_floor_area",
            Self::InvalidStationWalkMinutes => "invalid_station_walk_minutes",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    pub code: ValidationIssueCode,
    pub severity: ValidationSeverity,
    pub field: &'static str,
    pub raw_value: String,
    pub message: &'static str,
    pub disposition: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NormalizedTransactionObservation {
    pub asset_type: AssetType,
    pub raw_asset_type: String,
    pub price_category: PriceCategory,
    pub transaction_quarter: TransactionQuarter,
    pub trade_price_jpy: Option<i64>,
    pub source_unit_price_jpy_per_m2: Option<i64>,
    pub area_m2: Option<f64>,
    pub total_floor_area_m2: Option<AreaValue>,
    pub municipality_code: String,
    pub prefecture_name: String,
    pub municipality_name: String,
    pub district_name: Option<String>,
    pub nearest_station_name: Option<String>,
    pub station_walk_minutes: Option<u16>,
    pub floor_plan: Option<String>,
    pub structure: Option<String>,
    pub use_label: Option<String>,
    pub future_use: Option<String>,
    pub city_planning: Option<String>,
    pub renovation: Option<String>,
    pub transaction_circumstances: Option<String>,
    pub location_precision: LocationPrecision,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NormalizedRecord {
    pub source_position: usize,
    pub observation: Option<NormalizedTransactionObservation>,
    pub issues: Vec<ValidationIssue>,
}

/// Reads and parses an MLIT transaction CSV fixture from disk.
///
/// # Errors
///
/// Returns an error when the file cannot be read, cannot be decoded as
/// Windows-31J/CP932, or does not match the documented MLIT CSV fixture schema.
pub fn parse_mlit_csv_file(path: impl AsRef<Path>) -> Result<Vec<MlitSourceRow>, MlitParseError> {
    let path = path.as_ref();
    let bytes = fs::read(path).map_err(|source| MlitParseError::ReadFile {
        path: path.display().to_string(),
        source,
    })?;

    parse_mlit_csv_bytes(&bytes)
}

/// Decodes Windows-31J/CP932 bytes and parses MLIT transaction CSV rows.
///
/// # Errors
///
/// Returns an error when bytes contain decoding errors or the decoded CSV does
/// not match the documented MLIT CSV fixture schema.
pub fn parse_mlit_csv_bytes(bytes: &[u8]) -> Result<Vec<MlitSourceRow>, MlitParseError> {
    let Some(encoding) = Encoding::for_label(b"windows-31j") else {
        return Err(MlitParseError::InvalidEncoding);
    };
    let (decoded, _encoding_used, had_errors) = encoding.decode(bytes);
    if had_errors {
        return Err(MlitParseError::InvalidEncoding);
    }

    parse_mlit_csv_text(&decoded)
}

/// Parses decoded MLIT transaction CSV text.
///
/// # Errors
///
/// Returns an error when CSV quoting is malformed, the header is unexpected, or
/// a row has a different column count from the documented 30-column fixture.
pub fn parse_mlit_csv_text(text: &str) -> Result<Vec<MlitSourceRow>, MlitParseError> {
    let records = parse_csv_records(text)?;
    if records.is_empty() {
        return Ok(Vec::new());
    }

    if records[0] != HEADER {
        return Err(MlitParseError::UnexpectedHeader);
    }

    records
        .into_iter()
        .skip(1)
        .enumerate()
        .map(|(idx, columns)| {
            let row_number = idx + 2;
            if columns.len() != HEADER.len() {
                return Err(MlitParseError::WrongColumnCount {
                    row_number,
                    expected: HEADER.len(),
                    actual: columns.len(),
                });
            }

            let raw_values = HEADER
                .iter()
                .zip(columns.iter())
                .map(|(header, value)| ((*header).to_owned(), value.clone()))
                .collect::<BTreeMap<_, _>>();

            Ok(MlitSourceRow {
                source_position: idx + 1,
                fields: MlitTransactionFields::from_columns(&columns),
                raw_values,
            })
        })
        .collect()
}

impl MlitTransactionFields {
    fn from_columns(columns: &[String]) -> Self {
        Self {
            asset_type: columns[0].clone(),
            price_category: columns[1].clone(),
            region: columns[2].clone(),
            municipality_code: columns[3].clone(),
            prefecture_name: columns[4].clone(),
            municipality_name: columns[5].clone(),
            district_name: columns[6].clone(),
            nearest_station_name: columns[7].clone(),
            station_walk_minutes: columns[8].clone(),
            trade_price: columns[9].clone(),
            price_per_tsubo: columns[10].clone(),
            floor_plan: columns[11].clone(),
            area: columns[12].clone(),
            unit_price: columns[13].clone(),
            land_shape: columns[14].clone(),
            frontage: columns[15].clone(),
            total_floor_area: columns[16].clone(),
            building_year: columns[17].clone(),
            structure: columns[18].clone(),
            use_label: columns[19].clone(),
            future_use: columns[20].clone(),
            road_direction: columns[21].clone(),
            road_type: columns[22].clone(),
            road_width: columns[23].clone(),
            city_planning: columns[24].clone(),
            building_coverage_ratio: columns[25].clone(),
            floor_area_ratio: columns[26].clone(),
            transaction_period: columns[27].clone(),
            renovation: columns[28].clone(),
            transaction_circumstances: columns[29].clone(),
        }
    }
}

/// Normalizes one MLIT source row into an optional canonical transaction observation.
#[must_use]
pub fn normalize_source_row(row: &MlitSourceRow) -> NormalizedRecord {
    let mut issues = Vec::new();
    let fields = &row.fields;

    let asset_type = normalize_asset_type(&fields.asset_type, &mut issues);
    let price_category = normalize_price_category(&fields.price_category, &mut issues);
    let transaction_quarter = parse_transaction_quarter(&fields.transaction_period, &mut issues);
    validate_municipality_code(&fields.municipality_code, &mut issues);

    if has_rejections(&issues) {
        return NormalizedRecord {
            source_position: row.source_position,
            observation: None,
            issues,
        };
    }

    let (Some(price_category), Some(transaction_quarter)) = (price_category, transaction_quarter)
    else {
        return NormalizedRecord {
            source_position: row.source_position,
            observation: None,
            issues,
        };
    };

    let observation = NormalizedTransactionObservation {
        asset_type,
        raw_asset_type: fields.asset_type.clone(),
        price_category,
        transaction_quarter,
        trade_price_jpy: parse_positive_i64(
            &fields.trade_price,
            "取引価格（総額）",
            ValidationIssueCode::InvalidTradePrice,
            ValidationIssueCode::NegativePrice,
            "invalid trade price was set to null",
            &mut issues,
        ),
        source_unit_price_jpy_per_m2: parse_positive_i64(
            &fields.unit_price,
            "取引価格（㎡単価）",
            ValidationIssueCode::InvalidSourceUnitPrice,
            ValidationIssueCode::NegativePrice,
            "invalid source unit price was set to null",
            &mut issues,
        ),
        area_m2: parse_non_negative_f64(
            &fields.area,
            "面積（㎡）",
            ValidationIssueCode::InvalidArea,
            ValidationIssueCode::NegativeArea,
            &mut issues,
        ),
        total_floor_area_m2: parse_total_floor_area(&fields.total_floor_area, &mut issues),
        municipality_code: fields.municipality_code.clone(),
        prefecture_name: fields.prefecture_name.clone(),
        municipality_name: fields.municipality_name.clone(),
        district_name: blank_to_none(&fields.district_name),
        nearest_station_name: blank_to_none(&fields.nearest_station_name),
        station_walk_minutes: parse_station_walk_minutes(&fields.station_walk_minutes, &mut issues),
        floor_plan: blank_to_none(&fields.floor_plan),
        structure: blank_to_none(&fields.structure),
        use_label: blank_to_none(&fields.use_label),
        future_use: blank_to_none(&fields.future_use),
        city_planning: blank_to_none(&fields.city_planning),
        renovation: blank_to_none(&fields.renovation),
        transaction_circumstances: blank_to_none(&fields.transaction_circumstances),
        location_precision: LocationPrecision::Unknown,
    };

    NormalizedRecord {
        source_position: row.source_position,
        observation: Some(observation),
        issues,
    }
}

fn normalize_asset_type(raw: &str, issues: &mut Vec<ValidationIssue>) -> AssetType {
    match raw {
        "宅地(土地)" => AssetType::Land,
        "宅地(土地と建物)" => AssetType::LandAndBuilding,
        "中古マンション等" => AssetType::UsedCondominium,
        _ => {
            issues.push(issue(
                ValidationIssueCode::UnknownAssetType,
                ValidationSeverity::Warning,
                "種類",
                raw,
                "unknown source asset type was preserved",
                "preserved_unknown",
            ));
            AssetType::Unknown
        }
    }
}

fn normalize_price_category(raw: &str, issues: &mut Vec<ValidationIssue>) -> Option<PriceCategory> {
    if raw == "不動産取引価格情報" {
        Some(PriceCategory::TransactionPriceInformation)
    } else {
        issues.push(issue(
            ValidationIssueCode::InvalidPriceCategory,
            ValidationSeverity::Rejection,
            "価格情報区分",
            raw,
            "unsupported price category cannot become an MVP transaction observation",
            "record_rejected",
        ));
        None
    }
}

fn parse_transaction_quarter(
    raw: &str,
    issues: &mut Vec<ValidationIssue>,
) -> Option<TransactionQuarter> {
    let Some((year, suffix)) = raw.split_once("年第") else {
        issues.push(invalid_quarter(raw));
        return None;
    };
    let Some((quarter, trailing)) = suffix.split_once("四半期") else {
        issues.push(invalid_quarter(raw));
        return None;
    };
    if !trailing.is_empty() {
        issues.push(invalid_quarter(raw));
        return None;
    }

    let Ok(year) = year.parse::<u16>() else {
        issues.push(invalid_quarter(raw));
        return None;
    };
    let Ok(quarter) = quarter.parse::<u8>() else {
        issues.push(invalid_quarter(raw));
        return None;
    };

    if (1..=4).contains(&quarter) {
        Some(TransactionQuarter { year, quarter })
    } else {
        issues.push(invalid_quarter(raw));
        None
    }
}

fn invalid_quarter(raw: &str) -> ValidationIssue {
    issue(
        ValidationIssueCode::InvalidQuarter,
        ValidationSeverity::Rejection,
        "取引時期",
        raw,
        "transaction quarter must be a parseable published quarter",
        "record_rejected",
    )
}

fn validate_municipality_code(raw: &str, issues: &mut Vec<ValidationIssue>) {
    let valid_tokyo_ward =
        raw.len() == 5 && raw.starts_with("13") && raw.chars().all(|char| char.is_ascii_digit());

    if !valid_tokyo_ward {
        issues.push(issue(
            ValidationIssueCode::InvalidMunicipalityCode,
            ValidationSeverity::Rejection,
            "市区町村コード",
            raw,
            "MVP fixture imports require a Tokyo municipality code",
            "record_rejected",
        ));
    }
}

fn parse_positive_i64(
    raw: &str,
    field: &'static str,
    invalid_code: ValidationIssueCode,
    negative_code: ValidationIssueCode,
    message: &'static str,
    issues: &mut Vec<ValidationIssue>,
) -> Option<i64> {
    if raw.is_empty() {
        return None;
    }
    let normalized = raw.replace(',', "");
    match normalized.parse::<i64>() {
        Ok(value) if value > 0 => Some(value),
        Ok(_) => {
            issues.push(issue(
                negative_code,
                ValidationSeverity::Warning,
                field,
                raw,
                "non-positive price was set to null",
                "set_null",
            ));
            None
        }
        Err(_) => {
            issues.push(issue(
                invalid_code,
                ValidationSeverity::Warning,
                field,
                raw,
                message,
                "set_null",
            ));
            None
        }
    }
}

fn parse_non_negative_f64(
    raw: &str,
    field: &'static str,
    invalid_code: ValidationIssueCode,
    negative_code: ValidationIssueCode,
    issues: &mut Vec<ValidationIssue>,
) -> Option<f64> {
    if raw.is_empty() {
        return None;
    }
    let normalized = raw.replace(',', "");
    match normalized.parse::<f64>() {
        Ok(value) if value >= 0.0 && value.is_finite() => Some(value),
        Ok(_) => {
            issues.push(issue(
                negative_code,
                ValidationSeverity::Warning,
                field,
                raw,
                "negative area was set to null",
                "set_null",
            ));
            None
        }
        Err(_) => {
            issues.push(issue(
                invalid_code,
                ValidationSeverity::Warning,
                field,
                raw,
                "invalid numeric area was set to null",
                "set_null",
            ));
            None
        }
    }
}

fn parse_total_floor_area(raw: &str, issues: &mut Vec<ValidationIssue>) -> Option<AreaValue> {
    if raw.is_empty() {
        return None;
    }

    if let Some(value) = raw.strip_suffix("㎡以上") {
        return parse_area_number(value)
            .map(AreaValue::AtLeast)
            .or_else(|| {
                issues.push(issue(
                    ValidationIssueCode::InvalidTotalFloorArea,
                    ValidationSeverity::Warning,
                    "延床面積（㎡）",
                    raw,
                    "bounded total floor area could not be parsed and was set to null",
                    "set_null",
                ));
                None
            });
    }

    parse_area_number(raw).map(AreaValue::Exact).or_else(|| {
        issues.push(issue(
            ValidationIssueCode::InvalidTotalFloorArea,
            ValidationSeverity::Warning,
            "延床面積（㎡）",
            raw,
            "total floor area could not be parsed and was set to null",
            "set_null",
        ));
        None
    })
}

fn parse_area_number(raw: &str) -> Option<f64> {
    let value = raw.replace(',', "").parse::<f64>().ok()?;
    (value >= 0.0 && value.is_finite()).then_some(value)
}

fn parse_station_walk_minutes(raw: &str, issues: &mut Vec<ValidationIssue>) -> Option<u16> {
    if raw.is_empty() {
        return None;
    }

    if let Ok(value) = raw.replace(',', "").parse::<u16>() {
        Some(value)
    } else {
        issues.push(issue(
            ValidationIssueCode::InvalidStationWalkMinutes,
            ValidationSeverity::Warning,
            "最寄駅：距離（分）",
            raw,
            "station walking time could not be parsed and was set to null",
            "set_null",
        ));
        None
    }
}

fn blank_to_none(raw: &str) -> Option<String> {
    (!raw.is_empty()).then(|| raw.to_owned())
}

fn has_rejections(issues: &[ValidationIssue]) -> bool {
    issues
        .iter()
        .any(|issue| issue.severity == ValidationSeverity::Rejection)
}

fn issue(
    code: ValidationIssueCode,
    severity: ValidationSeverity,
    field: &'static str,
    raw_value: &str,
    message: &'static str,
    disposition: &'static str,
) -> ValidationIssue {
    ValidationIssue {
        code,
        severity,
        field,
        raw_value: raw_value.to_owned(),
        message,
        disposition,
    }
}

fn parse_csv_records(text: &str) -> Result<Vec<Vec<String>>, MlitParseError> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut chars = text.chars().peekable();
    let mut in_quotes = false;
    let mut row_number = 1;

    while let Some(char) = chars.next() {
        match char {
            '"' if in_quotes && chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            '"' => {
                in_quotes = !in_quotes;
            }
            ',' if !in_quotes => {
                row.push(std::mem::take(&mut field));
            }
            '\n' if !in_quotes => {
                row.push(std::mem::take(&mut field));
                rows.push(std::mem::take(&mut row));
                row_number += 1;
            }
            '\r' if !in_quotes => {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                row.push(std::mem::take(&mut field));
                rows.push(std::mem::take(&mut row));
                row_number += 1;
            }
            _ => field.push(char),
        }
    }

    if in_quotes {
        return Err(MlitParseError::UnterminatedQuote { row_number });
    }

    if !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_DIR: &str = "fixtures/transactions";

    fn fixture_path(file_name: &str) -> String {
        format!("{}/{FIXTURE_DIR}/{file_name}", env!("CARGO_MANIFEST_DIR"))
    }

    #[test]
    fn parses_all_committed_cp932_fixtures() {
        let cases = [
            ("mlit-reinfolib-chuo-2024-q4.csv", 176, "13102"),
            ("mlit-reinfolib-shinagawa-2024-q4.csv", 313, "13109"),
            ("mlit-reinfolib-shibuya-2024-q4.csv", 177, "13113"),
        ];

        let mut total = 0;
        for (file_name, expected_count, expected_ward_code) in cases {
            let rows = parse_mlit_csv_file(fixture_path(file_name)).expect("fixture parses");
            assert_eq!(rows.len(), expected_count);
            assert!(
                rows.iter()
                    .all(|row| row.fields.municipality_code == expected_ward_code)
            );
            assert!(rows.iter().all(|row| row.raw_values.len() == HEADER.len()));
            assert_eq!(rows[0].source_position, 1);
            assert_eq!(rows[rows.len() - 1].source_position, expected_count);
            total += rows.len();
        }

        assert_eq!(total, 666);
    }

    #[test]
    fn preserves_blank_raw_strings_and_decodes_japanese_headers() {
        let rows = parse_mlit_csv_file(fixture_path("mlit-reinfolib-chuo-2024-q4.csv"))
            .expect("fixture parses");
        let first = &rows[0];

        assert_eq!(first.raw_values["種類"], "中古マンション等");
        assert_eq!(first.raw_values["地域"], "");
        assert_eq!(first.fields.price_category, "不動産取引価格情報");
        assert_eq!(first.fields.nearest_station_name, "新富町(東京)");
    }

    #[test]
    fn normalizes_fixture_rows_without_rejections() {
        let file_names = [
            "mlit-reinfolib-chuo-2024-q4.csv",
            "mlit-reinfolib-shinagawa-2024-q4.csv",
            "mlit-reinfolib-shibuya-2024-q4.csv",
        ];
        let mut source_unit_price_count = 0;
        let mut bounded_floor_area_count = 0;

        for file_name in file_names {
            let rows = parse_mlit_csv_file(fixture_path(file_name)).expect("fixture parses");
            for row in &rows {
                let normalized = normalize_source_row(row);
                assert!(
                    normalized.observation.is_some(),
                    "fixture row {} should normalize",
                    row.source_position
                );
                assert!(
                    normalized
                        .issues
                        .iter()
                        .all(|issue| issue.severity == ValidationSeverity::Warning)
                );

                let observation = normalized.observation.expect("observation exists");
                assert_eq!(
                    observation.transaction_quarter,
                    TransactionQuarter {
                        year: 2024,
                        quarter: 4
                    }
                );
                assert_eq!(observation.location_precision, LocationPrecision::Unknown);
                if observation.source_unit_price_jpy_per_m2.is_some() {
                    source_unit_price_count += 1;
                }
                if matches!(observation.total_floor_area_m2, Some(AreaValue::AtLeast(_))) {
                    bounded_floor_area_count += 1;
                }
            }
        }

        assert_eq!(source_unit_price_count, 67);
        assert_eq!(bounded_floor_area_count, 4);
    }

    #[test]
    fn preserves_source_unit_price_only_when_mlit_supplies_it() {
        let rows = parse_mlit_csv_file(fixture_path("mlit-reinfolib-shinagawa-2024-q4.csv"))
            .expect("fixture parses");

        let land = rows
            .iter()
            .find(|row| !row.fields.unit_price.is_empty())
            .expect("fixture has source unit price");
        let condominium = rows
            .iter()
            .find(|row| {
                row.fields.asset_type == "中古マンション等" && row.fields.unit_price.is_empty()
            })
            .expect("fixture has blank source unit price");

        let land_observation = normalize_source_row(land).observation.expect("normalizes");
        let condo_observation = normalize_source_row(condominium)
            .observation
            .expect("normalizes");

        assert!(land_observation.source_unit_price_jpy_per_m2.is_some());
        assert_eq!(condo_observation.source_unit_price_jpy_per_m2, None);
    }

    #[test]
    fn invalid_values_become_warnings_or_rejections_without_fake_defaults() {
        let text = format!(
            "{}\n{}",
            HEADER
                .iter()
                .map(|header| format!("\"{header}\""))
                .collect::<Vec<_>>()
                .join(","),
            [
                "謎の種類",
                "不動産取引価格情報",
                "",
                "13102",
                "東京都",
                "中央区",
                "銀座",
                "銀座",
                "徒歩すぐ",
                "-10",
                "",
                "",
                "bad-area",
                "",
                "",
                "",
                "2,000㎡以上",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "2024年第4四半期",
                "",
                ""
            ]
            .iter()
            .map(|value| format!("\"{value}\""))
            .collect::<Vec<_>>()
            .join(",")
        );

        let rows = parse_mlit_csv_text(&text).expect("synthetic row parses");
        let normalized = normalize_source_row(&rows[0]);
        let observation = normalized.observation.expect("warnings still normalize");

        assert_eq!(observation.asset_type, AssetType::Unknown);
        assert_eq!(observation.trade_price_jpy, None);
        assert_eq!(observation.area_m2, None);
        assert_eq!(observation.station_walk_minutes, None);
        assert_eq!(
            observation.total_floor_area_m2,
            Some(AreaValue::AtLeast(2000.0))
        );
        assert!(normalized.issues.iter().any(|issue| {
            issue.code == ValidationIssueCode::UnknownAssetType
                && issue.disposition == "preserved_unknown"
        }));
        assert!(
            normalized
                .issues
                .iter()
                .any(|issue| issue.code == ValidationIssueCode::NegativePrice)
        );
        assert!(
            normalized
                .issues
                .iter()
                .any(|issue| issue.code == ValidationIssueCode::InvalidArea)
        );
        assert!(
            normalized
                .issues
                .iter()
                .any(|issue| issue.code == ValidationIssueCode::InvalidStationWalkMinutes)
        );
    }

    #[test]
    fn invalid_required_identity_or_period_rejects_without_observation() {
        let text = format!(
            "{}\n{}",
            HEADER
                .iter()
                .map(|header| format!("\"{header}\""))
                .collect::<Vec<_>>()
                .join(","),
            [
                "宅地(土地)",
                "成約価格情報",
                "",
                "999",
                "東京都",
                "中央区",
                "銀座",
                "銀座",
                "3",
                "1000000",
                "",
                "",
                "50",
                "100000",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "2024年第5四半期",
                "",
                ""
            ]
            .iter()
            .map(|value| format!("\"{value}\""))
            .collect::<Vec<_>>()
            .join(",")
        );

        let rows = parse_mlit_csv_text(&text).expect("synthetic row parses");
        let normalized = normalize_source_row(&rows[0]);

        assert!(normalized.observation.is_none());
        assert!(
            normalized
                .issues
                .iter()
                .any(|issue| issue.code == ValidationIssueCode::InvalidPriceCategory)
        );
        assert!(
            normalized
                .issues
                .iter()
                .any(|issue| issue.code == ValidationIssueCode::InvalidMunicipalityCode)
        );
        assert!(
            normalized
                .issues
                .iter()
                .any(|issue| issue.code == ValidationIssueCode::InvalidQuarter)
        );
    }
}
