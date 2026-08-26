#![allow(clippy::single_call_fn)] // public facade keeps stable auth module paths while this module owns router and OpenAPI composition
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::route_registry(
    state = super::SharedAdminAuthSvcStateArc,
    family = server_admin_contract::domain_types::AdminAuthenticationRouteFamily;
    (constants_str::ADMIN_COOKIE, constants_str::ADMIN_CSRF);
    schemas(
        server_admin_contract::domain_types::PositiveNonZeroI64,
        server_admin_contract::domain_types::AdminPermissionValues,
        server_admin_contract::domain_types::AdminRoleNames,
        server_admin_contract::domain_types::AdminRoleIds,
        server_admin_contract::domain_types::AdminPermissionIds,
        server_admin_contract::domain_types::AdminUserSummaries,
        server_admin_contract::domain_types::AdminRoleSummaries,
        server_admin_contract::domain_types::AdminPermissionSummaries,
        server_admin_contract::domain_types::AdminAuditViews,
        server_admin_contract::domain_types::AdminTexts,
        server_admin_contract::domain_types::AdminDataColumn,
        server_admin_contract::domain_types::AdminDataColumns,
        server_admin_contract::domain_types::AdminDataFilter,
        server_admin_contract::domain_types::AdminDataFilters,
        server_admin_contract::domain_types::AdminDataInputKind,
        server_admin_contract::domain_types::AdminDataRows,
        server_admin_contract::domain_types::AdminDataTables,
        server_admin_contract::domain_types::AdminOptionalSettings,
        server_admin_contract::domain_types::AdminSessionViews,
        server_admin_contract::domain_types::AdminSessionView,
        server_admin_contract::domain_types::AdminSessionTimestamp,
        server_admin_contract::domain_types::AdminSessionIdentifier,
        server_admin_contract::domain_types::AdminText,
        server_admin_contract::domain_types::AdminBool,
        server_admin_contract::domain_types::AdminPermissionValue,
        server_admin_contract::domain_types::AdminNewPassword,
        server_admin_contract::domain_types::AdminAuditView,
        server_admin_contract::domain_types::AdminAuditTimestamp,
        server_admin_contract::domain_types::AdminAuditLogId,
        server_admin_contract::domain_types::SerdeJsonAdminAuditDetails,
        server_admin_contract::domain_types::AdminOptionalSetting,
        server_admin_contract::domain_types::AdminDefaultRoute,
        server_admin_contract::domain_types::AdminMainLogo,
        server_admin_contract::domain_types::AdminOrganizationContacts,
        server_admin_contract::domain_types::AdminOrganizationName,
        server_admin_contract::domain_types::AdminPrimaryColor,
        server_admin_contract::domain_types::AdminSiteName,
        server_admin_contract::domain_types::AdminSupportUrl,
        server_admin_contract::domain_types::AdminTabTitle,
        server_admin_contract::domain_types::AdminUserSummary,
        server_admin_contract::domain_types::AdminRoleSummary,
        server_admin_contract::domain_types::AdminPermissionSummary,
        server_admin_contract::domain_types::AdminDataTable,
        server_admin_contract::domain_types::AdminDataRow,
        crate::domain_types::UuidAdminValue,
        crate::domain_types::AdminPassword,
        crate::domain_types::AdminLogin,
        crate::domain_types::AdminDisplayName,
        crate::domain_types::AdminRoleName,
        crate::domain_types::AdminUserId,
        crate::domain_types::AdminRoleId,
        crate::domain_types::AdminPermissionId,
        crate::domain_types::AdminPermission,
        crate::domain_types::AdminSessionId,
        crate::domain_types::AdminAuditLogId,
        crate::domain_types::AdminAuditAction,
        crate::domain_types::AdminAuditResource
    );
    (server_admin_contract::domain_types::AdminSignInRoute, super::api_sign_in::sign_in),
    (server_admin_contract::domain_types::AdminRefreshRoute, super::api_refresh::refresh),
    (server_admin_contract::domain_types::AdminSignOutRoute, super::api_sign_out::sign_out),
    (server_admin_contract::domain_types::AdminMeRoute, super::api_me::me),
    (server_admin_contract::domain_types::AdminChangeOwnPasswordRoute, super::api_change_own_password::change_own_password),
    (server_admin_contract::domain_types::AdminSessionsRoute, super::api_sessions::sessions),
    (server_admin_contract::domain_types::AdminRevokeSessionRoute, super::api_revoke_session::revoke_session),
    (server_admin_contract::domain_types::AdminRevokeAllSessionsRoute, super::api_revoke_all_sessions::revoke_all_sessions),
    (server_admin_contract::domain_types::AdminListUsersRoute, super::api_list_users::list_users),
    (server_admin_contract::domain_types::AdminCreateUserRoute, super::api_create_user::create_user),
    (server_admin_contract::domain_types::AdminUpdateUserRoute, super::api_update_user::update_user),
    (server_admin_contract::domain_types::AdminDeleteUserRoute, super::api_delete_user::delete_user),
    (server_admin_contract::domain_types::AdminSetUserPasswordRoute, super::api_set_user_password::set_user_password),
    (server_admin_contract::domain_types::AdminSetUserBanRoute, super::api_set_user_ban::set_user_ban),
    (server_admin_contract::domain_types::AdminSetUserRolesRoute, super::api_set_user_roles::set_user_roles),
    (server_admin_contract::domain_types::AdminListRolesRoute, super::api_list_roles::list_roles),
    (server_admin_contract::domain_types::AdminCreateRoleRoute, super::api_create_role::create_role),
    (server_admin_contract::domain_types::AdminUpdateRoleRoute, super::api_update_role::update_role),
    (server_admin_contract::domain_types::AdminDeleteRoleRoute, super::api_delete_role::delete_role),
    (server_admin_contract::domain_types::AdminSetRolePermissionsRoute, super::api_set_role_permissions::set_role_permissions),
    (server_admin_contract::domain_types::AdminListPermissionsRoute, super::api_list_permissions::list_permissions),
    (server_admin_contract::domain_types::AdminAuditLogRoute, super::api_audit_log::audit_log),
    (server_admin_contract::domain_types::AdminAuditExportRoute, super::api_export_audit_log::export_audit_log),
    (server_admin_contract::domain_types::AdminBrandingRoute, super::api_branding::branding),
    (server_admin_contract::domain_types::AdminSettingsRoute, super::api_settings::settings),
    (server_admin_contract::domain_types::AdminUpdateSettingsRoute, super::api_update_settings::update_settings),
    (server_admin_contract::domain_types::AdminDataTablesRoute, super::api_data_tables::data_tables),
    (server_admin_contract::domain_types::AdminDataTableRoute, super::api_data_table::data_table),
)]
#[openapi(
    tags((name = "admin_auth", description = "Administrator authentication and sessions"), (name = "admin_users", description = "Administrator user security operations"), (name = "admin_roles", description = "Administrator role security operations"), (name = "admin_audit", description = "Administrator audit log"), (name = "admin_settings", description = "Administrator system settings"), (name = "admin_tables", description = "Read-only administrator database views"))
)]
struct AdminAuthRouteRegistry;
pub(super) fn open_api() -> super::UtoipaAdminAuthOpenApi {
    let mut document = AdminAuthRouteRegistry::open_api();
    let body_limit_description =
        <server_admin_contract::domain_types::AdminAuthenticationRouteFamily as frontend_contract::domain_types::RouteFamily>::body_limit()
            .map(|limit| {
                format!(
                    "{}{}",
                    constants_str::OPENAPI_REQUEST_BODY_MAXIMUM_BYTES_PREFIX,
                    limit.get()
                )
            });
    document
        .paths
        .paths
        .values_mut()
        .flat_map(|path| {
            [
                path.get.as_mut(),
                path.put.as_mut(),
                path.post.as_mut(),
                path.delete.as_mut(),
                path.options.as_mut(),
                path.head.as_mut(),
                path.patch.as_mut(),
                path.trace.as_mut(),
            ]
            .into_iter()
            .flatten()
        })
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
            constants_str::ADMIN_COOKIE,
            utoipa::openapi::security::SecurityScheme::ApiKey(
                utoipa::openapi::security::ApiKey::Cookie(
                    utoipa::openapi::security::ApiKeyValue::with_description(
                        constants_str::SERVER_ADMIN_ACCESS_COOKIE_NAME,
                        constants_str::HTTPONLY_ADMINISTRATOR_ACCESS_TOKEN_COOKIE,
                    ),
                ),
            ),
        );
        components.add_security_scheme(
            constants_str::ADMIN_CSRF,
            utoipa::openapi::security::SecurityScheme::ApiKey(
                utoipa::openapi::security::ApiKey::Header(
                    utoipa::openapi::security::ApiKeyValue::with_description(
                        constants_str::X_CSRF_TOKEN,
                        constants_str::CSRF_TOKEN_BOUND_TO_THE_ADMINISTRATOR_ACCESS_SESSION,
                    ),
                ),
            ),
        );
    }
    super::UtoipaAdminAuthOpenApi(document)
}
pub(super) fn routes(state: super::SharedAdminAuthSvcStateArc) -> super::AxumAdminAuthRouter {
    let base_router = AdminAuthRouteRegistry::router()
        .method_not_allowed_fallback(async || super::AdminError::MethodNotAllowed);
    let router = match <server_admin_contract::domain_types::AdminAuthenticationRouteFamily as frontend_contract::domain_types::RouteFamily>::body_limit() {
        Some(limit) => base_router.layer(axum::extract::DefaultBodyLimit::max(limit.get())),
        None => base_router,
    };
    super::AxumAdminAuthRouter(router.with_state(state))
}
