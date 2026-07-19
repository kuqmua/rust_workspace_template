FROM rust:slim AS builder
WORKDIR /workspace
RUN apt-get update \
    && apt-get install -y --no-install-recommends libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*
COPY . .
RUN rustup default nightly \
    && cargo build --locked --release --package notification_service

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system application \
    && useradd --system --gid application --home-dir /nonexistent application
COPY --from=builder /workspace/target/release/notification_service /application/notification_service
USER application
EXPOSE 8081
ENTRYPOINT ["/application/notification_service"]
