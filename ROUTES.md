# HTTP Routes

This workspace contains two executable HTTP services: the main server and the
notification service.

## Main server

### Public operational routes

| Method | Path |
| --- | --- |
| `GET` | `/health/live` |
| `GET` | `/health/ready` |
| `GET` | `/health` |
| `GET` | `/health_check` |
| `GET` | `/git_info` |

### Admin authentication and session API

All routes in this section use the `/api/v1/admin` prefix.

| Method | Path |
| --- | --- |
| `POST` | `/auth/sign_in` |
| `POST` | `/auth/refresh` |
| `GET` | `/auth/me` |
| `POST` | `/auth/password` |
| `POST` | `/auth/sign_out` |
| `GET` | `/auth/sessions` |
| `DELETE` | `/auth/sessions` |
| `DELETE` | `/auth/sessions/{session_id}` |

### Admin user API

All routes in this section use the `/api/v1/admin` prefix.

| Method | Path |
| --- | --- |
| `GET` | `/users` |
| `POST` | `/users` |
| `PATCH` | `/users/{user_id}` |
| `DELETE` | `/users/{user_id}` |
| `POST` | `/users/{user_id}/password` |
| `POST` | `/users/{user_id}/ban` |
| `PUT` | `/users/{user_id}/roles` |

### Admin role and permission API

All routes in this section use the `/api/v1/admin` prefix.

| Method | Path |
| --- | --- |
| `GET` | `/roles` |
| `POST` | `/roles` |
| `PATCH` | `/roles/{role_id}` |
| `DELETE` | `/roles/{role_id}` |
| `PUT` | `/roles/{role_id}/permissions` |
| `GET` | `/permissions` |

### Admin audit, settings, and table API

All routes in this section use the `/api/v1/admin` prefix.

| Method | Path | Condition |
| --- | --- | --- |
| `GET` | `/audit_log` | Always |
| `GET` | `/audit_log/export` | Always |
| `GET` | `/branding` | Always |
| `GET` | `/tables` | Always |
| `GET` | `/tables/{table}` | Always |
| `GET` | `/system_settings` | Always |
| `PATCH` | `/system_settings` | Always |
| `GET` | `/metrics` | Always |
| `GET` | `/openapi.json` | Admin Swagger enabled |

### Generated read-only admin API

The generated admin tables expose two operations:

- `rm`: read many
- `ro`: read one

| Method | Path |
| --- | --- |
| `POST` | `/api/v1/admin/admin_users/rm` |
| `POST` | `/api/v1/admin/admin_users/ro` |
| `POST` | `/api/v1/admin/admin_user_roles/rm` |
| `POST` | `/api/v1/admin/admin_user_roles/ro` |
| `POST` | `/api/v1/admin/admin_role_permissions/rm` |
| `POST` | `/api/v1/admin/admin_role_permissions/ro` |
| `POST` | `/api/v1/admin/admin_roles/rm` |
| `POST` | `/api/v1/admin/admin_roles/ro` |
| `POST` | `/api/v1/admin/admin_permissions/rm` |
| `POST` | `/api/v1/admin/admin_permissions/ro` |
| `POST` | `/api/v1/admin/admin_system_settings/rm` |
| `POST` | `/api/v1/admin/admin_system_settings/ro` |

## Admin HTML routes

### Pages

| Method | Path | Notes |
| --- | --- | --- |
| `GET` | `/admin` | Redirects to the users page |
| `GET` | `/admin/sign_in` | Sign-in page |
| `GET` | `/admin/users` | Users page |
| `GET` | `/admin/roles` | Roles page |
| `GET` | `/admin/permissions` | Permissions page |
| `GET` | `/admin/sessions` | Sessions page |
| `GET` | `/admin/profile` | Profile page |
| `GET` | `/admin/settings` | Settings page |
| `GET` | `/admin/version` | Version page |
| `GET` | `/admin/metrics` | Metrics page |
| `GET` | `/admin/swagger_ui` | Admin Swagger enabled |
| `GET` | `/admin/{table}` | Generic data-table page |
| `GET` | `/admin/assets/*path` | Static frontend assets |

