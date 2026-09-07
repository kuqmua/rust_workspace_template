# Rust module style differences

Review date: 2026-09-07. Scope: the current workspace, containing 198 members and 2,919 Rust files under member `src` directories. This is a source-backed comparison of conventions, not a claim that every line has been manually audited. Existing frontend changes were present before the review.

## Policy inconsistencies and review concerns

1. **Test module naming.** `bounded_types/src/bounded_vec.rs:160` uses the canonical `tests` module, but `bounded_types/src/lib.rs:23`, `server_admin/src/lib.rs:328`, and `workspace_test_runner/src/main.rs:74` declare `test_tests`. The scan found 21 source files declaring that exact redundant module name. The policy explicitly forbids it.

   **Status:** complete

   **Completion:** Renamed all 21 external modules to `test_<owner>` (using the sole function name for single-item files), updated root declarations and the reviewed server-app-state test path. All renamed test bodies are byte-identical to their originals. `cargo fmt` and `cargo check --workspace --all-targets --all-features` passed; no external `mod test_tests` declarations remain. The shared classifier now recognizes canonical filenames only when the root declares them under `#[cfg(test)]`; regression coverage prevents feature-gated production helpers from being exempted.

   **Preferred common style and migration:** Use `tests` for small inline test modules and `test_<owner>` for external test files and their root declarations. Rename each external `test_tests.rs` to a meaningful owner-based name, update its `mod` declaration and references, and retain `test_` on every test function. This satisfies both module naming and the filename rule for root test functions; do not rename external files to `tests.rs`.

2. **Bounded text storage.** `workspace_test_runner/src/command_text.rs:9` stores a `BoundedString` with a 16,777,216-byte maximum. `frontend_admin/src/admin_ssr_html.rs:11` stores a raw `String` and checks the same maximum manually in `TryFrom`. Both represent bounded text, but only the former follows the required storage convention.

   **Status:** complete

   **Completion:** `AdminSsrHtml` now owns bounded byte-counted storage. All three focused tests passed, covering limits, UTF-8, empty input, fallback text, and allocation-preserving `String` conversion.

   **Preferred common style and migration:** Store every bounded text wrapper in `BoundedString`, with validation through `TryFrom`. Change `AdminSsrHtml` storage to the same byte-counted 16,777,216-byte bound, retaining the existing public wrapper, `TooLarge` error, text access, ownership conversions, and error-rendering behavior. Use repository derives where their generated API matches that contract; keep a small validating adapter where needed. Verify the exact limit, one byte over it, and multibyte UTF-8 without changing bytes to character counting.

3. **Wrapper coverage in macro helpers.** `proc_macro_newtype_shared/src/lib.rs:793` uses `NewtypeSynDeriveInputRef` and `ProcMacro2GeneratedTokenStream`, while `bounded_string_wrapper` in the same file at line 777 accepts and returns raw `proc_macro2::TokenStream`. That function is in a non-proc-macro shared crate, so it is not the compiler-required proc-macro entrypoint exemption.

   **Status:** complete

   **Completion:** All 41 public newtype helper boundaries now use wrappers; workspace compilation and newtype tests passed. Removed the now-obsolete duplicate-body inventory entry. The compiler-facing conversions use the standard `proc_macro` crate without adding dependencies.

   **Preferred common style and migration:** Use repository input/output wrappers at every shared-helper boundary and raw compiler token streams only at the required proc-macro entrypoint. Convert at that entrypoint, then pass the existing input and generated-token wrappers through the shared implementation. Apply the same convention to helper return types, typed closures, and analyzer fields. Keep generated tokens and compile-error diagnostics unchanged, and explicitly review any shared public signature changes.

