# Route API Error Types

Every fallible route operation has its own concrete `thiserror::Error` boundary
type. Infallible operations return their response directly and do not declare a
fake error type.

## Common operational routes

These logical routes are implemented by `common_routes` and can be mounted by
multiple executable services.

| Method | Path | Error type |
| --- | --- | --- |
| `GET` | `/health/live` | `HealthLiveError` |
| `GET` | `/health/ready` | `HealthReadyError` |
| `GET` | `/health` | `HealthError` |
| `GET` | `/health_check` | `HealthCheckError` |
| `GET` | `/git_info` | Infallible; no error type |

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

All paths in this section use the `/v1/admin` prefix.

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

All paths in this section use the `/v1/admin` prefix.

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

All paths in this section use the `/v1/admin` prefix.

| Method | Path | Error type |
| --- | --- | --- |
| `GET` | `/roles` | `AdminListRolesError` |
| `POST` | `/roles` | `AdminCreateRoleError` |
| `PATCH` | `/roles/{role_id}` | `AdminUpdateRoleError` |
| `DELETE` | `/roles/{role_id}` | `AdminDeleteRoleError` |
| `PUT` | `/roles/{role_id}/permissions` | `AdminSetRolePermissionsError` |
| `GET` | `/permissions` | `AdminListPermissionsError` |

## Admin audit, settings, and table API

All paths in this section use the `/v1/admin` prefix.

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

Every generated path includes the `/v1/admin` prefix. The `rm` operation
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
| `GET` | `/admin` | Infallible; no error type |
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
| `GET` | `/live` | Infallible; no error type |
| `GET` | `/ready` | `server_runtime::health::HealthReadyError` |

## Enforcement

The workspace code-style policy requires every fallible typed or
handler-registry route to declare a concrete error type and rejects reuse of an
error type by another operation. Infallible handlers declare the route
operation but return their response directly. Generated PostgreSQL routes
create their error enums from the table and operation identifiers, keeping
every generated combination distinct.

## Concrete declarations

The declarations below are the handwritten source declarations or the
declarations emitted by the corresponding macro. `From`, `Error`, and
`IntoResponse` implementations are omitted because they do not alter an error
type's fields or variants.

### Common and notification routes

```rust
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
enum OpenApiError {
    #[error("notification OpenAPI serialization failed: {0}")]
    Serialization(#[source] serde_json::Error),
}
```

The infallible Git information handler has no error declaration:

```rust
async fn git_info(app_state: ArcCommonRoutesAppState) -> JsonRes<GitInfo> {
    mk_commit_json_res(app_state.0.as_ref(), mk_git_info_payload)
}
```

`OpenApiError` is not genuinely infallible. The current handler passes the
document directly to `axum::Json`, whose `IntoResponse` implementation performs
serialization and converts a serialization failure into a response internally.
To preserve the route error boundary, the handler must serialize explicitly
and map `serde_json::Error` to `OpenApiError::Serialization`.

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
enum AdminGeneratedOpenApiError {
    #[error("administrator OpenAPI serialization failed: {0}")]
    Serialization(#[source] serde_json::Error),
}
```

Like the notification OpenAPI route, the current generated admin OpenAPI
handler delegates serialization to `axum::Json`. Its documented
`Serialization` variant is the failure that must be surfaced when that
serialization is moved before response construction.

### Generated read-only admin API

These are the concrete enum declarations emitted by the PostgreSQL CRUD
generator. Generator-only field attributes are omitted; all variants and field
types are shown.

```rust
pub enum AdminUsersRmError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::domain_types::Location },
    Pg { pg: sqlx::Error, location: location_lib::domain_types::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::domain_types::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::domain_types::Location },
    NotUniqueField { location: location_lib::domain_types::Location, not_unique_field: AdminUsersSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::domain_types::Location },
    TryBind { try_bind: String, location: location_lib::domain_types::Location },
}

pub enum AdminUsersRoError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::domain_types::Location },
    Pg { pg: sqlx::Error, location: location_lib::domain_types::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::domain_types::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::domain_types::Location },
    NotUniqueField { location: location_lib::domain_types::Location, not_unique_field: AdminUsersSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::domain_types::Location },
    TryBind { try_bind: String, location: location_lib::domain_types::Location },
}

