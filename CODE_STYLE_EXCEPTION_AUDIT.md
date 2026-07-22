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

- [ ] Replace `filter_map(Result::ok)` for `WalkDir` entries with error propagation that reports the affected path.
- [ ] Replace `.ok()?` around `std::fs::read_to_string` with a path-specific failure.
- [ ] Replace `.ok()?` around source-text conversion with an explicit diagnostic.
- [ ] Add tests for directory traversal, read, and source conversion failures where deterministic fixtures are practical.
- [ ] Verify that an unreadable or invalid source file cannot silently disappear from the snapshot.

Acceptance checks:

- [ ] `cargo test -p tests code_style::source_policy`
- [ ] `cargo test -p tests code_style`

### 1.2 Replace substring-based test crate detection

Current exception source: `is_test_crate` in `tests/src/code_style/mod.rs`.

- [ ] Replace package-name checks such as `contains("test")` with Cargo target metadata or an exact reviewed test-crate inventory.
- [ ] Add a fixture proving that a production crate whose name contains `test` is still checked.
- [ ] Add a fixture proving that a genuine test-only target remains excluded from runtime-only rules.

Acceptance checks:

- [ ] Runtime policy includes every production crate.
- [ ] Runtime policy excludes only exact test-only targets.

## Phase 2: Remove Immediate and Narrowable Exceptions

### 2.1 Finish `initialize_environment_files` policy coverage

Current state: domain type coverage is complete, but runtime and bounded-read exclusions remain.

- [x] Remove `initialize_environment_files/src` from `domain_type_policy_should_check_path`.
- [x] Replace raw `&str`, `bool`, `Vec<String>`, `Path`, external error, and related boundaries with local wrappers.
- [x] Use `TryFrom<String>` with validation for string wrapper types.
- [x] Add `environment_initializer_is_in_domain_boundary_policy_scope`.
- [ ] Remove `initialize_environment_files/src` from `is_runtime_policy_source_path`.
- [ ] Fix any runtime `expect`, `unwrap`, `panic`, lock, or async violations revealed by enabling the rule.
- [ ] Add explicit size limits for manifest and environment file reads.
- [ ] Remove `initialize_environment_files` from the unbounded-read exclusion.

Acceptance checks:

- [x] `cargo test -p initialize_environment_files`
- [x] `cargo test -p tests code_style::domain_type_policy`
- [ ] A regression test proves runtime-policy inclusion.
- [ ] A regression test proves bounded-read-policy inclusion.

### 2.2 Remove deterministic-test exceptions

Current exception source: `unit_tests_use_deterministic_time_and_randomness_patterns` in `tests/src/code_style/source_policy.rs`.

- [ ] Replace `tokio::time::sleep` in server-runtime health tests with paused or controlled Tokio time.
- [ ] Remove the reviewed sleep exception for `server_runtime/src/health.rs`.
- [ ] Replace `Uuid::new_v4` in `pg_crud_common` tests with deterministic UUID values.
- [ ] Remove the reviewed UUID exception for `pg_crud/pg_crud_common/src/lib.rs`.
- [ ] Add negative fixtures proving that these patterns are rejected without a reviewed owner.

Acceptance checks:

- [ ] Repeated test runs produce identical values and timing behavior.
- [ ] No nondeterminism exception remains for these two paths.

### 2.3 Remove duplicate lint exception entries

Current exception source: `tests/src/code_style/lint_sync.rs`.

- [ ] Remove the duplicate `default_overrides_default_fields` Rust lint entry.
- [ ] Add or extend a test that rejects duplicate lint exception names.

Acceptance checks:

- [ ] Both lint synchronization tests pass.

## Phase 3: Narrow File- and Directory-Wide Source Policy Exceptions

### 3.1 Replace the broad filesystem-access allowlist

Current exception source: `direct_environment_and_filesystem_access_stays_at_owned_boundaries` in `tests/src/code_style/source_policy.rs`.

Current broad owners include `config_lib`, `macro_clippy_check_common`, `macros_helpers`, `tests`, `workspace_test_runner`, `workspace_scaffold`, `initialize_environment_files`, `file_storage`, `server_runtime/src/bounded_read.rs`, and `server_admin_frontend/src/lib.rs`.

- [ ] Replace `path.contains("tests")` with exact test target classification.
- [ ] Replace crate- or directory-wide exemptions with exact owner modules or functions.
- [ ] Review `server_admin_frontend/src/lib.rs` and isolate the exact asset/filesystem boundary.
- [ ] Route non-owner tooling filesystem access through a dedicated shared abstraction where semantics permit it.
- [ ] Keep exact exclusions only for configuration loading, file storage, environment initialization, and bounded-read ownership.
- [ ] Add a regression fixture showing that unrelated code inside an owner crate is still checked.

Acceptance checks:

- [ ] No exception uses an ambiguous `contains` path match.
- [ ] Every retained filesystem exception names an exact path and owner responsibility.

