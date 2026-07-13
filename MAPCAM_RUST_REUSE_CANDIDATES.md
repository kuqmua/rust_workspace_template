# Reuse Candidates from `mapcam_rust`

## Scope

This report compares `/home/sergey/projects/mapcam_rust` with `/home/sergey/projects/rust_workspace_template`. It lists reusable designs supported by existing source code, not features inferred only from crate names. The recommendation is to port small, independently testable concepts into existing shared crates. Copying the large `shared` crate or introducing domain-specific Mapcam dependencies would create unwanted coupling.

No implementation or dependency change is part of this report.

## Priority Summary

| Priority | Candidate | Suggested owner in this workspace | Main benefit |
|---|---|---|---|
| P0 | Route contract as a single source of truth | `pg_crud/pg_tbl/gen_pg_tbl_src` plus a small shared contract model | Prevent drift between paths, clients, handlers, and OpenAPI |
| P0 | OpenAPI contract verification suite | `tests` and `server_tbl_example` | Detect undocumented routes and incorrect response contracts |
| P0 | General bounded collections | `newtype` or a dedicated existing shared crate | Enforce API and memory limits during construction and deserialization |
| P0 | Supervised application lifecycle tasks | `server_runtime` | Make shutdown, task panic, cancellation, and timeout ownership explicit |
| P1 | Bounded concurrency with timeout semantics | `server_runtime` or `route_validators` | Apply backpressure and return deterministic overload responses |
| P1 | Shared JSON contract test helpers | `macros_helpers` test surface or `tests` | Standardize encode/decode/re-encode contract checks |
| P1 | Property, compile-fail, and golden contract tests | Relevant crates plus `tests` | Cover parser invariants and generated API contracts beyond examples |
| P1 | Supply-chain and API compatibility CI gates | `.github/workflows/ci.yml` and `workspace_test_runner` | Detect dependency, feature, unused dependency, and semver regressions |
| P2 | Richer bounded text generation | Extend `newtype::BoundedString` selectively | Add trim/NUL/normalization policies without handwritten wrappers |
| P2 | Guarded database test configuration | `tests` or `workspace_test_runner` | Prevent integration tests from targeting a non-test database |
| P2 | Locked and isolated Cargo workflows | `.cargo/config.toml` | Improve reproducibility and parallel local testing |
| P2 | Security-focused redaction helpers | A relevant shared runtime crate | Prevent secrets from leaking through `Debug` and logs |

## 1. Route Contract as a Single Source of Truth

### Source evidence

- `mapcam_rust/route_contract_macros/src/lib.rs` implements `define_route_contract`.
- `mapcam_rust/shared/src/route_contract.rs` uses one declaration to generate route paths, API paths, client calls, response variants, metadata, and tests.
- `mapcam_rust/shared/src/api_route_contract.rs` represents authentication, request body, response body, schema, status, and operation tags as enums rather than loosely related strings.

### What to reuse

Add a compact route contract model to generated table APIs. For each CRUD operation it should be possible to derive from one model:

1. Axum method and path registration.
2. Client method and URL construction.
3. OpenAPI method, path, operation identifier, request schema, and success statuses.
4. Contract test descriptors.

The current `gen_pg_tbl_src` already generates routes, clients, and OpenAPI, so the useful part is the shared intermediate model and parity checks. Porting the 250 KB Mapcam macro unchanged would be excessive.

### Suggested first slice

Introduce a private `GeneratedRouteContract` model inside `gen_pg_tbl_src` and use it for one operation, such as read-one, before migrating all eight operations. Keep generated public signatures unchanged.

### Acceptance evidence

- Generated route path, client path, and OpenAPI path are equal for all eight CRUD operations.
- Generated success statuses match the response enum variants.
- Existing `gen_pg_tbl_test` output and API signatures remain stable.

## 2. OpenAPI Contract Verification Suite

### Source evidence

- `mapcam_rust/mapcam/tests/integration/openapi_route_parity.rs` compares live runtime routes with the served OpenAPI document.
- The Mapcam README defines an ordered suite: document parse/lint, route parity, runtime conformance, negative contracts, and breaking-change snapshot.
- `mapcam_rust/.cargo/config.toml` exposes this as `cargo openapi-contract-suite`.

### What to reuse

The current workspace generates a document and has schema-focused tests, but it does not have a complete runtime/OpenAPI parity gate. Add a DB-independent suite around `server_tbl_example` that verifies:

1. Every generated runtime method/path is documented.
2. Every documented generated operation has a runtime route.
3. Request and response content types match actual handlers.
4. Every generated success and error status is declared.
5. Operation identifiers are unique.
6. A normalized JSON snapshot detects accidental breaking changes.

Prefer testing route metadata directly where Axum does not expose route enumeration. A generated route descriptor list is more stable than parsing Axum internals.

### Acceptance evidence

