# Code style tests

This checklist lists all 244 tests discovered by `cargo test -p tests_code_style -- --list`.

Tests marked with **Exceptions** contain an explicit allowlist, reviewed inventory, snapshot, or exempt owner/path. Each marked entry states what is exempted, the current inventory size when it is meaningful, where the authoritative inventory is maintained, and how stale or newly introduced entries are rejected. Ordinary test-fixture and out-of-scope filtering is not marked.

## `code_style_advanced_policy` (21)

- [ ] `allocations_inside_loops_match_reviewed_inventory` — **Exceptions:** exact source paths, allocation counts, and non-empty reasons are reviewed in `code_style_advanced_policy.rs`; any new, removed, or relocated allocation changes the inventory and fails the test.
- [ ] `arc_lock_and_trait_object_usage_matches_reviewed_inventory` — **Exceptions:** workspace owners with intentional `Arc`, lock, or trait-object use have reviewed per-owner counts and reasons in `code_style_advanced_policy.rs`; unlisted use and stale counts fail.
- [ ] `architectural_boundaries_reject_upward_dependencies`
- [ ] `contract_public_api_matches_reviewed_snapshot` — **Exceptions:** selected contract crates intentionally expose public structs, enums, traits, and functions. Their complete reviewed API is stored in `tests_code_style/snapshots/contract_public_api.snapshot`, grouped by crate with a reason for each group.
- [ ] `from_vec_implementations_are_forbidden`
- [ ] `ignored_map_err_bindings_match_reviewed_inventory` — **Exceptions:** owners that intentionally ignore a `map_err` source binding have reviewed occurrence counts and reasons in `code_style_advanced_policy.rs`; wildcard source loss remains forbidden elsewhere.
- [ ] `lock_across_await_policy_requires_explicit_drop`
- [ ] `lock_guards_are_not_held_across_await`
- [ ] `production_code_does_not_use_explicit_leak_apis`
- [ ] `raw_vec_tuple_wrapper_visitor_detects_qualified_and_nested_types`
- [ ] `raw_vec_tuple_wrappers_match_reviewed_inventory` — **Exceptions:** named tuple wrappers that still contain a raw `Vec` are listed with a reason in `code_style_advanced_policy.rs`; the inventory is exact, so additions and stale entries fail.
- [ ] `retained_spawn_tasks_are_supervised`
- [ ] `route_path_policy_rejects_api_prefix`
- [ ] `route_path_policy_rejects_kebab_case`
- [ ] `route_path_segments_use_snake_case`
- [ ] `select_policy_rejects_cancellation_sensitive_operations`
- [ ] `select_sites_match_reviewed_cancellation_inventory` — **Exceptions:** every production `select!` site is reviewed by path, expected count, and cancellation-safety reason in `code_style_advanced_policy.rs`; unreviewed sites and empty reasons fail.
- [ ] `spawn_lifecycle_policy_rejects_unconsumed_tasks`
- [ ] `struct_error_exceptions_match_reviewed_snapshot` — **Exceptions:** 40 error types remain structs instead of `thiserror` enums. Of these, 38 preserve opaque external error sources without exposing third-party types at repository domain boundaries; the other 2 preserve an opaque public PostgreSQL query-bind error with a crate-private intermediate source and a tested two-level `Error::source()` chain. The exact `path::type` inventory is in `tests_code_style/snapshots/struct_errors.snapshot`; it has been reduced by 70 entries and any further change requires review.
- [ ] `usize_max_expression_visitor_skips_test_modules`
- [ ] `usize_max_usage_matches_reviewed_inventory` — **Exceptions:** intentional production `usize::MAX` expressions are reviewed by source path, count, and reason in `code_style_advanced_policy.rs`; test modules are excluded by the underlying visitor rather than allowlisted.

## `code_style_cargo_policy` (25)

