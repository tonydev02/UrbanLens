use std::{
    env,
    error::Error,
    fmt::{self, Display, Formatter},
    future::{Ready, ready},
    time::Duration,
};

use actix_cors::Cors;
use actix_web::{
    Error as ActixError, HttpMessage, HttpResponse,
    body::MessageBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    error::ErrorInternalServerError,
    http::header::{self, HeaderName, HeaderValue},
    middleware::Logger,
    web,
};
use async_graphql::{
    Context, EmptyMutation, EmptySubscription, InputObject, Object, Result, Schema, SimpleObject,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use futures_util::future::LocalBoxFuture;
use serde::Serialize;
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

const DEFAULT_API_HOST: &str = "0.0.0.0";
const DEFAULT_API_PORT: u16 = 8080;
const DEFAULT_CORS_ORIGIN: &str = "http://localhost:3000";
const DEFAULT_MAX_DB_CONNECTIONS: u32 = 5;
const DEFAULT_GRAPHQL_LIMIT: i64 = 25;
const MAX_GRAPHQL_LIMIT: i64 = 100;
const REQUEST_ID_HEADER: &str = "x-request-id";

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub database_max_connections: u32,
    pub cors_allowed_origins: Vec<String>,
}

impl ApiConfig {
    /// Load API configuration from process environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error when a required variable is missing or when a configured
    /// value cannot be parsed safely.
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_reader(|key| env::var(key).ok())
    }

    /// Build API configuration from an arbitrary key-value reader.
    ///
    /// # Errors
    ///
    /// Returns an error when `DATABASE_URL` is missing, a numeric value is
    /// invalid, or CORS origins are empty, wildcarded, or non-HTTP origins.
    pub fn from_reader<F>(mut read: F) -> Result<Self, ConfigError>
    where
        F: FnMut(&str) -> Option<String>,
    {
        let host = read("API_HOST").unwrap_or_else(|| DEFAULT_API_HOST.to_owned());
        let port = parse_port(read("API_PORT"))?;
        let database_url = read_required(&mut read, "DATABASE_URL")?;
        let database_max_connections = parse_positive_u32(
            read("API_DATABASE_MAX_CONNECTIONS"),
            "API_DATABASE_MAX_CONNECTIONS",
            DEFAULT_MAX_DB_CONNECTIONS,
        )?;
        let cors_allowed_origins =
            parse_cors_origins(read("CORS_ALLOWED_ORIGINS"), read("WEB_ORIGIN"))?;

        Ok(Self {
            host,
            port,
            database_url,
            database_max_connections,
            cors_allowed_origins,
        })
    }

    #[must_use]
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError {
    variable: &'static str,
    message: String,
}

impl ConfigError {
    fn new(variable: &'static str, message: impl Into<String>) -> Self {
        Self {
            variable,
            message: message.into(),
        }
    }
}

impl Display for ConfigError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.variable, self.message)
    }
}

impl Error for ConfigError {}

/// Create the API `PostgreSQL` connection pool.
///
/// # Errors
///
/// Returns an error when the configured database URL cannot be reached or `SQLx`
/// cannot establish the initial pool connection.
pub async fn create_pool(config: &ApiConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database_url)
        .await
}

#[must_use]
pub fn build_schema(pool: PgPool) -> AppSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish()
}

pub fn bounded_cors(config: &ApiConfig) -> Cors {
    config.cors_allowed_origins.iter().fold(
        Cors::default()
            .allowed_methods(["GET", "POST", "OPTIONS"])
            .allowed_headers([
                header::ACCEPT,
                header::CONTENT_TYPE,
                HeaderName::from_static(REQUEST_ID_HEADER),
            ])
            .expose_headers([HeaderName::from_static(REQUEST_ID_HEADER)])
            .max_age(3600),
        |cors, origin| cors.allowed_origin(origin),
    )
}

#[must_use]
pub fn request_logger() -> Logger {
    Logger::new(r#"%{x-request-id}o "%r" %s %b %Dms"#)
}

pub fn configure_routes(config: &mut web::ServiceConfig) {
    config
        .route("/health", web::get().to(health))
        .route("/ready", web::get().to(readiness))
        .route("/graphql", web::post().to(graphql));
}

#[derive(Clone, Copy, Debug)]
pub struct RequestId;

impl<S, B> Transform<S, ServiceRequest> for RequestId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Error = ActixError;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type InitError = ();
    type Response = ServiceResponse<B>;
    type Transform = RequestIdMiddleware<S>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestIdMiddleware { service }))
    }
}

