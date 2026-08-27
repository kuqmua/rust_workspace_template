// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::single_call_fn)] // public facade keeps stable auth module paths while this module owns router and OpenAPI composition
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::route_registry(
    state = super::super::SharedAdminAuthSvcStateArc,
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
    (server_admin_contract::domain_types::AdminSignInRoute, super::super::api_sign_in::api_sign_in),
    (server_admin_contract::domain_types::AdminRefreshRoute, super::super::api_refresh::api_refresh),
    (server_admin_contract::domain_types::AdminSignOutRoute, super::super::api_sign_out::api_sign_out),
    (server_admin_contract::domain_types::AdminMeRoute, super::super::api_me::api_me),
    (server_admin_contract::domain_types::AdminChangeOwnPasswordRoute, super::super::api_change_own_password::api_change_own_password),
    (server_admin_contract::domain_types::AdminSessionsRoute, super::super::api_sessions::api_sessions),
    (server_admin_contract::domain_types::AdminRevokeSessionRoute, super::super::api_revoke_session::api_revoke_session),
    (server_admin_contract::domain_types::AdminRevokeAllSessionsRoute, super::super::api_revoke_all_sessions::api_revoke_all_sessions),
    (server_admin_contract::domain_types::AdminListUsersRoute, super::super::api_list_users::api_list_users),
    (server_admin_contract::domain_types::AdminCreateUserRoute, super::super::api_create_user::api_create_user),
    (server_admin_contract::domain_types::AdminUpdateUserRoute, super::super::api_update_user::api_update_user),
    (server_admin_contract::domain_types::AdminDeleteUserRoute, super::super::api_delete_user::api_delete_user),
    (server_admin_contract::domain_types::AdminSetUserPasswordRoute, super::super::api_set_user_password::api_set_user_password),
    (server_admin_contract::domain_types::AdminSetUserBanRoute, super::super::api_set_user_ban::api_set_user_ban),
    (server_admin_contract::domain_types::AdminSetUserRolesRoute, super::super::api_set_user_roles::api_set_user_roles),
    (server_admin_contract::domain_types::AdminListRolesRoute, super::super::api_list_roles::api_list_roles),
    (server_admin_contract::domain_types::AdminCreateRoleRoute, super::super::api_create_role::api_create_role),
    (server_admin_contract::domain_types::AdminUpdateRoleRoute, super::super::api_update_role::api_update_role),
    (server_admin_contract::domain_types::AdminDeleteRoleRoute, super::super::api_delete_role::api_delete_role),
    (server_admin_contract::domain_types::AdminSetRolePermissionsRoute, super::super::api_set_role_permissions::api_set_role_permissions),
    (server_admin_contract::domain_types::AdminListPermissionsRoute, super::super::api_list_permissions::api_list_permissions),
    (server_admin_contract::domain_types::AdminAuditLogRoute, super::super::api_audit_log::api_audit_log),
    (server_admin_contract::domain_types::AdminAuditExportRoute, super::super::api_export_audit_log::api_export_audit_log),
    (server_admin_contract::domain_types::AdminBrandingRoute, super::super::api_branding::api_branding),
    (server_admin_contract::domain_types::AdminSettingsRoute, super::super::api_settings::api_settings),
    (server_admin_contract::domain_types::AdminUpdateSettingsRoute, super::super::api_update_settings::api_update_settings),
    (server_admin_contract::domain_types::AdminDataTablesRoute, super::super::api_data_tables::api_data_tables),
    (server_admin_contract::domain_types::AdminDataTableRoute, super::super::api_data_table::api_data_table),
)]
#[openapi(
    tags((name = "admin_auth", description = "Administrator authentication and sessions"), (name = "admin_users", description = "Administrator user security operations"), (name = "admin_roles", description = "Administrator role security operations"), (name = "admin_audit", description = "Administrator audit log"), (name = "admin_settings", description = "Administrator system settings"), (name = "admin_tables", description = "Read-only administrator database views"))
)]
pub(super) struct AdminAuthRouteRegistry;

impl AdminAuthRouteRegistry {
    pub(super) fn registry_open_api() -> utoipa::openapi::OpenApi {
        Self::open_api()
    }

    pub(super) fn registry_router() -> axum::Router<super::super::SharedAdminAuthSvcStateArc> {
        Self::router()
    }
}
