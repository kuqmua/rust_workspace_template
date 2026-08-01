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

The tag workflow publishes `release-manifest.txt` as a release asset. Treat that file, rather than
the mutable human-readable image tag, as the deployment input: it binds the tag and commit to both
signed image digests. Release publication is gated by core, database and browser acceptance suites,
and candidate images are promoted to version tags only after the configured vulnerability scan.
The workflow serializes executions for a tag and refuses to overwrite an existing release or its
manifest; recovery from a partially published release requires maintainer review rather than an
automatic mutable retry.

Only the latest patch of each release line listed in [`SECURITY.md`](../SECURITY.md) receives
security fixes. Pre-release versions may change incompatibly and must not be treated as stable
production interfaces.
