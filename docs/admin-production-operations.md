# Administrator production operations

## Deployment overlay

`deploy/k8s/base` is deliberately incomplete. A production overlay must provide immutable image
digests, TLS ingress, managed PostgreSQL, ConfigMaps, secret references, workload identity,
telemetry endpoints, and deployment-specific network policy. Never place real secrets in an
overlay committed to this repository.

[`deploy/k8s/overlays/production-example`](../deploy/k8s/overlays/production-example) shows
production guardrails and external Secret references with intentionally unusable example values.

Run migrations as one serialized pre-deployment job. Keep schema changes compatible with the old
and new application during rolling replacement. The application readiness probe must remain
unready during migration or database degradation; liveness should continue to report process
health.

## Backup and restore exercise

At the deployment's recovery-point interval, create an encrypted PostgreSQL backup including
schema, administrator data, and audit records. At least once per release:

1. restore the backup into an isolated network and database;
2. run migration, schema, and consistency checks at the restored version;
3. start the exact released image against the restored database;
4. verify administrator sign-in, RBAC, sessions, branding, audit reads, and cleanup status;
5. record duration, backup identifier, image digest, and discrepancies;
6. destroy the isolated copy according to data-retention policy.

## Scaling and capacity

Session, RBAC, rate-limit, audit, and cleanup coordination are PostgreSQL-backed, so replicas may
scale horizontally behind one trusted ingress. Size the pool so the sum of every replica's maximum
connections stays below the database limit with room for migrations and operations. Password
hashing is CPU and memory intensive; bound its concurrency below the pod CPU capacity and load-test
sign-in bursts without weakening hash parameters.

Cleanup work must remain single-owner through database coordination. Deployments must verify
cancellation during shutdown and must not depend on process-local rate-limit or session state.

## Alerts and retention

Alert on sustained failed-sign-in growth, unexpected lockouts, elevated `5xx`, readiness failures,
pool acquisition timeout or exhaustion, cleanup lag, audit-write or export failures, and repeated
session refresh rejection. Route alerts to deployment-owned responders with links to request IDs,
trace IDs, image digests, and this runbook; never include credentials or complete tokens.

Forward structured application logs and administrator audit exports to a protected destination.
Document independent retention periods for operational logs and audit records, legal holds,
access control, export verification, and deletion. Test that request and trace identifiers can
correlate an audit action without exposing secrets.