- [ ] `all_crates_have_publish_false`
- [ ] `all_crates_have_workspace_lints`
- [ ] `all_crates_inherit_shared_package_metadata`
- [ ] `all_crates_use_edition_2024`
- [ ] `check_workspace_dependencies_having_exact_version`
- [ ] `crate_names_follow_workspace_vocabulary`
- [ ] `env_and_env_example_have_same_keys`
- [ ] `external_workspace_dependencies_disable_default_features`
- [ ] `library_crates_with_public_logic_own_tests`
- [ ] `server_has_one_tracked_environment_example`
- [ ] `target_specific_dependencies_must_use_workspace_dependencies`
- [ ] `workspace_crate_src_modules_are_flat`
- [ ] `workspace_crates_are_direct_children_of_workspace_root`
- [ ] `workspace_crates_do_not_enable_default_features`
- [ ] `workspace_crates_must_use_workspace_dependencies`
- [ ] `workspace_dependencies_use_inline_table_style`
- [ ] `workspace_dependency_catalog_has_no_unused_entries`
- [ ] `workspace_dependency_default_feature_policy_rejects_missing_and_true_values`
- [ ] `workspace_lint_allow_reason_policy_rejects_missing_and_empty_comments`
- [ ] `workspace_lint_allows_have_inline_reasons`
- [ ] `workspace_members_exist_on_disk`
- [ ] `workspace_members_sorted_alphabetically`
- [ ] `workspace_normal_dependency_graph_is_acyclic`
- [ ] `workspace_packages_have_at_most_one_binary_target`
- [ ] `workspace_uses_one_async_runtime`

## `code_style_ci_policy` (5)

- [ ] `continuous_integration_contains_required_security_and_quality_commands`
- [ ] `continuous_integration_runs_specialized_test_families`
- [ ] `continuous_integration_uses_the_pinned_workspace_toolchain`
- [ ] `workflow_jobs_have_timeouts_and_marketplace_actions_use_commit_shas`
- [ ] `workflow_policy_ignores_commented_commands_and_actions`

## `code_style_contract_source_policy` (9)

- [ ] `admin_frontend_api_urls_come_from_typed_routes`
- [ ] `administrator_csr_page_behavior_comes_from_the_page_catalog`
- [ ] `administrator_data_table_queries_come_from_the_typed_spec`
- [ ] `config_reference_accessors_use_generated_forwarding`
- [ ] `generated_admin_table_consumers_use_the_shared_catalog`
- [ ] `private_shared_module_forwarding_policy_distinguishes_public_visibility_and_owner`
- [ ] `private_shared_modules_do_not_forward_crate_root_exports`
- [ ] `service_route_endpoint_composition_uses_shared_registries`
- [ ] `typed_route_registries_own_request_bodies_and_schema_catalogs`

## `code_style_deployment_policy` (6)

- [ ] `catalog_dockerfiles_pin_every_external_base_image_by_digest`
- [ ] `continuous_integration_uses_the_pinned_application_database_image`
- [ ] `dockerfile_base_image_policy_rejects_latest_and_allows_named_stages`
- [ ] `service_catalog_covers_every_build_and_runtime_projection`
- [ ] `service_catalog_matches_build_and_deployment_representations`
- [ ] `service_deployment_probes_use_registered_health_routes`

## `code_style_domain_type_policy` (29)

- [ ] `analyzer_state_raw_container_field_visitor_reports_helper_fields`
- [ ] `analyzer_state_struct_fields_use_repository_declared_wrappers`
- [ ] `bounded_string_derive_satisfies_string_wrapper_policy`
- [ ] `domain_boundaries_use_repository_declared_types`
- [ ] `domain_fixture_and_benchmark_directory_boundaries_are_exact`
- [ ] `domain_type_policy_allows_only_option_and_result_containers`
- [ ] `domain_type_policy_checks_explicit_closure_parameter_types`
- [ ] `domain_type_policy_reports_raw_browser_external_types_natively`
- [ ] `environment_initializer_is_in_domain_boundary_policy_scope`
- [ ] `external_leaf_tuple_wrappers_include_source_name`
- [ ] `external_leaf_wrapper_type_rule_has_no_name_exceptions`
- [ ] `from_string_impl_visitor_rejects_non_string_wrappers_too`
- [ ] `helper_raw_text_return_visitor_reports_free_and_inherent_helpers`
- [ ] `helper_return_types_use_repository_declared_text_wrappers`
- [ ] `manual_try_from_delegated_validator_satisfies_string_wrapper_policy`
- [ ] `newtype_try_from_explicit_error_satisfies_string_wrapper_policy`
- [ ] `newtype_try_from_validator_satisfies_string_wrapper_policy`
- [ ] `proc_macro_helpers_are_checked_while_compiler_entrypoints_are_exempt` — **Exceptions:** compiler-mandated `#[proc_macro]`, `#[proc_macro_attribute]`, and `#[proc_macro_derive]` entrypoint signatures may use raw compiler types; ordinary helper functions in the same crate remain subject to repository domain-wrapper rules.
- [ ] `server_admin_frontend_is_in_domain_boundary_policy_scope`
- [ ] `server_admin_frontend_ui_is_an_explicit_framework_adapter_boundary`
- [ ] `string_wrappers_do_not_use_from_string`
- [ ] `tuple_wrapper_deserialization_policy_rejects_direct_derive`
- [ ] `tuple_wrapper_deserialization_uses_from_or_try_from`
- [ ] `tuple_wrapper_initialization_policy_rejects_direct_constructors`
- [ ] `tuple_wrapper_rejects_from_and_try_from_for_same_inner_type`
- [ ] `tuple_wrappers_do_not_expose_inner_field`
- [ ] `tuple_wrappers_initialize_only_through_from_or_try_from`
- [ ] `workspace_scaffold_is_in_domain_boundary_policy_scope`
- [ ] `workspace_test_runner_uses_test_crate_domain_boundary`

