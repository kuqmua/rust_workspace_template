# Integer identifier migration plan

Status: in progress; stage 1 implementation is underway.

## Objective

Convert the remaining database identifiers to positive PostgreSQL `BIGINT` values,
one table at a time. Complete and verify each table before starting the next one.
Update the entire path from database schema through domain types, repository SQL,
API contracts, frontend behavior, and tests.

Use direct initial schema definitions and service initialization. Do not introduce
`ALTER TABLE`, compatibility columns, UUID-to-integer mapping migrations, or data
conversion backfills. Existing databases using the previous migration checksums
must be recreated before the updated migrations can run.

This document is an implementation plan. Writing it does not execute the changes
or recreate any database.

## Current state and target state

| Table | Identifier | Current type | Target |
| --- | --- | --- | --- |
| `refresh_tokens` | `id` | `UUID` | Generated positive `BIGINT` primary key |
| `audit_log` | `resource_id` | Nullable `TEXT` | Nullable positive `BIGINT` resource identifier |
| `audit_log` | `request_id` | Nullable `UUID` | Nullable positive `BIGINT` request identifier |
| `pg_table_idempotency` | Primary key | Composite text key | Generated positive `BIGINT` column named `id` |
| `pg_table_idempotency` | `actor` | `TEXT` containing a user ID in administrator requests | Positive `BIGINT` actor identifier |
| `pg_table_idempotency` | `idempotency_key` | `TEXT` | Positive `BIGINT` operation key |
| `system_settings` | `id` | `SMALLINT` | `BIGINT`, retaining the singleton value `1` |

`access_sessions.id` and `refresh_tokens.session_id` already use `BIGINT`.
`audit_log.id` already uses `BIGINT`. HTTP methods, route paths, logins, resource
names, token hashes, and token secrets remain in their appropriate non-integer
types. They are not database record identifiers.

The initial schema is owned by
`server_admin_migrations/0001_admin_schema.sql`; permission seed data is owned by
`server_admin_migrations/0002_admin_rules.sql`. The idempotency table is currently
created by the shared `pg_table` initialization owner, so its definition must be
changed there rather than duplicated in the administrator migrations.

## Rules for every stage

1. Inspect the current worktree and affected consumers before editing. Preserve
   unrelated changes and completed behavior from previous tasks.
2. Change only the current table and dependencies required by that table.
3. Use validated repository domain wrappers around positive `i64` values.
   Preserve private fields, generated getters, and existing architecture layers.
4. Keep shared behavior in existing shared crates. Do not add crates or external
   dependencies without an explicit request.
5. Use PostgreSQL identity columns for record IDs. Use database sequences for
   identifiers that must be allocated before their records exist.
6. Update serialization, typed routes, generated table descriptors, filters,
   OpenAPI schemas, SQL bind types, and frontend consumers together.
7. Review intentional API and schema snapshot changes explicitly. Do not update
   snapshots simply to hide a failing assertion.
8. Use disposable, provisioned databases for stage verification. Recreate those
   databases instead of applying incremental schema patches.
9. Keep the local development database running until the final reset. If a stage
   needs a live preview, use a separate provisioned database and server port.
10. Record verification evidence and the stage outcome before proceeding.

## Stage 1: refresh_tokens

### Schema and runtime changes

- Define `refresh_tokens.id` as `BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY`
  in the initial schema.
- Keep `session_id` numeric and preserve existing session linkage and cleanup
  behavior. Do not add a foreign key that changes existing expiry semantics.
- Stop generating a UUID for the refresh-token record ID. Retrieve the ID with
  `INSERT ... RETURNING id` wherever the caller needs it.
- Keep the random refresh-token secret and its stored hash independent of the
  sequential record ID. An integer ID must never become an authentication secret.
- Update refresh-token domain wrappers, SQL row extraction, revocation handlers,
  audit callers, and generated PostgreSQL descriptors.

### Contracts and frontend

- Replace UUID-shaped `AdminRefreshTokenId` validation with positive integer
  validation and numeric JSON serialization.
- Update detail URLs, route parameter parsing, numeric filters, form values,
  generated table consumers, and OpenAPI schemas.
- Inspect generic UUID filter validation before changing it: it currently uses
  the refresh-token identifier wrapper, but unrelated UUID fields must remain
  valid until their own stage is complete.
- Replace UUID fixtures with deterministic positive integer fixtures. Reject
  zero, negative values, overflow, and UUID strings at the numeric boundaries.

