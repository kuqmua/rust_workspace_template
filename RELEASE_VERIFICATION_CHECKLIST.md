# Release verification checklist

The workspace maintainers own every check below. CI timeouts are defined in `.github/workflows/ci.yml`; a timeout is a failed gate, not an allowed skip.

## Every pull request and push

- `cargo metadata --locked --format-version 1 --no-deps`
- `cargo fmt --all -- --check`
- `cargo clippy --locked --all-targets --all-features -- -D warnings`
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --all-features --no-deps`
- `cargo test --locked -p tests code_style`
- `cargo openapi-contract-suite`
- `cargo run --locked -p workspace_test_runner -- database` with a guarded loopback URL whose database name is explicitly test-only
- `cargo deny check` and `cargo audit`
- `cargo hack check --workspace --feature-powerset --depth 1 --locked`
- Gitleaks `v8.30.1`, downloaded from the upstream release and verified with its published SHA-256 checksum

## Pull requests that change public APIs

CI compares `newtype`, `gen_pg_tbl_src`, and `server_runtime` with the pull request base revision using `cargo semver-checks`. Add another package to the same gate when it acquires a supported public compatibility contract.

Local equivalent, with the desired baseline revision substituted:

```bash
cargo install --locked cargo-semver-checks
cargo semver-checks --package newtype --baseline-rev origin/main
cargo semver-checks --package gen_pg_tbl_src --baseline-rev origin/main
cargo semver-checks --package server_runtime --baseline-rev origin/main
```

## Weekly and before a release

The scheduled maintenance job runs unused-dependency analysis. Run the same check locally before a release:

```bash
cargo install --locked cargo-udeps
cargo udeps --workspace --all-targets --all-features --locked
```

Before tagging a release, run the complete local verification alias:

```bash
cargo verify-strict
```

Feature changes require the feature-matrix gate. Generated public API changes require both the OpenAPI contract suite and the semver gate. Dependency changes require metadata, deny, audit, and feature-matrix gates.
