# Test policy exceptions

This document inventories deliberate exceptions and reviewed baselines used by the workspace
policy tests. The Rust declarations linked below remain authoritative; an exception must be
removed there when its reason or owner is no longer valid.

## Central path inventories

| Inventory | Entries | What it permits |
| --- | ---: | --- |
| `CODE_STYLE_CLIPPY_LINT_EXCEPTIONS` | 0 | Clippy lints missing from the workspace lint catalog. No exceptions currently exist. |
| `CODE_STYLE_REVIEWED_PUBLIC_FIELD_*` | 14 owner/type records | Specifically named non-private fields required by macro, wire, generated-query, configuration, or application-state contracts. |
| `CODE_STYLE_DIRECT_FS_OWNER_*` | 20 files | Direct environment, filesystem, command-line, or generated-file operations at their owning adapters. |
| `CODE_STYLE_TEST_CRATE_*` | 6 crates | Crates treated as test-only boundaries. |
| `CODE_STYLE_UNBOUNDED_READ_OWNER_SUFFIXES` | 0 | Whole-file owners allowed to perform unbounded reads. No exceptions currently exist. |
| `CODE_STYLE_RUNTIME_TEST_HELPER_*` | 5 files | Deterministic test helpers allowed to use test-only panic/fixture construction behavior. |
| `CODE_STYLE_RUNTIME_ARC_OWNER_*` | 7 files | Runtime owners where `Arc` represents real cross-task or cross-thread sharing. |
| `CODE_STYLE_LEPTOS_PRELUDE_*` | 55 files | Exact UI owners allowed to import `leptos::prelude::{...}` because view macros require traits in lexical scope. |
| `CODE_STYLE_SINGLE_SOURCE_OWNER_*` | 6 files | Canonical owners of bounded reads, SQL identifiers, PostgreSQL classification, process commands, string constants, and scaffold templates. |

Definitions and per-entry reasons are in
[`constants_str/src/lib.rs`](constants_str/src/lib.rs). Every non-empty path inventory is checked
for equal path/reason counts, uniqueness, a non-empty reason, and a current target by
`retained_path_exception_inventories_are_exact_justified_unique_and_current`.

### Test-only crates

- `generate_pg_table_test`
- `generate_pg_types_test`
- `generate_where_filters_test`
- `location_test`
- `tests`
- `workspace_test_runner`

### Non-source directory boundaries

- `location_lib_location_test/src` — macro fixture with raw `Vec` fields required by its input contract.
- `pg_crud_common/benches` — benchmark-only code outside the production domain API.
- `tests/src/domain_type_policy_fixture.rs` — analyzer fixture, not production domain code.

### Import exception

Private imports and all public reexports are otherwise rejected. The only private-import exception
is a grouped import rooted exactly at `leptos::prelude` in one of the 55 reviewed frontend owner
files. Files declaring child modules and nested owner modules do not receive a general exception.
The owner list and individual reasons are `CODE_STYLE_LEPTOS_PRELUDE_SUFFIXES` and
`CODE_STYLE_LEPTOS_PRELUDE_REASONS`.

## Inline reviewed inventories

These exceptions live beside the policy that consumes them because their matching data is more
specific than a reusable path list.

| Policy test | Reviewed exception or baseline |
| --- | --- |
| `library_crates_with_public_logic_own_tests` | Macro/generator and narrowly scoped support crates whose behavior is exercised by downstream or generated tests. Each record contains a crate name and reason. |
| `expect_and_panic_messages_start_with_unique_diagnostic_ids` | Four generated diagnostic-message interpolations whose identifiers are supplied by their generators or fixture catalogs. |
| `unit_tests_use_deterministic_time_and_randomness_patterns` | One reviewed `Instant::now` owner used for the runtime measurement test. |
| `process_static_state_matches_reviewed_inventory` | Eight exact `(path, static identifier, reason)` records, including the cached module-declaration graph. |
| `library_print_macros_have_reviewed_terminal_owners` | Five exact terminal/process-boundary owner paths. |
| `large_module_exceptions_are_exact_and_still_needed` | Two production modules currently above the responsibility line limit. The test rejects missing or stale entries. |
| `allocations_inside_loops_match_reviewed_inventory` | Exact allocation sites that remain accepted inside loops. |
| `contract_public_api_matches_reviewed_snapshot` | Reviewed public API snapshots for contract crates. |
| `arc_lock_and_trait_object_usage_matches_reviewed_inventory` | Exact `Arc`, lock, and trait-object occurrences with expected counts and reasons. |
| `ignored_map_err_bindings_match_reviewed_inventory` | Exact ignored `map_err` binding occurrences with expected counts and reasons. |
| `raw_vec_tuple_wrappers_match_reviewed_inventory` | Exact raw-`Vec` tuple wrappers retained at reviewed boundaries. |
| `usize_max_usage_matches_reviewed_inventory` | Exact owners of `usize::MAX` expressions. |
| `select_sites_match_reviewed_cancellation_inventory` | Exact `select!` owners, occurrence counts, and cancellation-safety reasons. |
| `substantial_function_bodies_have_one_source_of_truth` | Reviewed groups of intentionally similar function bodies with extraction reasons. |

The inline inventories are defined in:

- [`tests/src/code_style_cargo_policy.rs`](tests/src/code_style_cargo_policy.rs)
- [`tests/src/code_style_source_policy.rs`](tests/src/code_style_source_policy.rs)
- [`tests/src/code_style_module_policy.rs`](tests/src/code_style_module_policy.rs)
- [`tests/src/code_style_advanced_policy.rs`](tests/src/code_style_advanced_policy.rs)
- [`tests/src/code_style_reuse_policy.rs`](tests/src/code_style_reuse_policy.rs)

## Compiler-lint synchronization exceptions

The Clippy synchronization list is empty. Rust lint synchronization temporarily excludes these 11
unstable lints; the test probes each lint and fails when an entry becomes supported or loses its
reason:

- `implicit_provenance_casts`
- `multiple_supertrait_upcastable`
- `must_not_suspend`
- `non_exhaustive_omitted_patterns`
- `default_overrides_default_fields`
- the synthetic unstable-lint probe fixture
- `resolving_to_items_shadowing_supertrait_items`
- `shadowing_supertrait_items`
- `unqualified_local_imports`
- `deprecated_llvm_intrinsic`
- `tail_call_track_caller`

The authoritative list is in
[`tests/src/code_style_lint_sync.rs`](tests/src/code_style_lint_sync.rs).

## Syntax and generated-code boundaries

The string-constant policy permits string literals only at explicitly recognized syntax boundaries,
including assertion/diagnostic macros, attributes, documentation, formatting machinery, parsing
fixtures, and generated token streams. These are syntax categories rather than path allowlists.
Their catalog starts at `CODE_STYLE_STRING_LITERAL_MACRO_BOUNDARIES` in
[`constants_str/src/lib.rs`](constants_str/src/lib.rs), and the policy verifies that the only
source-directory exclusion is the constants crate itself.

Generated/test-fixture handling also supplies narrow scope exceptions for analyzer fixtures and
code emitted inside `quote!`; it does not exempt an entire production crate from the policies.

## Explicitly empty exception classes

- Clippy lint synchronization exceptions: empty.
- Whole-file unbounded-read owners: empty.
- Public-reexport exceptions: none.
- General private-import exceptions for crate roots, module-declaring files, or nested modules: none.
- Name-based exceptions for external leaf wrapper types: none.
- Abort or transmute call exceptions: none.