4. **Error representation and propagation.** `server_runtime_core/src/resource_budget_reserve_error.rs:10` defines meaningful `thiserror` variants. In contrast, `workspace_test_runner/src/measure_cargo_command.rs:1` and `run_commands.rs:1` return `Result<(), ()>`. The runner even converts an I/O error into `ExecutionIoError` and then reduces it to unit after printing (`run_commands.rs:12`). This prevents callers from inspecting the typed source.

   **Status:** complete

   **Completion:** Implemented typed command, measurement, report, summary, and fixture conversion errors. Command failures remain available when report writing also fails, and CLI adapters render diagnostics. Six deterministic error tests pass alongside the existing runner tests. Full workspace Clippy and all 283 policy tests passed after centralizing fixture diagnostics in a CLI closure; the fixture command produced valid JSON with 11 cases. Ordinary workspace tests, including generated-code checks and documentation tests, passed. Validation logs: `/tmp/style-item-4-clippy-workspace.log`, `/tmp/style-item-4-static-recheck.log`, and `/tmp/style-item-4-workspace.log`.

   **Preferred common style and migration:** Give each fallible operation a meaningful `thiserror` enum with variants describing failures such as starting a command, an unsuccessful command exit, or writing results. Preserve underlying errors through the existing domain/observed-error wrappers and return them to the caller; render diagnostics once at the CLI boundary. Replace the runner unit-error results without changing exit codes, command aggregation, or whether remaining commands execute after a failure. Keep genuinely infallible helpers concrete.

5. **String constants.** `server_runtime_core/src/resource_budget_reserve_error.rs:11` references `constants_str` for error text. `server_admin/src/admin_repository_error.rs:3` embeds error messages directly, and `workspace_test_runner/src/measure_cargo_command.rs:94` embeds a production diagnostic in `eprintln!`. These differ from the required constants catalog and production printing conventions.

   **Status:** complete

   **Completion:** Centralized 54 distinct administrator diagnostic messages across 55 error attributes in 13 modules. Exact template reconstruction matches the original text, and administrator Clippy checks pass. Nine new word fragments are reused; punctuation uses existing fragments. All 283 policy tests passed for the constants migration. Added a shared fallible console writer with typed stream errors and replaced the runner-only execution I/O enum with shared `StdToolIoError` storage in `macro_helpers`. Both measurement footer helpers now propagate output failures while measurement errors retain exit status. Focused Clippy passed after applying the required error-field layout. Both deterministic buffer and broken-pipe tests passed. Policy validation identified the raw shared error boundary; the explicit item-5 architecture change adds exactly `macro_helpers::std_tool_io_error::StdToolIoError` to the struct-error snapshot. This external-leaf wrapper preserves the original standard I/O error and supports typed enum sources; operation errors remain enums. All measurement stdout/stderr paths now use the shared writer, including the pre-command unavailable notice. Twenty catalog segments replace the remaining measurement summary text; all three reconstructed templates match the original output. Focused Clippy and the two writer tests pass. All 16 runner tests and all 283 policy tests passed for the measurement migration. Command output now collects write failures alongside exit or spawn failures, and executor announcements use fallible output. A deterministic aggregation test covers output-error source retention. All 80 CLI output calls now pass through a terminal fallible-output closure, and no production print macros remain in the runner. Sixty-four additional catalog segments centralize 42 diagnostic and log templates; exact reconstruction matches the original text. Focused Clippy passed. CLI smoke checks passed for exact normal output, stdout write failure with a diagnostic and exit code 1, and stderr write failure with exit code 1. Full static checks passed, including all 283 policy tests and workspace Clippy (`/tmp/style-item-5-final-static.log`). Ordinary workspace tests, including generated-code and documentation checks, passed (`/tmp/style-item-5-final-workspace.log`).

   **Preferred common style and migration:** Put diagnostic text and reusable literals in `constants_str`, preserving their exact output. Compose repeated words from the existing word-fragment catalog; add a fragment only when it has at least two references, and keep Rust-source snippets in the Rust-specific blocks. Use tracing for runtime diagnostics and the designated CLI output boundary for command output. Replace production line-print macros with the repository-permitted output mechanism while preserving newlines and propagating write failures.

