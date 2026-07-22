# Code-Style Exception Removal Plan

This document is the execution plan for removing as many code-style test exceptions as are relevant and technically justified. It tracks completed and remaining work against the current repository state.

## Status Convention

- `[x]` — completed and verified in the current worktree.
- `[ ]` — not completed or not yet verified.
- **Retain** — intentionally not scheduled for removal because it matches the policy's scope or an architectural ownership boundary.

An exception is complete only when its bypass has been removed or narrowed, affected code has been migrated, a regression test covers the new scope, and all repository quality gates pass.

## Progress Summary

- [x] Removed `initialize_environment_files/src` from the domain type policy exclusion.
- [x] Migrated the resulting 21 raw domain boundaries to repository wrapper types.
- [x] Added a regression test proving that `initialize_environment_files/src/main.rs` is checked by the domain boundary policy.
- [ ] Remove or narrow the remaining actionable exceptions in this plan.
- [ ] Run the final completion audit and confirm that every unchecked item is either completed or explicitly moved to the retained-exception section with evidence.

## Phase 1: Eliminate Coverage Gaps

### 1.1 Fail source snapshot construction on errors

Current exception source: `tests/src/code_style/snapshot.rs`.

- [x] Replace `filter_map(Result::ok)` for `WalkDir` entries with error propagation that reports the affected path.
- [x] Replace `.ok()?` around `std::fs::read_to_string` with a path-specific failure.
- [x] Replace `.ok()?` around source-text conversion with an explicit diagnostic.
- [x] Add tests for directory traversal, read, and source conversion failures where deterministic fixtures are practical.
- [x] Verify that an unreadable or invalid source file cannot silently disappear from the snapshot.

Acceptance checks:

- [x] `cargo test -p tests code_style::source_policy`
- [x] `cargo test -p tests code_style`

### 1.2 Replace substring-based test crate detection

Current exception source: `is_test_crate` in `tests/src/code_style/mod.rs`.

- [x] Replace package-name checks such as `contains("test")` with an exact reviewed test-crate inventory and exact `tests` path components.
- [x] Add a fixture proving that a production crate whose name contains `test` is still checked.
- [x] Add a fixture proving that a genuine test-only target remains excluded from runtime-only rules.

Acceptance checks:

- [x] Runtime policy includes every production crate.
- [x] Runtime policy excludes only exact test-only targets.

## Phase 2: Remove Immediate and Narrowable Exceptions

### 2.1 Finish `initialize_environment_files` policy coverage

Current state: domain type coverage is complete, but runtime and bounded-read exclusions remain.

- [x] Remove `initialize_environment_files/src` from `domain_type_policy_should_check_path`.
- [x] Replace raw `&str`, `bool`, `Vec<String>`, `Path`, external error, and related boundaries with local wrappers.
- [x] Use `TryFrom<String>` with validation for string wrapper types.
- [x] Add `environment_initializer_is_in_domain_boundary_policy_scope`.
- [x] Remove `initialize_environment_files/src` from `is_runtime_policy_source_path`.
- [x] Fix any runtime `expect`, `unwrap`, `panic`, lock, or async violations revealed by enabling the rule. No violations were present after inclusion.
- [x] Add explicit 1 MiB size limits for manifest and environment file reads through the shared synchronous bounded reader.
- [x] Remove `initialize_environment_files` from the unbounded-read exclusion.

Acceptance checks:

- [x] `cargo test -p initialize_environment_files`
- [x] `cargo test -p tests code_style::domain_type_policy`
- [x] A regression test proves runtime-policy inclusion.
- [x] A regression test proves bounded-read-policy inclusion, and an initializer test rejects an environment example one byte over its limit.

### 2.2 Remove deterministic-test exceptions

Current exception source: `unit_tests_use_deterministic_time_and_randomness_patterns` in `tests/src/code_style/source_policy.rs`.

