# Route API Error Types

Every route operation has its own concrete `thiserror::Error` boundary type.
Infallible operations use distinct uninhabited error enums rather than sharing
`std::convert::Infallible` or a service-wide error.

## Common operational routes

These logical routes are implemented by `common_routes` and can be mounted by
multiple executable services.

| Method | Path | Error type |
| --- | --- | --- |
| `GET` | `/health/live` | `HealthLiveError` |
| `GET` | `/health/ready` | `HealthReadyError` |
| `GET` | `/health` | `HealthError` |
| `GET` | `/health_check` | `HealthCheckError` |
| `GET` | `/git_info` | `GitInfoError` |

The common-router fallback uses `CommonNotFoundError`.

## Notification service

| Method | Path | Error type |
| --- | --- | --- |
| `POST` | `/notifications` | `CreateNotificationError` |
| `GET` | `/metrics` | `MetricsError` |
| `GET` | `/openapi.json` | `OpenApiError` |

The notification service also mounts the common operational routes listed
above.

## Admin authentication and session API

All paths in this section use the `/api/v1/admin` prefix.

| Method | Path | Error type |
| --- | --- | --- |
| `POST` | `/auth/sign_in` | `AdminSignInError` |
| `POST` | `/auth/refresh` | `AdminRefreshError` |
| `GET` | `/auth/me` | `AdminMeError` |
| `POST` | `/auth/password` | `AdminChangeOwnPasswordError` |
| `POST` | `/auth/sign_out` | `AdminSignOutError` |
| `GET` | `/auth/sessions` | `AdminSessionsError` |
| `DELETE` | `/auth/sessions` | `AdminRevokeAllSessionsError` |
| `DELETE` | `/auth/sessions/{session_id}` | `AdminRevokeSessionError` |

## Admin user API

All paths in this section use the `/api/v1/admin` prefix.

| Method | Path | Error type |
| --- | --- | --- |
| `GET` | `/users` | `AdminListUsersError` |
| `POST` | `/users` | `AdminCreateUserError` |
| `PATCH` | `/users/{user_id}` | `AdminUpdateUserError` |
| `DELETE` | `/users/{user_id}` | `AdminDeleteUserError` |
| `POST` | `/users/{user_id}/password` | `AdminSetUserPasswordError` |
| `POST` | `/users/{user_id}/ban` | `AdminSetUserBanError` |
| `PUT` | `/users/{user_id}/roles` | `AdminSetUserRolesError` |

## Admin role and permission API

All paths in this section use the `/api/v1/admin` prefix.

| Method | Path | Error type |
| --- | --- | --- |
| `GET` | `/roles` | `AdminListRolesError` |
| `POST` | `/roles` | `AdminCreateRoleError` |
| `PATCH` | `/roles/{role_id}` | `AdminUpdateRoleError` |
| `DELETE` | `/roles/{role_id}` | `AdminDeleteRoleError` |
| `PUT` | `/roles/{role_id}/permissions` | `AdminSetRolePermissionsError` |
| `GET` | `/permissions` | `AdminListPermissionsError` |

## Admin audit, settings, and table API

All paths in this section use the `/api/v1/admin` prefix.

| Method | Path | Error type |
| --- | --- | --- |
| `GET` | `/audit_log` | `AdminAuditLogError` |
| `GET` | `/audit_log/export` | `AdminAuditExportError` |
| `GET` | `/branding` | `AdminBrandingError` |
| `GET` | `/tables` | `AdminDataTablesError` |
| `GET` | `/tables/{table}` | `AdminDataTableError` |
| `GET` | `/system_settings` | `AdminSettingsError` |
| `PATCH` | `/system_settings` | `AdminUpdateSettingsError` |
| `GET` | `/metrics` | `AdminMetricsError` |
| `GET` | `/openapi.json` | `AdminGeneratedOpenApiError` |

The OpenAPI route is present only when admin Swagger is enabled.

## Generated read-only admin API

Every generated path includes the `/api/v1/admin` prefix. The `rm` operation
reads many records, while `ro` reads one record.