### 3.2 Replace the broad bounded-read allowlist

Current exception source: `runtime_data_reads_are_bounded` in `tests/src/code_style/source_policy.rs`.

- [ ] Replace broad exclusions for tests and tooling with exact target or function ownership.
- [ ] Migrate `macros_helpers` reads to the bounded helper where inputs are not compile-time controlled.
- [ ] Migrate `macro_clippy_check_common` reads where relevant.
- [ ] Migrate `workspace_test_runner` reads where relevant.
- [ ] Migrate `workspace_scaffold` reads where relevant.
- [ ] Complete bounded reads in `initialize_environment_files`.
- [ ] Review and narrow `server_admin_frontend/src/lib.rs`.
- [ ] Retain only the implementation owner `server_runtime/src/bounded_read.rs` and exact trusted fixture boundaries.

Acceptance checks:

- [ ] Every production data read has an explicit size budget.
- [ ] Test/tooling exemptions are exact rather than crate-wide.

### 3.3 Replace the non-public `use` file allowlist

Current exception source: `no_non_public_use_imports_in_rust_sources` in `tests/src/code_style/source_policy.rs`.

- [ ] Change the analyzer so intentional facade `pub use` declarations are accepted without skipping the entire file.
- [ ] Remove whole-file exclusions for `frontend_contract/src/lib.rs`, `pg_crud_common/src/lib.rs`, `server_admin/src/lib.rs`, and `server_runtime/src/lib.rs` after the analyzer distinguishes public re-exports.
- [ ] Replace private imports in `server_admin_frontend/src/app.rs` and `server_admin_frontend/src/ssr.rs` with explicit paths.
- [ ] Cover `server_admin_frontend/src/app/forms.rs`, `app/tables.rs`, and `app/pages.rs` if those files exist or are generated in a variant.
- [ ] Replace private imports in the three generator implementation `lib.rs` files.
- [ ] Remove every whole-file path bypass from the rule.
- [ ] Add fixtures for allowed `pub use`, forbidden private `use`, and forbidden `use ... as ...` in the same file.

Acceptance checks:

- [ ] Facade re-exports pass.
- [ ] Private imports fail regardless of file path.
- [ ] Import aliases remain forbidden.

### 3.4 Replace the public-field directory allowlist

Current exception source: `CODE_STYLE_REVIEWED_PUBLIC_FIELD_PATH_PARTS` in `str_constants/src/lib.rs`.

- [ ] Replace path fragments with exact `Struct::field` entries and mandatory reasons.
- [ ] Remove test helpers and analyzer state from the production rule through exact target classification rather than directory fragments.
- [ ] Privatize production fields in `config_lib/config_lib_macros` where possible.
- [ ] Privatize production fields in `git_info` and `location_lib/src/location.rs` where possible.
- [ ] Audit and privatize fields in `macros_helpers` and `pg_crud` incrementally.
- [ ] Privatize fields in `route_validators/src/hdr_val.rs` where API compatibility permits it.
- [ ] Privatize fields in `server_admin/src/generated_tables.rs` at the generator source.
- [ ] Privatize fields in `server_app_state`, `server_config`, `to_err_string`, and `workspace_test_runner` where possible.
- [ ] Add a test requiring every retained exact field exception to include a non-empty reason.
- [ ] Remove `CODE_STYLE_REVIEWED_PUBLIC_FIELD_PATH_PARTS` after the exact inventory reaches zero.

Acceptance checks:

- [ ] No production directory is excluded wholesale.
- [ ] Every remaining public field is part of an explicitly reviewed public API.

## Phase 4: Expand Domain Type Policy Coverage

### 4.1 Cover `location_test`

- [ ] Remove the `location_test` component exclusion from `domain_type_policy_should_check_path`.
- [ ] Migrate raw fixture boundaries to wrappers.
- [ ] Add a regression test proving that the crate is included.

### 4.2 Cover `workspace_test_runner`

- [ ] Inventory all raw parameters, returns, fields, and external types in `workspace_test_runner/src`.
- [ ] Add or reuse domain wrappers in the appropriate shared crate when logic is shared.
- [ ] Migrate boundaries without changing runner behavior.
- [ ] Remove `WORKSPACE_TEST_RUNNER_SRC` from the domain policy exclusion.
- [ ] Add a regression test proving policy inclusion.

### 4.3 Cover `workspace_scaffold`

- [ ] Separate generated-template text from runtime domain boundaries.
- [ ] Wrap raw strings, paths, collections, syntax-tree types, and generator results.
- [ ] Preserve generated output byte-for-byte unless an output change is explicitly reviewed.
- [ ] Remove `WORKSPACE_SCAFFOLD_SRC` from the domain policy exclusion.
- [ ] Add snapshot or golden tests for generated output.
- [ ] Add a regression test proving policy inclusion.

### 4.4 Cover `server_admin_frontend`

