FROM rust:1.96.0-bookworm AS build

WORKDIR /app

COPY rust-toolchain.toml Cargo.toml Cargo.lock ./
COPY apps/api/Cargo.toml apps/api/Cargo.toml
COPY crates/domain/Cargo.toml crates/domain/Cargo.toml
COPY workers/importer/Cargo.toml workers/importer/Cargo.toml

COPY apps/api apps/api
COPY crates/domain crates/domain
COPY workers/importer workers/importer

RUN cargo build --release -p urbanlens-api --bins

FROM debian:bookworm-slim

COPY --from=build /app/target/release/urbanlens-api /usr/local/bin/urbanlens-api
COPY --from=build /app/target/release/urbanlens-healthcheck /usr/local/bin/urbanlens-healthcheck
COPY --from=build /app/target/release/urbanlens-migrate /usr/local/bin/urbanlens-migrate

CMD ["urbanlens-api"]
