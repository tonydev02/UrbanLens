use std::error::Error as _;
use std::ffi::OsStr;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::{PgPool, postgres::PgPoolOptions};
use thiserror::Error;
use urbanlens_importer::boundaries::{
    MLIT_N03_BOUNDARY_NORMALIZATION_VERSION, parse_mlit_n03_tokyo_wards,
};
use urbanlens_importer::mlit::{normalize_source_row, parse_mlit_csv_bytes};
use urbanlens_importer::persistence::{
    DataSourceInput, DatasetInput, ImportCounters, MLIT_NORMALIZATION_VERSION, PersistenceError,
    counters_from_outcomes, fail_import_run, persist_boundary_raw_feature,
    persist_normalized_record, start_import_run, upsert_data_source, upsert_dataset,
    upsert_ward_boundary,
};

const DEFAULT_DATABASE_URL: &str = "postgres://urbanlens:urbanlens_dev@localhost:5432/urbanlens";
const DEFAULT_FIXTURE_DIR: &str = "workers/importer/fixtures/transactions";
const DEFAULT_PERIOD: &str = "2024Q4";
const DEFAULT_PREFECTURE: &str = "13";
const FIXTURE_RETRIEVED_AT: &str = "2026-06-24T00:15:00Z";
const DEFAULT_BOUNDARY_FIXTURE: &str =
    "workers/importer/fixtures/boundaries/mlit-n03-tokyo-23-wards-2023.geojson";
const BOUNDARY_RETRIEVED_AT: &str = "2026-07-02T03:03:00Z";
const BOUNDARY_SOURCE_VERSION: &str = "2023-01-01";

