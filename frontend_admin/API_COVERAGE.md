# Administrator API coverage audit

The typed registry in `server_admin/src/admin_auth_route_registry.rs` registers 30 API
operations. Paths below are rooted at the server origin. The frontend must follow the route,
permission, request, and table catalogs in `server_admin_contract`.

Generated read APIs use the configured database resource name in their URL, independently
of the Rust type name: `/users/read` and the corresponding
`*_payload_example` routes. The same rule applies to `roles`, `permissions`,
`system_settings`, `user_roles`, and `role_permissions`; these paths have no `admin_` prefix. Single-record reads use a primary-key equality
filter with a limit of one and offset zero. Missing records return empty items; `/users/read` returns `{ items, roles, total }`.
The table generator exposes reads through `read`; `read_one` and `read_one_payload_example` have been removed from every API mode.
Mutations use `create_many`, `update`, and `delete_many`; the corresponding `_one` routes and payload examples have been removed. Single-record creation submits a one-element array. Single-record updates submit a one-element array keyed by the primary key; deletions use an equality filter on the primary key. Revision-aware bulk updates require `If-Match` and roll back the entire batch if any key is missing or its revision does not match.

The legacy `GET /users` endpoint has been removed. The users CSR client sends a typed JSON request to `/users/read`; the generated read registry owns its full request and response schemas.

This audit maps each operation to its administrator integration. Validation evidence
below covers user-visible workflows and direct API behavior.

| Method and path | Administrator UI or integration |
| --- | --- |
| POST `/auth/sign_in` | Sign-in form; server-rendered HTML adapter |
| POST `/auth/sign_out` | Navigation sign-out action |
| POST `/auth/refresh` | CSR reads and mutations recover expired access/CSRF cookies with one refresh and one retry; full-page requests preserve the server sign-in redirect |
| GET `/auth/me` | Authenticated shell permissions and profile |
| POST `/auth/password` | Profile password form and mandatory initial password replacement |
| GET `/auth/sessions` | Sessions page |
| DELETE `/auth/sessions/{session_id}` | Per-session confirmation dialog |
| DELETE `/auth/sessions` | Revoke-all confirmation dialog; includes the current session |
| POST `/users/read` | Users list with selected fields, combined search and filters, sorting, pagination, role assignments, and total count |
| POST `/users/create` | Create-user page; server-rendered HTML adapter |
| PATCH `/users/update` | Atomic batch updates through the typed API; uses the same user validation, administrator protection, audit, and session revocation as single-user updates |
| DELETE `/users/delete` | Atomic deletion of users matching a required filter |
| POST `/roles/read` | Roles list with search, filters, sorting, pagination, permission assignments, and total count |
| POST `/roles/create` | Atomic creation of multiple roles through the typed API |
| PATCH `/roles/update` | Atomic renaming of multiple roles through the typed API |
| DELETE `/roles/{role_id}` | Manage-roles page: delete role |
| PUT `/roles/{role_id}/permissions` | Manage-roles page: permission assignment |
| GET `/permissions` | Read-only permissions list |
| GET `/audit_log` | Audit view uses the catalog table endpoint; browser coverage also verifies the dedicated query and redacted mutation records |
| GET `/audit_log/export` | Prepare/download controls export the current audit page using its limit and offset; export permission governs visibility |
| GET `/system_settings` | Settings form |
| PATCH `/system_settings/update` | Save settings and reset supported settings to defaults |
| GET `/branding` | Shared branding in server-rendered pages |
| GET `/tables` | Catalog defines available database views; navigation follows the shared table specification |
| GET `/tables/{table}` (`users`, `roles`, `permissions`, `audit_log`, `system_settings`) | Catalog-driven columns, filters, ordering, and pagination |

The 12 table views are `users`, `roles`, `permissions`, `user_roles`, `role_permissions`,
`refresh_tokens`, `access_sessions`, `login_attempts`, `audit_log`, `system_settings`,
`rate_limits`, and `cleanup_status`. Their API is read-only. Account and role mutations
belong to the dedicated operations above; session revocation and settings changes belong
to their dedicated pages.

The HTML user and role actions are registered in
`server_admin/src/admin_html_user_action_route_registry.rs` and
`server_admin/src/admin_html_role_action_route_registry.rs`. Their forms are rendered by
`render_user_create`, `render_user_manage`, `render_role_create`, and `render_role_manage`.
Permission checks must govern both navigation visibility and each mutation control.

Browser acceptance covers navigation and CRUD workflows in `admin.spec.js`, page/catalog
coverage in `page-coverage.spec.js`, security and direct API behavior in
`production-readiness.spec.js`, and refresh/session behavior in `z-admin-full.spec.js`.
The latter suite requires `BROWSER_ACCEPTANCE_FULL=1`. Visual references require review
when the requested layout or controls change; replacing snapshots alone does not verify
behavior.

