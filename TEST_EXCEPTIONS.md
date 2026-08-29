# Test policy exceptions

This document inventories deliberate exceptions and reviewed baselines used by the workspace
policy tests. The Rust declarations linked below remain authoritative; an exception must be
removed there when its reason or owner is no longer valid.

## Reduction status

The centralized inventories contained 113 entries at the start of the reduction pass. They now
contain 11 entries. Inline reviewed inventories are tracked separately because several represent
generated snapshots or occurrence groups rather than one exception per entry.

| Reduction category | Removed |
| --- | ---: |
| Centralized path/type inventories | 102 |
| Duplicate-function reviewed groups | 41 |
| Shared-dispatch, ignored-`map_err`, and `select!` records | 65 |
| `usize::MAX` owners | 14 |
| Generated diagnostic interpolation records | 1 |
| Library test-ownership exceptions | 23 |
| Library print owners | 5 |
| Repository-specific benchmark and test-fixture paths | 2 |
| Allocations inside hot loops | 2 |
| **Total removed** | **255** |

Stale checks were added or strengthened for duplicate functions, direct filesystem owners,
shared-dispatch usage, ignored `map_err` bindings, `usize::MAX` owners, generated diagnostics,
large modules, raw-`Vec` wrappers, and other exact inventories. New observations remain violations
and were not copied into reviewed inventories.

## Central path inventories

| Inventory | Entries | What it permits |
| --- | ---: | --- |
| `CODE_STYLE_CLIPPY_LINT_EXCEPTIONS` | 0 | Clippy lints missing from the workspace lint catalog. No exceptions currently exist. |
| `CODE_STYLE_REVIEWED_PUBLIC_FIELD_*` | 1 owner/type record | Three `SynField` fields consumed by macro generators across crate boundaries. |
| `CODE_STYLE_DIRECT_FS_OWNER_*` | 10 files | Direct environment, filesystem, command-line, or generated-file operations at their owning adapters. |
| Test-only crates | 0 explicit entries | Test crates are recognized by a `test` name segment or the exact plural `tests` package name. |
| `CODE_STYLE_UNBOUNDED_READ_OWNER_SUFFIXES` | 0 | Whole-file owners allowed to perform unbounded reads. No exceptions currently exist. |
| Runtime test-helper paths | 0 | Test-only modules use recognized test filenames, so no dedicated exception inventory remains. |
| Runtime `Arc` paths | 0 explicit entries | Construction is allowed only in a repository wrapper whose name explicitly contains `Arc` or `Shared` and whose field contains `Arc`. |
| Leptos prelude paths | 0 explicit entries | The exact grouped `leptos::prelude::{...}` form is a framework syntax boundary rather than a per-file exception. |
| Duplicate single-source owner inventory | 0 | Canonical owners are enforced directly by their policies; no parallel metadata list remains. |

Definitions and per-entry reasons are in
[`constants_str/src/lib.rs`](constants_str/src/lib.rs). The direct-filesystem inventory checks equal
path/reason counts, current targets, and actual suppressions. The public-field inventory compares
its expected fields with exact current observations.

### Test-only crate convention

Packages named `tests` or containing an underscore-delimited `test` segment are test-only
boundaries. This recognizes the generated validation crates, `location_test`, `tests`, and
`workspace_test_runner` without maintaining a per-package exception list. A production-like name
such as `contest_service` is covered by the policy test and remains in runtime-policy scope.

### Non-source directory boundaries

- Test-only crates are recognized by package-name convention rather than repository paths.
- Any `benches` directory is recognized structurally as a benchmark boundary; no repository path exception is needed.
- Analyzer fixtures under a recognized test crate are covered by the test-crate boundary rather
  than repository-specific path exceptions.

### Import exception

Private imports and all public reexports are otherwise rejected. The only recognized private-import
syntax boundary is a grouped import rooted exactly at `leptos::prelude`, whose traits must be in
lexical scope for view macro expansion. This is checked structurally and no path, crate-root,
module-declaring-file, or nested-module allowlist remains.

## Inline reviewed inventories

These exceptions live beside the policy that consumes them because their matching data is more
specific than a reusable path list.

| Policy test | Reviewed exception or baseline |
| --- | --- |
| `library_crates_with_public_logic_own_tests` | Empty; all library crates with public logic must now own tests. |
| `expect_and_panic_messages_start_with_unique_diagnostic_ids` | Three current generated diagnostic-message interpolations whose identifiers are supplied by their generators or fixture catalogs. |
| `unit_tests_use_deterministic_time_and_randomness_patterns` | One reviewed `Instant::now` owner used for the runtime measurement test. |
| `process_static_state_matches_reviewed_inventory` | Eight exact `(path, static identifier, reason)` records, including the cached module-declaration graph. |
| `library_sources_do_not_use_print_macros` | No reviewed owners remain; library print macros are rejected unconditionally. |
| `large_module_exceptions_are_exact_and_still_needed` | Two production modules currently above the responsibility line limit. The test rejects missing or stale entries. |
| `allocations_inside_loops_match_reviewed_inventory` | Six exact allocation sites that remain accepted inside loops; two hot-loop allocations were removed. |
| `contract_public_api_matches_reviewed_snapshot` | Reviewed public API snapshots for contract crates. |
| `arc_lock_and_trait_object_usage_matches_reviewed_inventory` | Exact current `Arc`, lock, and trait-object occurrences with expected counts and reasons; 27 zero-observation records were removed. |
| `ignored_map_err_bindings_match_reviewed_inventory` | Exact current ignored `map_err` binding occurrences with expected counts and reasons; 35 zero-observation records were removed. |
| `raw_vec_tuple_wrappers_match_reviewed_inventory` | Exact raw-`Vec` tuple wrappers retained at reviewed boundaries. |
| `usize_max_usage_matches_reviewed_inventory` | 2 current bounded-container owners; 14 zero-observation owners were removed. |
| `select_sites_match_reviewed_cancellation_inventory` | Empty after removing all 3 obsolete reviewed owners; current `select!` sites are violations. |
| `substantial_function_bodies_have_one_source_of_truth` | 38 current groups of intentionally similar function bodies with extraction reasons; stale groups are rejected. |

The inline inventories are defined in:

- [`tests_code_style/src/code_style_cargo_policy.rs`](tests_code_style/src/code_style_cargo_policy.rs)
- [`tests_code_style/src/code_style_source_policy.rs`](tests_code_style/src/code_style_source_policy.rs)
- [`tests_code_style/src/code_style_module_policy.rs`](tests_code_style/src/code_style_module_policy.rs)
- [`tests_code_style/src/code_style_advanced_policy.rs`](tests_code_style/src/code_style_advanced_policy.rs)
- [`tests_code_style/src/code_style_reuse_policy.rs`](tests_code_style/src/code_style_reuse_policy.rs)

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
[`tests_code_style/src/code_style_lint_sync.rs`](tests_code_style/src/code_style_lint_sync.rs).

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
