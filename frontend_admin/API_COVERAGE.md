# Administrator API coverage audit

The typed registry in `server_admin/src/admin_auth_route_registry.rs` registers 28 API
operations. Paths below are relative to `/v1/admin`. The frontend must follow the route,
permission, request, and table catalogs in `server_admin_contract`.

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
| GET `/users` | Users list, pagination, and user management |
| POST `/users` | Create-user page; server-rendered HTML adapter |
| PATCH `/users/{user_id}` | Manage-users page: login and display name |
| DELETE `/users/{user_id}` | Manage-users page: delete account |
| POST `/users/{user_id}/password` | Manage-users page: reset password |
| POST `/users/{user_id}/ban` | Manage-users page: ban/unban account |
| PUT `/users/{user_id}/roles` | Manage-users page: role assignment |
| GET `/roles` | Roles list, pagination, and role management |
| POST `/roles` | Create-role page; server-rendered HTML adapter |
| PATCH `/roles/{role_id}` | Manage-roles page: rename role |
| DELETE `/roles/{role_id}` | Manage-roles page: delete role |
| PUT `/roles/{role_id}/permissions` | Manage-roles page: permission assignment |
| GET `/permissions` | Read-only permissions list |
| GET `/audit_log` | Audit view uses the catalog table endpoint; browser coverage also verifies the dedicated query and redacted mutation records |
| GET `/audit_log/export` | Prepare/download controls export the current audit page using its limit and offset; export permission governs visibility |
| GET `/system_settings` | Settings form |
| PATCH `/system_settings` | Save settings and reset supported settings to defaults |
| GET `/branding` | Shared branding in server-rendered pages |
| GET `/tables` | Catalog defines available database views; navigation follows the shared table specification |
| GET `/tables/{table}` | Catalog-driven columns, filters, ordering, and pagination |

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
