# Crate Split Analysis

## Goal

Reduce service binary sizes and improve compilation time by splitting crates only
where there is a meaningful dependency or rebuild boundary.

This analysis considers:

- source size;
- direct dependency weight;
- reverse dependency fan-out;
- which parts of a crate are used by each service;
- whether a split can remove dependencies from a service binary;
- whether a split only improves incremental compilation.

Creating more crates is not automatically beneficial. A useful split lets a
consumer avoid compiling or linking functionality that it does not use.

## Summary

Recommended priority:

| Priority | Current crate | Proposed boundary | Primary benefit |
| --- | --- | --- | --- |
| 1 | `server_runtime` | core, HTTP runtime, observability | Compilation and possible binary size |
| 2 | `frontend_contract` | portable contracts, server adapters, validation | Dependency fan-out and compilation |
| 3 | `server_admin` | core/auth, repository, HTTP, generated tables | Incremental and parallel compilation |
| 4 | `pg_crud_common` | runtime core, PostgreSQL support, code generation | Remove build-time dependencies from runtime |
| 5 | `server_admin_frontend` | shared UI, SSR host, WASM client | Target-specific compilation |
| 6 | `server_admin_contract` | core contract and table contract | Conditional, based on consumer usage |

The first split should be `server_runtime`. It combines unrelated functionality,
has many downstream consumers, and forces narrow consumers through a broad
dependency boundary.

## Implementation Result

The plan was implemented where the repository contains a proven dependency
boundary:

| Original responsibility | Implemented crate | Result |
| --- | --- | --- |
| dependency-light runtime primitives | `server_runtime_core` | extracted |
| tracing and OpenTelemetry | `server_observability` | extracted |
| HTTP and service integrations | `server_runtime_http` | extracted |
| OpenAPI and HTTP contract validation | `frontend_contract_validation` | extracted |
| administrator domain wrappers | `server_admin_core` | extracted |
| CRUD token rendering | `generate_pg_types_src` | consolidated into its only consumer |

All workspace consumers were migrated away from the original `server_runtime`
crate. The temporary facade was then removed. In particular,
`synchronization_service_runtime` now depends only on `server_runtime_core`.

OpenAPI, JSON snapshot, and HTTP contract fixture logic was removed from
`frontend_contract`. Its service-side validation uses are now development
dependencies through `frontend_contract_validation`, so they are absent from
normal service builds.

`proc-macro2` and `quote` were removed from `pg_crud_common`. The remaining
token rendering implementation now lives directly in its only consumer,
`generate_pg_types_src`, rather than in runtime service code.

The first inward administrator boundary was also implemented:
`server_admin_core` owns reusable administrator identifiers and domain wrapper
types. The repository, HTTP, and generated-table modules remain together
because their current interfaces refer to one another in both directions.
Splitting those modules immediately would either create crate cycles or require
a broader public API redesign unrelated to compilation. Future separation
should first move the remaining authentication-owned repository parameter types
into `server_admin_core`.

`server_admin_frontend` was intentionally left as one crate. Its manifest
already keeps native SSR dependencies and WASM CSR dependencies in mutually
exclusive target sections. A physical three-crate split would not remove those
dependencies from either target and would add crate scheduling overhead.

`server_admin_contract` was also left as one crate. Source-module
modularization would improve navigation but would not change its compilation
unit or service binary. No independent consumer boundary was found that
justifies a second contract crate.

An isolated clean baseline build was attempted from the pre-change `HEAD`, but
the environment exhausted its disk quota before linking. Therefore no
clean-build timing or binary-size comparison is claimed. Workspace compilation,
the repository quality gates, and dependency graph inspection are the
authoritative verification for this implementation.

The verified post-change development artifacts and dependency graph are:

| Measurement | Result |
| --- | ---: |
| `server` development executable | 355,640,936 bytes |
| `notification_service` development executable | 114,116,096 bytes |
| `server` normal dependency-tree lines | 428 |
| `notification_service` normal dependency-tree lines | 263 |
| `synchronization_service_runtime` normal dependency-tree lines | 22 |

These executable sizes include development debug information and are recorded
only as a post-change reference, not as release-size measurements.

The reverse dependency graph proves the intended isolation:

- the removed `server_runtime` package has no remaining package entry;
- `synchronization_service_runtime` depends directly on
  `server_runtime_core`;
- `frontend_contract_validation` is used only as a development dependency by
  `notification_service` and `server_admin`;
- CRUD token rendering is owned by `generate_pg_types_src`, not by a service
  runtime crate.

## Evidence

The largest relevant crates by source size are approximately:

