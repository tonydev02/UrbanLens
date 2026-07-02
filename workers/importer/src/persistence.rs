use std::collections::BTreeMap;

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

use crate::boundaries::{BoundaryFeature, WardBoundary};
use crate::mlit::{
    AreaValue, AssetType, LocationPrecision, MlitSourceRow, NormalizedRecord,
    NormalizedTransactionObservation, PriceCategory, ValidationIssue, ValidationSeverity,
};

pub const MLIT_NORMALIZATION_VERSION: &str = "mlit-transaction-csv-v1";

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("source position {0} does not fit in database integer range")]
    SourcePositionOutOfRange(usize),
    #[error("record count {0} does not fit in database integer range")]
    RecordCountOutOfRange(usize),
    #[error("boundary feature count {0} does not fit in database integer range")]
    BoundaryFeatureCountOutOfRange(usize),
    #[error("database persistence failed")]
    Database(#[from] sqlx::Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataSourceInput {
    pub name: String,
    pub publisher: String,
    pub source_url: String,
    pub license_url: String,
    pub metadata_verified_at: String,
}

impl DataSourceInput {
    #[must_use]
    pub fn mlit_reinfolib() -> Self {
        Self {
            name: "MLIT Real Estate Information Library transaction prices".to_owned(),
            publisher: "Ministry of Land, Infrastructure, Transport and Tourism".to_owned(),
            source_url: "https://www.reinfolib.mlit.go.jp/realEstatePrices/".to_owned(),
            license_url: "https://www.reinfolib.mlit.go.jp/help/termsOfUse/".to_owned(),
            metadata_verified_at: "2026-06-24T00:00:00Z".to_owned(),
        }
    }

