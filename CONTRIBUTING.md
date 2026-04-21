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

## CI model
This template uses two CI modes in `.github/workflows/ci.yml`:

- Fast mode: runs on pull requests and pushes to `develop`.
  - `fmt`, `clippy`, `test`, `no-default-features`, `taplo`, `typos`, `actionlint`
- Full mode: runs on pushes to `main`, nightly schedule, and manual dispatch.
  - `build`, `doc`, `msrv`, `audit`, `deny`, `machete`, `hack`, `udeps`, `llvm-cov`

## Pull requests
- Keep changes scoped.
- Preserve existing external contracts unless explicitly requested.
- Update tests and docs when behavior changes.

## Dependency policy
- Prefer workspace-level dependencies in `Cargo.toml`.
- Add new dependencies only when necessary.
- Keep default features disabled unless required.