- [x] Replace `tokio::time::sleep` in server-runtime health tests with a pending future under paused Tokio time.
- [x] Remove the reviewed sleep exception for `server_runtime/src/health.rs`.
- [x] Replace `Uuid::new_v4` in `pg_crud_common` tests with a deterministic UUID value.
- [x] Remove the reviewed UUID exception for `pg_crud/pg_crud_common/src/lib.rs`.
- [x] Add a negative analyzer fixture proving that Tokio sleep and random UUID calls are rejected in unit tests without a reviewed owner.

Acceptance checks:

- [x] Repeated test runs produce identical values and timing behavior.
- [x] No nondeterminism exception remains for these two paths.

### 2.3 Remove duplicate lint exception entries

Current exception source: `tests/src/code_style/lint_sync.rs`.

- [x] Remove the duplicate `default_overrides_default_fields` Rust lint entry.
- [x] Add tests that reject duplicate Rust and Clippy lint exception names.

Acceptance checks:

- [x] Both lint synchronization tests pass.

## Phase 3: Narrow File- and Directory-Wide Source Policy Exceptions

### 3.1 Replace the broad filesystem-access allowlist

Current exception source: `direct_environment_and_filesystem_access_stays_at_owned_boundaries` in `tests/src/code_style/source_policy.rs`.

Current broad owners include `config_lib`, `macro_clippy_check_common`, `macros_helpers`, `tests`, `workspace_test_runner`, `workspace_scaffold`, `initialize_environment_files`, `file_storage`, `server_runtime/src/bounded_read.rs`, and `server_admin_frontend/src/lib.rs`.

- [x] Replace `path.contains("tests")` with exact test-crate inventory and exact `tests` path-component classification.
- [x] Replace crate- and directory-wide exemptions with an exact nine-file owner inventory plus the bounded-read implementation owner.
- [x] Review `server_admin_frontend/src/lib.rs`; it performs no direct filesystem read and its redundant whole-file exception was removed.
- [x] Route non-owner data reads through the shared bounded reader; retain direct filesystem operations only in the nine exact owner files whose responsibilities require them.
- [x] Keep exact exclusions only for configuration/tooling generation, file storage, environment initialization, scaffold generation, and bounded-read ownership.
- [x] Add `direct_filesystem_owner_inventory_is_exact_justified_and_current`, proving the exact scaffold owner file is allowed while an unrelated sibling in the same crate is checked.

Acceptance checks:

- [x] No direct-filesystem exception uses an ambiguous `contains` path match.
- [x] Every retained direct-filesystem exception names an exact source path and owner responsibility.

### 3.2 Replace the broad bounded-read allowlist

Current exception source: `runtime_data_reads_are_bounded` in `tests/src/code_style/source_policy.rs`.

- [x] Replace broad exclusions for tests and tooling with exact test-target classification and a four-file unbounded-read owner inventory after migrating the initializer.
- [x] Migrate `macros_helpers` reads to the shared bounded helper; generated-file comparison uses the expected output length as its budget and test fixtures use the expected fixture length.
- [x] Migrate `macro_clippy_check_common` workspace-manifest reads to the shared bounded helper with a 1 MiB budget.
- [x] Migrate `workspace_test_runner` generated-stage output reads to the shared bounded helper with a 16 MiB budget.
- [x] Migrate `workspace_scaffold` reads to the shared bounded helper with a 16 MiB budget and add an over-limit regression test.
- [x] Complete bounded reads in `initialize_environment_files`.
- [x] Review `server_admin_frontend/src/lib.rs`; it contains no unbounded data read and its redundant exception was removed.
- [x] Remove every whole-file unbounded-read owner exception; retain only the checked implementation owner `server_runtime/src/bounded_read.rs`, which is protected by `bounded_read_policy_has_no_whole_file_owner_exceptions`.

Acceptance checks:

- [x] Every production data read covered by the analyzer has an explicit size budget; `CODE_STYLE_UNBOUNDED_READ_OWNER_SUFFIXES` is empty.
- [x] Test/tooling exemptions are exact rather than crate-wide.

### 3.3 Replace the non-public `use` file allowlist