6. **Generated versus handwritten getters.** `server_runtime_http/src/cleanup_report.rs:1` uses `Getters` with bare accessors. `server_admin/src/admin_generated_token.rs:1` also derives getters, but `server_admin/src/security.rs:204` manually forwards `hash()` and `token()` to generated `get_hash()` and `get_token()`. The latter duplicates mechanical accessor behavior that policy asks generation to own.

   **Status:** complete

   **Completion:** Added opt-in `#[getters(bare, legacy_refs)]` generation and removed the token's handwritten accessor forwarding. Existing `hash`, `token`, `get_hash`, `get_token`, and reference getter names remain available, with const borrowing and the bare methods' `must_use` markers preserved. Added tests for borrowed, optional, and Copy compatibility and the concrete token type. Focused Clippy and getter integration tests passed. The concrete token compatibility test passed. Full static checks passed, including all 283 policy tests and workspace Clippy (`/tmp/style-item-6-static.log`). Ordinary workspace tests, generated-code checks, and documentation tests passed (`/tmp/style-item-6-workspace.log`).

   **Preferred common style and migration:** Use generated, bare, field-named accessors as the canonical call-site style: borrow non-Copy fields and return Copy fields by value. Replace handwritten forwarding methods with equivalent generated methods, retaining handwritten methods only for computed or validated behavior. For `AdminGeneratedToken`, generate `hash()` and `token()` directly. Check the existing public API before removing `get_*` methods; retain compatibility through generation until an explicit API migration permits their removal.

7. **Copy-field accessor configuration.** `server_runtime_http/src/cleanup_report.rs:19` marks each `Copy` field with `#[getters(copy)]`. `server_admin/src/admin_cleanup_report.rs:12` has six fields of the `Copy` type `AdminCleanupRows`, with none of those annotations. It therefore uses a different generated accessor surface and misses the explicit Copy-field convention.

   **Status:** complete

   **Completion:** Applied `copy` to all six fields and enabled generated bare value accessors with legacy reference compatibility. The workspace has no existing accessor calls to migrate. Added a deterministic test for all six values, reference signatures, and the existing total. Focused tests, full workspace Clippy, all 283 policy tests, ordinary workspace tests, and documentation tests passed (`/tmp/style-item-7-static.log`, `/tmp/style-item-7-workspace.log`).

   **Preferred common style and migration:** Mark every Copy field exposed through `Getters` with `#[getters(copy)]`, consistently with item 6. Add the annotations to all six `AdminCleanupReport` fields and migrate callers to the canonical value accessors. Because annotations can change existing generated return types, compare the public API snapshot first and retain generated reference compatibility where required. Do not add Copy or clone non-Copy data just to make accessors look uniform.

8. **Shared implementation ownership.** `workspace_test_runner/src/strip_ansi.rs:5` and `strip_ansi_codes.rs:1` independently implement the same character-state fold: enter escape mode on ESC, exit on `m`, and append other characters outside escape mode. Names, wrappers, and tuple-pattern ordering differ, but the algorithm is duplicated. The shared-logic policy calls for one implementation; existing return wrappers can retain their separate validation behavior.

   **Status:** complete

   **Completion:** The state machine now lives only in `macro_helpers::tool_ansi_chars::ToolAnsiChars`, behind typed borrowed text and iterator-state wrappers. The shared `String` conversion owns preallocation and collection; local adapters retain their distinct bounded validation and error fallbacks. Algorithm cases moved to the shared owner; adapter tests cover length-error behavior. Initial focused tests passed. The policy check identified duplicated adapter allocation; that step now has one shared owner too. Focused Clippy, shared filtering tests, and adapter fallback tests passed after that consolidation. The full static recheck passed, including all 283 policy tests and workspace Clippy (`/tmp/style-item-8-final-static.log`). Ordinary workspace tests, generated-code checks, and documentation tests passed (`/tmp/style-item-8-final-workspace.log`).

   **Preferred common style and migration:** Put the escape-state algorithm in one existing shared tooling crate, preferably `macro_helpers`, which the runner already uses. Expose a typed text boundary and retain short local adapters for `CommandText` and `CleanAnsiText` validation and their existing fallback behavior. Preserve the current handling of ESC, `m`, incomplete escapes, and Unicode; do not silently expand this into a general ANSI parser. Test these cases once at the shared owner and test only wrapper-specific behavior in the adapters. Reuse existing dependencies; do not introduce a new crate for this consolidation.

