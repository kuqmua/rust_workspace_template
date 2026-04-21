# Changelog

All notable changes to this workspace template should be documented in this file.

The format is based on Keep a Changelog and this project follows Semantic Versioning.

## [Unreleased]

### Added
- `CalculationReportFormat` enum in `shared_logic` with explicit `text|json` contract.
- `render_calculation_report_with_format` API for deterministic report rendering by format.
- CLI support for `CALCULATION_REPORT_FORMAT` in `server` with fail-fast validation.
- Additional unit and integration coverage for output-format parsing and JSON report mode.
- `cargo workspace-verify` alias for required local verification sequence.
- Release profile hardening (`lto`, `codegen-units`, `panic=abort`, `strip`) in workspace root.
- Policy tests that enforce release-hardening and exact `workspace-verify` command order.
- CLI contract test for malformed wire format failures with stable error output.
- Shared-logic contract test that locks malformed wire-format error message text.
- Reusable CLI integration-test helpers in `test_helpers` (`run_server_command*`, UTF-8 accessors).
- CLI contract coverage for non-unicode `CALCULATION_REPORT_FORMAT` values on Unix.
- Policy checks that forbid debug print macros in non-test code (except entrypoint output).
- Policy check that enforces `rust-toolchain.toml` nightly channel contract.
- Shared-logic contract tests for report-format parsing and explicit-text/default render equivalence.
- CLI contract tests for JSON wire-format output and division-by-zero failure path.
- Policy checks for required daily cargo aliases in `.cargo/config.toml`.
- Policy checks that fast CI keeps `nextest` and doc-test execution.
- Policy checks that `README.md` and `CONTRIBUTING.md` keep required local verification order.
- Policy checks that every workflow defines top-level minimal `permissions` (`contents: read`).
- Policy checks that third-party GitHub Actions are pinned by full 40-character commit SHA.
- CLI contract test for `--help` long flag behavior.
- Shared-logic contract tests for arithmetic overflow and unknown-operation error payload stability.
- Shared-logic contract tests for division-overflow edge case and invalid-integer payload stability.
- CLI contract tests for invalid integer input, unknown wire-format operation, and division overflow.
- Policy check that enforces shared CLI command helpers in `server/tests` and forbids direct `Command::new` there.

### Changed
- `server --help` now documents the output format environment variable contract.
- Contribution and release docs now include output-contract verification steps.
- CI workflow now declares top-level least-privilege permissions.

### Fixed
- Eliminated implicit output-format assumptions by moving format parsing to typed shared logic.

## [0.1.0] - 2026-04-21

### Added
- Initial workspace template structure.
