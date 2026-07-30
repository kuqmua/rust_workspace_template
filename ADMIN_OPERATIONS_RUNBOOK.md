# Administrator operations runbook

## Bootstrap

The one-time bootstrap command uses the normal validated server configuration and refuses to run
after any administrator exists. Supply the password through a protected secret file:

```bash
cd server
cargo run --release --bin admin_bootstrap -- \
  administrator "Initial administrator" /run/secrets/admin_bootstrap_password
```

Production bootstrap credentials must come from the deployment secret provider. Never pass the
password as a command-line argument. Delete or revoke the bootstrap secret immediately after the
command succeeds, sign in, and rotate the initial password.

Set `PRODUCTION_MODE=true` only after replacing the template administrator JWT secret, enabling
secure administrator cookies, disabling administrator Swagger, and restricting CORS to explicit
HTTPS origins. Both the application server and bootstrap command fail closed when these production
requirements are not met.

New administrator passwords must contain 12 to 1024 characters with at least one uppercase letter,
one lowercase letter, one digit, and one special character, and must not contain whitespace.
Bootstrap and administrator-reset passwords are temporary: administrator application routes
redirect the affected administrator to the profile page until they replace the password
themselves. Sign-out and self-session revocation remain available.

For a local walkthrough, follow
[the administrator quickstart](docs/admin-quickstart.md).

## Access recovery

1. Confirm the incident and the target administrator through the organization’s normal identity
   verification process.
2. Revoke the administrator’s active sessions.
3. Set a temporary high-entropy password through the approved operational path.
4. Require immediate password rotation and verify that audit records were created.

Never edit session, role, or password-hash tables manually.

## Credential and session rotation

- Rotate database, cookie, and external-service secrets through the deployment secret provider.
- Generate administrator JWT secrets from at least 32 random bytes. `ADMIN_JWT_SECRET` accepts a
  comma-separated list: the first secret signs new tokens and every listed secret verifies tokens.
  Deploy `new,old`, wait at least the configured access-token lifetime, then deploy `new` alone.
- Roll service instances after rotation and verify `/health/ready` before terminating old
  instances.
- Revoke all administrator sessions when cookie-signing material or privileged credentials may
  have been exposed.
- Keep old credentials valid only for the shortest overlap required for a safe rollout.
- `ADMIN_LOGIN_FAILURE_LIMIT` controls the account-specific failed-sign-in lockout threshold;
  `ADMIN_SIGN_IN_RATE_LIMIT` independently bounds sign-in requests in the configured window.

## Audit and retention

Export administrator audit records to the organization’s protected log destination. Retention,
legal hold, and deletion periods are deployment policy and must be configured outside the
repository. Verify periodically that audit writes, exports, and restoration procedures work.

Detailed backup, restore, scaling, alert, and retention exercises are in
[administrator production operations](docs/admin-production-operations.md). Follow the
[upgrade guide](docs/admin-upgrade-guide.md) for migration and rollback decisions.
The exact password, ban, role, permission, and session revocation behavior is in the
[session invalidation matrix](docs/admin-session-invalidation.md).

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
