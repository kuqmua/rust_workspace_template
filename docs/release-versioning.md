# Release and versioning policy

Stable template releases use semantic versioning. A release tag `vMAJOR.MINOR.PATCH` identifies
one tested combination of Rust toolchain, Cargo lockfile, browser assets, database migrations,
container definitions, and public administrator API contracts.

- Patch releases contain compatible fixes and documentation corrections.
- Minor releases may add migrations, permissions, routes, and optional configuration while
  preserving existing supported deployments.
- Major releases may remove or redesign public contracts and require an explicit upgrade step.

The administrator HTTP API, migration history, documented configuration variables, and extension
contract are public compatibility surfaces. Internal crates remain implementation details unless
their documentation explicitly says otherwise. Every release records user-visible changes in
[`CHANGELOG.md`](../CHANGELOG.md), publishes immutable container digests, and links to the relevant
upgrade instructions.

Only the latest patch of each release line listed in [`SECURITY.md`](../SECURITY.md) receives
security fixes. Pre-release versions may change incompatibly and must not be treated as stable
production interfaces.
