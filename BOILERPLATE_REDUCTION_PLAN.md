# Boilerplate Generation and Reuse Plan

## Objective

Reduce mechanically repeated Rust code by generating syntax from existing
sources of truth or extracting genuinely shared behavior. Preserve runtime
behavior, public APIs, diagnostic quality, architecture boundaries, domain
wrapper types, generated contract order, and async cancellation behavior.

This plan does not authorize new crates or dependencies. Implementation should
extend the existing owning proc-macro/shared crates or use private shared
functions in the existing owner. A new crate requires a separate explicit
request.

## Analysis scope and method

Audit date: 2026-07-29.

Scope: every workspace Rust source outside `target/**`, with focused inspection
of:

- all 20 proc-macro crates;
- existing shared and generator crates;
- `server_admin_contract`, `server_admin`, and `server_admin_frontend`;
- the PostgreSQL table, type, and filter generators;
- configuration descriptor tests;
- the repository code-style reuse policy and sources-of-truth policy.

The audit combined source counts, call-site inspection, generated-code
ownership, and existing architectural tests. Counts are discovery evidence,
not automatic proof that every occurrence should be generated.

## Current baseline

### Existing generation is already extensive

| Mechanism | Current uses |
|---|---:|
| `newtype::FromInner` | 790 |
| `newtype::AsRef*` derives | 256 |
| `optml::Optml` | 180 |
| `newtype::IntoInner*` derives | 151 |
| `newtype::Display` | 119 |
| `frontend_contract::TypedRoute` | 59 |
| `newtype::TryFrom` | 35 |
| `frontend_contract::RouteCatalog` | 5 |

The repository already generates the most obvious tuple-wrapper, route,
configuration, and CRUD boilerplate. New work should extend these sources of
truth instead of introducing parallel mechanisms.

### Remaining handwritten shapes

| Shape | Workspace count | Audit conclusion |
|---|---:|---|
| `From` impls | 136 | Mostly domain conversions; only exact field projections are candidates. |
| `TryFrom` impls | 181 | Mostly validation and error semantics; do not blanket-generate. |
| `Display` impls | 29 | Many redact or format domain values; use existing derives only for exact forwarding/constant cases. |
| `Debug` impls | 30 | Usually redaction or external-type wrappers; retain unless an existing redaction derive is exactly equivalent. |
| `AsRef` impls | 21 | Audit tuple wrappers against existing `newtype` derives. |
| `Default` impls | 9 | Public-contract intent can make a derivable impl intentionally handwritten. |
| `Deref` impls | 7 | Audit only exact tuple forwarding against existing derives. |
| Public `const fn new` | 90 | Named-struct constructors are the largest safe generation opportunity. |
| Public `const fn get` | 46 | Many are exact copy accessors and can use opt-in field metadata. |
| Validator functions | 33 | Domain logic; retain handwritten bodies and derive only their wiring. |

The existing reuse test rejects identical substantial function bodies at 50 or
more AST expressions. It intentionally does not catch short mechanical
adapters, which are the main opportunity in this plan.

## Decision rules

Generate or extract code only when all of the following hold:

1. One authoritative declaration already contains every fact needed to produce
   the code.
2. The generated implementation is deterministic and has no domain decisions.
3. The generated public signature is explicit at the declaration site.
4. Compiler errors can point to the relevant field, variant, or attribute.
5. Removing the generator would reveal straightforward mechanical code, not
   hidden control flow.
6. Snapshot, serialization, OpenAPI, SQL, route, and error behavior remain
   byte-for-byte or structurally identical as applicable.

Prefer an ordinary shared function when behavior, rather than syntax, is
repeated. Prefer a proc-macro when the repetition consists of declarations,
field names/types, trait impl syntax, or framework registration that Rust
cannot express through functions.

Do not generate code merely because several functions have similar names.

## Priority 1 — Opt-in contract struct API derive

### Evidence

