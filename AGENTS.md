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
- Name function and method parameters after their types, except that the sole input parameter of `From::from` and `TryFrom::try_from` implementations must be named `value`.
- Store bounded text in `bounded_types::bounded_string::BoundedString`; a struct using
  `#[bounded_string]` must not store a raw `String` or derive the removed `proc_macro_newtype::BoundedString`.
- Use a single async runtime across workspace; do not mix async runtimes.
- Keep trait bounds explicit.
- Use trait objects only when dynamic dispatch is required.
- Add unit tests for public logic.
- Use test helpers for repeated setup.
- Keep tests deterministic.
- Prefix every test function and its containing test module with `test_`.
- If error message contains 8 random symbols then search workspace for that id.
- Avoid allocations inside hot loops.
- Preserve behavior and semantics unless a change is requested; never change semantics silently.
- Keep diffs minimal.
- Keep generated functions and closures inside usage scope.
- Define every repeated ASCII word in `constants_str` exactly once in a macro fragment block and
  compose constants from those word fragments. Every declared fragment must contain one word and
  be referenced at least twice.
- Keep constants containing Rust source in the shared `rust_constants` block. Extract every
  repeated Rust syntax substring into the shared `rust_fragments` block and reference each Rust
  fragment at least twice.
- Keep every struct field private, including tuple-struct fields. Expose required reads through
  generated getters such as `proc_macro_getters::Getters` or `proc_macro_newtype::GetInner`, and expose
  construction through validated conversions or generated constructors.
- Derive `proc_macro_new::New` instead of manually implementing a `new` method whose only behavior is
  to initialize `Self` fields directly from its input parameters.
- Rewrite a private production function with exactly one non-test call site as a closure inside
  that call site's function when doing so preserves attributes, async behavior, architecture
  boundaries, domain wrappers, and direct unit-test coverage. Keep an item-scoped, justified
  `clippy::single_call_fn` allowance only when a named function is required.
- Prefer explicit paths at usage sites over `use` imports.
- `expect()` messages must contain the **first 8 symbols from a random UUID v4**.
- Use established, unambiguous abbreviations when creating names; do not invent unclear short
  forms.

## WORKSPACE AND CARGO POLICY

- Name every crate with the workspace vocabulary accepted by the code-style policy; keep each
  package name identical to its directory name.
- Prefix every proc-macro crate with `proc_macro_` and keep exactly one proc-macro entrypoint
  function in its `src/lib.rs`; place reusable implementation logic in a non-proc-macro shared
  crate.
- Set `publish = false`, inherit the workspace package metadata and workspace lints, and use Rust
  edition 2024 in every workspace crate.
- Pin every external `workspace.dependencies` entry to an exact version and set
  `default-features = false`; workspace crates, including target-specific dependency tables, must
  reference catalog entries with inline `{ workspace = true }` tables and must not re-enable
  default features.
- Remove unused entries from `workspace.dependencies` and keep the normal workspace dependency
  graph acyclic.
- Keep workspace members sorted alphabetically, present on disk, and as direct children of the
  workspace root.
- Keep each crate's `src` module layout flat and give a workspace package at most one binary
  target.
- Keep `.env` and `.env.example` keys identical, and keep exactly one tracked server environment
  example.
- Keep every workspace lint allow entry unique and accompanied by a non-empty, specific inline
  reason; keep the repository's variable-lifetime safety lints and `single_call_fn` lint denied.

## SOURCE AND MODULE POLICY

- Write repository source and tracked text in English, with LF line endings, a final newline, and
  no trailing whitespace.
- Do not add Rust comments or duplicate `#[cfg(test)]` attributes; keep debug explanations in
  names, diagnostics, tests, or documentation outside Rust source as appropriate.
- Do not leave empty modules, empty enums, empty function or method bodies, `todo!()`, or
  `unimplemented!()` in source.
- Keep custom type names and free-function names unique across the workspace; method names may
  repeat.
- Use a single underscore between words in module and function names. Avoid unclear short forms in
  module, function, field, and serde names, and use explicit resource names for identifiers.
- Keep external `mod` declarations in crate roots, do not use `#[path]`, and do not bypass module
  ownership with local crate-root imports.
- A module containing one item must match that item's name. A function-only module may contain at
  most one function, and a production module may contain at most one named owner and one bounded
  responsibility.
- Keep production items in production-named modules, move tests out of large production modules,
  and keep every reviewed large-module exception exact and necessary.
- Do not create non-root re-export-only facade modules or private shared modules that forward
  crate-root exports.
- Do not use public re-exports. Restrict private imports according to module ownership, including
  inside declared child modules and `#[cfg(test)]` modules.