## `code_style_lint_sync` (4)

- [ ] `check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints`
- [ ] `check_if_workspace_cargo_toml_workspace_lints_rust_contains_all_rust_lints` — **Exceptions:** 11 known unstable rustc lints are omitted from `[workspace.lints.rust]`. Each lint has a reviewed reason and is probed on the current toolchain; the test fails once an exception becomes stable or otherwise stale.
- [ ] `clippy_lint_exceptions_are_unique`
- [ ] `lint_probe_distinguishes_supported_unstable_and_unknown_lints`

## `code_style_module_policy` (25)

- [ ] `administrator_account_initialization_and_password_reset_domain_types_exclude_application_workflows`
- [ ] `common_route_domain_types_exclude_http_and_database_workflows`
- [ ] `custom_type_name_visitor_covers_all_rust_type_declarations`
- [ ] `custom_type_names_are_unique_across_workspace`
- [ ] `domain_types_do_not_add_intermediate_representation_wrappers`
- [ ] `error_types_do_not_only_wrap_other_repository_errors`
- [ ] `external_module_declarations_exist_only_in_crate_roots`
- [ ] `file_storage_domain_types_exclude_filesystem_workflows`
- [ ] `free_function_name_visitor_excludes_methods`
- [ ] `free_function_names_are_unique_across_workspace`
- [ ] `function_only_modules_contain_at_most_one_function`
- [ ] `large_module_exceptions_are_exact_and_still_needed` — **Exceptions:** only the two generated emitters `emit_generate_pg_table.rs` and `emit_generate_pg_types.rs` may exceed the 2,500-line production-module limit; this test proves both targets still exist and still exceed the threshold.
- [ ] `large_production_modules_keep_tests_in_separate_files`
- [ ] `module_declarations_do_not_use_path_attributes`
- [ ] `non_root_workspace_modules_are_not_reexport_only_facades`
- [ ] `notification_service_domain_types_exclude_application_and_adapter_workflows`
- [ ] `production_modules_contain_at_most_one_named_owner`
- [ ] `production_modules_have_bounded_responsibility` — **Exceptions:** production modules are limited to 2,500 lines except the same two generated emitters validated by `large_module_exceptions_are_exact_and_still_needed`; all other oversized modules fail.
- [ ] `production_named_modules_contain_production_items`
- [ ] `server_admin_domain_types_exclude_repository_workflows`
- [ ] `server_domain_types_exclude_application_and_adapter_workflows`
- [ ] `single_item_modules_match_their_item_name`
- [ ] `workspace_modules_reject_local_root_use_imports`
- [ ] `workspace_scaffold_domain_types_exclude_entrypoint_and_template_filesystem_workflows`
- [ ] `workspace_test_runner_domain_types_exclude_application_and_adapter_workflows`

## `code_style_reuse_policy` (4)

- [ ] `function_body_similarity_ignores_identifier_names`
- [ ] `function_body_similarity_preserves_behavioral_structure`
- [ ] `short_mechanical_adapters_are_not_substantial`
- [ ] `substantial_function_bodies_have_one_source_of_truth` — **Exceptions:** 34 exact duplicate groups remain: 13 individually named fixture groups in `code_style_reuse_policy.rs`, 11 split-owner groups in `CODE_STYLE_SPLIT_OWNER_DUPLICATE_GROUPS`, and 10 independently reviewed groups in `CODE_STYLE_REVIEWED_DUPLICATE_GROUPS_2026`. Each group records exact function locations and a non-empty extraction rationale; renamed/split module paths are canonicalized, and both new duplicates and stale exceptions fail. The reviewed-group inventory has been reduced from 16 to 10.

