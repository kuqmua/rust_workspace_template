# Code Reuse Plan

## Purpose

This document defines which modules are reusable workspace building blocks, which modules may consume them, and which responsibilities must remain application-specific. The target is to add new HTTP, synchronization, administration, and background services without copying validation, runtime, persistence, contract, or test logic.

## Dependency direction

Dependencies must point from composition and domain crates toward reusable crates:

```text
server / service binaries
  -> domain services and adapters
  -> frontend_contract / pg_crud / file_storage / synchronization_service_runtime
  -> server_runtime / config_lib
  -> shared domain types and validation

proc-macro facade
  -> proc-macro implementation helpers
  -> workspace_macro_helpers / macros_helpers / naming / str_constants
```

Reusable crates must not depend on `server`, a concrete table crate, an application frontend, or a concrete service configuration. Domain crates must not move into a generic runtime crate merely because two functions look similar.

## Module reuse map

### 1. Text constants and domain values

#### `str_constants` and `str_constants_macros`

Reuse:

- constant fragments shared by generated and handwritten Rust code;
- composed constant strings used by API, SQL, diagnostics, tests, and macro parsing;
- compile-time construction of repeated strings without runtime allocation.

Consumers:

- every crate that needs a reusable string literal;
- proc-macro crates that parse named fields or generate diagnostics.

Do not duplicate:

- error messages or protocol tokens in consumer crates;
- local constants that merely rename a `str_constants` item;
- `include_str!`-based constant registries.

#### `newtype`

Reuse:

- `Newtype` for repository wrapper types;
- `BoundedString` for validated text boundaries;
- `EnumFromStr` for closed textual value sets.

Consumers:

- API contracts, configuration, runtime policies, identifiers, and persistence values.

Do not duplicate:

- manual length, NUL, trimming, serde, or OpenAPI validation when `BoundedString` expresses the same invariant;
- raw primitive or external types at domain boundaries.

### 2. Proc-macro infrastructure

#### `workspace_macro_helpers`

Reuse:

- top-level token splitting;
- first-identifier parsing;
- closure and fat-arrow parsing;
- unique option tracking;
- uniform compile-error token generation.

Consumers:

- all newly created proc-macro parsers.

Do not duplicate:

- ad hoc comma splitting or token-tree indexing;
- separate option-duplication checks.

#### `macros_helpers`, `pg_crud_macros_common`, and `naming`

Reuse:

- generated Rust token wrappers and writers;
- derive input and field metadata extraction;
- common `From`, `TryFrom`, `Display`, and error implementation generation;
- PostgreSQL CRUD token construction;
- consistent snake-case and upper-camel-case naming.

Consumers:

- `generate_pg_table`, `generate_pg_types`, `generate_where_filters`, and future derive macros.

Required structure for new macros:

1. Keep the public proc-macro entry point thin.
2. Put reusable parsing and expansion logic in a normal library crate.
3. Reuse existing token and naming wrappers.
4. Add expansion unit tests and compile-fail tests.

### 3. API and frontend contracts

#### `frontend_contract`

Reuse:

- `TypedRoute` as the single request, response, transport, method, path, and OpenAPI operation contract;
- `CoveredRoute` and `RouteFamily` for modular API registries;
- `RouteCoverageDescriptor` and `validate_route_coverage` for contract-test obligations;
- client and server request/response wrappers;
- OpenAPI operation and payload validation;
- canonical JSON contract snapshots;
- shared API problem responses;
- safe URL construction;
- form and action metadata consumed by Rust frontends.

Consumers:

- `server_admin_contract` and future service contract crates;
- server route adapters;
- Leptos or other Rust frontend clients;
- API integration and OpenAPI tests.

Module pattern:

```text
<domain>_contract
  route types -> TypedRoute
  route groups -> RouteFamily
  request/response DTOs -> validated domain wrappers

<domain> server adapter
  handler input/output -> RouteRequest<Route> / RouteResponse<Route>

<domain> frontend adapter
  request metadata -> client_route_metadata<Route>()
```

Do not duplicate:

- client-only copies of endpoint paths;
- handler-only request or response DTOs;
- separate OpenAPI operation identifiers;
- route coverage lists assembled manually when `RouteFamily` can generate them.

#### `frontend_contract_macros`

Reuse:

- `TypedRoute` derive for one-source route declarations;
- `RouteFamily` derive for grouped coverage descriptors.

Next extraction point:

- if two server adapters repeat the same typed Axum conversion, add a generic adapter in `frontend_contract` and keep the generated code limited to type-safe glue.

### 4. PostgreSQL and CRUD

#### `pg_crud_common`

Reuse:

- bounded collections and duplicate policies;
- pagination, cursors, list totals, and read plans;
- SQL identifier and LIKE-pattern validation;
- bind index and query-fragment construction;
- PostgreSQL error classification;
- database schema conformance checks;
- batch validation and operational invariants;
- operation budgets, advisory locks, and rollback reporting.

Consumers:

- generated CRUD code;
- handwritten transactional domain repositories;
- migration and schema verification tests.

Do not duplicate:

- local SQLSTATE matching;
- unbounded batch validation;
- string concatenation for identifiers or reusable predicates;
- pagination or cursor semantics inside individual table crates.

#### `pg_table`, `generate_pg_table`, and generated source crates

Reuse:

- table-level CRUD generation;
- idempotency request/replay handling;
- optimistic revision predicates;
- common query builders and test generation.

Consumers:

- `server_table_example` and future table/domain persistence crates.

Rule:

- extend the existing generator when a behavior applies to multiple tables; do not add a second CRUD derive family.

#### `pg_types`, `pg_types_common`, and concrete PostgreSQL type crates

Reuse:

- typed database value mappings;
- nullability and validation contracts;
- generated test cases for supported PostgreSQL types.

Consumers:

- table definitions and API field contract generation.

### 5. Service runtime

#### `server_runtime`

Reuse by module:

| Runtime module | Reused responsibility |
| --- | --- |
| `background_job`, `lifecycle` | owned task lifecycle, interval execution, shutdown |
| `retry` | bounded retry attempts and delay policy |
| `history` | bounded asynchronous run history |
| `execution_plan` | `DryRun` and `Apply` execution |
| `source_selection` | local, remote, or combined source selection |
| `bounded_read` | bounded file, HTTP, text, and JSON reads |
| `exclusive_run`, `single_flight` | duplicate execution prevention |
| `deduplicating_queue` | bounded unique work queues |
| `resource_budget`, `resource_utilization`, `limits` | concurrency and resource admission |
| `health`, `metrics_layer`, `request_id` | operational HTTP observability |
| `cors`, `csp`, `origin`, `http_policy` | reusable HTTP security decisions |
| `secure_cookie`, `secret_text`, `password_policy` | credential boundary policies |
| `outbound_url`, `redacted_url`, `client_ip` | safe outbound and proxy-aware networking |
| `notification` | notification provider contract and HTTP adapter |
| `identity_bootstrap` | desired-state planning for service identities |
| `child_process` | owned child process supervision and diagnostics |

Consumers:

- `server`, synchronization services, administration services, maintenance jobs, and integration adapters.

Do not duplicate:

- Tokio runtime creation, retry loops, task spawning, shutdown handling, bounded reads, or concurrency semaphores in service crates;
- service-specific wrappers inside `server_runtime` unless their invariant is genuinely cross-service.

#### `synchronization_service_runtime`

Reuse:

- synchronization-specific composition of `RetryPolicy` and `ExecutionMode`;
- the common entry point for future synchronization orchestration shared by at least two services.

Consumers:

- future independent synchronization services.

Next extraction threshold:

- move history, source selection, bounded input, and scheduled execution composition here only after the same orchestration appears in a second synchronization service.

### 6. Development and integration infrastructure

#### `development_data_bootstrap`

Reuse:

- typed service identity specifications;
- desired-state identity decisions and summaries;
- generic bootstrap plans separated from secrets and transport.

Consumers:

- local development setup and test-environment bootstrap binaries.

Keep application-specific:

- concrete role names, users, passwords, and sample domain records.

#### `external_service_emulators`

Reuse:

- deterministic notification recording;
- deterministic remote synchronization payloads and request counts.

Consumers:

- integration tests for notification and remote synchronization adapters.

Rule:

- add an emulator only with its real integration contract; do not create speculative HTTP services.

#### `prepare_postgresql_databases`

Reuse:

- validated database URLs and migration sources;
- deterministic migration command plans for multiple databases.

Consumers:

- development or CI tooling when the workspace has multiple PostgreSQL databases or roles.

### 7. Storage

#### `file_storage`

Reuse:

- safe relative path validation;
- staging and atomic replacement;
- bounded cleanup of stale staging entries;
- disk-cache eviction planning and durability policy.

Consumers:

- upload, export, generated artifact, and local cache adapters.

Do not duplicate:

- direct path joining from user-controlled strings;
- service-local staging directory protocols;
- unbounded cleanup scans.

### 8. Configuration and application state

#### `config_lib`, `try_from_env`, and `server_config`

Reuse:

- environment parsing wrappers and validation;
- generated configuration getters;
- one concrete server configuration composition.

Consumers:

- service binaries at startup boundaries.

Rule:

- parsing belongs in `config_lib`; a concrete list of fields belongs in the service configuration crate; environment access stays at the owned startup boundary.

#### `app_state`, `server_app_state`, and `server_app_state_macros`

Reuse:

