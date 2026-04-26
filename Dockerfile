FROM rust:latest AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY server/Cargo.toml server/
COPY tests/Cargo.toml tests/
COPY optml/Cargo.toml optml/

RUN mkdir server/src tests/src optml/src && \
    echo "fn main() {}" > server/src/main.rs && \
    echo "" > tests/src/lib.rs && \
    echo "" > optml/src/lib.rs

RUN cargo build --release -p server 2>/dev/null || true

COPY . .
RUN touch server/src/main.rs tests/src/lib.rs optml/src/lib.rs && \
    cargo build --release -p server

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/server /server

EXPOSE 8080

ENTRYPOINT ["/server"]
