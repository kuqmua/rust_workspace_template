# Administrator template readiness

## Purpose

This document evaluates what the repository still needs before it can be presented as a reusable
administrator application template rather than only as a production-oriented microservice
workspace that happens to include an administrator console.

The target product is:

> A repository an adopter can rename, configure, bootstrap, customize, extend with application
> resources, test, and deploy without first reverse-engineering the workspace.

## Implementation status

The release-blocking template work in this assessment is implemented. The maintained evidence is:

- `admin_bootstrap/src/main.rs` and database tests for one-time secret-safe bootstrap;
- `docs/admin-quickstart.md`, `docs/admin-feature-matrix.md`, and
  `docs/admin-resource-extension.md`;
- `browser_acceptance` with pull-request smoke and scheduled expanded CI jobs;
- production configuration validation, JWT overlap rotation, configurable account lockout,
  forced initial-password replacement, session invalidation, and security audit coverage;
- `SECURITY.md`, `CHANGELOG.md`, release, upgrade, recovery, customization, architecture, and
  production-operations documentation;
- generated administrator screenshots, issue forms, and the pull-request checklist.

The Priority 1 and Priority 2 capability lists below remain a post-release roadmap, not claims
about the stable feature set. The feature matrix is authoritative when this historical assessment
and current product documentation differ.

The administrator console should remain embedded in the application service. Splitting it into a
separate deployable service is not required unless independent scaling, ownership, or trust
boundaries become an explicit product requirement.

## Current foundation

The repository already contains most of the difficult backend foundations:

| Area | Current evidence | Assessment |
|---|---|---|
| Authentication | Short-lived access tokens, rotating refresh tokens, secure cookies, CSRF checks, trusted-origin checks, session context binding | Strong |
| Authorization | Typed permission catalog, roles, role-permission assignment, user-role assignment, last-administrator protection | Strong |
| User administration | Create, update, ban, delete, set password, assign roles | Strong |
| Session administration | List, revoke one, revoke all, bounded session count | Strong |
| Audit | Append-only database log, filtered reads, bounded CSV export, cleanup policy | Strong |
| Abuse protection | Sign-in and audit-export rate limiting, login-attempt records | Strong |
| Data administration | Generated table catalog, typed filters, sorting, pagination, read-only generic table views | Useful but extension workflow is undocumented |
| Customization | Site name, tab title, logo, primary color, default route, organization and support fields | Good initial branding support |
| Frontend | Server-rendered sign-in and operational pages plus typed CSR pages for administration | Functional |
| API contract | Typed routes, OpenAPI, problem responses, route/contract parity tests | Strong |
| Operations | Health, metrics, tracing, cleanup jobs, containers, Kubernetes bases, dependency and policy checks | Strong |
| Project reuse | Project identity replacement and service scaffolding commands | Partial |

This means the repository should not be rewritten as a new admin product. The highest-value work is
to make the existing system adoptable, explicitly scoped, extensible, and demonstrably safe.

## Release blockers

These items should be completed before calling the repository an administrator template.

### 1. Provide a real first-administrator workflow

`ADMIN_OPERATIONS_RUNBOOK.md` tells operators to run `development_data_bootstrap`, but that package
is a library and has no binary target. The usable primitive is
`server_admin::bootstrap_admin`, which is currently exercised from tests and not exposed through an
operator command.

Add one non-interactive, auditable command with these properties:

- accepts the database destination only through the normal validated server configuration;
- reads the initial password from standard input, a file descriptor, or a secret-file path, never a
  command-line argument;
- refuses to run when an administrator already exists;
- prints no password or password hash;
- returns distinct exit statuses for invalid input, already initialized, unavailable database, and
  successful creation;
- documents development and production invocation separately;
- has tests proving secret redaction, one-time behavior, and failure behavior.

The existing `bootstrap_admin` domain logic should remain the single implementation source.

### 2. Add an adopter quickstart that ends at a working admin screen

The root README explains the whole microservice workspace, but it does not provide a short
administrator-template success path. Add `docs/admin-quickstart.md` containing:

1. clone and rename the project;
2. create environment files;
3. set development database passwords;
4. build the browser assets;
5. start PostgreSQL and the application;
6. create the first administrator;
7. sign in at the exact administrator URL;
8. change the initial password;
9. customize branding;
10. run the required checks.

