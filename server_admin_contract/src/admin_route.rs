use super::{
    AdminAuditExportRoute, AdminAuditLogRoute, AdminBrandingRoute, AdminChangeOwnPasswordRoute,
    AdminCreateRoleRoute, AdminCreateUserRoute, AdminDataTableRoute, AdminDataTablesRoute,
    AdminDeleteRoleRoute, AdminDeleteUserRoute, AdminListPermissionsRoute, AdminListRolesRoute,
    AdminListUsersRoute, AdminMeRoute, AdminRefreshRoute, AdminRevokeAllSessionsRoute,
    AdminRevokeSessionRoute, AdminRoutePath, AdminSessionsRoute, AdminSetRolePermissionsRoute,
    AdminSetUserBanRoute, AdminSetUserPasswordRoute, AdminSetUserRolesRoute, AdminSettingsRoute,
    AdminSignInRoute, AdminSignOutRoute, AdminUpdateRoleRoute, AdminUpdateSettingsRoute,
    AdminUpdateUserRoute, admin_api_route_path, admin_permission_requirement,
};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    frontend_contract::domain_types::RouteCatalog,
)]
#[route_catalog(
    family = AdminAuthenticationRouteFamily,
    body_limit = crate::domain_types::ADMIN_API_BODY_MAX_BYTES_VALUE,
)]
pub enum AdminRoute {
    #[route_catalog_route(AdminAuditLogRoute)]
    Audit,
    #[route_catalog_route(AdminAuditExportRoute)]
    AuditExport,
    #[route_catalog_route(AdminBrandingRoute)]
    Branding,
    #[route_catalog_route(AdminDataTableRoute)]
    DataTable(crate::domain_types::AdminDataTable),
    #[route_catalog_route(AdminDataTablesRoute)]
    DataTables,
    #[route_catalog_route(AdminChangeOwnPasswordRoute)]
    ChangeOwnPassword,
    #[route_catalog_route(AdminCreateRoleRoute)]
    CreateRole,
    #[route_catalog_route(AdminCreateUserRoute)]
    CreateUser,
    #[route_catalog_route(AdminDeleteRoleRoute)]
    DeleteRole(crate::domain_types::AdminRoleId),
    #[route_catalog_route(AdminDeleteUserRoute)]
    DeleteUser(crate::domain_types::AdminUserId),
    #[route_catalog_route(AdminMeRoute)]
    Me,
    #[route_catalog_route(
        contract = frontend_contract::domain_types::RouteContract::new(
            admin_permission_requirement(crate::domain_types::AdminPermission::MetricsRead),
            frontend_contract::domain_types::RouteMethod::Get,
            frontend_contract::domain_types::MutationKind::ReadOnly,
            frontend_contract::domain_types::ContractStr::from(constants_str::METRICS),
            frontend_contract::domain_types::SuccessStatus::Code200,
        ),
        path = constants_str::METRICS,
        exclude_from_family,
    )]
    Metrics,
    #[route_catalog_route(
        contract = frontend_contract::domain_types::RouteContract::new(
            admin_permission_requirement(crate::domain_types::AdminPermission::OpenApiRead),
            frontend_contract::domain_types::RouteMethod::Get,
            frontend_contract::domain_types::MutationKind::ReadOnly,
            frontend_contract::domain_types::ContractStr::from(constants_str::OPENAPI_JSON),
            frontend_contract::domain_types::SuccessStatus::Code200,
        ),
        path = constants_str::OPENAPI_JSON,
        exclude_from_family,
    )]
    OpenApi,
    #[route_catalog_route(AdminListPermissionsRoute)]
    Permissions,
    #[route_catalog_route(AdminRefreshRoute)]
    Refresh,
    #[route_catalog_route(AdminRevokeAllSessionsRoute)]
    RevokeAllSessions,
    #[route_catalog_route(AdminRevokeSessionRoute)]
    RevokeSession,
    #[route_catalog_route(AdminListRolesRoute)]
    Roles,
    #[route_catalog_route(AdminSetRolePermissionsRoute)]
    SetRolePermissions(crate::domain_types::AdminRoleId),
    #[route_catalog_route(AdminSetUserBanRoute)]
    SetUserBan(crate::domain_types::AdminUserId),
    #[route_catalog_route(AdminSetUserPasswordRoute)]
    SetUserPassword(crate::domain_types::AdminUserId),
    #[route_catalog_route(AdminSetUserRolesRoute)]
    SetUserRoles(crate::domain_types::AdminUserId),
    #[route_catalog_route(AdminSettingsRoute)]
    Settings,
    #[route_catalog_route(AdminSignInRoute)]
    SignIn,
    #[route_catalog_route(AdminSignOutRoute)]
    SignOut,
    #[route_catalog_route(AdminSessionsRoute)]
    Sessions,
    #[route_catalog_route(AdminUpdateRoleRoute)]
    UpdateRole(crate::domain_types::AdminRoleId),
    #[route_catalog_route(AdminUpdateSettingsRoute)]
    UpdateSettings,
    #[route_catalog_route(AdminUpdateUserRoute)]
    UpdateUser(crate::domain_types::AdminUserId),
    #[route_catalog_route(AdminListUsersRoute)]
    Users,
    #[route_catalog_route(
        contract = frontend_contract::domain_types::RouteContract::new(
            frontend_contract::domain_types::AuthenticationRequirement::Public,
            frontend_contract::domain_types::RouteMethod::Get,
            frontend_contract::domain_types::MutationKind::ReadOnly,
            frontend_contract::domain_types::ContractStr::from(constants_str::COMMON_ROUTES_GIT_INFO),
            frontend_contract::domain_types::SuccessStatus::Code200,
        ),
        path = constants_str::COMMON_ROUTES_GIT_INFO,
        exclude_from_family,
    )]
    Version,
}
impl AdminRoute {
    #[must_use]
    pub fn path(self) -> AdminRoutePath {
        let suffix = self.catalog_path();
        if matches!(self, Self::Version) {
            AdminRoutePath::try_from(String::from(suffix)).unwrap_or_default()
        } else {
            admin_api_route_path(suffix)
        }
    }
}
