# Module boundaries

This document records the responsibility audit for every workspace crate. File size alone is not a
reason to split a module: generated token emitters, declarative catalogs, and conformance tests stay
together when they have one source of change. A module is split when it owns independently changing
runtime, transport, persistence, presentation, or policy concerns.

## Deployable services

| Crate | Responsibility | Internal boundary |
| --- | --- | --- |
| `server` | Public and administrator HTTP process | `bootstrap` builds infrastructure, `routing` mounts transports, `maintenance` owns cleanup policy, and `main` owns process orchestration. |
| `notification_service` | Notification HTTP process and persistence | `routes` owns HTTP/OpenAPI registration, `runtime` owns migration and service lifecycle, and `main` owns domain values plus process dispatch. |
| `synchronization_service_runtime` | Synchronization worker runtime | Runtime boundary only; business contracts remain provider-owned. |

## Administrator domain

| Crate | Responsibility | Internal boundary |
| --- | --- | --- |
| `server_admin` | Admin use cases and persistence | Authentication is split into API, HTML, accounts, roles, sessions, settings, users, shared policy, and generated data-table adapters; repository modules own SQL. |
| `server_admin_core` | Shared administrator domain types | Contains no transport or persistence implementation. |
| `server_admin_contract` | Typed admin route/page/permission catalogs | Kept as a declarative catalog because the generated consumers must change atomically. |
| `server_admin_frontend` | Admin CSR/SSR presentation | SSR rendering is split by page family; shared table cells and layout remain presentation-only. |
| `admin_bootstrap` | Administrator bootstrap command | One command boundary. |
| `development_data_bootstrap` | Development-only seed command | One command boundary; not linked into service startup. |

## Runtime and service infrastructure

| Crate | Responsibility | Internal boundary |
| --- | --- | --- |
| `server_runtime_core` | Runtime-independent lifecycle primitives | No HTTP or service-domain dependency. |
| `server_runtime_http` | Reusable HTTP/runtime infrastructure | Separate modules own outbound-client policy, error diagnostics, serving lifecycle, request timeouts, observability, request IDs, limits, CORS/CSP, cookies, multipart, outbound URLs, health, metrics, tracing, and notification transport. Its root is the reviewed public facade plus the remaining request/security composition layers. |
| `server_observability` | Tracing/metrics initialization | No service-domain behavior. |
| `server_app_state` / `app_state` | Application-state wrappers | State composition only; no route or repository behavior. |
| `common_routes` | Shared operational HTTP routes | Health/status transport shared by deployable services. |
| `route_validators` | Shared route precondition validation | Transport policy without service-domain persistence. |
| `file_storage` | Bounded file-storage operations | Storage boundary only. |
| `external_service_emulators` | Test/development service emulators | Never a production runtime dependency. |
| `runtime_tests` | Cross-crate runtime conformance | Test-only crate. |

## Configuration and contracts

| Crate family | Responsibility and boundary |
| --- | --- |
| `config_lib`, `config_lib_macros`, `generate_getter_traits_for_struct_fields`, `try_from_env` | Shared typed configuration values, derive generation, getter generation, and environment decoding respectively. Within `config_lib`, `admin` owns administrator numeric and token policy, `admin_jwt` owns JWT secret parsing and rotation access, `bool_flags` owns strict boolean flag decoding, `http` owns body-size and content-security-policy validation, and `pg_pool` owns pool sizing and shared positive timeout decoding; the crate root preserves the public configuration facade. |
| `server_config` | Application-server environment schema and production validation. |
| `notification_service_config` | Notification-service environment schema and production validation. |
| `notification_service_contract` | Provider-owned notification wire contract. |
| `frontend_contract`, `frontend_contract_macros`, `frontend_contract_validation` | Typed route contracts, their compile-time generation, and independent validation. Macro files are generation pipelines rather than runtime modules. |
| `str_constants`, `str_constants_macros` | Reviewed string catalog and its compile-time support. The catalog stays atomic to prevent competing sources of truth. |

## PostgreSQL CRUD generation

| Crate family | Responsibility and boundary |
| --- | --- |
| `pg_crud_common`, `pg_crud_common_macros` | Shared query/schema policy and its derive surface. Internal modules separate query construction, schema conformance, and validation. |
| `pg_crud_macros_common`, `pg_crud_macros_common_macros` | Shared proc-macro parsing and generation helpers. |
| `pg_table`, `generate_pg_table`, `generate_pg_table_src`, `generate_pg_table_test` | Table contract, proc-macro entry point, token emitter, and isolated generated-code tests. The large `source` module is one ordered emitter and is not runtime application logic. |
| `generate_pg_types`, `generate_pg_types_src`, `generate_pg_types_test` | PostgreSQL type proc-macro entry point, token emitter, and isolated generated-code tests. |
| `pg_types_common`, `pg_types_numeric`, `pg_types_text_misc`, `pg_types_chrono_net` | Type implementations split by semantic family. |
| `where_filters`, `generate_where_filters`, `generate_where_filters_src`, `generate_where_filters_test` | Filter contract, proc-macro entry point, token emitter, and generated-code tests. |
| `prepare_postgresql_databases` | Database preparation command; no application queries. |

## Foundation and macro support

| Crate family | Responsibility and boundary |
| --- | --- |
| `bounded_types`, `text_policy`, `token_patterns`, `token_patterns_macros` | Reusable validated domain primitives and token policy. `bounded_types` separates string, vector, hash-map, and B-tree-map storage/serde/schema implementations behind a stable crate facade. |
| `newtype` | Newtype derive macros only. |
| `naming`, `naming_common`, `naming_common_macros`, `naming_macros` | Naming domain, shared implementation, and compile-time frontends. |
| `location_lib`, `location`, `location_macros`, `location_test` | Location API, derive implementation, and isolated compile tests. |
| `to_err_string`, `to_err_string_macros`, `panic_location` | Error formatting and diagnostic-location support. |
| `workspace_macro_helpers`, `macros_helpers`, `generate_derive_token_stream_builder`, `generate_quotes`, `macro_clippy_check_common`, `optimal_memory_layout` | Compile-time support crates; none owns service runtime behavior. |
| `git_info` | Build/repository metadata only. |

## Repository tooling and verification

| Crate | Responsibility | Internal boundary |
| --- | --- | --- |
| `workspace_scaffold` | Scaffold and generated-projection command | `naming` owns project-name validation and case conversion; `template_fs` owns bounded traversal, copying, replacement, and marker insertion; `service_catalog` owns catalog parsing and CI/release matrix rendering; configuration/deployment projections and command dispatch remain private command-oriented groups. |
| `workspace_test_runner` | Workspace verification command | `admin_fixture` owns deterministic administrator contract fixture generation; discovery, execution, and reporting modules own reusable flow; allocation workloads and release orchestration remain command-local. |
| `initialize_environment_files` | Environment-file initialization command | One command boundary. |
| `tests` | Repository policy and conformance tests | Code-style checks are split by policy family; large source-policy files are declarative rule inventories. |
| `fuzz` | Fuzz targets | Test-only, excluded from production dependency flow. |

## Change rules

1. Put a new concern in the crate that owns its domain meaning; sharing identical syntax is not a
   reason to share ownership.
2. Split transport, persistence, runtime lifecycle, presentation, and policy when they can change
   independently.
3. Keep generated emitters and authoritative catalogs atomic unless their generated interfaces can
   also be separated.
4. A binary `main` may compose modules, but reusable logic belongs in its owning module or shared
   crate.
5. Add a new crate only for an independently owned boundary, never to reduce a line count.
