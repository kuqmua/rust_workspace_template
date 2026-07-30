# Administrator feature matrix

This file defines the supported boundary of the template. An absent capability is not implied by
the presence of a related table, migration, component, or internal helper.

## Supported

| Capability | Scope |
|---|---|
| Identity | Administrator identities are separate from application end users |
| Authentication | Local login and password |
| Authorization | Roles with typed permissions |
| Organization model | One administrator organization per deployment |
| Persistence | PostgreSQL |
| Deployment | Administrator console embedded in the application service |
| User management | Create, edit, ban, delete, set password, assign roles |
| Role management | Create, edit, delete, assign permissions |
| Sessions | Bounded concurrent sessions, refresh, list, revoke one or all |
| Security boundary | Secure cookies, CSRF, trusted-origin validation, rate limiting |
| Audit | Append-only records, filtered reads, bounded CSV export |
| Data tables | Read-only inspection for registered tables, with typed filters and pagination |
| Branding | Names, title, logo, primary color, support and organization fields |
| API | Typed JSON routes, OpenAPI and problem responses |
| UI language | English |
| Browser delivery | Server-rendered entry pages and browser WebAssembly administration pages |

## Not currently supported

- MFA, TOTP, recovery codes, passkeys, or WebAuthn;
- OIDC, OAuth, SAML, LDAP, or other SSO providers;
- administrator invitations or self-service password recovery;
- multi-tenancy or tenant isolation;
- application-user self-service;
- impersonation;
- arbitrary row-level authorization;
- generic write access to every database table;
- localization;
- offline browser operation.

MFA-related migrations were removed by migration `0012_remove_unused_tables.sql`; MFA is therefore
unsupported rather than partially enabled.

## Authentication roadmap decision

The selected future broad-production path is external OIDC, with local credentials retained only
for bootstrap and explicitly controlled recovery. Group-to-role mapping, issuer and audience
pinning, PKCE/state/nonce validation, account-linking rules, and recovery policy must be
implemented and accepted before OIDC is moved into the supported table. Native TOTP is not being
developed in parallel.

The first stable local-credential release makes no SSO or MFA claim.

## Application-owned decisions

These features require domain-specific trust and data models and are extension points, not template
flags:

- approval workflows;
- application analytics;
- domain dashboards;
- application-resource exports and bulk actions;
- field masking beyond the administrator schema;
- notifications for domain events;
- file and media administration.

Changes to this matrix require corresponding contracts, user-facing documentation, and acceptance
tests.
