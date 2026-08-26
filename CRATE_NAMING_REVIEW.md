## Application, runtime, and operations crates

| Current package | Observed responsibility | Proposed package name |
|---|---|---|
| `administrator_account_initialization_and_password_reset` | One-time initial administrator creation executable | `administrator_account_initialization_and_password_reset_cli` |
| `dev_identity_creation_planner` | Plans desired development identities and sample data | `development_sample_data_creation_planner` |
| `external_service_emulators` | Deterministic integration-test substitutes for external services | `integration_test_external_service_emulators` |
| `file_storage` | Symlink-resistant staged filesystem storage | `secure_staged_file_storage` |
| `init_env_files` | Initializes environment files from examples | `workspace_environment_file_initializer_cli` |
| `notification_service` | Deployable notification microservice executable | `notification_delivery_service` |
| `notification_service_config` | Validated notification-service environment configuration | `notification_service_configuration` |
| `notification_service_contract` | Notification transport DTOs and API contract | `notification_service_transport_contract` |
| `prepare_pg_databases` | Plans PostgreSQL database preparation commands | `postgres_database_preparation_planner` |
| `server` | Main application server startup, routing, and maintenance executable | `application_server` |
| `server_admin` | Reusable administrator API, persistence, authentication, and UI application logic | `administrator_service` |
| `server_admin_contract` | Shared administrator HTTP DTOs, queries, routes, and sessions | `administrator_http_contract` |
| `server_admin_core` | Administrator domain wrapper types and invariants | `administrator_domain_types` |
| `server_admin_frontend` | Reusable Rust/Leptos administrator frontend and SSR UI | `administrator_web_frontend` |
| `server_config` | Aggregated validated configuration used by the server | `application_server_configuration` |
| `server_observability` | Shared tracing, diagnostics, initialization, and capture primitives | `service_observability_runtime` |
| `server_runtime_core` | Dependency-light jobs, queues, leases, retry, budgets, and execution primitives | `service_runtime_core` |
| `server_runtime_http` | HTTP server, middleware, health, security, and client primitives | `http_service_runtime` |
| `synchronization_service_runtime` | Facade over shared synchronization-service runtime capabilities | `data_synchronization_service_runtime` |
| `workspace_scaffold` | Workspace scaffolding and project-identity migration executable | `workspace_project_scaffolding_cli` |

## State, configuration, route, and frontend foundations

| Current package | Observed responsibility | Proposed package name |
|---|---|---|
| `app_state` | Generic typed application-state domain values | `typed_application_state` |
| `server_app_state` | Server-specific application-state composition | `application_server_state` |
| `server_app_state_macros` | Derives/builds server application-state wiring | `application_server_state_proc_macros` |
| `config_lib` | Validated admin, JWT, HTTP, PostgreSQL, and flag configuration types | `service_configuration_types` |
| `config_lib_macros` | Proc macros supporting service configuration types | `service_configuration_proc_macros` |
| `generate_accessor_traits_for_struct_fields` | Generates accessor traits for struct fields | `struct_field_accessor_trait_proc_macro` |
| `try_from_env` | Generates environment-variable conversion implementations | `environment_conversion_proc_macro` |
| `common_routes` | Shared route adapters and route domain types | `shared_http_routes` |
| `route_validators` | Request body, commit, and header validation utilities | `http_route_request_validators` |
| `frontend_contract` | Typed client metadata, problems, routes, coverage, and URL construction | `typed_frontend_route_contract` |
| `frontend_contract_macros` | Derive macros for typed route contracts | `frontend_route_contract_proc_macros` |
| `frontend_contract_validation` | OpenAPI, snapshot, HTTP, and route-contract validation | `frontend_route_contract_validation` |

## General domain utility crates

| Current package | Observed responsibility | Proposed package name |
|---|---|---|
| `bounded_types` | Bounded owned text, vector, map, and set values | `bounded_collection_and_text_types` |
| `naming` | Naming-domain types and parameter naming rules | `identifier_naming_rules` |
| `naming_common` | Shared implementation used by naming macros | `identifier_naming_codegen_support` |
| `naming_common_macros` | Proc macros used by common naming implementation | `identifier_naming_support_proc_macros` |
| `naming_macros` | User-facing naming proc macros | `identifier_naming_proc_macros` |
| `newtype` | Generates domain newtype implementations | `domain_newtype_proc_macro` |
| `optimal_memory_layout` | Derives or checks memory-efficient field layout | `memory_layout_optimization_proc_macro` |
| `panic_location` | Typed extraction/reporting of panic source locations | `panic_source_location` |
| `text_policy` | Shared validation policies for bounded textual values | `bounded_text_validation_policy` |
| `to_err_string` | Domain support for converting errors to strings | `error_string_conversion` |
| `to_err_string_macros` | Generates error-to-string implementations | `error_string_conversion_proc_macros` |
| `token_patterns` | Reusable parsed token-pattern domain values | `rust_token_pattern_types` |
| `token_patterns_macros` | Generates token-pattern implementations | `rust_token_pattern_proc_macros` |

