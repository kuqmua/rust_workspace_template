# Administrator operations runbook

## Production deployment

- Build the complete image from the repository `Dockerfile`; it builds the release WASM bundle and
  copies both generated and static administrator assets into the runtime image. Do not deploy a
  standalone server binary without those assets.
- Terminate TLS at the application gateway, set `ADMIN_COOKIE_SECURE=true`, use an exact HTTPS
  origin in `CORS_ALLOW_ORIGIN`, and disable Swagger with `ADMIN_SWAGGER_ENABLED=false` unless its
  permission-protected UI is explicitly required.
- Replace every development credential. Keep `ADMIN_JWT_SECRET` in the deployment secret store,
  use a high-entropy primary value, and retain old values only during a planned rotation window.
- Keep the service port private to the gateway. Configure HSTS at the TLS terminator, preserve the
  restrictive content security policy, and expose health endpoints only to infrastructure that
  needs them.
- Apply and verify all database migrations before shifting traffic. The migration test exercises
  both a fresh schema and the supported version-3 baseline upgrade against PostgreSQL.
- Back up and restore-test PostgreSQL, alert on readiness failures and stale cleanup status, and
  retain structured server logs according to the incident-response policy.

## Bootstrap

Administrator bootstrap is deliberately available only through the server-side
`server_admin::bootstrap_admin` API. It succeeds only while `admin_users` is empty and creates the
first user and its administrator role assignment in one transaction. Do not expose this operation
through HTTP or leave bootstrap credentials in environment files.

For a new deployment, run a short-lived trusted bootstrap executable inside the same deployment
boundary, pass validated `AdminLogin`, `AdminDisplayName`, and `AdminPassword` values, verify that
the call succeeds, and then remove the executable and its input secret. A second invocation must
fail with `AlreadyInitialized`.

## Lost access

1. Preserve the database and application logs before making changes.
2. Check whether another active administrator can restore roles, revoke sessions, or issue MFA
   recovery codes through the normal UI.
3. If every administrator is locked out, restore the administrator tables from the most recent
   verified database backup. Do not delete users merely to make bootstrap run again.
4. Rotate the administrator JWT secret after an account or token compromise. Keep the previous
   verification secret only for the intended token transition window; the MFA encryption layer can
   decrypt with retained verification secrets and writes with the current primary secret.
5. Sign in, enroll MFA again where required, revoke all old sessions, and review the audit log.
6. Record the incident and the exact recovery actions outside the application database.

Direct password-hash edits, disabling audit triggers, arbitrary browser SQL, and exposing secrets
through an administrator endpoint are not supported recovery procedures.

## MFA recovery

Recovery codes are shown once and stored only as hashes. Each code is consumed atomically. A TOTP
time step is also accepted only once per account, including across concurrent requests. If an
administrator has neither a TOTP device nor an unused recovery code, another privileged
administrator must restore access according to the deployment's identity policy. MFA enrollment,
failed challenges, recovery-code use, step-up, and disable operations are audited.

## Cleanup job

The server runs bounded cleanup using the configured retention periods and batch size. Every fully
successful run updates the singleton `admin_cleanup_status` row with its completion time and total
deleted rows. The dashboard reports this value; an absent or stale value should alert operators to
inspect server logs and the cleanup-task configuration. Cleanup never bypasses the audit append-only
guard except inside its scoped database transaction.

## Session and audit data policy

The public session DTO deliberately contains only the opaque session identifier, creation and
expiry timestamps, and the server-derived `is_current` flag. The service does not persist or expose
raw browser, user-agent, or IP metadata for session selection. A one-way context hash is used only
to bind credentials to their request context; it cannot be used to reconstruct those values. Add
device metadata only after the deployment has an approved retention and privacy policy.

Audit timestamps are stored and returned in UTC. Retention is controlled by the bounded
`audit_retention` cleanup setting; operators should translate UTC only in their local presentation
layer. Login and resource identifiers are operational personal data and must follow the deployment's
access and retention policy. Passwords, tokens, MFA secrets, recovery codes, encryption keys, and
raw authorization headers must never enter audit details. Audit export has its own permission,
bounded row count, and rate limit; exported files must receive the same access controls and expiry
policy as the database audit log.

## Backups and diagnostics

Database backup and restore remain deployment operations, not administrator UI actions. Dependency
diagnostics must expose only health and timestamps, never connection strings, credentials, object
contents, or environment values. Email, object-storage, queue, and feature-flag controls should be
added only when those dependencies exist in the host application.