| Method | Path | Error type |
| --- | --- | --- |
| `POST` | `/admin_users/rm` | `AdminUsersRmError` |
| `POST` | `/admin_users/ro` | `AdminUsersRoError` |
| `POST` | `/admin_user_roles/rm` | `AdminUserRolesRmError` |
| `POST` | `/admin_user_roles/ro` | `AdminUserRolesRoError` |
| `POST` | `/admin_role_permissions/rm` | `AdminRolePermissionsRmError` |
| `POST` | `/admin_role_permissions/ro` | `AdminRolePermissionsRoError` |
| `POST` | `/admin_roles/rm` | `AdminRolesRmError` |
| `POST` | `/admin_roles/ro` | `AdminRolesRoError` |
| `POST` | `/admin_permissions/rm` | `AdminPermissionsRmError` |
| `POST` | `/admin_permissions/ro` | `AdminPermissionsRoError` |
| `POST` | `/admin_system_settings/rm` | `AdminSystemSettingsRmError` |
| `POST` | `/admin_system_settings/ro` | `AdminSystemSettingsRoError` |

## Admin HTML pages

| Method | Path | Error type |
| --- | --- | --- |
| `GET` | `/admin` | `AdminRootPageError` |
| `GET` | `/admin/sign_in` | `AdminSignInPageError` |
| `GET` | `/admin/users` | `AdminUsersPageError` |
| `GET` | `/admin/roles` | `AdminRolesPageError` |
| `GET` | `/admin/permissions` | `AdminPermissionsPageError` |
| `GET` | `/admin/{table}` | `AdminDataTablesPageError` |
| `GET` | `/admin/sessions` | `AdminSessionsPageError` |
| `GET` | `/admin/profile` | `AdminProfilePageError` |
| `GET` | `/admin/settings` | `AdminSettingsPageError` |
| `GET` | `/admin/version` | `AdminVersionPageError` |
| `GET` | `/admin/metrics` | `AdminHtmlMetricsError` |
| `GET` | `/admin/swagger_ui` | `AdminOpenApiPageError` |
| `GET` | `/admin/assets/*path` | `AdminAssetsError` |

The Swagger UI page is present only when admin Swagger is enabled.

## Admin HTML form actions

| Method | Path | Error type |
| --- | --- | --- |
| `POST` | `/admin/actions/sign_in` | `AdminHtmlSignInError` |
| `POST` | `/admin/actions/sign_out` | `AdminHtmlSignOutError` |
| `POST` | `/admin/actions/profile/password` | `AdminHtmlChangePasswordError` |
| `POST` | `/admin/actions/sessions/revoke` | `AdminHtmlRevokeSessionError` |
| `POST` | `/admin/actions/users/create` | `AdminHtmlCreateUserError` |
| `POST` | `/admin/actions/users/update` | `AdminHtmlUpdateUserError` |
| `POST` | `/admin/actions/users/password` | `AdminHtmlUserPasswordError` |
| `POST` | `/admin/actions/users/ban` | `AdminHtmlUserBanError` |
| `POST` | `/admin/actions/users/delete` | `AdminHtmlDeleteUserError` |
| `POST` | `/admin/actions/users/roles` | `AdminHtmlUserRolesError` |
| `POST` | `/admin/actions/roles/create` | `AdminHtmlCreateRoleError` |
| `POST` | `/admin/actions/roles/update` | `AdminHtmlUpdateRoleError` |
| `POST` | `/admin/actions/roles/delete` | `AdminHtmlDeleteRoleError` |
| `POST` | `/admin/actions/roles/permissions` | `AdminHtmlRolePermissionsError` |
| `POST` | `/admin/actions/settings/update` | `AdminHtmlUpdateSettingsError` |

## Reusable runtime routes

These routes are provided by `server_runtime::add_health_routes` but are not
mounted by the two current executable services.

| Method | Path | Error type |
| --- | --- | --- |
| `GET` | `/live` | `server_runtime::health::HealthLiveError` |
| `GET` | `/ready` | `server_runtime::health::HealthReadyError` |

## Enforcement

