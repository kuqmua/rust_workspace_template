# Administrator quickstart

This path starts the application database and server, creates the only permitted initial
administrator, and opens the administrator console. It is intended for a clean local clone.

## Prerequisites

- the pinned Rust nightly toolchain;
- Docker Compose;
- `wasm32-unknown-unknown`;
- Trunk for building the browser application.

Install the browser target and Trunk once:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
```

## 1. Rename a new project

Skip this step when contributing to the template itself.

```bash
cargo run -p workspace_scaffold -- \
  project order_platform https://github.com/acme/order_platform
```

## 2. Create local configuration

```bash
cargo run -p initialize_environment_files
```

The generated `server/.env` is for local development. Before a production deployment, replace
every development credential, set `ADMIN_COOKIE_SECURE=true`, disable public administrator Swagger
with `ADMIN_SWAGGER_ENABLED=false`, configure explicit HTTPS CORS origins, and then set
`PRODUCTION_MODE=true`. The server refuses to start in production mode with the template JWT
secret, an insecure administrator cookie, public administrator Swagger, or a non-HTTPS CORS
origin.
Set `ADMIN_LOGIN_FAILURE_LIMIT` to the number of recent failed attempts that locks one login;
`ADMIN_SIGN_IN_RATE_LIMIT` separately limits request volume.

## 3. Start PostgreSQL

The password below matches the database URL in the generated development environment:

```bash
POSTGRES_PASSWORD=change-me docker compose up -d database
```

Wait until `docker compose ps database` reports a healthy service.

## 4. Build the browser application

```bash
cd server_admin_frontend
trunk build --release
cd ..
```

The generated files are written below `server_admin_frontend/static/csr` and served below
`/admin/assets`.

## 5. Create the first administrator

Write the initial password into a temporary file without passing it through the process list:

```bash
umask 077
printf '%s\n' 'replace-with-a-long-local-password' > /tmp/admin-bootstrap-password
```

Run the one-time bootstrap workspace command from the server directory so it loads `server/.env`:

```bash
cd server
cargo run -p admin_bootstrap -- \
  admin "Local administrator" /tmp/admin-bootstrap-password
cd ..
```

The command applies pending administrator migrations, creates the first administrator and its
system role, and then refuses every later bootstrap attempt. It returns exit code `2` for invalid
input, `3` when bootstrap was already completed, and `1` for operational failures.

Remove the temporary secret immediately:

```bash
rm /tmp/admin-bootstrap-password
```

## 6. Start the application

```bash
cd server
cargo run --bin server
```

Open `http://127.0.0.1:8080/admin/sign_in`, sign in as `admin`, and change the initial password from
the Profile page. Branding and the default administrator route can be changed from Settings.

Readiness is available at `http://127.0.0.1:8080/health/ready`.

## 7. Verify the repository

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p tests code_style
cargo test --workspace
```

Database-backed administrator tests run through:

```bash
cargo run -p workspace_test_runner -- database
```

CI continuously executes the machine portion of this path—disposable PostgreSQL, browser build,
validated configuration, one-time bootstrap, server start, sign-in, password replacement, and
administrator navigation—through `browser_acceptance/run-server.sh` and the pull-request browser
suite.