9. **Proc-macro implementation placement.** `proc_macro_newtype_get_inner/src/lib.rs:2` delegates to a non-proc-macro shared crate. `proc_macro_getters/src/lib.rs:10` keeps parsing and substantial token generation inside its proc-macro entrypoint. Both have one entrypoint, but their implementation ownership differs. Review which generator logic should be reusable and moved to a shared owner; the single-entrypoint rule alone does not prohibit a substantial entrypoint body.

   **Status:** complete

   **Completed:** The requested migration uses the existing `workspace_macro_helpers` owner. A normal-graph check found no path from that owner back to `proc_macro_getters`. The dependency changes are limited to connecting the facade to that owner, moving its numeric-constant dependency to the owner, and retaining facade integration-test dependencies as dev dependencies. No external package or new crate is introduced. Parsing, validation, and emission now live in `generate_private_field_getters` behind the existing token wrapper; the facade only converts and delegates. Focused Clippy and tests passed. Direct comparison against the previous compiled macro matched representative expanded code and four rejection diagnostics with primary locations (`/tmp/getter-owner-comparison-2obbtr13`). Formatting, full Clippy, and all 283 code-style tests passed through the workspace runner (`/tmp/style-item-9-static.log`). Ordinary workspace tests also passed (`/tmp/style-item-9-workspace.log`).

   **Preferred common style and migration:** Keep each proc-macro entrypoint as a thin conversion/delegation adapter and put parsing, validation, and emission in a lower-level non-proc-macro shared owner. For getter generation, prefer the existing `workspace_macro_helpers` after verifying that its normal dependency graph remains acyclic. Do not move it into `proc_macro_newtype_shared`, which already depends on `proc_macro_getters`. Connecting the chosen owner requires an explicitly authorized dependency change under repository policy; this document does not authorize it. Preserve emitted API and diagnostics with existing expansion and compile-failure checks.

10. **Parameter naming.** `workspace_test_runner/src/measure_cargo_command.rs:2` names inputs `measurement_name` and `cargo_args` after their wrapper types. `frontend_contract/src/field_contract.rs:54` instead names two `ContractStr` inputs `name` and `label`. This is understandable role-based naming, but it differs from the supplied type-based naming rule. Multiple inputs of the same type need an explicit convention or distinct domain wrappers rather than a mechanical rename.

   **Status:** complete

   **Completed:** Explicitly migrating the constructor and generated name/label getter types from `ContractStr` to `FieldName` and `FieldLabel` in the existing frontend domain crate. Both wrappers retain `ContractStr` storage and its text behavior; the reviewed public API snapshot will record this intended signature change. No dependency changes are needed. Both constructor call sites now use the role wrappers; focused frontend tests verify text access, display, and conversion back to `ContractStr`. Focused Clippy passed. The snapshot records only the two new wrappers and the intended `FieldContract` signature/field changes. Formatting, full Clippy, and all 283 code-style tests passed through the workspace runner (`/tmp/style-item-10-static.log`). Ordinary workspace tests also passed (`/tmp/style-item-10-workspace.log`).

   **Preferred common style and migration:** Name parameters after their domain types in snake_case, with `value` reserved for the single input of `From` and `TryFrom`. Where identical text types represent different concepts, reuse or introduce distinct role wrappers within the existing domain crate, such as field-name and field-label wrappers, then name parameters after those types. Apply that to the two `ContractStr` inputs rather than inventing numeric suffixes. Review the resulting signature migration explicitly and preserve the underlying text validation and wire representation.

## Other structural variations

11. **Module granularity.** `workspace_test_runner/src/command_run.rs:1` owns one struct; `tests_code_style_rust/src/types.rs:1` groups 49 structs/enums across 515 lines. `server_admin/src/security.rs:1` groups implementations for multiple types, and `workspace_test_runner/src/main.rs:76` concentrates mode dispatch and workflows in a 1,198-line root file. These are materially different organization styles. Root/test exemptions mean size or grouping alone is not proof of a production-owner violation.

   **Status:** on work

   **Progress:** Moving the 49 analyzer types to flat owner modules and the security implementations to their 16 existing owners before restructuring runner workflows. The analyzer module paths change only within the test crate; their type visibility remains `pub(super)`. Security method/type paths remain unchanged. Two cross-owner mutable adapters will use generated access instead of private tuple fields; the password-hasher split-implementation allowance becomes unnecessary. The analyzer/security moves pass focused Clippy. The runner root now keeps wiring and mode dispatch in about 500 lines; fixture and measurement CLI adapters have separate function owners and return `RunnerCliOutcome` after rendering errors. Terminal console failure handling is shared beside the existing writer. Both moved bodies and every quoted literal match the previous source after path/output-adapter substitutions (`/tmp/verify_item_11_cli_move.py`). Two deterministic adapter tests cover mutation of the original collections. A concurrent frontend page-shell change required consuming the existing HTML input when appending it; its markup and signature are preserved. Final validation remains pending.

   **Preferred common style and migration:** Prefer one named owner per flat module, with that owner's implementations in the same file and all external module declarations in the crate root. Split the analyzer type collection into owner-named files, move security implementations beside their types, and keep the runner root focused on wiring and dispatch. Keep single-use private logic in its usage scope when policy requires a closure; put genuinely reused workflows in an existing shared owner. Move large tests to `test_<owner>` files. Treat this as a staged organization change that preserves module visibility and explicitly reviews public path changes.