Valid values for the `{table}` parameter are:

- `access_sessions`
- `audit_log`
- `cleanup_status`
- `login_attempts`
- `permissions`
- `rate_limits`
- `refresh_tokens`
- `role_permissions`
- `roles`
- `system_settings`
- `user_roles`
- `users`

### Form actions

| Method | Path |
| --- | --- |
| `POST` | `/admin/actions/sign_in` |
| `POST` | `/admin/actions/sign_out` |
| `POST` | `/admin/actions/profile/password` |
| `POST` | `/admin/actions/sessions/revoke` |
| `POST` | `/admin/actions/users/create` |
| `POST` | `/admin/actions/users/update` |
| `POST` | `/admin/actions/users/password` |
| `POST` | `/admin/actions/users/ban` |
| `POST` | `/admin/actions/users/delete` |
| `POST` | `/admin/actions/users/roles` |
| `POST` | `/admin/actions/roles/create` |
| `POST` | `/admin/actions/roles/update` |
| `POST` | `/admin/actions/roles/delete` |
| `POST` | `/admin/actions/roles/permissions` |
| `POST` | `/admin/actions/settings/update` |

## Notification service

| Method | Path |
| --- | --- |
| `POST` | `/notifications` |
| `GET` | `/metrics` |
| `GET` | `/openapi.json` |
| `GET` | `/health/live` |
| `GET` | `/health/ready` |
| `GET` | `/health` |
| `GET` | `/health_check` |
| `GET` | `/git_info` |

## Reusable runtime routes

`server_runtime::add_health_routes` provides the following routes for routers
that opt into it. They are not mounted by the two current executable services.

| Method | Path |
| --- | --- |
| `GET` | `/live` |
| `GET` | `/ready` |

## Route error types

Every route operation has a distinct `thiserror::Error` boundary type. An
infallible operation uses its own uninhabited error enum instead of sharing
`Infallible` or a service-wide error.

### Public and notification routes

| Route operation | Error type |
| --- | --- |
| `GET /health/live` | `HealthLiveError` |
| `GET /health/ready` | `HealthReadyError` |
| `GET /health` | `HealthError` |
| `GET /health_check` | `HealthCheckError` |
| `GET /git_info` | `GitInfoError` |
| `POST /notifications` | `CreateNotificationError` |
| Notification `GET /metrics` | `MetricsError` |
| Notification `GET /openapi.json` | `OpenApiError` |
| Runtime-only `GET /live` | `server_runtime::health::HealthLiveError` |
| Runtime-only `GET /ready` | `server_runtime::health::HealthReadyError` |

The common route types are implemented once in `common_routes` and reused when
the same logical common route registry is mounted by multiple services.

### Handwritten admin API routes

| Route operation | Error type |
| --- | --- |
| `POST /auth/sign_in` | `AdminSignInError` |
| `POST /auth/refresh` | `AdminRefreshError` |
| `GET /auth/me` | `AdminMeError` |
| `POST /auth/password` | `AdminChangeOwnPasswordError` |
| `POST /auth/sign_out` | `AdminSignOutError` |
| `GET /auth/sessions` | `AdminSessionsError` |
| `DELETE /auth/sessions` | `AdminRevokeAllSessionsError` |
| `DELETE /auth/sessions/{session_id}` | `AdminRevokeSessionError` |
| `GET /users` | `AdminListUsersError` |
| `POST /users` | `AdminCreateUserError` |
| `PATCH /users/{user_id}` | `AdminUpdateUserError` |
| `DELETE /users/{user_id}` | `AdminDeleteUserError` |
| `POST /users/{user_id}/password` | `AdminSetUserPasswordError` |
| `POST /users/{user_id}/ban` | `AdminSetUserBanError` |
| `PUT /users/{user_id}/roles` | `AdminSetUserRolesError` |
| `GET /roles` | `AdminListRolesError` |
| `POST /roles` | `AdminCreateRoleError` |
| `PATCH /roles/{role_id}` | `AdminUpdateRoleError` |
| `DELETE /roles/{role_id}` | `AdminDeleteRoleError` |
| `PUT /roles/{role_id}/permissions` | `AdminSetRolePermissionsError` |
| `GET /permissions` | `AdminListPermissionsError` |
| `GET /audit_log` | `AdminAuditLogError` |
| `GET /audit_log/export` | `AdminAuditExportError` |
| `GET /branding` | `AdminBrandingError` |
| `GET /tables` | `AdminDataTablesError` |
| `GET /tables/{table}` | `AdminDataTableError` |
| `GET /system_settings` | `AdminSettingsError` |
| `PATCH /system_settings` | `AdminUpdateSettingsError` |
| Admin API `GET /metrics` | `AdminMetricsError` |
| Admin API `GET /openapi.json` | `AdminGeneratedOpenApiError` |