| Crate | Rust LOC | Source files | Direct dependencies |
| --- | ---: | ---: | ---: |
| `server_admin` | 9,204 | 26 | 40 |
| `server_runtime` | 8,828 | 45 | 22 |
| `pg_crud_common` | 6,507 | 27 | 28 |
| `server_admin_contract` | 4,314 | 1 | 9 |
| `frontend_contract` | 4,099 | 10 | 10 |
| `server_admin_frontend` | 2,606 | 4 | 15 |

Large generator and test crates were examined as well, but they do not directly
increase deployed service binary size.

## 1. Split `server_runtime`

`server_runtime` currently contains more than 40 modules and directly depends on
Axum, HTTP, Tower, Reqwest, SQLx, Tokio, Serde JSON, metrics, OpenTelemetry, and
tracing.

Its consumers use substantially different subsets:

- `server` uses the HTTP server, observability, metrics, timeouts, and resource
  budgets.
- `notification_service` uses the HTTP server, observability, and error
  telemetry.
- `server_admin` uses cookies, origins, PostgreSQL rate limiting, and observed
  errors.
- `server_app_state` uses resource budgets.
- `synchronization_service_runtime` uses retries and execution plans.
- `common_routes` uses health probes.

### Initial split

Create only three crates initially:

```text
server_runtime_core
├── background jobs
├── concurrency primitives
├── execution plans
├── generation gates
├── leases
├── resource budgets
├── resource utilization
├── retries
├── secret text
├── single flight
└── source selection

server_runtime_http
├── bounded HTTP reads
├── client IP resolution
├── cookies
├── CORS and CSP
├── fallback handling
├── health routes
├── HTTP header and path policies
├── HTTP metrics layer
├── origins
├── request IDs
├── server bootstrap
└── timeouts and security layers

server_observability
├── error observation
├── metrics integration
├── OpenTelemetry initialization
├── trace context
└── tracing integration
```

The intended dependency direction is:

```text
server_runtime_core
        ↑
server_runtime_http
        ↑
server_observability adapters
```

`server_runtime_core` must not depend on Axum, HTTP, Tower, Reqwest, SQLx,
OpenTelemetry, or tracing subscribers.

### Possible later splits

Only after measuring the initial change, consider:

```text
server_runtime_io
├── bounded file and HTTP reads
├── child processes
├── multipart staging
├── outbound URL policy
└── redacted URLs

server_runtime_pg
└── PostgreSQL rate limiting
```

Creating all five crates immediately would add graph complexity before the
benefit is measured.

### Expected result

This is the most likely split to improve both service build time and service
size. Narrow consumers such as `server_app_state` and
`synchronization_service_runtime` would no longer compile the complete HTTP,
SQLx, Reqwest, and OpenTelemetry runtime.

## 2. Split `frontend_contract`

`frontend_contract` combines:

- portable wire and route metadata;
- Axum and Utoipa server integration;
- OpenAPI, route-coverage, snapshot, and HTTP contract validation.

It is used by both services, contract crates, generated PostgreSQL types, tests,
and scaffolding. This high reverse fan-out makes changes expensive.

### Proposed split

```text
frontend_contract
├── authentication session decisions
├── problem wire types
├── portable route metadata
├── status types
└── URL construction

frontend_contract_server
├── Axum handler adapters
├── Axum response conversion
├── server route metadata
└── Utoipa registration

frontend_contract_validation
├── HTTP contract fixtures
├── JSON snapshots
├── OpenAPI validation
└── route coverage validation
```

The validation crate should be a development dependency for consumers that only
need validation in tests.

Keep `frontend_contract_macros` separate. It already has the correct procedural
macro boundary.

### Expected result

Runtime contract consumers can avoid validation machinery. Portable consumers
can also avoid Axum server adapters when they only need serialized contract
types.

## 3. Split `server_admin`

`server_admin` has about 9,204 lines, 26 source files, and 40 direct
dependencies. It combines:

- authentication and authorization domain logic;
- password hashing and JWT handling;
- SQLx repositories and migrations;
- Axum API and HTML handlers;
- generated CRUD tables;
- OpenAPI assembly;
- frontend SSR integration.

### Proposed split

```text
server_admin_core
├── authentication decisions
├── authorization and RBAC
├── domain wrapper types
├── password policy
└── token domain types

server_admin_repository
├── audit persistence
├── migrations
├── roles and permissions
├── sessions
├── settings
├── SQLx repositories
└── users

server_admin_http
├── API handlers
├── cookies and origin handling
├── HTML form handlers
├── OpenAPI assembly
└── routes

server_admin_tables
├── generated table declarations
├── table authorization adapter
└── table repository integration
```

Move the shared administrator domain wrappers into `server_admin_core` first.
Repositories currently accept authentication-owned types, while generated
tables and repositories refer to each other. Establishing the inward core
dependency first prevents crate cycles.

### Expected result

This split primarily improves incremental and parallel compilation. It will not
necessarily reduce the main `server` binary because that service currently uses
most administrator functionality.