Current exception source: `no_non_public_use_imports_in_rust_sources` in `tests/src/code_style/source_policy.rs`.

- [x] Change the analyzer so intentional facade `pub use` declarations are accepted without skipping the entire file.
- [x] Remove whole-file exclusions for `frontend_contract/src/lib.rs`, `pg_crud_common/src/lib.rs`, `server_admin/src/lib.rs`, and `server_runtime/src/lib.rs` after the analyzer distinguishes public re-exports.
- [x] Narrow the required Leptos macro prelude imports in `server_admin_frontend/src/app.rs` and `server_admin_frontend/src/ssr.rs` to an exact `leptos::prelude::{...}` shape; all other private imports remain forbidden.
- [x] Confirm `server_admin_frontend/src/app/forms.rs`, `app/tables.rs`, and `app/pages.rs` are absent in the current workspace; no phantom file bypass remains.
- [x] Verify that the three generator implementation `lib.rs` files contain only intentional public re-exports and check them without whole-file bypasses.
- [x] Remove every whole-file path bypass from the rule.
- [x] Add fixtures for the exact Leptos prelude exception, forbidden private `use`, and public re-export detection. Alias rejection remains covered by the main policy visitor.

Acceptance checks:

- [x] Facade re-exports pass.
- [x] Private imports fail regardless of file path, except for the exact Leptos macro prelude requirement.
- [x] Import aliases remain forbidden.

### 3.4 Replace the public-field directory allowlist

Current exception source: `CODE_STYLE_REVIEWED_PUBLIC_FIELD_PATH_PARTS` in `str_constants/src/lib.rs`.

- [x] Replace path fragments with exact `path + Struct::field` entries and mandatory reasons.
- [x] Remove test helpers and analyzer state from the production rule through exact test-target classification rather than directory fragments.
- [x] Audit `config_lib/config_lib_macros`; no production public struct field remains in the exact inventory.
- [x] Privatize `ProjectGitInfo::commit`, add a borrowing accessor and `From<GitCommitIdRef>`, and migrate downstream construction/access; retain exact `location` wire/proc-macro fields with reasons.
- [x] Audit `macros_helpers` and `pg_crud`; retain only exact cross-crate generator descriptors and generated-query contracts with reasons.
- [x] Audit `route_validators/src/hdr_val.rs`; no production public struct field remains in the exact inventory.
- [x] Audit `server_admin/src/generated_tables.rs`; retain its exact generated database row fields as serialization/query contracts with a stale-entry check.
- [x] Audit `server_app_state`, `server_config`, `to_err_string`, and `workspace_test_runner`; retain exact immutable state/config API fields, confirm no `to_err_string` production field, and classify the runner by exact test-target ownership.
- [x] Add a test requiring every retained exact field exception to include a non-empty reason and rejecting stale entries.
- [x] Remove `CODE_STYLE_REVIEWED_PUBLIC_FIELD_PATH_PARTS`; the replacement inventory is field-exact and self-validating.
- [x] Exempt only the item-exact `REVIEWED_PUBLIC_FIELDS` policy metadata constant from string ownership; its identifiers and reasons remain colocated with and validated by the analyzer.

Acceptance checks:

- [x] No production directory is excluded wholesale by the public-field rule.
- [x] Every remaining public field is part of an explicitly reviewed public API inventory.

## Phase 4: Expand Domain Type Policy Coverage

### 4.1 Cover `location_test`

- [x] Review the `location_test` exclusion against the actual fixture contract.
- [x] Retain the exclusion: three reported `Vec<T>` fields deliberately exercise vector-field proc-macro attributes, and wrapping them would stop testing the required input syntax.
- [ ] Narrow the exclusion to the exact macro fixture item if the domain policy gains item-level path and item context.

### 4.2 Cover `workspace_test_runner`

