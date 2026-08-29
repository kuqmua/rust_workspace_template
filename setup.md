# Setup guide

The repository uses the nightly toolchain pinned in `rust-toolchain.toml`.

## Local environment

Create the untracked service environment files:

```bash
cargo run -p init_env_files
```

Start both services and their isolated PostgreSQL databases:

```bash
POSTGRES_PASSWORD=development-only \
NOTIFICATION_POSTGRES_PASSWORD=development-only \
docker compose up --build
```

The application listens on `127.0.0.1:8080`; the notification service listens on
`127.0.0.1:8081`. Their liveness and readiness endpoints are `/health/live` and
`/health/ready`.

To run only the application locally, make sure `server/.env` points to a reachable PostgreSQL
database and run:

```bash
cargo run -p server
```

If startup reports `Address already in use (os error 98)`, another process already owns the
configured socket. Find it with `ss -ltnp | grep ':8080'`, stop that process, or select another
`SERVICE_SOCKET_ADDRESS`.

## Database migrations

Both service binaries apply their own embedded migrations during startup. Add administrator
migrations to `server_admin_migrations/` and notification migrations to
`notification_service_migrations/`; do not access another service's tables.

## Quality gates

Run the same required checks used by the repository:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p tests_code_style
cargo test --workspace --exclude tests_code_style
```

Run database-backed ignored tests only after provisioning the required services:

```bash
cargo run -p workspace_test_runner -- database
```

Do not suppress warnings locally: CI treats them as errors.

## Adding a service

Use the maintained scaffold instead of copying files manually:

```bash
cargo run -p workspace_scaffold -- service order_service 8082
```

See [README.md](README.md#adding-a-service) and
[docs/architecture.md](docs/architecture.md) for ownership and deployment rules.
