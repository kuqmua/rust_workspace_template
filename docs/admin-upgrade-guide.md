# Administrator template upgrade guide

## Compatibility contract

Never rewrite or remove an applied migration. Add a forward migration and preserve data needed by
the previous application version during the deployment overlap. Public route, OpenAPI, permission,
and configuration changes follow the [release policy](release-versioning.md).

## Before upgrading

1. Read `CHANGELOG.md` from the current version through the target version.
2. Back up the database and verify the backup by restoring it into an isolated PostgreSQL
   instance.
3. Export administrator audit records to the protected retention destination.
4. Run the target version's database and contract suites against a copy of production data.
5. Record the image digest, schema migration version, backup identifier, and rollback owner.

## Rolling deployment

Run migrations once in a dedicated, serialized migration job before replacing application
instances. Do not let every replica race to own deployment orchestration. New migrations must be
compatible with both old and new application instances for the duration of the rollout.

Replace instances gradually. Keep the old image available, check `/health/live` and
`/health/ready`, then verify sign-in, session refresh, authorization, audit writes, and a
representative administrator read. Readiness must fail while PostgreSQL is unavailable.

## Recovery and rollback

Application rollback is safe only while the new schema remains compatible with the old binary.
Prefer rolling back the image without reversing a data migration. When a migration cannot be
made backward compatible, stop writes, restore the verified pre-upgrade backup, redeploy the old
image digest, and reconcile audit records created after the backup.

Never improvise destructive reverse SQL in production. Rehearse backup restoration and the exact
recovery decision before release.

## Required release evidence

- fresh-schema migration tests;
- upgrade tests from the previous stable release schema;
- OpenAPI and route-contract parity;
- browser acceptance against the upgraded database;
- backup restore verification;
- recorded rollback decision and recovery time.
