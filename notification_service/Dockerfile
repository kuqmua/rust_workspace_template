FROM rust:1.90.0-slim-bookworm@sha256:64232e656c058f4468e8d024e990acff04f0fd5a5c0a88a574dc37773d7325c9 AS builder
WORKDIR /workspace
RUN apt-get update \
    && apt-get install -y --no-install-recommends libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*
COPY . .
RUN rustup toolchain install nightly-2026-07-12 \
    && rustup default nightly-2026-07-12 \
    && cargo build --locked --release --package notification_service

FROM debian:bookworm-slim@sha256:7b140f374b289a7c2befc338f42ebe6441b7ea838a042bbd5acbfca6ec875818
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system application \
    && useradd --system --gid application --home-dir /nonexistent application
COPY --from=builder /workspace/target/release/notification_service /application/notification_service
USER application
EXPOSE 8081
ENTRYPOINT ["/application/notification_service"]
