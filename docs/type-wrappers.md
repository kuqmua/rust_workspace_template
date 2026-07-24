# Type wrapper policy

The Rust source tree is the only inventory of repository type wrappers. This document deliberately
does not duplicate the list: tuple structs are added, renamed, and removed in code, so a handwritten
table becomes stale immediately.

The `tests` crate inspects the workspace Rust AST and enforces the wrapper policy:

- tuple wrapper fields stay private;
- construction goes through `From` or `TryFrom`;
- deserialization also crosses the same conversion boundary;
- dynamically growing raw values use bounded repository types;
- domain boundaries expose repository wrappers instead of primitives or external crate types.

Run the policy checks with:

```bash
cargo test -p tests code_style
```

The relevant checks live in `tests/src/code_style/domain_type_policy.rs` and
`tests/src/code_style/source_policy.rs`. Their diagnostics contain the current file, item, and
violation, making the checked source and the reported inventory one and the same.
