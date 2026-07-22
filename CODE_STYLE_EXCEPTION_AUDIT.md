# Code-Style Test Exception Audit

This document inventories explicit exclusions, allowlists, reviewed baselines, and silent coverage gaps in the code-style tests under `tests/src/code_style`.

Status meanings:

- **Remove now**: the exception can be removed immediately or after a small local correction.
- **Remove after refactoring**: the exception is technical debt, but removing it requires a non-trivial migration.
- **Keep**: the exception matches the intended scope of the rule or protects a necessary architectural boundary.

## Domain Type Policy

The path filter is implemented in `tests/src/code_style/mod.rs` by `domain_type_policy_should_check_path`.

| Exception | Status | Assessment |
| --- | --- | --- |
| `workspace_test_runner/src` | Remove after refactoring | The entire crate is excluded. Its public and internal boundaries should be migrated to repository wrapper types. |
| `workspace_scaffold/src` | Remove after refactoring | The generator uses raw strings and external syntax-tree types extensively. The exclusion is technical rather than fundamental. |
| Every `benches/` directory | Keep | Benchmarks are not production domain APIs. |
| The `location_test` crate | Remove now | It is a test-only crate, but it can still follow the domain boundary policy with a small fixture refactor. |
| `server_admin_frontend/src/app.rs` and `server_admin_frontend/src/app/*` | Remove after refactoring | This broad exclusion hides browser strings, `web_sys` types, and raw UI state boundaries. These types can be wrapped incrementally. |
| Every proc-macro crate | Remove after refactoring | Proc-macro implementations naturally use `syn`, `quote`, and token-stream types, but those boundaries can still be wrapped. |
| `tests/src/code_style` | Keep | The meta-harness implements the analyzer itself. Excluding it prevents recursive application of the policy to its implementation machinery. |
| Items under `cfg(test)` | Keep | These items are test implementation details rather than production domain boundaries. |

The former exclusion for `initialize_environment_files/src` has already been removed. The crate is now covered by the policy.

## Runtime Policy

Runtime source selection is implemented by `is_runtime_policy_source_path` in `tests/src/code_style/mod.rs`. `Arc` exceptions are configured in `tests/src/code_style/runtime_policy.rs`.

| Exception | Status | Assessment |
| --- | --- | --- |
| Files named `test_hlp.rs` | Keep | These are test helpers. |
| Files outside a `src/` directory | Keep | Runtime rules intentionally target production source files. |
| `initialize_environment_files/src` | Remove now | The crate has been migrated to wrapper types and should no longer need this runtime-policy exclusion. |
| Proc-macro crates | Keep | Repository policy explicitly permits `panic!` and `expect` in proc-macro code. |
| Crates whose package name contains or ends with `test` | Keep, but narrow | Excluding test-only crates is correct, but substring matching is broad. Cargo target metadata would be safer. |
| Items under `cfg(test)` | Keep | They are not production runtime code. |
| `Arc` in `server/src/main.rs` | Keep | Application state is shared across threads. |
| `Arc` in `server_admin/src/password.rs` | Keep | Password hashing concurrency state is shared across tasks. |
| `Arc` in `server_runtime/src/bounded_read.rs` | Keep | The resource budget is shared state. |
| `Arc` in `server_runtime/src/limits.rs` | Keep | Tokio semaphore ownership is shared across tasks. |

The `Arc` exceptions are semantically justified, but they currently permit a whole file. They should eventually be narrowed to specific structures or expressions.

## Direct Environment and Filesystem Access

The rule in `tests/src/code_style/source_policy.rs` excludes the following owners:

- `config_lib`
- `macro_clippy_check_common`
- `macros_helpers`
- `tests`
- `workspace_test_runner`
- `workspace_scaffold`
- `initialize_environment_files`
- `file_storage`
- `server_runtime/src/bounded_read.rs`
- `server_admin_frontend/src/lib.rs`

| Category | Status | Assessment |
| --- | --- | --- |
| Configuration, file-storage, initializer, and bounded-reader owners | Keep | Direct environment or filesystem access is their architectural responsibility. |
| Macro and workspace tooling | Remove after refactoring | They can be routed through shared filesystem and command abstractions, but this requires broader changes. |
| The broad `tests` substring exclusion | Remove now | Replace it with exact test-only paths or Cargo target metadata. |
| `server_admin_frontend/src/lib.rs` | Remove after review | Verify its asset-loading boundary and narrow the exception to the exact operation that owns it. |

