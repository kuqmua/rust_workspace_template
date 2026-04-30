# Rust Workspace Template

Production-oriented Rust workspace template with strict linting, deterministic tests, and CI gates that scale from pull requests to nightly full verification.

## Goals

- Keep dependencies controlled: workspace-level declarations, no implicit defaults.
- Keep quality predictable: strict lint profile, deterministic tests, reproducible local verification order.
- Keep developer onboarding fast: copy-paste commands and cargo aliases for daily workflows.

## Workspace layout

- `server`: minimal entrypoint crate.
- `tests`: workspace-level policy and meta tests.

## Quick start

1. Install nightly toolchain:

```bash
rustup toolchain install nightly
```

2. Verify toolchain setup:

```bash
cargo +nightly --version
```

3. Run local checks in required order:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Developer shortcuts

Cargo aliases are configured in `.cargo/config.toml`:

- `cargo workspace-format`
- `cargo workspace-lint`
- `cargo workspace-test`
- `cargo workspace-check-no-default-features`
- `cargo workspace-doc`
- `cargo workspace-nextest`
- `cargo workspace-hack`
- `cargo workspace-deny`
- `cargo workspace-udeps`
- `cargo workspace-verify` (runs `fmt -> clippy -> test` in required order)

Extended local validation set (parity with CI lanes):

```bash
cargo workspace-nextest
cargo workspace-hack
cargo workspace-deny
cargo workspace-udeps
```

## Production build defaults

Release profile is hardened in workspace root `Cargo.toml`:

- `lto = "fat"`
- `codegen-units = 1`
- `panic = "abort"`
- `strip = "symbols"`

These settings optimize for smaller and more predictable production binaries.

## CI and governance

- Main CI: `.github/workflows/ci.yml`
- All CI jobs run on pull requests targeting `main` and pushes to `main` when Rust or CI files changed.
- Contribution guide: `CONTRIBUTING.md`
- Release process: `RELEASE.md`
- Security policy: `SECURITY.md`
- Changelog template: `CHANGELOG.md`

## Policy tests

All tests live in `tests/src/lib.rs` and enforce template rules, including:

- workspace dependencies and exact version pinning
- no forbidden runtime shortcuts (`unwrap`, `todo!`, source-dropping `map_err`)
- deterministic testing constraints
- hardened release profile and `workspace-verify` command order
- nightly toolchain contract (`rust-toolchain.toml` must stay on `channel = "nightly"`)
- no debug print macros outside entrypoint
- workflow permissions, concurrency, and timeouts
- GitHub Actions pinned by full commit SHA

## Extension rules

- Add crates.io dependencies only in `[workspace.dependencies]`.
- Use `*.workspace = true` in crate `Cargo.toml` files.
- Disable default features unless required by a concrete use case.
- Preserve external contracts unless changes are explicitly requested.
