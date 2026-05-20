## WHAT AGENT MUST DO

- Place shared logic in a dedicated shared crate.
- Use workspace-level dependencies.
- Add dependencies only when prompt explicitly requests it.
- Disable default features unless required.
- Prefer `std` over external crates.
- Declare crates.io dependencies only in workspace.dependencies.
- Use dependency.workspace = true for dependencies in workspace projects
- Prefer borrowing over cloning, especially for large structures.
- Use `Arc` only for cross-thread sharing.
- Use `Mutex` only for interior mutability.
- Prefer immutable data.
- Avoid memory leaks via static state.
- Use enums and `thiserror` for errors.
- Use domain-specific error enums per module/route/service boundary.
- Use `#[must_use]` on `Result`, `Option`, and domain return types where ignoring the value can hide failures or behavior.
- Use enums instead of `bool` for domain logic and API contracts, especially when the meaning is domain-specific or unclear at call sites.
- Use a single async runtime across workspace.
- Keep trait bounds explicit.
- Use trait objects only when dynamic dispatch is required.
- Keep public API minimal.
- Default to `pub(crate)` visibility; use `pub` only with explicit external API justification.
- Add unit tests for public logic.
- For public APIs, add both contract tests and round-trip tests.
- Use test helpers for repeated setup.
- Keep tests deterministic.
- For route tests, always reuse the corresponding `call_*_route_client` function (directly or via shared test helpers).
- If error message contains 8 random symbols then search workspace for that id.
- Avoid allocations inside hot loops.
- Use iterator-based style instead of regular loops in Rust code.
- Enforce performance budgets for hot paths and fail CI on benchmark regressions above agreed thresholds.
- Preserve and propagate error sources.
- Preserve behavior unless change is requested.
- If context is ambiguous, state uncertainty explicitly.
- Surface tradeoffs explicitly when multiple valid approaches exist.
- If a simpler approach exists, state it and prefer it by default.
- Push back when warranted if a request introduces unnecessary complexity or risk.
- Write the minimum code that solves the requested problem; avoid speculative additions.
- If an implementation is substantially larger than needed, rewrite it to a simpler smaller version.
- Touch only what is necessary for the task; clean up only changes introduced by the current work.
- Match the existing project style unless an explicit change request says otherwise.
- Define concrete success criteria for the task and iterate until those criteria are verified.
- For multi-step tasks, provide a brief plan in this format:
  1. [Step] -> verify: [check]
  2. [Step] -> verify: [check]
  3. [Step] -> verify: [check]
- Use only limit/offset pagination, even if cursor pagination could be more performant, because cursor pagination significantly increases code complexity.
- In SQL queries, always reuse table and column name constants (`table_names::*`, `COLUMN_*`, `FIELD_*`, `TABLE_*`) instead of hardcoded string literals for schema identifiers.
- For every new or edited SQL query (including idempotency, auth, handlers, models, and tests), add or reuse a shared constant first and then reference it in `format!`.
- Before adding any new string literal (including SQL text), first check existing string constants in the workspace and reuse them when possible; introduce a new constant only when no suitable reusable constant exists.
- Reuse shared error message constants instead of duplicating hardcoded error strings across handlers, models, and tests.
- Reuse shared error message parts (prefixes/suffixes/field fragments) via common constants or builders.
- For cross-cutting concerns like auth, rate limiting, idempotency, or validation, call a reusable function explicitly in each route handler to keep error types visible in the handler signature and avoid hidden control flow.
- Keep generated functions and closures inside usage scope.
- Keep single-use regular code logic inline.
- Use explicit, full names for variables, functions, methods, traits, constants, structs, enums, modules, type aliases, fields, parameters, and database schema names.
- Use explicit full-word prefixes/tokens in constants: `COLUMN_` instead of `COL_`, `PERMISSION_` instead of `PERM_`, `MESSAGE_` instead of `MSG_`, `PASSWORD` instead of `PWD`, `GEOMETRY` instead of `GEOM`, `VIRTUAL_USER` instead of `VU`.
- Use concrete crate types from crates.io in workspace Cargo.toml.
- Use `*CRATE NAME*.workspace = true` for workspace crate dependencies in Cargo.toml.
- Keep validation thresholds local (`let`/local const) when they are used in a single scope, and reuse those local values in error messages.
- Follow `cargo fmt` as the source of formatting truth.
- Prefer imports over absolute paths in type signatures and expressions.
- Prefer implementing automation and utilities in Rust when the same task can be implemented in Rust.
- For mass refactors (regex/sed/perl/global rename), first limit scope to an explicit file list, then review full `git diff` before completion.
- Use `From`/`TryFrom` and explicit bounds checks for numeric conversions.
- When renaming constants, keep external contract string values unchanged (rename Rust identifiers only, not protocol/schema strings).
- Run and pass `cargo hack` feature-matrix checks for each new feature flag and for every PR that changes `Cargo.toml` or `cfg(feature)` usage.
- Prevent hidden breaking changes: run semver checks and update changelog entries for externally visible changes.
- For externally visible API changes, run `cargo-semver-checks` locally before merge (not only in CI).
- Enforce `cargo deny` policy for licenses/sources with unknown registries and unknown git sources denied by default.
- Run `cargo udeps` on nightly on a schedule and before release.
- Require property-based tests (`proptest`) for parsers, validators, and domain invariants.
- Require compile-fail tests (`trybuild`) for critical type-level contracts.
- Require golden/snapshot tests for stable text/JSON CLI or API output contracts.
- In the final report, always list executed verification commands and their outcomes; if any required check was skipped, state it explicitly with reason.
- Prefer `Result` propagation and explicit `expect()` with 8-char id instead of `unwrap_or_default()`/`unwrap_or(...)` where failures could be hidden.
- Delete unused code immediately.