## General macro infrastructure

| Current package | Observed responsibility | Proposed package name |
|---|---|---|
| `generate_quotes` | Builds reusable quoted Rust token fragments | `rust_token_quote_generation` |
| `macro_clippy_check_common` | Shared domain logic for linting generated macro output | `proc_macro_clippy_validation` |
| `macro_helpers` | Token-stream builders and reusable derive-generation operations | `proc_macro_codegen_support` |
| `generate_derive_token_stream_builder` | Generates derive token-stream builder implementations | `derive_token_stream_builder_proc_macro` |
| `workspace_macro_helpers` | Workspace-specific proc-macro helper domain logic | `workspace_proc_macro_codegen_support` |

## PostgreSQL CRUD runtime and shared code generation

| Current package | Observed responsibility | Proposed package name |
|---|---|---|
| `pg_crud_common` | Runtime CRUD values: filters, cursors, batches, locks, schemas, and errors | `postgres_crud_runtime_types` |
| `pg_crud_common_macros` | Proc macros for common PostgreSQL CRUD runtime values | `postgres_crud_runtime_proc_macros` |
| `pg_crud_macro_common` | Shared filter, test-case, and token-stream codegen implementation | `postgres_crud_codegen_support` |
| `pg_crud_macro_common_macros` | Proc macros supporting PostgreSQL CRUD generators | `postgres_crud_codegen_support_macros` |

## PostgreSQL table generation family

| Current package | Observed responsibility | Proposed package name |
|---|---|---|
| `pg_table` | Runtime domain model for generated PostgreSQL tables | `postgres_crud_table_types` |
| `generate_pg_table` | User-facing table-generation proc macro | `postgres_crud_table_proc_macro` |
| `generate_pg_table_src` | Parser and codegen for clients, handlers, routes, SQL, OpenAPI, and tests | `postgres_crud_table_codegen` |
| `generate_pg_table_test` | Fixtures/tests for generated table code | `postgres_crud_table_codegen_tests` |

## PostgreSQL type generation family

| Current package | Observed responsibility | Proposed package name |
|---|---|---|
| `pg_types_common` | Shared PostgreSQL type-domain definitions | `postgres_crud_type_common` |
| `pg_types_chrono_net` | PostgreSQL mappings for chronological and network types | `postgres_crud_chrono_network_types` |
| `pg_types_numeric` | PostgreSQL numeric type mappings | `postgres_crud_numeric_types` |
| `pg_types_text_misc` | PostgreSQL textual and miscellaneous type mappings | `postgres_crud_text_and_misc_types` |
| `generate_pg_types` | User-facing PostgreSQL type-generation proc macro | `postgres_crud_type_proc_macro` |
| `generate_pg_types_src` | Catalog parsing and model/filter/schema/serde/sqlx codegen | `postgres_crud_type_codegen` |
| `generate_pg_types_test` | Fixtures/tests for generated PostgreSQL type code | `postgres_crud_type_codegen_tests` |

## PostgreSQL WHERE-filter generation family

| Current package | Observed responsibility | Proposed package name |
|---|---|---|
| `where_filters` | Runtime PostgreSQL WHERE-filter domain values | `postgres_crud_where_filter_types` |
| `generate_where_filters` | User-facing WHERE-filter generation proc macro | `postgres_crud_where_filter_proc_macro` |
| `generate_where_filters_src` | WHERE-filter binding, schema, client, SQL, and source codegen | `postgres_crud_where_filter_codegen` |
| `generate_where_filters_test` | Fixtures/tests for generated WHERE-filter code | `postgres_crud_where_filter_codegen_tests` |

## Test and developer-tool crates

| Current package | Observed responsibility | Proposed package name |
|---|---|---|
| `workspace_fuzz` | Fuzz targets for workspace domain boundaries | `workspace_domain_fuzz_targets` |
| `runtime_tests` | Reusable tests plus runner for deployed workspace services | `deployed_service_runtime_test_support` |
| `tests` | Workspace architecture, style, dependency, contract, and deployment policy tests | `workspace_architecture_policy_tests` |
| `workspace_test_runner` | Discovers, executes, and reports workspace test suites | `workspace_test_orchestrator_cli` |
