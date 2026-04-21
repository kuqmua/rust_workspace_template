# Release Process

## Versioning
- Follow Semantic Versioning (`MAJOR.MINOR.PATCH`).
- Bump versions only for crates that changed.

## Pre-release checklist
1. Ensure CI is green on `main`.
2. Run local checks:
   - `cargo fmt`
   - `cargo clippy --all-targets --all-features -- -D warnings`
   - `cargo test`
3. Update `CHANGELOG.md` (if present).
4. Confirm no unintended public API changes.

## Tagging
1. Create a release commit on `main`.
2. Create an annotated tag:
   - `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
3. Push branch and tag:
   - `git push origin main --tags`

## Post-release
- Verify release artifacts and notes.
- Announce release in project communication channels.