- typed database pool ownership and borrowing;
- generated state accessors required by handlers.

Consumers:

- route and domain services requiring application resources.

Do not turn application state into a service locator. Add only explicitly required, owned resources.

### 9. Administration domain

#### `server_admin_contract`

Reuse:

- administrator DTOs, validated identifiers and text values;
- route enumeration and route paths;
- API error contracts;
- table and action metadata shared with the frontend.

Consumers:

- `server_admin`, `server_admin_frontend`, OpenAPI tests, and API clients.

#### `server_admin`

Reuse internally by module:

- authentication and session handling;
- RBAC decisions;
- audit recording;
- password hashing and token policies;
- cleanup orchestration.

Boundary:

- these mechanisms remain administration-domain code until another authentication domain demonstrates the same semantics. Only then extract the invariant parts into a dedicated shared authentication crate.

### 10. Common routes and composition

#### `common_routes`

Reuse:

- health and Git information routes;
- their OpenAPI document contribution;
- the minimal state trait required by those routes.

Consumers:

- every HTTP service exposing the standard operational surface.

#### `server`

Keep composition-only:

- runtime initialization;
- configuration loading;
- database connection and migrations;
- router assembly;
- shutdown orchestration.

Any independently testable policy discovered in `server` must move to the appropriate shared or domain crate before a second binary copies it.

### 11. Testing and enforcement

#### `tests`

Reuse:

- workspace architecture and code-style policies;
- route contract compile-fail fixtures;
- CI and dependency policy checks.

#### Generated test crates and `workspace_test_runner`

Reuse:

- common generated PostgreSQL type/table/filter tests;
- deterministic workspace test discovery and execution.

Every reusable public behavior must have a unit test in its owning crate. Cross-crate contracts require an integration or compile-fail test.

## Adoption plan

### Phase 1: establish API contract reuse

1. Define each new endpoint as a `TypedRoute` in its domain contract crate.
2. Group related routes with `RouteFamily`.
3. Declare access, mutation, and coverage obligations next to route metadata.
4. Validate every family with `validate_route_coverage`.
5. Remove duplicated client paths, OpenAPI operation identifiers, and request/response DTOs.

Completion evidence:

- one route declaration supplies client, server, OpenAPI, and coverage metadata;
- duplicate route metadata fails validation;
- missing obligations fail validation or compilation tests.

### Phase 2: standardize new service runtime composition

1. Build every service with the single workspace Tokio runtime.
2. Use `server_runtime` for tracing, shutdown, health, metrics, request IDs, limits, and security policies.
3. Use `synchronization_service_runtime` for synchronization execution policy.
4. Use bounded reads and owned background tasks exclusively.
5. Use `common_routes` for the operational route surface.

Completion evidence:

- no service-local retry loop, detached task, or unbounded input read;
- service shutdown owns and terminates all background work.

### Phase 3: consolidate persistence behavior

1. Define new database fields through existing `pg_types` contracts.
2. Generate table CRUD through the current `pg_table` generator.
3. Place handwritten reusable query behavior in `pg_crud_common`.
4. Reuse PostgreSQL error classification, pagination, batch validation, and idempotency.
5. Add schema-conformance and generated CRUD tests.

Completion evidence:

- no second CRUD generator;
- no local SQLSTATE classifier or unbounded batch logic;
- generated and handwritten repositories share the same invariants.

### Phase 4: provide deterministic development integrations

1. Keep concrete sample roles and records outside the generic bootstrap crate.
2. Use `development_data_bootstrap` to plan identity creation.
3. Use `external_service_emulators` through the same interfaces as real adapters.
4. Activate multi-database preparation only when more than one database or role exists.

Completion evidence:

- integration tests do not depend on external services;
- bootstrap runs are idempotent and secrets are not embedded in specifications.

### Phase 5: enforce reuse continuously

For every pull request adding a service, endpoint, table, proc macro, or integration:

1. Search the module map before creating a helper.
2. Extend the owning shared crate when semantics are already shared.
3. Keep application-specific code in its domain crate when reuse is only hypothetical.
4. Add a code-style or contract test when accidental duplication can be detected mechanically.
5. Run the repository verification commands.

## New module decision checklist

Create or extend a shared module only when all answers are yes:

- Are the semantics identical in at least two consumers, or is the abstraction an explicit workspace boundary such as API contracts or runtime safety?
- Can the API use repository domain wrappers rather than primitives?
- Can the shared module avoid depending on a concrete service or table?
- Is cancellation, boundedness, and error propagation explicit?
- Can deterministic public-logic tests prove the invariant?

Otherwise keep the code in the concrete domain module and record the second-use extraction point.

## Required verification

After each reuse migration, run:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p tests code_style
```

Additionally run the unit and integration tests of every changed consumer and shared crate.