The workspace code-style policy requires every typed or handler-registry route
to declare a concrete error type and rejects reuse of an error type by another
operation. Generated PostgreSQL routes create their error enums from the table
and operation identifiers, keeping every generated combination distinct.

## Concrete declarations

The declarations below are the handwritten source declarations or the
declarations emitted by the corresponding macro. `From`, `Error`, and
`IntoResponse` implementations are omitted because they do not alter an error
type's fields or variants.

### Common and notification routes

```rust
#[derive(Debug, thiserror::Error)]
enum GitInfoError {}

#[derive(Debug, thiserror::Error)]
enum HealthCheckError {
    #[error("service is unavailable")]
    Unavailable,
}

#[derive(Debug, thiserror::Error)]
enum HealthError {
    #[error("service is unavailable")]
    Unavailable,
}

#[derive(Debug, thiserror::Error)]
enum HealthLiveError {
    #[error("service is unavailable")]
    Unavailable,
}

#[derive(Debug, thiserror::Error)]
enum HealthReadyError {
    #[error("service is unavailable")]
    Unavailable,
}

#[derive(Debug, thiserror::Error)]
enum CommonNotFoundError {
    #[error("common route was not found")]
    NotFound(NotFoundHandle),
}

#[derive(Debug, thiserror::Error)]
enum CreateNotificationError {
    #[error("notification persistence failed: {0}")]
    Persistence(
        #[source]
        server_runtime::ObservedError<SqlxNotificationDatabaseError>,
    ),
    #[error("notification request validation failed")]
    Validation,
}

#[derive(Debug, thiserror::Error)]
enum MetricsError {
    #[error("notification metrics response rendering failed: {0}")]
    Render(
        #[source]
        server_runtime::ObservedError<server_runtime::MetricsResponseBodyError>,
    ),
}

#[derive(Debug, thiserror::Error)]
enum OpenApiError {}
```

### Admin JSON API

`api_operation_error!` emits each route error below as a separate concrete
12-variant enum. No route error contains an `Operation(AdminError)` wrapper.
For every name in the invocation list, the compiler receives this complete
variant declaration:

```rust
#[derive(Debug, thiserror::Error)]
enum RouteSpecificAdminError {
    #[error("administrator authentication failed")]
    Authentication,
    #[error("administrator authorization failed")]
    Authorization,
    #[error("administrator operation conflicts with current state")]
    Conflict,
    #[error("administrator request failed CSRF validation")]
    Csrf,
    #[error("administrator authentication is temporarily rate limited")]
    RateLimited,
    #[error("administrator request validation failed")]
    Validation,
    #[error("administrator API database operation failed: {0:?}")]
    Pg(#[source] server_runtime::ObservedError<SqlxAdminError>),
    #[error("administrator password hashing failed: {0}")]
    PasswordHash(#[source] server_runtime::ObservedError<AdminPasswordHashError>),
    #[error("administrator request body is too large")]
    PayloadTooLarge,
    #[error("administrator route does not support this HTTP method")]
    MethodNotAllowed,
    #[error("administrator session operation failed: {0}")]
    Session(#[source] server_runtime::ObservedError<AdminSessionError>),
    #[error("administrator response header is invalid: {0:?}")]
    Header(#[source] server_runtime::ObservedError<HttpAdminHeaderValueError>),
}
```

`RouteSpecificAdminError` above is a notation placeholder for each concrete
identifier in this exhaustive expansion list:

```rust
api_operation_error!(AdminAuditLogError);
api_operation_error!(AdminAuditExportError);
api_operation_error!(AdminBrandingError);
api_operation_error!(AdminChangeOwnPasswordError);
api_operation_error!(AdminCreateRoleError);
api_operation_error!(AdminCreateUserError);
api_operation_error!(AdminDataTableError);
api_operation_error!(AdminDataTablesError);
api_operation_error!(AdminDeleteRoleError);
api_operation_error!(AdminDeleteUserError);
api_operation_error!(AdminListPermissionsError);
api_operation_error!(AdminListRolesError);
api_operation_error!(AdminListUsersError);
api_operation_error!(AdminMeError);
api_operation_error!(AdminRefreshError);
api_operation_error!(AdminRevokeAllSessionsError);
api_operation_error!(AdminRevokeSessionError);
api_operation_error!(AdminSessionsError);
api_operation_error!(AdminSetRolePermissionsError);
api_operation_error!(AdminSetUserBanError);
api_operation_error!(AdminSetUserPasswordError);
api_operation_error!(AdminSetUserRolesError);
api_operation_error!(AdminSettingsError);
api_operation_error!(AdminSignInError);
api_operation_error!(AdminSignOutError);
api_operation_error!(AdminUpdateRoleError);
api_operation_error!(AdminUpdateSettingsError);
api_operation_error!(AdminUpdateUserError);
```