### Completion gate

- Fresh initialization creates an identity-backed `BIGINT` refresh-token ID.
- Sign-in, refresh, token rotation, replay rejection, session revocation, and
  explicit refresh-token revocation preserve their existing behavior.
- Refresh-token listing, detail navigation, and numeric filtering pass.
- Token secrets remain random and are not exposed through diagnostics.
- Relevant unit, provisioned integration, and browser tests pass; contract and
  schema snapshots match the reviewed result.

## Stage 2: audit_log

Perform both identifier changes within this table before moving to idempotency.

### Resource identifiers

- Define `resource_id` as nullable `BIGINT` in the initial schema.
- Keep the resource kind in its existing field. The resource kind and numeric ID
  together identify the target; do not add a foreign key to a single resource
  table for this polymorphic reference.
- Make `AdminAuditResourceId` return a typed numeric value instead of formatting
  IDs as strings. Use `1` for the system-settings singleton.
- Use `NULL` for events without a specific resource. Do not use zero or a textual
  sentinel to represent an absent target.
- Update audit inserts, JSON details containing target IDs, generated descriptors,
  numeric filters, and frontend rendering. Remove numeric fields from text-only
  search configuration where required by the generated query contract.

### Request identifiers

- Define `request_id` as nullable `BIGINT` and create a PostgreSQL `BIGINT`
  sequence in the initial schema for server-issued request identifiers.
- Allocate an identifier once per incoming HTTP request and propagate it through
  request extensions, tracing, audit writes, and the response `X-Request-ID`.
- Represent the HTTP header as decimal text while keeping the internal domain
  value numeric. JSON API representations use numbers.
- Use an injected asynchronous provider in the appropriate existing shared
  owner. Preserve dependency direction between HTTP infrastructure and the
  application; do not make the HTTP crate depend on administrator workflows.
- Use the database sequence across restarts and multiple server instances. Do
  not introduce process-global counters, leaked state, or blocking database I/O.
- Define incoming header handling explicitly: the server issues its own unique
  numeric ID rather than trusting a client to choose one. Update proxy and
  correlation tests for this intentional contract change.
- Define provider failure behavior explicitly and test it. Do not silently fall
  back to a UUID or reuse an identifier. Include the additional database call in
  the latency and database-availability review.
- Use deterministic provider fixtures in unit tests; do not construct clients or
  depend on an actual database in those tests.

### Completion gate

- Resource and request identifiers are stored and returned as integers.
- Events without a target or an available request context retain valid nullable
  semantics.
- One HTTP request uses the same numeric ID in its response, tracing, and audit
  records, including error responses.
- IDs remain unique across concurrent requests, restarts, and service instances.
- Audit append-only restrictions, authorized cleanup, numeric filtering, and
  resource-kind distinctions remain correct.

## Stage 3: pg_table_idempotency

### Schema and domain changes

- Change the shared schema owner to create an identity-backed `BIGINT` primary
  key column named `id` directly.
- Convert `actor` to `BIGINT`. Administrator authentication currently supplies
  its user ID as decimal text; pass the validated numeric wrapper directly.
- Audit all other consumers of the shared actor type before narrowing it. Define
  explicit numeric mappings for any additional supported actor kinds; do not
  parse arbitrary strings or silently merge distinct actors.
- Convert `idempotency_key` to `BIGINT`, using a shared database sequence for
  allocation before a mutation begins.
- Keep `http_method` and `route_path` as text and retain a unique constraint on
  `(actor, http_method, route_path, idempotency_key)`. The new row primary key does
  not replace the operation lookup identity.
- Preserve request hashes, pending/completed state, response replay, retention,
  transaction ownership, and cleanup indexes.

### Key issuance and consumers

- Introduce a typed, authenticated operation-key issuance route in the existing
  route registry, with a concrete numeric response contract.
- Update callers of `new_pg_table_idempotency_key` and mutation consumers to use
  server-issued keys. Remove the client-side UUID allocation path after all
  consumers have been converted.
- Allocate one key per logical operation. Keep that key for retries after a
  timeout, connection failure, or uncertain response; allocate a new key only for
  a new operation.
- Send the key as decimal text in its HTTP header and retain a numeric wrapper
  internally. Enforce positive range validation at the HTTP adapter boundary.
- Authorize mutations using the authenticated actor. Knowing a numeric key must
  not grant access to another user's stored response.
- Update begin, completion, release, rollback, replay, and cleanup queries in the
  shared owner without introducing duplicated administrator-specific behavior.

