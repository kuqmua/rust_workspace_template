#![allow(clippy::needless_for_each)] // utoipa 4 generated OpenAPI registration uses iterator callbacks
mod account;
mod api;
mod authn;
pub(super) mod authorization;
mod cookie_response;
mod data_tables;
mod error_response;
mod extractors;
mod html;
mod persistence;
mod roles;
mod sessions;
mod settings;
mod shared;
mod state;
mod users;
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct JsonwebtokenAdminEncodingKey(jsonwebtoken::EncodingKey);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::AsRefTarget, newtype::FromInner,
)]
struct JsonwebtokenAdminDecodingKeys(Vec<jsonwebtoken::DecodingKey>);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::IntoInnerFrom,
    newtype::TryFrom,
)]
#[try_from(
    error = AdminAuthPositiveValueError,
    validator = StdAdminAccessTtlSeconds::validate
)]
pub struct StdAdminAccessTtlSeconds(u64);
impl StdAdminAccessTtlSeconds {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &u64) -> Result<(), AdminAuthPositiveValueError> {
        if *value == constants_u64::ZERO {
            Err(AdminAuthPositiveValueError)
        } else {
            Ok(())
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::IntoInnerFrom,
    newtype::TryFrom,
)]
#[try_from(
    error = AdminAuthPositiveValueError,
    validator = StdAdminRefreshTtlSeconds::validate
)]
pub struct StdAdminRefreshTtlSeconds(u64);
impl StdAdminRefreshTtlSeconds {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &u64) -> Result<(), AdminAuthPositiveValueError> {
        if *value == constants_u64::ZERO {
            Err(AdminAuthPositiveValueError)
        } else {
            Ok(())
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::IntoInnerFrom,
    newtype::TryFrom,
)]
#[try_from(
    error = AdminAuthPositiveValueError,
    validator = StdAdminSessionLimit::validate
)]
pub struct StdAdminSessionLimit(usize);
impl StdAdminSessionLimit {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &usize) -> Result<(), AdminAuthPositiveValueError> {
        if *value == constants_usize::ZERO {
            Err(AdminAuthPositiveValueError)
        } else {
            Ok(())
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::IntoInnerFrom,
    newtype::TryFrom,
)]
#[try_from(
    error = AdminAuthPositiveValueError,
    validator = StdAdminFailureThreshold::validate
)]
pub struct StdAdminFailureThreshold(i64);
impl StdAdminFailureThreshold {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &i64) -> Result<(), AdminAuthPositiveValueError> {
        if *value <= constants_i64::ZERO {
            Err(AdminAuthPositiveValueError)
        } else {
            Ok(())
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{self:?}")]
pub struct AdminAuthPositiveValueError;
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub struct StdAdminFailureDelayMillis(u64);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct StdAdminRateLimitCount(i64);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct StdAdminRateLimitWindowSeconds(i32);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub struct AdminAuthPolicy {
    audit_export_limit: StdAdminRateLimitCount,
    failure_delay: StdAdminFailureDelayMillis,
    failure_threshold: StdAdminFailureThreshold,
    mutation_limit: StdAdminRateLimitCount,
    refresh_limit: StdAdminRateLimitCount,
    sign_in_ip_limit: StdAdminRateLimitCount,
    sign_in_limit: StdAdminRateLimitCount,
    audit_export_window: StdAdminRateLimitWindowSeconds,
    mutation_window: StdAdminRateLimitWindowSeconds,
    refresh_window: StdAdminRateLimitWindowSeconds,
    sign_in_window: StdAdminRateLimitWindowSeconds,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct AdminAuthSvcState {
    access_ttl: StdAdminAccessTtlSeconds,
    allowed_origins: server_runtime_http::domain_types::AllowedOrigins,
    audience: config_lib::domain_types::AdminTokenAudience,
    decoding_keys: JsonwebtokenAdminDecodingKeys,
    encoding_key: JsonwebtokenAdminEncodingKey,
    issuer: config_lib::domain_types::AdminTokenIssuer,
    password_hasher: super::AdminPasswordHasher,
    policy: AdminAuthPolicy,
    pool: app_state::domain_types::SqlxPgPool,
    refresh_ttl: StdAdminRefreshTtlSeconds,
    session_limit: StdAdminSessionLimit,
    cookie_secure: super::AdminCookieSecure,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefOwned,
    newtype::FromInner,
)]
pub struct SharedAdminAuthSvcStateArc(std::sync::Arc<AdminAuthSvcState>);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, thiserror::Error)]
pub enum AdminAuthSvcStateBuildError {
    #[error("administrator allowed origin is invalid")]
    AllowedOrigin,
    #[error("administrator JWT secret list is empty")]
    JwtSecret,
    #[error("administrator password hash concurrency is zero")]
    PasswordHashConcurrency,
    #[error("administrator authentication numeric value is not positive")]
    PositiveValue(#[source] AdminAuthPositiveValueError),
}
fn admin_password_from_contract(
    value: server_admin_contract::domain_types::AdminPassword,
) -> Result<super::AdminPassword, super::AdminPasswordTryFromStringError> {
    super::AdminPassword::try_from(value.into_inner())
}
fn admin_new_password_from_contract(
    value: server_admin_contract::domain_types::AdminNewPassword,
) -> Result<super::AdminPassword, super::AdminPasswordTryFromStringError> {
    super::AdminPassword::try_from(value.into_inner())
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Serialize, utoipa::ToSchema)]
pub struct AuthenticatedAdmin {
    display_name: super::AdminDisplayName,
    id: super::AdminUserId,
    login: super::AdminLogin,
    permissions: super::AdminPermissions,
    roles: super::AdminRoleNames,
    session_id: super::AdminSessionId,
    #[schema(value_type = bool)]
    password_change_required: super::AdminPasswordChangeRequired,
}
impl AuthenticatedAdmin {
    #[must_use]
    pub const fn id(&self) -> super::AdminUserId {
        self.id
    }
    #[must_use]
    pub(crate) const fn password_change_required(&self) -> super::AdminPasswordChangeRequired {
        self.password_change_required
    }
}
fn authenticated_admin_contract(
    value: &AuthenticatedAdmin,
) -> Result<server_admin_contract::domain_types::AuthenticatedAdmin, AdminError> {
    let permissions = value
        .permissions
        .as_ref()
        .iter()
        .map(|permission| {
            server_admin_contract::domain_types::AdminPermissionValue::try_from(
                permission.as_str().as_ref().to_owned(),
            )
            .map_err(|_error| AdminError::Validation)
        })
        .collect::<Result<Vec<_>, AdminError>>()?;
    let roles = value
        .roles
        .as_ref()
        .iter()
        .map(|role| {
            server_admin_contract::domain_types::AdminRoleName::try_from(role.as_ref().to_owned())
                .map_err(|_error| AdminError::Validation)
        })
        .collect::<Result<Vec<_>, AdminError>>()?;
    Ok(
        server_admin_contract::domain_types::AuthenticatedAdmin::new(
            server_admin_contract::domain_types::AdminDisplayName::try_from(
                value.display_name.as_ref().to_owned(),
            )
            .map_err(|_error| AdminError::Validation)?,
            server_admin_contract::domain_types::AdminUserId::from(value.id.value()),
            server_admin_contract::domain_types::AdminLogin::try_from(
                value.login.as_ref().to_owned(),
            )
            .map_err(|_error| AdminError::Validation)?,
            server_admin_contract::domain_types::AdminPermissionValues::try_from(permissions)
                .map_err(|_error| AdminError::Validation)?,
            server_admin_contract::domain_types::AdminRoleNames::try_from(roles)
                .map_err(|_error| AdminError::Validation)?,
        ),
    )
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, serde::Deserialize, utoipa::IntoParams,
)]
#[into_params(parameter_in = Query)]
pub struct AdminAuditQuery {
    created_after: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
    created_before: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
    cursor_created_at: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
    cursor_id: Option<server_admin_contract::domain_types::AdminAuditLogId>,
    resource_id: Option<server_admin_contract::domain_types::AdminText>,
    #[param(inline)]
    user_id: Option<super::AdminUserId>,
    user_login: Option<server_admin_contract::domain_types::AdminLogin>,
    #[serde(default)]
    #[param(value_type = u32)]
    offset: server_admin_contract::domain_types::AdminPageOffset,
    #[serde(default)]
    #[param(value_type = u16, minimum = 1, maximum = 100)]
    limit: server_admin_contract::domain_types::AdminPageLimit,
    #[param(inline)]
    resource: Option<super::AdminAuditResource>,
    succeeded: Option<server_admin_contract::domain_types::AdminBool>,
    #[param(inline)]
    action: Option<super::AdminAuditAction>,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(clippy::field_scoped_visibility_modifiers)] // repository query binding consumes this internal cross-module DTO field-by-field