Every command should be copy-pasteable from a clean clone. CI should execute the quickstart's
machine-executable path so documentation drift is detected.

### 3. Define the supported product scope

The repository needs an explicit support matrix. Without it, adopters cannot distinguish a
deliberate boundary from unfinished functionality.

Document whether the first stable template supports:

- local username/password authentication only;
- single-organization administration;
- administrator identities only, separate from application end users;
- PostgreSQL only;
- one application service containing the admin console;
- English-only UI;
- read-only generic data-table inspection;
- server-side deployment with browser WebAssembly assets.

Features outside that matrix should be labeled either planned or intentionally application-owned.
In particular, SSO, multi-tenancy, application-user self-service, and arbitrary CRUD generation
must not be implied unless they are actually supported.

### 4. Make application-specific extension a documented contract

An admin template needs a stable answer to “how do I add my resource?” Add a guide and one small
reference resource showing:

- where its domain wrapper types live;
- where permissions are declared;
- how permissions are seeded and reconciled;
- how repository queries and migrations are owned;
- how typed routes enter the API and OpenAPI catalogs;
- how a page enters navigation;
- how forms, validation, filtering, sorting, and pagination are defined;
- how audit actions and resource identifiers are recorded;
- which unit, contract, database, and authorization tests are required.

The example should exercise the supported extension mechanism. If generated CRUD is the preferred
mechanism, use it end to end. If custom handlers are expected, show that instead. Do not make
adopters infer the pattern from internal users and roles code.

### 5. Add browser-level acceptance coverage

Current frontend tests primarily validate generated HTML strings, contracts, and server routes.
Add a browser acceptance suite for the distributable admin experience:

- first sign-in and forced initial password change;
- failed sign-in and rate limiting;
- session refresh and expiry;
- user creation, update, banning, role assignment, and deletion;
- role and permission management;
- self-session and all-session revocation;
- settings and branding persistence;
- table filtering and pagination;
- forbidden navigation and direct API access;
- keyboard-only navigation and basic accessibility checks;
- narrow and wide viewport smoke tests;
- browser refresh and deep-link loading of every CSR route.

Run a small deterministic smoke set on every pull request and the full matrix on a scheduled job.
Tests must use local services and disposable PostgreSQL only.

### 6. Fix documentation and naming drift

The README refers to a `server_runtime` foundation and Miri package, while the current packages are
`server_runtime_core` and `server_runtime_http`. Audit every documented command against current
package names.

Also replace or archive root-level scratch documents such as `todo.md` and `usefull.md`. They read
as personal notes and weaken the repository's template presentation. Product plans should use a
maintained roadmap with owners or release targets.

## Security requirements for the first stable release

### Required

- Reject known development JWT secrets and insecure cookie settings when an explicit production
  mode is selected.
- Document secret generation and support signing-key rotation with an overlap window.
- Add a password-policy description to the sign-in/change-password UI and operations guide.
- Add configurable login lockout or progressive delay in addition to request-level rate limiting.
- Record security-sensitive configuration changes and bootstrap attempts in an operator-visible
  audit destination.
- Define session invalidation behavior for password changes, bans, role changes, and permission
  changes.
- Add security headers and cookie behavior to browser acceptance tests.
- Publish `SECURITY.md` with supported versions and private vulnerability-reporting instructions.
- Document database backup, restore, and migration rollback/recovery exercises.

### Authentication roadmap decision

MFA migrations existed and were later removed by migration `0012_remove_unused_tables.sql`.
Therefore MFA must currently be described as unsupported, not partially available.

Before a broad production claim, choose and document one of:

1. implement TOTP plus single-use recovery codes, enrollment confirmation, replay prevention,
   administrator reset, and step-up authentication; or
2. integrate an external OIDC identity provider and treat local passwords as bootstrap/recovery
   only.

Passkeys/WebAuthn can follow later. Implementing several incomplete authentication methods would be
worse than one explicit, tested policy.

## Product capabilities after the release blockers

### Priority 1

- Invitation flow with expiring, single-use activation tokens instead of administrators sharing
  initial passwords.
- Password reset/recovery workflow with no account-enumeration leak.
- Dashboard page with actionable operational summaries rather than making users infer status from
  raw tables.
- Saved filters and stable shareable query URLs.
- Bulk selection and bounded bulk actions with confirmation and audit records.
- CSV export for explicitly exportable application resources.
- In-app display of the current actor, active role/permissions, environment, and deployed version.
- Maintenance/read-only mode visible in both UI and API.

