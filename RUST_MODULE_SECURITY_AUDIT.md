# Rust module correctness and security audit

Audit snapshot: 2026-08-28, before remediation. This report inventories every Rust source and test module under every direct workspace crate. Generated files under `target/` are excluded because they are build artifacts, not source modules. Inventory size: 88 crates and 3079 Rust modules.

## Method and interpretation

Each module was included in a complete filesystem inventory and screened for correctness, memory/resource lifetime, unsafe/raw-memory operations, unbounded input reads, filesystem and process boundaries, network/HTTP behavior, SQL/database access, authentication/secrets, panic/unwrap use, task ownership, locks across await points, and dependency advisories. The audit also uses the workspace's 229 AST-based policy tests, full Clippy configuration, unit/integration/doc tests, `cargo tree`, and the RustSec advisory database. A row marked **Reviewed — no confirmed issue** means no concrete defect was established by those checks; it is not a claim that future defects are impossible.

Risk-sensitive modules received an additional source-pattern review and are labeled separately. Test/fixture modules are non-production but remain inventoried. Facades and declarative domain wrappers generally contain little executable behavior; their risk comes through the implementation modules they expose.

## Findings recorded before fixes

| ID | Severity | Scope | Category | Pre-fix result |
|---|---:|---|---|---|
| BR-001 | Medium | `server_runtime_http/src/read_bounded_file.rs`, `server_runtime_http/src/read_bounded_file_async.rs` | Availability / excessive allocation | Confirmed. Both functions checked metadata, then used whole-file reads. Concurrent file growth after metadata could allocate and read beyond the configured maximum before returning `ExceedsMaximum`. |
| DEP-001 | Low | Active dependency graph: `h2 0.4.15` through Hyper/Reqwest/Axum | Availability / memory exhaustion | Confirmed by RUSTSEC-2026-0258. Empty HTTP/2 DATA frames could be queued without a bound. Patched in `h2 >= 0.4.16`. |
| DEP-002 | Contextual | Lockfile-only `rkyv 0.7.46` | Out-of-bounds read | RUSTSEC-2026-0235 appears in `cargo audit`, but `cargo tree --target all -e all` has no reachable `rkyv` node. It is an optional package recorded for `rust_decimal`, not compiled by any workspace feature; no reachable vulnerability was established. |
| DEP-003 | Informational | Transitive `paste 1.0.15`, `proc-macro-error2 2.0.1`; yanked `chacha20 0.10.1` | Maintenance / supply chain | No exploit advisory was reported for these versions. They are transitive dependencies; replacement requires upstream framework/SQLx changes and is tracked as maintenance debt, not a confirmed workspace vulnerability. |
| CI-001 | High | Root `.gitignore`, `Cargo.lock`, and workflows using `--locked` | Build reproducibility / patch persistence | Confirmed. The root lockfile was ignored and untracked while CI required `--locked`. A fresh clone could not reproduce the tested graph, and DEP-001's patched resolution would not be delivered. |

No intentional memory leak or reachable unsafe/raw-memory implementation was found in workspace source. Matches for `unsafe`, leak APIs, and raw-pointer APIs were diagnostic strings or policy fixtures rather than executable operations.

## Complete module inventory

### `administrator_account_initialization_and_password_reset`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `administrator_account_initialization_and_password_reset/src/admin_command.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `administrator_account_initialization_and_password_reset/src/administrator_account_command_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `administrator_account_initialization_and_password_reset/src/administrator_account_command_exit_code.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `administrator_account_initialization_and_password_reset/src/administrator_account_command_status.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `administrator_account_initialization_and_password_reset/src/administrator_command_args_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `administrator_account_initialization_and_password_reset/src/administrator_password_file_path_buf.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `administrator_account_initialization_and_password_reset/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `administrator_account_initialization_and_password_reset/src/error_status.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `administrator_account_initialization_and_password_reset/src/initial_administrator_creation_args.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `administrator_account_initialization_and_password_reset/src/main.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `administrator_account_initialization_and_password_reset/src/parse_args.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `administrator_account_initialization_and_password_reset/src/password_from_bytes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `administrator_account_initialization_and_password_reset/src/password_from_file.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `administrator_account_initialization_and_password_reset/src/password_reset_args.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `administrator_account_initialization_and_password_reset/src/run.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `administrator_account_initialization_and_password_reset/src/sqlx_administrator_database_connection_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `app_state`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `app_state/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `app_state/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `app_state/src/sqlx_pg_pool.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `app_state/src/sqlx_pg_pool_provider.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `app_state/src/sqlx_pg_pool_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `bounded_types`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `bounded_types/src/bounded_b_tree_map.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `bounded_types/src/bounded_b_tree_map_visitor_phantom_data.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `bounded_types/src/bounded_hash_map.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `bounded_types/src/bounded_hash_map_visitor_phantom_data.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `bounded_types/src/bounded_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `bounded_types/src/bounded_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `bounded_types/src/bounded_value_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `bounded_types/src/bounded_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `bounded_types/src/bounded_vec_visitor_phantom_data.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `bounded_types/src/btree.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `bounded_types/src/collection_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `bounded_types/src/deserialize_bounded_map.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `bounded_types/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `bounded_types/src/hash.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `bounded_types/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `bounded_types/src/serde_prealloc_max_items.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `bounded_types/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `bounded_types/src/validate_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `bounded_types/src/vector.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `common_routes`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `common_routes/src/adapters.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/arc_common_routes_app_state.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/axum_common_routes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/axum_health_check_status.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/axum_http_uri_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `common_routes/src/axum_json_payload.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/common_no_body.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/common_not_found_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/common_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/common_route_registry.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/common_routes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/common_routes_open_api.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/common_routes_parameters.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/database_is_ready.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `common_routes/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `common_routes/src/domain_types_tests.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `common_routes/src/domain_types_tests_health.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/domain_types_tests_route_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/git_info.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/git_info_response.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/git_info_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_check.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_check_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_check_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_check_succeeded.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_component.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_component_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_components.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_components_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_components_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_database_available.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `common_routes/src/health_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_live.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_live_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_live_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_probe_timeout.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_ready.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `common_routes/src/health_ready_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `common_routes/src/health_ready_route.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `common_routes/src/health_report.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_report_response.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_status.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/health_unavailable_response.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/json_res.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `common_routes/src/make_commit_json_response.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/make_git_info_payload.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/make_json_response.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/make_no_route_message.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/make_no_route_message_for_suffix.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/make_not_found_payload.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/make_not_found_payload_with_message.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/map_health_check_status.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/no_route_message_capacity.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/not_found_payload.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/open_api.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/open_api_specification_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `common_routes/src/readiness_report.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `common_routes/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `common_routes/src/uri_suffix.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/uri_suffix_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `common_routes/src/utoipa_common_routes_open_api_document.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `config_lib`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `config_lib/src/admin.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/admin_access_token_ttl_seconds.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/admin_bool_parsing_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/admin_cookie_secure.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/admin_jwt.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/admin_jwt_secret.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/admin_jwt_secret_max_count.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/admin_jwt_secret_min_len.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/admin_login_failure_limit.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/admin_password_hash_concurrency.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/admin_positive_u64_parsing_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/admin_positive_usize_parsing_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/admin_refresh_token_ttl_seconds.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/admin_session_limit.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/admin_sign_in_rate_limit.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/admin_swagger_enabled.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/admin_token_audience.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/admin_token_issuer.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/bool_flags.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/chrono_east_fixed_offset.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/chrono_fixed_offset_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/chrono_timezone.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/config_example_validity.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/config_field_descriptor.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/config_field_example_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/config_field_requirement.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/config_field_sensitivity.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/config_lib_string_wrapper_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/config_lib_string_wrapper_try_from_string_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/config_non_zero_u64.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/config_non_zero_usize.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/config_rust_type_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/content_security_policy.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/content_security_policy_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `config_lib/src/env_parse_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/env_var_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/env_var_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/env_var_name_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/env_var_result_var_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/env_var_value_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/http.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/http_gzip_enabled.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/i32_parse_int_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `config_lib/src/maximum_size_of_http_body_in_bytes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/maximum_size_of_http_body_in_bytes_try_from_usize_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/parse_admin_positive_u64.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/parse_admin_token_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/parse_bool_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/parse_ctx_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/parse_east_fixed_offset.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/parse_env_var_name_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/parse_from_env_var_from_str.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/parse_from_env_var_with.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/parse_from_str_with_ctx.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/parse_from_str_with_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/parse_int_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/parse_pg_pool_non_zero_seconds.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/parse_required_env_var.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/pg_pool.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/pg_pool_acquire_timeout_seconds.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/pg_pool_config_parse_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/pg_pool_idle_timeout_seconds.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/pg_pool_max_connections.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/pg_pool_max_connections_try_from_u32_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/pg_pool_max_lifetime_seconds.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/pg_pool_min_connections.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/production_mode.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/request_timeout_seconds.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/secrecy_secret_box_string.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/src_place_type.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/std_config_secret_string.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/std_env_var_ok.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/std_env_var_ok_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/svc_mode.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `config_lib/src/timezone_seconds.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/tracing_format.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/tracing_level.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/tracing_level_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/try_from_std_env_var_ok.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/try_from_std_env_var_ok_admin_cookie_secure_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/try_from_std_env_var_ok_admin_jwt_secret_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/try_from_std_env_var_ok_admin_password_hash_concurrency_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/try_from_std_env_var_ok_admin_positive_u64_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/try_from_std_env_var_ok_admin_token_text_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/try_from_std_env_var_ok_maximum_size_of_http_body_in_bytes_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/try_from_std_env_var_ok_pg_pool_max_connections_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/try_from_std_env_var_ok_svc_mode_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/try_from_std_env_var_ok_timezone_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/try_map_non_empty_env_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/types.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `config_lib/src/u32_parse_int_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib/src/usize_parse_int_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `config_lib_macros`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `config_lib_config_lib_macros/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `config_lib_config_lib_macros/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `config_lib_config_lib_macros/src/proc_macro2_try_from_parse_fixed_error_ty.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib_config_lib_macros/src/proc_macro2_try_from_parse_input.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `config_lib_config_lib_macros/src/proc_macro_try_from_parse_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `generate_accessor_traits_for_struct_fields`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `config_lib_generate_accessor_traits_for_struct_fields/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |

### `try_from_env`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `config_lib_try_from_env/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |

### `constants_i32`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `constants_i32/src/lib.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |

### `constants_i64`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `constants_i64/src/lib.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |

### `constants_str`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `constants_str/src/catalog.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |
| `constants_str/src/integration_fixtures.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `constants_str/src/lib.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |
| `constants_str/src/test_fixtures.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |

### `constants_str_macros`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `constants_str_macros/src/collection_max_len.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |
| `constants_str_macros/src/constant.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |
| `constants_str_macros/src/constant_part.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |
| `constants_str_macros/src/constant_parts.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |
| `constants_str_macros/src/constants.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |
| `constants_str_macros/src/define_str_constants_input.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |
| `constants_str_macros/src/domain_types.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |
| `constants_str_macros/src/fragment.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |
| `constants_str_macros/src/fragments.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |
| `constants_str_macros/src/lib.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |
| `constants_str_macros/src/syn_ident.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |
| `constants_str_macros/src/syn_lit_str.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |
| `constants_str_macros/src/syn_visibility.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |

### `constants_u128`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `constants_u128/src/lib.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |

### `constants_u16`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `constants_u16/src/lib.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |

### `constants_u32`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `constants_u32/src/lib.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |

### `constants_u64`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `constants_u64/src/lib.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |

### `constants_u8`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `constants_u8/src/lib.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |

### `constants_usize`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `constants_usize/src/lib.rs` | Constant catalog | Reviewed — no executable resource lifetime or input-processing path; no confirmed issue. |