`server_admin_contract/src/lib.rs` contains the largest concentration of manual
trait and inherent impls: 56 manual impl blocks matching the audited trait
families. In the administrator DTO region alone, the audit found:

- 31 mechanical `pub const fn new(...) -> Self` constructors;
- 12 consuming tuple/single-field projections such as `into_parts`;
- 77 simple copy/reference/slice accessors.

Examples include `AdminCreateUserReq`, `AdminUpdateUserReq`,
`AdminChangeOwnPasswordReq`, `AdminSetUserRolesReq`, `AdminUserSummary`, and the
three page response structures. Their constructors repeat field names and
types already declared by the struct.

### Proposed owner and interface

Extend the existing `frontend_contract_macros` crate with an opt-in derive for
named contract structs. Do not add a macro crate. A working name is
`ContractStructApi`; final naming should follow repository abbreviations.

Use field/type attributes to make the generated surface reviewable:

```rust
#[derive(frontend_contract::ContractStructApi)]
#[contract_struct_api(new)]
pub struct AdminUpdateUserReq {
    #[contract_struct_api(into)]
    display_name: Option<AdminDisplayName>,
    #[contract_struct_api(into)]
    login: Option<AdminLogin>,
}
```

The first implementation should support only:

- `new`: a `#[must_use] pub const fn new` taking fields in declaration order;
- borrowed accessors returning `&FieldType`;
- copy accessors returning `FieldType`, requiring an explicit field attribute;
- consuming accessors returning one field;
- `into_parts` returning all fields in declaration order;
- slice projections only when the attribute names an existing accessor
  expression or the field directly stores a supported repository collection.

Do not infer copy-vs-borrow behavior from trait resolution in the proc-macro.
Require explicit attributes so adding or removing `Copy` cannot silently change
the public API.

### Migration

1. Add compile-pass and compile-fail tests in `frontend_contract_macros`.
2. Generate one private fixture and compare expanded signatures.
3. Migrate leaf response types with constructor-only impls.
4. Migrate request types with `new` plus consuming projections.
5. Migrate summary/page types last; keep domain methods such as
   `has_permission`, `can_access`, and catalog lookup handwritten.
6. Compare the reviewed contract public API snapshot after every small batch.

### Acceptance criteria

- The public API snapshot is unchanged.
- Serde and OpenAPI snapshots are unchanged.
- Field order and tuple return order are unchanged.
- Every removed method is completely mechanical.
- Macro diagnostics name the unsupported struct/field attribute.

Expected payoff: remove dozens of handwritten methods without hiding domain
behavior.

## Priority 2 — Generate delegating route adapters from `route_openapi`

### Evidence

`server_admin/src/auth.rs` contains 28 functions annotated with
`frontend_contract::route_openapi`. At least 26 are thin adapters shaped as:

```rust
async fn create_user(/* extractors */) -> Result<AxumAdminResponse, AdminCreateUserError> {
    handlers::create_user(/* same arguments */)
        .await
        .map_err(AdminCreateUserError::from)
}
```

The function name, parameters, handler path, await, response type, error type,
and error conversion are all repeated syntax. These functions also carry
single-use lint annotations because framework registration owns their only
call.

### Proposed owner and interface

Extend the existing `frontend_contract_macros::route_openapi` attribute rather
than adding a second route macro. Add an explicit delegation option:

```rust
#[frontend_contract::route_openapi(
    delegate = handlers::create_user,
    tag = "admin_users"
)]
async fn create_user(
    auth: AdminAuthReq,
    path: AxumAdminPath<AdminUserId>,
    request: AxumAdminJson<AdminCreateUserReq>,
) -> Result<AxumAdminResponse, AdminCreateUserError> {}
```

The macro should replace an empty body with:

```rust
delegate(parameters_in_declaration_order)
    .await
    .map_err(ReturnError::from)
```

Requirements:

- reject non-empty bodies when `delegate` is present;
- reject receivers, patterns that cannot be forwarded, and non-`Result`
  returns;