- [ ] Inventory raw browser, URL, query, DOM, status, and UI-state boundaries in `app.rs` and `app/*`.
- [ ] Introduce bounded local wrappers and initialize them through `From` or `TryFrom` as appropriate.
- [ ] Avoid wrapping purely structural generic parameters that the policy already supports.
- [ ] Remove the frontend path exclusions from `domain_type_policy_should_check_path`.
- [ ] Add native analyzer fixtures for browser/external types.
- [ ] Run both native and `wasm32-unknown-unknown` clippy checks.

Acceptance checks:

- [ ] CSR behavior is unchanged.
- [ ] SSR/CSR contract tests pass.
- [ ] The frontend is fully included in domain policy.

### 4.5 Cover proc-macro crates where relevant

- [ ] Inventory proc-macro public boundaries separately from internal `syn` traversal details.
- [ ] Wrap public and cross-module token, syntax-tree, diagnostic, and generated-source boundaries.
- [ ] Narrow the exception to unavoidable compiler entry-point signatures if necessary.
- [ ] Remove the blanket proc-macro exclusion when all relevant boundaries are covered.
- [ ] Add fixtures proving that proc-macro helper functions are checked even if compiler entry points are exempt.

Acceptance checks:

- [ ] Only compiler-mandated proc-macro signatures remain exempt.
- [ ] Ordinary helper APIs inside proc-macro crates follow the domain type policy.

## Phase 5: Remove String Policy Exceptions

### 5.1 Narrow duplicate-string exclusions

- [ ] Review `tests/src/lib.rs` and remove its complete-file exclusion.
- [ ] Keep only exact analyzer fixtures that intentionally duplicate diagnostic syntax.
- [ ] Replace path substring checks for `tests/src/code_style` with exact fixture ownership.
- [ ] Add a fixture proving ordinary test modules are checked.

### 5.2 Bring `workspace_scaffold` into string ownership rules

- [ ] Inventory embedded generated-template literals.
- [ ] Move reusable production strings to `str_constants` or dedicated typed template fragments.
- [ ] Preserve generated output.
- [ ] Remove `workspace_scaffold` from the long-production-string exclusion.
- [ ] Remove `workspace_scaffold` from the string-constant ownership exclusion.

Acceptance checks:

- [ ] Generated snapshots are unchanged.
- [ ] No broad scaffold string exception remains.

## Phase 6: Reduce Reviewed Baselines and Exceptional Operations

### 6.1 Reduce the raw SQL identifier baseline

- [ ] Inspect the six reviewed matches in `str_constants/src/lib.rs`.
- [ ] Replace matches with typed SQL ownership where possible.
- [ ] Reduce the expected count after each removal.
- [ ] Retain exact test SQL, scaffold templates, and `sql_identifier.rs` ownership exclusions only where the rule would otherwise produce a false positive.

### 6.2 Remove `process::abort` exceptions

- [ ] Replace abort-based failure handling in `macros_helpers/src/panic_if_err.rs` with structured error propagation where the API permits it.
- [ ] Replace abort-based failure handling in `pg_crud/where_filters/src/lib.rs`.
- [ ] Remove both paths from `expected_abort_suffixes`.
- [ ] Preserve compile-time diagnostics and generated output behavior.

### 6.3 Rename the external wrapper exception

Current exception: `GeneratedRustTokenStream` in `external_leaf_wrapper_name_exceptions`.

- [ ] Choose a name with the required external crate prefix.
- [ ] Perform an explicit public API migration across generator crates.
- [ ] Remove `GeneratedRustTokenStream` from the exception inventory.
- [ ] Add a compile-time regression proving that wrappers over external leaf types require the prefix.

This task must not be performed as an incidental rename because the current type is a shared public API.

## Phase 7: Maintain Lint Exception Inventories

### 7.1 Clippy exceptions

Current source: `CODE_STYLE_CLIPPY_LINT_EXCEPTIONS` in `str_constants/src/lib.rs`.

- [ ] Add a probe that distinguishes unsupported lints from supported-but-unconfigured lints.
- [ ] Fail when an exception becomes supported by the active Clippy but remains in the exception list.
- [ ] Remove supported entries from the current 22-item inventory.
- [ ] Reject duplicate exception names.
- [ ] Require a reason or upstream tracking reference for every remaining exception.

### 7.2 Rust lint exceptions

- [ ] Probe the active nightly for every Rust lint exception.
- [ ] Remove entries as soon as the toolchain supports them.
- [ ] Remove the duplicate `default_overrides_default_fields` entry.
- [ ] Resolve the `unqualified_local_imports` test-flag issue noted in the source comment.
- [ ] Reject duplicate exception names.
- [ ] Require a reason or upstream tracking reference for every remaining exception.

Acceptance checks:

- [ ] Exception inventories contain only unsupported toolchain lints.
- [ ] A nightly upgrade automatically exposes stale exceptions.

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
