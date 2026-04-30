# CI

CI is defined in `.github/workflows/ci.yml`.

## Triggers

1. `push` to `main`.
2. `pull_request` targeting `main`.

For the same branch, an older workflow run is cancelled when a newer run starts.

## Global Settings

All jobs use these defaults:

1. `ubuntu-26.04`.
2. `20` minute timeout per job.
3. Minimal permissions: `contents: read`.
4. `CARGO_INCREMENTAL=0`.
5. `CARGO_NET_RETRY=10`.
6. `CARGO_NET_TIMEOUT=60`.
7. `RUSTFLAGS="-D warnings -D unreachable_pub -D dead_code"`.
8. `RUSTDOCFLAGS="-D warnings"`.
9. `RUST_BACKTRACE=short`.

## Detect Changes

The first job is `changed-files`. It determines whether checks need to run.

Rust-related changes:

```text
**/*.rs
**/Cargo.toml
**/Cargo.lock
rust-toolchain.toml
.cargo/config.toml
.config/nextest.toml
deny.toml
```

CI-related changes:

```text
.github/workflows/**
```

Every check below runs for both `push` and `pull_request` events when Rust or CI files changed:

1. `fmt`.
2. `clippy`.
3. `test`.
4. `no-default-features`.
5. `taplo`.
6. `typos`.
7. `actionlint`.
8. `build`.
9. `doc`.
10. `msrv`.
11. `audit`.
12. `deny`.
13. `machete`.
14. `check-semver`.
15. `hack`.
16. `udeps`.
17. `llvm-cov`.

## Formatting

The `fmt` job runs:

```bash
cargo fmt --check
```

It verifies that Rust code is already formatted. CI does not rewrite files and fails when formatting differs.

## Clippy

The `clippy` job runs on two toolchains:

1. `stable`.
2. `nightly`.

Command:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

It checks the whole workspace, all targets, and all features. Every warning is treated as an error.

## Tests

The `test` job depends on `clippy`, so tests only start after clippy succeeds.

Main tests:

```bash
cargo nextest run --all-targets --all-features --profile ci
```

Doc tests:

```bash
cargo test --doc --all-features
```

Nightly full test harness parity:

```bash
cargo +nightly test --workspace --all-targets --all-features
```

## No Default Features

The `no-default-features` job runs on:

1. `stable`.
2. `nightly`.

Commands:

```bash
cargo check --workspace --all-targets --no-default-features
cargo test --workspace --all-targets --no-default-features
```

It verifies that the workspace builds and tests without default features.

## Taplo

The `taplo` job runs:

```bash
taplo fmt --check
```

It checks TOML formatting.

## Typos

The `typos` job runs:

```bash
typos
```

It checks spelling in code and text files.

## Actionlint

The `actionlint` job checks GitHub Actions workflows for syntax errors and invalid expressions.

## Build

The `build` job runs for every CI run with Rust or CI changes.

Matrix:

1. `stable`, `dev`.
2. `stable`, `release`.
3. `nightly`, `dev`.
4. `nightly`, `release`.

Dev command:

```bash
cargo build --all-targets --all-features
```

Release command:

```bash
cargo build --all-targets --all-features --release
```

For `nightly + release`, CI also uploads this artifact path:

```text
target/release/*server*
```

The artifact is named `release-binaries` and is retained for 7 days.

## Docs

The `doc` job runs for every CI run with Rust or CI changes.

Command:

```bash
cargo doc --all-features --no-deps --document-private-items
```

Documentation builds with `RUSTDOCFLAGS="-D warnings"`, so documentation warnings fail CI.

## MSRV

The `msrv` job runs for every CI run with Rust or CI changes.

Command:

```bash
cargo check --all-features
```

The workspace declares `rust-version = "1.85"`.

## Security Audit

The `audit` job runs for every CI run with Rust or CI changes.

It uses `rustsec/audit-check` and checks dependencies for known RustSec advisories.

## Cargo Deny

The `deny` job runs for every CI run with Rust or CI changes.

Command:

```bash
cargo deny check advisories bans licenses sources
```

It checks:

1. Security advisories.
2. Banned dependencies.
3. Licenses.
4. Dependency sources.

## Cargo Machete

The `machete` job runs for every CI run with Rust or CI changes.

Command:

```bash
cargo machete
```

It detects unused dependencies.

## Semver Check

The `check-semver` job runs on pull requests and pushes when Rust or CI files changed.

It uses `obi1kenobi/cargo-semver-checks-action` and checks for semver-breaking changes in public APIs.

## Cargo Hack Feature Matrix

The `hack` job runs for every CI run with Rust or CI changes.

Command:

```bash
cargo hack check --workspace --feature-powerset --no-dev-deps
```

It checks feature flag combinations.

## Cargo Udeps

The `udeps` job runs for every CI run with Rust or CI changes.

Command:

```bash
cargo +nightly udeps --workspace --all-targets --all-features
```

It detects unused dependencies through `cargo-udeps`.

## Coverage

The `llvm-cov` job runs for every CI run with Rust or CI changes.

Command:

```bash
cargo llvm-cov --workspace --all-features --all-targets --summary-only
```

It prints a test coverage summary. CI currently reports the summary without enforcing a coverage threshold.

## Final Gate

The `ci-success` job always runs and collects the results of all jobs:

1. `fmt`.
2. `clippy`.
3. `test`.
4. `no-default-features`.
5. `taplo`.
6. `typos`.
7. `actionlint`.
8. `build`.
9. `doc`.
10. `msrv`.
11. `audit`.
12. `deny`.
13. `machete`.
14. `check-semver`.
15. `hack`.
16. `udeps`.
17. `llvm-cov`.

If any job result is `failure` or `cancelled`, the final gate fails. A `skipped` job is not treated as an error.

## Local Verification

The required baseline local verification set from `AGENTS.md` is:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

This is not complete parity with GitHub CI. It does not include `nextest`, `taplo`, `typos`, `actionlint`, `cargo deny`, `cargo hack`, `cargo udeps`, `cargo llvm-cov`, semver checks, or the build matrix.