pub struct RequestIdMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = ServiceResponse<B>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let request_id = request
            .headers()
            .get(REQUEST_ID_HEADER)
            .and_then(|value| value.to_str().ok())
            .filter(|value| !value.trim().is_empty())
            .map_or_else(|| Uuid::new_v4().to_string(), ToOwned::to_owned);

        request.extensions_mut().insert(request_id.clone());
        let response = self.service.call(request);

        Box::pin(async move {
            let mut response = response.await?;
            let header_value =
                HeaderValue::from_str(&request_id).map_err(ErrorInternalServerError)?;
            response
                .headers_mut()
                .insert(HeaderName::from_static(REQUEST_ID_HEADER), header_value);
            Ok(response)
        })
    }
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse { status: "ok" })
}

#[derive(Debug, Serialize)]
struct ReadinessResponse {
    status: &'static str,
    database_reachable: bool,
    migrations_applied: bool,
}

async fn readiness(pool: web::Data<PgPool>) -> HttpResponse {
    let state = readiness_state(pool.get_ref()).await;
    let response = ReadinessResponse {
        status: state.status(),
        database_reachable: state.database_reachable,
        migrations_applied: state.migrations_applied,
    };

    if state.is_ready() {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::ServiceUnavailable().json(response)
    }
}

async fn graphql(schema: web::Data<AppSchema>, request: GraphQLRequest) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

#[derive(Clone, Copy, Debug)]
struct ReadinessState {
    database_reachable: bool,
    migrations_applied: bool,
}

impl ReadinessState {
    const fn is_ready(self) -> bool {
        self.database_reachable && self.migrations_applied
    }

    const fn status(self) -> &'static str {
        if self.is_ready() {
            "ready"
        } else {
            "not_ready"
        }
    }
}

