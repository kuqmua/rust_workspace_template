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
| Service configuration keys | Config derive field descriptors | `.env.example`, Compose and Kubernetes checks |
| PostgreSQL physical schema | Ordered SQL migrations | Applied database and introspected conformance checks |
| Domain validation | Repository domain wrapper type | API validation and database policy tests |
| Administrator permissions | `AdminPermission` wire enum | Seed reconciliation, authorization and OpenAPI |

## Generated deployment projections

After changing `deploy/services.toml`, update checked-in projections:

```bash
cargo run -p workspace_scaffold -- deployment sync
```

CI verifies that the projections are current:

```bash
cargo run -p workspace_scaffold -- deployment check
```

Content between `BEGIN GENERATED` and `END GENERATED` markers is owned by the
service catalog and must not be edited manually.

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
