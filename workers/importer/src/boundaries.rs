use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use thiserror::Error;

pub const MLIT_N03_BOUNDARY_NORMALIZATION_VERSION: &str = "mlit-n03-boundary-geojson-v1";
pub const TOKYO_SPECIAL_WARD_COUNT: usize = 23;

#[derive(Debug, Error)]
pub enum BoundaryParseError {
    #[error("boundary fixture is not valid JSON")]
    Json(#[from] serde_json::Error),
    #[error("expected GeoJSON FeatureCollection")]
    NotFeatureCollection,
    #[error("GeoJSON FeatureCollection does not contain a features array")]
    MissingFeatures,
    #[error("feature at position {position} is not a GeoJSON Feature")]
    NotFeature { position: usize },
    #[error("feature at position {position} is missing properties")]
    MissingProperties { position: usize },
    #[error("feature at position {position} is missing geometry")]
    MissingGeometry { position: usize },
    #[error("feature at position {position} has unsupported geometry type `{geometry_type}`")]
    UnsupportedGeometry {
        position: usize,
        geometry_type: String,
    },
    #[error("feature at position {position} does not have Tokyo prefecture label")]
    NonTokyoFeature { position: usize },
    #[error("feature at position {position} is missing administrative code N03_007")]
    MissingAdministrativeCode { position: usize },
    #[error("feature at position {position} has unsupported administrative code `{code}`")]
    UnsupportedAdministrativeCode { position: usize, code: String },
    #[error("feature at position {position} is missing ward name N03_004")]
    MissingWardName { position: usize },
    #[error("boundary fixture contains {actual} special wards; expected 23")]
    UnexpectedWardCount { actual: usize },
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoundaryFeature {
    pub source_position: usize,
    pub administrative_code: String,
    pub name_ja: String,
    pub payload_json: Value,
    pub geometry_json: Value,
    pub payload_sha256: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WardBoundary {
    pub administrative_code: String,
    pub name_ja: String,
    pub source_record_hash: String,
    pub feature_positions: Vec<usize>,
    pub feature_hashes: Vec<String>,
    pub geometries: Vec<Value>,
}

/// Parses the committed MLIT N03 `GeoJSON` fixture into source features and
/// dissolved-boundary inputs grouped by administrative code.
///
/// # Errors
///
/// Returns an error when the fixture is not the expected Tokyo special-ward
/// `FeatureCollection` shape.
pub fn parse_mlit_n03_tokyo_wards(
    bytes: &[u8],
) -> Result<(Vec<BoundaryFeature>, Vec<WardBoundary>), BoundaryParseError> {
    let root: Value = serde_json::from_slice(bytes)?;
    if root.get("type").and_then(Value::as_str) != Some("FeatureCollection") {
        return Err(BoundaryParseError::NotFeatureCollection);
    }

    let features = root
        .get("features")
        .and_then(Value::as_array)
        .ok_or(BoundaryParseError::MissingFeatures)?;

    let mut parsed_features = Vec::with_capacity(features.len());
    for (index, feature) in features.iter().enumerate() {
        let source_position = index + 1;
        parsed_features.push(parse_feature(feature, source_position)?);
    }

    let mut ward_codes = BTreeSet::new();
    let mut grouped: BTreeMap<String, WardBoundaryBuilder> = BTreeMap::new();
    for feature in &parsed_features {
        ward_codes.insert(feature.administrative_code.clone());
        grouped
            .entry(feature.administrative_code.clone())
            .or_insert_with(|| {
                WardBoundaryBuilder::new(&feature.administrative_code, &feature.name_ja)
            })
            .push(feature);
    }

    if ward_codes.len() != TOKYO_SPECIAL_WARD_COUNT {
        return Err(BoundaryParseError::UnexpectedWardCount {
            actual: ward_codes.len(),
        });
    }

    let wards = grouped
        .into_values()
        .map(WardBoundaryBuilder::build)
        .collect();

    Ok((parsed_features, wards))
}

fn parse_feature(
    feature: &Value,
    source_position: usize,
) -> Result<BoundaryFeature, BoundaryParseError> {
    if feature.get("type").and_then(Value::as_str) != Some("Feature") {
        return Err(BoundaryParseError::NotFeature {
            position: source_position,
        });
    }

    let properties = feature.get("properties").and_then(Value::as_object).ok_or(
        BoundaryParseError::MissingProperties {
            position: source_position,
        },
    )?;
    let geometry = feature
        .get("geometry")
        .ok_or(BoundaryParseError::MissingGeometry {
            position: source_position,
        })?;
    let geometry_type = geometry.get("type").and_then(Value::as_str).unwrap_or("");
    if geometry_type != "Polygon" && geometry_type != "MultiPolygon" {
        return Err(BoundaryParseError::UnsupportedGeometry {
            position: source_position,
            geometry_type: geometry_type.to_owned(),
        });
    }

    if string_property(properties, "N03_001") != Some("東京都") {
        return Err(BoundaryParseError::NonTokyoFeature {
            position: source_position,
        });
    }

    let administrative_code = string_property(properties, "N03_007")
        .ok_or(BoundaryParseError::MissingAdministrativeCode {
            position: source_position,
        })?
        .to_owned();
    if !is_tokyo_special_ward_code(&administrative_code) {
        return Err(BoundaryParseError::UnsupportedAdministrativeCode {
            position: source_position,
            code: administrative_code,
        });
    }

    let name_ja = string_property(properties, "N03_004")
        .ok_or(BoundaryParseError::MissingWardName {
            position: source_position,
        })?
        .to_owned();

    let payload_json = feature.clone();
    let geometry_json = geometry.clone();
    let payload_sha256 = sha256_json_hex(&payload_json);

    Ok(BoundaryFeature {
        source_position,
        administrative_code,
        name_ja,
        payload_json,
        geometry_json,
        payload_sha256,
    })
}

fn string_property<'a>(properties: &'a Map<String, Value>, key: &str) -> Option<&'a str> {
    properties
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
}

fn is_tokyo_special_ward_code(code: &str) -> bool {
    matches!(code.parse::<u16>(), Ok(value) if (13_101..=13_123).contains(&value))
}

fn sha256_json_hex(value: &Value) -> String {
    let bytes = serde_json::to_vec(value).expect("JSON value serializes to bytes");
    let digest = Sha256::digest(bytes);
    format!("{digest:x}")
}

struct WardBoundaryBuilder {
    administrative_code: String,
    name_ja: String,
    feature_positions: Vec<usize>,
    feature_hashes: Vec<String>,
    geometries: Vec<Value>,
}

impl WardBoundaryBuilder {
    fn new(administrative_code: &str, name_ja: &str) -> Self {
        Self {
            administrative_code: administrative_code.to_owned(),
            name_ja: name_ja.to_owned(),
            feature_positions: Vec::new(),
            feature_hashes: Vec::new(),
            geometries: Vec::new(),
        }
    }