```rust
#[derive(Debug, thiserror::Error)]
enum AdminMetricsError {
    #[error(transparent)]
    Render(server_runtime::MetricsResponseBodyError),
}

#[derive(Debug, thiserror::Error)]
enum AdminGeneratedOpenApiError {}
```

### Generated read-only admin API

These are the concrete enum declarations emitted by the PostgreSQL CRUD
generator. Generator-only field attributes are omitted; all variants and field
types are shown.

```rust
pub enum AdminUsersRmError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::location::Location },
    Pg { pg: sqlx::Error, location: location_lib::location::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::location::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::location::Location },
    NotUniqueField { location: location_lib::location::Location, not_unique_field: AdminUsersSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::location::Location },
    TryBind { try_bind: String, location: location_lib::location::Location },
}

pub enum AdminUsersRoError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::location::Location },
    Pg { pg: sqlx::Error, location: location_lib::location::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::location::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::location::Location },
    NotUniqueField { location: location_lib::location::Location, not_unique_field: AdminUsersSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::location::Location },
    TryBind { try_bind: String, location: location_lib::location::Location },
}

pub enum AdminUserRolesRmError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::location::Location },
    Pg { pg: sqlx::Error, location: location_lib::location::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::location::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::location::Location },
    NotUniqueField { location: location_lib::location::Location, not_unique_field: AdminUserRolesSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::location::Location },
    TryBind { try_bind: String, location: location_lib::location::Location },
}

pub enum AdminUserRolesRoError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::location::Location },
    Pg { pg: sqlx::Error, location: location_lib::location::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::location::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::location::Location },
    NotUniqueField { location: location_lib::location::Location, not_unique_field: AdminUserRolesSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::location::Location },
    TryBind { try_bind: String, location: location_lib::location::Location },
}

pub enum AdminRolePermissionsRmError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::location::Location },
    Pg { pg: sqlx::Error, location: location_lib::location::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::location::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::location::Location },
    NotUniqueField { location: location_lib::location::Location, not_unique_field: AdminRolePermissionsSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::location::Location },
    TryBind { try_bind: String, location: location_lib::location::Location },
}

pub enum AdminRolePermissionsRoError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::location::Location },
    Pg { pg: sqlx::Error, location: location_lib::location::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::location::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::location::Location },
    NotUniqueField { location: location_lib::location::Location, not_unique_field: AdminRolePermissionsSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::location::Location },
    TryBind { try_bind: String, location: location_lib::location::Location },
}

pub enum AdminRolesRmError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::location::Location },
    Pg { pg: sqlx::Error, location: location_lib::location::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::location::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::location::Location },
    NotUniqueField { location: location_lib::location::Location, not_unique_field: AdminRolesSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::location::Location },
    TryBind { try_bind: String, location: location_lib::location::Location },
}

pub enum AdminRolesRoError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::location::Location },
    Pg { pg: sqlx::Error, location: location_lib::location::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::location::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::location::Location },
    NotUniqueField { location: location_lib::location::Location, not_unique_field: AdminRolesSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::location::Location },
    TryBind { try_bind: String, location: location_lib::location::Location },
}

pub enum AdminPermissionsRmError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::location::Location },
    Pg { pg: sqlx::Error, location: location_lib::location::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::location::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::location::Location },
    NotUniqueField { location: location_lib::location::Location, not_unique_field: AdminPermissionsSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::location::Location },
    TryBind { try_bind: String, location: location_lib::location::Location },
}

pub enum AdminPermissionsRoError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::location::Location },
    Pg { pg: sqlx::Error, location: location_lib::location::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::location::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::location::Location },
    NotUniqueField { location: location_lib::location::Location, not_unique_field: AdminPermissionsSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::location::Location },
    TryBind { try_bind: String, location: location_lib::location::Location },
}

pub enum AdminSystemSettingsRmError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::location::Location },
    Pg { pg: sqlx::Error, location: location_lib::location::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::location::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::location::Location },
    NotUniqueField { location: location_lib::location::Location, not_unique_field: AdminSystemSettingsSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::location::Location },
    TryBind { try_bind: String, location: location_lib::location::Location },
}

pub enum AdminSystemSettingsRoError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::location::Location },
    Pg { pg: sqlx::Error, location: location_lib::location::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::location::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::location::Location },
    NotUniqueField { location: location_lib::location::Location, not_unique_field: AdminSystemSettingsSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::location::Location },
    TryBind { try_bind: String, location: location_lib::location::Location },
}
```

