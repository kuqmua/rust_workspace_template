#![allow(clippy::single_call_fn)] // public facade keeps stable auth module paths while this module owns router and OpenAPI composition
#[frontend_contract::route_registry(
    state = super::StdSharedAdminAuthSvcState,
    family = server_admin_contract::AdminAuthenticationRouteFamily;
    (str_constants::ADMIN_COOKIE, str_constants::ADMIN_CSRF);
    schemas(
        server_admin_contract::StdAdminPositiveI64,
        server_admin_contract::AdminPermissionValues,
        server_admin_contract::AdminRoleNames,
        server_admin_contract::AdminRoleIds,
        server_admin_contract::AdminPermissionIds,
        server_admin_contract::AdminUserSummaries,
        server_admin_contract::AdminRoleSummaries,
        server_admin_contract::AdminPermissionSummaries,
        server_admin_contract::AdminAuditViews,
        server_admin_contract::AdminTexts,
        server_admin_contract::AdminDataColumn,
        server_admin_contract::AdminDataColumns,
        server_admin_contract::AdminDataFilter,
        server_admin_contract::AdminDataFilters,
        server_admin_contract::AdminDataInputKind,
        server_admin_contract::AdminDataRows,
        server_admin_contract::AdminDataTables,
        server_admin_contract::AdminOptionalSettings,
        server_admin_contract::AdminSessionViews,
        server_admin_contract::AdminSessionView,
        server_admin_contract::AdminSessionTimestamp,
        server_admin_contract::AdminSessionIdentifier,
        server_admin_contract::AdminText,
        server_admin_contract::AdminBool,
        server_admin_contract::AdminPermissionValue,
        server_admin_contract::AdminNewPassword,
        server_admin_contract::AdminAuditView,
        server_admin_contract::AdminAuditTimestamp,
        server_admin_contract::AdminAuditLogId,
        server_admin_contract::SerdeJsonAdminAuditDetails,
        server_admin_contract::AdminOptionalSetting,
        server_admin_contract::AdminDefaultRoute,
        server_admin_contract::AdminMainLogo,
        server_admin_contract::AdminOrganizationContacts,
        server_admin_contract::AdminOrganizationName,
        server_admin_contract::AdminPrimaryColor,
        server_admin_contract::AdminSiteName,
        server_admin_contract::AdminSupportUrl,
        server_admin_contract::AdminTabTitle,
        server_admin_contract::AdminUserSummary,
        server_admin_contract::AdminRoleSummary,
        server_admin_contract::AdminPermissionSummary,
        server_admin_contract::AdminDataTable,
        server_admin_contract::AdminDataRow,
        crate::UuidAdminValue,
        crate::AdminPassword,
        crate::AdminLogin,
        crate::AdminDisplayName,
        crate::AdminRoleName,
        crate::AdminUserId,
        crate::AdminRoleId,
        crate::AdminPermissionId,
        crate::AdminPermission,
        crate::AdminSessionId,
        crate::AdminAuditLogId,
        crate::AdminAuditAction,
        crate::AdminAuditResource
    );
    (server_admin_contract::AdminSignInRoute, super::sign_in),
    (server_admin_contract::AdminRefreshRoute, super::refresh),
    (server_admin_contract::AdminSignOutRoute, super::sign_out),
    (server_admin_contract::AdminMeRoute, super::me),
    (server_admin_contract::AdminChangeOwnPasswordRoute, super::change_own_password),
    (server_admin_contract::AdminSessionsRoute, super::sessions),
    (server_admin_contract::AdminRevokeSessionRoute, super::revoke_session),
    (server_admin_contract::AdminRevokeAllSessionsRoute, super::revoke_all_sessions),
    (server_admin_contract::AdminListUsersRoute, super::list_users),
    (server_admin_contract::AdminCreateUserRoute, super::create_user),
    (server_admin_contract::AdminUpdateUserRoute, super::update_user),
    (server_admin_contract::AdminDeleteUserRoute, super::delete_user),
    (server_admin_contract::AdminSetUserPasswordRoute, super::set_user_password),
    (server_admin_contract::AdminSetUserBanRoute, super::set_user_ban),
    (server_admin_contract::AdminSetUserRolesRoute, super::set_user_roles),
    (server_admin_contract::AdminListRolesRoute, super::list_roles),
    (server_admin_contract::AdminCreateRoleRoute, super::create_role),
    (server_admin_contract::AdminUpdateRoleRoute, super::update_role),
    (server_admin_contract::AdminDeleteRoleRoute, super::delete_role),
    (server_admin_contract::AdminSetRolePermissionsRoute, super::set_role_permissions),
    (server_admin_contract::AdminListPermissionsRoute, super::list_permissions),
    (server_admin_contract::AdminAuditLogRoute, super::audit_log),
    (server_admin_contract::AdminAuditExportRoute, super::export_audit_log),
    (server_admin_contract::AdminBrandingRoute, super::branding),
    (server_admin_contract::AdminSettingsRoute, super::settings),
    (server_admin_contract::AdminUpdateSettingsRoute, super::update_settings),
    (server_admin_contract::AdminDataTablesRoute, super::data_tables),
    (server_admin_contract::AdminDataTableRoute, super::data_table),
)]
#[openapi(
    tags((name = "admin_auth", description = "Administrator authentication and sessions"), (name = "admin_users", description = "Administrator user security operations"), (name = "admin_roles", description = "Administrator role security operations"), (name = "admin_audit", description = "Administrator audit log"), (name = "admin_settings", description = "Administrator system settings"), (name = "admin_tables", description = "Read-only administrator database views"))
)]
struct AdminAuthRouteRegistry;
pub(super) fn open_api() -> super::UtoipaAdminAuthOpenApi {
    let mut document = AdminAuthRouteRegistry::open_api();
    let body_limit_description =
        <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::body_limit()
            .map(|limit| {
                format!(
                    "{}{}",
                    str_constants::OPENAPI_REQUEST_BODY_MAXIMUM_BYTES_PREFIX,
                    limit.get()
                )
            });
    document
        .paths
        .paths
        .values_mut()
        .flat_map(|path| path.operations.values_mut())
        .for_each(|operation| {
            if let (Some(request_body), Some(description)) = (
                operation.request_body.as_mut(),
                body_limit_description.as_ref(),
            ) {
                request_body.description = Some(description.clone());
            }
        });
    if let Some(components) = document.components.as_mut() {
        components.add_security_scheme(
            str_constants::ADMIN_COOKIE,
            utoipa::openapi::security::SecurityScheme::ApiKey(
                utoipa::openapi::security::ApiKey::Cookie(
                    utoipa::openapi::security::ApiKeyValue::with_description(
                        str_constants::SERVER_ADMIN_ACCESS_COOKIE_NAME,
                        str_constants::HTTPONLY_ADMINISTRATOR_ACCESS_TOKEN_COOKIE,
                    ),
                ),
            ),
        );
        components.add_security_scheme(
            str_constants::ADMIN_CSRF,
            utoipa::openapi::security::SecurityScheme::ApiKey(
                utoipa::openapi::security::ApiKey::Header(
                    utoipa::openapi::security::ApiKeyValue::with_description(
                        str_constants::X_CSRF_TOKEN,
                        str_constants::CSRF_TOKEN_BOUND_TO_THE_ADMINISTRATOR_ACCESS_SESSION,
                    ),
                ),
            ),
        );
    }
    super::UtoipaAdminAuthOpenApi(document)
}
pub(super) fn routes(state: super::StdSharedAdminAuthSvcState) -> super::AxumAdminAuthRouter {
    let base_router = AdminAuthRouteRegistry::router()
        .method_not_allowed_fallback(async || super::AdminError::MethodNotAllowed);
    let router = match <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::body_limit() {
        Some(limit) => base_router.layer(axum::extract::DefaultBodyLimit::max(limit.get())),
        None => base_router,
    };
    super::AxumAdminAuthRouter(router.with_state(state))
}
