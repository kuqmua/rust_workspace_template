FROM rust:slim AS builder
WORKDIR /workspace
COPY . .
RUN rustup default nightly \
    && cargo build --locked --release --package server

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system application \
    && useradd --system --gid application --home-dir /nonexistent application
COPY --from=builder /workspace/target/release/server /application/server
USER application
EXPOSE 8080
ENTRYPOINT ["/application/server"]
