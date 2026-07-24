# Type catalog

The Rust source is the canonical type catalog. This document intentionally does not mirror every
type, visibility, module, line number, or crate count: such a snapshot becomes stale as soon as the
source changes.

Use Cargo for the current workspace package graph:

```bash
cargo metadata --no-deps --format-version 1
```

Use repository search for the current declarations:

```bash
rg -n '^\s*(pub(\([^)]*\))?\s+)?(struct|enum|trait|union)\s+' --glob '*.rs'
```

The enforced design rules live in [`AGENTS.md`](../AGENTS.md), while automated architecture and
domain-wrapper checks live in [`tests/src/code_style`](../tests/src/code_style). The wrapper design
rationale and examples are documented in [`type-wrappers.md`](type-wrappers.md).

When a durable catalog is needed for external publication, generate it from the same commit being
published; do not edit a second hand-maintained inventory.