## `code_style_route_contract_policy` (1)

- [ ] `route_contract_policy`

## `code_style_runtime_policy` (11)

- [ ] `async_blocking_policy_rejects_sync_filesystem_network_and_executor_calls`
- [ ] `async_functions_do_not_make_blocking_executor_calls`
- [ ] `environment_initializer_is_in_runtime_policy_scope`
- [ ] `external_service_policy_rejects_http_database_and_socket_clients`
- [ ] `external_service_policy_requires_a_reason_for_ignored_integration_tests`
- [ ] `runtime_arc_usage_is_limited_to_cross_thread_state`
- [ ] `runtime_code_does_not_use_expect_unwrap_or_panic`
- [ ] `runtime_code_does_not_use_mutex`
- [ ] `runtime_test_crate_detection_uses_test_name_segments`
- [ ] `runtime_test_module_exclusion_uses_test_filename`
- [ ] `unit_tests_do_not_create_external_service_clients`

## `code_style_secret_policy` (4)

- [ ] `repository_secret_box_policy_checks_generated_tokens`
- [ ] `repository_secret_box_policy_rejects_raw_string_generic_argument`
- [ ] `repository_secret_boxes_use_bounded_string_types`
- [ ] `secret_boxes_do_not_use_raw_string_anywhere_in_repository`

## `code_style_snapshot` (3)

- [ ] `invalid_project_source_content_fails_snapshot_loading`
- [ ] `missing_project_source_file_fails_snapshot_loading`
- [ ] `walk_error_fails_snapshot_loading`

## `code_style_source_policy` (97)