### Completion gate

- The row ID, actor ID, and operation key are all numeric.
- Repeating the same operation returns its stored result without another mutation.
- Concurrent requests with the same scope preserve existing coordination rules.
- Reusing a scope with a different request body is rejected.
- Different actors, methods, or routes remain isolated.
- Failed transactions, abandoned operations, retries, and cleanup preserve their
  existing guarantees.
- Key issuance, header validation, and frontend retry behavior pass integration
  and browser checks.

## Stage 4: system_settings

- Define `system_settings.id` as `BIGINT` in the initial schema.
- Preserve its default value `1`, singleton constraint, and initial seeded row.
- Update generated PostgreSQL types, SQL bindings, wrappers where necessary,
  table schemas, and snapshot expectations.
- Keep public settings and administrator settings behavior unchanged.
- Verify fresh initialization creates exactly one row, updates preserve its ID,
  and additional singleton rows are rejected.

## Verification workflow

For each stage, run focused tests covering the changed behavior first. Run the
repository gates before accepting the stage. Use the workspace runner for the
code-style suite exactly once per stage; exclude that suite from ordinary
workspace tests.

```bash
cargo fmt
cargo run -p workspace_test_runner -- static
cargo test --workspace --exclude tests_code_style_rust
```

The static runner includes the code-style suite and Clippy. Confirm its Clippy
command covers `--all-targets --all-features -- -D warnings`; run that exact
command separately if the runner no longer provides the required coverage.

Run database-backed ignored tests only after provisioning a disposable database
and configuring `DATABASE_URL` for it:

```bash
cargo run -p workspace_test_runner -- database
```

Run existing relevant Playwright scenarios against a separate disposable browser
database. Provide `BROWSER_ACCEPTANCE_DATABASE_URL` with the required
`_browser_test` suffix and enable the full scenarios when checking refresh and
revocation. Add meaningful regression coverage for new numeric request IDs and
operation-key issuance. Never point disposable test scripts at the working
development database.

Record commands, outcomes, reviewed snapshot differences, and unresolved failures
for each stage. Do not mark a stage complete while any required check is failing.

## Final local database recreation and initialization

1. Confirm the target database and running server from the effective configuration.
2. Build the updated server and administrator initialization binaries before
   stopping the running server.
3. Create a private backup of the working development database.
4. Stop the existing server and confirm its listening socket has been released.
5. Drop and recreate only the authorized development database, retaining its
   configured owner. Do not reset unrelated databases or PostgreSQL roles.
6. Run the service with `SVC_MODE=migrate` against the recreated database.
7. Create the administrator through the existing initialization command, using a
   private password file and the credentials documented in
   `ADMIN_LOGIN_AND_PASSWORD.md`. Remove the temporary password file afterward.
8. Run migration mode again to verify repeat initialization is safe and does not
   duplicate seeded rows.
9. Start a single server with `SVC_MODE=serve` on its configured port.
10. Verify database column types, identity definitions, sequence allocation,
    constraints, indexes, migration history, seeded permissions, and administrator
    role assignments from the actual database catalog.
11. Verify the health route, documented administrator login, integer detail URLs,
    refresh and revocation behavior, audit correlation, settings, and idempotent
    mutation retries against the updated server.
12. Remove disposable verification databases and update the local initialization
    documentation. Old sessions and data are intentionally absent after recreation.

## Final acceptance checklist

- [ ] Stage 1: refresh tokens completed and verified.
- [ ] Stage 2: audit resource and request identifiers completed and verified.
- [ ] Stage 3: idempotency row, actor, and operation identifiers completed and verified.
- [ ] Stage 4: settings identifier standardized and verified.
- [ ] Initial schema definitions contain the final types without incremental alterations.
- [ ] Numeric identifiers use validated wrappers and numeric JSON contracts.
- [ ] Authentication secrets remain independent of sequential record identifiers.
- [ ] Reviewed API and PostgreSQL schema snapshots match the implementation.
- [ ] Required Rust, database, and browser checks pass.
- [ ] Working database recreated and initialized through migrations and the service.
- [ ] Updated server runs and documented administrator login succeeds.

## Execution record

| Stage | Status | Verification evidence |
| --- | --- | --- |
| Refresh tokens | In progress | Numeric schema and contracts implemented; verification pending |
| Audit log | Not started | Pending |
| Idempotency | Not started | Pending |
| System settings | Not started | Pending |
| Final database initialization | Not started | Pending |