    fn push(&mut self, feature: &BoundaryFeature) {
        self.feature_positions.push(feature.source_position);
        self.feature_hashes.push(feature.payload_sha256.clone());
        self.geometries.push(feature.geometry_json.clone());
    }

    fn build(self) -> WardBoundary {
        let source_record_hash = sha256_json_hex(&json!({
            "administrative_code": self.administrative_code,
            "feature_hashes": self.feature_hashes,
            "feature_positions": self.feature_positions,
        }));

        WardBoundary {
            administrative_code: self.administrative_code,
            name_ja: self.name_ja,
            source_record_hash,
            feature_positions: self.feature_positions,
            feature_hashes: self.feature_hashes,
            geometries: self.geometries,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_bytes() -> Vec<u8> {
        std::fs::read(format!(
            "{}/fixtures/boundaries/mlit-n03-tokyo-23-wards-2023.geojson",
            env!("CARGO_MANIFEST_DIR")
        ))
        .expect("fixture reads")
    }

    #[test]
    fn parses_tokyo_ward_boundary_fixture() {
        let (features, wards) =
            parse_mlit_n03_tokyo_wards(&fixture_bytes()).expect("fixture parses");

        assert_eq!(features.len(), 118);
        assert_eq!(wards.len(), 23);
        assert_eq!(wards[0].administrative_code, "13101");
        assert_eq!(wards[0].name_ja, "千代田区");
        assert!(wards.iter().all(|ward| !ward.geometries.is_empty()));
    }

    #[test]
    fn source_feature_and_ward_hashes_are_stable() {
        let (first_features, first_wards) =
            parse_mlit_n03_tokyo_wards(&fixture_bytes()).expect("fixture parses");
        let (second_features, second_wards) =
            parse_mlit_n03_tokyo_wards(&fixture_bytes()).expect("fixture parses");

        assert_eq!(
            first_features[0].payload_sha256,
            second_features[0].payload_sha256
        );
        assert_eq!(
            first_wards[0].source_record_hash,
            second_wards[0].source_record_hash
        );
        assert_eq!(first_features[0].payload_sha256.len(), 64);
        assert_eq!(first_wards[0].source_record_hash.len(), 64);
    }

    #[test]
    fn rejects_non_polygon_geometry() {
        let json = r#"{
          "type": "FeatureCollection",
          "features": [{
            "type": "Feature",
            "properties": {"N03_001": "東京都", "N03_004": "千代田区", "N03_007": "13101"},
            "geometry": {"type": "Point", "coordinates": [139.7, 35.6]}
          }]
        }"#;

        let error = parse_mlit_n03_tokyo_wards(json.as_bytes()).expect_err("point is rejected");
        assert!(matches!(
            error,
            BoundaryParseError::UnsupportedGeometry {
                position: 1,
                geometry_type
            } if geometry_type == "Point"
        ));
    }
}