- [ ] `abort_and_transmute_calls_match_reviewed_baseline`
- [ ] `admin_route_errors_do_not_wrap_a_shared_operation_error`
- [ ] `all_files_are_english_only`
- [ ] `all_string_constants_are_declared_in_str_constants` — **Exceptions:** declarations inside the dedicated `constants_str` crate are the sole owner exemption; generated macro token streams are inspected too, while ordinary runtime string literals are not constant declarations.
- [ ] `api_response_error_source_policy_rejects_raw_sources`
- [ ] `api_response_error_sources_use_observed_error`
- [ ] `api_response_errors_keep_source_locations_out_of_public_error_enums`
- [ ] `api_response_location_policy_rejects_location_fields`
- [ ] `bounded_read_policy_has_no_whole_file_owner_exceptions`
- [ ] `cfg_test_modules_do_not_hide_forbidden_public_reexports`
- [ ] `check_rs_files_contains_only_unique_uuid_v4`
- [ ] `commented_debug_statement_policy_rejects_debug_macros_only`
- [ ] `constant_display_implementations_derive_display_const`
- [ ] `declared_child_does_not_bypass_non_public_use_import_policy`
- [ ] `diagnostic_id_visitor_checks_expect_methods_and_panic_macros`
- [ ] `diagnostic_id_visitor_checks_generated_expect_and_panic_tokens`
- [ ] `direct_environment_and_filesystem_access_stays_at_owned_boundaries` — **Exceptions:** direct `std::env`, `std::fs`, and `tokio::fs` calls are limited to test code, the bounded-read adapter, and 16 reviewed filesystem-owner modules/split modules; production callers outside those boundaries fail.
- [ ] `direct_filesystem_owner_inventory_is_exact_justified_and_current` — **Exceptions:** validates all 16 filesystem-owner suffixes and their reasons, proves every target exists and still performs direct access, and rejects uncovered direct-access owners or stale entries. The inventory is maintained in `constants_str::CODE_STYLE_DIRECT_FS_OWNER_*`.
- [ ] `direct_process_command_creation_stays_in_shared_tooling`
- [ ] `domain_owned_string_catalogs_do_not_return_to_str_constants`
- [ ] `empty_enum_policy_checks_items_and_attribute_payloads`
- [ ] `environment_initializer_is_in_bounded_read_policy_scope`
- [ ] `error_formatters_do_not_expose_sensitive_fields`
- [ ] `error_implementation_source_uses_only_thiserror_derive`
- [ ] `error_implementations_derive_thiserror_error`
- [ ] `every_fallible_typed_route_operation_has_its_own_error_type`
- [ ] `every_workspace_struct_and_enum_derives_optimal_memory_layout` — **Exceptions:** `optimal_memory_layout/src/lib.rs` is exempt because it defines the derive itself; every struct and enum elsewhere, including generated syntax inspected by the visitor, must derive it.
- [ ] `expect_and_panic_messages_start_with_unique_diagnostic_ids` — **Exceptions:** three reviewed generated-message interpolations cannot begin with a literal diagnostic ID at the source site. They are matched by exact owner/error text with reasons; all literal IDs must still be unique UUID-v4 prefixes.
- [ ] `field_getters_are_generated`
- [ ] `generated_randomness_policy_inspects_quote_token_streams`
- [ ] `generated_source_templates_do_not_embed_random_test_values`
- [ ] `infallible_functions_return_concrete_types`
- [ ] `infallible_result_policy_rejects_wrappers_and_free_function_results`
- [ ] `json_api_error_response_policy_rejects_structs_and_accepts_thiserror_enums`
- [ ] `json_api_error_responses_originate_from_thiserror_enums`
- [ ] `library_sources_do_not_use_print_macros`
- [ ] `map_err_does_not_discard_source_with_wildcard`
- [ ] `module_and_function_names_do_not_use_unclear_short_forms`
- [ ] `module_and_function_names_use_single_underscores`
- [ ] `new_runtime_structs_keep_fields_private` — **Exceptions:** 455 legacy public fields remain: 452 fields tracked as exact counts across 14 owners in `CODE_STYLE_REVIEWED_PUBLIC_FIELD_OWNERS`, plus one exact three-field struct entry in `CODE_STYLE_REVIEWED_PUBLIC_FIELD_*`. Exact entries carry path, struct, field names, and reasons. New public fields, missing exact fields, empty reasons, and owner-count drift fail; test-crate sources are out of scope.
- [ ] `no_dbg_macro_in_source_code`
- [ ] `no_duplicated_string_literals_in_non_policy_test_code`
- [ ] `no_empty_enums_in_rust_sources`
- [ ] `no_for_loops_in_source_code`
- [ ] `no_include_asset_macros_outside_allowlist`
- [ ] `no_macro_rules_in_source_code`
- [ ] `no_simple_constant_aliases_in_rust_sources`
- [ ] `no_todo_or_unimplemented_macro_in_source_code`
- [ ] `no_type_aliases_in_rust_sources`
- [ ] `no_unwrap_in_source_code`
- [ ] `numeric_conversions_do_not_use_as_casts`
- [ ] `optimal_memory_layout_derive_visitor_checks_structs_and_enums`
- [ ] `ordinary_test_fixture_is_in_duplicate_string_policy_scope`
- [ ] `process_static_state_matches_reviewed_inventory` — **Exceptions:** eight intentional process-wide statics are reviewed by exact identifier, path suffix, and reason in `code_style_source_policy.rs` (primarily caches/registries); any other static state fails.
- [ ] `production_code_does_not_use_line_print_macros`
- [ ] `production_line_print_macro_policy_allows_test_code_and_rejects_production_code`
- [ ] `production_pg_error_classification_is_centralized`
- [ ] `production_string_literals_are_reused`
- [ ] `project_text_files_have_stable_line_endings_and_no_trailing_whitespace`
- [ ] `provider_traits_do_not_use_get_prefix`
- [ ] `public_reexports_are_forbidden_and_private_imports_are_restricted`
- [ ] `raw_runtime_sql_identifier_inventory_matches_reviewed_baseline`
- [ ] `repository_identifiers_use_explicit_resource_names`
- [ ] `route_operation_error_policy_rejects_shared_types`
- [ ] `runtime_data_reads_are_bounded`
- [ ] `runtime_struct_fields_do_not_expose_untyped_json_values`
- [ ] `sensitive_error_format_policy_rejects_named_and_tuple_placeholders`
- [ ] `sensitive_text_debug_policy_distinguishes_redacted_derives`
- [ ] `sensitive_text_wrappers_do_not_derive_unredacted_debug_or_display`
- [ ] `server_admin_string_constants_reuse_macro_fragments`
- [ ] `single_call_fn_is_never_allowed_for_a_whole_module`
- [ ] `source_does_not_retain_commented_debug_statements`
- [ ] `source_lint_reason_policy_accepts_argument_and_comment_reasons`
- [ ] `source_lint_suppressions_have_explicit_reasons`
- [ ] `spawned_task_policy_rejects_bare_wildcard_and_ignored_bindings`
- [ ] `spawned_tasks_must_retain_an_owner`
- [ ] `str_constants_does_not_own_typed_domain_values`
- [ ] `string_constant_declaration_policy_ignores_runtime_literals_and_rejects_all_const_forms`
- [ ] `string_constant_declaration_policy_rejects_aliases_to_exported_constants`
- [ ] `string_constant_policy_has_only_the_constants_crate_source_directory_exception` — **Exceptions:** verifies that the string-constant exemption recognizes only Rust sources under the dedicated `constants_str/src` directory and rejects lookalike paths, sibling crates, and files outside that source tree.
- [ ] `string_constant_visitor_checks_test_code_and_allows_reviewed_syntax_boundaries`
- [ ] `string_constant_visitor_detects_expression_and_nested_macro_literals`
- [ ] `struct_field_visibility_policy_rejects_restricted_visibility`
- [ ] `struct_fields_do_not_use_restricted_visibility` — **Exceptions:** 348 restricted-visible fields remain across 10 reviewed workspace owners. Exact per-owner counts of `pub(crate)`, `pub(super)`, and `pub(in path)` fields are stored in `CODE_STYLE_REVIEWED_RESTRICTED_VISIBLE_FIELD_OWNERS`; all 145 previously reviewed `server_runtime_http` fields, all 42 previously reviewed `server_runtime_core` fields, all 21 previously reviewed `common_routes` fields, all 18 previously reviewed `macro_helpers` fields, all 18 previously reviewed `route_validators` fields, all 14 previously reviewed `workspace_test_runner` fields, 12 `pg_crud_pg_table` fields, 8 `server_admin_contract` fields, 8 `workspace_scaffold` fields, 6 `pg_crud_pg_types_generate_src` fields, and 4 `pg_crud_pg_table_generate_src` fields have been privatized. Unlisted owners, newly added fields, and stale count entries fail.
- [ ] `text_content_hygiene_policy_rejects_all_line_ending_violations`
- [ ] `tuple_newtypes_derive_borrow_instead_of_implementing_forwarding_borrow` — **Exceptions:** the foundational `newtype` implementation that powers `Borrow` generation may implement the forwarding trait manually; workspace consumers must use the derive.
- [ ] `tuple_newtypes_derive_deref_inner_instead_of_implementing_forwarding_deref` — **Exceptions:** the foundational `newtype` implementation that powers `DerefInner` generation may implement forwarding manually; workspace consumers must use the derive.
- [ ] `tuple_newtypes_derive_display_instead_of_implementing_forwarding_display` — **Exceptions:** foundational implementations used to provide `Display` generation are exempt where the derive cannot implement itself; consumer tuple newtypes must derive instead of forwarding manually.
- [ ] `tuple_newtypes_derive_from_inner_instead_of_implementing_passthrough_from` — **Exceptions:** foundational implementations used to provide `FromInner` generation are exempt where the derive cannot implement itself; consumer tuple newtypes must use the derive.
- [ ] `tuple_newtypes_derive_into_inner_from_instead_of_implementing_passthrough_from` — **Exceptions:** foundational implementations used to provide `IntoInner` generation are exempt where the derive cannot implement itself; consumer tuple newtypes must use the derive.
- [ ] `tuple_newtypes_derive_into_iterator_instead_of_forwarding_into_iter` — **Exceptions:** the foundational `newtype` implementation that powers `IntoIterator` generation may forward manually; workspace consumers must use the derive.
- [ ] `tuple_newtypes_derive_not_inner_instead_of_implementing_not` — **Exceptions:** foundational `newtype` and macro-helper sources that implement `NotInner` generation are exempt because the derive cannot bootstrap itself; workspace consumers must use the derive.
- [ ] `typed_route_operation_error_policy_rejects_shared_types`
- [ ] `unit_test_nondeterminism_visitor_rejects_sync_async_time_and_randomness`
- [ ] `unit_tests_use_deterministic_time_and_randomness_patterns` — **Exceptions:** one reviewed `Instant::now` call in `frontend_contract/src/auth_session_keep_alive.rs` is allowed with an exact reason; all other unit-test wall-clock, sleep, and randomness calls—including generated tokens—fail.
- [ ] `use_import_policy_detects_private_imports_and_public_reexports`
- [ ] `workspace_scaffold_is_in_bounded_read_policy_scope`