## Bounded Read Policy

The unbounded-read rule excludes:

- `tests`
- `macros_helpers`
- `macro_clippy_check_common`
- `workspace_test_runner`
- `workspace_scaffold`
- `initialize_environment_files`
- `server_runtime/src/bounded_read.rs`
- `server_admin_frontend/src/lib.rs`

| Exception | Status | Assessment |
| --- | --- | --- |
| `server_runtime/src/bounded_read.rs` | Keep | This file implements the safe bounded-read abstraction. |
| Test and local tooling crates | Remove after refactoring | Trusted local input reduces risk, but these crates can still reuse the bounded helper. |
| `initialize_environment_files` | Remove after refactoring | Environment files and manifests are currently read without an explicit size bound. |
| `server_admin_frontend/src/lib.rs` | Remove after review | The exception should be narrowed or removed after its asset-loading behavior is verified. |

## Public Struct Field Allowlist

`CODE_STYLE_REVIEWED_PUBLIC_FIELD_PATH_PARTS` in `str_constants/src/lib.rs` excludes these path groups from the private-field rule:

- `config_lib/config_lib_macros/`
- `git_info/`
- `location_lib/location_test/`
- `location_lib/src/location.rs`
- `macros_helpers/`
- `pg_crud/`
- `route_validators/src/test_hlp.rs`
- `route_validators/src/hdr_val.rs`
- `server_admin/src/generated_tables.rs`
- `server_app_state/`
- `server_config/`
- `tests/src/code_style/`
- `to_err_string/`
- `workspace_test_runner/`

| Category | Status | Assessment |
| --- | --- | --- |
| Test helpers and the code-style meta-harness | Keep | Public fields are acceptable in local fixtures and analyzer state. |
| Production and generator directories | Remove after refactoring | Privatize fields and add minimal constructors or accessors. |

The current path-based allowlist is too broad. It should eventually identify exact `Struct::field` entries instead of whole directories.

## Non-Public `use` Import Allowlist

The rule in `tests/src/code_style/source_policy.rs` excludes these files completely:

- `server_admin_frontend/src/app.rs`
- `server_admin_frontend/src/ssr.rs`
- `server_admin_frontend/src/app/forms.rs`
- `server_admin_frontend/src/app/tables.rs`
- `server_admin_frontend/src/app/pages.rs`
- `frontend_contract/src/lib.rs`
- `pg_crud/pg_crud_common/src/lib.rs`
- `pg_crud/pg_table/generate_pg_table_src/src/lib.rs`
- `pg_crud/pg_types/generate_pg_types_src/src/lib.rs`
- `pg_crud/where_filters/generate_where_filters_src/src/lib.rs`
- `server_admin/src/lib.rs`
- `server_runtime/src/lib.rs`

| Category | Status | Assessment |
| --- | --- | --- |
| Facade `lib.rs` files containing intentional `pub use` declarations | Keep, but narrow | Public re-exports are architectural boundaries. The analyzer should permit only `pub use`, not skip the entire file. |
| Frontend implementation files | Remove after refactoring | Ordinary private imports can be replaced with explicit paths. |
| Generator implementation files | Remove after refactoring | Explicit paths are possible but require a substantial mechanical rewrite. |

## SQL and PostgreSQL Policy

| Exception | Status | Assessment |
| --- | --- | --- |
| Test files in the raw SQL identifier inventory | Keep | Test SQL is outside the production ownership rule. |
| `workspace_scaffold` SQL templates | Keep | The crate owns generated source templates. |
| `pg_crud_common/src/sql_identifier.rs` | Keep | This is the canonical SQL identifier abstraction. |
| Six reviewed matches in `str_constants/src/lib.rs` | Remove after review | SQL fragments are expected there, but the baseline should be reduced if typed ownership can replace them. |
| `pg_crud_common/src/pg_error.rs` | Keep | This is the canonical PostgreSQL error classifier. |
| SQLSTATE values in `str_constants` | Keep | The constants crate owns reusable wire-level constants. |
| PostgreSQL classification in tests | Keep | Test assertions are outside the production centralization rule. |

## Process Creation, Abort, and Test Nondeterminism

