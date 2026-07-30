## Summary

Describe the behavior and compatibility impact.

## Administrator checklist

- [ ] Public behavior and API compatibility are preserved or documented.
- [ ] Domain wrappers and architecture boundaries are preserved.
- [ ] New or changed migrations have fresh-schema and upgrade coverage.
- [ ] Permission catalog, seed reconciliation, and authorization tests are updated.
- [ ] Security-sensitive mutations and resource identifiers have audit coverage.
- [ ] Typed routes and OpenAPI/contract snapshots are updated.
- [ ] Session invalidation behavior is verified where identities or RBAC change.
- [ ] Browser acceptance covers affected administrator flows.
- [ ] Operations, upgrade, and feature-matrix documentation is current.
- [ ] `cargo fmt`, Clippy with warnings denied, and code-style tests pass.
