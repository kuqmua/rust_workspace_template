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

### Changed
- `server --help` now documents the output format environment variable contract.
- Contribution and release docs now include output-contract verification steps.

### Fixed
- Eliminated implicit output-format assumptions by moving format parsing to typed shared logic.

## [0.1.0] - 2026-04-21

### Added
- Initial workspace template structure.
