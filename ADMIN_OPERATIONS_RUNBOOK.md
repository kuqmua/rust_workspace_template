# Administrator operations runbook

## Bootstrap

Run `development_data_bootstrap` only against an explicitly selected non-production database.
Production bootstrap credentials must come from the deployment secret provider and must be rotated
immediately after the first administrator signs in.

## Access recovery

1. Confirm the incident and the target administrator through the organization’s normal identity
   verification process.
2. Revoke the administrator’s active sessions.
3. Set a temporary high-entropy password through the approved operational path.
4. Require immediate password rotation and verify that audit records were created.

Never edit session, role, or password-hash tables manually.

## Credential and session rotation

- Rotate database, cookie, and external-service secrets through the deployment secret provider.
- Roll service instances after rotation and verify `/health/ready` before terminating old
  instances.
- Revoke all administrator sessions when cookie-signing material or privileged credentials may
  have been exposed.
- Keep old credentials valid only for the shortest overlap required for a safe rollout.

## Audit and retention

Export administrator audit records to the organization’s protected log destination. Retention,
legal hold, and deletion periods are deployment policy and must be configured outside the
repository. Verify periodically that audit writes, exports, and restoration procedures work.

## Deployment verification

The canonical service mapping is [`deploy/services.toml`](deploy/services.toml). After changing a
service port, image, Dockerfile, or manifest, run:

```bash
cargo test -p tests code_style
```

For each rollout, confirm liveness at `/health/live`, dependency readiness at `/health/ready`, and
that unavailable dependencies produce HTTP 503 rather than a redirect or a successful fallback.

## Incident handling

Preserve request IDs, trace IDs, timestamps, immutable image digests, audit records, and relevant
service logs. Do not place credentials or full session tokens in tickets or chat. Escalation
contacts and external notification timelines belong in the deployment-specific incident plan.
