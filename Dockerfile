FROM rust:slim AS builder
WORKDIR /workspace
RUN apt-get update \
    && apt-get install -y --no-install-recommends libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*
COPY . .
ENV ADMIN_FRONTEND_DIST_DIR=/application/admin/dist
ENV ADMIN_FRONTEND_STATIC_DIR=/application/admin/static
RUN rustup default nightly \
    && rustup target add wasm32-unknown-unknown \
    && cargo install trunk --version 0.21.14 --locked \
    && cd server_admin_frontend \
    && trunk build --release \
    && cd /workspace \
    && cargo build --locked --release --package server

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system application \
    && useradd --system --gid application --home-dir /nonexistent application
COPY --from=builder /workspace/target/release/server /application/server
COPY --from=builder /workspace/server_admin_frontend/dist /application/admin/dist
COPY --from=builder /workspace/server_admin_frontend/static /application/admin/static
USER application
EXPOSE 8080
ENTRYPOINT ["/application/server"]