- A deliberately removed route causes a missing-runtime-route failure.
- A deliberately removed OpenAPI path causes a missing-documentation failure.
- Snapshot updates are explicit and deterministic.

## 3. General Bounded Collections

### Source evidence

- `mapcam_rust/shared/src/models.rs` contains `BoundedVec<T, MIN_ITEMS, MAX_ITEMS>` with constructor and Serde boundary validation.
- Tests cover below-minimum, above-maximum, valid round trips, serialization, and reporting the first overflow length.
- Mapcam API contracts use bounded collections for permissions and bulk mutation bodies.

### Current gap

This workspace has `wh_flts::BoundedVec<T, LENGTH>`, which enforces an exact length, and `NotEmptyUnqVec`, which enforces non-empty uniqueness. It lacks one reusable collection that expresses an inclusive item-count range and rejects oversized input while deserializing.

### What to reuse

Add a generic private-field wrapper with const minimum and maximum bounds, consuming `TryFrom<Vec<T>>`, borrowed slice access, consuming `into_vec`, Serde validation, and an explicit error enum. Do not merge uniqueness into the same type; cardinality and uniqueness are separate invariants.

This is particularly useful for generated bulk CRUD payloads, selected columns, order clauses, and filter lists. It prevents unbounded JSON arrays from being allocated and processed without an application-level limit.

### Acceptance evidence

- Minimum, maximum, empty, and maximum-plus-one cases.
- Deserialization stops at the first item beyond the maximum rather than building the complete oversized vector.
- OpenAPI `minItems` and `maxItems` match runtime validation.
- Existing exact-length and uniqueness wrappers retain their semantics.

## 4. Supervised Application Lifecycle Tasks

### Source evidence

- `mapcam_rust/shared/src/server.rs` defines `ApplicationLifecycleBackgroundTask`, shutdown signal enums, task log configuration, task panic reporting, and server shutdown functions.
- `mapcam_rust/microservice_runtime/src/lib.rs` returns a `ServiceRuntime` that owns the router and optional task handle.

### Current gap

`server_runtime` already has graceful shutdown and interval tasks, but `spawn_interval_task` returns a bare optional `JoinHandle<()>`. The owner must remember to cancel and await it, and a task panic can be lost unless every caller implements the same policy.

### What to reuse

Extend the existing `server_runtime` rather than adding another runtime crate:

1. Return a task owner that is `#[must_use]`.
2. Define explicit shutdown-requested, completed, panicked, and timed-out outcomes.
3. Signal cancellation, await the task, and enforce the same shutdown deadline as the HTTP server.
4. Preserve task panic details in a domain error.
5. Keep interval missed-tick behavior as `Skip`.

### Acceptance evidence

- Normal shutdown awaits task completion.
- A stuck task reaches a deterministic timeout outcome.
- A panicking task is observable by the server owner.
- Dropping a task owner cannot silently detach a long-lived task.

## 5. Bounded Concurrency and Overload Responses

### Source evidence

- `mapcam_rust/shared/src/async_permits.rs` acquires an owned semaphore permit under `tokio::time::timeout` and maps timeout to an explicit too-many-requests error with retry information.
- Tests cover available, saturated, and closed semaphore cases.

### What to reuse

Add a small reusable helper only when at least two generated or common routes need the same policy. Likely consumers are bulk mutation routes and expensive read/filter routes. The helper should accept an `Arc<Semaphore>`, a bounded wait duration, and caller-owned error constructors so route-specific errors remain visible.

Complete the existing `Retry-After` TODO in `gen_pg_tbl_src` by representing retry duration as a validated domain type and emitting a standards-compatible header for overload responses.

### Acceptance evidence

- Permit lifetime covers the complete operation and is released on cancellation.
- Saturation returns the documented status and `Retry-After` header.
- Closed semaphore maps to an internal lifecycle error, not overload.
- No lock or permit acquisition is held across unrelated work.

## 6. Shared JSON Contract Test Helpers

### Source evidence

- `mapcam_rust/shared/src/test_support.rs` contains phase-aware JSON helpers, including `ensure_json_contract_round_trips_for_test`.
- The helper distinguishes deserialize, serialize, and reparse failures and is reused by several API-contract modules.

### What to reuse

Create a small test-only helper that performs:

1. Deserialize fixture JSON.
2. Serialize the typed value.
3. Deserialize the serialized value again.
4. Compare typed values and optionally normalized JSON values.
5. Identify the failing phase in its error enum or message.

Use it in `gen_pg_types_test`, `gen_pg_tbl_test`, and `gen_wh_flts_test`. Keep fixtures and domain assertions local; only the mechanical round-trip logic should be shared.

### Acceptance evidence

- Deterministic tests for failures in every phase.
- Existing wire-contract tests retain their names and fixtures.
- The helper remains test-only and does not expand production public API.

## 7. Property, Compile-Fail, and Golden Tests

