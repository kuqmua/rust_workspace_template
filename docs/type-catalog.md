# Type catalog

The Rust source is the canonical type catalog. The generated
[`domain-types.md`](domain-types.md) snapshot lists every declaration recognized by the repository's
domain-type policy, including its explicitly supported declaration macros.

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

Regenerate the declaration snapshot from the same commit being published; do not edit its entries
by hand.
