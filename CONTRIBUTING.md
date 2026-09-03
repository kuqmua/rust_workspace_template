# Contributing

Keep service ownership and public contracts explicit. Shared mechanics belong in a shared crate only
after at least two consumers demonstrate the same requirement; service domain logic remains with its
owner.

Before submitting a change, run:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p tests_code_style_rust
cargo test --workspace --exclude tests_code_style_rust
```

Database changes require forward migrations and fresh-schema plus supported-upgrade tests. Public
HTTP or event changes require compatibility review and corresponding contract tests.
Run the database-backed ignored suite in a provisioned environment with:

```bash
cargo run -p workspace_test_runner -- database
```
