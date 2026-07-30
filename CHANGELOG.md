# Changelog

All notable user-facing changes will be documented in this file.

The project follows semantic versioning after its first stable release. Until then, the
`Unreleased` section is authoritative.

## Unreleased

### Added

- Administrator-template readiness roadmap and supported-feature matrix.
- Safe one-time `admin_bootstrap` command using a bounded password file.
- Clean-clone administrator quickstart.
- Administrator resource extension guide.
- Private vulnerability-reporting policy.
- Forced initial-password replacement with transactional session invalidation.
- Production-mode security validation and overlapping JWT signing-key rotation.
- Configurable account-specific failed-login lockout.
- Pull-request and scheduled Playwright acceptance suites using disposable PostgreSQL.
- Administrator architecture, customization, release, upgrade, backup, restore, recovery,
  scaling, alerting, retention, and session-invalidation documentation.
- Real administrator screenshots, issue forms, pull-request checklist, and production overlay
  example.

### Fixed

- Dedicated administrator CSR pages now take precedence over same-named generic database tables.
- CI Miri and semantic-version checks use the current `server_runtime_core` and
  `server_runtime_http` package names.