### Generated admin API routes

Each generated table and operation combination produces its own error enum:

| Route suffix | Error-type suffix |
| --- | --- |
| `admin_users/rm` | `AdminUsersRmError` |
| `admin_users/ro` | `AdminUsersRoError` |
| `admin_user_roles/rm` | `AdminUserRolesRmError` |
| `admin_user_roles/ro` | `AdminUserRolesRoError` |
| `admin_role_permissions/rm` | `AdminRolePermissionsRmError` |
| `admin_role_permissions/ro` | `AdminRolePermissionsRoError` |
| `admin_roles/rm` | `AdminRolesRmError` |
| `admin_roles/ro` | `AdminRolesRoError` |
| `admin_permissions/rm` | `AdminPermissionsRmError` |
| `admin_permissions/ro` | `AdminPermissionsRoError` |
| `admin_system_settings/rm` | `AdminSystemSettingsRmError` |
| `admin_system_settings/ro` | `AdminSystemSettingsRoError` |

### Admin HTML routes

| Route operation | Error type |
| --- | --- |
| `GET /admin` | `AdminRootPageError` |
| `GET /admin/sign_in` | `AdminSignInPageError` |
| `GET /admin/users` | `AdminUsersPageError` |
| `GET /admin/roles` | `AdminRolesPageError` |
| `GET /admin/permissions` | `AdminPermissionsPageError` |
| `GET /admin/{table}` | `AdminDataTablesPageError` |
| `GET /admin/sessions` | `AdminSessionsPageError` |
| `GET /admin/profile` | `AdminProfilePageError` |
| `GET /admin/settings` | `AdminSettingsPageError` |
| `GET /admin/version` | `AdminVersionPageError` |
| `GET /admin/metrics` | `AdminHtmlMetricsError` |
| `GET /admin/swagger_ui` | `AdminOpenApiPageError` |
| `GET /admin/assets/*path` | `AdminAssetsError` |
| `POST /admin/actions/sign_in` | `AdminHtmlSignInError` |
| `POST /admin/actions/sign_out` | `AdminHtmlSignOutError` |
| `POST /admin/actions/profile/password` | `AdminHtmlChangePasswordError` |
| `POST /admin/actions/sessions/revoke` | `AdminHtmlRevokeSessionError` |
| `POST /admin/actions/users/create` | `AdminHtmlCreateUserError` |
| `POST /admin/actions/users/update` | `AdminHtmlUpdateUserError` |
| `POST /admin/actions/users/password` | `AdminHtmlUserPasswordError` |
| `POST /admin/actions/users/ban` | `AdminHtmlUserBanError` |
| `POST /admin/actions/users/delete` | `AdminHtmlDeleteUserError` |
| `POST /admin/actions/users/roles` | `AdminHtmlUserRolesError` |
| `POST /admin/actions/roles/create` | `AdminHtmlCreateRoleError` |
| `POST /admin/actions/roles/update` | `AdminHtmlUpdateRoleError` |
| `POST /admin/actions/roles/delete` | `AdminHtmlDeleteRoleError` |
| `POST /admin/actions/roles/permissions` | `AdminHtmlRolePermissionsError` |
| `POST /admin/actions/settings/update` | `AdminHtmlUpdateSettingsError` |