- [x] Inventory all raw parameters, returns, fields, and external types in `workspace_test_runner/src`; the current analyzer reports 31 boundaries across `main.rs`, `execution.rs`, `reporting.rs`, and `discovery.rs`.
- [x] Migrate all 12 boundaries in `discovery.rs`, `reporting.rs`, and tool availability/mode handling in `main.rs`.
- [x] Add local domain wrappers for runner-only concepts; reuse existing `macros_helpers` tool-command wrappers where logic is shared across crates.
- [x] Migrate the remaining 19 execution/admin-fixture boundaries without changing runner behavior, including bounded command text and fixture strings.
- [x] Remove `WORKSPACE_TEST_RUNNER_SRC` from the domain policy exclusion.
- [x] Add a regression test proving policy inclusion.

### 4.3 Cover `workspace_scaffold`

- [x] Separate generated-template text from runtime domain boundaries with `ScaffoldText`, `ScaffoldTextRef`, path, replacement, boolean, port, and error wrappers.
- [x] Wrap all raw strings, paths, collections, generator results, and external errors reported by the domain analyzer.
- [x] Preserve generated output byte-for-byte; the service scaffold test now compares every generated and modified artifact exactly.
- [x] Remove `WORKSPACE_SCAFFOLD_SRC` from the domain policy exclusion.
- [x] Upgrade `service_scaffold_registers_all_artifacts` to an exact golden test covering manifest, three copied crates, Kubernetes resources, Compose, and generated SQL constants.
- [x] Add `workspace_scaffold_is_in_domain_boundary_policy_scope`; the complete 21-test domain-policy suite passes.

### 4.4 Cover `server_admin_frontend`

- [x] Inventory raw browser, URL, query, DOM, status, and UI-state boundaries in the current frontend sources; full inclusion reported only `mutation_confirmed`'s message and boolean result.
- [x] Introduce `MutationConfirmationMessageRef` and `MutationConfirmed`, initialized through `From`, without changing browser confirmation behavior.
- [x] Leave structural generic parameters unchanged; the policy already accepts their explicit bounds.
- [x] Remove both frontend path exclusions from `domain_type_policy_should_check_path`.
- [x] Add a native analyzer fixture proving that a raw `web_sys::Response` boundary is rejected.
- [x] Run native strict Clippy and `cargo clippy -p server_admin_frontend --target wasm32-unknown-unknown -- -D warnings`; both pass.

Acceptance checks:

- [x] CSR confirmation behavior is unchanged; the wrappers only mediate the existing message and boolean values.
- [x] Seven SSR/frontend contract tests pass and the CSR wasm32 target passes strict Clippy.
- [x] The frontend is fully included in domain policy, protected by `server_admin_frontend_is_in_domain_boundary_policy_scope`.

### 4.5 Cover proc-macro crates where relevant

- [x] Inventory proc-macro boundaries separately by enabling all proc-macro crates; only three internal `Vec` fields in `str_constants_macros` violated the policy.
- [x] Wrap those internal constant, fragment, and part collections; no other helper boundary is excluded.
- [x] Narrow the exception to item-exact functions carrying compiler `#[proc_macro]`, `#[proc_macro_derive]`, or `#[proc_macro_attribute]` attributes.
- [x] Remove the blanket proc-macro crate exclusion from `domain_type_policy_should_check_path`.
- [x] Add `proc_macro_helpers_are_checked_while_compiler_entrypoints_are_exempt`.

Acceptance checks:

- [x] Only compiler-mandated proc-macro signatures remain exempt.
- [x] Ordinary helper APIs inside proc-macro crates follow the domain type policy; the complete 24-test suite passes.

## Phase 5: Remove String Policy Exceptions

### 5.1 Narrow duplicate-string exclusions

- [x] Review `tests/src/lib.rs` and remove its complete-file exclusion; the file contains only the test-module declaration.
- [x] Keep only the exact `tests/src/code_style` analyzer harness boundary that intentionally duplicates diagnostic syntax.
- [x] Replace path substring checks for `tests/src/code_style` with exact path-prefix ownership.
- [x] Add a scope regression proving an ordinary test fixture is checked while the exact code-style analyzer harness remains excluded.