12. **Constructor style.** `workspace_test_runner/src/command_run.rs:1` derives `New`; `server_runtime_core/src/resource_budget.rs:13` implements `new` manually to create shared atomic state. This difference is justified: that constructor does more than copy its inputs into fields and should not be treated as a trivial-constructor violation.

   **Status:** wait

   **Preferred common style and migration:** Use one decision rule: derive `New` when construction only assigns inputs to fields, use `TryFrom` for validation, and retain a handwritten constructor when initialization performs real work. Keep `ResourceBudget::new` because it initializes shared atomic state. Preserve its sharing semantics and avoid adding a fallible return when initialization is infallible. Constructor spelling need not be identical when behavior differs; the common style is this consistent decision rule.

13. **Import style.** `frontend_admin/src/render_roles.rs:1` imports Leptos extension traits, while its domain types and calls use explicit paths. Core and wrapper modules generally use explicit paths throughout. Trait imports can be necessary for method resolution, so their presence alone is not a policy violation.

   **Status:** wait

   **Preferred common style and migration:** Use explicit paths for domain types, free functions, and ordinary associated calls. Allow only the specific trait imports needed for method syntax or framework-generated code, following module ownership rules; avoid wildcard imports and aliases. In frontend modules, retain the minimal required Leptos traits and use one consistent placement for those imports. Remove an import only after checking that method resolution and generated view code still compile.

14. **Inline versus external test-module enforcement.** `tests_code_style_rust/src/source_analysis.rs:3140` checks the containing module name when it visits a test function, and `visit_item_mod` only tracks the module while visiting that AST node. `tests_code_style_rust/src/code_style.rs:696` visits each source file independently. Consequently, an inline `mod test_tests { ... }` is rejected, but an external `mod test_tests;` has no child functions in that file, and its separately parsed `test_tests.rs` passes the filename-prefix check. This explains how the redundant names in item 1 coexist with a passing policy suite.

   **Status:** wait

   **Preferred common style and migration:** Apply the same test-name predicate to inline modules, external declarations, and external file owners. Reject the exact name `test_tests` at declaration and filename level, and carry containing-module context when walking external test modules so other invalid names cannot bypass the rule. Keep module discovery in the shared snapshot/analysis owner, using the existing crate-root declarations and failing on missing or invalid files. Add deterministic inline/external fixtures for valid `tests` and `test_<owner>` cases, redundant names, and unprefixed names; use this check to enforce the migration in item 1.

## Validation of the original analysis

- `cargo fmt` completed successfully and left the pre-existing worktree diff unchanged.
- `cargo clippy --all-targets --all-features -- -D warnings` completed successfully. Cargo separately reported a future incompatibility in `proc-macro-error2`.
- `cargo run -p workspace_test_runner -- static` completed successfully: all 282 code-style tests passed. The suite ran once through the runner. Its log is `test_results/workspace_test_runner/1788776004940112468-3857013-0/02-cargo-test---locked.log`.
- `cargo test --workspace --exclude tests_code_style_rust` completed successfully, including generated-code fixture checks and documentation tests.
- No database-backed ignored tests were requested or run.

Passing formatting and Clippy checks does not establish conformance to the additional repository policies above.

Implementation is proceeding in numbered order. Status values: `wait`, `on work`, `complete`. Completion notes record the checks performed for each item.

Implementation validation for items 1-3: `cargo fmt`, Clippy, and all 283 policy tests passed through the workspace runner (`/tmp/style-items-1-3-static-verified.log`). The ordinary workspace tests passed (`/tmp/style-items-1-3-workspace.log`). Focused frontend and newtype tests also passed.