- preserve parameter patterns and extractor types in the visible function;
- derive the return error type from the explicit signature, not naming
  conventions;
- preserve spans for the delegate path, parameter, and return type;
- emit the existing narrow framework lint reason at the generated function.

Do not generate handlers from a remote catalog in the first iteration. Keeping
the signature visible beside OpenAPI attributes makes review and compiler
diagnostics substantially better.

### Exclusions

Keep custom bodies handwritten when they:

- perform authorization or state transformations;
- build or mutate cookies/headers;
- select between repositories;
- add observability context;
- map more than one error source;
- change cancellation or future ownership.

### Acceptance criteria

- Axum route inventory and OpenAPI snapshots are unchanged.
- Route compile-fail fixtures cover wrong parameters, response, error, and
  delegate future shapes.
- Handler integration tests exercise representative zero-, one-, two-, and
  three-extractor adapters.
- Generated code does not broaden visibility or erase typed operation errors.

Expected payoff: replace roughly 26 repeated bodies and their local lint
annotations while retaining explicit route signatures.

## Priority 3 — Generate admin-settings field projections from the catalog

### Evidence

The administrator settings source of truth is `AdminSetting`. Eight settings
are repeated in `server_admin_frontend/src/shared.rs` as:

- fields in `AdminSettingsFormSignals`;
- signal initialization;
- an eight-arm `get` match;
- fields in `AdminSettingsFormValues`;
- conversion from `AdminSettingsView`;
- eight accessors;
- CSR update-payload assembly.

The setting catalog already owns input kind, optionality, label, and wire
identity. The frontend should not maintain a second field inventory.

### Proposed approach

Prefer a catalog projection over a generic struct derive:

1. Extend the existing `frontend_contract_macros::UnitEnumCatalog` or introduce
   a sibling derive in that same crate to generate a stable index and count for
   unit variants.
2. Store same-typed frontend values/signals in a fixed-size repository wrapper
   indexed by `AdminSetting`.
3. Construct the array by mapping `AdminSetting::ALL`.
4. Keep typed conversion to and from `AdminSettingsView` in one explicit
   function because optional/required semantics are domain logic.
5. Generate or centralize only the exhaustive field projection; do not move
   Leptos signals into the contract crate.

Avoid `HashMap`/`BTreeMap`: the catalog is fixed, a map allocates, and setting
lookup occurs in rendering paths. Do not use static mutable state.

### Acceptance criteria

- Adding an `AdminSetting` variant creates a compile error or failing exhaustive
  test until its typed conversion is supplied.
- No allocation is added inside render loops.
- CSR and SSR settings rendering tests remain unchanged.
- Clearable-vs-required behavior and wire field names remain owned by the
  contract catalog.

Expected payoff: remove five parallel eight-item inventories and make future
settings additions single-source.

## Priority 4 — Share pure pagination and table-query behavior

### Evidence

CSR and SSR rendering intentionally differ, but both compute:

- first displayed row;
- last displayed row;
- previous/next offsets;
- page-size/query preservation.

The same saturating/min arithmetic currently appears in
`server_admin_frontend/src/app.rs` and `server_admin_frontend/src/ssr.rs`.
Sharing Leptos views would couple different event and transport semantics, but
sharing the calculations is appropriate.

### Proposed owner

Add immutable wrapper types and pure functions to the existing
`server_admin_frontend/src/shared.rs` owner:

- input: domain wrappers for offset, limit, and total;
- output: a private `AdminPageRange` structure using domain wrappers;
- helpers for previous/next availability and offsets;
- no Leptos view, browser API, HTTP form, or string formatting.

If another independent frontend crate later needs the same semantics, move the
pure policy into an existing shared contract crate only after that second owner
exists. Do not broaden the public API preemptively.

### Acceptance criteria

- Unit tests cover empty results, first page, final partial page, out-of-range
  offsets, and overflow boundaries.
- CSR and SSR rendered output snapshots remain unchanged.
- No allocation is introduced.