### 5.2 Bring `workspace_scaffold` into string ownership rules

- [x] Inventory embedded generated-template literals by enabling both string policies and resolving the complete diagnostic inventory.
- [x] Move reusable production template markers, paths, command names, and replacement tokens to `str_constants`; keep one-use formatted templates at their generation sites.
- [x] Preserve generated output through exact golden comparisons for every generated artifact.
- [x] Remove `workspace_scaffold` from the long-production-string exclusion.
- [x] Remove `workspace_scaffold` from the string-constant ownership exclusion.

Acceptance checks:

- [x] Generated golden outputs pass unchanged.
- [x] No broad scaffold string exception remains; both full source-policy tests pass with the scaffold included.

## Phase 6: Reduce Reviewed Baselines and Exceptional Operations

### 6.1 Reduce the raw SQL identifier baseline

- [x] Inspect the six reviewed matches in `str_constants/src/lib.rs`; they are definitions/usages of the scanner markers themselves rather than runtime SQL identifier interpolation.
- [x] Replace the numeric six-match baseline with an exact `str_constants/src/lib.rs` owner exclusion and a zero-match production baseline.
- [x] Reduce the expected count from six to zero.
- [x] Retain exact test SQL, scaffold templates, `str_constants`, and `sql_identifier.rs` ownership exclusions only where the textual scanner would otherwise inspect its fixtures or implementation vocabulary.

### 6.2 Remove `process::abort` exceptions

- [x] Remove `panic_if_err`; keep fallible `try_*` APIs and convert generator write failures into explicit `compile_error!` token streams.
- [x] Replace the `RegexRegex` abort path by storing the validated pattern text, retaining `TryFrom<String>` regex validation and using a private marker for the vetted default pattern.
- [x] Remove both reviewed abort paths; the policy now requires the observed process-abort inventory to be empty.
- [x] Preserve successful generated output behavior and improve failure behavior from process termination to compiler diagnostics; generator and where-filter tests pass.

### 6.3 Rename the external wrapper exception

Current exception: `GeneratedRustTokenStream` in `external_leaf_wrapper_name_exceptions`.

- [x] Rename the wrapper to `ProcMacro2GeneratedRustTokenStream`, using the required external crate prefix.
- [x] Perform the explicit public API migration across all generator crates and rename its public module to `proc_macro2_tokens`.
- [x] Remove the former `GeneratedRustTokenStream` special case; the external-wrapper exception inventory is empty.
- [x] Add `external_leaf_wrapper_prefix_rule_has_no_name_exceptions`, which rejects an unprefixed `proc_macro2::TokenStream` wrapper and accepts the prefixed form.

This task must not be performed as an incidental rename because the current type is a shared public API.

## Phase 7: Maintain Lint Exception Inventories

### 7.1 Clippy exceptions

Current source: `CODE_STYLE_CLIPPY_LINT_EXCEPTIONS` in `str_constants/src/lib.rs`.

- [x] Add `probe_lint`, with a regression distinguishing supported Clippy, unstable Rust, and unknown lint dispositions.
- [x] Reject stale Clippy exception names that are absent from the active `clippy-driver -W help` inventory.
- [x] Fail when an exception becomes supported; compile probing is part of the inventory test.
- [x] Probe and remove all remaining six Clippy exceptions; all are supported and already configured as `deny` in workspace lints.
- [x] Remove 16 stale entries absent from the active Clippy inventory, reducing the inventory from 22 entries to 6.
- [x] Reject duplicate exception names.
- [x] Require reasons for retained exceptions; the Clippy exception inventory is now empty.

### 7.2 Rust lint exceptions

