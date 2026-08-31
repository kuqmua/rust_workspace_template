#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    frontend_contract_macros::RouteCatalog,
)]
#[route_catalog(
    family = AdminAuthenticationRouteFamily,
    body_limit = crate::admin_api_body_max_bytes::ADMIN_API_BODY_MAX_BYTES_VALUE,
)]
pub enum AdminRoute {
    #[route_catalog_route(crate::admin_audit_log_route::AdminAuditLogRoute)]
    Audit,
    #[route_catalog_route(crate::admin_audit_export_route::AdminAuditExportRoute)]
    AuditExport,
    #[route_catalog_route(crate::admin_branding_route::AdminBrandingRoute)]
    Branding,
    #[route_catalog_route(crate::admin_data_table_route::AdminDataTableRoute)]
    DataTable(crate::admin_data_table::AdminDataTable),
    #[route_catalog_route(crate::admin_data_tables_route::AdminDataTablesRoute)]
    DataTables,
    #[route_catalog_route(crate::admin_change_own_password_route::AdminChangeOwnPasswordRoute)]
    ChangeOwnPassword,
    #[route_catalog_route(crate::admin_create_role_route::AdminCreateRoleRoute)]
    CreateRole,
    #[route_catalog_route(crate::admin_create_user_route::AdminCreateUserRoute)]
    CreateUser,
    #[route_catalog_route(crate::admin_delete_role_route::AdminDeleteRoleRoute)]
    DeleteRole(crate::admin_role_id::AdminRoleId),
    #[route_catalog_route(crate::admin_delete_user_route::AdminDeleteUserRoute)]
    DeleteUser(crate::admin_user_id::AdminUserId),
    #[route_catalog_route(crate::admin_me_route::AdminMeRoute)]
    Me,
    #[route_catalog_route(
        contract = frontend_contract::route_contract::RouteContract::new(
            crate::admin_permission_requirement::admin_permission_requirement(crate::admin_permission::AdminPermission::MetricsRead),
            frontend_contract::route_method::RouteMethod::Get,
            frontend_contract::mutation_kind::MutationKind::ReadOnly,
            frontend_contract::contract_str::ContractStr::from(constants_str::METRICS),
            frontend_contract::success_status::SuccessStatus::Code200,
        ),
        path = constants_str::METRICS,
        exclude_from_family,
    )]
    Metrics,
    #[route_catalog_route(
        contract = frontend_contract::route_contract::RouteContract::new(
            crate::admin_permission_requirement::admin_permission_requirement(crate::admin_permission::AdminPermission::OpenApiRead),
            frontend_contract::route_method::RouteMethod::Get,
            frontend_contract::mutation_kind::MutationKind::ReadOnly,
            frontend_contract::contract_str::ContractStr::from(constants_str::OPENAPI_JSON),
            frontend_contract::success_status::SuccessStatus::Code200,
        ),
        path = constants_str::OPENAPI_JSON,
        exclude_from_family,
    )]
    OpenApi,
    #[route_catalog_route(crate::admin_list_permissions_route::AdminListPermissionsRoute)]
    Permissions,
    #[route_catalog_route(crate::admin_refresh_route::AdminRefreshRoute)]
    Refresh,
    #[route_catalog_route(crate::admin_revoke_all_sessions_route::AdminRevokeAllSessionsRoute)]
    RevokeAllSessions,
    #[route_catalog_route(crate::admin_revoke_session_route::AdminRevokeSessionRoute)]
    RevokeSession,
    #[route_catalog_route(crate::admin_list_roles_route::AdminListRolesRoute)]
    Roles,
    #[route_catalog_route(crate::admin_set_role_permissions_route::AdminSetRolePermissionsRoute)]
    SetRolePermissions(crate::admin_role_id::AdminRoleId),
    #[route_catalog_route(crate::admin_set_user_ban_route::AdminSetUserBanRoute)]
    SetUserBan(crate::admin_user_id::AdminUserId),
    #[route_catalog_route(crate::admin_set_user_password_route::AdminSetUserPasswordRoute)]
    SetUserPassword(crate::admin_user_id::AdminUserId),
    #[route_catalog_route(crate::admin_set_user_roles_route::AdminSetUserRolesRoute)]
    SetUserRoles(crate::admin_user_id::AdminUserId),
    #[route_catalog_route(crate::admin_settings_route::AdminSettingsRoute)]
    Settings,
    #[route_catalog_route(crate::admin_sign_in_route::AdminSignInRoute)]
    SignIn,
    #[route_catalog_route(crate::admin_sign_out_route::AdminSignOutRoute)]
    SignOut,
    #[route_catalog_route(crate::admin_sessions_route::AdminSessionsRoute)]
    Sessions,
    #[route_catalog_route(crate::admin_update_role_route::AdminUpdateRoleRoute)]
    UpdateRole(crate::admin_role_id::AdminRoleId),
    #[route_catalog_route(crate::admin_update_settings_route::AdminUpdateSettingsRoute)]
    UpdateSettings,
    #[route_catalog_route(crate::admin_update_user_route::AdminUpdateUserRoute)]
    UpdateUser(crate::admin_user_id::AdminUserId),
    #[route_catalog_route(crate::admin_list_users_route::AdminListUsersRoute)]
    Users,
    #[route_catalog_route(
        contract = frontend_contract::route_contract::RouteContract::new(
            frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
            frontend_contract::route_method::RouteMethod::Get,
            frontend_contract::mutation_kind::MutationKind::ReadOnly,
            frontend_contract::contract_str::ContractStr::from(constants_str::COMMON_ROUTES_GIT_INFO),
            frontend_contract::success_status::SuccessStatus::Code200,
        ),
        path = constants_str::COMMON_ROUTES_GIT_INFO,
        exclude_from_family,
    )]
    Version,
}
impl AdminRoute {
    #[must_use]
    pub fn path(self) -> crate::admin_route_path::AdminRoutePath {
        let suffix = self.catalog_path();
        if matches!(self, Self::Version) {
            crate::admin_route_path::AdminRoutePath::try_from(String::from(suffix))
                .unwrap_or_default()
        } else {
            crate::admin_api_route_path::admin_api_route_path(suffix)
        }
    }
}