pub(crate) struct AdminAuditQueryParts {
    pub(crate) created_after: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
    pub(crate) created_before: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
    pub(crate) cursor_created_at: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
    pub(crate) cursor_id: Option<server_admin_contract::domain_types::AdminAuditLogId>,
    pub(crate) resource_id: Option<server_admin_contract::domain_types::AdminText>,
    pub(crate) user_id: Option<super::AdminUserId>,
    pub(crate) user_login: Option<server_admin_contract::domain_types::AdminLogin>,
    pub(crate) offset: server_admin_contract::domain_types::AdminPageOffset,
    pub(crate) limit: server_admin_contract::domain_types::AdminPageLimit,
    pub(crate) resource: Option<super::AdminAuditResource>,
    pub(crate) succeeded: Option<server_admin_contract::domain_types::AdminBool>,
    pub(crate) action: Option<super::AdminAuditAction>,
}
impl AdminAuditQuery {
    pub(crate) fn cursor_is_complete(&self) -> super::StdAdminBool {
        super::StdAdminBool::from(self.cursor_created_at.is_some() == self.cursor_id.is_some())
    }
    pub(crate) fn into_parts(self) -> AdminAuditQueryParts {
        AdminAuditQueryParts {
            action: self.action,
            created_after: self.created_after,
            created_before: self.created_before,
            cursor_created_at: self.cursor_created_at,
            cursor_id: self.cursor_id,
            limit: self.limit,
            offset: self.offset,
            resource: self.resource,
            resource_id: self.resource_id,
            succeeded: self.succeeded,
            user_id: self.user_id,
            user_login: self.user_login,
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefOwned,
    newtype::FromInner,
)]
pub struct HttpAdminHeaderMap(http::HeaderMap);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone)]
pub(crate) struct AdminAuthReq {
    headers: HttpAdminHeaderMap,
    state: SharedAdminAuthSvcStateArc,
    peer: AdminPeerAddr,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct AdminPeerAddr(super::AdminSocketAddr);
impl AdminPeerAddr {
    pub(crate) const fn socket_addr(self) -> super::AdminSocketAddr {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AdminSignInJson(server_admin_contract::domain_types::AdminSignInReq);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumAdminJson<Value>(Value);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumAdminForm<Value>(Value);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumAdminPath<Value>(Value);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumAdminQuery<Value>(Value);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct AdminSessionPath(super::AdminSessionId);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DebugTransparent,
    thiserror::Error,
    newtype::FromInner,
)]
#[error(transparent)]
pub struct HttpAdminHeaderValueError(http::header::InvalidHeaderValue);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminError {
    #[error("administrator authentication failed")]
    Authentication,
    #[error("administrator authentication secret text is invalid")]
    AuthenticationSecretText(
        #[source] server_runtime_http::domain_types::ObservedError<super::AdminSecretTextError>,
    ),
    #[error("administrator authorization failed")]
    Authorization,
    #[error("administrator operation conflicts with current state")]
    Conflict,
    #[error("administrator request failed CSRF validation")]
    Csrf,
    #[error("administrator CSRF secret text is invalid")]
    CsrfSecretText(
        #[source] server_runtime_http::domain_types::ObservedError<super::AdminSecretTextError>,
    ),
    #[error("administrator authentication is temporarily rate limited")]
    RateLimited,
    #[error("administrator request validation failed")]
    Validation,
    #[error("administrator API database operation failed: {0:?}")]
    Pg(#[source] server_runtime_http::domain_types::ObservedError<super::SqlxAdminError>),
    #[error("administrator password hashing failed: {0}")]
    PasswordHash(
        #[source] server_runtime_http::domain_types::ObservedError<super::AdminPasswordHashError>,
    ),
    #[error("administrator password text is invalid")]
    PasswordText(
        #[source]
        server_runtime_http::domain_types::ObservedError<super::AdminPasswordTryFromStringError>,
    ),
    #[error("administrator request body is too large")]
    PayloadTooLarge,
    #[error("administrator secret text is invalid")]
    SecretText(
        #[source] server_runtime_http::domain_types::ObservedError<super::AdminSecretTextError>,
    ),
    #[error("administrator route does not support this HTTP method")]
    MethodNotAllowed,
    #[error("administrator session operation failed: {0}")]
    Session(#[source] server_runtime_http::domain_types::ObservedError<AdminSessionError>),
    #[error("administrator response header is invalid: {0:?}")]
    Header(#[source] server_runtime_http::domain_types::ObservedError<HttpAdminHeaderValueError>),
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::IntoInnerFrom, newtype::FromInner,
)]
pub struct AxumAdminResponse(axum::response::Response);
fn admin_error_response_parts(
    route_error_status: frontend_contract::domain_types::RouteErrorStatus,
    optional_diagnostic: Option<server_runtime_http::domain_types::HttpErrorDiagnostic>,
) -> axum::response::Response {
    let problem_status = frontend_contract::domain_types::ApiProblemStatus::try_from(u16::from(
        route_error_status.transport_status(),
    ))
    .unwrap_or_else(|_error| {
        frontend_contract::domain_types::ApiProblemStatus::from(
            frontend_contract::domain_types::KnownHttpStatus::InternalServerError,
        )
    });
    let mut response = axum::response::IntoResponse::into_response(
        frontend_contract::domain_types::ApiProblemError::from_status(problem_status),
    );
    if let Some(diagnostic) = optional_diagnostic {
        let _previous_diagnostic = response.extensions_mut().insert(diagnostic);
    }
    response
}
frontend_contract::domain_types::api_operation_error!(AdminAuditLogError,);
frontend_contract::domain_types::api_operation_error!(AdminAuditExportError,);
frontend_contract::domain_types::api_operation_error!(AdminBrandingError,);
frontend_contract::domain_types::api_operation_error!(AdminChangeOwnPasswordError,);
frontend_contract::domain_types::api_operation_error!(AdminCreateRoleError,);
frontend_contract::domain_types::api_operation_error!(AdminCreateUserError,);
frontend_contract::domain_types::api_operation_error!(AdminDataTableError,);
frontend_contract::domain_types::api_operation_error!(AdminDataTablesError,);
frontend_contract::domain_types::api_operation_error!(AdminDeleteRoleError,);
frontend_contract::domain_types::api_operation_error!(AdminDeleteUserError,);
frontend_contract::domain_types::api_operation_error!(AdminListPermissionsError,);
frontend_contract::domain_types::api_operation_error!(AdminListRolesError,);
frontend_contract::domain_types::api_operation_error!(AdminListUsersError,);
frontend_contract::domain_types::api_operation_error!(AdminMeError,);
frontend_contract::domain_types::api_operation_error!(AdminRefreshError,);
frontend_contract::domain_types::api_operation_error!(AdminRevokeAllSessionsError,);
frontend_contract::domain_types::api_operation_error!(AdminRevokeSessionError,);
frontend_contract::domain_types::api_operation_error!(AdminSessionsError,);
frontend_contract::domain_types::api_operation_error!(AdminSetRolePermissionsError,);
frontend_contract::domain_types::api_operation_error!(AdminSetUserBanError,);
frontend_contract::domain_types::api_operation_error!(AdminSetUserPasswordError,);
frontend_contract::domain_types::api_operation_error!(AdminSetUserRolesError,);
frontend_contract::domain_types::api_operation_error!(AdminSettingsError,);
frontend_contract::domain_types::api_operation_error!(AdminSignInError,);
frontend_contract::domain_types::api_operation_error!(AdminSignOutError,);
frontend_contract::domain_types::api_operation_error!(AdminUpdateRoleError,);
frontend_contract::domain_types::api_operation_error!(AdminUpdateSettingsError,);
frontend_contract::domain_types::api_operation_error!(AdminUpdateUserError,);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct AxumAdminAuthRouter(axum::Router);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(crate) struct AxumAdminStateRouter(axum::Router<SharedAdminAuthSvcStateArc>);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::IntoInnerFrom, newtype::FromInner,
)]
pub struct UtoipaAdminAuthOpenApi(utoipa::openapi::OpenApi);
impl std::fmt::Debug for UtoipaAdminAuthOpenApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::UTOIPAADMINAUTHOPENAPI)
    }
}
#[must_use]
pub fn open_api() -> UtoipaAdminAuthOpenApi {
    routes::open_api()
}
#[must_use]
pub fn routes(state: SharedAdminAuthSvcStateArc) -> AxumAdminAuthRouter {
    routes::routes(state)
}
#[must_use]
pub fn html_routes(state: SharedAdminAuthSvcStateArc) -> AxumAdminAuthRouter {
    html::routes(state, AdminHtmlSwaggerEnabled::from(true))
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct AdminHtmlSwaggerEnabled(bool);
#[must_use]
pub fn html_routes_with_swagger(
    state: SharedAdminAuthSvcStateArc,
    swagger_enabled: AdminHtmlSwaggerEnabled,
) -> AxumAdminAuthRouter {
    html::routes(state, swagger_enabled)
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct AdminSessionBundle {
    access_token: super::StdAdminAccessToken,
    csrf_token: super::AdminOpaqueToken,
    refresh_token: super::AdminRefreshToken,
    session_id: super::AdminSessionId,
}
impl AdminSessionBundle {
    #[must_use]
    pub const fn access_token(&self) -> &super::StdAdminAccessToken {
        &self.access_token
    }
    #[must_use]
    pub const fn csrf_token(&self) -> &super::AdminOpaqueToken {
        &self.csrf_token
    }
    #[must_use]
    pub const fn refresh_token(&self) -> &super::AdminRefreshToken {
        &self.refresh_token
    }
    #[must_use]
    pub const fn session_id(&self) -> super::AdminSessionId {
        self.session_id
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminSessionError {
    #[error("administrator access token creation failed: {0:?}")]
    AccessToken(super::AdminAccessTokenError),
    #[error("administrator session database operation failed: {0:?}")]
    Pg(super::SqlxAdminError),
    #[error("administrator session secret text is invalid: {0}")]
    SecretText(super::AdminSecretTextError),
    #[error("system clock is before the Unix epoch")]
    SystemClock,
}
mod audit;
mod rate_limit;
mod routes;
mod session;
#[cfg(test)]
mod tests;