### Admin HTML pages and form actions

`route_error` emits each route error below as a separate concrete uninhabited
enum:

```rust
#[derive(Debug, thiserror::Error)]
enum AdminRootPageError {}
#[derive(Debug, thiserror::Error)]
enum AdminSignInPageError {}
#[derive(Debug, thiserror::Error)]
enum AdminUsersPageError {}
#[derive(Debug, thiserror::Error)]
enum AdminRolesPageError {}
#[derive(Debug, thiserror::Error)]
enum AdminPermissionsPageError {}
#[derive(Debug, thiserror::Error)]
enum AdminDataTablesPageError {}
#[derive(Debug, thiserror::Error)]
enum AdminSessionsPageError {}
#[derive(Debug, thiserror::Error)]
enum AdminProfilePageError {}
#[derive(Debug, thiserror::Error)]
enum AdminSettingsPageError {}
#[derive(Debug, thiserror::Error)]
enum AdminVersionPageError {}
#[derive(Debug, thiserror::Error)]
enum AdminOpenApiPageError {}
#[derive(Debug, thiserror::Error)]
enum AdminHtmlSignInError {}
#[derive(Debug, thiserror::Error)]
enum AdminHtmlSignOutError {}
#[derive(Debug, thiserror::Error)]
enum AdminHtmlChangePasswordError {}
#[derive(Debug, thiserror::Error)]
enum AdminHtmlRevokeSessionError {}
#[derive(Debug, thiserror::Error)]
enum AdminHtmlCreateUserError {}
#[derive(Debug, thiserror::Error)]
enum AdminHtmlUpdateUserError {}
#[derive(Debug, thiserror::Error)]
enum AdminHtmlUserPasswordError {}
#[derive(Debug, thiserror::Error)]
enum AdminHtmlUserBanError {}
#[derive(Debug, thiserror::Error)]
enum AdminHtmlDeleteUserError {}
#[derive(Debug, thiserror::Error)]
enum AdminHtmlUserRolesError {}
#[derive(Debug, thiserror::Error)]
enum AdminHtmlCreateRoleError {}
#[derive(Debug, thiserror::Error)]
enum AdminHtmlUpdateRoleError {}
#[derive(Debug, thiserror::Error)]
enum AdminHtmlDeleteRoleError {}
#[derive(Debug, thiserror::Error)]
enum AdminHtmlRolePermissionsError {}
#[derive(Debug, thiserror::Error)]
enum AdminHtmlUpdateSettingsError {}

#[derive(Debug, thiserror::Error)]
enum AdminHtmlMetricsError {}

#[derive(Debug, thiserror::Error)]
enum AdminAssetsError {
    #[error("administrator asset read failed: {0}")]
    Read(to_err_string::ErrorText),
}
```

### Reusable runtime routes

These declarations belong to `server_runtime::health` and are distinct from
the similarly named `common_routes` errors:

```rust
#[derive(Debug, thiserror::Error)]
enum HealthReadyError {
    #[error("service is unavailable")]
    Unavailable(HealthSnapshot),
}

#[derive(Debug, thiserror::Error)]
enum HealthLiveError {}
```
