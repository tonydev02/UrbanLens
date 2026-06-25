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
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use futures_util::future::LocalBoxFuture;
use serde::Serialize;
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

const DEFAULT_API_HOST: &str = "0.0.0.0";
const DEFAULT_API_PORT: u16 = 8080;
const DEFAULT_CORS_ORIGIN: &str = "http://localhost:3000";
const DEFAULT_MAX_DB_CONNECTIONS: u32 = 5;
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
}

#[derive(SimpleObject)]
struct Connectivity {
    service: &'static str,
    status: String,
    database_reachable: bool,
    migrations_applied: bool,
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