## WHAT AGENT MUST NOT DO

- Merge unrelated crates.
- Break architecture boundaries or introduce hidden coupling.
- Hide confusion or uncertainty when context is ambiguous.
- Edit Cargo.toml of unrelated crates.
- Add new crates unless explicitly requested.
- Silence clippy without justification.
- Use `#[allow(dead_code)]`.
- Leave commented dead code.
- Commit debug prints.
- Use `unwrap()`.
- Use import or re-export aliases with `as`, including `use ... as ...` and `pub use ... as ...`; use the original item name or rename the item at its definition when a rename is explicitly required.
- Use `todo!()`.
- Use `unimplemented!()`.
- Use `panic!()`.
- Use `assert!()`.
- Use `expect()`.
- Use `abort()`.
- Write documentation prose/doc comments unless explicitly requested.
- Use one common error type for all routes/services in an application.
- Use `anyhow::Error` or `Box<dyn Error>` as public library API boundary error types.
- Use `serde_json::Value` in structs or enums.
- Add a crate default feature without explicit RFC-level justification.
- Use `std::env::*` or `std::fs::*` directly in domain logic instead of adapters/abstractions.
- Use `tokio::spawn` or `std::thread::spawn` without explicit error-ownership and cancellation policy.
- Use `Arc<Mutex<_>>` in single-thread scenarios without explicit synchronization justification.
- Allow unbounded collection growth in long-lived structures without limits/eviction policy.
- Ignore `Result` or swallow errors.
- Use or write `unsafe`.
- Use global mutable/singleton state (`static mut`, lazy singletons) without explicit RFC-level justification.
- Assume `Send` or `Sync` without proof.
- Use outdated versions in case of adding new crate.
- Block async executors.
- Hold locks across `.await`.
- Mix async runtimes.
- Ignore cancellation safety.
- Depend on external services in tests.
- Use flaky time-based tests.
- Use `sleep` in tests.
- Depend on wall-clock time or timezone in tests without explicit time injection.
- Change public API without instruction.
- Rename public items casually.
- Change semantics silently.
- Add features beyond what was explicitly requested.
- Add abstractions for single-use code.
- Add flexibility or configurability that was not requested.
- Leak generics to users.
- Use `Makefile` or `Justfile`.
- Centralize all failures into one global shared error type.
- Expose public struct fields in API types without explicit boundary-level justification.
- Build route paths independently in route tests when a client route function exists.
- Add allocations in hot paths unless performance impact is justified in a nearby comment.
- Use `map_err(|_| ...)` and similar source-dropping conversions without explicit justification.
- Use Axum middleware layers (`.layer(from_fn(...))`) for cross-cutting concerns like auth, rate limiting, idempotency, or validation.
- Create a separate function for logic used only once in regular code. Exceptions: route handlers and closely related routing code, middleware code, entrypoint code (`main` and startup wiring), and tests may use single-use helper functions when this clearly improves readability, structure, or reduces duplication.
- Use abbreviations in names for variables, functions, methods, traits, constants, structs, enums, modules, type aliases, fields, parameters, or database schema names.
- Enforce manual formatting rules that conflict with `cargo fmt` output.
- Add empty lines between code lines manually when `cargo fmt` does not produce them.
- Create shell scripts when the same task can be implemented in Rust.
- Add lint `allow` attributes (`#[allow(...)]` or `#![allow(...)]`) to bypass workspace lints, including in tests.
- Add new module-level `#[cfg_attr(...)]` or `#![cfg_attr(...)]` attributes without explicit permission in the prompt.
- Use cursor/keyset pagination.
- Change external contracts without explicit request: environment variable names, HTTP header names, JSON field names, and route paths.
- Use indexing access like `[0]` or `[1]` instead of `first()`/`get()` (with explicit handling), even in tests.
- Mask failures with `unwrap_or_default()`/`unwrap_or(...)` where this can hide errors.
- Keep unused code behind `#[allow(unused_...)]`.

## Run before completion

```bash
cargo fmt
```

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

```bash
cargo test --quiet
```

## Toolchain note

- This repository is intended for the latest Rust nightly toolchain.