#[derive(Debug, Error)]
enum CliError {
    #[error("{0}")]
    Usage(String),
    #[error("failed to read fixture directory `{path}`")]
    ReadFixtureDir {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to read fixture file `{path}`")]
    ReadFixtureFile {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("fixture directory `{0}` does not contain any CSV files")]
    EmptyFixtureDir(String),
    #[error("unsupported source `{0}`; supported value is `mlit`")]
    UnsupportedSource(String),
    #[error("database operation failed")]
    Sqlx(#[from] sqlx::Error),
    #[error("MLIT fixture parsing failed")]
    Parse(#[from] urbanlens_importer::mlit::MlitParseError),
    #[error("MLIT N03 boundary fixture parsing failed")]
    BoundaryParse(#[from] urbanlens_importer::boundaries::BoundaryParseError),
    #[error("import persistence failed")]
    Persistence(#[from] PersistenceError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ImportTransactionsOptions {
    source: String,
    prefecture: String,
    period: String,
    fixture_dir: PathBuf,
    normalization_version: String,
    database_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ImportWardBoundariesOptions {
    source: String,
    fixture_path: PathBuf,
    normalization_version: String,
    boundary_version: String,
    database_url: String,
}

impl Default for ImportTransactionsOptions {
    fn default() -> Self {
        Self {
            source: "mlit".to_owned(),
            prefecture: DEFAULT_PREFECTURE.to_owned(),
            period: DEFAULT_PERIOD.to_owned(),
            fixture_dir: PathBuf::from(DEFAULT_FIXTURE_DIR),
            normalization_version: MLIT_NORMALIZATION_VERSION.to_owned(),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_owned()),
        }
    }
}

impl Default for ImportWardBoundariesOptions {
    fn default() -> Self {
        Self {
            source: "mlit-n03".to_owned(),
            fixture_path: PathBuf::from(DEFAULT_BOUNDARY_FIXTURE),
            normalization_version: MLIT_N03_BOUNDARY_NORMALIZATION_VERSION.to_owned(),
            boundary_version: BOUNDARY_SOURCE_VERSION.to_owned(),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
struct FixtureArtifact {
    path: PathBuf,
    filename: String,
    bytes: Vec<u8>,
    sha256: String,
}

#[derive(Debug, Clone, Copy)]
struct ArtifactImportSummary {
    counters: ImportCounters,
}

#[derive(Debug, Default, Clone, Copy)]
struct ImportSummary {
    artifacts: usize,
    counters: ImportCounters,
}

#[derive(Debug, Default, Clone, Copy)]
struct BoundaryImportSummary {
    source_features: usize,
    wards: usize,
    counters: ImportCounters,
}

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("import failed: {error}");
        let mut source = error.source();
        while let Some(error) = source {
            eprintln!("caused by: {error}");
            source = error.source();
        }
        std::process::exit(1);
    }
}

async fn run() -> Result<(), CliError> {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        return Err(CliError::Usage(usage()));
    };

    match command.as_str() {
        "import-transactions" => {
            let options = parse_import_transactions_args(args)?;
            let summary = import_transactions(&options).await?;
            print_summary(&options, summary);
            Ok(())
        }
        "import-ward-boundaries" => {
            let options = parse_import_ward_boundaries_args(args)?;
            let summary = import_ward_boundaries(&options).await?;
            print_boundary_summary(&options, summary);
            Ok(())
        }
        "--help" | "-h" | "help" => {
            println!("{}", usage());
            Ok(())
        }
        other => Err(CliError::Usage(format!(
            "unknown command `{other}`\n\n{}",
            usage()
        ))),
    }
}

fn parse_import_transactions_args<I>(args: I) -> Result<ImportTransactionsOptions, CliError>
where
    I: IntoIterator<Item = String>,
{
    let mut options = ImportTransactionsOptions::default();
    let mut args = args.into_iter();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--source" => options.source = next_value(&mut args, "--source")?,
            "--prefecture" => options.prefecture = next_value(&mut args, "--prefecture")?,
            "--period" => options.period = next_value(&mut args, "--period")?,
            "--fixture-dir" => {
                options.fixture_dir = PathBuf::from(next_value(&mut args, "--fixture-dir")?);
            }
            "--normalization-version" => {
                options.normalization_version = next_value(&mut args, "--normalization-version")?;
            }
            "--database-url" => options.database_url = next_value(&mut args, "--database-url")?,
            "--help" | "-h" => return Err(CliError::Usage(usage())),
            unknown => {
                return Err(CliError::Usage(format!(
                    "unknown import-transactions option `{unknown}`\n\n{}",
                    usage()
                )));
            }
        }
    }

    if options.source != "mlit" {
        return Err(CliError::UnsupportedSource(options.source));
    }

    Ok(options)
}

fn parse_import_ward_boundaries_args<I>(args: I) -> Result<ImportWardBoundariesOptions, CliError>
where
    I: IntoIterator<Item = String>,
{
    let mut options = ImportWardBoundariesOptions::default();
    let mut args = args.into_iter();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--source" => options.source = next_value(&mut args, "--source")?,
            "--fixture-path" => {
                options.fixture_path = PathBuf::from(next_value(&mut args, "--fixture-path")?);
            }
            "--normalization-version" => {
                options.normalization_version = next_value(&mut args, "--normalization-version")?;
            }
            "--boundary-version" => {
                options.boundary_version = next_value(&mut args, "--boundary-version")?;
            }
            "--database-url" => options.database_url = next_value(&mut args, "--database-url")?,
            "--help" | "-h" => return Err(CliError::Usage(usage())),
            unknown => {
                return Err(CliError::Usage(format!(
                    "unknown import-ward-boundaries option `{unknown}`\n\n{}",
                    usage()
                )));
            }
        }
    }

    if options.source != "mlit-n03" {
        return Err(CliError::UnsupportedSource(options.source));
    }

    Ok(options)
}

fn next_value<I>(args: &mut I, flag: &str) -> Result<String, CliError>
where
    I: Iterator<Item = String>,
{
    args.next()
        .ok_or_else(|| CliError::Usage(format!("missing value for `{flag}`\n\n{}", usage())))
}

async fn import_transactions(
    options: &ImportTransactionsOptions,
) -> Result<ImportSummary, CliError> {
    let artifacts = discover_fixture_artifacts(&options.fixture_dir)?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&options.database_url)
        .await?;

    let data_source = upsert_data_source(&pool, &DataSourceInput::mlit_reinfolib()).await?;
    let mut summary = ImportSummary::default();

    for artifact in artifacts {
        let artifact_summary = import_artifact(&pool, options, data_source.id, &artifact).await?;
        summary.artifacts += 1;
        add_counters(&mut summary.counters, artifact_summary.counters);
    }

    Ok(summary)
}

