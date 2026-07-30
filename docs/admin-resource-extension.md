# Adding an administrator resource

An application resource should follow the repository's existing sources of truth. Avoid creating a
second route, permission, or schema inventory.

## 1. Own the domain and database schema

Add the table through an ordered migration in the service that owns the resource. Use repository
domain wrapper types for identifiers and values. The migration remains authoritative for physical
constraints, indexes, defaults, triggers, and foreign keys.

Do not expose secret columns, internal tokens, password hashes, or unrestricted JSON through a
generated administrator contract.

## 2. Choose the resource boundary

Use generated PostgreSQL CRUD when the resource is a conventional table whose operations map
directly to create, read, update, and delete. Follow the table descriptors in
`server_admin/src/generated_tables.rs`.

Use custom repository and handler logic when an operation:

- spans several aggregates;
- has workflow or approval semantics;
- needs a transaction beyond one generated operation;
- performs a privileged action such as credential or session mutation.

Keep shared mechanics in shared crates and application semantics in the owning service.

## 3. Declare authorization once

Add each administrator permission to `server_admin_contract::AdminPermission`. Its wire value uses
`resource:operation`. Migration preparation reconciles the typed catalog with PostgreSQL.

Attach the permission requirement to the typed route contract. Test both an allowed actor and an
authenticated actor without the permission. Hiding navigation is not authorization; direct API
access must independently reject the request.

## 4. Declare transport contracts and routes

Define bounded request and response values in the owning contract crate. Add a typed route with:

- method and path;
- request body kind;
- success status;
- authentication/permission requirement;
- mutation classification;
- error policy;
- coverage obligations;
- stable OpenAPI operation identifier.

Register the route through the owned route catalog so Axum, OpenAPI, clients, and validation tests
consume the same metadata.

## 5. Add repository logic

Keep SQL in the owning repository module. Bind values rather than interpolating them and use the
shared query-fragment types for generated filters. Preserve optimistic concurrency or idempotency
contracts for mutations where the route declares them.

Every mutation must write an audit action containing the actor, resource type, bounded resource
identifier, request identifier, success state, and non-secret details.

## 6. Add the administrator page

Add page metadata to the `AdminPage` catalog when the resource owns a dedicated page. The catalog
drives frontend paths, navigation, client mode, and route association.

For a registered read-only table, add its table specification to `AdminDataTable` and connect it to
the generated table catalog. For a custom page, keep API paths derived from typed routes and reuse
the shared loading, problem, pagination, and mutation behavior.

Verify keyboard navigation, labels, focus behavior, empty state, loading state, validation errors,
forbidden state, and narrow viewport behavior.

## 7. Required tests

Add:

1. wrapper boundary and serialization unit tests;
2. route/OpenAPI contract tests;
3. permission-denied and authentication tests;
4. repository tests against disposable PostgreSQL;
5. migration conformance and supported-upgrade tests;
6. successful and failed mutation audit assertions;
7. browser acceptance coverage for the main user flow;
8. pagination, filter, sort, body-limit, and concurrency tests where applicable.

Regenerate checked projections and run:

```bash
cargo run -p workspace_scaffold -- generate sync
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p tests code_style
cargo test --workspace
cargo run -p workspace_test_runner -- database
```

## Review checklist

- The resource has one owning service.
- Its schema, route, permission, and page each have one authoritative declaration.
- Public values are bounded domain types.
- Secret/internal fields cannot enter generated responses.
- Direct API authorization is tested.
- Mutations are transactional, auditable, and replay-safe where required.
- Fresh-schema and upgrade paths are verified.
- Browser behavior is accessible and deep-link safe.

