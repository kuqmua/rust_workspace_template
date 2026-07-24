# Repository sources of truth

Every repository concept has one authoritative declaration. Other representations
must be generated from it or checked against it. A conformance test protects an
external representation; it does not make two independently edited declarations
authoritative.

| Concept | Authoritative source | Derived or checked consumers |
| --- | --- | --- |
| Workspace dependencies | `Cargo.toml` `[workspace.dependencies]` | Workspace crate manifests |
| Rust toolchain | `rust-toolchain.toml` | Local builds, CI setup action, Docker builds |
| HTTP routes | Typed route and route-family contracts | Axum routers and OpenAPI |
| Admin pages | `AdminPage` page catalog metadata | CSR navigation, SSR navigation and page routing |
| Admin settings presentation | `AdminSetting` catalog | CSR and SSR form fields and clear semantics |
| Service inventory | `deploy/services.toml` | CI image builds and release matrix |
| Service configuration schema | Config derive field descriptors | `.env.example`, Compose and Kubernetes checks |
| PostgreSQL physical schema | Ordered SQL migrations | Applied database and generated current-schema snapshot |
| Domain validation | Repository domain wrapper type | API validation and database policy tests |
| Administrator permissions | `AdminPermission` wire enum | Seed reconciliation, authorization and OpenAPI |

## Generated projections

After changing a generated deployment or reviewed code contract, update all static
checked-in projections:

```bash
cargo run -p workspace_scaffold -- generate sync
```

This updates service matrices, generated Compose/Kubernetes sections,
configuration examples, and the readable contract/error inventory snapshots. CI
verifies that all static projections are current:

```bash
cargo run -p workspace_scaffold -- generate check
```

Content between `BEGIN GENERATED` and `END GENERATED` markers and files carrying a
`GENERATED ... DO NOT EDIT` header are owned by their source catalog and must not
be edited manually. The narrower `deployment sync|check` command remains available
for deployment-only tooling.

The PostgreSQL snapshot additionally requires a migrated database. Refresh it only
after adding an ordered migration:

```bash
UPDATE_ADMIN_CURRENT_SCHEMA_SNAPSHOT=1 \
  cargo test -p server_admin --test admin_api \
  postgresql_migration_creates_complete_schema -- --ignored --exact
```

The same test without `UPDATE_ADMIN_CURRENT_SCHEMA_SNAPSHOT` verifies the snapshot.
It runs in the CI database suite after migrations are applied to an empty schema.

## Ownership rule

Strings, SQL and metadata belong to the crate or module that owns their meaning.
They belong in a shared crate only when multiple independent owners use the same
semantic value. Coincidentally equal text is not shared domain logic.

SQL migrations own the physical database schema. Rust table descriptors describe
the generated CRUD boundary and are checked against a migrated database; they do
not duplicate defaults, indexes, checks, functions or triggers.

## Review checklist

For every new field, route, service, table or permission:

1. Identify its authoritative catalog.
2. Change that catalog once.
3. Regenerate projections, when applicable.
4. Run the conformance checks.
5. Reject a change that introduces another independently editable inventory.
6. Keep generated files read-only in review; update them through `generate sync` or
   the migration snapshot command.