| Exception | Status | Assessment |
| --- | --- | --- |
| `Command::new` in `macros_helpers/src/tool_command.rs` | Keep | This is the dedicated command-construction owner. |
| `process::abort` in `macros_helpers/src/panic_if_err.rs` | Remove after refactoring | Removal requires a new fatal-error strategy for macro tooling. |
| `process::abort` in `pg_crud/where_filters/src/lib.rs` | Remove after refactoring | Removal requires changing fatal generator error handling. |
| `tokio::time::sleep` in server-runtime health tests | Remove now | Use paused or controlled Tokio time. |
| `Uuid::new_v4` in `pg_crud_common` tests | Remove now | Use fixed deterministic UUID values. |

## String Policy Exceptions

| Exception | Status | Assessment |
| --- | --- | --- |
| `tests/src/code_style` in duplicate-string checks | Keep | Analyzer fixtures intentionally repeat syntax and diagnostic fragments. |
| `tests/src/lib.rs` in duplicate-string checks | Remove after review | The complete-file exclusion is broader than necessary. |
| `workspace_scaffold` in production-string checks | Remove after refactoring | Generated templates can use centralized fragments, but readability and generator ownership must be preserved. |
| `str_constants` in the string-constant ownership rule | Keep | It is the canonical owner. |
| `workspace_scaffold` in the string-constant ownership rule | Remove after refactoring | The exception is needed only while generated templates embed constants directly. |

## External Wrapper Naming Exception

`GeneratedRustTokenStream` is the only explicit exception to the rule requiring an external crate prefix on wrappers around external types.

**Status: Remove after an explicit API migration.** It can be renamed, but it is already a public generator API used across crates. It should not be renamed casually.

## Lint Synchronization Exceptions

### Clippy lint exceptions

`CODE_STYLE_CLIPPY_LINT_EXCEPTIONS` contains 22 entries:

- `disallowed_fields`
- `unnecessary_trailing_comma`
- `manual_pop_if`
- `assign_ops`
- `extend_from_slice`
- `match_on_vec_items`
- `misaligned_transmute`
- `option_map_or_err_ok`
- `pub_enum_variant_names`
- `range_step_by_zero`
- `regex_macro`
- `replace_consts`
- `should_assert_eq`
- `string_to_string`
- `unsafe_vector_initialization`
- `unstable_as_mut_slice`
- `unstable_as_slice`
- `unused_collect`
- `wrong_pub_self_convention`
- `manual_noop_waker`
- `manual_option_zip`
- `useless_borrows_in_formatting`

**Status: Keep conditionally.** These entries are required while the installed Clippy does not expose the corresponding lint. Remove each entry as soon as the active nightly supports it.

### Rust lint exceptions

The Rust lint exception list contains unstable or not-yet-connected lints, including provenance-cast, supertrait-shadowing, `must_not_suspend`, `non_exhaustive_omitted_patterns`, `unqualified_local_imports`, linker, duplicate-feature, LLVM intrinsic, and tail-call diagnostics.

**Status: Keep conditionally**, except that the duplicate `default_overrides_default_fields` entry can be removed now. The remaining entries should be probed and pruned automatically when the nightly toolchain changes.

## Silent Source Snapshot Exclusions

`tests/src/code_style/snapshot.rs` currently uses:

- `filter_map(Result::ok)` for `WalkDir` failures;
- `.ok()?` for `read_to_string` failures;
- `.ok()?` for source-text conversion failures.

**Status: Remove now.** A file that cannot be walked, read, or converted silently disappears from every code-style test. Snapshot construction should fail with a path-specific diagnostic instead.

## Workspace Dependency Path Exception

The Cargo policy permits `path = ...` for workspace-owned dependencies while requiring third-party dependencies to be declared in `workspace.dependencies`.

**Status: Keep.** Local workspace crates necessarily use repository paths at the workspace ownership boundary.

## Recommended Removal Order

1. Make source snapshot construction fail instead of silently skipping files.
2. Remove the runtime-policy exclusion for `initialize_environment_files`.
3. Replace broad test-path substring checks with exact Cargo target classification.
4. Bring `server_admin_frontend` into the domain type policy.
5. Replace whole-file `use` exclusions with an exact `pub use` allowance.
6. Replace the public-field path allowlist with exact field entries, then privatize production fields.
7. Bring `workspace_test_runner` and `workspace_scaffold` into the domain type policy.
8. Automatically probe and prune lint synchronization exceptions on each nightly update.