Expected payoff: one tested pagination policy while preserving separate
renderers.

## Priority 5 — Extract configuration descriptor conformance helpers

### Evidence

Both:

- `server_config/tests/config_descriptor.rs`, and
- `notification_service_config/tests/config_descriptor.rs`

parse generated `.env.example` text into a `BTreeMap`, compare descriptor count
and examples, require all fields, and validate public examples. The
notification test then adds service-specific Compose/Kubernetes assertions.

### Proposed approach

Extract only the descriptor/example comparison into an existing
`config_lib` test-support function or macro:

- input generated example text and an iterator of
  `ConfigFieldDescriptor`;
- return a list of typed diagnostic messages or a `Result`;
- leave filesystem updates, environment access, service names, ports, Compose,
  Kubernetes, CI, and release assertions in the owning integration tests.

Before implementation, verify that exposing test support from `config_lib`
does not broaden its production public API. Prefer:

1. a function already useful to production projection validation;
2. an existing proc-macro expansion into the integration test;
3. retaining the small duplication if neither can stay private.

Do not create a test-helper crate without explicit authorization.

Expected payoff: modest. Implement only after Priorities 1–4.

## Priority 6 — Incrementally decompose and reuse CRUD token emitters

### Evidence

The three generator families contain approximately 18,051 Rust lines across
their modules. Their three monolithic `source.rs` files account for 17,023 of
those lines:

| Generator | Main source lines | Local `generate_*` closures | `quote!` sites |
|---|---:|---:|---:|
| PostgreSQL table | 10,184 | 113 | 889 |
| PostgreSQL types | 5,790 | 107 | 573 |
| Where filters | 1,049 | 31 | 70 |

The generator crates already have typed model/projection modules and the shared
`pg_crud_macros_common`, `macros_helpers`, and
`workspace_macro_helpers` crates. The remaining opportunity is incremental
reuse, not another top-level generator.

### Proposed sequence

1. Inventory local closures by emitted artifact: model, Serde, SQLx, OpenAPI,
   route, client, handler, frontend, and contract test.
2. Move a closure only when it is used by multiple projections or can be
   independently unit-tested.
3. Put cross-generator token syntax in the existing
   `pg_crud_macros_common`; keep table/type/filter semantics in their owning
   generator crate.
4. Pass typed descriptor wrappers rather than long token-stream argument lists.
5. Return repository token wrapper types, not raw external token streams at
   module boundaries.
6. Preserve local closures that capture many operation-specific values and are
   clearer beside their only use.
7. Add compact token/snapshot tests before moving each emitter.

### First concrete candidates

- shared generated Serde impl scaffolding around `_serde` imports;
- field iteration and punctuation construction already repeated across table
  and type emitters;
- generated contract-test JSON round-trip scaffolding;
- OpenAPI schema registration fragments;
- common error/location field emission.

Do not introduce a trait abstraction until at least two concrete emitters have
the same typed inputs and output semantics. Avoid trait objects; generation is
static.

### Acceptance criteria

- Generated source snapshots are unchanged unless a separately reviewed
  behavior change is requested.
- Macro-Clippy fixtures pass for every affected generator.
- Compiler error spans remain attached to the originating field/attribute.
- No generated function moves outside its usage scope.
- Generator compilation time and output size do not regress materially.

Expected payoff: long-term maintainability and fewer divergent token fragments;
execute in small batches because risk is high.

## Priority 7 — Repository row decoding: reuse behavior, do not generate SQL

### Evidence

Administrator repository modules repeat:

- SQLx error conversion;
- fallible conversion of primitive row fields into domain wrappers;
- mapping conversion failures to `InvalidStoredValue`;
- collection of converted rows.

The SQL statements, transaction ordering, lock behavior, and row semantics are
not boilerplate even when their syntax looks similar.

### Proposed approach

1. Introduce small private generic conversion helpers in the repository owner
   only where error mapping is exactly identical.
