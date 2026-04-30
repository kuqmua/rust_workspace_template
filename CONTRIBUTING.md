# Contributing

## Prerequisites
- Latest Rust nightly installed (`rustup toolchain install nightly`)
- `cargo-nextest` available for local parity with CI (optional)

## Local verification
Run checks in this exact order before opening a pull request:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Optional shortcut aliases are available via `.cargo/config.toml`:

```bash
cargo workspace-format
cargo workspace-lint
cargo workspace-test
cargo workspace-verify
```

Extended local checks for CI parity:

```bash
cargo workspace-nextest
cargo workspace-hack
cargo workspace-deny
cargo workspace-udeps
```

## CI model
CI runs on pull requests targeting `main` and pushes to `main`.

All CI jobs run for both event types when Rust or CI files changed: `fmt`, `clippy`, `test`, `no-default-features`, `taplo`, `typos`, `actionlint`, `build`, `doc`, `audit`, `deny`, `machete`, `check-semver`, `hack`, `udeps`, `llvm-cov`.

## Pull requests
- Keep changes scoped.
- Preserve existing external contracts unless explicitly requested.
- Update tests and docs when behavior changes.
- Include executed verification commands and outcomes in PR description.

## Dependency policy
- Prefer workspace-level dependencies in `Cargo.toml`.
- Add new dependencies only when necessary.
- Keep default features disabled unless required.

## Test ergonomics
- Cover edge-cases in contract tests.
- Keep startup output contracts stable.
