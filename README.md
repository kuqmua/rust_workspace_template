# Rust Workspace Template

Production-oriented Rust workspace template with strict linting, shared domain logic, deterministic tests, and CI gates that scale from pull requests to nightly full verification.

## Goals

- Keep architecture boundaries explicit: application crate (`server`) depends on domain/shared crate (`shared_logic`).
- Keep dependencies controlled: workspace-level declarations, no implicit defaults.
- Keep quality predictable: strict lint profile, deterministic tests, reproducible local verification order.
- Keep developer onboarding fast: copy-paste commands and cargo aliases for daily workflows.
- Keep machine integrations simple: stable text output by default plus optional JSON report format.

## Workspace layout

- `shared_logic`: reusable domain logic and public API contract with unit and integration coverage.
- `server`: minimal entrypoint crate that consumes shared logic and demonstrates startup argument validation.
- `test_helpers`: shared deterministic test setup and fixtures.

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
- `cargo workspace-verify` (runs `fmt -> clippy -> test` in required order)

## Production build defaults

Release profile is hardened in workspace root `Cargo.toml`:

- `lto = "fat"`
- `codegen-units = 1`
- `panic = "abort"`
- `strip = "symbols"`

These settings optimize for smaller and more predictable production binaries.

## Server usage examples

Show help:

```bash
cargo run -p server -- --help
```

Positional input:

```bash
cargo run -p server -- 10 + 5
```

Wire-format input:

```bash
cargo run -p server -- --wire-format "10|*|4"
```

JSON output for machine consumers:

```bash
CALCULATION_REPORT_FORMAT=json cargo run -p server -- 10 + 5
```

## Output format contract

- Default format is `text`.
- Optional environment variable: `CALCULATION_REPORT_FORMAT`.
- Supported values: `text`, `json`.
- Unknown values fail fast with a typed error and non-zero exit code.

## Extension rules

- Add shared/domain behavior to `shared_logic`, not to `server`.
- Add crates.io dependencies only in `[workspace.dependencies]`.
- Use `*.workspace = true` in crate `Cargo.toml` files.
- Disable default features unless required by a concrete use case.
- Preserve external contracts unless changes are explicitly requested.

## CI and governance

- Main CI: `.github/workflows/ci.yml`
- Contribution guide: `CONTRIBUTING.md`
- Release process: `RELEASE.md`
- Security policy: `SECURITY.md`
- Changelog template: `CHANGELOG.md`

## Policy tests

`test_helpers/tests/policy_rules.rs` enforces template rules, including:

- workspace dependencies and disabled default features
- no forbidden runtime shortcuts (`unwrap`, `todo!`, source-dropping `map_err`)
- deterministic testing constraints
- hardened release profile and `workspace-verify` command order
- nightly toolchain contract (`rust-toolchain.toml` must stay on `channel = "nightly"`)
- no `dbg!` and no ad-hoc `println!`/`eprintln!` outside entrypoint runtime path
- CLI contract tests in `server/tests` must use shared `test_helpers::run_server_command*` wrappers and must not call `Command::new` directly

## CLI test helper

`test_helpers` provides reusable helpers for CLI integration tests:

- `run_server_command`
- `run_server_command_with_report_format`
- `stdout_as_utf8` / `stderr_as_utf8`

These helpers keep test code concise and make edge-case assertions (including non-unicode environment values) consistent across new tests.

When adding a new CLI test, prefer this pattern:

```rust
let output = run_server_command(SERVER_BINARY_PATH, &["10", "+", "5"]).expect("1a2b3c4d");
let standard_output = stdout_as_utf8(&output).expect("5e6f7a8b");
assert!(output.status.success());
assert!(standard_output.contains("result=15"));
```
