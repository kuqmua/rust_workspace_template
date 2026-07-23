# Rust microservice workspace template

Production-oriented Rust workspace for starting a typed HTTP system composed of independently
deployable microservices.

## Included services

| Service | Port | Persistence | Responsibility |
|---|---:|---|---|
| `server` | 8080 | application PostgreSQL | public API, generated CRUD and administrator console |
| `notification_service` | 8081 | notification PostgreSQL | reference independently deployable notification API |

The notification service deliberately owns a separate contract crate, configuration crate,
migrations, database credentials, Dockerfile, health checks, and Kubernetes workload. It is the
reference boundary to follow when adding another service.

## Shared foundations

- `server_runtime`: lifecycle, graceful shutdown, health, limits, request metadata, retries,
  outbound URL policy and metrics;
- `frontend_contract` and service-owned contract crates: typed transport values;
- `config_lib`: validated environment configuration with secret redaction;
- `pg_crud`: generated PostgreSQL CRUD contracts, handlers, clients and OpenAPI metadata;
- `server_admin`: sessions, RBAC, audit, rate limits and operational cleanup;
- `text_policy` and `newtype`: reusable validation and domain-wrapper mechanics;
- `external_service_emulators`: deterministic integration-test doubles.

See [microservice architecture](docs/architecture.md) for ownership and communication rules.

## Prerequisites

- the Rust nightly pinned in `rust-toolchain.toml`;
- PostgreSQL 16 or Docker Compose;

## Local development

Initialize environment files and start both services with isolated databases:

```bash
cargo run -p initialize_environment_files
POSTGRES_PASSWORD=development-only \
NOTIFICATION_POSTGRES_PASSWORD=development-only \
docker compose up --build
```

Health endpoints:

```text
http://127.0.0.1:8080/health/live
http://127.0.0.1:8080/health/ready
http://127.0.0.1:8081/health/live
http://127.0.0.1:8081/health/ready
```

## Distributed tracing

Both backend services create OpenTelemetry server spans for incoming requests, preserve an
incoming W3C `traceparent`/`tracestate` context, and export spans to an OTLP/HTTP collector. HTTP
requests executed through `server_runtime::ReqwestClient::execute` create client spans and inject
the current W3C trace context automatically.

Incoming HTTP spans use Axum's matched route template as `http.route`; unmatched requests receive
the stable `__unmatched__` label. Raw `url.path` is emitted only for an exact static route, so
dynamic identifiers are not exported. `client.address` uses the direct peer unless that peer is in
the configured trusted proxy ranges, in which case validated forwarded address headers are used.
The spans also include method, response status, error classification, server address, trace/span
identifiers and service name.

The exporter uses the standard OpenTelemetry environment variables. A typical production
configuration is:

```text
OTEL_EXPORTER_OTLP_TRACES_ENDPOINT=http://otel-collector:4318/v1/traces
OTEL_EXPORTER_OTLP_TIMEOUT=10000
OTEL_RESOURCE_ATTRIBUTES=service.namespace=backend,deployment.environment.name=production
```

`service.name` is always set by each binary. If no endpoint is configured, the OpenTelemetry SDK
uses its standard OTLP/HTTP default. The tracer provider is flushed and shut down after HTTP and
background-task graceful shutdown completes.

Development credentials are not production defaults. Production deployments must supply secrets,
exact allowed origins, secure cookies, immutable image tags and managed database URLs.

## Build

```bash
cargo build --workspace
```

Each backend can also be built independently:

```bash
cargo build --release -p server
cargo build --release -p notification_service
```

## Quality gates

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p tests code_style
cargo test --workspace
```

CI additionally checks dependency policy, unused dependencies, coverage, documentation, secrets,
container vulnerabilities, feature combinations and semantic-version compatibility.

## Deployment

- Dockerfiles build non-root, read-only compatible runtime images.
- `deploy/k8s/base` contains Kustomize-ready workloads, probes, resource budgets, disruption
  budgets and default-deny networking.
- Environment-specific overlays must supply ConfigMaps, Secrets, ingress, immutable images and
  database endpoints.
- Administrator bootstrap, recovery, rotation and retention procedures are documented in
  [the operations runbook](ADMIN_OPERATIONS_RUNBOOK.md).

## Adding a service

Create the three-crate service boundary from the maintained notification template:

```bash
cargo run -p workspace_scaffold -- service order_service 8082
docker compose -f docker-compose.yml -f docker-compose.order_service.yml up --build
```

The command performs the following steps:

1. create `<name>`, `<name>_config`, and `<name>_contract` crates;
2. give the service its own database principal and migrations;
3. expose separate live and ready probes;
4. add a service Dockerfile and Compose entry;
5. add Kubernetes workload and network access;
6. add contract, migration, integration and graceful-shutdown tests;
7. keep repository code private to the owning service.

Before using the repository as a new project, replace its template identity in tracked text files:

```bash
cargo run -p workspace_scaffold -- project order_platform https://github.com/acme/order_platform
cargo fmt
cargo test -p tests code_style
```

## License

MIT. See [LICENSE](LICENSE).