    #[must_use]
    pub fn mlit_n03_administrative_areas() -> Self {
        Self {
            name: "MLIT National Land Numerical Information administrative areas N03".to_owned(),
            publisher: "Ministry of Land, Infrastructure, Transport and Tourism".to_owned(),
            source_url: "https://nlftp.mlit.go.jp/ksj/gml/datalist/KsjTmplt-N03-v3_1.html"
                .to_owned(),
            license_url: "https://nlftp.mlit.go.jp/ksj/other/agreement.html".to_owned(),
            metadata_verified_at: "2026-07-02T03:03:00Z".to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatasetInput {
    pub source_id: Uuid,
    pub source_dataset_name: String,
    pub retrieval_method: String,
    pub retrieval_query: Value,
    pub source_version: Option<String>,
    pub retrieved_at: String,
    pub artifact_sha256: String,
    pub format: String,
    pub record_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataSourceRecord {
    pub id: Uuid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DatasetRecord {
    pub id: Uuid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImportRunRecord {
    pub id: Uuid,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ImportCounters {
    pub records_received: i64,
    pub records_imported: i64,
    pub records_updated: i64,
    pub duplicates_skipped: i64,
    pub records_rejected: i64,
    pub warning_records: i64,
}

impl ImportCounters {
    #[must_use]
    pub fn terminal_status(self) -> &'static str {
        if self.records_rejected > 0 || self.warning_records > 0 {
            "completed_with_warnings"
        } else {
            "completed"
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationWrite {
    Inserted,
    Updated,
    SkippedDuplicate,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PersistedRecord {
    pub raw_record_id: Uuid,
    pub observation_id: Option<Uuid>,
    pub write: ObservationWrite,
    pub warning_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryWrite {
    Inserted,
    Updated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PersistedBoundary {
    pub area_id: Uuid,
    pub boundary_id: Uuid,
    pub write: BoundaryWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PersistedRawBoundaryFeature {
    pub raw_record_id: Uuid,
    pub inserted: bool,
}

/// Creates or updates a publisher-level data source row.
///
/// # Errors
///
/// Returns an error if Postgres rejects the row or the connection fails.
pub async fn upsert_data_source(
    pool: &PgPool,
    input: &DataSourceInput,
) -> Result<DataSourceRecord, PersistenceError> {
    let id = sqlx::query_scalar::<_, Uuid>(
        r"
        INSERT INTO data_sources (
            name,
            publisher,
            source_url,
            license_url,
            metadata_verified_at
        )
        VALUES ($1, $2, $3, $4, $5::timestamptz)
        ON CONFLICT (name) DO UPDATE
        SET
            publisher = EXCLUDED.publisher,
            source_url = EXCLUDED.source_url,
            license_url = EXCLUDED.license_url,
            metadata_verified_at = EXCLUDED.metadata_verified_at,
            updated_at = now()
        RETURNING id
        ",
    )
    .bind(&input.name)
    .bind(&input.publisher)
    .bind(&input.source_url)
    .bind(&input.license_url)
    .bind(&input.metadata_verified_at)
    .fetch_one(pool)
    .await?;

    Ok(DataSourceRecord { id })
}

/// Creates or updates an exact source artifact/query dataset row.
///
/// # Errors
///
/// Returns an error if the row violates schema constraints or cannot be saved.
pub async fn upsert_dataset(
    pool: &PgPool,
    input: &DatasetInput,
) -> Result<DatasetRecord, PersistenceError> {
    let record_count = i64::try_from(input.record_count)
        .map_err(|_| PersistenceError::RecordCountOutOfRange(input.record_count))?;
    let id = sqlx::query_scalar::<_, Uuid>(
        r"
        INSERT INTO datasets (
            source_id,
            source_dataset_name,
            retrieval_method,
            retrieval_query,
            source_version,
            retrieved_at,
            artifact_sha256,
            format,
            record_count
        )
        VALUES ($1, $2, $3, $4, $5, $6::timestamptz, $7, $8, $9)
        ON CONFLICT (
            source_id,
            source_dataset_name,
            retrieval_method,
            retrieval_query,
            artifact_sha256
        ) DO UPDATE
        SET
            source_version = EXCLUDED.source_version,
            retrieved_at = EXCLUDED.retrieved_at,
            format = EXCLUDED.format,
            record_count = EXCLUDED.record_count,
            updated_at = now()
        RETURNING id
        ",
    )
    .bind(input.source_id)
    .bind(&input.source_dataset_name)
    .bind(&input.retrieval_method)
    .bind(&input.retrieval_query)
    .bind(&input.source_version)
    .bind(&input.retrieved_at)
    .bind(&input.artifact_sha256)
    .bind(&input.format)
    .bind(record_count)
    .fetch_one(pool)
    .await?;

    Ok(DatasetRecord { id })
}

/// Starts a visible import-run lifecycle for one dataset.
///
/// # Errors
///
/// Returns an error when the import run cannot be inserted.
pub async fn start_import_run(
    pool: &PgPool,
    dataset_id: Uuid,
    normalization_version: &str,
) -> Result<ImportRunRecord, PersistenceError> {
    let id = sqlx::query_scalar::<_, Uuid>(
        r"
        INSERT INTO import_runs (dataset_id, status, normalization_version)
        VALUES ($1, 'running', $2)
        RETURNING id
        ",
    )
    .bind(dataset_id)
    .bind(normalization_version)
    .fetch_one(pool)
    .await?;

    Ok(ImportRunRecord { id })
}

/// Completes an import run using explicit counters.
///
/// # Errors
///
/// Returns an error if the run does not exist or counters violate constraints.
pub async fn complete_import_run(
    pool: &PgPool,
    import_run_id: Uuid,
    counters: ImportCounters,
) -> Result<(), PersistenceError> {
    sqlx::query(
        r"
        UPDATE import_runs
        SET
            status = $2,
            completed_at = now(),
            records_received = $3,
            records_imported = $4,
            records_updated = $5,
            duplicates_skipped = $6,
            records_rejected = $7,
            warning_records = $8,
            error_kind = NULL,
            updated_at = now()
        WHERE id = $1
        ",
    )
    .bind(import_run_id)
    .bind(counters.terminal_status())
    .bind(counters.records_received)
    .bind(counters.records_imported)
    .bind(counters.records_updated)
    .bind(counters.duplicates_skipped)
    .bind(counters.records_rejected)
    .bind(counters.warning_records)
    .execute(pool)
    .await?;

    Ok(())
}

/// Marks an import run as failed while preserving the run row and reached counters.
///
/// # Errors
///
/// Returns an error if the failure state cannot be saved.
pub async fn fail_import_run(
    pool: &PgPool,
    import_run_id: Uuid,
    error_kind: &str,
    counters: ImportCounters,
) -> Result<(), PersistenceError> {
    sqlx::query(
        r"
        UPDATE import_runs
        SET
            status = 'failed',
            completed_at = now(),
            records_received = $2,
            records_imported = $3,
            records_updated = $4,
            duplicates_skipped = $5,
            records_rejected = $6,
            warning_records = $7,
            error_kind = $8,
            updated_at = now()
        WHERE id = $1
        ",
    )
    .bind(import_run_id)
    .bind(counters.records_received)
    .bind(counters.records_imported)
    .bind(counters.records_updated)
    .bind(counters.duplicates_skipped)
    .bind(counters.records_rejected)
    .bind(counters.warning_records)
    .bind(error_kind)
    .execute(pool)
    .await?;

    Ok(())
}

/// Persists one source row, its validation issues, and its normalized observation.
///
/// Duplicate raw records for the same dataset/source position are reported as
/// skipped. The original raw-record lineage is preserved instead of being
/// reassigned to the newest import run.
///
/// # Errors
///
/// Returns an error when conversion or database persistence fails.
pub async fn persist_normalized_record(
    pool: &PgPool,
    dataset_id: Uuid,
    import_run_id: Uuid,
    row: &MlitSourceRow,
    normalized: &NormalizedRecord,
    normalization_version: &str,
) -> Result<PersistedRecord, PersistenceError> {
    let source_position = i64::try_from(row.source_position)
        .map_err(|_| PersistenceError::SourcePositionOutOfRange(row.source_position))?;
    let payload_json = raw_payload_json(&row.raw_values);
    let payload_sha256 = sha256_json_hex(&payload_json);
    let validation_errors = validation_errors_json(&normalized.issues);
    let validation_status = validation_status(normalized);

    let maybe_raw_id = sqlx::query_scalar::<_, Uuid>(
        r"
        INSERT INTO raw_records (
            dataset_id,
            import_run_id,
            source_position,
            payload_json,
            payload_sha256,
            validation_status,
            validation_errors
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (dataset_id, source_position) DO NOTHING
        RETURNING id
        ",
    )
    .bind(dataset_id)
    .bind(import_run_id)
    .bind(source_position)
    .bind(&payload_json)
    .bind(&payload_sha256)
    .bind(validation_status)
    .bind(&validation_errors)
    .fetch_optional(pool)
    .await?;

    let Some(raw_record_id) = maybe_raw_id else {
        let raw_record_id = sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM raw_records WHERE dataset_id = $1 AND source_position = $2",
        )
        .bind(dataset_id)
        .bind(source_position)
        .fetch_one(pool)
        .await?;

        return Ok(PersistedRecord {
            raw_record_id,
            observation_id: None,
            write: ObservationWrite::SkippedDuplicate,
            warning_count: 0,
        });
    };

    let mut observation_id = None;
    let mut write = ObservationWrite::Rejected;
    if let Some(observation) = &normalized.observation {
        let persisted_observation_id = upsert_observation(ObservationUpsert {
            pool,
            dataset_id,
            import_run_id,
            raw_record_id,
            source_record_hash: &payload_sha256,
            observation,
            normalized,
            normalization_version,
        })
        .await?;
        upsert_unknown_location(pool, persisted_observation_id, observation).await?;
        observation_id = Some(persisted_observation_id);
        write = ObservationWrite::Inserted;
    }

    insert_validation_issues(
        pool,
        import_run_id,
        raw_record_id,
        observation_id,
        &normalized.issues,
    )
    .await?;

    Ok(PersistedRecord {
        raw_record_id,
        observation_id,
        write,
        warning_count: normalized
            .issues
            .iter()
            .filter(|issue| issue.severity == ValidationSeverity::Warning)
            .count(),
    })
}

/// Persists one official boundary source feature as a raw record.
///
/// Duplicate source positions for the same dataset are skipped while preserving
/// the original raw-record/import-run lineage.
///
/// # Errors
///
/// Returns an error when conversion or database persistence fails.
pub async fn persist_boundary_raw_feature(
    pool: &PgPool,
    dataset_id: Uuid,
    import_run_id: Uuid,
    feature: &BoundaryFeature,
) -> Result<PersistedRawBoundaryFeature, PersistenceError> {
    let source_position = i64::try_from(feature.source_position)
        .map_err(|_| PersistenceError::SourcePositionOutOfRange(feature.source_position))?;

    let maybe_raw_id = sqlx::query_scalar::<_, Uuid>(
        r"
        INSERT INTO raw_records (
            dataset_id,
            import_run_id,
            source_position,
            external_id,
            payload_json,
            payload_sha256,
            validation_status,
            validation_errors
        )
        VALUES ($1, $2, $3, $4, $5, $6, 'valid', '[]'::jsonb)
        ON CONFLICT (dataset_id, source_position) DO NOTHING
        RETURNING id
        ",
    )
    .bind(dataset_id)
    .bind(import_run_id)
    .bind(source_position)
    .bind(format!(
        "{}:{}",
        feature.administrative_code, feature.source_position
    ))
    .bind(&feature.payload_json)
    .bind(&feature.payload_sha256)
    .fetch_optional(pool)
    .await?;

    if let Some(raw_record_id) = maybe_raw_id {
        return Ok(PersistedRawBoundaryFeature {
            raw_record_id,
            inserted: true,
        });
    }

    let raw_record_id = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM raw_records WHERE dataset_id = $1 AND source_position = $2",
    )
    .bind(dataset_id)
    .bind(source_position)
    .fetch_one(pool)
    .await?;

    Ok(PersistedRawBoundaryFeature {
        raw_record_id,
        inserted: false,
    })
}

/// Upserts one governed ward area and its current dissolved boundary geometry.
///
/// Geometry dissolution is performed by `PostGIS` from the exact source-feature
/// geometries so later query behavior matches database spatial semantics.
///
/// # Errors
///
/// Returns an error if the area or boundary rows cannot be saved.
#[allow(clippy::too_many_lines)]
pub async fn upsert_ward_boundary(
    pool: &PgPool,
    source_id: Uuid,
    dataset_id: Uuid,
    import_run_id: Uuid,
    boundary_version: &str,
    ward: &WardBoundary,
) -> Result<PersistedBoundary, PersistenceError> {
    let feature_count = i64::try_from(ward.geometries.len())
        .map_err(|_| PersistenceError::BoundaryFeatureCountOutOfRange(ward.geometries.len()))?;
    let geometries_json = Value::Array(ward.geometries.clone());

    let mut tx = pool.begin().await?;

    let area_id = sqlx::query_scalar::<_, Uuid>(
        r"
        INSERT INTO areas (
            dataset_id,
            source_code,
            name,
            area_type,
            geometry,
            administrative_code,
            name_ja,
            source_id
        )
        VALUES ($1, $2, $3, 'ward', NULL, $2, $3, $4)
        ON CONFLICT (area_type, administrative_code) DO UPDATE
        SET
            dataset_id = EXCLUDED.dataset_id,
            source_id = EXCLUDED.source_id,
            source_code = EXCLUDED.source_code,
            name = EXCLUDED.name,
            name_ja = EXCLUDED.name_ja,
            updated_at = now()
        RETURNING id
        ",
    )
    .bind(dataset_id)
    .bind(&ward.administrative_code)
    .bind(&ward.name_ja)
    .bind(source_id)
    .fetch_one(&mut *tx)
    .await?;

    let existing_boundary_id = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM area_boundaries WHERE area_id = $1 AND boundary_version = $2",
    )
    .bind(area_id)
    .bind(boundary_version)
    .fetch_optional(&mut *tx)
    .await?;

    let boundary_id = sqlx::query_scalar::<_, Uuid>(
        r"
        WITH feature_geometries AS (
            SELECT ST_SetSRID(ST_GeomFromGeoJSON(value::text), 4326) AS geometry
            FROM jsonb_array_elements($10::jsonb) AS value
        ),
        dissolved AS (
            SELECT ST_Multi(ST_UnaryUnion(ST_Collect(geometry)))::geometry(MultiPolygon, 4326)
                AS geometry
            FROM feature_geometries
        )
        INSERT INTO area_boundaries (
            area_id,
            source_id,
            dataset_id,
            import_run_id,
            raw_record_id,
            administrative_code,
            name_ja,
            source_record_hash,
            source_feature_id,
            source_feature_position,
            boundary_version,
            location_precision,
            geometry,
            is_current
        )
        SELECT
            $1,
            $2,
            $3,
            $4,
            NULL,
            $5,
            $6,
            $7,
            $8,
            NULL,
            $9,
            'ward_polygon',
            dissolved.geometry,
            true
        FROM dissolved
        ON CONFLICT (area_id, boundary_version) DO UPDATE
        SET
            source_id = EXCLUDED.source_id,
            dataset_id = EXCLUDED.dataset_id,
            import_run_id = EXCLUDED.import_run_id,
            administrative_code = EXCLUDED.administrative_code,
            name_ja = EXCLUDED.name_ja,
            source_record_hash = EXCLUDED.source_record_hash,
            source_feature_id = EXCLUDED.source_feature_id,
            boundary_version = EXCLUDED.boundary_version,
            location_precision = EXCLUDED.location_precision,
            geometry = EXCLUDED.geometry,
            is_current = true,
            updated_at = now()
        RETURNING id
        ",
    )
    .bind(area_id)
    .bind(source_id)
    .bind(dataset_id)
    .bind(import_run_id)
    .bind(&ward.administrative_code)
    .bind(&ward.name_ja)
    .bind(&ward.source_record_hash)
    .bind(format!(
        "N03_007:{};source_features:{}",
        ward.administrative_code, feature_count
    ))
    .bind(boundary_version)
    .bind(&geometries_json)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        r"
        UPDATE areas
        SET
            current_boundary_id = $2,
            geometry = area_boundaries.geometry,
            updated_at = now()
        FROM area_boundaries
        WHERE areas.id = $1
            AND area_boundaries.id = $2
        ",
    )
    .bind(area_id)
    .bind(boundary_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(PersistedBoundary {
        area_id,
        boundary_id,
        write: if existing_boundary_id.is_some() {
            BoundaryWrite::Updated
        } else {
            BoundaryWrite::Inserted
        },
    })
}

#[must_use]
pub fn counters_from_outcomes(outcomes: &[PersistedRecord]) -> ImportCounters {
    let mut counters = ImportCounters {
        records_received: i64::try_from(outcomes.len()).unwrap_or(i64::MAX),
        ..ImportCounters::default()
    };

    for outcome in outcomes {
        match outcome.write {
            ObservationWrite::Inserted => counters.records_imported += 1,
            ObservationWrite::Updated => counters.records_updated += 1,
            ObservationWrite::SkippedDuplicate => counters.duplicates_skipped += 1,
            ObservationWrite::Rejected => counters.records_rejected += 1,
        }
        if outcome.warning_count > 0 {
            counters.warning_records += 1;
        }
    }

    counters
}

struct ObservationUpsert<'a> {
    pool: &'a PgPool,
    dataset_id: Uuid,
    import_run_id: Uuid,
    raw_record_id: Uuid,
    source_record_hash: &'a str,
    observation: &'a NormalizedTransactionObservation,
    normalized: &'a NormalizedRecord,
    normalization_version: &'a str,
}

#[allow(clippy::too_many_lines)]
async fn upsert_observation(input: ObservationUpsert<'_>) -> Result<Uuid, PersistenceError> {
    let observation = input.observation;
    let (total_floor_area_m2, total_floor_area_is_lower_bound) =
        match observation.total_floor_area_m2 {
            Some(AreaValue::Exact(value)) => (Some(value), false),
            Some(AreaValue::AtLeast(value)) => (Some(value), true),
            None => (None, false),
        };

    let id = sqlx::query_scalar::<_, Uuid>(
        r"
        INSERT INTO transaction_observations (
            raw_record_id,
            import_run_id,
            dataset_id,
            source_record_hash,
            normalization_version,
            validation_status,
            asset_type,
            raw_asset_type,
            price_category,
            transaction_year,
            transaction_quarter,
            trade_price_jpy,
            source_unit_price_jpy_per_m2,
            area_m2,
            total_floor_area_m2,
            total_floor_area_is_lower_bound,
            municipality_code,
            prefecture_name,
            municipality_name,
            district_name,
            nearest_station_name,
            station_walk_minutes,
            floor_plan,
            structure,
            source_use,
            intended_future_use,
            city_planning,
            renovation,
            transaction_circumstances
        )
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9,
            $10,
            $11,
            $12,
            $13,
            $14::numeric,
            $15::numeric,
            $16,
            $17,
            $18,
            $19,
            $20,
            $21,
            $22,
            $23,
            $24,
            $25,
            $26,
            $27,
            $28,
            $29
        )
        ON CONFLICT (raw_record_id) DO UPDATE
        SET
            source_record_hash = EXCLUDED.source_record_hash,
            normalization_version = EXCLUDED.normalization_version,
            validation_status = EXCLUDED.validation_status,
            asset_type = EXCLUDED.asset_type,
            raw_asset_type = EXCLUDED.raw_asset_type,
            price_category = EXCLUDED.price_category,
            transaction_year = EXCLUDED.transaction_year,
            transaction_quarter = EXCLUDED.transaction_quarter,
            trade_price_jpy = EXCLUDED.trade_price_jpy,
            source_unit_price_jpy_per_m2 = EXCLUDED.source_unit_price_jpy_per_m2,
            area_m2 = EXCLUDED.area_m2,
            total_floor_area_m2 = EXCLUDED.total_floor_area_m2,
            total_floor_area_is_lower_bound = EXCLUDED.total_floor_area_is_lower_bound,
            municipality_code = EXCLUDED.municipality_code,
            prefecture_name = EXCLUDED.prefecture_name,
            municipality_name = EXCLUDED.municipality_name,
            district_name = EXCLUDED.district_name,
            nearest_station_name = EXCLUDED.nearest_station_name,
            station_walk_minutes = EXCLUDED.station_walk_minutes,
            floor_plan = EXCLUDED.floor_plan,
            structure = EXCLUDED.structure,
            source_use = EXCLUDED.source_use,
            intended_future_use = EXCLUDED.intended_future_use,
            city_planning = EXCLUDED.city_planning,
            renovation = EXCLUDED.renovation,
            transaction_circumstances = EXCLUDED.transaction_circumstances,
            updated_at = now()
        RETURNING id
        ",
    )
    .bind(input.raw_record_id)
    .bind(input.import_run_id)
    .bind(input.dataset_id)
    .bind(input.source_record_hash)
    .bind(input.normalization_version)
    .bind(validation_status(input.normalized))
    .bind(asset_type(observation.asset_type))
    .bind(&observation.raw_asset_type)
    .bind(price_category(observation.price_category))
    .bind(i16::try_from(observation.transaction_quarter.year).unwrap_or(i16::MAX))
    .bind(i16::from(observation.transaction_quarter.quarter))
    .bind(observation.trade_price_jpy)
    .bind(observation.source_unit_price_jpy_per_m2)
    .bind(observation.area_m2)
    .bind(total_floor_area_m2)
    .bind(total_floor_area_is_lower_bound)
    .bind(&observation.municipality_code)
    .bind(&observation.prefecture_name)
    .bind(&observation.municipality_name)
    .bind(&observation.district_name)
    .bind(&observation.nearest_station_name)
    .bind(observation.station_walk_minutes.map(i32::from))
    .bind(&observation.floor_plan)
    .bind(&observation.structure)
    .bind(&observation.use_label)
    .bind(&observation.future_use)
    .bind(&observation.city_planning)
    .bind(&observation.renovation)
    .bind(&observation.transaction_circumstances)
    .fetch_one(input.pool)
    .await?;

    Ok(id)
}

async fn upsert_unknown_location(
    pool: &PgPool,
    observation_id: Uuid,
    observation: &NormalizedTransactionObservation,
) -> Result<(), PersistenceError> {
    sqlx::query(
        r"
        INSERT INTO transaction_location_contexts (
            transaction_observation_id,
            location_precision,
            location,
            source_location_label
        )
        VALUES ($1, $2, NULL, $3)
        ON CONFLICT (transaction_observation_id) DO UPDATE
        SET
            location_precision = EXCLUDED.location_precision,
            location = EXCLUDED.location,
            source_location_label = EXCLUDED.source_location_label,
            updated_at = now()
        ",
    )
    .bind(observation_id)
    .bind(location_precision(observation.location_precision))
    .bind(&observation.district_name)
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_validation_issues(
    pool: &PgPool,
    import_run_id: Uuid,
    raw_record_id: Uuid,
    observation_id: Option<Uuid>,
    issues: &[ValidationIssue],
) -> Result<(), PersistenceError> {
    for issue in issues {
        sqlx::query(
            r"
            INSERT INTO validation_issues (
                import_run_id,
                raw_record_id,
                transaction_observation_id,
                issue_code,
                severity,
                field_name,
                raw_value_summary,
                message,
                disposition
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ",
        )
        .bind(import_run_id)
        .bind(raw_record_id)
        .bind(observation_id)
        .bind(issue.code.as_str())
        .bind(severity(issue.severity))
        .bind(issue.field)
        .bind(raw_value_summary(&issue.raw_value))
        .bind(issue.message)
        .bind(issue.disposition)
        .execute(pool)
        .await?;
    }

    Ok(())
}

fn raw_payload_json(raw_values: &BTreeMap<String, String>) -> Value {
    serde_json::to_value(raw_values).expect("BTreeMap<String, String> serializes to JSON")
}

fn validation_errors_json(issues: &[ValidationIssue]) -> Value {
    Value::Array(
        issues
            .iter()
            .map(|issue| {
                json!({
                    "code": issue.code.as_str(),
                    "severity": severity(issue.severity),
                    "field": issue.field,
                    "raw_value_summary": raw_value_summary(&issue.raw_value),
                    "message": issue.message,
                    "disposition": issue.disposition,
                })
            })
            .collect(),
    )
}

fn sha256_json_hex(value: &Value) -> String {
    let bytes = serde_json::to_vec(value).expect("JSON value serializes to bytes");
    let digest = Sha256::digest(bytes);
    format!("{digest:x}")
}

fn validation_status(normalized: &NormalizedRecord) -> &'static str {
    if normalized.observation.is_none() {
        "rejected"
    } else if normalized.issues.is_empty() {
        "valid"
    } else {
        "valid_with_warnings"
    }
}

fn raw_value_summary(raw_value: &str) -> String {
    const LIMIT: usize = 120;
    raw_value.chars().take(LIMIT).collect()
}

fn asset_type(asset_type: AssetType) -> &'static str {
    match asset_type {
        AssetType::Land => "land",
        AssetType::LandAndBuilding => "land_and_building",
        AssetType::UsedCondominium => "used_condominium",
        AssetType::Unknown => "unknown",
    }
}

fn price_category(price_category: PriceCategory) -> &'static str {
    match price_category {
        PriceCategory::TransactionPriceInformation => "transaction_price_information",
    }
}

fn location_precision(location_precision: LocationPrecision) -> &'static str {
    match location_precision {
        LocationPrecision::Unknown => "unknown",
    }
}

fn severity(severity: ValidationSeverity) -> &'static str {
    match severity {
        ValidationSeverity::Warning => "warning",
        ValidationSeverity::Rejection => "rejection",
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use sqlx::{PgPool, postgres::PgPoolOptions};

    use super::*;
    use crate::boundaries::{MLIT_N03_BOUNDARY_NORMALIZATION_VERSION, parse_mlit_n03_tokyo_wards};
    use crate::mlit::{normalize_source_row, parse_mlit_csv_file, parse_mlit_csv_text};

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../apps/api/migrations");

    async fn test_pool() -> Option<PgPool> {
        let database_url = std::env::var("DATABASE_URL").ok()?;
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .ok()?;
        MIGRATOR.run(&pool).await.ok()?;
        Some(pool)
    }

    fn unique_source() -> DataSourceInput {
        let suffix = Uuid::new_v4();
        DataSourceInput {
            name: format!("MLIT repository test {suffix}"),
            publisher: "MLIT".to_owned(),
            source_url: "https://www.reinfolib.mlit.go.jp/realEstatePrices/".to_owned(),
            license_url: "https://www.reinfolib.mlit.go.jp/help/termsOfUse/".to_owned(),
            metadata_verified_at: "2026-06-24T00:00:00Z".to_owned(),
        }
    }

    fn fixture_path(file_name: &str) -> String {
        format!(
            "{}/fixtures/transactions/{file_name}",
            env!("CARGO_MANIFEST_DIR")
        )
    }

    fn boundary_fixture_path() -> String {
        format!(
            "{}/fixtures/boundaries/mlit-n03-tokyo-23-wards-2023.geojson",
            env!("CARGO_MANIFEST_DIR")
        )
    }

    async fn fixture_dataset(pool: &PgPool, source: &DataSourceRecord) -> DatasetRecord {
        upsert_dataset(
            pool,
            &DatasetInput {
                source_id: source.id,
                source_dataset_name: "MLIT transaction fixture".to_owned(),
                retrieval_method: "fixture_csv".to_owned(),
                retrieval_query: json!({
                    "prefecture": "13",
                    "period": "2024Q4",
                    "fixture": Uuid::new_v4().to_string(),
                }),
                source_version: Some("2024Q4".to_owned()),
                retrieved_at: "2026-06-24T00:00:00Z".to_owned(),
                artifact_sha256: "a".repeat(64),
                format: "csv; encoding=cp932".to_owned(),
                record_count: 1,
            },
        )
        .await
        .expect("dataset upsert succeeds")
    }

    async fn boundary_dataset(pool: &PgPool, source: &DataSourceRecord) -> DatasetRecord {
        upsert_dataset(
            pool,
            &DatasetInput {
                source_id: source.id,
                source_dataset_name: format!("MLIT N03 boundary fixture {}", Uuid::new_v4()),
                retrieval_method: "fixture_geojson".to_owned(),
                retrieval_query: json!({
                    "prefecture": "13",
                    "area_type": "ward",
                    "fixture": Uuid::new_v4().to_string(),
                }),
                source_version: Some("2023-01-01".to_owned()),
                retrieved_at: "2026-07-02T03:03:00Z".to_owned(),
                artifact_sha256: "c".repeat(64),
                format: "geojson; encoding=utf-8; crs=JGD2011/SRID4326-compatible".to_owned(),
                record_count: 118,
            },
        )
        .await
        .expect("boundary dataset upsert succeeds")
    }

    #[tokio::test]
    async fn upserts_source_and_dataset_by_stable_identity() {
        let Some(pool) = test_pool().await else {
            return;
        };

        let source_input = unique_source();
        let first_source = upsert_data_source(&pool, &source_input)
            .await
            .expect("source inserts");
        let second_source = upsert_data_source(&pool, &source_input)
            .await
            .expect("source reuses");
        assert_eq!(first_source.id, second_source.id);

        let dataset_input = DatasetInput {
            source_id: first_source.id,
            source_dataset_name: "MLIT transaction fixture".to_owned(),
            retrieval_method: "fixture_csv".to_owned(),
            retrieval_query: json!({"prefecture": "13", "period": "2024Q4"}),
            source_version: Some("2024Q4".to_owned()),
            retrieved_at: "2026-06-24T00:00:00Z".to_owned(),
            artifact_sha256: "b".repeat(64),
            format: "csv; encoding=cp932".to_owned(),
            record_count: 666,
        };
        let first_dataset = upsert_dataset(&pool, &dataset_input)
            .await
            .expect("dataset inserts");
        let second_dataset = upsert_dataset(&pool, &dataset_input)
            .await
            .expect("dataset reuses");
        assert_eq!(first_dataset.id, second_dataset.id);
    }

    #[tokio::test]
    async fn persists_observation_lineage_and_skips_duplicate_source_position() {
        let Some(pool) = test_pool().await else {
            return;
        };

        let source = upsert_data_source(&pool, &unique_source())
            .await
            .expect("source inserts");
        let dataset = fixture_dataset(&pool, &source).await;
        let first_run = start_import_run(&pool, dataset.id, MLIT_NORMALIZATION_VERSION)
            .await
            .expect("first run starts");
        let second_run = start_import_run(&pool, dataset.id, MLIT_NORMALIZATION_VERSION)
            .await
            .expect("second run starts");

        let rows = parse_mlit_csv_file(fixture_path("mlit-reinfolib-chuo-2024-q4.csv"))
            .expect("fixture parses");
        let row = &rows[0];
        let normalized = normalize_source_row(row);

        let first = persist_normalized_record(
            &pool,
            dataset.id,
            first_run.id,
            row,
            &normalized,
            MLIT_NORMALIZATION_VERSION,
        )
        .await
        .expect("first row persists");
        assert_eq!(first.write, ObservationWrite::Inserted);
        assert!(first.observation_id.is_some());

        let duplicate = persist_normalized_record(
            &pool,
            dataset.id,
            second_run.id,
            row,
            &normalized,
            MLIT_NORMALIZATION_VERSION,
        )
        .await
        .expect("duplicate row skips");
        assert_eq!(duplicate.write, ObservationWrite::SkippedDuplicate);
        assert_eq!(duplicate.raw_record_id, first.raw_record_id);

        let observation_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM transaction_observations WHERE raw_record_id = $1",
        )
        .bind(first.raw_record_id)
        .fetch_one(&pool)
        .await
        .expect("count observations");
        assert_eq!(observation_count, 1);
    }

    #[tokio::test]
    async fn persists_rejections_warnings_counters_and_failed_runs() {
        let Some(pool) = test_pool().await else {
            return;
        };

        let source = upsert_data_source(&pool, &unique_source())
            .await
            .expect("source inserts");
        let dataset = fixture_dataset(&pool, &source).await;
        let run = start_import_run(&pool, dataset.id, MLIT_NORMALIZATION_VERSION)
            .await
            .expect("run starts");

        let csv = "種類,価格情報区分,地域,市区町村コード,都道府県名,市区町村名,地区名,最寄駅：名称,最寄駅：距離（分）,取引価格（総額）,坪単価,間取り,面積（㎡）,取引価格（㎡単価）,土地の形状,間口,延床面積（㎡）,建築年,建物の構造,用途,今後の利用目的,前面道路：方位,前面道路：種類,前面道路：幅員（ｍ）,都市計画,建ぺい率（％）,容積率（％）,取引時期,改装,取引の事情等\n謎の種類,不動産取引価格情報,,13102,東京都,中央区,銀座,銀座,not-minutes,1000000,,,20,,,,,,,,,,,,,,,2024年第4四半期,,\n宅地(土地),成約価格情報,,13102,東京都,中央区,銀座,銀座,3,1000000,,,20,,,,,,,,,,,,,,,2024年第4四半期,,\n";
        let rows = parse_mlit_csv_text(csv).expect("edge csv parses");

        let mut outcomes = Vec::new();
        for row in &rows {
            outcomes.push(
                persist_normalized_record(
                    &pool,
                    dataset.id,
                    run.id,
                    row,
                    &normalize_source_row(row),
                    MLIT_NORMALIZATION_VERSION,
                )
                .await
                .expect("row persists"),
            );
        }

        let counters = counters_from_outcomes(&outcomes);
        assert_eq!(counters.records_received, 2);
        assert_eq!(counters.records_imported, 1);
        assert_eq!(counters.records_rejected, 1);
        assert_eq!(counters.warning_records, 1);

        complete_import_run(&pool, run.id, counters)
            .await
            .expect("run completes with warnings");

        let issue_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM validation_issues WHERE import_run_id = $1",
        )
        .bind(run.id)
        .fetch_one(&pool)
        .await
        .expect("count issues");
        assert_eq!(issue_count, 3);

        let failed_run = start_import_run(&pool, dataset.id, MLIT_NORMALIZATION_VERSION)
            .await
            .expect("failed run starts");
        fail_import_run(
            &pool,
            failed_run.id,
            "repository_test_forced_failure",
            ImportCounters {
                records_received: 1,
                ..ImportCounters::default()
            },
        )
        .await
        .expect("failure is visible");

        let failed_status = sqlx::query_scalar::<_, String>(
            "SELECT status FROM import_runs WHERE id = $1 AND error_kind = 'repository_test_forced_failure'",
        )
        .bind(failed_run.id)
        .fetch_one(&pool)
        .await
        .expect("read failed run");
        assert_eq!(failed_status, "failed");
    }

    #[tokio::test]
    #[allow(clippy::too_many_lines)]
    async fn persists_ward_boundaries_and_skips_duplicate_raw_features() {
        let Some(pool) = test_pool().await else {
            return;
        };

        let source_input = DataSourceInput {
            name: format!("MLIT N03 repository test {}", Uuid::new_v4()),
            ..DataSourceInput::mlit_n03_administrative_areas()
        };
        let source = upsert_data_source(&pool, &source_input)
            .await
            .expect("source inserts");
        let dataset = boundary_dataset(&pool, &source).await;
        let fixture_bytes = std::fs::read(boundary_fixture_path()).expect("boundary fixture reads");
        let (features, wards) =
            parse_mlit_n03_tokyo_wards(&fixture_bytes).expect("boundary fixture parses");

        let first_run =
            start_import_run(&pool, dataset.id, MLIT_N03_BOUNDARY_NORMALIZATION_VERSION)
                .await
                .expect("first run starts");
        let mut first_raw_inserts = 0;
        for feature in &features {
            let persisted = persist_boundary_raw_feature(&pool, dataset.id, first_run.id, feature)
                .await
                .expect("raw boundary feature persists");
            if persisted.inserted {
                first_raw_inserts += 1;
            }
        }
        assert_eq!(first_raw_inserts, 118);

        let mut first_boundary_inserts = 0;
        for ward in &wards {
            let persisted = upsert_ward_boundary(
                &pool,
                source.id,
                dataset.id,
                first_run.id,
                "2023-01-01-test",
                ward,
            )
            .await
            .expect("ward boundary persists");
            if persisted.write == BoundaryWrite::Inserted {
                first_boundary_inserts += 1;
            }
        }
        assert_eq!(first_boundary_inserts, 23);

        let second_run =
            start_import_run(&pool, dataset.id, MLIT_N03_BOUNDARY_NORMALIZATION_VERSION)
                .await
                .expect("second run starts");
        let mut duplicate_raw_features = 0;
        for feature in &features {
            let persisted = persist_boundary_raw_feature(&pool, dataset.id, second_run.id, feature)
                .await
                .expect("duplicate raw boundary feature skips");
            if !persisted.inserted {
                duplicate_raw_features += 1;
            }
        }
        assert_eq!(duplicate_raw_features, 118);

        let mut updated_boundaries = 0;
        for ward in &wards {
            let persisted = upsert_ward_boundary(
                &pool,
                source.id,
                dataset.id,
                second_run.id,
                "2023-01-01-test",
                ward,
            )
            .await
            .expect("ward boundary updates");
            if persisted.write == BoundaryWrite::Updated {
                updated_boundaries += 1;
            }
        }
        assert_eq!(updated_boundaries, 23);

        let counts: (i64, i64, i64) = sqlx::query_as(
            r"
            SELECT
                (SELECT COUNT(*) FROM raw_records WHERE dataset_id = $1),
                (SELECT COUNT(*) FROM areas WHERE source_id = $2 AND area_type = 'ward'),
                (SELECT COUNT(*) FROM area_boundaries WHERE dataset_id = $1)
            ",
        )
        .bind(dataset.id)
        .bind(source.id)
        .fetch_one(&pool)
        .await
        .expect("boundary counts query");
        assert_eq!(counts, (118, 23, 23));

        let geometry_check: (i64, i64, bool) = sqlx::query_as(
            r"
            SELECT
                COUNT(*),
                COUNT(*) FILTER (
                    WHERE ST_SRID(geometry) = 4326
                        AND GeometryType(geometry) = 'MULTIPOLYGON'
                ),
                bool_and(ST_IsValid(geometry))
            FROM area_boundaries
            WHERE dataset_id = $1
            ",
        )
        .bind(dataset.id)
        .fetch_one(&pool)
        .await
        .expect("geometry check query");
        assert_eq!(geometry_check, (23, 23, true));
    }
}