### Source evidence

- Mapcam uses `proptest` in parsers and reconciliation logic, including `buildings_sync`, `buscheb_sync`, `map_tile_walker`, and `mapcam_to_mapcam_layers_synchronization_service`.
- The root workspace declares `trybuild` for type-level macro contracts.
- `mapcam/tests/integration/api_json_snapshots.rs` keeps deterministic JSON contract snapshots.

### What to reuse

Apply these techniques selectively:

- Property tests for naming conversions, bounded strings, SQL placeholder numbering, range filters, and duplicate detection.
- Compile-fail tests for malformed proc-macro attributes and generated trait/type contracts.
- Golden tests for normalized OpenAPI JSON, generated SQL, error JSON, and generated Rust source where formatting is part of the contract.

Adding `proptest` or `trybuild` requires a separate explicit dependency request under this workspace's rules. Useful zero-dependency property coverage can begin with deterministic exhaustive inputs for small alphabets and bounds.

### Acceptance evidence

- Every property has a small reference model.
- Failing cases are reproducible with fixed seeds or persisted regressions.
- Golden updates require an explicit environment flag or command.

## 8. Supply-Chain and API Compatibility Gates

### Source evidence

`mapcam_rust/.github/workflows/ci.yml` contains separate jobs for:

- `cargo deny`;
- `cargo audit`;
- `cargo machete`;
- `cargo semver-checks`;
- `cargo hack --feature-powerset`;
- `cargo udeps`;
- filesystem vulnerability and secret scans;
- coverage and benchmark comparison.

`mapcam_rust/release_verification_checklist.md` records when these checks are required.

### Current gap

This workspace CI runs formatting, Clippy, docs, audit, and tests in one job. A failure late in that job delays feedback, and feature-matrix, unused-dependency, license/source, semver, secret, and benchmark gates are absent.

### What to reuse

Start with the highest-value low-coupling gates:

1. `cargo metadata --locked` and `--locked` on reproducibility-sensitive CI commands.
2. Separate format, Clippy, static tests, and database tests.
3. `cargo deny` for licenses, sources, and bans.
4. `cargo hack` when features change.
5. `cargo semver-checks` for public generated APIs.
6. `cargo udeps` on a schedule rather than every fast PR.
7. Secret scanning with pinned action revisions.

Do not copy action revisions blindly; verify current trusted revisions when implementing because CI action versions are time-sensitive.

### Acceptance evidence

- Every new gate has an owner, trigger policy, timeout, and documented local equivalent.
- Database-independent jobs do not start PostgreSQL.
- Public proc-macro/generated API changes run semver checks against an appropriate baseline.

## 9. Richer Bounded Text Generation

### Source evidence

- `mapcam_rust/domain_text_macros/src/lib.rs` generates required, NUL-free, bounded, delegating, and validator-backed text wrappers.
- `mapcam_rust/shared/src/domain_text.rs` applies these macros to headers, identifiers, tokens, paths, and API fields.

### Current state and recommendation

The local `newtype::BoundedString` derive already covers the most valuable invariant: maximum length. Extend it only for repeated local needs:

- byte-length versus character-count policy;
- required/non-empty text;
- trimming policy;
- rejection of NUL bytes;
- optional custom validator mapping;
- Serde validation and OpenAPI constraint parity.

Do not port Mapcam's macro wholesale. Its many construction strategies and options reflect a much larger domain and would enlarge the public macro surface prematurely.

### Acceptance evidence

- Runtime, Serde, and OpenAPI limits are generated from the same attribute.
- Compile-fail tests cover incompatible or missing options.
- Existing `BoundedString` users compile without changes.

## 10. Guarded Database Test Configuration

### Source evidence

- `mapcam_rust/shared/src/test_support.rs` validates automated-test database URLs and produces a sanitized database target for diagnostics.
- Mapcam CI separates static, database, and heavy-load test modes.

### What to reuse

Before any test creates, truncates, migrates, or drops database objects, validate that the URL identifies the expected local/CI test database. Keep credentials out of diagnostics. Put the guard in the shared test runner so individual test crates cannot accidentally bypass it.

### Acceptance evidence

- Production-like and ambiguous database URLs are rejected before connection or mutation.
- Credentials are redacted from all error messages.
- Static tests remain completely database-independent.

## 11. Locked and Isolated Cargo Workflows

### Source evidence

- Both projects already have isolated test aliases, but Mapcam consistently uses `--locked` in local verification aliases and CI metadata checks.
- Mapcam exposes a dedicated OpenAPI suite and separates static/database/heavy-load runner modes.

### What to reuse

- Add `--locked` to CI and release verification commands.
- Add a named OpenAPI contract alias after the suite exists.
- Consider explicit static, database, and heavy-load modes in `workspace_test_runner` if the current `all` mode cannot provide fast feedback.

