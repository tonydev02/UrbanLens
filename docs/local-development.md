# Local Development

UrbanLens runs its local API dependencies through Docker Compose.

## Services

- `postgres`: PostGIS-backed PostgreSQL database.
- `migrate`: one-shot SQLx migration runner.
- `api`: Actix Web API. It starts only after PostgreSQL is healthy and `migrate` exits successfully.

## API Configuration

The API reads these environment variables:

- `DATABASE_URL`: PostgreSQL connection string. Required.
- `API_HOST`: bind host. Defaults to `0.0.0.0`.
- `API_PORT`: bind port. Defaults to `8080`.
- `API_DATABASE_MAX_CONNECTIONS`: SQLx pool size. Defaults to `5`.
- `CORS_ALLOWED_ORIGINS`: comma-separated HTTP or HTTPS origins. Wildcards are rejected.
- `RUST_LOG`: logging filter.

## API Checks

- `GET /health`: process liveness only.
- `GET /ready`: readiness. Returns HTTP 200 only when the database is reachable and SQLx migration metadata exists.
- `POST /graphql`: product GraphQL endpoint. The initial connectivity query is:

```graphql
{
  connectivity {
    service
    status
    databaseReachable
    migrationsApplied
  }
}
```

Responses include an `x-request-id` header. If the request already has `x-request-id`, the API preserves it; otherwise it generates one.