- Generate struct field getters, and do not prefix provider-trait methods with `get_`.
- Do not expose multiple struct fields through borrowed positional tuple accessors such as
  `parts(&self)`. Derive `proc_macro_getters::Getters`, use named getters, and mark `Copy` fields
  with `#[getters(copy)]`. Keep `into_parts(self)` only for intentional ownership transfer when
  moving non-`Copy` fields avoids cloning.
- Do not use `for` loops; use iterator operations while preserving early-exit and error semantics.
- Do not use numeric `as` casts or unstable sorting methods (`sort_unstable*` and equivalent
  unstable variants).
- Do not use simple constant aliases, `dbg!`, print macros in library code, or line-print macros in
  production code.
- Give every source-level lint suppression an explicit, specific reason; never suppress a lint for
  an entire module when an item-scoped suppression is sufficient.
- Do not use `std::process::Command` outside the shared tooling owner, and keep `abort` and
  `transmute` calls limited to the reviewed inventory.
- Every workspace struct and enum must derive `OptimalMemoryLayout` unless it is covered by an
  exact reviewed exception.
- Keep `usize::MAX`, raw SQL identifiers, process static state, raw `Vec` tuple wrappers, ignored
  `map_err` bindings, struct-shaped error exceptions, and allocations inside loops aligned with
  their reviewed inventories; do not broaden an inventory silently.
- Centralize production PostgreSQL error classification in its designated shared owner.
- Do not implement `From<Vec<_>>` for repository wrappers.
- Treat code-style snapshot read, parse, and directory-walk failures as test failures; do not
  silently replace missing or invalid snapshot input with defaults.

## DOMAIN TYPE AND GENERATED CODE POLICY

- Apply repository domain-boundary checks to production, frontend, environment initialization,
  workspace scaffolding, workspace test tooling, fixtures, benchmarks, and proc-macro helper code;
  exempt only compiler-required proc-macro entrypoints and explicit framework adapter boundaries.
- In domain boundaries, permit only `Option` and `Result` as transparent containers around
  repository wrappers; check explicit local and closure parameter types as well as item signatures
  and fields.
- Wrap external leaf types in tuple wrappers whose names include the external source name; do not
  create name-based exceptions.
- Do not add intermediate-representation wrappers to domain type modules or put application,
  adapter, repository, or filesystem workflows in domain type modules.
- Analyzer/helper state fields and helper text return values must use repository-declared wrapper
  types rather than raw containers, `String`, or `&str` when a wrapper exists.
- String wrappers must validate through `TryFrom` (directly or through a delegated validator), not
  implement `From<String>`; never implement both `From` and `TryFrom` for the same inner type.
- Deserialize tuple wrappers through their validated `From` or `TryFrom` conversion instead of
  deriving direct deserialization, and never initialize their private fields directly.
- Use the repository proc-macro derives for forwarding tuple-newtype behavior: `FromInner`,
  `IntoInnerFrom`, `IntoIterator`, `Display`, `DisplayConst`, `DerefInner`, `Borrow`, and `NotInner`;
  do not hand-write equivalent passthrough implementations or `into_*` methods.
- Keep generated struct fields private under every visibility spelling, and apply the same source,
  domain-type, diagnostic, randomness, and secrecy policies to tokens emitted with `quote!`.
- Infallible functions must return concrete values rather than `Result` wrappers that cannot fail.

## ERROR, SECRET, AND DIAGNOSTIC POLICY

- Error types must add domain meaning rather than merely wrap another repository error. Derive
  `thiserror::Error`; do not hand-write the `Error` implementation.
- Every fallible typed-route operation must own a distinct error type; route and admin errors must
  not share or wrap a common operation error.
- JSON API error responses must originate from `thiserror` enums. Keep source locations out of
  public API error enums and wrap internal error sources in the repository observed-error type.
- Never interpolate sensitive fields into error messages. Sensitive wrappers must use bounded
  string storage and must not derive or implement unredacted `Debug` or `Display`.
- Store `SecretBox` text in bounded repository string types everywhere, including generated code;
  never use raw `String` as its secret value type.
- Begin every `expect()` and `panic!()` message, including generated messages, with a unique
  lowercase eight-character prefix from a UUID v4, and keep all diagnostic UUIDs unique and valid.
- Preserve error sources in `map_err`; do not discard them with `_` bindings.

## ASYNC, RUNTIME, AND I/O POLICY

- Runtime production code must not use `expect()`, `unwrap()`, `panic!()`, or `Mutex`.
- Use `Arc` in runtime code only for state that is actually shared across threads, and keep Arc,
  lock, and dynamic-dispatch use aligned with its reviewed inventory.