The replacement `frontend_admin` library owns the implementation. The former `frontend`
package and directory are removed; server consumers, Docker builds, CI, and release
projections use the replacement. Shared transport contracts remain in `frontend_contract`.
The lockfile changes only the replacement package name and its two consumers.

The desktop header wraps navigation links above full-width content. Mobile navigation
expands into two columns. The reviewed visual references cover pages, CRUD forms, errors,
and navigation. A 1920px geometry check exposed and verified removal of the old 1600px
content cap. Updated asset version queries invalidate cached loaders and styles.

The audit download uses the typed export route and current pagination. Browser tests
verify that the downloaded CSV matches the API response, excludes submitted passwords,
and is unavailable without export permission. Disabled destructive controls have no
dialog trigger. The settings failure test verifies preserved input and no mutation replay.

CSR requests recover expired access and CSRF cookies with at most one refresh and one
retry. A missing CSRF cookie triggers recovery before sending a mutation. Missing refresh
credentials terminate recovery, and ordinary network failures and non-authentication
server errors are not replayed. Full-page navigation preserves the server's existing
sign-in redirect when the access session has expired.

Verification passed:

- `cargo fmt` and formatting checks.
- Workspace Clippy with all targets, all features, and warnings denied.
- WebAssembly Clippy for `frontend_admin` with warnings denied, also added to CI.
- All 282 code-style tests through `workspace_test_runner static`.
- `cargo test --workspace --exclude tests_code_style_rust`.
- All 82 functional Playwright tests with `BROWSER_ACCEPTANCE_FULL=1`.
- All 53 visual regression checks against reviewed desktop and mobile references.
- The isolated session-limit test with a limit of two.
- The documentation screenshot run.
- The provisioned database suite against the disposable browser database.

Database integration also verified two corrections discovered during acceptance:
administrator initialization creates the shared `pg_table_idempotency` schema under a
transaction lock, and session revocation timestamps preserve creation-time constraints.
User, role, and permission sorting follows the API's ascending/descending wire values.

The table views for `user_roles`, `role_permissions`, `refresh_tokens`,
`access_sessions`, `login_attempts`, `rate_limits`, and `cleanup_status` use
GET routes at `/{resource}`. Their former `/tables/{resource}` paths are rejected.

The `changes` object in `PATCH /users/update` accepts optional `display_name`, `login`, `is_banned`, `password`,
and the paired `role_ids` / `expected_role_ids` arrays.
Omitting `is_banned` preserves the current state. Banning retains the self-ban and
last-active-administrator checks, session revocation, and transactional audit.

## User batch updates

`PATCH /users/update` requires `users:update`, a valid session, an allowed Origin,
and a CSRF token. It returns `204 No Content` after all changes commit.

```json
{
  "updates": [
    { "filter": { "user_id": 12 }, "changes": { "display_name": "Updated Name" } },
    { "filter": { "user_id": 13 }, "changes": { "is_banned": true } }
  ]
}
```

Each item can change `login`, `display_name`, `is_banned`, `password`, and role assignments. Omitted fields remain
unchanged. Empty batches, duplicate user identifiers, empty changes, and invalid field
values are rejected. The existing administrator collection limit is 10,000 items, and
the route also enforces the common request body limit.

A missing user, a conflicting login, self-blocking, or removal of the last active
administrator rolls back the entire batch, including its audit records and session
revocations. Unblocking updates run before blocking updates so a valid administrator
replacement does not depend on request item order. Role assignments are updated through the same endpoint with `role_ids` and `expected_role_ids`. User updates use `/users/update` for both individual users and groups.

Each update contains `filter` and `changes`. Filters support exact matches on
`user_id`, `login`, `display_name`, and `is_banned`; supplied conditions are combined
with AND. Every filter must contain at least one non-null condition. All matching
users receive that entry's changes. Filters are resolved before any changes are
applied, and matched rows remain locked until the transaction completes.
An empty selection returns 409. Empty filters, overlapping selections, empty
changes, and more than 10,000 selected users across the batch return 422.
The entire request rolls back on any error. HTML user actions use this same
filtered update workflow.

## User batch deletion

`DELETE /users/delete` accepts a required filter:

```json
{"filter": {"display_name": "Inactive accounts", "is_banned": true}}
```

The filter has the same exact-match fields and AND semantics as user updates:
`user_id`, `login`, `display_name`, and `is_banned`. At least one non-null condition
is required. The route requires `users:delete`, an authenticated session, an allowed
Origin, and a CSRF token. Success returns `204 No Content`.