#[allow(clippy::too_many_lines)]
async fn import_ward_boundaries(
    options: &ImportWardBoundariesOptions,
) -> Result<BoundaryImportSummary, CliError> {
    let bytes = fs::read(&options.fixture_path).map_err(|source| CliError::ReadFixtureFile {
        path: options.fixture_path.display().to_string(),
        source,
    })?;
    let sha256 = sha256_hex(&bytes);
    let filename = options
        .fixture_path
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("unknown.geojson")
        .to_owned();
    let (features, wards) = parse_mlit_n03_tokyo_wards(&bytes)?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&options.database_url)
        .await?;

    let data_source =
        upsert_data_source(&pool, &DataSourceInput::mlit_n03_administrative_areas()).await?;
    let dataset = upsert_dataset(
        &pool,
        &DatasetInput {
            source_id: data_source.id,
            source_dataset_name: format!("MLIT N03 Tokyo ward boundary fixture {filename}"),
            retrieval_method: "fixture_geojson".to_owned(),
            retrieval_query: json!({
                "source": options.source,
                "prefecture": "13",
                "area_type": "ward",
                "fixture_file": filename,
                "fixture_path": options.fixture_path.display().to_string(),
            }),
            source_version: Some(options.boundary_version.clone()),
            retrieved_at: BOUNDARY_RETRIEVED_AT.to_owned(),
            artifact_sha256: sha256,
            format: "geojson; encoding=utf-8; crs=JGD2011/SRID4326-compatible".to_owned(),
            record_count: features.len(),
        },
    )
    .await?;

    let import_run = start_import_run(&pool, dataset.id, &options.normalization_version).await?;
    let mut counters = ImportCounters {
        records_received: i64::try_from(features.len()).unwrap_or(i64::MAX),
        ..ImportCounters::default()
    };

    for feature in &features {
        match persist_boundary_raw_feature(&pool, dataset.id, import_run.id, feature).await {
            Ok(outcome) if outcome.inserted => {}
            Ok(_) => counters.duplicates_skipped += 1,
            Err(error) => {
                fail_import_run(&pool, import_run.id, "boundary_raw_record_error", counters)
                    .await?;
                return Err(CliError::Persistence(error));
            }
        }
    }

    for ward in &wards {
        match upsert_ward_boundary(
            &pool,
            data_source.id,
            dataset.id,
            import_run.id,
            &options.boundary_version,
            ward,
        )
        .await
        {
            Ok(boundary) => match boundary.write {
                urbanlens_importer::persistence::BoundaryWrite::Inserted => {
                    counters.records_imported += 1;
                }
                urbanlens_importer::persistence::BoundaryWrite::Updated => {
                    counters.records_updated += 1;
                }
            },
            Err(error) => {
                fail_import_run(&pool, import_run.id, "boundary_upsert_error", counters).await?;
                return Err(CliError::Persistence(error));
            }
        }
    }

    complete_run(&pool, import_run.id, counters).await?;

    println!(
        "artifact={} import_run={} status={} source_features={} wards={} received={} imported={} updated={} duplicates_skipped={} rejected={} warning_records={}",
        filename,
        import_run.id,
        counters.terminal_status(),
        features.len(),
        wards.len(),
        counters.records_received,
        counters.records_imported,
        counters.records_updated,
        counters.duplicates_skipped,
        counters.records_rejected,
        counters.warning_records
    );

    Ok(BoundaryImportSummary {
        source_features: features.len(),
        wards: wards.len(),
        counters,
    })
}

fn discover_fixture_artifacts(fixture_dir: &Path) -> Result<Vec<FixtureArtifact>, CliError> {
    let entries = fs::read_dir(fixture_dir).map_err(|source| CliError::ReadFixtureDir {
        path: fixture_dir.display().to_string(),
        source,
    })?;

    let mut paths = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|source| CliError::ReadFixtureDir {
            path: fixture_dir.display().to_string(),
            source,
        })?;
        let path = entry.path();
        if path.extension().and_then(OsStr::to_str) == Some("csv") {
            paths.push(path);
        }
    }
    paths.sort();

    if paths.is_empty() {
        return Err(CliError::EmptyFixtureDir(fixture_dir.display().to_string()));
    }

    paths
        .into_iter()
        .map(|path| {
            let bytes = fs::read(&path).map_err(|source| CliError::ReadFixtureFile {
                path: path.display().to_string(),
                source,
            })?;
            let filename = path
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap_or("unknown.csv")
                .to_owned();
            let sha256 = sha256_hex(&bytes);
            Ok(FixtureArtifact {
                path,
                filename,
                bytes,
                sha256,
            })
        })
        .collect()
}