Binary-size reduction becomes possible when a deployment omits HTML
administration, generated data tables, or SSR integration.

## 4. Split `pg_crud_common`

`pg_crud_common` combines runtime query logic with dependencies used for schema
and source generation, including:

- `proc-macro2`;
- `quote`;
- naming helpers;
- location/code-generation helpers;
- Schemars and Utoipa;
- SQLx.

### Proposed split

```text
pg_crud_core
├── bounded collections
├── invariant validation
├── operation budgets
├── pagination
├── patch fields
└── query plans

pg_crud_pg
├── advisory locks
├── PostgreSQL error classification
├── query binding
├── schema conformance
└── SQL identifiers

generate_pg_types_src
├── generated schema metadata
├── naming integration
├── proc-macro2 token generation
└── quote-based helpers
```

`generate_pg_types_src` may depend on `pg_crud_core`. Runtime crates must not
depend on `generate_pg_types_src`.

### Expected result

Runtime paths can avoid compiling generator-oriented code and host-side token
construction dependencies. The benefit depends on removing the old broad
`pg_crud_common` dependency from service paths rather than leaving it as a
permanent facade dependency.

## 5. Separate `server_admin_frontend` targets

`server_admin_frontend` already declares target-specific dependencies:

- native builds use Axum, Leptos SSR, and Tower HTTP;
- WASM builds use Leptos CSR, `wasm-bindgen`, and `web-sys`.

That existing setup already prevents many cross-target dependencies from being
compiled. A split is useful mainly when developers frequently build the native
and WASM targets independently.

### Proposed split

```text
server_admin_ui
server_admin_frontend_ssr
server_admin_frontend_wasm
```

Shared renderable components belong in `server_admin_ui`. Native hosting belongs
in the SSR crate, and browser startup and transport belong in the WASM crate.

This is lower priority than `server_runtime` and `frontend_contract`.

## 6. Treat `server_admin_contract` conservatively

`server_admin_contract` contains about 4,314 lines in one source file. First
divide it into source modules without creating crates:

```text
auth.rs
data_tables.rs
pages.rs
permissions.rs
roles.rs
sessions.rs
settings.rs
users.rs
```

Only introduce a crate boundary if consumer analysis confirms that the
administrator frontend and backend need different subsets:

```text
server_admin_contract_core
server_admin_table_contract
```

Turning every source module into a crate would increase clean-build scheduling,
manifest maintenance, and dependency coordination without proving a reduction
in compiled work.

## Crates Not Recommended for Splitting

### `str_constants`

It is a large source file but has only two direct dependencies and mostly
contains constants. Splitting it would increase dependency graph fan-out.
Domain-specific constants should instead move gradually into their owning
crates.

### Generator crates

`generate_pg_table_src` and `generate_pg_types_src` contain very large source
files, but their procedural macro consumers normally require the complete
generation pipeline. Split these into internal source modules for
maintainability before considering new crates.

Such changes may improve editing and incremental recompilation within the crate,
but they will not reduce deployed service binary size.

### Procedural macro crates

Do not split `newtype`, `frontend_contract_macros`, or similar macro crates
without compiler-timing evidence. Additional procedural macro crates add
host-side compilation and dynamic-library linking overhead.

### Test crates

The `tests` crate is large, but splitting it affects test iteration rather than
deployed service size. A separate code-style test crate may be useful, but it is
outside the service-size goal.

### Small service and configuration crates

The service entry-point and configuration crates are already relatively
focused. Splitting them would add graph nodes without removing substantial
dependencies.

## Implementation Order

1. Extract `server_runtime_core`.
2. Extract `server_observability`.
3. Move HTTP-specific functionality into `server_runtime_http`.
4. Migrate consumers to direct narrow dependencies.
5. Split validation and server adapters from `frontend_contract`.
6. Extract `server_admin_core`.
7. Extract the administrator repository and HTTP layers.
8. Keep CRUD token rendering in the source generator rather than runtime CRUD
   types.
9. Re-evaluate the frontend and administrator contract splits using build
   measurements.

An interim facade can preserve imports during migration, but consumers must
eventually depend directly on the narrow crates. If every service continues to
depend on a facade that re-exports all subcrates, compilation and binary-size
benefits will be limited.

## Measurement Plan

Measure before the first split and after each boundary is introduced.

Use isolated target directories when comparing clean builds so existing
incremental artifacts do not distort results. Record:

- clean `cargo build -p server` time;
- clean `cargo build -p notification_service` time;
- no-change incremental build time;
- rebuild time after changing one core module;
- rebuild time after changing one HTTP or validation module;
- final service executable sizes;
- the transitive dependency count of each service.

Do not evaluate a split using source line count alone. Keep it only when it
reduces compiled dependencies, narrows rebuild invalidation, permits useful
parallel compilation, or establishes an architecture boundary needed by those
improvements.
