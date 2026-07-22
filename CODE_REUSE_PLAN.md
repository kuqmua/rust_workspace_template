# Code reuse and single-source-of-truth audit

## Completion rule

Every Rust function and method outside `target` is reviewed as one of these projections:

1. **Domain owner** — owns a rule with no competing implementation.
2. **Shared projection** — delegates to a domain owner or shared helper.
3. **Generated projection** — is emitted from one typed specification.
4. **Trait adapter** — contains only behavior required at a trait boundary; mechanical wrapper
   adapters use a `newtype` derive where one exists.
5. **Test projection** — verifies an owner and does not independently define production policy.

The audit is complete only while all evidence below passes. New modules are automatically included
because the code-style snapshot walks every workspace member and every Rust source outside `target`.

## Executed work

| Area | Single source of truth | Executed evidence | Status |
|---|---|---|---|
| Constants and diagnostics | `str_constants` and typed error owners | Duplicate long literals, production constants outside `str_constants`, and repeated ordinary fixtures are rejected by `source_policy`. | Verified |
| Wrapper construction and conversions | Domain `From`/`TryFrom` implementations and `newtype` derives | `domain_type_policy` rejects direct tuple-wrapper construction and bypassing conversion during deserialization; `source_policy` rejects manual mechanical forwarding implementations. For an identical source/target pair Rust coherence already prevents a simultaneous custom `From` and fallible `TryFrom`, because `From` supplies the blanket `TryFrom`. | Verified |
| Macro and naming families | `workspace_macro_helpers`, `generate_quotes`, common naming crates, and typed derive emitters | Macro entry points remain boundary adapters; generated wrapper behavior is covered by compile tests and the forwarding-implementation policies. | Verified |
| Configuration and runtime | Wrapper parsers, `config_lib`, `server_runtime`, and bounded I/O owners | Direct environment/filesystem/process access, blocking async calls, unowned spawned tasks, raw runtime SQL identifiers, and decentralized PostgreSQL error classification are checked workspace-wide. | Verified |
| Route and HTTP contracts | `frontend_contract` typed route metadata | Route type mismatches have compile-fail coverage; client/server/OpenAPI projections are checked against the contract. | Verified |
| PostgreSQL generators | Validated PG type, table, and filter specifications | Generator contract and compile tests verify SQL, bind, schema, route, client, frontend, and test projections. The generated where-filter compile manifest now declares the `newtype` workspace dependency used by emitted code. | Verified |
| Admin domain and service | `server_admin_contract` plus service/repository owners | Domain-boundary and SQL-ownership policies cover admin crates; handlers retain transport orchestration rather than owning admin SQL or query rules. | Verified |
| Admin CSR/SSR rendering | `server_admin_frontend::shared` | Table filters, audit filters, audit query inputs, and base pagination query inputs now have one shared renderer. A typed presentation enum preserves the intentional CSR numeric-limit/SSR hidden-limit difference; native, WASM, and SSR tests cover both compilation targets and SSR output. | Verified |
| Remaining production and test functions | Their domain owner, generator model, shared helper, trait boundary, or fixture owner | `reuse_policy::substantial_function_bodies_have_one_source_of_truth` scans non-test free functions and all methods and rejects structurally duplicate substantial bodies after identifier normalization. Its tests prove that identifier renames normalize, behavior changes remain distinct, and short adapters stay outside this gate. | Verified |

## Enforcement

- `cargo fmt`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test -p tests code_style`
- `cargo test --workspace --all-features`
- native and `wasm32-unknown-unknown` checks for `server_admin_frontend`

## Change protocol

Change the owner first. A projection must delegate or be generated. If the structural gate reports a
duplicate, extract a shared owner into the lowest existing crate both callers may depend on without
reversing an architecture boundary. Do not add exceptions to the duplicate-body gate. Semantically
required differences remain explicit, typed, and tested at their boundary.