### `dev_identity_creation_planner`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `dev_identity_creation_planner/src/development_identity_count.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `dev_identity_creation_planner/src/development_identity_creation_plan.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `dev_identity_creation_planner/src/development_identity_creation_summary.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `dev_identity_creation_planner/src/development_identity_specs.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `dev_identity_creation_planner/src/development_identity_specs_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `dev_identity_creation_planner/src/development_identity_specs_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `dev_identity_creation_planner/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `dev_identity_creation_planner/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |

### `external_service_emulators`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `external_service_emulators/src/create_mock_notification_provider.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `external_service_emulators/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `external_service_emulators/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `external_service_emulators/src/mock_notification_inbox.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `external_service_emulators/src/mock_notification_provider.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `external_service_emulators/src/mock_notification_provider_closed.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `external_service_emulators/src/remote_sync_request_count.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `external_service_emulators/src/remote_sync_source.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `external_service_emulators/src/tokio_mock_notification_receiver.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `external_service_emulators/src/tokio_mock_notification_sender.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `file_storage`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `file_storage/src/adapters.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/atomic_replace_durability.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/disk_cache_budget_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/disk_cache_entry.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/disk_cache_eviction_plan.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/disk_cache_modified_at_system_time.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `file_storage/src/file_storage_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/file_storage_io_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/file_storage_path_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/file_storage_root_path_buf.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/file_storage_staging_area.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `file_storage/src/plan_disk_cache_eviction.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/safe_file_storage.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/stale_before_system_time.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/stale_staging_cleanup_cfg.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/stale_staging_cleanup_cfg_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/stale_staging_cleanup_report.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/std_disk_cache_size.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/std_file_bytes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/std_stale_staging_entry_count.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/std_stale_staging_entry_limit.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/std_storage_operation_id.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/storage_directory_name_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/storage_path_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/storage_relative_path_buf.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `file_storage/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |

### `frontend_contract`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `frontend_contract/src/action_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/action_contracts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/api_problem.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/api_problem_detail.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/api_problem_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/api_problem_field.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/api_problem_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/api_problem_request_id.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/api_problem_status.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/api_problem_violation.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/api_problem_violations.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/api_url.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/api_url_build_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/api_url_component_encode_set.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/api_url_path_segment_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/api_url_query_component_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/apply_openapi_error_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/apply_openapi_path_parameter_contract.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/apply_openapi_request_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/apply_openapi_security_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/apply_openapi_success_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/auth_session_instant.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/auth_session_keep_alive.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/auth_session_keep_alive_decision.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/auth_session_keep_alive_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/auth_session_presence.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/auth_session_refresh_interval_duration.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/auth_session_refresh_outcome.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/auth_session_refresh_state.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/authenticated_transport.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/authentication_requirement.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/axum_method_filter.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/axum_route_method_router.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/capability_support.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/client.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/client_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/client_request.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/client_route_metadata.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/confirmation_requirement.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/contract_i64.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/contract_str.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/covered_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/create_form_value_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/decode_api_problem.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `frontend_contract/src/empty_filter_contracts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/field_capability.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/field_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/field_contracts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/field_order.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/field_placeholder.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/field_visibility.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/filter_contracts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/filter_form_value_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/filter_operation.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/filter_value_shape.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/filter_wire_json.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/form_field_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/form_field_name_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/form_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/form_value_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/form_value_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/form_value_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/frontend_contract_body_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/functions.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `frontend_contract/src/has_filter_contracts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/has_type_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/http_method.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/http_status.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/http_status_try_from_u16_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/input_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/input_step.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/known_http_status.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `frontend_contract/src/missing_required_test_categories.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/mutation_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/nullability.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/numeric_bound.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/open_api_security_scheme_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/openapi_route_metadata.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/operation_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/page_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/page_transport.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/parameterized_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/parameterized_route_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/parameterized_route_path_try_from_string_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/primary_key_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/problem.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/public_transport.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/register_openapi_route_schemas.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/register_openapi_schema.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/registered_route_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/required_test_categories.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_access.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/route_body_limit.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_contracts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_coverage.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_coverage_descriptor.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_coverage_descriptors.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_coverage_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_coverage_evidence.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_coverage_obligation.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_database_usage.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/route_error_policy.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_error_status.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_family.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_in_family.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_json_body_usage.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_metadata.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_metadata_list.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_method.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_method_router.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_mutation.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_registration_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_request.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_request_body.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_response.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_response_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_schema_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_schema_contracts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_test_capabilities.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_test_categories.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_test_category.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/route_transport.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/server_response.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/server_route_metadata.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/success_status.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `frontend_contract/src/to_axum_method_filter.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/transport.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/transport_body.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/transport_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/transport_idempotency_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/transport_if_match.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/transport_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/transport_request.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/transport_response.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/transport_retry_after.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/transport_status.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/type_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/typed_client.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/typed_parameterized_route_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/typed_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/typed_route_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/url_builder.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/utoipa_open_api_components_ref_mut.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/utoipa_open_api_path_parameter.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract/src/utoipa_open_api_ref_mut.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/utoipa_open_api_route_schema.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/validate_route_coverage.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/value_example.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/src/value_format.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract/tests/typed_route.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |

### `frontend_contract_macros`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `frontend_contract_macros/src/contract_struct_api_args.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/contract_struct_api_field_args.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `frontend_contract_macros/src/endpoint_registry_args.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/endpoint_registry_binding.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `frontend_contract_macros/src/page_catalog_args.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/page_catalog_page_args.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/route_catalog_args.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/route_catalog_route_args.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/route_registry_args.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/route_registry_binding.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/std_bool.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/syn_attributes_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/syn_endpoint_registry_bindings.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/syn_endpoint_registry_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/syn_endpoint_registry_endpoint.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/syn_endpoint_registry_state.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/syn_expr.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/syn_ident.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/syn_route_registry_bindings.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/syn_route_registry_endpoint.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/syn_route_registry_family.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/syn_route_registry_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/syn_route_registry_schemas.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/syn_route_registry_state.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/syn_type.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/syn_typed_route_errors.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `frontend_contract_macros/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `frontend_contract_macros/src/typed_route_args.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `frontend_contract_validation`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `frontend_contract_validation/src/artifact.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/canonical_json_contract_snapshot.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `frontend_contract_validation/src/http_contract_body.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/http_contract_body_kind.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/http_contract_expectation.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/http_contract_mismatch.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/http_contract_observation.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/http_contract_status.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/json_contract_snapshot.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/json_contract_snapshot_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/json_snapshot_dynamic_field_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `frontend_contract_validation/src/open_api_contract_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/open_api_contract_text_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/open_api_operation_expectation.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/open_api_operation_validation_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/open_api_payload_validation_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/open_api_response_status.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/open_api_schema_mismatch.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/open_api_schema_references_b_tree_set.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/open_api_security_expectation.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/open_api_validation_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/openapi_schema_references.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/openapi_validation.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/route_contract_mismatch.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/route_contract_mismatches.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/route_contract_validation.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/run_http_contract_fixture.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `frontend_contract_validation/src/runtime_routes_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/serde_json_open_api_serialization_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/validate_openapi_contract.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/validate_openapi_json_payload.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/validate_openapi_operations.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/validate_openapi_schema_references.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/validate_route_contract_metadata.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `frontend_contract_validation/src/validate_typed_route_contract.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `workspace_fuzz`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `fuzz/fuzz_targets/domain_boundaries.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `generate_quotes`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `generate_quotes/src/binary_double_quote_style.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/binary_double_quoted_str.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/binary_double_quoted_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `generate_quotes/src/binary_single_quote_style.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/binary_single_quotes_str.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/binary_single_quotes_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `generate_quotes/src/build_quote_style.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `generate_quotes/src/double_quote_style.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/double_quoted_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/dq_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `generate_quotes/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `generate_quotes/src/proc_macro2_quoted_literal_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `generate_quotes/src/quote_char.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/quote_literal.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/quote_panic_id.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/quote_prefix.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/quote_str.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/quote_style.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/quote_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `generate_quotes/src/quoted_literal.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/quoted_literal_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/single_quote_style.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/single_quotes_str.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `generate_quotes/src/single_quotes_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `git_info`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `git_info/src/base_git_commit_link_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/build_git_commit_link.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/build_git_commit_link_cow.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/check_is_project_commit.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `git_info/src/functions.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `git_info/src/git_commit_id.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/git_commit_id_cow.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/git_commit_id_fallback.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/git_commit_id_provider.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/git_commit_id_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/git_commit_link.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/git_commit_link_capacity.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/git_commit_link_capacity_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/git_commit_link_cow.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/git_commit_link_output_ref_mut.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/git_commit_link_provider.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/git_info_string_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/git_info_string_try_from_string_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/is_project_commit.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `git_info/src/project_git_commit_link.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/project_git_commit_link_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/project_git_commit_link_ref_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/project_git_info.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/project_git_info_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `git_info/src/validate_project_commit.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/validate_project_commit_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/with_git_commit_id_ref_or.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `git_info/src/write_git_commit_link.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `init_env_files`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `init_env_files/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `init_env_files/src/env_content.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/env_content_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/env_key.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/env_keys.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/environment_keys.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/init_entries.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/init_io_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/init_max_bytes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/init_path_exists.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/init_path_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/init_string_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/initialization_entry.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/initialization_status.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/initialize.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/initialize_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/main.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `init_env_files/src/path_exists.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/read_bounded_content.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/run.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/run_mode.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/server_runtime_bounded_read_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/toml_init_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/workspace_member.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/workspace_root_path_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `init_env_files/src/write_content.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `location_lib`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `location_lib/src/chrono_location_date_time.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `location_lib/src/chrono_location_display_timezone.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `location_lib/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `location_lib/src/formatter_ref_mut.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `location_lib/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `location_lib/src/location.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `location_lib/src/location_column.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `location_lib/src/location_column_non_zero_u32.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `location_lib/src/location_commit.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `location_lib/src/location_coordinate_try_from_u32_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `location_lib/src/location_duration.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `location_lib/src/location_file.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `location_lib/src/location_file_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `location_lib/src/location_line.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `location_lib/src/location_line_non_zero_u32.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `location_lib/src/occr.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `location_lib/src/std_time_duration.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `location_lib/src/std_time_duration_nanos.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `location_lib/src/std_time_duration_nanos_try_from_u32_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `location_lib/src/std_time_duration_secs.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `location_lib/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |

### `location`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `location_lib_location/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `location_lib_location/src/syn_item_enum_mut_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `location_macros`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `location_lib_location_macros/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |

### `location_test`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `location_lib_location_test/src/create_location_test_text.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `location_lib_location_test/src/display_struct.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `location_lib_location_test/src/domain_types.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `location_lib_location_test/src/error_one.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `location_lib_location_test/src/error_two.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `location_lib_location_test/src/error_unnamed_one.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `location_lib_location_test/src/loc_test_text_max_len.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `location_lib_location_test/src/location_test_count.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `location_lib_location_test/src/location_test_flag.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `location_lib_location_test/src/location_test_text.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `location_lib_location_test/src/main.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `location_lib_location_test/src/run.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `location_lib_location_test/src/serde_struct.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |

### `macro_clippy_check_common`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `macro_clippy_check_common/src/generated_crate_phase.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_clippy_check_common/src/generated_crate_step.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_clippy_check_common/src/generated_crate_steps.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_clippy_check_common/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `macro_clippy_check_common/src/remove_dir_on_drop.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `macro_helpers`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `macro_helpers/src/assert_file_content.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/assert_file_path_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/attr_identifier_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/attr_identifier_str.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/attribute_identifier_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/cleanup_test_file.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/compile_error_message.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/compile_error_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/contract_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/derive_token_stream_builder.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `macro_helpers/src/ensure_json_contract_round_trip.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/expected_file_content.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/expected_file_content_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/field_location_column.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/field_location_column_non_zero_u32.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/field_location_coordinate_try_from_u32_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/field_location_file.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/field_location_line.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/field_location_line_non_zero_u32.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/find_macro_attribute.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/format_with_cargofmt.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/generate_const_new_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_const_try_new_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_field_location_new_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_if_write_is_error_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_impl_const_new_for_identifier_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_impl_default_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_impl_display_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_impl_from_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_impl_modified_new_for_identifier_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_impl_modified_try_new_for_identifier_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_impl_new_for_identifier_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_impl_pub_const_new_for_identifier_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_impl_pub_const_try_new_for_identifier_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_impl_pub_new_for_identifier_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_impl_pub_try_new_for_identifier_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_impl_to_err_string_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_impl_try_from_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_impl_try_new_for_identifier_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_modified_new_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_modified_try_new_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_new_or_try_new.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/generate_new_or_try_new_tests.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/generate_new_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_pub_const_new_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_pub_const_try_new_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_pub_new_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_pub_try_new_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_pub_type_alias_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_serde_version_of_named_syn_variant.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generate_simple_syn_punct.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/generate_try_new_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/generated_file_maximum_bytes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/get_macro_attribute.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/impl_identifier_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/json_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/json_fixture_ref.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `macro_helpers/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `macro_helpers/src/location.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/location_field_attr.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/location_syn_field.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/macro_attr_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/only_one.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/only_one_status_code_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/os_string_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/pagination_start_end_initialization_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/path_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/proc_macro2_derive_tokens_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/proc_macro2_generated_rust_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/proc_macro2_if_write_is_err_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/proc_macro2_macro_attr_meta_list_token_stream_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/proc_macro2_token_stream_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/process_command.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/process_exit_status.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/process_output.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/rs_file_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/rs_file_path_buf.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/sanitized_database_target.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/serde_json_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/should_write_string.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/should_write_string_into_file.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/should_write_token_stream_into_file.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/status_code.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/std_assert_file_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/string_file_content_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/string_syn_punct.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/syn_field.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/syn_field_identifier.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/syn_field_type.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/syn_field_vis.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/syn_location_field.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/syn_macro_attr_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/syn_path_segment.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/syn_path_segments.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/syn_status_code_variant_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/syn_variant_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/test_database.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/test_helper.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/test_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/test_path_stem.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/test_path_stem_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/tool_arg_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/tool_args_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/tool_command.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/tool_env_key_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/tool_env_value_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/tool_program_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/try_get_macro_attr_meta_list_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/try_get_macro_attribute.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/try_maybe_write_token_stream_into_file.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/try_write_string_into_file.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/try_write_string_into_file_with_outcome.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/try_write_string_into_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/try_write_string_into_path_with_outcome.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/url_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/url_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/validate_existing_file_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/validate_test_database_url.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/with_attr_token_stream_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/wrap_derive.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `macro_helpers/src/write_path_outcome.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/write_string_if_needed.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/write_string_into_file.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/write_token_stream_into_file.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/written_file_path_buf.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers/src/written_file_path_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `generate_derive_token_stream_builder`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `macro_helpers_generate_derive_token_stream_builder/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `macro_helpers_generate_derive_token_stream_builder/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `macro_helpers_generate_derive_token_stream_builder/src/snake_case_string.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `macro_helpers_generate_derive_token_stream_builder/src/to_snake_case_input.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `naming`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `naming/src/display_plus_to_tokens.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `naming/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `naming/src/hash_map.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `naming/src/hash_map_snake_case.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `naming/src/hash_map_upper_camel_case.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `naming/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `naming/src/parameter.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `naming/src/swagger_url_path_prefix.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `naming/src/swagger_url_path_self_quotes_str.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `naming/src/swagger_url_path_self_quotes_str_value.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `naming/src/swagger_url_path_self_quotes_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `naming/src/swagger_url_path_self_quotes_token_stream_value.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `naming_common`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `naming_naming_common/src/case_from_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `naming_naming_common/src/case_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `naming_naming_common/src/case_string_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `naming_naming_common/src/convert_case_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `naming_naming_common/src/display_case_str.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `naming_naming_common/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `naming_naming_common/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `naming_naming_common/src/proc_macro2_case_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `naming_naming_common/src/str_case.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `naming_naming_common/src/to_token_stream_or_panic.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `naming_naming_common/src/tokenized_case_str.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `naming_common_macros`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `naming_naming_common_macros/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |

### `naming_macros`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `naming_naming_macros/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `naming_naming_macros/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `naming_naming_macros/src/proc_macro2_generated_naming_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `naming_naming_macros/src/proc_macro2_variant_matching_tokens_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `naming_naming_macros/src/syn_enum_identifier_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `newtype`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `newtype/src/bounded_string_attrs.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/bounded_string_option.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `newtype/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `newtype/src/newtype_attrs.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/newtype_bool.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/newtype_option.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/newtype_try_from_attrs.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/proc_macro2_generated_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `newtype/src/proc_macro_input_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `newtype/src/snake_ident_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/snake_identifier.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/snake_identifierifier_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/snake_identifierifier_try_from_string_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/syn_derive_input_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/syn_expr.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/syn_identifier.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/syn_identifier_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/syn_type.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/syn_type_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `newtype/src/to_err_string_mode.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/src/wire_enum_attrs.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `newtype/tests/newtype.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |

### `notification_service`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `notification_service/src/axum_notification_json.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/axum_notification_response.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/axum_notification_router.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/axum_notification_state.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/create_notification.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `notification_service/src/create_notification_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `notification_service/src/http_notification_status_code.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `notification_service/src/main.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `notification_service/src/metrics.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/metrics_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/metrics_exporter_prometheus_notification_build_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/metrics_exporter_prometheus_renderer.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/migrate_notification.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `notification_service/src/notification_api_route_registry.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/notification_body_maximum_bytes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/notification_config_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/notification_error_code.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/notification_exit_code.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `notification_service/src/notification_io_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/notification_observability_init_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/notification_observability_shutdown_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/notification_route_registry.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/notification_serve_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/notification_service_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/notification_state.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `notification_service/src/open_api.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/open_api_document.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/router.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/routes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service/src/run.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `notification_service/src/sqlx_notification_database_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `notification_service/src/sqlx_notification_migration_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `notification_service/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |

### `notification_service_config`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `notification_service_config/src/config.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service_config/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `notification_service_config/tests/config_descriptor.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |

### `notification_service_contract`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `notification_service_contract/src/create_notification_req.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service_contract/src/create_notification_res.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service_contract/src/create_notification_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service_contract/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `notification_service_contract/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `notification_service_contract/src/notification_api_body_max_bytes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service_contract/src/notification_message.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service_contract/src/notification_message_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service_contract/src/notification_message_try_from_string_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service_contract/src/notification_operational_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service_contract/src/notification_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `notification_service_contract/src/uuid_notification_id.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `optimal_memory_layout`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `optimal_memory_layout/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |

### `panic_location`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `panic_location/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `panic_location/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `panic_location/src/panic_column.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `panic_location/src/panic_file.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `panic_location/src/panic_line.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `panic_location/src/panic_with_location_message.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `pg_crud_common`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_common/benches/query_builders.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/add_operator.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/all_enum_variants.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/all_enum_variants_array_default_some_one_element.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/all_enum_variants_array_default_some_one_element_max_page_size.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/batch_duplicate_policy.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/batch_invalid_item_count.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/batch_invalid_items.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/batch_processed_item_count.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/batch_records_b_tree_map.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/batch_stopped_early.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/batch_validation.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/batch_validation_report.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/bind_index.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/bool_test_cases_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/bounded_b_tree_map.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/bounded_b_tree_map_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/bounded_btree_map.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/bounded_unique_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/bounded_unique_vec_visitor_phantom_data.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/bounded_vec.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/bounded_vec_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/bounded_vec_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/build_date_sql_filter.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/build_pg_scoped_foreign_key_clause.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/build_sql_like_pattern.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/build_stable_read_query_plan.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/bulk_mutation_outcome.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/chrono_utc_date_time_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/chrono_utc_date_times.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/classify_pg_code.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/classify_pg_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/classify_slice_ordering.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/contains_duplicate_identifier.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/cursor_codec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/cursor_codec_build_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/cursor_decode_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/cursor_encode_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/cursor_maximum_length.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/cursor_maximum_length_non_zero_usize.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/cursor_pagination_usage.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/cursor_payload.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/cursor_payload_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/cursor_signing_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/cursor_signing_key_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/cursor_signing_key_maximum_length.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/data_invariant_violation.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/date_filter_bounds.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/date_sql_bind_start_non_zero_u32.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/date_sql_filter.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/date_sql_filter_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/db_catalog_snapshot.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_column_contract_snapshot.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_column_contract_snapshots.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_column_has_server_default.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_column_nullable.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_column_snapshot.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_column_snapshots.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_column_spec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_column_specs.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_default_spec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_default_specs.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_extended_table_schema.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_key_contract_snapshot.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_key_contract_snapshots.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_key_spec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_key_specs.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_object_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_object_snapshot.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_object_snapshots.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_object_spec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_object_specs.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_schema_conformance.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/db_schema_conformance_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_schema_name_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_schema_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_schema_text_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_schema_texts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_static_schema_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_static_schema_texts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_table_name_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_table_schema.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/db_table_snapshot.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/deduplicate_preserving_order_by_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/default_some_one_element.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/default_some_one_element_max_page_size.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_common/src/domain_types_db_schema_conformance_tests.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/domain_types_query_pagination_tests.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/domain_types_tests_operator_to_query_part.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/duplicate_candidates.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/duplicate_idx.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/eq_operator.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/eq_operator_query_str.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/errors.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/f32_test_cases_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/f64_test_cases_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/filter_bind_plan.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/finite_f64.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/finite_f64_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/first_duplicate_index.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/first_duplicate_index_by_hash.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/i16_test_cases_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/i32_test_cases_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/i64_test_cases_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/i8_test_cases_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/increment_checked_add_one_returning_increment.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/inspect_postgres_catalog.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/inspect_postgres_table.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/is_primary_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/is_string_empty.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/is_string_empty_res.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_common/src/list_items.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/list_offset.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/list_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/list_rows.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/list_rows_presence.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/list_total.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/list_total_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/list_total_source.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/lock_pg_relation_resources.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/make_query_bind_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/maximum_resource_count.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/maximum_scoped_foreign_key_columns.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/minimum_scoped_foreign_key_columns.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/non_primary_key_pg_type_read_ids.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/not_empty_unique_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/not_empty_unique_vec_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/not_empty_unique_vec_try_new_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/not_zero_unsigned_part_of_i32.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/not_zero_unsigned_part_of_i32_non_zero_i32.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/not_zero_unsigned_part_of_i32_try_from_i32_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/nullable_json_obj_pg_type_where_filter.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/offset_pagination_presence.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/operation_budget.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/operation_budget_exceeded.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/operation_count.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/operator.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/order.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/order_by.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/order_preserving_values.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/order_snake_case_str.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/order_text_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/order_upper_camel_case_str.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pagination.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pagination_base.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pagination_end.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pagination_limit.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pagination_offset.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pagination_policy.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pagination_start.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pagination_starts_with_zero.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pagination_starts_with_zero_raw.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pagination_starts_with_zero_try_new_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pagination_total.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/patch_field.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_column_schema.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_counter_reconciliation.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_counter_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_crud_string_wrapper_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_crud_string_wrapper_try_from_string_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_duplicate_identifier_presence.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_error_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_filter_bind_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_filter_bool.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_filter_i64.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_filter_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_filter_text_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_operational_limit.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_operational_limit_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_operational_limit_non_zero_u64.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_operational_limit_update_authority.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/pg_relation_capacity_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_relation_capacity_maximum.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_relation_capacity_maximum_non_zero_u64.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_relation_lock_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/pg_relation_lock_namespace.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/pg_relation_resource_id.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_relation_resource_ids.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_relation_row_count.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_scoped_foreign_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_scoped_foreign_key_clause_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_scoped_foreign_key_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_scoped_foreign_key_on_delete.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_sql_identifiers.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/pg_type.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/pg_type_eq_operator.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_type_greater_than_test.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_type_greater_than_variant.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_type_len_greater_than_test.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_type_not_primary_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_type_primary_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_type_test_cases.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_type_where.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/pg_type_where_filter.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/positive_finite_f64.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/positive_finite_f64_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/push_identifier_list.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/query_fragment.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/query_part_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/query_part_fragment.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/query_part_increment.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/query_part_increment_mut.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/query_sort_order.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/read_query_bind_index_non_zero_u32.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/read_query_plan.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/read_query_plan_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/reconcile_pg_counter.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/resolve_list_total_source.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/resolve_pg_operational_limit_update.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/run_list_with_total.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/schema_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/schema_texts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/serde_prealloc_max_items.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/signed_cursor.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/signed_cursor_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/signed_cursor_presence.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/single_or_multiple.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/slice_ordering.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/sql_column_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sql_identifier.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sql_identifier_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sql_identifier_list_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sql_identifiers.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sql_like_input_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sql_like_match_mode.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sql_like_pattern.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sql_like_pattern_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sql_qualified_identifier.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sql_query_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sql_select_builder.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sql_sort_order_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sqlx_box_dyn_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sqlx_db_schema_inspection_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sqlx_pg_error_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sqlx_pg_pool_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sqlx_pg_relation_lock_connection_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sqlx_pg_relation_lock_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sqlx_postgres_query.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/sqlx_postgres_query_bind_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/static_schema_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/static_schema_texts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/std_bounded_b_tree_map_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/string_test_cases_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/take_fst_dup.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/take_fst_dup_by_hash.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/transaction_failure.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/try_new_unique_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/u16_test_cases_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/u32_test_cases_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/u64_test_cases_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/u8_test_cases_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/unique_vec_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/unique_vec_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/unit_interval_f64.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/unit_interval_f64_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/unsigned_part_of_i32.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/unsigned_part_of_i32_raw.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/unsigned_part_of_i32_try_from_i32_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/uuid_uuid_test_cases.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/uuid_uuid_test_cases_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/v.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/validate_batch_by_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/validate_bulk_atomicity.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/validate_generated_postgres_table.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/validate_migration_idempotency.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/validate_operation_budget.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/validate_pagination_invariants.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/validate_pg_relation_capacity.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/validate_postgres_catalog.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/validate_postgres_table_extensions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/validate_postgres_table_schema.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_common/src/validate_snapshot.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_common/src/window_total_presence.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `pg_crud_common_macros`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_common_macros/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |

### `pg_crud_macro_common`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_macro_common/src/common_d_token_stream_builder.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/de_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/default_some_one_or_default_some_one_with_max_page_size.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/derive_or_impl.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/dimension.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/dimension_index_number.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/dimension_number.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_macro_common/src/domain_types_tests.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/domain_types_token_emission_tests.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/emission_types.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/eq_operator_variant.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/eq_or_eq_using_fields.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/error_enum_d_token_stream_builder.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/filters.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/generate_de_double_quoted_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_dimension_number_pagination_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_if_let_some_match_ok_assign_query_or_return_err_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_all_variants_default_some_one_element_max_page_size_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_all_variants_default_some_one_element_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_crate_is_string_empty_for_identifier_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_de_for_struct_by_fields_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_de_for_struct_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_default_some_one_element_max_page_size_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_default_some_one_element_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_display_and_to_err_string_debug_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_pg_crud_all_variants_default_some_one_element_max_page_size_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_pg_crud_all_variants_default_some_one_element_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_pg_crud_common_all_variants_default_some_one_element_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_pg_crud_common_default_some_one_element_max_page_size_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_pg_crud_common_default_some_one_element_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_pg_crud_default_some_one_element_max_page_size_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_pg_crud_default_some_one_element_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_pg_type_not_primary_key_for_identifier_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_pg_type_test_cases_for_identifier_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_pg_type_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_sqlx_decode_sqlx_pg_for_identifier_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_sqlx_encode_sqlx_pg_for_identifier_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_sqlx_type_and_encode_for_identifier_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_sqlx_type_for_identifier_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_impl_to_err_string_no_generics_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_match_not_empty_unique_vec_try_new_some_or_none_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_match_ok_assign_or_return_err_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_match_ok_or_return_err_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_match_try_new_in_de_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_mod_with_pub_use_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_optional_type_declaration_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_pg_type_where_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_query_part_error_write_into_buffer_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_read_ids_and_create_into_vec_where_eq_using_fields_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_read_ids_and_create_into_where_eq_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_read_inner_into_read_or_update_with_new_or_try_new_unwraped_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_return_err_query_part_error_write_into_buffer_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_sqlx_types_json_type_declaration_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_struct_identifier_double_quoted_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_struct_identifier_with_number_els_double_quoted_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_v_declaration_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_v_initialization_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/generate_vec_tokens_declaration_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/impl_pg_type_eq_operator_for_identifier_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/impl_pg_type_where_filter_for_identifier_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/import.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/import_path_str.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/import_snake_case_str.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/is_nl_prefix_str_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/is_nullable.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/is_nullable_prefix_str.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/is_standard_non_null.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_macro_common/src/maybe_wrap_into_braces_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/names_ctx.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/non_null_or_nullable_str.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/panic_uuid_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/parse_error_id_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/parse_strs_to_ts2_vec.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/parse_token_stream_strings.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/pg_crud_common_query_part_error_checked_add_initialization_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/pg_crud_common_query_part_error_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/pg_filter.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/pg_type_filter.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/pg_type_test_cases.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/proc_macro2_generated_rust_token_stream_vec.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/read_or_update.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/serde_error_enum_d_token_stream_builder.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/struct_els_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/syn_field_refs.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/syn_identifier_type_refs.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/token_emission.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/token_stream_helpers.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_macro_common/src/wrap_into_braces.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_macro_common/src/wrap_into_scopes_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `pg_crud_macro_common_macros`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_macro_common_macros/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |

### `pg_table`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_pg_table/src/add_uo_optimistic_revision_predicate.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/begin_pg_table_idempotency.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table/src/calculate_pg_table_idempotency_request_hash.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table/src/cleanup_pg_table_idempotency.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table/src/combination_of_app_state_logic_traits.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/complete_pg_table_idempotency.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table/src/complete_pg_table_idempotency_in_connection.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_pg_table/src/domain_types_tests_idempotency.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/ensure_pg_table_idempotency_schema.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table/src/functions.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_pg_table/src/generate_cm_query_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/generate_co_query_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/generate_column_eq_v_comma_uo_query_part.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/generate_delete_query_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/generate_dlo_query_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/generate_dm_query_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/generate_insert_query_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/generate_rm_query_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/generate_ro_query_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/generate_select_query_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/generate_um_query_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/generate_uo_query_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/generate_update_query_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/generate_when_column_id_then_v_um_query_part.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/insert_values_fmt.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_pg_table/src/new_pg_table_idempotency_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_actor.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_begin.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_body.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_body_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_body_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_cleanup_batch_size.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_cleanup_batch_size_non_zero_i64.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_cleanup_retention_seconds.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_cleanup_rows.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_cleanup_value_try_from_i64_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_known_response_status.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_method.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_replay.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_request.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_request_hash.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table/src/pg_table_idempotency_response_status.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_response_status_try_from_u16_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_scope.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_text_bytes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_idempotency_text_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_name_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_query_part_fragment.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_query_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_revision.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table/src/pg_table_revision_parse_int_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table/src/pg_table_revision_try_from_string_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_table_sql_fragment_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table/src/pg_table_string_wrapper_try_from_string_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_tbl_idempotency_route_max_bytes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_tbl_idempotency_text_max_bytes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/pg_tbl_string_wrapper_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/release_pg_table_idempotency.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table/src/select_where_fmt.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table/src/sqlx_pg_table_idempotency_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table/src/sqlx_pg_table_pg_connection_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `pg_crud_pg_table/src/update_selector_fmt.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `generate_pg_table`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_pg_table_generate/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |

### `generate_pg_table_src`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_pg_table_generate_src/src/build_generate_pg_table.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/compile_error_message.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/compile_error_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table_generate_src/src/contract_tests.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_pg_table_generate_src/src/emit_generate_pg_table.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table_generate_src/src/frontend_http_method.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table_generate_src/src/frontend_operation_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/frontend_permission_action.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table_generate_src/src/frontend_success_status.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/generate_pg_table.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/generate_pg_table_field_count.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/generate_pg_table_model.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/generate_pg_table_pipeline_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/http_method.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table_generate_src/src/idempotency_capable.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_pg_table_generate_src/src/openapi.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/openapi_http_method.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table_generate_src/src/openapi_success_status.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/operation_dsc.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/optimistic_concurrency_capable.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/parse_generate_pg_table.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table_generate_src/src/pipeline.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/route_http_method.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table_generate_src/src/route_operation_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/route_permission_action.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table_generate_src/src/route_success_status.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/source.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/sql.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table_generate_src/src/struct_shape.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/success_status.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/syn_built_generate_pg_table_input.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/syn_generate_pg_table_model_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/syn_generate_pg_table_model_input.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/syn_generate_pg_table_pipeline_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/syn_parsed_generate_pg_table_input.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_table_generate_src/src/syn_validated_generate_pg_table_input.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/table.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/table_test_names.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_table_generate_src/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `pg_crud_pg_table_generate_src/src/validate_generate_pg_table.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `generate_pg_table_test`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_pg_table_generate_test/src/lib.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |

### `pg_types_chrono_net`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_pg_types_chrono_net/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |

### `pg_types_common`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_pg_types_common/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_pg_types_common/src/is_primary_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_common/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_pg_types_common/src/maybe_primary_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_common/src/pagination_starts_with_one.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_common/src/pagination_starts_with_one_raw.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_common/src/pagination_starts_with_one_try_new_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_common/src/pagination_starts_with_one_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `generate_pg_types`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_pg_types_generate/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |

### `generate_pg_types_src`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_pg_types_generate_src/src/build_generate_pg_types.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/built_generate_pg_types_model.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/can_be_nullable.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/can_be_primary_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/contract_tests.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_types_generate_src/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_pg_types_generate_src/src/emit_generate_pg_types.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_types_generate_src/src/filter_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/functions.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_pg_types_generate_src/src/generate_pg_type_records.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/generate_pg_types.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/generate_pg_types_config.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/generate_pg_types_config_variant.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/generate_pg_types_length_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/generate_pg_types_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/generate_pg_types_pipeline_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/generate_pg_types_tokens.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_types_generate_src/src/generate_secret_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_types_generate_src/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_pg_types_generate_src/src/parse_generate_pg_types.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_types_generate_src/src/parsed_generate_pg_types_config.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_types_generate_src/src/pg_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/pg_sql_name.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_types_generate_src/src/pg_type.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/pg_type_can_be_nullable.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/pg_type_can_be_primary_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/pg_type_deserialize.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_types_generate_src/src/pg_type_filter_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/pg_type_impl_new_for_deserialize_or_try_new_for_de.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_types_generate_src/src/pg_type_impl_try_new_for_de.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/pg_type_initialization_try_new.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/pg_type_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/pg_type_pattern.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/pg_type_record.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/pg_type_record_raw.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/pg_type_spec.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_types_generate_src/src/pg_types_model_entry_count.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/range.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/rust_type_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/rust_type_wire_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/schema_wire_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/serde_json_generate_pg_types_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_types_generate_src/src/serde_wire_kind.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_types_generate_src/src/source.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/sqlx.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_pg_types_generate_src/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `pg_crud_pg_types_generate_src/src/validate_generate_pg_types.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/validated_generate_pg_types_config.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_pg_types_generate_src/src/wire_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `generate_pg_types_test`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_pg_types_generate_test/src/lib.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |

### `pg_types_numeric`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_pg_types_numeric/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |

### `pg_types_text_misc`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_pg_types_text_misc/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |

### `where_filters`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_where_filters/src/between.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_where_filters/src/between_try_new_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters/src/bounded_vec.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_where_filters/src/bounded_vec_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters/src/bounded_vec_try_new_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters/src/default_regex_pattern.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_where_filters/src/encode_format.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_where_filters/src/pg_type_not_empty_unique_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters/src/regex_case.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters/src/regex_case_postgreql_syntax.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters/src/regex_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters/src/regex_regex.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters/src/regex_regex_try_from_string_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `pg_crud_where_filters/src/variant.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `generate_where_filters`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_where_filters_generate/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |

### `generate_where_filters_src`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_where_filters_generate_src/src/bind_count.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/bind_count_matches.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/build_generate_where_filters.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/built_generate_where_filters_model.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/client.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/client_text_search_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_where_filters_generate_src/src/client_uses_text_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_where_filters_generate_src/src/emit_generate_where_filters.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_where_filters_generate_src/src/filter_placeholder_count.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/filter_spec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/filter_spec_contract_is_valid.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/filter_spec_valid.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/filter_sql_operator.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_where_filters_generate_src/src/filter_sql_operator_value.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_where_filters_generate_src/src/filter_sql_suffix.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_where_filters_generate_src/src/filter_sql_suffix_value.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_where_filters_generate_src/src/filter_value_shape.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/generate_where_filters.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/generate_where_filters_pipeline_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `pg_crud_where_filters_generate_src/src/parse_generate_where_filters.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_where_filters_generate_src/src/parsed_generate_where_filters_config.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_where_filters_generate_src/src/proc_macro2_generate_where_filters_input.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/proc_macro2_generate_where_filters_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_where_filters_generate_src/src/schema.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/schema_text_search_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_where_filters_generate_src/src/schema_uses_text_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/serde_json_generate_where_filters_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_where_filters_generate_src/src/source.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/spec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/sql.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_where_filters_generate_src/src/text_search_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `pg_crud_where_filters_generate_src/src/validate_generate_where_filters.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `pg_crud_where_filters_generate_src/src/validated_generate_where_filters_config.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `generate_where_filters_test`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `pg_crud_where_filters_generate_test/src/lib.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |

### `prepare_pg_databases`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `prepare_pg_databases/src/database_preparation_spec.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `prepare_pg_databases/src/database_url.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `prepare_pg_databases/src/database_url_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `prepare_pg_databases/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `prepare_pg_databases/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `prepare_pg_databases/src/migration_commands.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `prepare_pg_databases/src/migrations_source.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `prepare_pg_databases/src/migrations_source_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `prepare_pg_databases/src/process_argument.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `prepare_pg_databases/src/process_arguments.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `prepare_pg_databases/src/process_command.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `prepare_pg_databases/src/process_commands.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `prepare_pg_databases/src/process_program.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `prepare_pg_databases/src/process_static_argument.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `prepare_pg_databases/src/validate_database_url.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `prepare_pg_databases/src/validate_migrations_source.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `route_validators`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `route_validators/src/assert_err_status_code.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/assert_err_status_code_only.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/assert_err_status_code_variant_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/assert_ok_eq.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/assert_panics.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/axum_body.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/axum_body_size_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/axum_commit_to_str_conversion_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/axum_header_value_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/axum_headers_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/axum_http_status_code.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `route_validators/src/axum_http_status_code_provider.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `route_validators/src/axum_test_header_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/axum_test_headers.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/axum_test_headers_mut_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/block_on.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `route_validators/src/body_size_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/body_size_limit_bytes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/bytes_body_bytes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/check_body_size.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/check_commit.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/commit_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/commit_header_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/commit_not_eq_message.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/commit_to_use.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `route_validators/src/enable_api_git_commit_check.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/expect_err_variant_ref_with_status.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/expect_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/expect_error_mapped.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/expect_error_variant_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/expect_ok.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/expect_variant.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/expect_variant_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/header_str_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/header_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/http_body_size_hint.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `route_validators/src/increment_block_on_poll_count.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `route_validators/src/insert_header_no_prev.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/is_block_on_poll_limit_reached.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `route_validators/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `route_validators/src/make_headers_with_entry.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/map_err.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/map_err_after_status_check.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/map_or_panic_unexpected_variant.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/max_block_on_polls.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `route_validators/src/no_commit_header_message.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/non_utf8_header_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/panic_unexpected_result.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/panic_unexpected_variant.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/read_commit_header_str.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `route_validators/src/replace_header_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/required_header_str.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/required_header_str_parsed.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `route_validators/src/required_header_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/test_exp_id.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/test_helper.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/test_panic_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/test_poll_count.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/test_poll_limit_reached.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/validate_commit_header.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `route_validators/src/validate_commit_header_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `runtime_tests`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `runtime_tests/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `runtime_tests/src/http_runtime_test_status.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `runtime_tests/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `runtime_tests/src/main.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `runtime_tests/src/reqwest_runtime_test_client.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `runtime_tests/src/reqwest_runtime_test_response.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `runtime_tests/src/runtime_test_config.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `runtime_tests/src/runtime_test_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `runtime_tests/src/runtime_test_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `runtime_tests/src/runtime_test_report.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `runtime_tests/src/runtime_test_url.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `runtime_tests/src/service_base_url.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `runtime_tests/src/service_base_url_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `server`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `server/src/admin_metrics_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/axum_api_routes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/configuration.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server/src/frontend_fallback_routes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/http_body_maximum_bytes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server/src/interval.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/main.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server/src/make_postgresql_pool.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server/src/metrics_exporter_prometheus_build_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/metrics_exporter_prometheus_renderer.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/migrate_server.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/mount_service_routes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/routes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/run_server.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server/src/run_server_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/server_admin_auth_svc_state_build_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server/src/server_admin_cleanup_cfg_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/server_admin_migrate_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/server_config_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/server_config_production_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/server_exit_code.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server/src/server_io_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/server_observability_init_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/server_observability_shutdown_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/server_runtime_background_task_shutdown_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server/src/server_runtime_content_security_policy_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/server_runtime_request_timeout_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/server_runtime_run_interval_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/server_runtime_serve_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/server_runtime_trusted_proxy_ranges_parse_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server/src/shared_server_app_state_arc.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server/src/sqlx_server_pg_connect_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server/src/tokio_server_runtime.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `server_admin`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `server_admin/src/account_change_own_password.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/account_me.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/account_me_context_view_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/action_result_impl.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/adapters_repository_data_tables.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/adapters_repository_data_tables_tests.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/adapters_repository_roles.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_access_claims.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_access_token_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_active_administrator_count.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_api_open_api.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_audit_action.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_audit_query.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_audit_query_parts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_audit_resource.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_audit_resource_id.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_audit_success_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_auth_collection_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_auth_collection_max_len.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_auth_html_routes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_auth_permissions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_auth_policy.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_auth_positive_value_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_auth_req.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_auth_route_registry.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_auth_svc_state.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_auth_svc_state_build_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_cleanup_batch_size.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_cleanup_cfg.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_cleanup_cfg_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_cleanup_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_cleanup_report.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_cleanup_retention_seconds.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_cleanup_rows.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_cookie_kind.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_cookie_max_age_seconds.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_cookie_secure.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_crud_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_db_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_error_response_parts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_generated_auth_layer.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_generated_auth_service.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_generated_route_contract.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_generated_table.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_generated_tables_validation_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_generated_token.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_html_action_route_registry.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_action_router.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_auth_action_route_registry.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_html_auth_action_router.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_html_form_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_form_key_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_form_selected_max_items.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_form_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_form_text_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_open_api.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_page_route_registry.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_page_router.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_role_action_route_registry.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_role_action_router.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_session_action_route_registry.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_html_session_action_router.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_html_sessions_page.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_html_settings_action_route_registry.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_settings_action_router.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_swagger_enabled.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_swagger_route_registry.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_user_action_route_registry.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_html_user_action_router.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_jwt_secret.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_migrate_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_migrate_error_inner.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_migrator.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_new_password_from_contract.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_observed_error_code.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_opaque_token.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_page_total_count.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_password.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_password_change_required.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_password_from_contract.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_password_hash.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_password_hash_concurrency.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_password_hash_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_password_hasher.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_password_reset_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_password_try_from_string_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_peer_addr.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_permissions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_rate_limit_scope.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_recent_login_failure_count.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_refresh_token.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_repository_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_role_names.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_role_permissions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_roles.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_secret_text_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_session_bundle.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_session_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_session_id.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_session_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_shared_semaphore_arc.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_sign_in_json.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_sign_in_user.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_system_settings.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_token_hash.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_unix_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/admin_user_roles.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/admin_users.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_audit_log.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_branding.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_change_own_password.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/api_create_role.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_create_user.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_data_table.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_data_tables.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_delete_role.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_delete_user.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_export_audit_log.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_list_permissions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/api_list_roles.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_list_users.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_me.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_refresh.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_revoke_all_sessions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/api_revoke_session.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/api_sessions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/api_set_role_permissions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/api_set_user_ban.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_set_user_password.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/api_set_user_roles.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_settings.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_sign_in.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_sign_out.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_update_role.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_update_settings.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/api_update_user.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/append_cleared_session_cookies.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/append_session_cookies.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/application_auth.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/application_html_actions.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/application_html_actions_auth.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/application_html_actions_roles.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/application_html_actions_sessions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/application_html_actions_settings.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/application_html_actions_users.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/application_html_forms.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/application_html_pages.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/application_html_tests.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/application_roles.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/application_tests.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/application_tests_helper.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/application_users.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/argon2_admin_password_hash_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/assignment_action.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/assignment_ids_impl.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/audit_export_log.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/audit_query_log.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/authenticated_action_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/authenticated_admin.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/authenticated_admin_contract.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/authenticated_selected_form_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/authn_apply_refresh_failure_delay.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/authn_refresh.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/authn_sign_in.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/authn_sign_out.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/authorization_authenticate.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/authorization_authorize_generated_request.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/authorization_hash_refresh_token_with_context.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/authorization_origin_is_present_and_allowed.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/authorization_session_context_hash.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/authorization_validate_csrf.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/authorize_custom.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/axum_admin_auth_router.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/axum_admin_form.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/axum_admin_json.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/axum_admin_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/axum_admin_query.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/axum_admin_response.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/axum_admin_state_router.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/base_sql.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/build_admin_cookie.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/change_password.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/change_password_form.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/cleanup_admin_tables.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/clear_admin_cookie.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/create_initial_administrator.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/create_role.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/create_role_form.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/create_session_in_connection.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/create_user.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/create_user_form.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/crud_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/crud_resource_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/csr_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/data_columns.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/data_filter.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/data_flt.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/data_flt_json.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/data_permissions_flt.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/data_role_permissions_flt.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/data_roles_flt.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/data_system_settings_flt.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/data_tables.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/data_tables_get.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/data_tables_list.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/data_user_roles_flt.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/data_users_flt.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/decode_access_token.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/delete_role.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/delete_user.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_admin/src/domain_types_generated_tables_tests.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/domain_types_tests.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/encode_access_token.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/enforce_rate_limit.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/extractors.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/filtered_sql.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/find_admin_cookie.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/form_auth_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/generated_auth.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/generated_open_api.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/generated_routes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/generated_tables.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/hash_opaque_token.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/html.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/html_page_error_impl.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/html_response_impl.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/html_routes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/html_routes_with_swagger.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/http_admin_header_map.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/http_admin_header_map_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/http_admin_header_value_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/initial_administrator_creation_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/insert_audit_success.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/insert_user.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/json_response.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/jsonwebtoken_admin_decoding_keys.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/jsonwebtoken_admin_encoding_key.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/jsonwebtoken_admin_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/last_admin_state.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_admin/src/load_authenticated_admin.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/load_authenticated_admin_from_db.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/lock_last_admin.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/maintenance.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/map_repository_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/map_unique_violation.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/migrate_create_initial_administrator.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/migrate_reset_admin_password.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/migrations.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/migrator.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/mutations_set_ban.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/mutations_set_password.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/mutations_set_permissions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/mutations_set_roles.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/optional_setting_impl.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/page_context_impl.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/page_total.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/password.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/permission_ids_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/permissions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/persistence.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/prepare_postgresql.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/profile.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/queries_list_permissions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/queries_roles_page.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/queries_users_page.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/query_audit_log.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/rate_limit.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/rbac.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/read.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/read_last_admin_state.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/read_settings.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/record_audit_success_in_connection.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/record_login_attempt.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/replace_role_permissions_outcome.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/replace_user_roles_outcome.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/repository.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/repository_page_total.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/reset_admin_password.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/revoke_access_session.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/revoke_refresh_token.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/revoke_session.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/revoke_session_form.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/revoke_user_sessions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/role_id_form.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/role_ids_impl.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/role_mutations_create.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/role_mutations_delete.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/role_mutations_update.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/role_path_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/role_permissions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/role_permissions_form.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/role_queries_list.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/roles.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/roles_create_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/roles_manage_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/root.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/routes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/security.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/sessions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/sessions_revoke_all_sessions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/sessions_revoke_session.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/settings.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/settings_branding.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/settings_branding_view.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/settings_branding_view_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/settings_form.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/settings_get.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/settings_update.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/shared.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/shared_admin_auth_svc_state_arc.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/shared_admin_generated_table_state_arc.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/sign_in.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/sign_in_form.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/sign_in_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/sign_out.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/sqlx_admin_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/sqlx_admin_migrate_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/sqlx_admin_migrator_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/sqlx_admin_pg_connection_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/sqlx_admin_repository_connection_mut_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/sqlx_admin_repository_pool_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/state.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/std_admin_access_token.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/std_admin_access_ttl_seconds.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/std_admin_access_ttl_seconds_non_zero_u64.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/std_admin_cookie.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/std_admin_failure_delay_millis.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/std_admin_failure_threshold.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/std_admin_html_selected.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/std_admin_html_selected_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/std_admin_rate_limit_count.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/std_admin_rate_limit_window_seconds.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/std_admin_refresh_ttl_seconds.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/std_admin_refresh_ttl_seconds_non_zero_u64.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/std_admin_session_limit.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/std_admin_session_limit_non_zero_usize.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/success_redirect_impl.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/swagger_router.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `server_admin/src/token.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/tokio_admin_acquire_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/tokio_admin_join_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/tokio_admin_owned_semaphore_permit.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/update_role.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/update_role_form.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/update_settings.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/update_user.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/update_user_form.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/update_user_password.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/user_ban.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/user_ban_form.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/user_id_form.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/user_mutations_create.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/user_mutations_delete.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/user_mutations_update.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/user_password.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/user_password_form.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/user_path_impl.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/user_queries_list.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/user_roles.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/user_roles_form.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/users.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/users_create_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/users_manage_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/utoipa_admin_auth_open_api.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin/src/utoipa_admin_open_api.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/validate_catalog_schema.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/validate_table_sort.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/src/version.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin/tests/admin_api/data_tables.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `server_admin/tests/admin_api/flow.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `server_admin/tests/admin_api/html.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `server_admin/tests/admin_api/maintenance.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `server_admin/tests/admin_api/policy.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `server_admin/tests/admin_api/routing.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `server_admin/tests/admin_api/schema.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `server_admin/tests/admin_api.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |

### `server_admin_contract`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `server_admin_contract/src/admin_api_body_max_bytes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_api_route_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_audit_cursor.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_audit_details_bytes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_audit_details_max_bytes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_audit_details_too_large.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_audit_export.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_audit_export_csv.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_audit_export_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_audit_log_id.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_audit_log_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_audit_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_audit_timestamp.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_audit_view.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_audit_views.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_bool.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_bounded_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_branding_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_branding_view.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_change_own_password_req.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_change_own_password_route.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_collection_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_collection_max_items.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_create_role_req.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_create_role_res.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_create_role_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_create_user_req.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_create_user_res.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_create_user_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_column.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_columns.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_columns_csv_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_filter.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_filters.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_input_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_order_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_row.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_rows.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_table.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_table_catalog.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_table_filter_query.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_table_frontend_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_data_table_query.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_table_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_table_spec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_table_str_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_table_view.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_tables.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_data_tables_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_default_page_limit.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_default_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_delete_role_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_delete_user_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_display_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_empty_collection.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_filter_field.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_filter_operation_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_filter_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_frontend_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_html_action.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_id_try_from_i64_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_list_permissions_route.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_list_roles_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_list_users_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_login.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_main_logo.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_me_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_new_password.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_no_body.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_open_api_vec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_open_api_vec_phantom_data.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_optional_setting.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_optional_settings.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_organization_contacts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_organization_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_page_capability.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_page_client_mode.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_page_limit.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_page_limit_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_page_limit_visitor.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_page_metadata.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_page_navigation.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_page_offset.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_page_offset_visitor.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_page_path_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_page_spec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_page_title.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_page_total.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_parameterized_route_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_password.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_path_route_name.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_permission.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_permission_id.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_permission_ids.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_permission_requirement.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_permission_str_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_permission_summaries.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_permission_summary.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_permission_value.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_permission_values.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_permissions_page.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_primary_color.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_refresh_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_revoke_all_sessions_route.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_revoke_session_route.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_role_id.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_role_ids.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_role_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_role_names.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_role_summaries.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_role_summary.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_roles_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_route_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_route_path_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_session_identifier.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_session_timestamp.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_session_view.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_session_views.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_sessions_page.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_sessions_route.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_set_role_permissions_req.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_set_role_permissions_route.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_set_user_ban_req.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_set_user_ban_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_set_user_password_req.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_set_user_password_route.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_set_user_roles_req.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_set_user_roles_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_setting.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_setting_input_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_setting_label.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_setting_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_setting_optionality.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_setting_spec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_settings_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_settings_view.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_sign_in_req.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_sign_in_res.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_sign_in_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_sign_out_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_site_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_sort_direction.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_support_url.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/admin_tab_title.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_table_query.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_table_search.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_table_sort_field.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_table_sort_field_try_from_key_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_table_sort_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_table_sort_key_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_table_sort_values.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_texts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_update_role_req.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_update_role_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_update_settings_req.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_update_settings_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_update_user_req.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_update_user_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_user_id.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_user_summaries.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_user_summary.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/admin_users_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/audit_branding.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/authenticated_admin.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/authorization_catalog.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/collections.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/default_admin_api_body_max_bytes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_admin_contract/src/domain_types_dto_tests.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/domain_types_query_tests.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/domain_types_routes_tests.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/domain_types_sessions_tests.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/domain_types_settings_tests.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/domain_types_tests.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/dto.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/identifier.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/identity.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_admin_contract/src/positive_non_zero_i64.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/query.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/routes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/serde_json_admin_audit_details.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/sessions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_contract/src/settings.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_contract/src/table_sort.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `server_admin_core`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `server_admin_core/src/admin_audit_log_id.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_core/src/admin_id_try_from_i64_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_core/src/admin_non_zero_usize.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_core/src/admin_permission_id.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_core/src/admin_permission_name.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_core/src/admin_resource_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_core/src/admin_role_id.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_core/src/admin_socket_addr.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_core/src/admin_user_id.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_core/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_admin_core/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_admin_core/src/secrecy_admin_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_core/src/std_admin_bool.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_core/src/std_admin_str_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_core/src/std_admin_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_core/src/uuid_admin_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `server_admin_frontend`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `server_admin_frontend/src/admin_alert.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_alert_dialog.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_alert_variant.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_api_url.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/admin_api_url_with_suffix.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/admin_app.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_assets_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_badge.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_badge_variant.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_button.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_button_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_button_link.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_button_variant.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_card.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_card_description.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_card_footer.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_card_header.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_card_title.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_card_variant.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_change_password.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/admin_checkbox.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_csr_api_url.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/admin_csr_api_url_suffix_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/admin_csr_query.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_csrf_token.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/admin_data_grid.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_data_grid_column.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_data_grid_filter.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_data_grid_filter_option.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_data_grid_input_type.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_data_grid_row.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_data_table_grid.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_empty.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_field.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_field_label.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_filter_hidden_inputs.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_filter_range_end.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_filter_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_http_status.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/admin_input.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_input_group.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_input_kind.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_input_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_joined_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_joined_text_try_from_string_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_load_state.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_mutation_method.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_navigation_link.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_page_nav_disabled.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_page_range.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_pagination.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_permissions_view.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/admin_profile_account.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/admin_profile_view.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/admin_role_permissions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/admin_roles_view.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_route_path_url.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/admin_sessions_view.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/admin_setting_disabled.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_setting_input.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_setting_input_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_setting_inputs.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_setting_required.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_setting_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_setting_textarea.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_settings_form_signals.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_settings_form_values.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_settings_view.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_sidebar.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_sidebar_item.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_spinner.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_ssr_error_message.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_ssr_html.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_ssr_html_try_from_string_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_ssr_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_ssr_text_try_from_string_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_ssr_view_ext.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_table_load_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_table_query_direction.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_table_query_hidden_inputs.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_textarea.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_user_roles.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/admin_users_view.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/axum_admin_frontend_router.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/crud.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/crud_render_role_create.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/crud_render_role_manage.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/crud_render_shell.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/crud_render_user_create.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/crud_render_user_manage.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/csr_admin_nav.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/csr_admin_role_row.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/csr_admin_user_row.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/csr_page_from_location.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/csrf_token.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/data_grid.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/data_table_grid.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/data_tables.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_admin_frontend/src/domain_types_shared_settings.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types_shared_settings_input.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types_ssr_document.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types_ssr_tests.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types_ssr_tests_document.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types_ssr_tests_navigation.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types_ssr_tests_settings.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types_start_http_mutation.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/domain_types_start_mutation.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types_with_owner_alert.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types_with_owner_badge.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types_with_owner_button.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types_with_owner_card.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types_with_owner_field.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types_with_owner_input.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types_with_owner_navigation.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types_with_owner_table.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/domain_types_with_owner_tests.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/fetch_json.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/fetch_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/filter.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/http.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/join_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/leptos_admin_filter_operation_signal.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/leptos_admin_input_signal.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_admin_frontend/src/location.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/page_render.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/page_render_with_access.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/page_render_with_table_access.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/pagination.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/reload_after.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render_admin_csr.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render_admin_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render_admin_page_with_access.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/render_admin_page_with_table_access.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/render_admin_permissions_page.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/render_admin_profile_page.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/render_admin_sessions_page.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/render_admin_settings_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render_data_tables.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render_data_tables_csr.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render_document.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render_permissions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/render_profile.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/render_role_create.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render_role_manage.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render_roles.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render_sessions.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/render_settings.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render_sign_in.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render_text_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render_text_page_with_access.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/render_user_create.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render_user_manage.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render_users.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/render_view.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/reset.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/routes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/save.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/send_json.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/shared.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/show_mutation_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/ssr.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/ssr_admin_nav.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/ssr_admin_role_row.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/ssr_admin_user_row.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/start.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/state.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/static_pages.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/table.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/table_body.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/table_caption.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/table_cell.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/table_filters.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/table_footer.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/table_head.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/table_header.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/table_pagination.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/table_row.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/table_wrapper.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/text_page.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/url.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_admin_frontend/src/values.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_admin_frontend/src/with_owner.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `server_app_state`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `server_app_state/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_app_state/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_app_state/src/make_test_server_app_state.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_app_state/src/server_app_state.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_app_state/src/test_env.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `server_app_state_macros`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `server_app_state_server_app_state_macros/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |

### `server_config`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `server_config/src/config.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_config/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_config/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_config/src/production_config_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_config/tests/config_descriptor.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |

### `server_observability`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `server_observability/src/capture.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_observability/src/init_service_observability.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_observability/src/initialization.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_observability/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_observability/src/observability_guard.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_observability/src/observability_init_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_observability/src/observed_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_observability/src/observed_error_backtrace.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_observability/src/observed_error_code.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_observability/src/opentelemetry_otlp_exporter_build_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_observability/src/opentelemetry_sdk_observability_shutdown_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_observability/src/opentelemetry_sdk_tracer_provider.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_observability/src/service_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_observability/src/service_tracing_format.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_observability/src/std_panic_location.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_observability/src/tracing_observed_error_span_trace.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_observability/src/tracing_subscriber_init_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `server_runtime_core`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `server_runtime_core/src/arc_single_flight_rw_lock.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_core/src/async_run_history.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/async_run_history_maximum_len_non_zero_usize.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/async_run_history_snapshot.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/background_job.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/bounded_secret_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_core/src/bounded_secret_text_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_core/src/bulk_item_resource_budget_provider.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/calculate_resource_utilization.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/collections_hash_set.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_core/src/collections_vec_deque.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/critical_percent.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/deduplicating_queue.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_runtime_core/src/exclusive_run.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/exclusive_run_already_active.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_core/src/exclusive_run_atomic_bool.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/exclusive_run_guard.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/execute_plan.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/execution_mode.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/execution_plan.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/execution_report.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/generation.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/generation_atomic_u64.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/generation_commit.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/generation_gate.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/history.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/idempotency_response_resource_budget_provider.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/identity_creation_decision.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/identity_creation_plan.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/identity_presence.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/identity_role_presence.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/identity_spec.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/lease_entry.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/lease_heartbeat.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/lease_id.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/lease_ids.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/lease_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/lease_registry.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/lease_registry_inner.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/lease_registry_maximum_non_zero_usize.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/lease_reservation.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/lease_stale_timeout_duration.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/lease_state.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/lease_text_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/lease_text_maximum_bytes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/lease_text_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_runtime_core/src/plan_identity_creation.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/queue_maximum_non_zero_usize.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/queue_push.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/reject_non_essential_writes_percent.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_core/src/resource_amount.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/resource_budget.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/resource_budget_amount.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/resource_budget_config_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/resource_budget_maximum.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/resource_budget_maximum_non_zero_usize.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/resource_budget_reservation.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/resource_budget_reserve_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/resource_utilization.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/resource_utilization_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/resource_utilization_known_percent.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/resource_utilization_percent.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/resource_utilization_percent_try_from_u8_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/resource_utilization_status.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/retry.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/retry_attempts_non_zero_usize.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/retry_delay_duration.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/retry_outcome.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/retry_policy.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/run_reports_vec_deque.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/run_with_retries.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/secret_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_core/src/secret_text_match.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_core/src/secret_text_minimum_bytes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_core/src/secret_text_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_core/src/secret_texts_match.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_core/src/select_sources.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/shared_atomic_usize_arc.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/shared_run_reports_arc.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/single_flight.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/single_flight_acquire.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/single_flight_inner.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/single_flight_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/single_flight_key_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/single_flight_key_maximum_bytes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/single_flight_maximum_non_zero_usize.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/single_flight_owner.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/single_flight_rw_lock_write_guard.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_core/src/single_flight_signal.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/single_flight_wait_outcome.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/single_flight_waiter.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/source_selection.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/source_selection_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/std_async_run_history_maximum_len_try_from_usize_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/std_async_run_history_report_count.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/std_lease_stale_timeout_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/std_retry_attempts_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/tokio_lease_instant.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/tokio_lease_registry_rw_lock_arc.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_core/src/tokio_single_flight_receiver.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/tokio_single_flight_sender.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/validate_lease_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/warning_percent.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `server_runtime_core/src/write_inner.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `server_runtime_http`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `server_runtime_http/src/abort_and_wait_task.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/acquire_permit.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/acquire_permit_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/add_health_routes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/add_status_route.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/allow_origin_suffix.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/allowed_origin.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/allowed_origin_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/allowed_origins.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/allowed_origins_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/arc_tokio_semaphore.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/axum_notification_json.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/axum_notification_router.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/axum_notification_state.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/axum_router.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/background_task.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/background_task_outcome.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/background_task_shutdown_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/batched_cleanup.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/bearer_authorization_resolution.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/bounded_bytes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/bounded_json_read_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/bounded_json_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/bounded_read.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/bounded_read_concurrency_arc_semaphore.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/bounded_read_concurrency_maximum_non_zero_usize.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/bounded_read_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/bounded_read_from_utf8_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/bounded_read_io_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/bounded_read_maximum_bytes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/bounded_read_observed_bytes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/bounded_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/build_attachment_content_disposition.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/build_secure_strict_cookie.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/build_service_runtime.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/capture_without_context.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/child_diagnostic.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/child_diagnostic_maximum_non_zero_usize.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/child_exit_status.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/child_process.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/child_process_completion.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/child_process_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/child_process_id.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/child_process_io_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/child_process_report.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/child_process_reports.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/child_process_set.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/child_process_set_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/child_process_set_maximum_non_zero_usize.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/child_process_succeeded.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/child_process_supervisor.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/classify_http_error_status.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/classify_not_found_io_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/classify_optional_json_content_type.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/cleanup_batch_count.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/cleanup_batch_size.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/cleanup_batch_size_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/cleanup_batch_size_non_zero_u64.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/cleanup_completion.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/cleanup_continuation.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/cleanup_report.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/cleanup_rows.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/client_addr_parse_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/client_ip.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/client_socket_addr.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/content_disposition_percent_encode_set.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/cookie_resolution.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/cors.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/cors_allow_origin_max_bytes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/cors_allow_origin_max_items.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/cors_allow_origin_split_ch.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/csp.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_runtime_http/src/domain_types_request_id.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/domain_types_security_headers.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/domain_types_service_runtime.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/domain_types_tests_request_id.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/domain_types_tests_resource_budget.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/domain_types_tests_security_headers.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/domain_types_tests_service_runtime.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/enforce_pg_rate_limit.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/ensure_size_within_limit.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/extract_remote_trace_context.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/fallback.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/fallback_response_mode.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/file_staging_action.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/file_staging_directory_name.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/forwarded_proto_trust.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/geo_json_document_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/geo_json_validation.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/geo_json_validation_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/geojson.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/header_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/health.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/health_component_status.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/health_probe_succeeded.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/health_probe_timeout_duration.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/health_readiness.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/health_ready_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/health_snapshot.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_accept_header_maximum_bytes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_allowed_path_prefix_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_attachment_file_name_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_authorization_header_text_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_bearer_token_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_client.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_content_disposition.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_content_disposition_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_content_length.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_content_length_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_content_security_policy.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_content_security_policy_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_content_type_text_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_cookie_access.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_cookie_headers_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_cookie_name.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_cookie_name_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_cookie_secure.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_cookie_value.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_cookie_value_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_cors_allow_origin_header_values.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_cors_allow_origin_header_values_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_cors_allow_origin_text_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_csp_builder.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_csp_directive_name.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_csp_directive_value.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_csp_maximum_bytes_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_csp_token_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_error_class.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_error_code.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_error_diagnostic.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_error_status.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_error_telemetry.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_error_type.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_error_without_diagnostic_context.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_fallback_api_prefix_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_fallback_metrics_path_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_fallback_request_path_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_header_extractor.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_header_injector.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_header_map_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_header_name.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_header_policy.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_header_text_bytes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_header_text_maximum_bytes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_header_text_maximum_bytes_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_header_text_maximum_bytes_non_zero_usize.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_header_text_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_header_text_resolution.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_header_to_str_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_host_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_method_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_metrics_layer.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_metrics_path_cache.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_metrics_path_cache_maximum.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_metrics_path_cache_maximum_non_zero_usize.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_metrics_path_cache_maximum_try_from_usize_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_metrics_path_entries_rw_lock.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_metrics_path_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_metrics_path_text_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_metrics_path_text_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_metrics_service.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_metrics_tower_layer.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_normalized_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_normalized_path_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_notification_header_map.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_opentelemetry_header_map_mut.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_opentelemetry_header_map_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_optional_accept_header_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_origin_authority_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_origin_headers_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_origin_scheme_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_origin_text_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_policy.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_proxy_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_proxy_path_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_proxy_path_prefix_match.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_proxy_path_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_request_path_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_request_span_config.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_secure_cookie_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_set_cookie_header_value.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_status_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_trace_parent.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_trace_parent_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_trace_state.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/http_trace_state_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/identifier_file_storage_relative_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/inject_trace_context.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/io_error_presence_disposition.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/ipnet_network.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/join_diagnostic.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `server_runtime_http/src/lifecycle.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/limits.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/metrics_layer.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/metrics_response_body.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/metrics_response_body_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/metrics_shared_string.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/multipart.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/multipart_bytes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/multipart_bytes_part.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/multipart_bytes_parts.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/multipart_field_name.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/multipart_file_name.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/multipart_payload_maximum.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/multipart_request_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/multipart_text_part.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/multipart_text_parts.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/multipart_text_value.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/multipart_upload_request.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/multipart_value_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/multipart_value_length.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/normalize_identifier_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/notification.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/notification_api_token.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/notification_api_token_authorized.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/notification_api_token_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/notification_api_token_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/notification_message.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/notification_message_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/notification_request.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/notification_router.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/notification_sender.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/notification_service_state.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/opentelemetry_context.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/optional_json_body_presence.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/optional_json_content_type.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/optional_json_content_type_decision.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/origin.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/outbound_address_disposition.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/outbound_allowed_host.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/outbound_host_allowlist.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/outbound_host_allowlist_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/outbound_host_policy.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/outbound_ip_addr.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/outbound_trace_context.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/outbound_url.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/outbound_url_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/outbound_url_policy.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/outbound_url_scheme.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/outbound_url_text_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/parse_bounded_json.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/parse_bounded_json_owned.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/parse_cors_allow_origin.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/parse_int_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/parse_trusted_proxy_ranges.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/parsed_http_origin_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/parsed_ip_addr.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/path_policy.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/path_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/permit_wait_timeout_duration.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/pg_rate_limit.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/pg_rate_limit_decision.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/pg_rate_limit_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/pg_rate_limit_key_part_max_len.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/pg_rate_limit_maximum.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/pg_rate_limit_maximum_non_zero_i64.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/pg_rate_limit_query_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/pg_rate_limit_scope_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/pg_rate_limit_subject_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/pg_rate_limit_validation_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/pg_rate_limit_window_seconds.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/pg_rate_limit_window_seconds_non_zero_i32.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/proxy_path_matches_prefix.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/read_bounded_file.rs` | Risk-sensitive I/O | **BR-001 confirmed:** whole-file read occurred after a racy metadata size check, permitting allocation above the configured limit. |
| `server_runtime_http/src/read_bounded_file_async.rs` | Risk-sensitive I/O | **BR-001 confirmed:** whole-file read occurred after a racy metadata size check, permitting allocation above the configured limit. |
| `server_runtime_http/src/read_bounded_http_response.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/read_bounded_json_file_async.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/read_bounded_json_http_response.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/read_child_diagnostic.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/redact_rtsp_url_userinfo.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/redact_url_userinfo.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/redacted_url.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/redacted_url_text_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/request_id.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/request_id_layer.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/request_id_service.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/request_id_tower_layer.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/request_id_try_from_http_header_value_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/request_id_try_from_string_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/request_origin_allowed.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/request_origin_value_is_allowed.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/request_timeout.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/request_timeout_body.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/request_timeout_duration.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/request_timeout_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/request_timeout_layer.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/request_timeout_service.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/request_timeout_tower_layer.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/reqwest_client.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/reqwest_client_build_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/reqwest_client_policy.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/reqwest_connect_timeout_duration.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/reqwest_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/reqwest_outbound_url.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/reqwest_request.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/reqwest_request_builder.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/reqwest_request_timeout_duration.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/reqwest_response.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/resolve_bearer_authorization.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/resolve_client_ip.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/resolve_fallback_response_mode.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/resolve_header_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/resolve_optional_json_content_type_decision.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/resolve_outbound_address_disposition.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/resolve_request_origin_allowed.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/resolve_unique_cookie.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/resolved_client_ip_addr.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/retry_after_secs.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/retry_after_secs_non_zero_u64.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/retry_after_secs_try_from_u64_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/run_batched_cleanup.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/run_health_probe.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/run_interval_duration.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/secure_cookie.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/security_headers_layer.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/security_headers_service.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/security_headers_tower_layer.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/semaphore_permit_count_non_zero_usize.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/send_notification.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/serde_json_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/serde_json_geo_json_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/serve_io_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/serve_with_graceful_shutdown.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/serve_with_graceful_shutdown_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/service.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/service_liveness_snapshot.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/service_runtime.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/service_runtime_io_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/shared_health_readiness_arc.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/shared_http_metrics_path_cache_arc.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/spawn_interval_task.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/sqlx_pg_rate_limit_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/sqlx_pg_rate_limit_pool_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/staging_directory_name.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/std_collections_child_process_map.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/std_cookie_max_age_seconds.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/std_http_error_backtrace.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/std_http_error_chain.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/std_range_contains.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/std_request_timeout_message.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/std_request_timeout_try_from_duration_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/std_reqwest_timeout_duration_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/std_reqwest_timeout_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/std_run_interval_try_from_duration_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/storage_path_segment.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/storage_path_segment_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/storage_relative_path_buf.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/supported_geo_json_type_validation.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `server_runtime_http/src/tokio_abort_task.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/tokio_acquire_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/tokio_background_task_join.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/tokio_background_task_shutdown_sender.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/tokio_child_diagnostic_task.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/tokio_child_process.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/tokio_child_process_join_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/tokio_managed_child.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/tokio_owned_semaphore_permit.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/tokio_service_runtime.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/tokio_task_join_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/tokio_tcp_listener.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/trace_context.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/tracing_http_client_span.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/tracing_http_span_trace.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/trusted_proxy_range.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/trusted_proxy_range_parse_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/trusted_proxy_ranges.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/trusted_proxy_ranges_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/trusted_proxy_ranges_parse_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/trusted_proxy_ranges_text_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/versioned_url_safe_wire_token_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/versioned_url_safe_wire_token_text_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/wait_for_service_shutdown_signal.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `server_runtime_http/src/wire_token.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `synchronization_service_runtime`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `synchronization_service_runtime/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `synchronization_service_runtime/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `synchronization_service_runtime/src/synchronization_payload.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `synchronization_service_runtime/src/synchronization_payload_max_bytes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `synchronization_service_runtime/src/synchronization_payload_too_large.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `synchronization_service_runtime/src/synchronization_runtime_configuration.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `synchronization_service_runtime/src/synchronization_source.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `tests`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `tests/src/advanced_policy.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `tests/src/cargo_policy.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `tests/src/ci_policy.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `tests/src/code_style.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `tests/src/contract_source_policy.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/src/deployment_policy.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `tests/src/domain_analysis.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/src/domain_type_policy.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/src/domain_type_policy_fixture.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `tests/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `tests/src/lint_sync.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `tests/src/module_policy.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/src/reuse_policy.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/src/route_contract_policy.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/src/runtime_analysis.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/src/runtime_policy.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/src/secret_policy.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `tests/src/snapshot.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `tests/src/source_analysis.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/src/source_policy.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `tests/src/types.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/trybuild/route_contract_catalog_missing_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/trybuild/route_contract_delegate_non_empty.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/trybuild/route_contract_page_catalog_non_unit.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/trybuild/route_contract_struct_api_non_named.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/trybuild/route_contract_wire_enum_duplicate.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/trybuild/route_contract_wire_enum_non_unit.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/trybuild/route_contract_wrong_family_empty.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/trybuild/route_contract_wrong_family_missing_attribute.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/trybuild/route_contract_wrong_path_parameter.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `tests/trybuild/route_contract_wrong_request.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/trybuild/route_contract_wrong_response.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/trybuild/route_contract_wrong_route.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `tests/trybuild/route_contract_wrong_transport.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `text_policy`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `text_policy/src/bounded_text_policy_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `text_policy/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `text_policy/src/fixed_length_ascii_hex_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `text_policy/src/fixed_length_ascii_hex_text_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `text_policy/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `text_policy/src/non_empty_trimmed_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `text_policy/src/password_length.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `text_policy/src/password_length_range.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `text_policy/src/password_length_range_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `text_policy/src/password_policy_violation.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `text_policy/src/password_text_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `text_policy/src/required_nul_free_bounded_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `text_policy/src/url_safe_token_part_maximum_bytes.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `text_policy/src/url_safe_token_part_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `text_policy/src/url_safe_token_part_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `text_policy/src/url_safe_token_part_text_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `text_policy/src/validate_password_policy.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `text_policy/src/validate_url_safe_token_part.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `to_err_string`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `to_err_string/src/as_ref_str_to_owned.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `to_err_string/src/debug_to_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `to_err_string/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `to_err_string/src/error_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `to_err_string/src/error_text_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `to_err_string/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `to_err_string/src/static_str_to_owned.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `to_err_string/src/static_str_to_owned_input.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `to_err_string/src/to_err_string.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `to_err_string_macros`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `to_err_string_to_err_string_macros/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |

### `token_patterns`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `token_patterns/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `token_patterns/src/proc_macro2_tokens_mut.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `token_patterns_macros`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `token_patterns_token_patterns_macros/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `token_patterns_token_patterns_macros/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `token_patterns_token_patterns_macros/src/proc_macro2_generate_tp_input.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `token_patterns_token_patterns_macros/src/proc_macro2_generate_tp_output.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

### `workspace_macro_helpers`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `workspace_macro_helpers/src/closure_identifier_and_body.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/collection_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/compile_error_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_macro_helpers/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `workspace_macro_helpers/src/first_comma_stripped.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/first_ident_max_len.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/first_identifier.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/first_identifier_at.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/first_identifierifier_try_from_string_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/functions.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `workspace_macro_helpers/src/lib.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `workspace_macro_helpers/src/parse_first_identifier.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_macro_helpers/src/part_at.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/part_index.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/proc_macro2_macro_tokens.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_macro_helpers/src/proc_macro2_top_level_comma_parts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/split_fat_arrow.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/split_top_level_commas.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/std_unique_option_set_contains.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/std_unique_option_set_is_empty.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/strip_first_comma.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/syn_derive_input_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/syn_fields_named_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/syn_fields_unnamed_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/syn_struct_shape_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/top_level_comma_part.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_macro_helpers/src/unique_option_b_tree_set.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `workspace_scaffold`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `workspace_scaffold/src/cargo_args_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `workspace_scaffold/src/generated_projection.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/main.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `workspace_scaffold/src/naming_capitalized_parts.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/naming_kebab_case.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/naming_title_case.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/naming_upper_camel_case.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/naming_validate_project_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/naming_validate_repository_url.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/project_name_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/replacements_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/repository_url_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/scaffold_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/scaffold_io_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/scaffold_path_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/scaffold_run_ok.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/scaffold_service.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/scaffold_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/scaffold_text_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/server_runtime_bounded_read_error.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/service_catalog_draft.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/service_catalog_entries.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/service_catalog_entries_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/service_catalog_entry.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/service_catalog_parse.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/service_catalog_render_ci_matrix.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/service_catalog_render_release_entries.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/service_catalog_render_release_matrix.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/service_catalog_string_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/service_compose_file.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/service_compose_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/service_crate.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/service_dockerfile.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/service_image.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/service_kubernetes_manifest.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/service_port.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/service_socket_env.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/should_release.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/should_skip.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/should_write.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/synchronize_cargo_owned_projection.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/synchronize_deployment_projections.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/synchronize_generated_file.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/template_fs_copy_template_tree.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/template_fs_insert_once.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/template_fs_read_bounded_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/template_fs_rename_identity.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/template_fs_replace_file.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/template_fs_should_skip.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_scaffold/src/template_fs_write_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_scaffold/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `workspace_scaffold/src/update_env_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |

### `workspace_test_runner`

| Module | Class | Potential bug / leak / vulnerability assessment |
|---|---|---|
| `workspace_test_runner/src/admin_contract_fixture.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `workspace_test_runner/src/admin_fixture.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `workspace_test_runner/src/admin_fixture_string.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `workspace_test_runner/src/allocation_tool.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/allocation_tools.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/ansi_text_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/cargo_args.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/cargo_subcommand_available.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/check_tool_available.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/clean_ansi_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/command_duration.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/command_duration_millis.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/command_idx.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/command_run.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/command_started_at_instant.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/command_succeeded.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/command_text.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/command_texts.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/commands_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/create_admin_fixture_string.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `workspace_test_runner/src/domain_types.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `workspace_test_runner/src/execution.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/execution_io_error.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/failed_test_names.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/generate_pg_table_measure_input_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/macro_generation_measurements.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/main.rs` | Facade / entrypoint | Reviewed — no confirmed issue in this module; delegated implementations are assessed in their own rows. |
| `workspace_test_runner/src/measure_cargo_command.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/measure_memusage_command.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/measure_mode.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/measurement_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/memusage_column_idx.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/memusage_heap_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/memusage_key.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/memusage_prog_name_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/memusage_row_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/memusage_table_value.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/memusage_value_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/mode.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/print_without_measurement_footer.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/print_without_memusage_footer.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/program_args_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/program_path_ref.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/quote_token_stream_generate_pg_table_measure_input_token_stream.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/run_commands.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |
| `workspace_test_runner/src/run_counter.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/run_pg_crud_common.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/run_where_filters.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/run_workspace_tests.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/runner_mode.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/stderr_text_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/strip_ansi.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/strip_ansi_codes.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/summary_text.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/tests.rs` | Test / fixture | Reviewed — no confirmed production issue; non-production assertions or fixtures only. |
| `workspace_test_runner/src/text_ref.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/tool_available.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/tool_name.rs` | Domain / implementation | Reviewed — no confirmed bug, leak, or vulnerability. |
| `workspace_test_runner/src/tool_path.rs` | Risk-sensitive implementation | Reviewed — no confirmed issue; boundary/resource patterns were checked against bounded-input, ownership, error-propagation, and policy-test invariants. |

## Remediation status

The findings above were recorded while the affected implementation and lockfile were still unchanged. Remediation was performed only after that pre-fix report existed.

- BR-001: Resolved. Both file readers now open the file first and stream through a `maximum + 1` byte `Take` adapter. They can detect growth beyond the configured maximum without reading the remainder of the file or allocating for its full size. The synchronous and asynchronous bounded-read tests pass.
- DEP-001: Resolved. `Cargo.lock` now selects `h2 0.4.16`, the patched release. The update was completed from the local Cargo cache after crates.io was temporarily unreachable.
- DEP-002: Not reachable. `cargo tree --locked --target all -e all -i rkyv@0.7.46` returns no dependency path. `cargo audit` scans optional lockfile package records without feature reachability, so `.cargo/audit.toml` documents this exact non-reachable exception. `cargo deny`, which evaluates the enabled all-feature graph, does not encounter the advisory.
- DEP-003: Informational transitive maintenance debt. Existing `cargo deny` exceptions retain owners and review dates. The yanked `chacha20` release is introduced by pinned SQLx and has no RustSec exploit advisory; replacement remains an upstream dependency update rather than a source-module defect.
- CI-001: Resolved. The root `Cargo.lock` ignore entry was removed. The workspace lockfile is now trackable, preserves `h2 0.4.16`, and matches CI's existing `--locked` contract; nested generated lockfiles remain ignored where intended.

## Final verification

- `cargo fmt`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo test -p tests code_style`: passed, 229 tests.
- `cargo test --workspace --all-features`: passed, including generated-crate Clippy/tests and doc tests.
- `cargo audit --no-fetch`: passed with the documented unreachable `rkyv` exception; no reachable vulnerability remains in the audit result.
- `cargo deny check --disable-fetch advisories`: passed; only the already-owned maintenance/yanked warnings in DEP-003 remain.
- `git diff --check`: passed.