async fn readiness_state(pool: &PgPool) -> ReadinessState {
    let database_reachable = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(pool)
        .await
        .is_ok();

    let migrations_applied = if database_reachable {
        sqlx::query_scalar::<_, bool>(
            "SELECT COUNT(*) FILTER (WHERE success = true) > 0
                AND COUNT(*) FILTER (WHERE success = false) = 0
             FROM _sqlx_migrations",
        )
        .fetch_one(pool)
        .await
        .unwrap_or(false)
    } else {
        false
    };

    ReadinessState {
        database_reachable,
        migrations_applied,
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn connectivity(&self, context: &Context<'_>) -> Connectivity {
        let pool = context.data_unchecked::<PgPool>();
        let state = readiness_state(pool).await;

        Connectivity {
            service: "urbanlens-api",
            status: state.status().to_owned(),
            database_reachable: state.database_reachable,
            migrations_applied: state.migrations_applied,
        }
    }

    async fn transaction_observations(
        &self,
        context: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
        import_run_id: Option<String>,
        dataset_id: Option<String>,
        municipality_code: Option<String>,
    ) -> Result<Vec<TransactionObservation>> {
        let pool = context.data_unchecked::<PgPool>();
        let pagination = Pagination::from_args(limit, offset);
        let import_run_id = parse_optional_uuid(import_run_id.as_deref(), "importRunId")?;
        let dataset_id = parse_optional_uuid(dataset_id.as_deref(), "datasetId")?;

        sqlx::query_as::<_, TransactionObservation>(
            r"
            SELECT
                observation.id::text AS id,
                observation.raw_record_id::text AS raw_record_id,
                observation.import_run_id::text AS import_run_id,
                observation.dataset_id::text AS dataset_id,
                observation.source_record_hash,
                observation.normalization_version,
                observation.validation_status,
                observation.asset_type,
                observation.raw_asset_type,
                observation.price_category,
                observation.transaction_year::integer AS transaction_year,
                observation.transaction_quarter::integer AS transaction_quarter,
                observation.trade_price_jpy,
                observation.source_unit_price_jpy_per_m2,
                observation.area_m2::text AS area_m2,
                observation.total_floor_area_m2::text AS total_floor_area_m2,
                observation.total_floor_area_is_lower_bound,
                observation.municipality_code,
                observation.prefecture_name,
                observation.municipality_name,
                observation.district_name,
                observation.nearest_station_name,
                observation.station_walk_minutes,
                location.location_precision,
                location.source_location_label
            FROM transaction_observations observation
            LEFT JOIN transaction_location_contexts location
                ON location.transaction_observation_id = observation.id
            WHERE ($3::uuid IS NULL OR observation.import_run_id = $3)
                AND ($4::uuid IS NULL OR observation.dataset_id = $4)
                AND ($5::text IS NULL OR observation.municipality_code = $5)
            ORDER BY observation.created_at DESC, observation.id DESC
            LIMIT $1 OFFSET $2
            ",
        )
        .bind(pagination.limit)
        .bind(pagination.offset)
        .bind(import_run_id)
        .bind(dataset_id)
        .bind(municipality_code)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn import_runs(
        &self,
        context: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
        status: Option<String>,
        dataset_id: Option<String>,
    ) -> Result<Vec<ImportRun>> {
        let pool = context.data_unchecked::<PgPool>();
        let pagination = Pagination::from_args(limit, offset);
        let dataset_id = parse_optional_uuid(dataset_id.as_deref(), "datasetId")?;

        sqlx::query_as::<_, ImportRun>(
            r"
            SELECT
                import_run.id::text AS id,
                import_run.dataset_id::text AS dataset_id,
                to_char(import_run.started_at AT TIME ZONE 'UTC', 'YYYY-MM-DD HH24:MI:SS UTC') AS started_at,
                to_char(import_run.completed_at AT TIME ZONE 'UTC', 'YYYY-MM-DD HH24:MI:SS UTC') AS completed_at,
                import_run.status,
                import_run.normalization_version,
                import_run.records_received,
                import_run.records_imported,
                import_run.records_updated,
                import_run.duplicates_skipped,
                import_run.records_rejected,
                import_run.warning_records,
                import_run.error_kind,
                dataset.source_dataset_name,
                data_source.name AS data_source_name
            FROM import_runs import_run
            JOIN datasets dataset ON dataset.id = import_run.dataset_id
            JOIN data_sources data_source ON data_source.id = dataset.source_id
            WHERE ($3::text IS NULL OR import_run.status = $3)
                AND ($4::uuid IS NULL OR import_run.dataset_id = $4)
            ORDER BY import_run.started_at DESC, import_run.id DESC
            LIMIT $1 OFFSET $2
            ",
        )
        .bind(pagination.limit)
        .bind(pagination.offset)
        .bind(status)
        .bind(dataset_id)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn validation_issues(
        &self,
        context: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
        filter: Option<ValidationIssueFilter>,
    ) -> Result<Vec<ValidationIssue>> {
        let pool = context.data_unchecked::<PgPool>();
        let pagination = Pagination::from_args(limit, offset);
        let filter = filter.unwrap_or_default();
        let import_run_id = parse_optional_uuid(filter.import_run_id.as_deref(), "importRunId")?;
        let raw_record_id = parse_optional_uuid(filter.raw_record_id.as_deref(), "rawRecordId")?;
        let transaction_observation_id = parse_optional_uuid(
            filter.transaction_observation_id.as_deref(),
            "transactionObservationId",
        )?;

        sqlx::query_as::<_, ValidationIssue>(
            r"
            SELECT
                issue.id::text AS id,
                issue.import_run_id::text AS import_run_id,
                issue.raw_record_id::text AS raw_record_id,
                issue.transaction_observation_id::text AS transaction_observation_id,
                issue.issue_code,
                issue.severity,
                issue.field_name,
                issue.raw_value_summary,
                issue.message,
                issue.disposition,
                to_char(issue.created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD HH24:MI:SS UTC') AS created_at
            FROM validation_issues issue
            WHERE ($3::uuid IS NULL OR issue.import_run_id = $3)
                AND ($4::uuid IS NULL OR issue.raw_record_id = $4)
                AND ($5::uuid IS NULL OR issue.transaction_observation_id = $5)
                AND ($6::text IS NULL OR issue.severity = $6)
                AND ($7::text IS NULL OR issue.issue_code = $7)
            ORDER BY issue.created_at DESC, issue.id DESC
            LIMIT $1 OFFSET $2
            ",
        )
        .bind(pagination.limit)
        .bind(pagination.offset)
        .bind(import_run_id)
        .bind(raw_record_id)
        .bind(transaction_observation_id)
        .bind(filter.severity)
        .bind(filter.issue_code)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn data_sources(
        &self,
        context: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<DataSource>> {
        let pool = context.data_unchecked::<PgPool>();
        let pagination = Pagination::from_args(limit, offset);

        sqlx::query_as::<_, DataSource>(
            r"
            SELECT
                source.id::text AS id,
                source.name,
                source.publisher,
                source.source_url,
                source.license_url,
                to_char(source.metadata_verified_at AT TIME ZONE 'UTC', 'YYYY-MM-DD HH24:MI:SS UTC') AS metadata_verified_at,
                COUNT(dataset.id) AS dataset_count
            FROM data_sources source
            LEFT JOIN datasets dataset ON dataset.source_id = source.id
            GROUP BY source.id
            ORDER BY source.name ASC, source.id ASC
            LIMIT $1 OFFSET $2
            ",
        )
        .bind(pagination.limit)
        .bind(pagination.offset)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn transaction_observation_provenance(
        &self,
        context: &Context<'_>,
        observation_id: String,
    ) -> Result<Option<TransactionObservationProvenance>> {
        let pool = context.data_unchecked::<PgPool>();
        let observation_id = parse_uuid(&observation_id, "observationId")?;

        sqlx::query_as::<_, TransactionObservationProvenance>(
            r"
            SELECT
                observation.id::text AS observation_id,
                observation.raw_record_id::text AS raw_record_id,
                raw_record.source_position,
                raw_record.external_id,
                raw_record.payload_sha256,
                raw_record.validation_status AS raw_record_validation_status,
                observation.import_run_id::text AS import_run_id,
                import_run.status AS import_run_status,
                import_run.normalization_version,
                observation.dataset_id::text AS dataset_id,
                dataset.source_dataset_name,
                dataset.retrieval_method,
                dataset.retrieval_query::text AS retrieval_query_json,
                dataset.source_version,
                dataset.artifact_sha256,
                dataset.format AS dataset_format,
                dataset.record_count AS dataset_record_count,
                to_char(dataset.retrieved_at AT TIME ZONE 'UTC', 'YYYY-MM-DD HH24:MI:SS UTC') AS dataset_retrieved_at,
                source.id::text AS data_source_id,
                source.name AS data_source_name,
                source.publisher AS data_source_publisher,
                source.source_url,
                source.license_url
            FROM transaction_observations observation
            JOIN raw_records raw_record ON raw_record.id = observation.raw_record_id
            JOIN import_runs import_run ON import_run.id = observation.import_run_id
            JOIN datasets dataset ON dataset.id = observation.dataset_id
            JOIN data_sources source ON source.id = dataset.source_id
            WHERE observation.id = $1
            ",
        )
        .bind(observation_id)
        .fetch_optional(pool)
        .await
        .map_err(Into::into)
    }
}

#[derive(SimpleObject)]
struct Connectivity {
    service: &'static str,
    status: String,
    database_reachable: bool,
    migrations_applied: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pagination {
    limit: i64,
    offset: i64,
}

impl Pagination {
    fn from_args(limit: Option<i32>, offset: Option<i32>) -> Self {
        let limit = limit
            .map_or(DEFAULT_GRAPHQL_LIMIT, i64::from)
            .clamp(1, MAX_GRAPHQL_LIMIT);
        let offset = offset.map_or(0, i64::from).max(0);

        Self { limit, offset }
    }
}

#[derive(Debug, FromRow, SimpleObject)]
struct TransactionObservation {
    id: String,
    raw_record_id: String,
    import_run_id: String,
    dataset_id: String,
    source_record_hash: String,
    normalization_version: String,
    validation_status: String,
    asset_type: String,
    raw_asset_type: String,
    price_category: String,
    transaction_year: i32,
    transaction_quarter: i32,
    trade_price_jpy: Option<i64>,
    source_unit_price_jpy_per_m2: Option<i64>,
    area_m2: Option<String>,
    total_floor_area_m2: Option<String>,
    total_floor_area_is_lower_bound: bool,
    municipality_code: String,
    prefecture_name: String,
    municipality_name: String,
    district_name: Option<String>,
    nearest_station_name: Option<String>,
    station_walk_minutes: Option<i32>,
    location_precision: Option<String>,
    source_location_label: Option<String>,
}

#[derive(Debug, FromRow, SimpleObject)]
struct ImportRun {
    id: String,
    dataset_id: String,
    started_at: String,
    completed_at: Option<String>,
    status: String,
    normalization_version: String,
    records_received: i64,
    records_imported: i64,
    records_updated: i64,
    duplicates_skipped: i64,
    records_rejected: i64,
    warning_records: i64,
    error_kind: Option<String>,
    source_dataset_name: String,
    data_source_name: String,
}

#[derive(Debug, FromRow, SimpleObject)]
struct ValidationIssue {
    id: String,
    import_run_id: String,
    raw_record_id: Option<String>,
    transaction_observation_id: Option<String>,
    issue_code: String,
    severity: String,
    field_name: Option<String>,
    raw_value_summary: Option<String>,
    message: String,
    disposition: String,
    created_at: String,
}

#[derive(Debug, Default, InputObject)]
struct ValidationIssueFilter {
    import_run_id: Option<String>,
    raw_record_id: Option<String>,
    transaction_observation_id: Option<String>,
    severity: Option<String>,
    issue_code: Option<String>,
}

#[derive(Debug, FromRow, SimpleObject)]
struct DataSource {
    id: String,
    name: String,
    publisher: String,
    source_url: String,
    license_url: String,
    metadata_verified_at: String,
    dataset_count: i64,
}

#[derive(Debug, FromRow, SimpleObject)]
struct TransactionObservationProvenance {
    observation_id: String,
    raw_record_id: String,
    source_position: i64,
    external_id: Option<String>,
    payload_sha256: String,
    raw_record_validation_status: String,
    import_run_id: String,
    import_run_status: String,
    normalization_version: String,
    dataset_id: String,
    source_dataset_name: String,
    retrieval_method: String,
    retrieval_query_json: String,
    source_version: Option<String>,
    artifact_sha256: String,
    dataset_format: String,
    dataset_record_count: i64,
    dataset_retrieved_at: String,
    data_source_id: String,
    data_source_name: String,
    data_source_publisher: String,
    source_url: String,
    license_url: String,
}

fn parse_optional_uuid(value: Option<&str>, argument: &'static str) -> Result<Option<Uuid>> {
    value.map(|raw| parse_uuid(raw, argument)).transpose()
}

fn parse_uuid(value: &str, argument: &'static str) -> Result<Uuid> {
    Uuid::parse_str(value).map_err(|_| format!("{argument} must be a valid UUID").into())
}

fn parse_port(value: Option<String>) -> Result<u16, ConfigError> {
    match value {
        Some(raw) => raw
            .parse::<u16>()
            .map_err(|_| ConfigError::new("API_PORT", "must be a TCP port between 0 and 65535")),
        None => Ok(DEFAULT_API_PORT),
    }
}

fn parse_positive_u32(
    value: Option<String>,
    variable: &'static str,
    default: u32,
) -> Result<u32, ConfigError> {
    match value {
        Some(raw) => {
            let parsed = raw
                .parse::<u32>()
                .map_err(|_| ConfigError::new(variable, "must be a positive integer"))?;
            if parsed == 0 {
                Err(ConfigError::new(variable, "must be greater than zero"))
            } else {
                Ok(parsed)
            }
        }
        None => Ok(default),
    }
}

fn parse_cors_origins(
    cors_allowed_origins: Option<String>,
    web_origin: Option<String>,
) -> Result<Vec<String>, ConfigError> {
    let raw = cors_allowed_origins
        .or(web_origin)
        .unwrap_or_else(|| DEFAULT_CORS_ORIGIN.to_owned());
    let origins = raw
        .split(',')
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .map(validate_origin)
        .collect::<Result<Vec<_>, _>>()?;

    if origins.is_empty() {
        Err(ConfigError::new(
            "CORS_ALLOWED_ORIGINS",
            "must include at least one HTTP or HTTPS origin",
        ))
    } else {
        Ok(origins)
    }
}

fn validate_origin(origin: &str) -> Result<String, ConfigError> {
    if origin == "*" {
        return Err(ConfigError::new(
            "CORS_ALLOWED_ORIGINS",
            "wildcard origins are not allowed",
        ));
    }

    if origin.starts_with("http://") || origin.starts_with("https://") {
        Ok(origin.to_owned())
    } else {
        Err(ConfigError::new(
            "CORS_ALLOWED_ORIGINS",
            "origins must start with http:// or https://",
        ))
    }
}

fn read_required<F>(read: &mut F, variable: &'static str) -> Result<String, ConfigError>
where
    F: FnMut(&str) -> Option<String>,
{
    read(variable)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| ConfigError::new(variable, "must be set"))
}

#[cfg(test)]
mod tests {
    use actix_web::{App, http::StatusCode, test as actix_test};

    use super::*;

    #[test]
    fn config_reads_defaults_and_bounded_cors_from_web_origin() {
        let config = ApiConfig::from_reader(|key| match key {
            "DATABASE_URL" => {
                Some("postgres://urbanlens:urbanlens_dev@localhost/urbanlens".to_owned())
            }
            "WEB_ORIGIN" => Some("http://localhost:3000".to_owned()),
            _ => None,
        })
        .expect("config should parse");

        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 8080);
        assert_eq!(config.database_max_connections, 5);
        assert_eq!(config.cors_allowed_origins, ["http://localhost:3000"]);
    }

    #[test]
    fn config_rejects_wildcard_cors_origin() {
        let error = ApiConfig::from_reader(|key| match key {
            "DATABASE_URL" => {
                Some("postgres://urbanlens:urbanlens_dev@localhost/urbanlens".to_owned())
            }
            "CORS_ALLOWED_ORIGINS" => Some("*".to_owned()),
            _ => None,
        })
        .expect_err("wildcard CORS should be rejected");

        assert_eq!(error.variable, "CORS_ALLOWED_ORIGINS");
    }

    #[test]
    fn graphql_pagination_is_strictly_bounded() {
        assert_eq!(
            Pagination::from_args(None, None),
            Pagination {
                limit: DEFAULT_GRAPHQL_LIMIT,
                offset: 0
            }
        );
        assert_eq!(
            Pagination::from_args(Some(500), Some(-10)),
            Pagination {
                limit: MAX_GRAPHQL_LIMIT,
                offset: 0
            }
        );
        assert_eq!(
            Pagination::from_args(Some(0), Some(12)),
            Pagination {
                limit: 1,
                offset: 12
            }
        );
    }

    #[actix_web::test]
    async fn graphql_schema_exposes_bounded_ingestion_inspection() {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://urbanlens:urbanlens_dev@localhost/urbanlens")
            .expect("lazy pool should parse database URL");
        let schema = build_schema(pool);
        let sdl = schema.sdl();

        assert!(sdl.contains("transactionObservations("));
        assert!(sdl.contains("importRuns("));
        assert!(sdl.contains("validationIssues("));
        assert!(sdl.contains("dataSources("));
        assert!(sdl.contains("transactionObservationProvenance("));
        assert!(sdl.contains("limit: Int"));
        assert!(sdl.contains("locationPrecision"));
        assert!(sdl.contains("payloadSha256"));
        assert!(!sdl.contains("payloadJson"));
    }

    #[actix_web::test]
    async fn health_endpoint_returns_request_id() {
        let app =
            actix_test::init_service(App::new().wrap(RequestId).configure(configure_routes)).await;
        let request = actix_test::TestRequest::get().uri("/health").to_request();

        let response = actix_test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.headers().contains_key(REQUEST_ID_HEADER));
    }

    #[actix_web::test]
    async fn health_endpoint_preserves_inbound_request_id() {
        let app =
            actix_test::init_service(App::new().wrap(RequestId).configure(configure_routes)).await;
        let request = actix_test::TestRequest::get()
            .uri("/health")
            .insert_header((REQUEST_ID_HEADER, "test-request-id"))
            .to_request();

        let response = actix_test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(REQUEST_ID_HEADER),
            Some(&HeaderValue::from_static("test-request-id"))
        );
    }
}