pub enum AdminUserRolesRmError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::domain_types::Location },
    Pg { pg: sqlx::Error, location: location_lib::domain_types::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::domain_types::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::domain_types::Location },
    NotUniqueField { location: location_lib::domain_types::Location, not_unique_field: AdminUserRolesSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::domain_types::Location },
    TryBind { try_bind: String, location: location_lib::domain_types::Location },
}

pub enum AdminUserRolesRoError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::domain_types::Location },
    Pg { pg: sqlx::Error, location: location_lib::domain_types::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::domain_types::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::domain_types::Location },
    NotUniqueField { location: location_lib::domain_types::Location, not_unique_field: AdminUserRolesSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::domain_types::Location },
    TryBind { try_bind: String, location: location_lib::domain_types::Location },
}

pub enum AdminRolePermissionsRmError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::domain_types::Location },
    Pg { pg: sqlx::Error, location: location_lib::domain_types::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::domain_types::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::domain_types::Location },
    NotUniqueField { location: location_lib::domain_types::Location, not_unique_field: AdminRolePermissionsSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::domain_types::Location },
    TryBind { try_bind: String, location: location_lib::domain_types::Location },
}

pub enum AdminRolePermissionsRoError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::domain_types::Location },
    Pg { pg: sqlx::Error, location: location_lib::domain_types::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::domain_types::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::domain_types::Location },
    NotUniqueField { location: location_lib::domain_types::Location, not_unique_field: AdminRolePermissionsSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::domain_types::Location },
    TryBind { try_bind: String, location: location_lib::domain_types::Location },
}

pub enum AdminRolesRmError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::domain_types::Location },
    Pg { pg: sqlx::Error, location: location_lib::domain_types::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::domain_types::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::domain_types::Location },
    NotUniqueField { location: location_lib::domain_types::Location, not_unique_field: AdminRolesSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::domain_types::Location },
    TryBind { try_bind: String, location: location_lib::domain_types::Location },
}

pub enum AdminRolesRoError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::domain_types::Location },
    Pg { pg: sqlx::Error, location: location_lib::domain_types::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::domain_types::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::domain_types::Location },
    NotUniqueField { location: location_lib::domain_types::Location, not_unique_field: AdminRolesSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::domain_types::Location },
    TryBind { try_bind: String, location: location_lib::domain_types::Location },
}

pub enum AdminPermissionsRmError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::domain_types::Location },
    Pg { pg: sqlx::Error, location: location_lib::domain_types::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::domain_types::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::domain_types::Location },
    NotUniqueField { location: location_lib::domain_types::Location, not_unique_field: AdminPermissionsSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::domain_types::Location },
    TryBind { try_bind: String, location: location_lib::domain_types::Location },
}

pub enum AdminPermissionsRoError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::domain_types::Location },
    Pg { pg: sqlx::Error, location: location_lib::domain_types::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::domain_types::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::domain_types::Location },
    NotUniqueField { location: location_lib::domain_types::Location, not_unique_field: AdminPermissionsSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::domain_types::Location },
    TryBind { try_bind: String, location: location_lib::domain_types::Location },
}

pub enum AdminSystemSettingsRmError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::domain_types::Location },
    Pg { pg: sqlx::Error, location: location_lib::domain_types::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::domain_types::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::domain_types::Location },
    NotUniqueField { location: location_lib::domain_types::Location, not_unique_field: AdminSystemSettingsSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::domain_types::Location },
    TryBind { try_bind: String, location: location_lib::domain_types::Location },
}