2. Prefer `TryFrom<StoredRow>` on private row-domain structures when the same row
   shape is consumed more than once.
3. Keep queries, fetch mode, transaction ownership, and mapping order explicit.
4. Consider proc-macro generation only after at least three private row
   structures demonstrate identical field-by-field conversion rules.
5. Never generate SQL from Rust DTOs; migrations remain the physical schema
   source of truth.

Expected payoff: reduced error-mapping noise. Risk is medium because hidden row
semantics can weaken database boundaries.

## Existing derives to apply before adding capabilities

Before writing new macro features, audit exact forwarding implementations
against capabilities already present in `newtype`:

- `AsRef`, `AsRefInner`, `AsRefOwned`, `AsRefStr`, `AsRefTarget`;
- `BorrowInner`, `BorrowOwned`, `BorrowPath`, `BorrowStr`;
- `CloneFields`, `CloneInner`;
- `DebugRedacted`, `DebugTransparent`, `DebugDisplay`;
- `DefaultInner`;
- `DerefInner`, `DerefTarget`, and mutable variants;
- `Display`, `DisplayConst`;
- `FromInner`, `IntoInner`, `IntoInnerFrom`;
- `Getter`, `IntoIterator`, `IntoVec`;
- `ToErrString*`.

For each manual impl, compare the complete behavior, bounds, visibility, and
redaction semantics. Do not replace a manual impl merely because the trait name
matches.

## Rejected or deferred candidates

### Domain validators and `TryFrom`

The 181 manual `TryFrom` impls include validation, normalization, external error
classification, and security policy. Continue using `newtype::TryFrom` only
where an explicit validator already owns the behavior. Do not create a
configuration-driven validation DSL.

### Debug and Display formatting

Manual formatting frequently redacts secrets, stabilizes external errors, or
implements a wire/domain format. Generate only exact transparent, constant, or
redacted shapes supported by existing derives.

### Whole CSR/SSR views

CSR uses event handlers and asynchronous mutations; SSR uses HTTP forms and
server navigation. Their visual similarity is not equivalent behavior. Share
pure calculations, typed catalogs, and small view fragments only.

### Axum repository/service handlers with business logic

Do not generate authorization, transactions, audit logging, cookie mutation,
rate limiting, or error observation. Priority 2 applies only to the outer
delegating adapters.

### SQL and migration declarations

Do not derive migrations or physical schema from Rust structures. Migrations
remain authoritative; generated CRUD descriptors are checked consumers.

### Error enums

Repeated `thiserror` syntax is intentionally explicit public/domain behavior.
Do not generate variants or messages from naming conventions.

## Implementation order

1. Add measurement and regression fixtures for short mechanical adapters.
2. Implement the contract struct API derive and migrate constructor-only DTOs.
3. Add route delegation to `route_openapi` and migrate thin adapters.
4. Replace the repeated settings field inventory with a catalog-indexed
   projection.
5. Extract pure pagination policy.
6. Evaluate the config conformance helper.
7. Decompose CRUD emitters one artifact family at a time.
8. Add repository conversion helpers only where three exact cases remain.

Each phase should be independently reviewable and must not depend on later
phases.

## Verification per phase

Always run:

```bash
cargo fmt
```

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

```bash
cargo test -p tests code_style
```

Also run the focused gates:

| Changed area | Required focused verification |
|---|---|
| `newtype` or contract struct derive | proc-macro unit/compile-fail tests, contract public API snapshot, Serde/OpenAPI tests |
| `route_openapi` | `frontend_contract_macros` tests, route-contract trybuild tests, admin API/OpenAPI integration tests |
| settings projection | admin contract tests plus CSR and SSR frontend tests |
| pagination | shared unit tests plus CSR/SSR rendered output tests |
| config descriptors | both config descriptor integration targets |
| CRUD generators | source tests, generator-consumer tests, macro-Clippy fixtures, generated contract tests |
| repository conversion | owning repository unit tests and database integration tests |

