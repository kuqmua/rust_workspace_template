# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Check Commands

```bash
cargo fmt                                              # format
cargo clippy --all-targets --all-features -- -D warnings # lint
cargo build                                            # build all
cargo test                                             # test all
cargo test -p <crate_name>                             # test single crate
cargo test -p <crate_name> <test_name>                 # test single test
cargo build --release -p server                        # build server binary
docker compose build                                   # build Docker image
docker compose up                                      # run with Docker
```

Always run `cargo fmt` and `cargo clippy --all-targets --all-features -- -D warnings` before finishing work.

## Toolchain

Rust nightly, edition 2024.

## Architecture

Rust workspace with `server`, `tests`, and `optml` members. Workspace root `Cargo.toml` owns:
- All crates.io dependency versions (pinned with `=`, default-features disabled)
- All lint configuration (~260 rust lints + ~800 clippy lints, nearly everything is `deny`)
- Shared package metadata (version, edition, license, rust-version)

Member crates inherit via `version.workspace = true`, `dependency.workspace = true`, `[lints] workspace = true`.

Policy tests live in `tests/src/lib.rs` (45 tests) and enforce workspace invariants at compile/check time.

## Key Conventions (from AGENTS.md)

- No `unwrap()`. Use `expect()` only in tests, with message containing **8 first symbols of a random UUID v4**.
- No `unsafe`. No Axum middleware layers — call reusable functions explicitly in route handlers.
- Errors: enums + `thiserror`. Never swallow `Result`.
- Dependencies: workspace-level only, disable default features, prefer `std` over external crates.
- Do not use abbreviations in names. Keep generated functions/closures inside usage scope.
- Public API: keep minimal, don't change without instruction.
- Keep tests deterministic: no `sleep`, no wall-clock time.
