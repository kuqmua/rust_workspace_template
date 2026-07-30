# Security policy

## Supported versions

Until the first tagged stable template release, security fixes are applied to the default branch
only. After a stable release, this section will list the supported release lines explicitly.

## Reporting a vulnerability

Do not open a public issue for a suspected vulnerability.

Use the repository host's private security-advisory feature to report:

- the affected revision;
- the vulnerable route, component, or deployment configuration;
- reproduction steps using non-production data;
- expected and observed impact;
- any suggested mitigation.

Do not include production credentials, cookies, tokens, database contents, or personal data.

The maintainer should acknowledge a complete report within five business days, coordinate a fix and
disclosure window with the reporter, and publish remediation guidance when the fix is released.

## Security boundary

The current administrator security capabilities and unsupported authentication methods are listed
in [the administrator feature matrix](docs/admin-feature-matrix.md). Deployment operators remain
responsible for TLS termination, secret storage, database backups, audit retention, ingress policy,
and incident contacts.