Run `git diff --check` before completion.

## Completion criteria

The plan is implemented only when:

- every migrated block has one authoritative declaration;
- generated public APIs match the reviewed snapshot;
- handwritten domain logic remains visible and tested;
- no new crate or dependency was introduced without explicit authorization;
- generated functions and closures remain inside their usage scope;
- no async lock/cancellation/transaction boundary changed;
- no source-of-truth inventory was duplicated;
- generated code has focused compile-fail and behavioral coverage;
- reduction counts are recorded by category and file;
- all required and focused verification passes.

The final implementation report should list handwritten lines/methods removed,
new macro attributes or shared functions, retained non-candidates, generated
output stability evidence, and exact test results.

## Implementation outcome

Implementation date: 2026-07-29.

### Completed generation and reuse work

1. `frontend_contract_macros` now owns the opt-in `ContractStructApi` derive.
   Its explicit type and field attributes generate constructors, borrowed
   accessors, copy accessors with either receiver form, optional borrowed
   accessors, consuming accessors, tuple projections, and collection slices.
   Twenty-five administrator contract structures use the derive. This removed
   23 handwritten `Admin*` impl blocks from `server_admin_contract` while
   retaining domain methods and Utoipa name-accessor lint context by hand.
2. `route_openapi` now accepts `delegate = path`, rejects non-empty delegated
   bodies and unsupported signatures, forwards visible identifier parameters
   in declaration order, awaits the delegate, and converts the explicit return
   error with `From`. All 28 administrator route adapters use it.
3. `UnitEnumIndex` generates the fixed catalog count and total variant-to-index
   projection for `AdminSetting`. Settings form values and Leptos signals now
   use fixed arrays indexed by that projection. The typed
   `AdminSettingsView` conversion remains one exhaustive handwritten match.
4. `AdminPageRange` centralizes range endpoints, previous/next offsets, and
   navigation availability for CSR data grids, CSR catalog tables, and SSR
   tables. It uses administrator domain wrappers and performs no allocation.
5. `server_admin::repository` now owns the private
   `invalid_stored_value` conversion helper. The permissions and roles row
   projections reuse it at 17 identical conversion sites; SQL, fetch modes,
   transaction ordering, and row construction remain explicit.
6. The contract public-API verifier now projects the explicit
   `ContractStructApi` and `UnitEnumIndex` declarations into its reviewed
   signature inventory. Generated APIs therefore remain reviewed instead of
   disappearing from the source-AST snapshot.

### Conditional items retained after evaluation

- The configuration descriptor comparison remains local to its two integration
  tests. The tests are in separate crates, `config_lib` has no private
  cross-crate test-support surface, and exposing the helper would broaden its
  production API. A helper crate or new dependency was not authorized. This is
  the plan's explicit retain-duplication outcome when neither a production
  function nor a private macro expansion is justified.
- No CRUD token emitter moved. Reinspection found that common derive and
  error/location scaffolding already uses `pg_crud_macros_common` or
  `macros_helpers`; the remaining proposed Serde attribute closures occur only
  in the PostgreSQL type generator and therefore do not meet the required two
  concrete emitters threshold. Generated snapshots were left untouched.
- No row-decoding proc-macro or private row domain structure was introduced.
  The audited row shapes do not have three identical field-by-field rules;
  only their error projection is identical and was centralized.

### Added focused coverage

- Proc-macro parser tests cover explicit contract-API attributes and unknown
  attribute rejection.
- Trybuild fixtures cover a non-named `ContractStructApi` input and a delegated
  route with a non-empty body.
- Pagination unit tests cover empty, first, final partial, out-of-range, and
  overflow-boundary pages.
- The existing contract public-API snapshot test covers generated signature
  stability for the migrated DTOs and catalog index.

No crate or dependency was added. No handler business logic, validator, error
enum, query, migration, transaction boundary, async runtime, or rendered
frontend behavior was changed.