At most 10,000 users can be deleted in one transaction. An empty filter or an
oversized selection returns 422; no matches, selection of the acting user, or
removal of the last active administrator returns 409. Any failure rolls back all
deletions and audit records. Related sessions and role assignments follow the
existing database deletion rules. Each deleted user receives an audit record.
Single-user deletion uses `DELETE /users/delete` with `{"filter":{"user_id":12}}`.
The former `DELETE /users/{user_id}` route has been removed.

## Password updates

Set `password` inside an update entry's `changes` object:

```json
{"updates":[{"filter":{"user_id":12},"changes":{"password":"Replacement-password1!"}}]}
```

A group filter sets the same supplied password for every selected user. Each user
receives an independently salted hash through the existing password hasher. The
existing new-password policy applies; omitted or null `password` leaves the
password unchanged. Password updates revoke all target sessions and require a
password change at the next sign-in. Profile changes, password changes, session
revocations, and one audit record per user commit in the same transaction. Any
failure rolls back the complete batch. Password values are redacted in Debug
output and excluded from audit details.

The separate `POST /users/{user_id}/password` route has been removed. The account
creation route still accepts its initial password, and `/auth/password` still
handles a user's own password change.

## Role assignments

`POST /users/create` accepts optional `role_ids` to assign roles during creation.
Omission or null creates the user without roles. A supplied array, including an
empty array, requires `UserRolesUpdate` in addition to `UsersCreate`.

`PATCH /users/update` accepts `role_ids` and `expected_role_ids` together in
`changes`. Both arrays are required when changing roles. Omitted or null arrays
leave roles unchanged; an empty `role_ids` array removes all roles. The expected
array must match each selected user's current role set, irrespective of order.
A group filter replaces every selected user's roles with the supplied set.

```json
{"updates":[{"filter":{"user_id":12},"changes":{"expected_role_ids":[1],"role_ids":[2,3]}}]}
```

Role updates require both `UsersUpdate` and `UserRolesUpdate`. Duplicate identifiers
and unknown roles are rejected. A stale expected set returns a conflict and rolls
back the entire request. Session revocation and audit commit with the user changes.
The transaction must preserve an active administrator when one existed before it;
this is checked against the final batch state so an administrator role can be
transferred between users atomically. The former `PUT /users/{user_id}/roles` route
is removed; the HTML assignment form uses the shared update workflow.

## Role reads

`GET /roles` has been removed. Use `POST /roles/read` with the generated selection,
filter, ordering, search, and pagination body. The response contains `items`,
`permissions`, and `total`; every selected row includes its `permission_ids`.
The role name search is case-insensitive. Reading requires `RolesRead` and a valid
session. Role creation uses `POST /roles/create`.

### Bulk role creation

`POST /roles/create` accepts a JSON array of 1 to 10,000 role objects,
for example `[{"name":"editor"},{"name":"reviewer"}]`. It returns HTTP 201
with a JSON array of role identifiers in input order. The standard 65,536-byte
request body limit also applies, so the effective batch size depends on name lengths.
All roles are non-system.
Role names use the same validation as single creation; unknown fields are rejected.
The complete batch and one audit entry per role commit in one transaction.
Duplicate or existing names return HTTP 409 and roll back the complete batch.
An empty batch returns HTTP 422. The route requires `roles:create`, an authenticated
session, a valid CSRF token, and the same origin checks as single creation.
`POST /roles` and `POST /roles/create_many` have been removed. To create one role,
send a one-element array to `POST /roles/create`; the response is still an ID array.
The server-rendered create-role form uses the same transactional workflow.

### Filtered role updates

`PATCH /roles/update` accepts
`{"updates":[{"filter":{"role_id":2},"changes":{"name":"editor"}}]}` and returns
HTTP 204 without a body. Filter fields are optional `role_id`, `name`, and
`is_system`; supplied values are combined with AND using exact equality.
At least one non-null filter field is required. Every change requires a valid name.
Unknown fields, empty batches or filters, and overlapping selections return HTTP 422.
The collection and selected-role limits are 10,000; the standard 65,536-byte body limit applies.
All selections are resolved and locked before changes are applied. A missing match,
a system role, or a name conflict returns HTTP 409 and rolls back the entire batch
including its audit entries. Role names are unique, so assigning one name to multiple
selected roles fails atomically. Name swaps that violate the unique constraint are rejected.
The route requires `roles:update`, authentication, CSRF, and the usual origin checks.
The former `PATCH /roles/{role_id}` route and flat-array update body have been removed.
The HTML rename form submits an ID filter to the shared transaction workflow.