async fn import_artifact(
    pool: &PgPool,
    options: &ImportTransactionsOptions,
    source_id: uuid::Uuid,
    artifact: &FixtureArtifact,
) -> Result<ArtifactImportSummary, CliError> {
    let rows = parse_mlit_csv_bytes(&artifact.bytes)?;
    let dataset = upsert_dataset(
        pool,
        &DatasetInput {
            source_id,
            source_dataset_name: format!("MLIT transaction price fixture {}", artifact.filename),
            retrieval_method: "fixture_csv".to_owned(),
            retrieval_query: json!({
                "source": options.source,
                "prefecture": options.prefecture,
                "period": options.period,
                "fixture_file": artifact.filename,
                "fixture_path": artifact.path.display().to_string(),
            }),
            source_version: Some(options.period.clone()),
            retrieved_at: FIXTURE_RETRIEVED_AT.to_owned(),
            artifact_sha256: artifact.sha256.clone(),
            format: "csv; encoding=cp932".to_owned(),
            record_count: rows.len(),
        },
    )
    .await?;

    let import_run = start_import_run(pool, dataset.id, &options.normalization_version).await?;
    let mut outcomes = Vec::with_capacity(rows.len());

    for row in rows {
        let normalized = normalize_source_row(&row);
        match persist_normalized_record(
            pool,
            dataset.id,
            import_run.id,
            &row,
            &normalized,
            &options.normalization_version,
        )
        .await
        {
            Ok(outcome) => outcomes.push(outcome),
            Err(error) => {
                let counters = counters_from_outcomes(&outcomes);
                fail_import_run(pool, import_run.id, "persistence_error", counters).await?;
                return Err(CliError::Persistence(error));
            }
        }
    }

    let counters = counters_from_outcomes(&outcomes);
    complete_run(pool, import_run.id, counters).await?;

    println!(
        "artifact={} import_run={} status={} received={} imported={} updated={} duplicates_skipped={} rejected={} warning_records={}",
        artifact.filename,
        import_run.id,
        counters.terminal_status(),
        counters.records_received,
        counters.records_imported,
        counters.records_updated,
        counters.duplicates_skipped,
        counters.records_rejected,
        counters.warning_records
    );

    Ok(ArtifactImportSummary { counters })
}

async fn complete_run(
    pool: &PgPool,
    import_run_id: uuid::Uuid,
    counters: ImportCounters,
) -> Result<(), CliError> {
    urbanlens_importer::persistence::complete_import_run(pool, import_run_id, counters).await?;
    Ok(())
}

fn print_summary(options: &ImportTransactionsOptions, summary: ImportSummary) {
    println!(
        "summary source={} prefecture={} period={} artifacts={} normalization_version={} received={} imported={} updated={} duplicates_skipped={} rejected={} warning_records={} status={}",
        options.source,
        options.prefecture,
        options.period,
        summary.artifacts,
        options.normalization_version,
        summary.counters.records_received,
        summary.counters.records_imported,
        summary.counters.records_updated,
        summary.counters.duplicates_skipped,
        summary.counters.records_rejected,
        summary.counters.warning_records,
        summary.counters.terminal_status()
    );
}

fn print_boundary_summary(options: &ImportWardBoundariesOptions, summary: BoundaryImportSummary) {
    println!(
        "summary source={} boundary_version={} source_features={} wards={} normalization_version={} received={} imported={} updated={} duplicates_skipped={} rejected={} warning_records={} status={}",
        options.source,
        options.boundary_version,
        summary.source_features,
        summary.wards,
        options.normalization_version,
        summary.counters.records_received,
        summary.counters.records_imported,
        summary.counters.records_updated,
        summary.counters.duplicates_skipped,
        summary.counters.records_rejected,
        summary.counters.warning_records,
        summary.counters.terminal_status()
    );
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("{digest:x}")
}

