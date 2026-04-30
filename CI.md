# CI

CI is defined in `.github/workflows/ci.yml`.

## Triggers

1. `push` to `main`.
2. `pull_request` targeting `main`.

## Global Settings

All jobs use these defaults:

1. `ubuntu-latest`.
2. `20` minute timeout per job.
3. Minimal permissions: `contents: read`.
4. `CARGO_INCREMENTAL=0`.
5. `CARGO_NET_RETRY=10`.
6. `CARGO_NET_TIMEOUT=60`.
7. `RUSTFLAGS="-D warnings -D unreachable_pub -D dead_code"`.
8. `RUSTDOCFLAGS="-D warnings"`.
9. `RUST_BACKTRACE=short`.

Every check below runs for both `push` and `pull_request` events. Checks run in parallel unless a job explicitly depends on another job:

1. `fmt`.
2. `metadata`.
3. `clippy`.
4. `test`.
5. `taplo`.
6. `typos`.
7. `actionlint`.
8. `deny`.
9. `machete`.
10. `check-semver`.
11. `hack`.
12. `udeps`.
13. `llvm-cov`.

## Formatting

The `fmt` job runs:

```bash
cargo fmt
```

Then it runs:

```bash
git diff --exit-code
```

This fails CI if formatting changed files in the runner.

## Metadata

The `metadata` job runs:

```bash
cargo metadata --locked --format-version 1
```

It verifies that `Cargo.lock` and the dependency graph are consistent.

## Clippy

The `clippy` job runs on `nightly`:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

It checks the whole workspace, all targets, and all features. Every warning is treated as an error.

## Tests

The `test` job runs:

```bash
cargo test
```

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

## Cargo Deny

The `deny` job runs:

```bash
cargo deny check advisories bans licenses sources
```

It checks security advisories, banned dependencies, licenses, and dependency sources.

## Cargo Machete

The `machete` job runs:

```bash
cargo machete
```

It detects unused dependencies.

## Semver Check

The `check-semver` job uses `obi1kenobi/cargo-semver-checks-action` and checks for semver-breaking changes in public APIs.

## Cargo Hack Feature Matrix

The `hack` job runs:

```bash
cargo hack check --workspace --feature-powerset --no-dev-deps
```

It checks feature flag combinations.

## Cargo Udeps

The `udeps` job runs:

```bash
cargo +nightly udeps --workspace --all-targets --all-features
```

It detects unused dependencies through `cargo-udeps`.

## Coverage

The `llvm-cov` job runs:

```bash
cargo llvm-cov --workspace --all-features --all-targets --summary-only
```

It prints a test coverage summary. CI currently reports the summary without enforcing a coverage threshold.

## Local Verification

The required baseline local verification set from `AGENTS.md` is:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

This is not complete parity with GitHub CI. It does not include `taplo`, `typos`, `actionlint`, `cargo deny`, `cargo hack`, `cargo udeps`, `cargo llvm-cov`, or semver checks.