The isolated target-directory alias is already present here and does not need to be copied.

## 12. Security-Focused Redaction Helpers

### Source evidence

- `mapcam_rust/shared` centralizes secret-bearing domain types and redaction behavior.
- `RUST_CODE_REUSE_PLAN.md` documents the completed canonical RTSP/RTSPS userinfo redaction shared by multiple consumers.
- Proxy tests assert that authorization data is redacted in `Debug` output.

### What to reuse

Audit this workspace's configuration, database URL, authorization header, API client, and future proxy types. For any value that can reach tracing or `Debug`, provide one canonical redacted formatter in an existing shared crate. Prefer wrapper-specific `Debug` implementations over call-site discipline.

Only add a generic URL-userinfo helper after two real consumers exist. There is currently no reason to import RTSP-specific code into this workspace.

### Acceptance evidence

- Table tests cover username-only, username/password, percent-encoded, malformed, query, fragment, and IPv6 inputs for any URL helper.
- Original credentials never appear in `Debug`, tracing fields, or errors.
- Valid non-secret URL components remain visible for diagnosis.

## Items Not Recommended for Transfer

1. **The complete `shared` crate.** It is roughly 786 KB in its root source plus many large modules and is tightly coupled to Mapcam domain contracts.
2. **`mapcam_string_constants`.** Multi-megabyte generated constant modules would increase compile time and hide local ownership. Extract only genuinely repeated protocol constants near their consumers.
3. **The full route macro immediately.** Its parser and generated surface are much larger than the eight CRUD operations currently need. First establish a private route model and parity suite.
4. **Mapcam authentication, GIS, camera, RTSP, tile-cache, and synchronization domain code.** These have no matching consumer in the template.
5. **Cursor pagination helpers.** This workspace explicitly standardizes on limit/offset pagination.
6. **A second runtime crate.** Extend `server_runtime`; duplicating `microservice_runtime` would split lifecycle ownership.
7. **Mapcam-specific string naming policy.** This workspace explicitly prefers abbreviations, while Mapcam explicitly forbids them.
8. **Large generated string-constant catalogs.** They are not a substitute for typed contracts and would conflict with the local minimal-public-API rule.

## Suggested Adoption Order

1. Add route descriptors and OpenAPI/runtime parity tests without changing generated public APIs.
2. Add a reusable min/max bounded collection and use it for generated bulk/filter inputs.
3. Strengthen `server_runtime` task ownership and graceful shutdown supervision.
4. Add overload permits and `Retry-After` only for measured expensive routes.
5. Consolidate JSON contract test mechanics.
6. Add CI gates incrementally, beginning with locked metadata, `cargo deny`, and feature-matrix checks.
7. Extend bounded text generation only when repeated validation policies are demonstrated.

Each adoption should be a separate change with its own contract tests. No candidate requires copying a Mapcam domain crate into this workspace.

## Implementation status

The adoption described by this report has been implemented as follows:

1. `gen_pg_tbl_src` generates one eight-entry route contract model and derives route, client, operation, method, authentication, success-status, and OpenAPI parity metadata from it.
2. Generated DB-independent contract tests execute in `cargo openapi-contract-suite`. They check both route-set directions, unique operation identifiers, JSON content types, success/error statuses, and all eight paths.
3. `pg_crud_cmn::bounded_vec::BoundedVec<T, MIN, MAX>` validates construction and streaming deserialization, exposes borrowed and consuming access, and shares bounds with Schemars and Utoipa. Generated bulk create/update payloads use it during deserialization.
4. `server_runtime::BackgroundTask` owns cancellation and join state. Drop aborts instead of detaching; tests cover normal shutdown, panic propagation, and a deterministic paused-time timeout.
5. `server_runtime` provides validated permit-wait and retry-after types, a semaphore owner, a permit RAII wrapper, and distinct saturation/closed errors. No hard-coded route concurrency limit was introduced because the report requires two measured consumers before adopting one; route-specific status mapping therefore remains caller-owned.
6. `macros_helpers::json_contract` is available only through `test-utils`, reports fixture, serialization, and reparse phases, and is used by all three generator test crates.
7. Zero-dependency exhaustive coverage now compares bounded text against a small reference model. Proc-macro compile-error tests cover missing and incompatible options, while generated JSON/OpenAPI contract tests act as deterministic contract gates. `proptest` and `trybuild` were not added because the report requires a separate dependency request.
8. CI has separate metadata, formatting, Clippy, docs, static contract, database, dependency policy, feature matrix, semver, secret scan, and scheduled unused-dependency jobs. Action revisions and the Gitleaks artifact checksum are pinned.
9. `newtype::BoundedString` supports optional minimum length, character-count semantics, trimming, NUL rejection, Serde validation, and Utoipa limits. Existing byte-count users remain unchanged; Utoipa requires character-count mode so runtime and schema semantics cannot drift.
10. The shared test-only database guard rejects malformed, non-loopback, and ambiguously named databases without including credentials in diagnostics. Both the database runner and generated destructive CRUD tests invoke it before connecting.
11. Reproducibility-sensitive aliases and runner commands use `--locked`; isolated testing remains available, and named static, database, OpenAPI, and measurement modes cover the distinct workloads.
12. Secret URL wrappers retain `SecretBox`-based `Debug` redaction, now covered for username, password, percent-encoded credentials, query, fragment, and IPv6 cases. No generic URL helper was introduced because there are not yet two non-PostgreSQL consumers.