- Do not hold any lock guard across `.await`; explicitly drop a guard before awaiting when lexical
  scope alone does not prove release.
- Async functions must not call blocking executors or synchronous filesystem/network operations.
- Keep direct environment and filesystem access in their designated owner modules, with every
  exception exact, justified, and current.
- Bound runtime reads; do not use synchronous or asynchronous whole-file read helpers, and do not
  create whole-file owner exceptions.
- Retain an owner for every spawned task and supervise retained tasks through completion,
  cancellation, or shutdown; do not discard join handles with bare or ignored bindings.
- At every `select!` site, use only cancellation-safe operations or an explicitly reviewed design,
  and keep the cancellation inventory current.
- Unit tests must not construct HTTP, database, or socket clients. Mark provisioned integration
  tests ignored with an explicit reason.
- Do not use wall-clock time, sleeping, thread scheduling, or randomness in unit tests or generated
  test templates; use deterministic fixtures.
- Do not call explicit leak APIs or introduce equivalent permanently retained allocations.

## ROUTE AND CONTRACT POLICY

- Build admin frontend API URLs from typed routes, service endpoints from shared route registries,
  and administrator CSR page behavior from the page catalog.
- Typed-route registries own request bodies and schema catalogs; generated admin table consumers
  use the shared catalog, and administrator data-table queries come from the typed specification.
- Use snake_case route path segments; do not use kebab-case segments or embed an `/api` prefix in
  registered route paths.
- Keep route contracts serializable and reject wildcard route contracts not supported by the
  typed route policy.
- Generate config-reference accessors through the repository forwarding mechanism instead of
  hand-writing them.
- Preserve the reviewed public contract API snapshot; review intentional API changes explicitly.

## STRING CONSTANT POLICY

- Declare tracing messages and reusable string constants in `constants_str`; do not declare local
  string constants or aliases to exported constants outside its source directory.
- Reuse production string literals and non-policy test literals through the constants catalog,
  including literals inside expressions, nested macros, assertions, and tracing macros.
- Keep typed domain values in their owning domain crates rather than moving them into
  `constants_str`.

## CI AND DEPLOYMENT POLICY

- Keep CI security and quality checks for permissions, `actionlint`, dependency hygiene, coverage,
  container scanning, unused dependencies, Miri, database tests, and browser tests.
- Run the code-style suite exactly once through the workspace test runner and exclude it from the
  runner's ordinary workspace test command.
- Consume the pinned repository Rust toolchain through the repository setup action; do not repeat
  the channel in workflows or Dockerfiles.
- Give every CI job a timeout and pin every external marketplace action to a full commit SHA;
  commented workflow text does not satisfy required commands or actions.
- Keep the application database image identical and pinned across Compose and CI.
- Keep the service catalog as the single source of truth for crates, Compose services,
  Dockerfiles, images, ports, Kubernetes manifests, CI builds, and release builds; every build and
  runtime projection must be represented exactly once.
- Pin every external Dockerfile base image by digest, reject `latest`, and allow named build stages.
- Use registered live and ready health-route paths in Compose and Kubernetes probes.

## REUSE AND ARCHITECTURE POLICY

- Keep substantial function bodies and repeated explicit domain shapes in one shared source of
  truth; identifier renaming does not make duplicate logic distinct, while short mechanical
  adapters may remain local.
- Preserve dependency direction and reject upward dependencies across architecture layers.
- Keep the reviewed public API, exception lists, and policy inventories exact; update them only as
  part of an explicit, justified architecture or API change.

## WHAT AGENT MUST NOT DO

- Merge unrelated crates.
- Edit Cargo.toml of unrelated crates.
- Add new crates unless explicitly requested.
- Silence clippy without justification.
- Leave commented dead code.
- Commit debug prints.
- Use import or re-export aliases with `as`, including `use ... as ...` and `pub use ... as ...`; use the original item name or rename the item at its definition when a rename is explicitly required.
- Create type aliases with `type`; use explicit types at usage sites.
- Define declarative macros with `macro_rules!`; use a proc-macro crate for code generation.
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
- Declare `pub`, `pub(crate)`, `pub(super)`, or `pub(in ...)` visibility on a struct field.

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
cargo test -p tests_code_style_rust
```

```bash
cargo test --workspace --exclude tests_code_style_rust
```

Run database-backed ignored tests only in a provisioned environment:

```bash
cargo run -p workspace_test_runner -- database
```

## Toolchain note

- This repository is intended for the latest Rust nightly toolchain.
