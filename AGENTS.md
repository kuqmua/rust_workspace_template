## WHAT AGENT MUST DO

- Place shared logic in a dedicated shared crate.
- Add dependencies only when prompt explicitly requests it.
- Disable default features unless required.
- Prefer `std` over external crates.
- Declare crates.io dependencies only in `workspace.dependencies` using concrete crate types, and reference all workspace dependencies as `dep = { workspace = true }` in workspace projects.
- Prefer borrowing over cloning, especially for large structures.
- Prefer immutable data.
- Avoid memory leaks via static state.
- Use enums and `thiserror` for errors.
- Use repository domain wrapper types in struct fields, enum fields, function parameters, method parameters, and return values; initialize raw values through `From` or `TryFrom`.
- Use a single async runtime across workspace; do not mix async runtimes.
- Keep trait bounds explicit.
- Use trait objects only when dynamic dispatch is required.
- Add unit tests for public logic.
- Use test helpers for repeated setup.
- Keep tests deterministic.
- If error message contains 8 random symbols then search workspace for that id.
- Avoid allocations inside hot loops.
- Preserve behavior and semantics unless a change is requested; never change semantics silently.
- Keep diffs minimal.
- Keep generated functions and closures inside usage scope.
- Prefer explicit paths at usage sites over `use` imports.
- `expect()` messages must contain the **first 8 symbols from a random UUID v4**.
- Use abbreviations when creating names.

## WHAT AGENT MUST NOT DO

- Merge unrelated crates.
- Edit Cargo.toml of unrelated crates.
- Add new crates unless explicitly requested.
- Silence clippy without justification.
- Leave commented dead code.
- Commit debug prints.
- Use import or re-export aliases with `as`, including `use ... as ...` and `pub use ... as ...`; use the original item name or rename the item at its definition when a rename is explicitly required.
- Create type aliases with `type`; use explicit types at usage sites.
- Expose primitive or external crate types in domain boundaries when a repository domain wrapper type can be used instead.
- Use `unwrap()`.
- Use `expect()` or `panic!()` in library code except in `proc-macro`, tests, or generated test code inside `quote!`.
- Ignore `Result` or swallow errors.
- Use or write `unsafe`.
- Assume `Send` or `Sync` without proof.
- Use outdated versions in case of adding new crate.
- Block async executors.
- Hold locks across `.await`.
- Ignore cancellation safety.
- Depend on external services in tests.
- Use flaky time-based tests.
- Use `include_str!()` or `include_bytes!()` outside explicit generated/test fixture allowlist.
- Leak generics to users.
- Refactor or reformat without request.
- Rename public items casually.

## Review-only rules

These rules require code review judgment and are not fully proven by automated tests:

- Do not break architecture boundaries or introduce hidden coupling.
- Keep public API minimal and do not change it without instruction.
- Use `Arc` only when semantically required for cross-thread sharing.
- Use `Mutex` only when semantically required for interior mutability.

## Run before completion

```bash
cargo fmt
```

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

```bash
cargo test -p tests_code_style
```

```bash
cargo test --workspace --exclude tests_code_style
```

Run database-backed ignored tests only in a provisioned environment:

```bash
cargo run -p workspace_test_runner -- database
```

## Toolchain note

- This repository is intended for the latest Rust nightly toolchain.