fn usage() -> String {
    let mut usage = String::new();
    writeln!(usage, "Usage: urbanlens-importer <command> [options]")
        .expect("write to String succeeds");
    writeln!(usage).expect("write to String succeeds");
    writeln!(usage, "Commands:").expect("write to String succeeds");
    writeln!(usage, "  import-transactions").expect("write to String succeeds");
    writeln!(usage, "  import-ward-boundaries").expect("write to String succeeds");
    writeln!(usage).expect("write to String succeeds");
    writeln!(usage, "Transaction options:").expect("write to String succeeds");
    writeln!(usage, "  --source mlit").expect("write to String succeeds");
    writeln!(usage, "  --prefecture 13").expect("write to String succeeds");
    writeln!(usage, "  --period 2024Q4").expect("write to String succeeds");
    writeln!(
        usage,
        "  --fixture-dir workers/importer/fixtures/transactions"
    )
    .expect("write to String succeeds");
    writeln!(
        usage,
        "  --normalization-version {MLIT_NORMALIZATION_VERSION}"
    )
    .expect("write to String succeeds");
    writeln!(
        usage,
        "  --database-url postgres://urbanlens:urbanlens_dev@localhost:5432/urbanlens"
    )
    .expect("write to String succeeds");
    writeln!(usage).expect("write to String succeeds");
    writeln!(usage, "Ward boundary options:").expect("write to String succeeds");
    writeln!(usage, "  --source mlit-n03").expect("write to String succeeds");
    writeln!(
        usage,
        "  --fixture-path workers/importer/fixtures/boundaries/mlit-n03-tokyo-23-wards-2023.geojson"
    )
    .expect("write to String succeeds");
    writeln!(
        usage,
        "  --normalization-version {MLIT_N03_BOUNDARY_NORMALIZATION_VERSION}"
    )
    .expect("write to String succeeds");
    writeln!(usage, "  --boundary-version {BOUNDARY_SOURCE_VERSION}")
        .expect("write to String succeeds");
    writeln!(
        usage,
        "  --database-url postgres://urbanlens:urbanlens_dev@localhost:5432/urbanlens"
    )
    .expect("write to String succeeds");
    usage
}

fn add_counters(total: &mut ImportCounters, next: ImportCounters) {
    total.records_received += next.records_received;
    total.records_imported += next.records_imported;
    total.records_updated += next.records_updated;
    total.duplicates_skipped += next.duplicates_skipped;
    total.records_rejected += next.records_rejected;
    total.warning_records += next.warning_records;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_import_transactions_defaults_and_overrides() {
        let options = parse_import_transactions_args([
            "--source".to_owned(),
            "mlit".to_owned(),
            "--prefecture".to_owned(),
            "13".to_owned(),
            "--period".to_owned(),
            "2024Q4".to_owned(),
            "--fixture-dir".to_owned(),
            "fixtures".to_owned(),
            "--normalization-version".to_owned(),
            "test-version".to_owned(),
            "--database-url".to_owned(),
            "postgres://example".to_owned(),
        ])
        .expect("options parse");

        assert_eq!(options.source, "mlit");
        assert_eq!(options.prefecture, "13");
        assert_eq!(options.period, "2024Q4");
        assert_eq!(options.fixture_dir, PathBuf::from("fixtures"));
        assert_eq!(options.normalization_version, "test-version");
        assert_eq!(options.database_url, "postgres://example");
    }

    #[test]
    fn rejects_unknown_source() {
        let error =
            parse_import_transactions_args(["--source".to_owned(), "private-listings".to_owned()])
                .expect_err("unknown source is rejected");

        assert!(
            matches!(error, CliError::UnsupportedSource(source) if source == "private-listings")
        );
    }

    #[test]
    fn parses_import_ward_boundaries_defaults_and_overrides() {
        let options = parse_import_ward_boundaries_args([
            "--source".to_owned(),
            "mlit-n03".to_owned(),
            "--fixture-path".to_owned(),
            "boundaries.geojson".to_owned(),
            "--normalization-version".to_owned(),
            "boundary-test-version".to_owned(),
            "--boundary-version".to_owned(),
            "2023-01-01".to_owned(),
            "--database-url".to_owned(),
            "postgres://example".to_owned(),
        ])
        .expect("options parse");

        assert_eq!(options.source, "mlit-n03");
        assert_eq!(options.fixture_path, PathBuf::from("boundaries.geojson"));
        assert_eq!(options.normalization_version, "boundary-test-version");
        assert_eq!(options.boundary_version, "2023-01-01");
        assert_eq!(options.database_url, "postgres://example");
    }
}
