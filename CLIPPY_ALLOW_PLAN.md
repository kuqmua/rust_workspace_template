# Clippy Allow Reduction Plan

## Objective

Minimize Clippy allow annotations without changing runtime behavior, weakening repository architecture, broadening public APIs, or replacing narrow suppressions with broader module- or crate-level suppressions.

## Baseline

- Inventory date: 2026-07-29
- Rust source annotations containing at least one clippy:: lint: **499**
- Scope: every tracked/worktree *.rs file except target/**, including generated-token templates.
- Counting rule: one row per complete allow annotation; multiline annotations count once.

## Reduction workflow

1. Remove annotations that compiler probes prove are stale.
2. Remove single_call_fn suppressions from externally reachable functions where the lint cannot apply.
3. Inline genuinely single-use trivial wrappers only when behavior, diagnostic location, cancellation behavior, and public API remain unchanged.
4. Replace mixed Rust/Clippy generated-code suppressions with a precise Rust expect when the Rust lint is the actual requirement.
5. Keep narrow suppressions at derive, proc-macro, framework-registration, and policy-conflict boundaries when removal would change semantics or force a broader suppression.
6. After edits, regenerate the current-state inventory so paths and line numbers remain authoritative.

## Required verification

    cargo fmt
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test -p tests code_style

## Complete baseline inventory

| # | Path | Line | Annotation | Audit |
|---:|---|---:|---|---|
| 1 | common_routes/src/lib.rs | 1 | #![allow( clippy::arbitrary_source_item_ordering, clippy::needless_for_each, reason = "generated route registries stay adjacent to their handlers and utoipa expands to an internal for_each" )] | Pending |
| 2 | common_routes/src/lib.rs | 79 | #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call | Pending |
| 3 | common_routes/src/lib.rs | 393 | #[allow(clippy::single_call_fn)] // keeps commit-link extraction shape shared between handlers and tests | Pending |
| 4 | common_routes/src/lib.rs | 397 | #[allow(clippy::single_call_fn)] // single source for no-route text reused by payload builder and tests | Pending |
| 5 | common_routes/src/lib.rs | 401 | #[allow(clippy::single_call_fn)] // isolated for reuse in tests and payload builder when suffix is precomputed | Pending |
| 6 | common_routes/src/lib.rs | 409 | #[allow(clippy::single_call_fn)] // isolated for reuse in tests and message builder | Pending |
| 7 | common_routes/src/lib.rs | 417 | #[allow(clippy::single_call_fn)] // keeps route text construction consistent for path-only and path+query URIs | Pending |
| 8 | common_routes/src/lib.rs | 425 | #[allow(clippy::single_call_fn)] // keeps fallback payload assembly in one place | Pending |
| 9 | common_routes/src/lib.rs | 432 | #[allow(clippy::single_call_fn)] // shared payload constructor keeps not-found response shape centralized | Pending |
| 10 | common_routes/src/lib.rs | 445 | #[allow(clippy::single_call_fn)] // shared helper keeps commit-based status+json responses consistent across handlers | Pending |
| 11 | common_routes/src/lib.rs | 462 | #[allow(clippy::single_call_fn)] // shared mapping keeps health-check status behavior centralized | Pending |
| 12 | common_routes/src/lib.rs | 512 | #[allow( clippy::single_call_fn, reason = "the concrete handler is intentionally shared by Axum and OpenAPI metadata" )] | Pending |
| 13 | common_routes/src/lib.rs | 520 | #[allow( clippy::single_call_fn, reason = "the concrete handler is intentionally owned by the generated route registry" )] | Pending |
| 14 | common_routes/src/lib.rs | 533 | #[allow( clippy::single_call_fn, reason = "the concrete handler is intentionally owned by the generated route registry" )] | Pending |
| 15 | common_routes/src/lib.rs | 546 | #[allow( clippy::single_call_fn, reason = "the concrete handler is intentionally owned by the generated route registry" )] | Pending |
| 16 | common_routes/src/lib.rs | 561 | #[allow( clippy::single_call_fn, reason = "the concrete handler is intentionally owned by the generated route registry" )] | Pending |
| 17 | common_routes/src/lib.rs | 586 | #[allow(clippy::arbitrary_source_item_ordering)] // fixtures remain adjacent to the tests that exercise their route state | Pending |
| 18 | config_lib/src/lib.rs | 1 | #![allow(clippy::arbitrary_source_item_ordering)] // configuration declarations stay grouped with their parse errors and TryFromStdEnvVarOk implementations | Pending |
| 19 | config_lib/src/lib.rs | 874 | #[allow(clippy::single_call_fn)] // extracted timezone conversion keeps conversion + message mapping reusable and directly testable | Pending |
| 20 | config_lib/src/types.rs | 50 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 21 | config_lib/src/types.rs | 137 | #[allow(clippy::single_call_fn)] // helper keeps env-read error context centralized and deterministic for tests | Pending |
| 22 | config_lib/src/types.rs | 146 | #[allow(clippy::single_call_fn)] // helper centralizes env var context mapping for string parsers and is reused by enum parsing | Pending |
| 23 | config_lib/src/types.rs | 159 | #[allow(clippy::single_call_fn)] // helper centralizes std::str::FromStr context formatting and keeps per-type parsing helpers minimal | Pending |
| 24 | config_lib/src/types.rs | 174 | #[allow(clippy::single_call_fn)] // helper composes env var read + std::str::FromStr context mapping for reuse across enum env parsers | Pending |
| 25 | development_data_bootstrap/src/lib.rs | 13 | #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call | Pending |
| 26 | file_storage/src/lib.rs | 205 | #[allow(clippy::arbitrary_source_item_ordering)] // transactional API is grouped as prepare, stage, commit, and rollback operations | Pending |
| 27 | file_storage/src/lib.rs | 483 | #[allow(clippy::while_let_on_iterator)] | Pending |
| 28 | frontend_contract/src/lib.rs | 1 | #![allow(clippy::arbitrary_source_item_ordering)] // contract implementations keep constructors before accessors and fluent modifiers | Pending |
| 29 | frontend_contract/src/lib.rs | 952 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 30 | frontend_contract/src/lib.rs | 1104 | #[allow(clippy::needless_for_each)] // iterator form follows the workspace ban on explicit for loops | Pending |
| 31 | frontend_contract/src/problem.rs | 78 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 32 | frontend_contract/src/problem.rs | 201 | #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call | Pending |
| 33 | frontend_contract/src/problem.rs | 329 | #[allow(clippy::needless_for_each)] // workspace source policy requires iterator methods | Pending |
| 34 | frontend_contract/src/route.rs | 467 | #[allow(clippy::needless_for_each)] // iterator form follows the workspace no-for-loop policy | Pending |
| 35 | frontend_contract_macros/src/lib.rs | 1 | #![allow( clippy::arbitrary_source_item_ordering, reason = "proc-macro parser models precede their entrypoints while related derive parsers remain adjacent" )] | Pending |
| 36 | frontend_contract_macros/src/lib.rs | 517 | #[allow(clippy::needless_for_each)] | Pending |
| 37 | frontend_contract_macros/src/lib.rs | 717 | #[allow( clippy::wildcard_enum_match_arm, reason = "typed route methods intentionally reject every current and future non-path expression" )] | Pending |
| 38 | frontend_contract_macros/src/lib.rs | 1318 | #[allow(clippy::unused_async, reason = #unused_async_reason)] | Pending |
| 39 | frontend_contract_macros/src/lib.rs | 1806 | #[allow(clippy::needless_for_each)] // iterator form follows the workspace no-for-loop policy | Pending |
| 40 | frontend_contract_validation/src/json_snapshot.rs | 7 | #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call | Pending |
| 41 | frontend_contract_validation/src/openapi_validation.rs | 73 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 42 | frontend_contract_validation/src/route_contract_validation.rs | 27 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 43 | generate_quotes/src/lib.rs | 37 | #[allow(clippy::single_call_fn)] // shared with prefix-aware token quote wrapper to keep parse+panic-id flow in one place | Pending |
| 44 | git_info/src/lib.rs | 33 | #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call | Pending |
| 45 | git_info/src/lib.rs | 106 | #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call | Pending |
| 46 | git_info/src/lib.rs | 378 | #[allow(clippy::single_call_fn)] // shared writer keeps link assembly consistent across builders and tests | Pending |
| 47 | initialize_environment_files/src/main.rs | 26 | #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call | Pending |
| 48 | initialize_environment_files/src/main.rs | 48 | #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call | Pending |
| 49 | initialize_environment_files/src/main.rs | 67 | #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call | Pending |
| 50 | initialize_environment_files/src/main.rs | 142 | #[allow( clippy::single_call_fn, reason = "keeps lexical path validation independently testable and reviewable" )] | Pending |
| 51 | initialize_environment_files/src/main.rs | 172 | #[allow( clippy::needless_for_each, clippy::single_call_fn, reason = "isolates the testable merge algorithm and repository policy forbids for loops" )] | Pending |
| 52 | initialize_environment_files/src/main.rs | 206 | #[allow( clippy::single_call_fn, reason = "separates manifest validation from filesystem mutation" )] | Pending |
| 53 | initialize_environment_files/src/main.rs | 241 | #[allow( clippy::single_call_fn, reason = "provides one testable dry-run and apply entry point" )] | Pending |
| 54 | location_lib/location/src/lib.rs | 25 | #[allow(clippy::single_call_fn)] // isolated transformation is unit-tested independently from proc-macro parsing | Pending |
| 55 | location_lib/location/src/lib.rs | 160 | #[allow(clippy::redundant_closure_for_method_calls)] | Pending |
| 56 | location_lib/location_test/src/main.rs | 110 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 57 | location_lib/src/location.rs | 1 | #![allow(clippy::module_name_repetitions)] | Pending |
| 58 | location_lib/src/location.rs | 52 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 59 | location_lib/src/location.rs | 91 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 60 | location_lib/src/location.rs | 173 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 61 | location_lib/src/location.rs | 190 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 62 | location_lib/src/location.rs | 203 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 63 | location_lib/src/location.rs | 211 | #[allow(clippy::arbitrary_source_item_ordering, clippy::needless_pass_by_value)] | Pending |
| 64 | location_lib/src/location.rs | 213 | #[allow(clippy::single_call_fn)] // shared offset accessor is reused by formatter and tests | Pending |
| 65 | location_lib/src/location.rs | 241 | #[allow(clippy::single_call_fn)] // centralizes datetime + timezone composition so formatting can stay branch-light and tests can target conversion separately | Pending |
| 66 | location_lib/src/location.rs | 333 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 67 | location_lib/src/location.rs | 352 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 68 | location_lib/src/location.rs | 375 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 69 | macros_helpers/generate_derive_token_stream_builder/src/lib.rs | 7 | #[allow(clippy::single_call_fn)] // extracted to isolate case-normalization logic and keep macro expansion flow focused | Pending |
| 70 | macros_helpers/src/generate_field_location_new_token_stream.rs | 20 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 71 | macros_helpers/src/generate_field_location_new_token_stream.rs | 45 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 72 | macros_helpers/src/get_macro_attr.rs | 14 | #[allow(clippy::single_call_fn)] // helper keeps segment comparison logic isolated and reusable for future attr queries | Pending |
| 73 | macros_helpers/src/location.rs | 1 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 74 | macros_helpers/src/rs_file_path.rs | 3 | #[allow(clippy::single_call_fn)] // centralized .rs extension mapping keeps path behavior consistent across file-write helpers | Pending |
| 75 | macros_helpers/src/status_code.rs | 1 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 76 | macros_helpers/src/write_string_into_file.rs | 50 | #[allow(clippy::single_call_fn)] // write-decision logic is split out to keep file write path minimal and focused | Pending |
| 77 | macros_helpers/src/write_string_into_file.rs | 111 | #[allow(clippy::single_call_fn)] // extracted side-effect helper keeps write/no-write branching reusable and test-focused | Pending |
| 78 | macros_helpers/src/write_string_into_file.rs | 122 | #[allow(clippy::single_call_fn)] // preserves write/no-write state so callers can skip extra work (e.g. formatting) on unchanged files | Pending |
| 79 | macros_helpers/src/write_token_stream_into_file.rs | 17 | #[allow(clippy::single_call_fn)] // rustfmt execution is isolated so io/process errors stay localized and easy to test | Pending |
| 80 | macros_helpers/src/write_token_stream_into_file.rs | 35 | #[allow(clippy::single_call_fn)] // keeps ShouldWriteTokenStreamIntoFile flag interpretation centralized | Pending |
| 81 | macros_helpers/src/write_token_stream_into_file.rs | 39 | #[allow(clippy::single_call_fn)] // centralizes token-to-file write mapping and outcome extraction | Pending |
| 82 | newtype/src/lib.rs | 309 | #[allow(clippy::single_call_fn)] // keeps TryFrom attribute parsing separate from its proc-macro entry point | Pending |
| 83 | newtype/src/lib.rs | 1361 | #[allow(clippy::single_call_fn)] // checked String wrapper generation is separate from forwarding newtype impls | Pending |
| 84 | newtype/src/lib.rs | 1537 | #[allow(clippy::single_call_fn)] // keeps enum parsing derive independent from newtype tuple-struct generation | Pending |
| 85 | newtype/src/lib.rs | 1591 | #[allow(clippy::single_call_fn)] // required checked-string options are parsed together for focused diagnostics | Pending |
| 86 | newtype/src/lib.rs | 1685 | #[allow(clippy::single_call_fn)] // string wrapper policy belongs to newtype validation before From impl generation | Pending |
| 87 | newtype/src/lib.rs | 1722 | #[allow(clippy::single_call_fn)] // tuple field extraction is separate to keep derive input validation explicit | Pending |
| 88 | newtype/src/lib.rs | 1753 | #[allow(clippy::single_call_fn)] // newtype validation only needs terminal path identifier matching for concrete String wrappers | Pending |
| 89 | newtype/src/lib.rs | 1764 | #[allow(clippy::single_call_fn)] // proc-macro generated getter names need local snake_case conversion without adding another dependency | Pending |
| 90 | newtype/src/lib.rs | 1784 | #[allow(clippy::single_call_fn)] // ToErrString code generation has distinct modes from base newtype impls | Pending |
| 91 | newtype/tests/newtype.rs | 174 | #[allow(clippy::single_call_fn)] // validates named-function support in the Newtype derive | Pending |
| 92 | notification_service/src/main.rs | 1 | #![allow(clippy::single_call_fn)] // binary composition functions intentionally have one startup or route registration owner | Pending |
| 93 | notification_service/src/main.rs | 2 | #![allow(clippy::arbitrary_source_item_ordering)] // OpenAPI document stays next to its generated schema and operation marker | Pending |
| 94 | notification_service/src/main.rs | 3 | #![allow(clippy::needless_for_each)] // utoipa OpenApi derive expands to an internal for_each | Pending |
| 95 | notification_service_config/src/lib.rs | 1 | #[allow( clippy::arbitrary_source_item_ordering, reason = "fields are ordered by decreasing alignment as enforced by optml" )] | Pending |
| 96 | notification_service_config/tests/config_descriptor.rs | 13 | #[allow(clippy::single_call_fn)] // isolates descriptor-derived port resolution from deployment assertions | Pending |
| 97 | notification_service_config/tests/config_descriptor.rs | 37 | #[allow(clippy::single_call_fn)] // derives deployment identity from the config crate instead of repeating it | Pending |
| 98 | notification_service_config/tests/config_descriptor.rs | 49 | #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy | Pending |
| 99 | notification_service_config/tests/config_descriptor.rs | 120 | #[allow(clippy::needless_for_each)] // workspace source policy forbids for loops | Pending |
| 100 | panic_location/src/lib.rs | 10 | #[allow(clippy::single_call_fn)] // keeps panic message construction reusable and testable in one place | Pending |
| 101 | pg_crud/pg_crud_common/benches/query_builders.rs | 8 | #[allow( clippy::needless_for_each, clippy::single_call_fn, reason = "Criterion requires a named benchmark function, and repository policy requires iterator methods instead of for loops" )] | Pending |
| 102 | pg_crud/pg_crud_common/benches/query_builders.rs | 39 | #[allow( clippy::single_call_fn, reason = "Criterion requires a named benchmark function consumed by its registration macro" )] | Pending |
| 103 | pg_crud/pg_crud_common/benches/query_builders.rs | 58 | #[allow( clippy::single_call_fn, reason = "Criterion requires a named benchmark function consumed by its registration macro" )] | Pending |
| 104 | pg_crud/pg_crud_common/src/bounded_vec.rs | 4 | #[allow(clippy::module_name_repetitions)] // the public name remains explicit when imported outside this module | Pending |
| 105 | pg_crud/pg_crud_common/src/bounded_vec.rs | 13 | #[allow(clippy::module_name_repetitions)] // callers need an unambiguous error name in public signatures | Pending |
| 106 | pg_crud/pg_crud_common/src/cursor.rs | 133 | #[allow(clippy::arbitrary_source_item_ordering)] // constructor remains before operational methods | Pending |
| 107 | pg_crud/pg_crud_common/src/db_schema_conformance.rs | 995 | #[allow(clippy::needless_for_each)] // repository policy requires iterator traversal in source tests | Pending |
| 108 | pg_crud/pg_crud_common/src/lib.rs | 278 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 109 | pg_crud/pg_crud_common/src/lib.rs | 332 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 110 | pg_crud/pg_crud_common/src/lib.rs | 347 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 111 | pg_crud/pg_crud_common/src/lib.rs | 352 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 112 | pg_crud/pg_crud_common/src/lib.rs | 428 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 113 | pg_crud/pg_crud_common/src/lib.rs | 435 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 114 | pg_crud/pg_crud_common/src/lib.rs | 593 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 115 | pg_crud/pg_crud_common/src/lib.rs | 643 | #[allow(clippy::absolute_paths)] | Pending |
| 116 | pg_crud/pg_crud_common/src/lib.rs | 644 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 117 | pg_crud/pg_crud_common/src/lib.rs | 1361 | #[allow(clippy::absolute_paths)] | Pending |
| 118 | pg_crud/pg_crud_common/src/lib.rs | 1362 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 119 | pg_crud/pg_crud_common/src/query_fragment.rs | 12 | #[allow( clippy::field_scoped_visibility_modifiers, reason = "the private parent module assembles query fragments without widening public API" )] | Pending |
| 120 | pg_crud/pg_crud_common/src/sql_identifier.rs | 5 | #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call | Pending |
| 121 | pg_crud/pg_crud_common/src/sql_identifier.rs | 161 | #[allow( clippy::needless_for_each, reason = "repository source policy requires iterator methods instead of for loops" )] | Pending |
| 122 | pg_crud/pg_crud_common/src/sql_like_pattern.rs | 25 | #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call | Pending |
| 123 | pg_crud/pg_crud_macros_common/src/filters.rs | 1 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 124 | pg_crud/pg_crud_macros_common/src/lib.rs | 457 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 125 | pg_crud/pg_crud_macros_common/src/lib.rs | 465 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 126 | pg_crud/pg_crud_macros_common/src/lib.rs | 1263 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 127 | pg_crud/pg_crud_macros_common/src/lib.rs | 1268 | #[allow(clippy::absolute_paths)] | Pending |
| 128 | pg_crud/pg_crud_macros_common/src/lib.rs | 1312 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 129 | pg_crud/pg_crud_macros_common/src/lib.rs | 1317 | #[allow(clippy::absolute_paths)] | Pending |
| 130 | pg_crud/pg_crud_macros_common/src/pg_type_test_cases.rs | 527 | #[allow(clippy::absolute_paths)] | Pending |
| 131 | pg_crud/pg_crud_macros_common/src/pg_type_test_cases.rs | 530 | #[allow(clippy::float_arithmetic)] | Pending |
| 132 | pg_crud/pg_crud_macros_common/src/token_stream_helpers.rs | 16 | #[allow(clippy::absolute_paths)] | Pending |
| 133 | pg_crud/pg_crud_macros_common/src/token_stream_helpers.rs | 17 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 134 | pg_crud/pg_table/generate_pg_table_src/src/client.rs | 1 | #![allow( clippy::single_call_fn, reason = "client transport emission has a private physical boundary from route metadata" )] | Pending |
| 135 | pg_crud/pg_table/generate_pg_table_src/src/contract_tests.rs | 1 | #![allow( clippy::single_call_fn, reason = "generated contract-test emission has a private physical boundary from route metadata" )] | Pending |
| 136 | pg_crud/pg_table/generate_pg_table_src/src/frontend.rs | 1 | #![allow( clippy::single_call_fn, reason = "frontend contract emission has a private physical boundary from route metadata" )] | Pending |
| 137 | pg_crud/pg_table/generate_pg_table_src/src/model.rs | 1 | #![allow(clippy::field_scoped_visibility_modifiers)] // sibling emitters read the private descriptor directly while it remains hidden outside the generator | Pending |
| 138 | pg_crud/pg_table/generate_pg_table_src/src/model.rs | 19 | #[allow(clippy::single_call_fn)] // construction is isolated as the typed build-stage boundary | Pending |
| 139 | pg_crud/pg_table/generate_pg_table_src/src/openapi.rs | 1 | #![allow( clippy::single_call_fn, reason = "OpenAPI emission has a private physical boundary from route metadata" )] | Pending |
| 140 | pg_crud/pg_table/generate_pg_table_src/src/parse.rs | 1 | #![allow( clippy::single_call_fn, reason = "table parsing has a private physical boundary from descriptor and token emitters" )] | Pending |
| 141 | pg_crud/pg_table/generate_pg_table_src/src/route.rs | 1 | #![allow( clippy::single_call_fn, reason = "route projections are private physical boundaries shared by transport emitters" )] | Pending |
| 142 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 73 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 143 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 105 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 144 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 293 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 145 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 365 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 146 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 374 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 147 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 427 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 148 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 433 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 149 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 439 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 150 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 704 | #[allow(clippy::single_call_fn)] | Pending |
| 151 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 832 | #[allow(clippy::single_call_fn)] | Pending |
| 152 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 909 | #[allow(clippy::single_call_fn)] | Pending |
| 153 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 937 | #[allow(clippy::single_call_fn)] | Pending |
| 154 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 1026 | #[allow(clippy::single_call_fn)] | Pending |
| 155 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 1173 | #[allow(clippy::single_call_fn)] | Pending |
| 156 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 1205 | #[allow(clippy::single_call_fn)] | Pending |
| 157 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 1228 | #[allow(clippy::single_call_fn)] | Pending |
| 158 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 2436 | #[allow(clippy::items_after_statements)] | Pending |
| 159 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 3619 | &quote::quote! {#[allow(clippy::redundant_pattern_matching)]}, //todo check if 1 then different logic | Pending |
| 160 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 5349 | #[allow(clippy::single_call_fn)] | Pending |
| 161 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 6355 | #[allow(clippy::single_call_fn)] | Pending |
| 162 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 6617 | #[allow(clippy::absolute_paths)] | Pending |
| 163 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 6922 | #[allow(clippy::future_not_send)] // browser transports and WASM futures are intentionally single-threaded | Pending |
| 164 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 7298 | #[allow(clippy::needless_for_each)] // generated schema registration uses iterator callbacks internally | Pending |
| 165 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 7325 | #[allow(clippy::needless_for_each)] // recursive schema-reference normalization is clearer as iterator traversal | Pending |
| 166 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 10100 | #[allow(clippy::single_call_fn)] | Pending |
| 167 | pg_crud/pg_table/generate_pg_table_src/src/source.rs | 10132 | #[allow(clippy::absolute_paths)] | Pending |
| 168 | pg_crud/pg_table/src/lib.rs | 1 | #![allow(clippy::arbitrary_source_item_ordering)] // SQL helpers stay grouped by generated CRUD concern rather than alphabetically | Pending |
| 169 | pg_crud/pg_table/src/lib.rs | 68 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 170 | pg_crud/pg_table/src/lib.rs | 89 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 171 | pg_crud/pg_table/src/lib.rs | 105 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 172 | pg_crud/pg_types/generate_pg_types_src/src/catalog.rs | 1 | #![allow( clippy::single_call_fn, reason = "the catalog projection is a physical boundary between descriptors and emitters" )] | Pending |
| 173 | pg_crud/pg_types/generate_pg_types_src/src/filter.rs | 1 | #![allow( clippy::single_call_fn, reason = "the filter projection is a physical boundary between descriptors and emitters" )] | Pending |
| 174 | pg_crud/pg_types/generate_pg_types_src/src/model.rs | 1 | #![allow(clippy::field_scoped_visibility_modifiers)] // the private descriptor is constructed by its sibling catalog while fields remain hidden outside this generator | Pending |
| 175 | pg_crud/pg_types/generate_pg_types_src/src/serde.rs | 1 | #![allow( clippy::single_call_fn, reason = "the Serde projection is a physical boundary between descriptors and emitters" )] | Pending |
| 176 | pg_crud/pg_types/generate_pg_types_src/src/source.rs | 1 | #![allow(clippy::unreachable, clippy::wildcard_enum_match_arm)] // schema branches are guarded by the PostgreSQL type category selected immediately before each match | Pending |
| 177 | pg_crud/pg_types/generate_pg_types_src/src/source.rs | 4 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 178 | pg_crud/pg_types/generate_pg_types_src/src/source.rs | 60 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 179 | pg_crud/pg_types/generate_pg_types_src/src/source.rs | 122 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 180 | pg_crud/pg_types/generate_pg_types_src/src/source.rs | 448 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 181 | pg_crud/pg_types/generate_pg_types_src/src/source.rs | 504 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 182 | pg_crud/pg_types/generate_pg_types_src/src/source.rs | 521 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 183 | pg_crud/pg_types/generate_pg_types_src/src/source.rs | 539 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 184 | pg_crud/pg_types/generate_pg_types_src/src/source.rs | 611 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 185 | pg_crud/pg_types/generate_pg_types_src/src/source.rs | 620 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 186 | pg_crud/pg_types/generate_pg_types_src/src/source.rs | 688 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 187 | pg_crud/pg_types/generate_pg_types_src/src/source.rs | 1478 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 188 | pg_crud/pg_types/generate_pg_types_src/src/source.rs | 1503 | #[allow(clippy::absolute_paths)] | Pending |
| 189 | pg_crud/pg_types/generate_pg_types_src/src/source.rs | 1728 | #[allow(clippy::absolute_paths)] | Pending |
| 190 | pg_crud/pg_types/generate_pg_types_src/src/sqlx.rs | 1 | #![allow( clippy::single_call_fn, reason = "SQLx capability projections are a physical boundary between descriptors and emitters" )] | Pending |
| 191 | pg_crud/pg_types/generate_pg_types_test/src/lib.rs | 2 | #[allow(clippy::default_numeric_fallback, clippy::indexing_slicing)] // literal JSON assertions mirror the exact serialized OpenAPI wire values | Pending |
| 192 | pg_crud/where_filters/generate_where_filters_src/src/bind.rs | 1 | #![allow( clippy::single_call_fn, reason = "the bind emitter boundary is intentionally isolated from descriptor and contract emitters" )] | Pending |
| 193 | pg_crud/where_filters/generate_where_filters_src/src/bind.rs | 6 | #[allow( clippy::field_scoped_visibility_modifiers, reason = "the sibling descriptor validates bind count without exposing a primitive boundary" )] | Pending |
| 194 | pg_crud/where_filters/generate_where_filters_src/src/client.rs | 1 | #![allow( clippy::single_call_fn, reason = "the client emitter boundary is intentionally isolated from descriptor and contract emitters" )] | Pending |
| 195 | pg_crud/where_filters/generate_where_filters_src/src/contract_tests.rs | 1 | #[allow(clippy::single_call_fn)] // validation remains an independently testable typed pipeline stage | Pending |
| 196 | pg_crud/where_filters/generate_where_filters_src/src/contract_tests.rs | 15 | #[allow(clippy::needless_for_each)] | Pending |
| 197 | pg_crud/where_filters/generate_where_filters_src/src/model.rs | 102 | #[allow(clippy::needless_for_each)] // descriptor matrix avoids repository-forbidden for loops | Pending |
| 198 | pg_crud/where_filters/generate_where_filters_src/src/schema.rs | 1 | #![allow( clippy::single_call_fn, reason = "the schema emitter boundary is intentionally isolated from descriptor and contract emitters" )] | Pending |
| 199 | pg_crud/where_filters/generate_where_filters_src/src/source.rs | 94 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 200 | pg_crud/where_filters/generate_where_filters_src/src/source.rs | 99 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 201 | pg_crud/where_filters/generate_where_filters_src/src/source.rs | 995 | #[allow(clippy::wildcard_imports)] | Pending |
| 202 | pg_crud/where_filters/generate_where_filters_src/src/source.rs | 1011 | #[allow(clippy::absolute_paths)] | Pending |
| 203 | pg_crud/where_filters/generate_where_filters_src/src/source.rs | 1012 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 204 | pg_crud/where_filters/generate_where_filters_src/src/sql.rs | 1 | #![allow( clippy::single_call_fn, reason = "the SQL emitter boundary is intentionally isolated from descriptor and source assembly" )] | Pending |
| 205 | pg_crud/where_filters/generate_where_filters_test/src/lib.rs | 2 | #[allow(clippy::needless_for_each)] // table-driven assertions avoid repository-forbidden for loops | Pending |
| 206 | pg_crud/where_filters/generate_where_filters_test/src/lib.rs | 92 | "#![allow(dead_code)]\n#![allow(unreachable_pub)]\n#![allow(unused_imports)]\n#[allow(clippy::wildcard_imports)]\nuse where_filters::*;\n{}", | Pending |
| 207 | pg_crud/where_filters/src/lib.rs | 93 | #[allow(clippy::absolute_paths)] | Pending |
| 208 | pg_crud/where_filters/src/lib.rs | 94 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 209 | pg_crud/where_filters/src/lib.rs | 164 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 210 | pg_crud/where_filters/src/lib.rs | 241 | #[allow(clippy::absolute_paths)] | Pending |
| 211 | pg_crud/where_filters/src/lib.rs | 242 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 212 | pg_crud/where_filters/src/lib.rs | 550 | #[allow(clippy::absolute_paths)] | Pending |
| 213 | pg_crud/where_filters/src/lib.rs | 551 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 214 | prepare_postgresql_databases/src/lib.rs | 103 | #[allow(clippy::single_call_fn)] // named validation boundary is consumed by the Newtype derive | Pending |
| 215 | prepare_postgresql_databases/src/lib.rs | 118 | #[allow(clippy::single_call_fn)] // named validation boundary is consumed by the Newtype derive | Pending |
| 216 | route_validators/src/check_body_size.rs | 50 | #[allow(clippy::single_call_fn)] // keeps body-size error construction reusable and testable in one place | Pending |
| 217 | route_validators/src/check_body_size.rs | 100 | #[allow(clippy::single_call_fn)] // shared extractor keeps reached-max-size assertions reusable across tests | Pending |
| 218 | route_validators/src/check_commit.rs | 39 | #[allow(clippy::single_call_fn)] // keeps mismatch error construction reusable and explicit | Pending |
| 219 | route_validators/src/check_commit.rs | 49 | #[allow(clippy::single_call_fn)] // keeps header to-str conversion error construction reusable | Pending |
| 220 | route_validators/src/check_commit.rs | 56 | #[allow(clippy::single_call_fn)] // keeps missing-commit-header error construction reusable | Pending |
| 221 | route_validators/src/check_commit.rs | 66 | #[allow(clippy::single_call_fn)] // separates commit-value validation from header parsing for reuse and focused tests | Pending |
| 222 | route_validators/src/check_commit.rs | 78 | #[allow(clippy::single_call_fn)] // shared extractor keeps commit-header parsing reusable across commit-check entry points | Pending |
| 223 | route_validators/src/check_commit.rs | 89 | #[allow(clippy::single_call_fn)] // reusable validator keeps check_commit focused on feature-toggle behavior | Pending |
| 224 | route_validators/src/hdr_val.rs | 19 | #[allow(clippy::single_call_fn)] // typed accessor keeps test assertions from exposing the tuple field | Pending |
| 225 | route_validators/src/hdr_val.rs | 24 | #[allow(clippy::single_call_fn)] // shared helper centralizes required-header extraction and no-header error mapping | Pending |
| 226 | route_validators/src/hdr_val.rs | 36 | #[allow(clippy::single_call_fn)] // shared helper keeps HeaderValue->str conversion and error mapping centralized for header parsers | Pending |
| 227 | route_validators/src/hdr_val.rs | 47 | #[allow(clippy::single_call_fn)] // core helper centralizes required-header transform flow reused by parsing helpers | Pending |
| 228 | route_validators/src/hdr_val.rs | 58 | #[allow(clippy::single_call_fn)] // helper centralizes required-header parsing and is reusable across validators | Pending |
| 229 | route_validators/src/hdr_val.rs | 67 | #[allow(clippy::single_call_fn)] // helper centralizes required-header string parsing and is reusable across validators | Pending |
| 230 | route_validators/src/hdr_val.rs | 76 | #[allow(clippy::single_call_fn)] // helper centralizes required-header string parsing and is reused by route validators | Pending |
| 231 | route_validators/src/test_hlp.rs | 1 | #![allow(clippy::shadow_reuse)] | Pending |
| 232 | route_validators/src/test_hlp.rs | 151 | #[allow(clippy::single_call_fn)] // shared helper composes result extraction with variant mapping for concise validator tests | Pending |
| 233 | route_validators/src/test_hlp.rs | 212 | #[allow(clippy::single_call_fn)] // shared helper composes status-code assertion with variant mapping to reduce repetitive test boilerplate | Pending |
| 234 | server/src/main.rs | 140 | #[allow(clippy::single_call_fn)] // keeps validated maintenance policy separate from startup orchestration | Pending |
| 235 | server/src/main.rs | 159 | #[allow(clippy::single_call_fn)] // isolates the fallback router for an end-to-end routing test | Pending |
| 236 | server/src/main.rs | 165 | #[allow(clippy::single_call_fn)] // startup and tests share the service route mounting invariant | Pending |
| 237 | server/src/main.rs | 182 | #[allow(clippy::single_call_fn)] // route wiring is reused by startup flow and isolated from layer setup | Pending |
| 238 | server/src/main.rs | 255 | #[allow(clippy::single_call_fn)] // keeps state creation shape reusable and type-stable in one place | Pending |
| 239 | server/src/main.rs | 277 | #[allow(clippy::single_call_fn)] // runtime builder is shared by main and can be reused by startup tests | Pending |
| 240 | server/src/main.rs | 286 | #[allow(clippy::single_call_fn)] // isolated pool builder keeps startup flow linear and reuses config getters in one place | Pending |
| 241 | server/src/main.rs | 324 | #[allow(clippy::single_call_fn)] // startup flow is grouped for separation from process/bootstrap concerns | Pending |
| 242 | server/src/main.rs | 574 | #[allow(clippy::single_call_fn)] // shutdown signal ownership stays isolated from server assembly | Pending |
| 243 | server/src/main.rs | 581 | #[allow( clippy::integer_division_remainder_used, clippy::single_call_fn, reason = "tokio::select macro internals trigger the remainder lint; shutdown signal ownership stays isolated" )] | Pending |
| 244 | server_admin/src/auth.rs | 1 | #![allow(clippy::needless_for_each)] // utoipa 4 generated OpenAPI registration uses iterator callbacks | Pending |
| 245 | server_admin/src/auth.rs | 14 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 246 | server_admin/src/auth.rs | 30 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 247 | server_admin/src/auth.rs | 46 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 248 | server_admin/src/auth.rs | 73 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 249 | server_admin/src/auth.rs | 106 | #[allow( clippy::single_call_fn, reason = "keeps every administrator authentication threshold in one immutable policy constructor" )] | Pending |
| 250 | server_admin/src/auth.rs | 154 | #[allow(clippy::single_call_fn)] // sign-in accepts existing credentials without applying the policy for newly assigned passwords | Pending |
| 251 | server_admin/src/auth.rs | 238 | #[allow(clippy::field_scoped_visibility_modifiers)] // repository query binding consumes this internal cross-module DTO field-by-field | Pending |
| 252 | server_admin/src/auth.rs | 492 | #[allow(clippy::single_call_fn)] // CSRF origin validation stays isolated from token validation | Pending |
| 253 | server_admin/src/auth.rs | 954 | #[allow(clippy::single_call_fn)] // sign-in alone creates the long-lived refresh cookie | Pending |
| 254 | server_admin/src/auth.rs | 1031 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 255 | server_admin/src/auth.rs | 1042 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 256 | server_admin/src/auth.rs | 1047 | #[allow(clippy::single_call_fn)] | Pending |
| 257 | server_admin/src/auth.rs | 1057 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 258 | server_admin/src/auth.rs | 1067 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 259 | server_admin/src/auth.rs | 1074 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 260 | server_admin/src/auth.rs | 1087 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 261 | server_admin/src/auth.rs | 1097 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 262 | server_admin/src/auth.rs | 1120 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 263 | server_admin/src/auth.rs | 1130 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 264 | server_admin/src/auth.rs | 1141 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 265 | server_admin/src/auth.rs | 1152 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 266 | server_admin/src/auth.rs | 1163 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 267 | server_admin/src/auth.rs | 1173 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 268 | server_admin/src/auth.rs | 1183 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 269 | server_admin/src/auth.rs | 1194 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 270 | server_admin/src/auth.rs | 1204 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 271 | server_admin/src/auth.rs | 1215 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 272 | server_admin/src/auth.rs | 1226 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 273 | server_admin/src/auth.rs | 1236 | #[allow(clippy::single_call_fn)] | Pending |
| 274 | server_admin/src/auth.rs | 1246 | #[allow(clippy::single_call_fn)] | Pending |
| 275 | server_admin/src/auth.rs | 1253 | #[allow(clippy::single_call_fn)] | Pending |
| 276 | server_admin/src/auth.rs | 1260 | #[allow(clippy::single_call_fn)] | Pending |
| 277 | server_admin/src/auth.rs | 1274 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 278 | server_admin/src/auth.rs | 1284 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 279 | server_admin/src/auth.rs | 1297 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 280 | server_admin/src/auth.rs | 1310 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 281 | server_admin/src/auth.rs | 1323 | #[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory | Pending |
| 282 | server_admin/src/auth.rs | 1462 | #[allow(clippy::single_call_fn)] // facade keeps session persistence private to the session module | Pending |
| 283 | server_admin/src/auth.rs | 1471 | #[allow(clippy::single_call_fn)] // facade keeps refreshed-session persistence private to the session module | Pending |
| 284 | server_admin/src/auth/audit.rs | 1 | #![allow(clippy::single_call_fn)] // audit boundaries isolate append/query SQL from route and transaction facades | Pending |
| 285 | server_admin/src/auth/handlers.rs | 1 | #![allow(clippy::single_call_fn)] // route facade preserves utoipa inventory while private implementations own handler logic | Pending |
| 286 | server_admin/src/auth/html.rs | 1 | #![allow(clippy::single_call_fn)] // each server-rendered HTML handler is registered once in the Axum route inventory | Pending |
| 287 | server_admin/src/auth/html.rs | 2 | #![allow( clippy::shadow_reuse, reason = "form adapters deliberately replace unvalidated extractor values with validated domain values" )] | Pending |
| 288 | server_admin/src/auth/rate_limit.rs | 1 | #![allow(clippy::field_scoped_visibility_modifiers)] // auth state reads the validated count while the private module owns construction and enforcement | Pending |
| 289 | server_admin/src/auth/rate_limit.rs | 11 | #[allow(clippy::single_call_fn)] // scope serialization is shared by persistence and exhaustive contract tests | Pending |
| 290 | server_admin/src/auth/routes.rs | 1 | #![allow(clippy::single_call_fn)] // public facade keeps stable auth module paths while this module owns router and OpenAPI composition | Pending |
| 291 | server_admin/src/auth/session.rs | 1 | #![allow(clippy::single_call_fn)] // public facade preserves session API while this module owns persistence and rotation | Pending |
| 292 | server_admin/src/auth/session.rs | 2 | #[allow(clippy::single_call_fn)] // clock failure mapping remains isolated from session persistence | Pending |
| 293 | server_admin/src/auth/session.rs | 9 | #[allow(clippy::single_call_fn)] // token identifier conversion keeps secret construction explicit | Pending |
| 294 | server_admin/src/cleanup.rs | 1 | #![allow(clippy::single_call_fn)] // stable root cleanup API delegates to the private bounded-cleanup module | Pending |
| 295 | server_admin/src/generated_tables.rs | 1 | #![allow(clippy::needless_for_each, clippy::partial_pub_fields)] // generated contracts expose operation fields while source table fields stay private to protect password hashes | Pending |
| 296 | server_admin/src/generated_tables.rs | 2 | #[allow( clippy::arbitrary_source_item_ordering, clippy::needless_for_each, clippy::partial_pub_fields )] | Pending |
| 297 | server_admin/src/generated_tables.rs | 33 | #[allow(clippy::missing_fields_in_debug)] // password_hash is intentionally represented by a redacted constant | Pending |
| 298 | server_admin/src/generated_tables.rs | 47 | #[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order | Pending |
| 299 | server_admin/src/generated_tables.rs | 72 | #[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order | Pending |
| 300 | server_admin/src/generated_tables.rs | 97 | #[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order | Pending |
| 301 | server_admin/src/generated_tables.rs | 122 | #[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order | Pending |
| 302 | server_admin/src/generated_tables.rs | 142 | #[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order | Pending |
| 303 | server_admin/src/lib.rs | 1 | #![allow(clippy::arbitrary_source_item_ordering)] // domain declarations are grouped by authentication and authorization responsibility | Pending |
| 304 | server_admin/src/lib.rs | 199 | #[allow( clippy::single_call_fn, reason = "the crate-private constructor is the invariant boundary for SHA-256 token hashes" )] | Pending |
| 305 | server_admin/src/lib.rs | 598 | #[allow(clippy::needless_for_each, clippy::single_call_fn)] // repository policy forbids for loops and compact fixtures keep secret setup deterministic | Pending |
| 306 | server_admin/src/migrations.rs | 1 | #![allow(clippy::single_call_fn)] // stable root migration/bootstrap API delegates to the private persistence module | Pending |
| 307 | server_admin/src/password.rs | 3 | #[allow(clippy::missing_const_for_fn)] // Tokio semaphore and Arc constructors are not const | Pending |
| 308 | server_admin/src/repository.rs | 1 | #![allow(clippy::single_call_fn)] // shared repository boundary types support thematic SQL owner modules | Pending |
| 309 | server_admin/src/repository/audit.rs | 1 | #![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract | Pending |
| 310 | server_admin/src/repository/cleanup.rs | 1 | #![allow(clippy::single_call_fn)] // the cleanup transaction owns all related retention queries | Pending |
| 311 | server_admin/src/repository/data_tables.rs | 1 | #![allow(clippy::single_call_fn)] // one bounded query serves the read-only table inspection boundary | Pending |
| 312 | server_admin/src/repository/data_tables.rs | 621 | #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy | Pending |
| 313 | server_admin/src/repository/permissions.rs | 1 | #![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract | Pending |
| 314 | server_admin/src/repository/rate_limits.rs | 1 | #![allow(clippy::single_call_fn)] // the typed function owns the PostgreSQL rate-limit contract | Pending |
| 315 | server_admin/src/repository/roles.rs | 1 | #![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract | Pending |
| 316 | server_admin/src/repository/roles.rs | 246 | #[allow(clippy::single_call_fn)] // one transaction-level operation owns the lock/read ordering for the last-administrator invariant | Pending |
| 317 | server_admin/src/repository/sessions.rs | 1 | #![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract | Pending |
| 318 | server_admin/src/repository/sessions.rs | 127 | #[allow(clippy::too_many_arguments)] // the access-session row has six independently typed persisted fields | Pending |
| 319 | server_admin/src/repository/settings.rs | 1 | #![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract | Pending |
| 320 | server_admin/src/repository/users.rs | 1 | #![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract | Pending |
| 321 | server_admin/src/token.rs | 1 | #![allow(clippy::single_call_fn)] // stable root token API delegates to the private cryptographic responsibility module | Pending |
| 322 | server_admin/tests/admin_api.rs | 3 | #![allow(clippy::tests_outside_test_module)] // every item in this integration target is compiled exclusively by the test harness | Pending |
| 323 | server_admin/tests/admin_api.rs | 482 | #[allow( clippy::needless_for_each, reason = "repository policy requires iterator methods instead of for loops" )] | Pending |
| 324 | server_admin_contract/src/lib.rs | 1 | #![allow(clippy::arbitrary_source_item_ordering)] // DTO implementations keep constructors adjacent to their accessors and route metadata grouped by concern | Pending |
| 325 | server_admin_contract/src/lib.rs | 1695 | #[allow( clippy::derivable_impls, reason = "only identifier request collections intentionally expose Default" )] | Pending |
| 326 | server_admin_contract/src/lib.rs | 1704 | #[allow( clippy::derivable_impls, reason = "only identifier request collections intentionally expose Default" )] | Pending |
| 327 | server_admin_contract/src/lib.rs | 2065 | #[allow(clippy::same_name_method)] // Utoipa 5's static schema name intentionally coexists with this domain accessor | Pending |
| 328 | server_admin_contract/src/lib.rs | 2089 | #[allow(clippy::same_name_method)] // Utoipa 5's static schema name intentionally coexists with this domain accessor | Pending |
| 329 | server_admin_contract/src/lib.rs | 2328 | #[allow(clippy::same_name_method)] // Utoipa 5's static schema name intentionally coexists with this domain accessor | Pending |
| 330 | server_admin_contract/src/lib.rs | 4165 | #[allow( clippy::needless_for_each, reason = "repository source policy requires iterator methods" )] | Pending |
| 331 | server_admin_core/src/lib.rs | 1 | #![allow(clippy::field_scoped_visibility_modifiers)] // sibling domain modules require raw representations while facade reexports must keep fields externally private | Pending |
| 332 | server_admin_frontend/src/app.rs | 1 | #![allow( clippy::same_name_method, clippy::shadow_reuse, clippy::single_call_fn, clippy::unused_trait_names, reason = "Leptos component and entry-point macro expansion produces these patterns" )] | Pending |
| 333 | server_admin_frontend/src/app.rs | 262 | #[allow( clippy::future_not_send, reason = "browser fetch futures run exclusively on wasm_bindgen_futures::spawn_local" )] | Pending |
| 334 | server_admin_frontend/src/app.rs | 312 | #[allow( clippy::future_not_send, reason = "browser mutation requests run exclusively on wasm_bindgen_futures::spawn_local" )] | Pending |
| 335 | server_admin_frontend/src/app.rs | 395 | #[allow( clippy::future_not_send, reason = "browser page loads run exclusively on wasm_bindgen_futures::spawn_local" )] | Pending |
| 336 | server_admin_frontend/src/lib.rs | 49 | #[allow( clippy::arbitrary_source_item_ordering, reason = "test modules stay last to satisfy clippy::items_after_test_module" )] | Pending |
| 337 | server_admin_frontend/src/shared.rs | 1 | #![allow( clippy::arbitrary_source_item_ordering, clippy::shadow_reuse, clippy::single_call_fn, clippy::unused_trait_names, reason = "shared Leptos renderers stay adjacent to their field metadata; view expansion requires attribute traits, consumes converted query values, and each target uses the shared renderer once" )] | Pending |
| 338 | server_admin_frontend/src/ssr.rs | 1 | #![allow( clippy::unused_trait_names, reason = "Leptos view macro expansion requires these attribute traits in lexical scope and repository policy forbids underscore import aliases" )] | Pending |
| 339 | server_admin_frontend/src/ssr.rs | 231 | #[allow(clippy::single_call_fn)] // isolates the metadata-driven grid for focused SSR contract testing | Pending |
| 340 | server_app_state/src/lib.rs | 10 | #[allow(clippy::single_call_fn)] // keeps config forwarding in one place for all generated trait impls | Pending |
| 341 | server_app_state/src/lib.rs | 222 | #[allow(clippy::single_call_fn)] // shared fixture keeps commit test input consistent across ServerAppState tests | Pending |
| 342 | server_config/src/lib.rs | 1 | #[allow(clippy::arbitrary_source_item_ordering)] | Pending |
| 343 | server_config/tests/config_descriptor.rs | 6 | #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy | Pending |
| 344 | server_runtime_core/src/lease_registry.rs | 185 | #[allow(clippy::single_call_fn)] // keeps the two-index conflict update atomic and locally auditable | Pending |
| 345 | server_runtime_core/src/lease_registry.rs | 195 | #[allow(clippy::single_call_fn)] // keeps stale eviction synchronized across both indexes | Pending |
| 346 | server_runtime_core/src/lease_registry.rs | 197 | #[allow(clippy::needless_collect)] // ids must be owned before mutating both registry indexes | Pending |
| 347 | server_runtime_core/src/resource_utilization.rs | 34 | #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value | Pending |
| 348 | server_runtime_core/src/single_flight.rs | 41 | #[allow(clippy::missing_const_for_fn)] // the lock-backed return value cannot be constructed in const context | Pending |
| 349 | server_runtime_http/src/bounded_read.rs | 152 | #[allow( clippy::single_call_fn, reason = "test seam simulates file growth between metadata and content reads" )] | Pending |
| 350 | server_runtime_http/src/child_process.rs | 232 | #[allow(clippy::single_call_fn)] // isolates optional diagnostic task joining from process state transitions | Pending |
| 351 | server_runtime_http/src/child_process.rs | 246 | #[allow(clippy::single_call_fn)] // generic reader keeps bounded diagnostic behavior independently testable | Pending |
| 352 | server_runtime_http/src/fallback.rs | 1 | #![allow( clippy::single_call_fn, reason = "media-range classification stays isolated from fallback policy resolution" )] | Pending |
| 353 | server_runtime_http/src/http_header_policy.rs | 1 | #![allow( clippy::arbitrary_source_item_ordering, reason = "HTTP header policy types stay grouped with their builders" )] | Pending |
| 354 | server_runtime_http/src/http_policy.rs | 1 | #![allow( clippy::arbitrary_source_item_ordering, reason = "HTTP policy types stay grouped with their corresponding resolver functions" )] | Pending |
| 355 | server_runtime_http/src/lib.rs | 257 | #[allow(clippy::single_call_fn)] // shared preparation keeps production execution and deterministic propagation tests on the same implementation | Pending |
| 356 | server_runtime_http/src/lib.rs | 938 | #[allow(clippy::integer_division_remainder_used)] // tokio::select expansion uses internal randomized branch arithmetic | Pending |
| 357 | server_runtime_http/src/lifecycle.rs | 116 | #[allow(clippy::integer_division_remainder_used)] | Pending |
| 358 | server_runtime_http/src/metrics_layer.rs | 96 | #[allow(clippy::arbitrary_source_item_ordering)] // constructor precedes cache lookup implementation | Pending |
| 359 | server_runtime_http/src/metrics_layer.rs | 98 | #[allow(clippy::single_call_fn)] // cache construction owns its capacity invariant | Pending |
| 360 | server_runtime_http/src/multipart.rs | 1 | #![allow( clippy::arbitrary_source_item_ordering, reason = "multipart domain declarations stay adjacent to their validation implementations" )] | Pending |
| 361 | server_runtime_http/src/notification.rs | 171 | #[allow(clippy::single_call_fn)] // named handler keeps axum extractor boundaries domain-typed | Pending |
| 362 | server_runtime_http/src/origin.rs | 103 | #[allow(clippy::single_call_fn)] // parsing is independently testable through origin resolution | Pending |
| 363 | server_runtime_http/src/outbound_url.rs | 206 | #[allow(clippy::single_call_fn)] // keeps percent-encoded control recognition separate from URL parsing policy | Pending |
| 364 | server_runtime_http/src/path_policy.rs | 1 | #![allow( clippy::arbitrary_source_item_ordering, reason = "path policy types stay grouped with their validation operations" )] | Pending |
| 365 | tests/src/code_style/advanced_policy.rs | 1 | #![allow( clippy::arbitrary_source_item_ordering, clippy::needless_for_each, clippy::single_call_fn, clippy::wildcard_enum_match_arm, reason = "policy visitors stay grouped with their invariant, repository policy requires iterator methods, and syn non-exhaustive enums require fallback handling" )] | Pending |
| 366 | tests/src/code_style/cargo_policy.rs | 1 | #![allow( clippy::needless_for_each, reason = "repository source policy requires iterator methods instead of for loops" )] | Pending |
| 367 | tests/src/code_style/ci_policy.rs | 22 | #[allow( clippy::single_call_fn, reason = "keeps YAML quote state isolated and fixture-testable" )] | Pending |
| 368 | tests/src/code_style/ci_policy.rs | 57 | #[allow( clippy::needless_for_each, reason = "repository source policy requires iterator methods instead of for loops" )] | Pending |
| 369 | tests/src/code_style/deployment_policy.rs | 1 | #![allow( clippy::needless_for_each, reason = "repository source policy requires iterator methods instead of for loops" )] | Pending |
| 370 | tests/src/code_style/mod.rs | 659 | #[allow(clippy::needless_for_each)] // repository source policy requires iterator methods instead of for loops | Pending |
| 371 | tests/src/code_style/mod.rs | 1057 | #[allow( clippy::single_call_fn, reason = "the expression-shape predicate keeps the visitor branch readable" )] | Pending |
| 372 | tests/src/code_style/mod.rs | 1114 | #[allow( clippy::single_call_fn, reason = "the expression-shape predicate keeps the visitor branch readable" )] | Pending |
| 373 | tests/src/code_style/mod.rs | 1619 | #[allow( clippy::single_call_fn, reason = "the expression-shape predicate keeps the visitor branch readable" )] | Pending |
| 374 | tests/src/code_style/mod.rs | 1794 | #[allow( clippy::single_call_fn, reason = "the expression-shape predicate keeps the visitor branch readable" )] | Pending |
| 375 | tests/src/code_style/mod.rs | 3552 | #[allow(clippy::single_call_fn)] // named exception source keeps policy data separate from its validation loop | Pending |
| 376 | tests/src/code_style/mod.rs | 3556 | #[allow(clippy::single_call_fn)] // validates every external wrapper naming exception has an explicit reason before matching idents | Pending |
| 377 | tests/src/code_style/mod.rs | 3588 | #[allow(clippy::single_call_fn)] // keeps diagnostic-ID syntax validation named and fixture-tested | Pending |
| 378 | tests/src/code_style/mod.rs | 3693 | #[allow(clippy::single_call_fn)] // centralizes cross-file uniqueness validation behind the public policy test | Pending |
| 379 | tests/src/code_style/mod.rs | 3757 | #[allow(clippy::single_call_fn)] // shared lint-compare wrapper keeps clippy/rust lint test flow aligned and reduces duplicate wiring | Pending |
| 380 | tests/src/code_style/mod.rs | 3774 | #[allow(clippy::single_call_fn)] // helper intentionally stays extracted so command parsing remains decoupled from lint comparison orchestration | Pending |
| 381 | tests/src/code_style/mod.rs | 3811 | #[allow(clippy::single_call_fn)] // shared command-output assertions keep status/stderr checks reusable for command-driven tests | Pending |
| 382 | tests/src/code_style/mod.rs | 3821 | #[allow(clippy::single_call_fn)] // centralizes lint-name normalization used by command output parsing | Pending |
| 383 | tests/src/code_style/mod.rs | 3826 | #[allow(clippy::single_call_fn)] // keeps workspace-dependency shape checks reusable and focused in one helper | Pending |
| 384 | tests/src/code_style/mod.rs | 3947 | #[allow(clippy::single_call_fn)] // separates version shape assertion from dependency-table flow and keeps IDs stable | Pending |
| 385 | tests/src/code_style/mod.rs | 3969 | #[allow(clippy::single_call_fn)] // extracted to avoid repeated feature-type checks for dependency tables | Pending |
| 386 | tests/src/code_style/mod.rs | 3985 | #[allow(clippy::single_call_fn)] // isolates exact-version parsing so version-format checks are reusable and testable | Pending |
| 387 | tests/src/code_style/mod.rs | 3999 | #[allow(clippy::single_call_fn)] // helper intentionally stays extracted so lint diff logic remains reusable and independently readable | Pending |
| 388 | tests/src/code_style/mod.rs | 4036 | #[allow(clippy::single_call_fn)] // shared parser keeps .env line-to-key extraction reusable and test behavior centralized | Pending |
| 389 | tests/src/code_style/mod.rs | 4069 | #[allow(clippy::single_call_fn)] // centralized formatter keeps env key mismatch diagnostics consistent | Pending |
| 390 | tests/src/code_style/mod.rs | 4089 | #[allow(clippy::single_call_fn)] // split keeps lint exception handling explicit while reusing missing-item collection | Pending |
| 391 | tests/src/code_style/mod.rs | 4106 | #[allow(clippy::single_call_fn)] // helper intentionally stays extracted so workspace-lints table parsing remains separate from test driver wiring | Pending |
| 392 | tests/src/code_style/mod.rs | 4134 | #[allow(clippy::single_call_fn)] // reusable collector stays split from assertion helper for callsites that need raw error vectors | Pending |
| 393 | tests/src/code_style/mod.rs | 4147 | #[allow(clippy::single_call_fn)] // centralizes repeated cargo-toml assertion shape used by multiple tests | Pending |
| 394 | tests/src/code_style/mod.rs | 4157 | #[allow(clippy::single_call_fn)] // shared crate-manifest cargo policy assertion keeps joined-diagnostic behavior consistent across package-metadata checks | Pending |
| 395 | tests/src/code_style/mod.rs | 4195 | #[allow(clippy::single_call_fn)] // shared sort+assert helper keeps joined diagnostics deterministic for tests that accumulate path-dependent errors | Pending |
| 396 | tests/src/code_style/mod.rs | 4211 | #[allow(clippy::single_call_fn)] // shared duplicate finder keeps uniqueness checks reusable and consistent | Pending |
| 397 | tests/src/code_style/mod.rs | 4222 | #[allow(clippy::single_call_fn)] // reusable collector stays available for AST-policy tests and keeps collection logic separate from assertion wrappers | Pending |
| 398 | tests/src/code_style/mod.rs | 4239 | #[allow(clippy::single_call_fn)] // shared assertion wrapper keeps AST-policy tests focused on visitor logic while reusing collection and joined-report formatting | Pending |
| 399 | tests/src/code_style/mod.rs | 4253 | #[allow(clippy::single_call_fn)] // shared lookup avoids rereading crate manifests in text-based Cargo.toml style checks | Pending |
| 400 | tests/src/code_style/mod.rs | 4257 | #[allow(clippy::single_call_fn)] // isolates non-english diagnostics so file-level test stays focused on traversal and assertion | Pending |
| 401 | tests/src/code_style/mod.rs | 4283 | #[allow(clippy::single_call_fn)] // shared character predicate keeps english-only symbol policy centralized | Pending |
| 402 | tests/src/code_style/mod.rs | 4290 | #[allow(clippy::single_call_fn)] // shared repeated-file error helper keeps AST visitor diagnostics consistent | Pending |
| 403 | tests/src/code_style/mod.rs | 4302 | #[allow(clippy::single_call_fn)] // package names are used to distinguish workspace paths from external crate paths | Pending |
| 404 | tests/src/code_style/mod.rs | 4306 | #[allow(clippy::single_call_fn)] // shared traversal uses cargo metadata so crate manifests match Cargo's view of workspace packages | Pending |
| 405 | tests/src/code_style/mod.rs | 4312 | #[allow(clippy::single_call_fn)] // shared extension predicate keeps source-policy file-kind checks consistent | Pending |
| 406 | tests/src/code_style/mod.rs | 4337 | #[allow(clippy::single_call_fn)] // names the From<String> trait-shape check for the string-wrapper policy visitor | Pending |
| 407 | tests/src/code_style/mod.rs | 4348 | #[allow(clippy::single_call_fn)] // names the TryFrom<String> trait-shape check for the string-wrapper policy visitor | Pending |
| 408 | tests/src/code_style/mod.rs | 4359 | #[allow(clippy::single_call_fn)] // keeps length-check detection local to the string-wrapper TryFrom policy | Pending |
| 409 | tests/src/code_style/mod.rs | 4422 | #[allow(clippy::single_call_fn)] // isolates tuple field visibility parsing from policy diagnostics | Pending |
| 410 | tests/src/code_style/mod.rs | 4434 | #[allow(clippy::single_call_fn)] // isolated serde derive detection keeps the visitor condition declarative | Pending |
| 411 | tests/src/code_style/mod.rs | 4447 | #[allow(clippy::single_call_fn)] // isolated serde conversion detection keeps attribute parsing reusable and testable | Pending |
| 412 | tests/src/code_style/mod.rs | 4465 | #[allow(clippy::single_call_fn)] // conversion derive recognition is kept separate from wrapper collection | Pending |
| 413 | tests/src/code_style/mod.rs | 4482 | #[allow(clippy::single_call_fn)] // keeps FromInner derive detection reusable inside wrapper conversion collection | Pending |
| 414 | tests/src/code_style/mod.rs | 4497 | #[allow(clippy::single_call_fn)] // keeps TryFrom derive detection reusable inside wrapper conversion collection | Pending |
| 415 | tests/src/code_style/mod.rs | 4514 | #[allow(clippy::single_call_fn)] // isolates `From<T>` impl detection for tuple-wrapper conversion analysis | Pending |
| 416 | tests/src/code_style/mod.rs | 4522 | #[allow(clippy::single_call_fn)] // isolates `TryFrom<T>` impl detection for tuple-wrapper conversion analysis | Pending |
| 417 | tests/src/code_style/mod.rs | 4556 | #[allow(clippy::single_call_fn)] // diagnostic conversion errors intentionally carry raw length metadata | Pending |
| 418 | tests/src/code_style/mod.rs | 4575 | #[allow(clippy::single_call_fn)] // keeps the derive-validator boundary exception explicit and narrow | Pending |
| 419 | tests/src/code_style/mod.rs | 4609 | #[allow(clippy::single_call_fn)] // keeps FromInner derive detection reusable inside the string-wrapper policy | Pending |
| 420 | tests/src/code_style/mod.rs | 4621 | #[allow(clippy::single_call_fn)] // keeps BoundedString derive parsing reusable inside the string-wrapper policy | Pending |
| 421 | tests/src/code_style/mod.rs | 4633 | #[allow(clippy::single_call_fn)] // bounded string wrappers satisfy length policy only when max is explicit | Pending |
| 422 | tests/src/code_style/mod.rs | 4665 | #[allow(clippy::single_call_fn)] // names Self-path handling separately from domain type path traversal | Pending |
| 423 | tests/src/code_style/mod.rs | 4719 | #[allow(clippy::single_call_fn)] // extracts repo macro domain type discovery from the visitor traversal | Pending |
| 424 | tests/src/code_style/mod.rs | 4735 | #[allow(clippy::single_call_fn)] // config_lib helper macros declare domain wrapper structs from their first argument | Pending |
| 425 | tests/src/code_style/mod.rs | 4798 | #[allow(clippy::single_call_fn)] // keeps Arc type policy readable apart from syn type matching | Pending |
| 426 | tests/src/code_style/mod.rs | 4824 | #[allow(clippy::single_call_fn)] // names the async-blocking method policy separately from traversal code | Pending |
| 427 | tests/src/code_style/mod.rs | 4834 | #[allow(clippy::single_call_fn)] // names the async-blocking function policy separately from traversal code | Pending |
| 428 | tests/src/code_style/mod.rs | 4878 | #[allow(clippy::single_call_fn)] // names the external-service unit-test policy separately from traversal code | Pending |
| 429 | tests/src/code_style/mod.rs | 4981 | #[allow(clippy::single_call_fn)] // extracted to keep the domain policy test focused on assertion flow | Pending |
| 430 | tests/src/code_style/mod.rs | 5011 | #[allow(clippy::single_call_fn)] // keeps domain policy exception handling centralized and documented | Pending |
| 431 | tests/src/code_style/mod.rs | 5027 | #[allow(clippy::single_call_fn)] // helper-return text wrappers live in the code-style meta harness types module | Pending |
| 432 | tests/src/code_style/mod.rs | 5034 | #[allow(clippy::single_call_fn)] // keeps transparent container policy separate from path validation | Pending |
| 433 | tests/src/code_style/mod.rs | 5097 | #[allow(clippy::single_call_fn)] // separates return path matching from nested raw text return traversal | Pending |
| 434 | tests/src/code_style/mod.rs | 5144 | #[allow(clippy::single_call_fn)] // keeps nested helper-return traversal independent from field-state diagnostics | Pending |
| 435 | tests/src/code_style/mod.rs | 5169 | #[allow(clippy::single_call_fn)] // separates path-shape matching from recursive wrapper/state field traversal | Pending |
| 436 | tests/src/code_style/mod.rs | 5219 | #[allow(clippy::single_call_fn)] // keeps nested container traversal readable where state fields are diagnosed | Pending |
| 437 | tests/src/code_style/mod.rs | 5407 | #[allow(clippy::single_call_fn)] // exact terminal parsing prevents redacted derive names from matching Debug | Pending |
| 438 | tests/src/code_style/mod.rs | 5442 | #[allow(clippy::single_call_fn)] // limits the secret Debug policy to wrappers that directly contain text or bytes | Pending |
| 439 | tests/src/code_style/mod.rs | 5507 | #[allow(clippy::single_call_fn)] // keeps external-wrapper naming suggestion generation readable at the call site | Pending |
| 440 | tests/src/code_style/mod.rs | 5529 | #[allow(clippy::single_call_fn)] // centralizes production-source filtering for panic/expect/unwrap policy | Pending |
| 441 | tests/src/code_style/mod.rs | 5579 | #[allow(clippy::single_call_fn)] // resolves exact Cargo package ownership for source-policy test exclusions | Pending |
| 442 | tests/src/code_style/mod.rs | 5596 | #[allow(clippy::single_call_fn)] // exact path helper is shared by the duplicate-string policy and its scope regression | Pending |
| 443 | tests/src/code_style/mod.rs | 5606 | #[allow(clippy::single_call_fn)] // exact owner-path matching is shared by direct-filesystem policy and its scope regression | Pending |
| 444 | tests/src/code_style/mod.rs | 5615 | #[allow(clippy::single_call_fn)] // proc-macro crates are allowed to panic by repository policy | Pending |
| 445 | tests/src/code_style/mod.rs | 5712 | #[allow(clippy::single_call_fn)] // shared rust-file reader keeps skip-on-read-error behavior centralized across source policy checks | Pending |
| 446 | tests/src/code_style/mod.rs | 5739 | #[allow(clippy::single_call_fn)] // shared owned-value table extractor keeps table-shape validation reusable where ownership is required | Pending |
| 447 | tests/src/code_style/mod.rs | 5765 | #[allow(clippy::single_call_fn)] // shared collector keeps workspace-dependency policy checks reusable and centralized | Pending |
| 448 | tests/src/code_style/mod.rs | 5835 | #[allow(clippy::single_call_fn)] // keeps dependency-policy validation centralized for dependencies/dev-dependencies/build-dependencies checks | Pending |
| 449 | tests/src/code_style/mod.rs | 5849 | #[allow(clippy::single_call_fn)] // shared message builder keeps dependency-policy errors identical across call sites | Pending |
| 450 | tests/src/code_style/mod.rs | 5863 | #[allow(clippy::single_call_fn)] // dedicated collector keeps workspace-members existence diagnostics reusable and deterministic with caller-managed sorting | Pending |
| 451 | tests/src/code_style/mod.rs | 5885 | #[allow(clippy::single_call_fn)] // central member extraction keeps workspace-members readers strict and reusable across membership checks | Pending |
| 452 | tests/src/code_style/snapshot.rs | 47 | #[allow(clippy::single_call_fn)] // named constructor keeps snapshot initialization readable at the thread-local OnceCell call site | Pending |
| 453 | tests/src/code_style/snapshot.rs | 122 | #[allow(clippy::single_call_fn)] | Pending |
| 454 | tests/src/code_style/snapshot.rs | 131 | #[allow(clippy::single_call_fn)] // named constructor keeps process-wide immutable source initialization readable | Pending |
| 455 | tests/src/code_style/snapshot.rs | 192 | #[allow(clippy::single_call_fn)] // isolates the process-wide source cache from thread-local parsed snapshot construction | Pending |
| 456 | tests/src/code_style/snapshot.rs | 206 | #[allow(clippy::single_call_fn)] // isolates cargo_metadata command setup from snapshot construction | Pending |
| 457 | tests/src/code_style/snapshot.rs | 215 | #[allow(clippy::single_call_fn)] // keeps workspace membership extraction named while snapshot construction reuses it twice | Pending |
| 458 | tests/src/code_style/snapshot.rs | 227 | #[allow(clippy::single_call_fn)] // keeps filesystem walker rules separate from snapshot materialization | Pending |
| 459 | tests/src/code_style/source_policy.rs | 243 | #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy | Pending |
| 460 | tests/src/code_style/source_policy.rs | 271 | #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy | Pending |
| 461 | tests/src/code_style/source_policy.rs | 322 | #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy | Pending |
| 462 | tests/src/code_style/source_policy.rs | 400 | #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy | Pending |
| 463 | tests/src/code_style/source_policy.rs | 1748 | #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy | Pending |
| 464 | tests/src/code_style/source_policy.rs | 2635 | #[allow( clippy::needless_for_each, reason = "repository source policy requires iterator methods instead of for loops" )] | Pending |
| 465 | tests/src/code_style/types.rs | 71 | #[allow(clippy::single_call_fn)] // preserves the source lifetime where AsRef would borrow the wrapper temporary | Pending |
| 466 | text_policy/src/lib.rs | 1 | #![allow( clippy::arbitrary_source_item_ordering, reason = "validators stay adjacent to their domain wrappers and ranges retain minimum-then-maximum order" )] | Pending |
| 467 | token_patterns/src/lib.rs | 146 | token_patterns_macros::tp!(AllowClippyArbitrarySrcItemOrdering, #[allow(clippy::arbitrary_source_item_ordering)]); | Pending |
| 468 | workspace_scaffold/src/main.rs | 157 | #[allow( clippy::single_call_fn, reason = "project command owns repository URL validation" )] | Pending |
| 469 | workspace_scaffold/src/main.rs | 189 | #[allow( clippy::single_call_fn, reason = "service scaffold owns identifier case conversion" )] | Pending |
| 470 | workspace_scaffold/src/main.rs | 202 | #[allow( clippy::single_call_fn, reason = "identity traversal owns ignored directory policy" )] | Pending |
| 471 | workspace_scaffold/src/main.rs | 250 | #[allow( clippy::single_call_fn, reason = "project command owns identity traversal" )] | Pending |
| 472 | workspace_scaffold/src/main.rs | 356 | #[allow( clippy::single_call_fn, reason = "deployment synchronization owns catalog parsing" )] | Pending |
| 473 | workspace_scaffold/src/main.rs | 478 | #[allow( clippy::single_call_fn, reason = "deployment synchronization owns the CI projection" )] | Pending |
| 474 | workspace_scaffold/src/main.rs | 499 | #[allow( clippy::single_call_fn, reason = "deployment synchronization owns the release projection" )] | Pending |
| 475 | workspace_scaffold/src/main.rs | 520 | #[allow( clippy::single_call_fn, reason = "catalog validation keeps path traversal checks explicit and typed" )] | Pending |
| 476 | workspace_scaffold/src/main.rs | 533 | #[allow( clippy::single_call_fn, reason = "deployment synchronization validates every non-generated catalog consumer" )] | Pending |
| 477 | workspace_scaffold/src/main.rs | 597 | #[allow( clippy::single_call_fn, reason = "deployment synchronization owns all per-service generated sections" )] | Pending |
| 478 | workspace_scaffold/src/main.rs | 806 | #[allow( clippy::single_call_fn, reason = "generated file synchronization owns marker replacement" )] | Pending |
| 479 | workspace_scaffold/src/main.rs | 850 | #[allow( clippy::single_call_fn, reason = "the deployment command owns all generated projections" )] | Pending |
| 480 | workspace_scaffold/src/main.rs | 885 | #[allow( clippy::single_call_fn, reason = "the aggregate generation command delegates snapshot ownership to code-style tests" )] | Pending |
| 481 | workspace_scaffold/src/main.rs | 939 | #[allow( clippy::single_call_fn, reason = "the aggregate generation command delegates environment projection ownership to config crates" )] | Pending |
| 482 | workspace_scaffold/src/main.rs | 965 | #[allow( clippy::single_call_fn, reason = "the generate command exposes one aggregate synchronization boundary" )] | Pending |
| 483 | workspace_scaffold/src/main.rs | 978 | #[allow( clippy::single_call_fn, reason = "service command owns complete scaffold composition" )] | Pending |
| 484 | workspace_scaffold/src/main.rs | 1180 | #[allow( clippy::single_call_fn, reason = "binary entry point delegates fallible argument handling" )] | Pending |
| 485 | workspace_test_runner/src/discovery.rs | 1 | #![allow(clippy::single_call_fn)] // discovery remains a separate responsibility even when a mode has one orchestration caller | Pending |
| 486 | workspace_test_runner/src/execution.rs | 95 | #[allow(clippy::single_call_fn)] // summary sanitization stays independently unit-testable | Pending |
| 487 | workspace_test_runner/src/execution.rs | 114 | #[allow(clippy::single_call_fn)] // bounded artifact naming stays isolated from process execution | Pending |
| 488 | workspace_test_runner/src/execution.rs | 138 | #[allow(clippy::single_call_fn)] // unique run-directory construction has one filesystem owner | Pending |
| 489 | workspace_test_runner/src/execution.rs | 153 | #[allow(clippy::single_call_fn)] // log parsing stays independently unit-testable | Pending |
| 490 | workspace_test_runner/src/execution.rs | 174 | #[allow(clippy::single_call_fn)] // summary persistence remains separate from command orchestration | Pending |
| 491 | workspace_test_runner/src/main.rs | 170 | #[allow(clippy::single_call_fn)] // runtime construction keeps wrapper initialization on From while centralizing tool metadata | Pending |
| 492 | workspace_test_runner/src/main.rs | 551 | #[allow(clippy::single_call_fn)] | Pending |
| 493 | workspace_test_runner/src/main.rs | 565 | #[allow(clippy::single_call_fn)] | Pending |
| 494 | workspace_test_runner/src/main.rs | 585 | #[allow(clippy::single_call_fn)] | Pending |
| 495 | workspace_test_runner/src/main.rs | 613 | #[allow(clippy::single_call_fn)] | Pending |
| 496 | workspace_test_runner/src/main.rs | 663 | #[allow( clippy::needless_for_each, clippy::single_call_fn, reason = "keeps release-tool reporting separate and repository policy forbids for loops" )] | Pending |
| 497 | workspace_test_runner/src/main.rs | 686 | #[allow(clippy::single_call_fn)] // release orchestration is an explicit CLI mode boundary | Pending |
| 498 | workspace_test_runner/src/main.rs | 757 | #[allow( clippy::single_call_fn, reason = "the command-mode facade keeps fixture generation out of main dispatch" )] | Pending |
| 499 | workspace_test_runner/src/reporting.rs | 1 | #![allow(clippy::single_call_fn)] // reporting stays independent from command execution so diagnostics have one owner | Pending |
