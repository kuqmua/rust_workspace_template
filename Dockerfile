FROM rust:1.90.0-slim-bookworm@sha256:64232e656c058f4468e8d024e990acff04f0fd5a5c0a88a574dc37773d7325c9 AS builder
WORKDIR /workspace
RUN apt-get update \
    && apt-get install -y --no-install-recommends libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*
COPY . .
ENV ADMIN_FRONTEND_STATIC_DIR=/application/admin/static
RUN rust_toolchain="$(sed -n 's/^channel = \"\\([^\"]*\\)\"$/\\1/p' rust-toolchain.toml)" \
    && test -n "${rust_toolchain}" \
    && rustup toolchain install "${rust_toolchain}" \
    && rustup default "${rust_toolchain}" \
    && rustup target add wasm32-unknown-unknown \
    && cargo install trunk --version 0.21.14 --locked
WORKDIR /workspace/server_admin_frontend
RUN NO_COLOR=true trunk build --release
WORKDIR /workspace
RUN cargo build --locked --release --package server

FROM debian:bookworm-slim@sha256:7b140f374b289a7c2befc338f42ebe6441b7ea838a042bbd5acbfca6ec875818
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --gid 10001 application \
    && useradd --uid 10001 --gid 10001 --no-create-home --home-dir /nonexistent application
COPY --from=builder /workspace/target/release/server /application/server
COPY --from=builder /workspace/server_admin_frontend/static /application/admin/static
USER 10001:10001
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
    CMD curl --fail --silent http://127.0.0.1:8080/health/ready || exit 1
ENTRYPOINT ["/application/server"]