Operational ownership, triggers, timeouts, and local equivalents for CI gates are recorded in `RELEASE_VERIFICATION_CHECKLIST.md`.

## Second-Wave Adoption Plan

The first twelve candidates above are implemented. The following candidates were identified during a deeper comparison of API mutation guarantees, PostgreSQL filtering, operational maintenance, health reporting, metrics, and client authentication behavior. They are intentionally separate from the completed work.

| Priority | Candidate | Suggested owner | Status |
|---|---|---|---|
| P0 | Idempotent generated mutations | `gen_pg_tbl_src`, `pg_crud_cmn`, server migration owner | Completed |
| P0 | Escaped text search filters | `gen_wh_flts_src` and `wh_flts` | Completed |
| P1 | Batched service-table cleanup | `server_admin` and `server_runtime` | Completed |
| P1 | Composite liveness and readiness | `cmn_routes` and `server_runtime` | Completed |
| P1 | Bounded metric label cardinality | `server_runtime` and generated CRUD metrics | Completed |
| P1 | Optimistic concurrency for updates | `gen_pg_tbl_src` | Completed |
| P2 | Shared resource budgets | `server_runtime` | Completed |
| P2 | Standard API problem responses | contract crates and generated routes | Completed |
| P2 | Frontend authentication keep-alive state machine | `server_admin_frontend` | Completed |
| P2 | Negative HTTP contract suite | `tests` and generated contract tests | Completed |
| P2 | Migration and seed invariants | `workspace_test_runner` and integration tests | Completed |
| Deferred | Safe file staging | an existing shared runtime crate after an upload consumer exists | Deferred |

### 13. Idempotent Generated Mutations

Status: Completed.

#### Source evidence

- `mapcam_rust` stores API idempotency state in `api_idempotency` and applies per-user pending and total limits through `api_idempotency_limits`.
- Integration tests cover replay, changed payload conflicts, route scoping, user scoping, concurrent requests, validation failures, and cleanup limits.
- Idempotency keys, request hashes, route paths, statuses, and cached response bodies use validated domain wrappers.

#### Planned design

Add an opt-in idempotency property to generated `create`, `update`, and `delete` operation descriptors. Scope a key by authenticated actor, HTTP method, normalized route path, and idempotency key. Hash the canonical request bytes and store successful response status and body transactionally.

The behavior must distinguish:

1. Missing key: execute normally unless the operation requires a key.
2. First matching key: reserve the operation and execute it once.
3. Completed matching request: replay the exact documented response.
4. Same key with a different request hash: return a conflict.
5. Matching request already in progress: return a deterministic retryable response.
6. Validation failure before reservation: allow a corrected retry with the same key.

Do not copy Mapcam table names or domain types. Generate only operation integration points; keep storage and lifecycle logic in an existing shared crate.

#### Acceptance evidence

- Concurrent identical requests perform one mutation.
- A replay returns the original status and body without repeating database writes.
- Keys are isolated by actor, method, and route.
- Reusing a key for another payload returns the documented conflict.
- Pending and completed records have bounded retention and batched cleanup.
- Generated OpenAPI documents the header and every idempotency response.

#### Implementation evidence

- Generated mutation descriptors opt in through `idempotent_mutations`; handlers require one bounded `Idempotency-Key` and scope it by actor, method, route, key, and SHA-256 request hash.
- A committed pending reservation coordinates concurrent callers. Matching completed requests replay the stored status/body, changed payloads return 409, and in-progress requests return 425.
- Mutation SQL and the transition from pending to the cached successful response run in the same SQLx transaction; serialization, budget, completion, and commit failures roll back the mutation and release the pending reservation.
- Completed and pending retention use separately validated durations and one validated cleanup batch size.
- PostgreSQL integration tests cover replay, conflict, actor isolation, corrected retry after release, concurrent acquisition, transactional rollback, and bounded cleanup; generated OpenAPI and clients share the required header/status contract.

### 14. Escaped PostgreSQL Text Search Filters — Completed

#### Source evidence