pub enum AdminSystemSettingsRoError {
    CheckBodySize { check_body_size: route_validators::check_body_size::BodySizeError, location: location_lib::domain_types::Location },
    Pg { pg: sqlx::Error, location: location_lib::domain_types::Location },
    SerdeJson { serde_json: serde_json::Error, location: location_lib::domain_types::Location },
    HeaderContentTypeAppJsonNotFound { location: location_lib::domain_types::Location },
    NotUniqueField { location: location_lib::domain_types::Location, not_unique_field: AdminSystemSettingsSelect },
    QueryPart { error: pg_crud_common::QueryPartError, location: location_lib::domain_types::Location },
    TryBind { try_bind: String, location: location_lib::domain_types::Location },
}
```

### Admin HTML pages and form actions

Fallible HTML handlers need route-specific errors. The admin root route is
genuinely infallible: it constructs a redirect from a static, repository-owned
path and performs no parsing, I/O, authentication, database access, or other
fallible operation. It therefore returns a response directly:

```rust
#[frontend_contract::route_operation]
async fn root() -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Redirect::to(
        server_admin_contract::AdminFrontendPath::Users.get(),
    ))
}
```

No `AdminRootPageError` exists. A handler that catches another error and
converts it into a `Response` is operationally fallible and needs its own
concrete variants.

For example, `AdminUsersPageError` must be derived from its actual call graph:
`users` calls `csr_page`, which calls `page_context`, `me_view`,
`branding_view`, authentication, and repository reads. It also performs an
explicit page and table authorization check. Its route-specific declaration
should therefore be:

```rust
#[derive(Debug, thiserror::Error)]
enum AdminUsersPageError {
    #[error("administrator authentication failed")]
    Authentication,
    #[error("administrator authorization failed")]
    Authorization,
    #[error("administrator users page data is invalid")]
    Validation,
    #[error("administrator users page database operation failed: {0:?}")]
    Pg(
        #[source]
        server_runtime::ObservedError<super::super::SqlxAdminError>,
    ),
}
```

These variants come directly from the code:

- `Authentication` comes from `authenticate` and
  `load_authenticated_admin_from_db`.
- `Authorization` comes from the failed `AdminPage::Tables` or
  `AdminDataTable::Users` access check in `csr_page`.
- `Validation` comes from `authenticated_admin_contract` and invalid stored
  branding values.
- `Pg` comes from session, user, and branding repository reads.

The current `route_error` implementation still needs to be changed to emit and
propagate the fallible declarations below.

The common field-bearing variants use these concrete types:

```rust
Pg(
    #[source]
    server_runtime::ObservedError<super::super::SqlxAdminError>,
)
PasswordHash(
    #[source]
    server_runtime::ObservedError<super::super::AdminPasswordHashError>,
)
Session(
    #[source]
    server_runtime::ObservedError<super::AdminSessionError>,
)
Header(
    #[source]
    server_runtime::ObservedError<super::HttpAdminHeaderValueError>,
)
SsrText(
    #[source]
    server_admin_frontend::ssr::AdminSsrTextTryFromStringError,
)
SsrMessage(
    #[source]
    to_err_string::ErrorTextTryFromStringError,
)
Serialization(#[source] serde_json::Error)
```

The page-route declarations derived from their current code paths are:

```rust
#[derive(Debug, thiserror::Error)]
enum AdminSignInPageError {
    Authentication,
    Validation,
    Pg(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminRolesPageError {
    Authentication,
    Authorization,
    Validation,
    Pg(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminPermissionsPageError {
    Authentication,
    Authorization,
    Validation,
    Pg(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminDataTablesPageError {
    Authentication,
    Authorization,
    Validation,
    Pg(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminSessionsPageError {
    Authentication,
    Authorization,
    Validation,
    Pg(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminProfilePageError {
    Authentication,
    Authorization,
    Validation,
    Pg(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminSettingsPageError {
    Authentication,
    Authorization,
    Validation,
    Pg(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminVersionPageError {
    Authentication,
    Validation,
    Pg(/* concrete type above */),
    SsrText(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminOpenApiPageError {
    Authentication,
    Authorization,
    Validation,
    Pg(/* concrete type above */),
    Serialization(/* concrete type above */),
    SsrText(/* concrete type above */),
}
```

`AdminUsersPageError` has the equivalent data-backed page variants shown in
full above. The authorization variants for these pages come from `csr_page`;
the authentication, validation, and database variants come from
`page_context`, `me_view`, `branding_view`, and their repository calls.

The form-action declarations derived from the current handler and extractor
paths are:

```rust
#[derive(Debug, thiserror::Error)]
enum AdminHtmlSignInError {
    Authentication,
    RateLimited,
    Validation,
    Pg(/* concrete type above */),
    PasswordHash(/* concrete type above */),
    PayloadTooLarge,
    Session(/* concrete type above */),
    Header(/* concrete type above */),
    SsrMessage(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminHtmlSignOutError {
    Authentication,
    Csrf,
    Pg(/* concrete type above */),
    Header(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminHtmlChangePasswordError {
    Authentication,
    Conflict,
    Csrf,
    RateLimited,
    Validation,
    Pg(/* concrete type above */),
    PasswordHash(/* concrete type above */),
    PayloadTooLarge,
    Header(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminHtmlRevokeSessionError {
    Authentication,
    Csrf,
    Validation,
    Pg(/* concrete type above */),
    PayloadTooLarge,
    Header(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminHtmlCreateUserError {
    Authentication,
    Authorization,
    Csrf,
    RateLimited,
    Validation,
    Pg(/* concrete type above */),
    PasswordHash(/* concrete type above */),
    PayloadTooLarge,
    Header(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminHtmlUpdateUserError {
    Authentication,
    Authorization,
    Conflict,
    Csrf,
    RateLimited,
    Validation,
    Pg(/* concrete type above */),
    PayloadTooLarge,
    Header(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminHtmlUserPasswordError {
    Authentication,
    Authorization,
    Conflict,
    Csrf,
    RateLimited,
    Validation,
    Pg(/* concrete type above */),
    PasswordHash(/* concrete type above */),
    PayloadTooLarge,
    Header(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminHtmlUserBanError {
    Authentication,
    Authorization,
    Conflict,
    Csrf,
    RateLimited,
    Validation,
    Pg(/* concrete type above */),
    PayloadTooLarge,
    Header(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminHtmlDeleteUserError {
    Authentication,
    Authorization,
    Conflict,
    Csrf,
    RateLimited,
    Validation,
    Pg(/* concrete type above */),
    PayloadTooLarge,
    Header(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminHtmlUserRolesError {
    Authentication,
    Authorization,
    Conflict,
    Csrf,
    RateLimited,
    Validation,
    Pg(/* concrete type above */),
    PayloadTooLarge,
    Header(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminHtmlCreateRoleError {
    Authentication,
    Authorization,
    Csrf,
    RateLimited,
    Validation,
    Pg(/* concrete type above */),
    PayloadTooLarge,
    Header(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminHtmlUpdateRoleError {
    Authentication,
    Authorization,
    Conflict,
    Csrf,
    RateLimited,
    Validation,
    Pg(/* concrete type above */),
    PayloadTooLarge,
    Header(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminHtmlDeleteRoleError {
    Authentication,
    Authorization,
    Conflict,
    Csrf,
    RateLimited,
    Validation,
    Pg(/* concrete type above */),
    PayloadTooLarge,
    Header(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminHtmlRolePermissionsError {
    Authentication,
    Authorization,
    Conflict,
    Csrf,
    RateLimited,
    Validation,
    Pg(/* concrete type above */),
    PayloadTooLarge,
    Header(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminHtmlUpdateSettingsError {
    Authentication,
    Authorization,
    Conflict,
    Csrf,
    RateLimited,
    Validation,
    Pg(/* concrete type above */),
    PayloadTooLarge,
    Header(/* concrete type above */),
}

#[derive(Debug, thiserror::Error)]
enum AdminHtmlMetricsError {
    #[error("administrator HTML metrics rendering failed: {0}")]
    Render(#[source] server_runtime::MetricsResponseBodyError),
    #[error("administrator HTML metrics text is invalid: {0}")]
    SsrText(
        #[source]
        server_admin_frontend::ssr::AdminSsrTextTryFromStringError,
    ),
}
```

`AdminHtmlMetricsError` is fallible because the current handler calls both
`MetricsResponseBody::try_from` and `AdminSsrText::try_from`; it currently
catches both failures and erases them into `500 Internal Server Error`
responses. No infallible admin HTML route declares an error type.

```rust
#[derive(Debug, thiserror::Error)]
enum AdminAssetsError {
    #[error("administrator asset read failed: {0}")]
    Read(to_err_string::ErrorText),
}
```

### Reusable runtime routes

The readiness route is fallible:

```rust
#[derive(Debug, thiserror::Error)]
enum HealthReadyError {
    #[error("service is unavailable")]
    Unavailable(HealthSnapshot),
}
```

The liveness route is infallible and returns its response directly:

```rust
async || {
    axum::Json(ServiceLivenessSnapshot {
        service: HealthComponentStatus::Ok,
    })
}
```

No `server_runtime::health::HealthLiveError` exists. This is separate from
`common_routes::HealthLiveError`, which remains fallible and has an
`Unavailable` variant.