### Priority 2

- OIDC/SSO provider abstraction and group-to-role mapping.
- Optional MFA or passkeys according to the authentication decision above.
- Notification hooks for privileged changes, repeated failed sign-ins, bans, and recovery actions.
- Localization architecture and locale-aware formatting.
- Dark mode and a small design-token catalog beyond one primary color.
- Pluggable dashboard cards owned by application modules.
- Per-resource field visibility and masking policy for sensitive data.

### Application-dependent, not template defaults

- Multi-tenancy and tenant isolation.
- User impersonation.
- Approval workflows.
- Fine-grained row-level authorization.
- Domain-specific analytics.
- File/media administration.

These affect trust and data models deeply. The template should provide extension points and
guidance, not pretend they can be enabled by a flag.

## Repository productization

Add the normal public-template artifacts:

- `SECURITY.md`;
- `CHANGELOG.md`;
- a release/versioning policy;
- an admin-focused architecture diagram;
- screenshots for sign-in, users, roles, settings, sessions, and data tables;
- a feature/support matrix;
- an upgrade guide covering schema and public API compatibility;
- an issue template for security-safe bug reports;
- an issue template for new admin resources;
- a pull-request checklist for migrations, permissions, audit coverage, API contracts, and browser
  tests.

The root README should lead with what an adopter receives, show the UI, state supported deployment
targets, and link to a ten-minute quickstart. Deep internal quality tooling can remain documented
without dominating the first-use path.

## Customization model

Branding is currently stored in the database. Complete the model by separating:

- **compile-time identity**: crate/package names, repository URL, container names, default service
  identifiers;
- **deployment configuration**: public origin, cookie/security policy, identity provider,
  database, telemetry, enabled capabilities;
- **runtime branding**: site title, logo, colors, organization, support links, default page;
- **application extensions**: permissions, routes, resources, navigation, dashboard cards.

Document precedence and restart requirements. Validate logo and support URLs with the outbound URL
policy or require same-origin asset paths. Provide a reset-to-template-default operation that is
audited and cannot clear required values accidentally.

## Operational completeness

The Kubernetes bases are a good starting point, but a template release also needs:

- a documented production overlay example with no real secrets;
- migration job/strategy for rolling deployments;
- backup and restore verification;
- horizontal scaling notes for session, rate-limit, and cleanup behavior;
- readiness behavior during migrations and database degradation;
- capacity guidance for password hashing and database pools;
- alert examples for failed sign-ins, elevated `5xx`, cleanup lag, exhausted pools, and audit export
  failures;
- an explicit log retention and audit export integration example;
- a supported upgrade test from the previous released schema, not only fresh-schema tests.

## Suggested implementation order

### Milestone A: adoptable

1. First-admin command.
2. Admin quickstart and corrected commands.
3. Product scope and feature matrix.
4. Extension guide plus reference resource.
5. Screenshots and README restructuring.

Exit criterion: a new contributor can reach, brand, and extend a working admin console from a clean
clone using only documented commands.

### Milestone B: trustworthy

1. Production-mode configuration guardrails.
2. Browser acceptance suite.
3. `SECURITY.md`, recovery, backup, and upgrade procedures.
4. Authentication roadmap decision and implementation.
5. Security event and session invalidation matrix.

Exit criterion: authentication, authorization, recovery, and upgrade claims are verified through
real browser and PostgreSQL flows.

### Milestone C: productive

1. Invitations and recovery.
2. Dashboard and application extension slots.
3. Bulk actions and exports.
4. SSO/notification hooks.
5. Localization and expanded theming.

Exit criterion: common admin applications can be built by adding domain modules rather than
modifying framework internals.

## Definition of done

The repository is ready to advertise as an administrator template when all of the following are
true:

- a clean-clone quickstart is continuously tested;
- first-administrator creation is safe and documented;
- the supported feature matrix is explicit;
- one application resource demonstrates the complete extension path;
- browser tests cover critical authentication, authorization, CRUD, session, and branding flows;
- production configuration rejects known insecure development values;
- security reporting, upgrade, backup, restore, and recovery procedures exist;
- public documentation contains current commands, screenshots, and architecture;
- unsupported features such as MFA, SSO, and multi-tenancy are not implied;
- all repository quality gates and database integration suites pass.