- `mapcam_rust/shared/src/core_impl/sql_like_pattern_text.rs` supports contains, starts-with, and ends-with modes.
- It escapes PostgreSQL `LIKE` metacharacters `%`, `_`, and `\` and bounds source text before allocating the pattern.

#### Planned design

Extend `gen_wh_flts_src` with explicit text match modes. Generate pattern construction and SQL using a declared escape character, for example `ILIKE $1 ESCAPE '\'`. Treat literal search and raw SQL pattern search as different types; generated public API must not accept an unvalidated raw pattern accidentally.

#### Acceptance evidence

- Literal `%`, `_`, and `\` match themselves.
- Contains, starts-with, and ends-with produce distinct expected SQL and bind values.
- Empty and oversized inputs follow one documented validation policy.
- Placeholder order remains aligned with generated bind order.
- Unicode input and worst-case escaping boundaries are deterministic.

#### Implementation evidence

- `gen_wh_flts_src` generates `TextSearchMode`, `TextSearchPattern`, `TextSearchValueEr`, and `PgTypeWhTextSearch`.
- Generated patterns escape `%`, `_`, and `\`, enforce a 1,024-byte source limit, and distinguish contains, starts-with, and ends-with.
- `PgTypeWhTextSearch` implements `PgTypeWhFlt`, binds the escaped value, and emits `ILIKE $n ESCAPE '\'` with the shared placeholder counter and logical operator.
- `gen_wh_flts_test` covers all modes, reserved symbols, empty and oversized inputs, SQL text, and placeholder progression. Its generated-crate Clippy/test check passes.

### 15. Batched Cleanup of Service Tables

Status: Completed.

#### Source evidence

- Mapcam builds bounded-delete SQL for expired records and old revoked sessions.
- Its cleanup covers token, session, rate-limit, audit, and idempotency-style operational data without issuing an unbounded delete.

#### Planned design

Add a supervised maintenance task that deletes a configured maximum number of rows per transaction. Initial consumers should be expired access sessions, refresh tokens, login attempts, and obsolete rate-limit windows. Retention duration, run interval, and batch size must use validated configuration wrappers.

#### Acceptance evidence

- Each transaction deletes no more than the configured batch size.
- Cancellation between batches is safe and promptly observed.
- Cleanup does not hold a lock across a wait interval.
- Task outcomes are visible through existing background-task supervision.
- Tests use fixed timestamps or explicit database timestamps rather than sleeps.

#### Implementation evidence

- `server_admin::cleanup_admin_tables` applies the validated batch size independently to expired access sessions, refresh tokens, login attempts, audit entries, rate-limit windows, and idempotency records.
- The server owns cleanup through the existing supervised `server_runtime::BackgroundTask`; cancellation and shutdown use the same observable task lifecycle as other maintenance work.
- Migration `0004_admin_audit_cleanup.sql` preserves the append-only audit trigger for ordinary callers while allowing only a transaction-local maintenance delete.
- `postgresql_cleanup_is_batched_and_preserves_append_only_policy` uses fixed PostgreSQL timestamps, proves a batch of two leaves one of three rows in every populated table, and proves an ordinary audit delete still fails.

### 16. Composite Liveness and Readiness — Completed

#### Source evidence

- Mapcam models database and service health independently and maps dependency failure to degraded service state.
- Its health responses distinguish component kind, component status, and aggregate status.

#### Planned design

Define three contracts:

- `/health/live`: the process and runtime are alive; no external dependency query.
- `/health/ready`: required dependencies are usable and migrations are compatible.
- `/health`: a typed component summary for operators.

Database probes must have a short timeout. If probes become expensive, cache only the immutable snapshot for a short bounded duration rather than holding a lock while awaiting PostgreSQL.

#### Acceptance evidence

- PostgreSQL failure does not make liveness fail but makes readiness fail.
- Probe timeout and query failure are distinct internal outcomes with stable public mapping.
- OpenAPI schemas match runtime response bodies.
- Health endpoints never disclose connection strings or internal SQL errors.

#### Implementation evidence

- `cmn_routes` exposes `/health/live`, `/health/ready`, and `/health` while preserving the legacy `/health_check` endpoint.
- Liveness performs no external query. Readiness runs the PostgreSQL probe under a two-second timeout and maps failure to HTTP 503 plus a degraded aggregate state.
- `HealthReport`, `HealthComponent`, `HealthComponentKind`, and `HealthStatus` are typed Serde and Utoipa contracts and contain no database error detail.
- Unit tests cover live, ready, and degraded component/status mappings.

### 17. Bounded Metric Label Cardinality

Status: Completed.

#### Source evidence

- Mapcam normalizes methods and statuses into a fixed label set and bounds cached path-label entries.
- It records route templates rather than arbitrary request paths where possible.

#### Planned design

Ensure generated and common HTTP metrics use only finite descriptors:

- route template rather than a concrete identifier-bearing URI;
- generated table and operation values;
- normalized HTTP method and status;
- no login, request identifier, error detail, query value, or user-provided text.

Unknown route templates must map to a bounded fallback instead of expanding the label set indefinitely.

#### Acceptance evidence