- [x] Probe the active nightly for every Rust lint exception with `-D unknown-lints` and require the `Unstable` disposition.
- [x] Reject stale Rust exception names that are absent from the active `rustc -W help` inventory.
- [x] Remove the stale supported `aarch64_softfloat_neon`, `duplicate_features`, `linker_info`, and `unreachable_cfg_select_predicates` entries; all four are already enforced by `[workspace.lints.rust]`.
- [x] Remove the stale renamed/removed `fuzzy_provenance_casts`, `lossy_provenance_casts`, `supertrait_item_shadowing_definition`, and `supertrait_item_shadowing_usage` entries that are absent from the active nightly inventory.
- [x] Make the test fail as soon as any remaining entry becomes supported, so it must be removed on the corresponding toolchain update.
- [x] Remove the duplicate `default_overrides_default_fields` entry.
- [x] Resolve the `unqualified_local_imports` test-flag issue by probing without enabling its feature and classifying the compiler's explicit unstable-lint diagnostic.
- [x] Reject duplicate exception names.
- [x] Require a non-empty adjacent reason or upstream issue for every remaining Rust exception.

Acceptance checks:

- [x] Exception inventories contain only unsupported unstable Rust lints; the Clippy inventory is empty.
- [x] A nightly upgrade automatically exposes stale or newly supported exceptions through help-inventory and compile-probe checks.
- [x] A nightly inventory change automatically exposes renamed or removed exception entries.

## Retained Exceptions and Required Narrowing

The following exclusions match an intentional policy boundary. They are not removal targets, but broad implementations must still be narrowed where noted.

- **Retain:** `cfg(test)` items in production runtime/domain visitors.
- **Retain:** files outside `src/` for production runtime policy.
- **Retain:** `test_hlp.rs` for runtime-only checks.
- **Retain:** benchmark directories for production domain boundaries.
- **Retain:** the `tests/src/code_style` meta-harness exclusion from its own domain boundary rule.
- **Retain:** proc-macro permission for `panic!` and `expect`, as explicitly allowed by repository policy.
- **Retain:** exact `Arc` uses for cross-thread application state, password hashing limits, bounded-read budgets, and Tokio semaphores.
- **Retain:** exact filesystem/environment owners in configuration, file storage, environment initialization, and bounded reading.
- **Retain:** `server_runtime/src/bounded_read.rs` as the bounded-read implementation owner.
- **Retain:** intentional facade `pub use` declarations, but not whole-file private-import exemptions.
- **Retain:** `pg_crud_common/src/sql_identifier.rs` as the SQL identifier owner.
- **Retain:** `pg_crud_common/src/pg_error.rs` as the PostgreSQL error classifier.
- **Retain:** reusable SQLSTATE constants in `str_constants`.
- **Retain:** `macros_helpers/src/tool_command.rs` as the process command-construction owner.
- **Retain:** `str_constants` as the string constant owner.
- **Retain:** workspace dependency `path` entries for local workspace crates.

Required narrowing checks:

- [ ] Every retained exception uses exact paths, items, or target metadata instead of substring matching.
- [ ] Every retained exception has a non-empty reason adjacent to its declaration.
- [ ] A code-style test rejects stale exception entries that no longer match source code.
- [ ] A code-style test rejects exception inventories with duplicate entries.

## Verification Gates for Every Phase

Run these checks before marking any phase complete:

- [ ] `cargo fmt`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test -p tests code_style`
- [ ] Tests for every crate modified by the phase.
- [ ] `git diff --check`

Additional checks when relevant:

- [ ] `cargo clippy -p server_admin_frontend --target wasm32-unknown-unknown -- -D warnings`
- [ ] Generator snapshot or golden tests.
- [ ] Repeated deterministic test execution.
- [ ] API compatibility review for public wrapper renames or field privatization.

## Final Completion Audit

- [ ] Every actionable exception in this document is removed or narrowed.
- [ ] Every retained exception is exact, justified, and protected by a stale-entry test.
- [ ] No filesystem or parse error can silently reduce code-style coverage.
- [ ] No production crate is excluded through ambiguous path or package-name substring matching.
- [ ] No whole production file is skipped when an item-level exception is sufficient.
- [ ] All lint exception entries are unsupported by the active toolchain and contain reasons.
- [ ] Full workspace formatting, clippy, code-style, crate, target, and generator checks pass.