- Requests to many identifiers create one route-template series.
- Unknown paths do not grow an unbounded cache.
- A source-level or unit-test allowlist covers every metric label source.
- Existing generated metric names remain stable unless a migration is explicitly documented.

#### Implementation evidence

- Generated CRUD request, duration, and response metrics use compile-time table and operation labels rather than request-derived table values.
- Response statuses are normalized to the finite set `200`, `201`, `400`, `409`, `413`, `425`, `500`, and `other`.
- The existing metric names remain unchanged.
- `gen_pg_tbl_test::generated_metrics_use_bounded_labels` is a source-level generation test that rejects a dynamic table-label expression and requires the idempotency response labels.

### 18. Optimistic Concurrency for Generated Updates

Status: Completed.

#### Planned design

Add an optional table capability backed by a revision column or another explicit concurrency token. Generated update SQL must include the expected token in its predicate and atomically return the next token. Expose the token through a typed contract and optionally through `ETag`/`If-Match` at the HTTP boundary.

Idempotency and optimistic concurrency solve different problems: idempotency prevents a retry from executing twice, while a concurrency token prevents one client from silently overwriting a newer update.

#### Acceptance evidence

- Updating with the current revision succeeds and increments it exactly once.
- Updating with a stale revision changes no row and returns the documented conflict or precondition status.
- Concurrent updates cannot both succeed with the same expected revision.
- Generated SQL, client contracts, and OpenAPI use the same token semantics.

#### Implementation evidence

- `optimistic_revision_field` is an opt-in generated-table capability restricted to a non-primary-key signed 64-bit revision field.
- Generated update-one SQL increments the revision and predicates the update on the typed `If-Match` value in the same statement; stale values return 412 and missing values return 428.
- Generated frontend/Reqwest clients and OpenAPI use the same `If-Match` contract, and the ambiguous update-many route is omitted for an optimistic table.
- `postgresql_optimistic_revision_allows_one_concurrent_writer` proves that two updates with revision zero produce one success, one miss, and a final revision of one.

### 19. Shared Resource Budgets

Status: Completed.

#### Source evidence

- `mapcam_rust/shared/src/core_impl/atomic_count_reservation.rs` atomically reserves a count under a maximum and releases it through an RAII guard.
- This bounds aggregate work even when individual concurrent operations have different sizes.

#### Planned design

Add a generic resource budget only after two consumers exist. Likely consumers are bulk CRUD item counts and total bytes in concurrent import/export work. Reservation must fail on arithmetic overflow or limit exhaustion and release automatically on success, error, panic unwind, or future cancellation.

#### Acceptance evidence

- Concurrent reservations never exceed the configured maximum.
- Failed reservations do not alter the counter.
- Dropping guards returns exactly their own reservation.
- Overflow has a distinct error and never saturates silently.

#### Implementation evidence

- `server_runtime::ResourceBudget` uses an atomic checked update and an RAII `ResourceBudgetReservation` guard.
- Unit tests cover exhaustion, unchanged state after failure, exact release, and distinct arithmetic overflow.
- Generated bulk create/update handlers reserve aggregate item counts, while idempotent mutation completion reserves cached response bytes; these are two independent production consumers.
- Both consumers hold RAII reservations only for the bounded work they protect, so errors, unwind, and future cancellation release their own amounts.

### 20. Standard API Problem Responses

Status: Completed.

#### Planned design

Define one bounded, non-secret error contract shared by generated handlers, administration handlers, clients, and OpenAPI. It should carry a stable machine-readable kind, HTTP status, safe detail, request identifier, and optional field violations. Internal SQLx and cryptographic errors must remain error sources and must not enter the response body.

#### Acceptance evidence

- Every generated error status declares the common schema in OpenAPI.
- Runtime responses validate against the documented schema.
- Snapshot tests keep status-to-problem-kind mappings stable.
- Secret and internal-error regression tests prove redaction.

#### Implementation evidence

- `frontend_contract::ApiProblem` provides bounded safe detail, stable kind/status mapping, optional request identifiers, and bounded field violations.
- Generated and administration handlers normalize non-success responses to `application/problem+json`; SQLx and cryptographic source errors are not serialized.
- Generated OpenAPI error responses all reference the common schema, including idempotency and optimistic-concurrency statuses.
- Runtime negative-contract tests deserialize every generated operation's rejection as `ApiProblem`; mapping and redaction unit tests keep internal details out of bodies.

### 21. Frontend Authentication Keep-Alive State Machine

Status: Completed.

#### Source evidence

- Mapcam's `AuthSessionKeepAlive` prevents overlapping refresh attempts and distinguishes refreshed, temporarily failed, rejected, and missing-session outcomes.

#### Planned design

Use a small state machine in `server_admin_frontend` so simultaneous `401` responses do not start multiple refresh requests. A rejected refresh clears local authentication state; a temporary transport failure observes a bounded retry schedule. The state owner must remain local to the frontend application and must not use process-global mutable state.

#### Acceptance evidence

- Multiple simultaneous callers share one in-flight refresh.
- Rejection clears session state without a retry loop.
- Temporary failure schedules at most one subsequent attempt.
- Deterministic tests use injected instants rather than wall-clock sleeps.

#### Implementation evidence

- `AdminRoute::Refresh` now provides the typed `/auth/refresh` client contract.
- The WASM client responds to an authenticated request's `401` by refreshing and retrying the original request exactly once; refresh rejection is returned without a retry loop.
- Application-local `Arc<RwLock<_>>` coordination and one-shot waiters make simultaneous callers share one refresh result without holding a lock across `.await`.
- Rejection is terminal and redirects to sign-in; a temporary failure enters one bounded retry deadline instead of looping.
- Deterministic injected-instant tests cover join, rejection, and retry scheduling; host tests and `cargo check --target wasm32-unknown-unknown` pass.

### 22. Negative HTTP Contract Suite

Status: Completed.

#### Source evidence

- Mapcam integration tests cover unknown JSON fields, malformed payloads, wrong content types, unsupported methods, route coverage, and OpenAPI negative cases.

#### Planned design

Generate negative cases from the same route descriptors as positive contract tests. Cover unknown fields, duplicate singleton headers, wrong content type, oversized bodies, malformed JSON, unsupported methods, invalid authentication material, and idempotency conflicts where enabled.

#### Acceptance evidence

- Every generated operation participates without handwritten route enumeration.
- Actual status and problem body match OpenAPI.
- Tests require no database when rejection occurs before handler execution.
- Unknown fields remain rejected for generated mutation payloads.

#### Implementation evidence

- The DB-independent runtime matrix iterates `TblExampleRouteContract::ALL`, so every enabled generated operation is rejected and checked without handwritten route enumeration.
- It covers missing and duplicate singleton headers, wrong content type, malformed JSON, oversized bodies, missing optimistic revision, and unsupported methods before database access.
- Every checked response has its expected status and a deserializable `application/problem+json` body, while generated OpenAPI tests require the common problem schema for every error status.
- Generated payload tests reject unknown fields and invalid read-filter shapes.

### 23. Migration and Seed Invariants

Status: Completed.

#### Source evidence

- Mapcam tests migration idempotency, seed integrity, bootstrap behavior, and database initialization separately from API behavior.

#### Planned design

Extend database verification with an empty-database migration scenario and an upgrade scenario from the supported baseline. Verify required administrator roles and permissions, foreign keys, unique constraints, and important indexes. Bootstrap commands must use idempotent SQL and preserve explicitly changed administrator data.

#### Acceptance evidence

- Fresh migration and supported upgrade both reach the expected schema version.
- Re-running bootstrap creates no duplicates and does not reset user-managed secrets.
- Required role/permission relationships contain no dangling references.
- Destructive checks remain protected by the existing test-database guard.

#### Implementation evidence

- The administration PostgreSQL integration flow invokes schema preparation twice.
- It verifies repeated bootstrap creates no second administrator and preserves the original password hash.
- It checks administrator-role and role-permission link tables for dangling references.
- `postgresql_migrations_cover_fresh_and_supported_baseline_upgrade` applies all migrations in an empty isolated schema and independently upgrades a schema from migrations 1–3 to migration 4.
- Both scenarios reach version 4; repeated preparation remains idempotent, and destructive checks execute only against the guarded local test database workflow.

### 24. Safe File Staging, Deferred

Mapcam's bounded staging directories, path containment checks, temporary writes, atomic rename, and rollback-error composition are useful only once this workspace has a real upload, import, or export consumer. Do not add the abstraction speculatively. When triggered, place shared filesystem lifecycle logic in an existing shared runtime crate and keep file-domain policy in the consuming crate.

## Second-Wave Implementation Order

1. Add escaped text match modes because they are local to `gen_wh_flts_src` and have a small, testable surface.
2. Design idempotency storage and transaction boundaries, then enable it for one generated create operation before generalizing it.
3. Add batched cleanup for existing administration session and rate-limit tables.
4. Split liveness and readiness and expose the existing supervised-task state where appropriate.
5. Audit and bound metric labels before adding more metrics.
6. Add optimistic concurrency as an opt-in table capability without changing existing generated update contracts.
7. Generate the negative contract matrix from route descriptors.
8. Add shared resource budgets only after bulk CRUD and another measured consumer need aggregate limits.
9. Introduce the common problem response incrementally while preserving existing public error contracts until an explicit migration is approved.
10. Add frontend refresh coordination and migration/seed invariant coverage independently.

Each numbered candidate should be implemented as a separate change. Adding new dependencies, changing generated public APIs, or changing existing wire error formats requires explicit approval under the workspace rules.
