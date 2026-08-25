fn admin_permission_requirement(
    permission: super::AdminPermission,
) -> frontend_contract::domain_types::AuthenticationRequirement {
    frontend_contract::domain_types::AuthenticationRequirement::Permission(
        frontend_contract::domain_types::ContractStr::from(permission.as_str().get()),
    )
}
#[cfg(test)]
mod tests;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::domain_types::AuthenticationRequirement::Public,
    method = frontend_contract::domain_types::RouteMethod::Post,
    mutation = frontend_contract::domain_types::RouteMutation::Mutating,
    obligations = frontend_contract::domain_types::PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    error_policy = frontend_contract::domain_types::RouteErrorPolicy::Authentication,
    openapi_operation_id = "sign_in",
    path = "/auth/sign_in",
    request = super::AdminSignInReq,
    request_body = frontend_contract::domain_types::RouteRequestBody::Json,
    response = super::AdminSignInRes,
    success_status = frontend_contract::domain_types::SuccessStatus::Code200,
    transport = frontend_contract::domain_types::PublicTransport,
)]
pub struct AdminSignInRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::domain_types::AuthenticationRequirement::Public,
    method = frontend_contract::domain_types::RouteMethod::Post,
    mutation = frontend_contract::domain_types::RouteMutation::Mutating,
    obligations = frontend_contract::domain_types::PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    error_policy = frontend_contract::domain_types::RouteErrorPolicy::Authentication,
    openapi_operation_id = "refresh",
    path = "/auth/refresh",
    request = super::AdminNoBody,
    response = super::AdminSignInRes,
    success_status = frontend_contract::domain_types::SuccessStatus::Code200,
    transport = frontend_contract::domain_types::PublicTransport,
)]
pub struct AdminRefreshRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::domain_types::AuthenticationRequirement::Authenticated,
    method = frontend_contract::domain_types::RouteMethod::Get,
    mutation = frontend_contract::domain_types::RouteMutation::ReadOnly,
    obligations = frontend_contract::domain_types::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS,
    error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default,
    openapi_operation_id = "me",
    path = "/auth/me",
    request = super::AdminNoBody,
    response = super::AuthenticatedAdmin,
    success_status = frontend_contract::domain_types::SuccessStatus::Code200,
    transport = frontend_contract::domain_types::AuthenticatedTransport,
)]
pub struct AdminMeRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = frontend_contract::domain_types::AuthenticationRequirement::Authenticated, method = frontend_contract::domain_types::RouteMethod::Post, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "change_own_password", path = "/auth/password", request = super::AdminChangeOwnPasswordReq, request_body = frontend_contract::domain_types::RouteRequestBody::Json, response = super::AdminNoBody, success_status = frontend_contract::domain_types::SuccessStatus::Code204, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminChangeOwnPasswordRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = frontend_contract::domain_types::AuthenticationRequirement::Authenticated, method = frontend_contract::domain_types::RouteMethod::Post, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "sign_out", path = "/auth/sign_out", request = super::AdminNoBody, response = super::AdminNoBody, success_status = frontend_contract::domain_types::SuccessStatus::Code204, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminSignOutRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = frontend_contract::domain_types::AuthenticationRequirement::Authenticated, method = frontend_contract::domain_types::RouteMethod::Get, mutation = frontend_contract::domain_types::RouteMutation::ReadOnly, obligations = frontend_contract::domain_types::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "sessions", path = "/auth/sessions", request = super::AdminNoBody, response = super::AdminSessionsPage, success_status = frontend_contract::domain_types::SuccessStatus::Code200, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminSessionsRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = frontend_contract::domain_types::AuthenticationRequirement::Authenticated, method = frontend_contract::domain_types::RouteMethod::Delete, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "revoke_session", path = "/auth/sessions/{session_id}", path_parameter = super::AdminSessionIdentifier, request = super::AdminNoBody, response = super::AdminNoBody, success_status = frontend_contract::domain_types::SuccessStatus::Code204, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminRevokeSessionRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = frontend_contract::domain_types::AuthenticationRequirement::Authenticated, method = frontend_contract::domain_types::RouteMethod::Delete, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "revoke_all_sessions", path = "/auth/sessions", request = super::AdminNoBody, response = super::AdminNoBody, success_status = frontend_contract::domain_types::SuccessStatus::Code204, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminRevokeAllSessionsRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(super::AdminPermission::UsersRead), method = frontend_contract::domain_types::RouteMethod::Get, mutation = frontend_contract::domain_types::RouteMutation::ReadOnly, obligations = frontend_contract::domain_types::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_users", path = "/users", request = super::AdminNoBody, response = super::AdminUsersPage, success_status = frontend_contract::domain_types::SuccessStatus::Code200, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminListUsersRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(super::AdminPermission::UsersCreate), method = frontend_contract::domain_types::RouteMethod::Post, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "create_user", path = "/users", request = super::AdminCreateUserReq, request_body = frontend_contract::domain_types::RouteRequestBody::Json, response = super::AdminCreateUserRes, success_status = frontend_contract::domain_types::SuccessStatus::Code201, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminCreateUserRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(super::AdminPermission::UsersUpdate), method = frontend_contract::domain_types::RouteMethod::Patch, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_user", path = "/users/{user_id}", path_parameter = super::AdminUserId, request = super::AdminUpdateUserReq, request_body = frontend_contract::domain_types::RouteRequestBody::Json, response = super::AdminNoBody, success_status = frontend_contract::domain_types::SuccessStatus::Code204, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminUpdateUserRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Delete, authentication = admin_permission_requirement(super::AdminPermission::UsersDelete), method = frontend_contract::domain_types::RouteMethod::Delete, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "delete_user", path = "/users/{user_id}", path_parameter = super::AdminUserId, request = super::AdminNoBody, response = super::AdminNoBody, success_status = frontend_contract::domain_types::SuccessStatus::Code204, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminDeleteUserRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(super::AdminPermission::UsersUpdate), method = frontend_contract::domain_types::RouteMethod::Post, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_user_password", path = "/users/{user_id}/password", path_parameter = super::AdminUserId, request = super::AdminSetUserPasswordReq, request_body = frontend_contract::domain_types::RouteRequestBody::Json, response = super::AdminNoBody, success_status = frontend_contract::domain_types::SuccessStatus::Code204, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminSetUserPasswordRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(super::AdminPermission::UsersUpdate), method = frontend_contract::domain_types::RouteMethod::Post, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_user_ban", path = "/users/{user_id}/ban", path_parameter = super::AdminUserId, request = super::AdminSetUserBanReq, request_body = frontend_contract::domain_types::RouteRequestBody::Json, response = super::AdminNoBody, success_status = frontend_contract::domain_types::SuccessStatus::Code204, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminSetUserBanRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(super::AdminPermission::UserRolesUpdate), method = frontend_contract::domain_types::RouteMethod::Put, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_user_roles", path = "/users/{user_id}/roles", path_parameter = super::AdminUserId, request = super::AdminSetUserRolesReq, request_body = frontend_contract::domain_types::RouteRequestBody::Json, response = super::AdminNoBody, success_status = frontend_contract::domain_types::SuccessStatus::Code204, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminSetUserRolesRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(super::AdminPermission::RolesRead), method = frontend_contract::domain_types::RouteMethod::Get, mutation = frontend_contract::domain_types::RouteMutation::ReadOnly, obligations = frontend_contract::domain_types::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_roles", path = "/roles", request = super::AdminNoBody, response = super::AdminRolesPage, success_status = frontend_contract::domain_types::SuccessStatus::Code200, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminListRolesRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(super::AdminPermission::RolesCreate), method = frontend_contract::domain_types::RouteMethod::Post, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "create_role", path = "/roles", request = super::AdminCreateRoleReq, request_body = frontend_contract::domain_types::RouteRequestBody::Json, response = super::AdminCreateRoleRes, success_status = frontend_contract::domain_types::SuccessStatus::Code201, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminCreateRoleRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(super::AdminPermission::RolesUpdate), method = frontend_contract::domain_types::RouteMethod::Patch, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_role", path = "/roles/{role_id}", path_parameter = super::AdminRoleId, request = super::AdminUpdateRoleReq, request_body = frontend_contract::domain_types::RouteRequestBody::Json, response = super::AdminNoBody, success_status = frontend_contract::domain_types::SuccessStatus::Code204, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminUpdateRoleRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Delete, authentication = admin_permission_requirement(super::AdminPermission::RolesDelete), method = frontend_contract::domain_types::RouteMethod::Delete, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "delete_role", path = "/roles/{role_id}", path_parameter = super::AdminRoleId, request = super::AdminNoBody, response = super::AdminNoBody, success_status = frontend_contract::domain_types::SuccessStatus::Code204, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminDeleteRoleRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(super::AdminPermission::RolePermissionsUpdate), method = frontend_contract::domain_types::RouteMethod::Put, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_role_permissions", path = "/roles/{role_id}/permissions", path_parameter = super::AdminRoleId, request = super::AdminSetRolePermissionsReq, request_body = frontend_contract::domain_types::RouteRequestBody::Json, response = super::AdminNoBody, success_status = frontend_contract::domain_types::SuccessStatus::Code204, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminSetRolePermissionsRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(super::AdminPermission::PermissionsRead), method = frontend_contract::domain_types::RouteMethod::Get, mutation = frontend_contract::domain_types::RouteMutation::ReadOnly, obligations = frontend_contract::domain_types::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_permissions", path = "/permissions", request = super::AdminNoBody, response = super::AdminPermissionsPage, success_status = frontend_contract::domain_types::SuccessStatus::Code200, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminListPermissionsRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::ValidatedRead, authentication = admin_permission_requirement(super::AdminPermission::AuditLogRead), method = frontend_contract::domain_types::RouteMethod::Get, mutation = frontend_contract::domain_types::RouteMutation::ReadOnly, obligations = frontend_contract::domain_types::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "audit_log", path = "/audit_log", request = super::AdminNoBody, response = super::AdminAuditPage, success_status = frontend_contract::domain_types::SuccessStatus::Code200, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminAuditLogRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::ValidatedRead, authentication = admin_permission_requirement(super::AdminPermission::AuditLogExport), method = frontend_contract::domain_types::RouteMethod::Get, mutation = frontend_contract::domain_types::RouteMutation::ReadOnly, obligations = frontend_contract::domain_types::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "export_audit_log", path = "/audit_log/export", request = super::AdminNoBody, response = super::AdminAuditExport, success_status = frontend_contract::domain_types::SuccessStatus::Code200, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminAuditExportRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = frontend_contract::domain_types::AuthenticationRequirement::Public, method = frontend_contract::domain_types::RouteMethod::Get, mutation = frontend_contract::domain_types::RouteMutation::ReadOnly, obligations = frontend_contract::domain_types::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "branding", path = "/branding", request = super::AdminNoBody, response = super::AdminBrandingView, success_status = frontend_contract::domain_types::SuccessStatus::Code200, transport = frontend_contract::domain_types::PublicTransport)]
pub struct AdminBrandingRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(super::AdminPermission::TablesRead), method = frontend_contract::domain_types::RouteMethod::Get, mutation = frontend_contract::domain_types::RouteMutation::ReadOnly, obligations = frontend_contract::domain_types::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_data_tables", path = "/tables", request = super::AdminNoBody, response = super::AdminDataTableCatalog, success_status = frontend_contract::domain_types::SuccessStatus::Code200, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminDataTablesRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::ValidatedRead, authentication = frontend_contract::domain_types::AuthenticationRequirement::Authenticated, method = frontend_contract::domain_types::RouteMethod::Get, mutation = frontend_contract::domain_types::RouteMutation::ReadOnly, obligations = frontend_contract::domain_types::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "read_data_table", path = "/tables/{table}", path_parameter = super::AdminDataTable, request = super::AdminNoBody, response = super::AdminDataTableView, success_status = frontend_contract::domain_types::SuccessStatus::Code200, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminDataTableRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(super::AdminPermission::SystemSettingsRead), method = frontend_contract::domain_types::RouteMethod::Get, mutation = frontend_contract::domain_types::RouteMutation::ReadOnly, obligations = frontend_contract::domain_types::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "settings", path = "/system_settings", request = super::AdminNoBody, response = super::AdminSettingsView, success_status = frontend_contract::domain_types::SuccessStatus::Code200, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminSettingsRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(super::AdminPermission::SystemSettingsUpdate), method = frontend_contract::domain_types::RouteMethod::Patch, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_settings", path = "/system_settings", request = super::AdminUpdateSettingsReq, request_body = frontend_contract::domain_types::RouteRequestBody::Json, response = super::AdminNoBody, success_status = frontend_contract::domain_types::SuccessStatus::Code204, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminUpdateSettingsRoute;

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
    body_limit = super::ADMIN_API_BODY_MAX_BYTES_VALUE,
)]
pub enum AdminRoute {
    #[route_catalog_route(AdminAuditLogRoute)]
    Audit,
    #[route_catalog_route(AdminAuditExportRoute)]
    AuditExport,
    #[route_catalog_route(AdminBrandingRoute)]
    Branding,
    #[route_catalog_route(AdminDataTableRoute)]
    DataTable(super::AdminDataTable),
    #[route_catalog_route(AdminDataTablesRoute)]
    DataTables,
    #[route_catalog_route(AdminChangeOwnPasswordRoute)]
    ChangeOwnPassword,
    #[route_catalog_route(AdminCreateRoleRoute)]
    CreateRole,
    #[route_catalog_route(AdminCreateUserRoute)]
    CreateUser,
    #[route_catalog_route(AdminDeleteRoleRoute)]
    DeleteRole(super::AdminRoleId),
    #[route_catalog_route(AdminDeleteUserRoute)]
    DeleteUser(super::AdminUserId),
    #[route_catalog_route(AdminMeRoute)]
    Me,
    #[route_catalog_route(
        contract = frontend_contract::domain_types::RouteContract::new(
            admin_permission_requirement(super::AdminPermission::MetricsRead),
            frontend_contract::domain_types::HttpMethod::Get,
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
            admin_permission_requirement(super::AdminPermission::OpenApiRead),
            frontend_contract::domain_types::HttpMethod::Get,
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
    SetRolePermissions(super::AdminRoleId),
    #[route_catalog_route(AdminSetUserBanRoute)]
    SetUserBan(super::AdminUserId),
    #[route_catalog_route(AdminSetUserPasswordRoute)]
    SetUserPassword(super::AdminUserId),
    #[route_catalog_route(AdminSetUserRolesRoute)]
    SetUserRoles(super::AdminUserId),
    #[route_catalog_route(AdminSettingsRoute)]
    Settings,
    #[route_catalog_route(AdminSignInRoute)]
    SignIn,
    #[route_catalog_route(AdminSignOutRoute)]
    SignOut,
    #[route_catalog_route(AdminSessionsRoute)]
    Sessions,
    #[route_catalog_route(AdminUpdateRoleRoute)]
    UpdateRole(super::AdminRoleId),
    #[route_catalog_route(AdminUpdateSettingsRoute)]
    UpdateSettings,
    #[route_catalog_route(AdminUpdateUserRoute)]
    UpdateUser(super::AdminUserId),
    #[route_catalog_route(AdminListUsersRoute)]
    Users,
    #[route_catalog_route(
        contract = frontend_contract::domain_types::RouteContract::new(
            frontend_contract::domain_types::AuthenticationRequirement::Public,
            frontend_contract::domain_types::HttpMethod::Get,
            frontend_contract::domain_types::MutationKind::ReadOnly,
            frontend_contract::domain_types::ContractStr::from(constants_str::COMMON_ROUTES_GIT_INFO),
            frontend_contract::domain_types::SuccessStatus::Code200,
        ),
        path = constants_str::COMMON_ROUTES_GIT_INFO,
        exclude_from_family,
    )]
    Version,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::Display,
)]
pub struct AdminDataTableFrontendPath(Box<str>);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::Display,
)]
pub struct AdminRoutePath(Box<str>);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdminRoutePathError {
    TooLong,
}
impl std::fmt::Display for AdminRoutePathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong => f.write_str(constants_str::ADMINISTRATOR_ROUTE_PATH_IS_TOO_LONG),
        }
    }
}
impl TryFrom<String> for AdminRoutePath {
    type Error = AdminRoutePathError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_8_192 {
            Err(AdminRoutePathError::TooLong)
        } else {
            Ok(Self(value.into_boxed_str()))
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub struct AdminPagePathRef<'path_lt>(&'path_lt str);
impl<'path_lt> AdminPagePathRef<'path_lt> {
    pub(crate) const fn get(self) -> &'path_lt str {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum_macros::IntoStaticStr,
)]
pub enum AdminFrontendPath {
    #[strum(serialize = "/admin/assets")]
    Assets,
    #[strum(serialize = "/admin/metrics")]
    Metrics,
    #[strum(serialize = "/admin/openapi.json")]
    OpenApiDocument,
    #[strum(serialize = "/admin/swagger_ui")]
    OpenApi,
    #[strum(serialize = "/admin/permissions")]
    Permissions,
    #[strum(serialize = "/admin/profile")]
    Profile,
    #[strum(serialize = "/admin/roles")]
    Roles,
    #[strum(serialize = "/admin/roles/create")]
    RolesCreate,
    #[strum(serialize = "/admin/roles/manage")]
    RolesManage,
    #[strum(serialize = "/admin/sessions")]
    Sessions,
    #[strum(serialize = "/admin")]
    Root,
    #[strum(serialize = "/admin/sign_in")]
    SignIn,
    #[strum(serialize = "/admin/settings")]
    Settings,
    #[strum(serialize = "/admin/{table}")]
    Tables,
    #[strum(serialize = "/admin/users")]
    Users,
    #[strum(serialize = "/admin/users/create")]
    UsersCreate,
    #[strum(serialize = "/admin/users/manage")]
    UsersManage,
    #[strum(serialize = "/admin/version")]
    Version,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum_macros::IntoStaticStr,
    frontend_contract::domain_types::UnitEnumCatalog,
)]
pub enum AdminHtmlAction {
    #[strum(serialize = "/admin/actions/profile/password")]
    ProfilePassword,
    #[strum(serialize = "/admin/actions/roles/create")]
    RoleCreate,
    #[strum(serialize = "/admin/actions/roles/delete")]
    RoleDelete,
    #[strum(serialize = "/admin/actions/roles/permissions")]
    RolePermissions,
    #[strum(serialize = "/admin/actions/roles/update")]
    RoleUpdate,
    #[strum(serialize = "/admin/actions/sessions/revoke")]
    SessionRevoke,
    #[strum(serialize = "/admin/actions/settings/update")]
    SettingsUpdate,
    #[strum(serialize = "/admin/actions/sign_in")]
    SignIn,
    #[strum(serialize = "/admin/actions/sign_out")]
    SignOut,
    #[strum(serialize = "/admin/actions/users/ban")]
    UserBan,
    #[strum(serialize = "/admin/actions/users/create")]
    UserCreate,
    #[strum(serialize = "/admin/actions/users/delete")]
    UserDelete,
    #[strum(serialize = "/admin/actions/users/password")]
    UserPassword,
    #[strum(serialize = "/admin/actions/users/roles")]
    UserRoles,
    #[strum(serialize = "/admin/actions/users/update")]
    UserUpdate,
}
impl AdminHtmlAction {
    #[must_use]
    pub fn get(self) -> &'static str {
        <&'static str>::from(self)
    }
    #[must_use]
    pub fn route_name(self) -> frontend_contract::domain_types::ContractStr {
        admin_path_route_name(AdminPagePathRef::from(self.get()))
    }
}
impl frontend_contract::domain_types::RouteRegistrationContract for AdminHtmlAction {
    fn method(self) -> frontend_contract::domain_types::RouteMethod {
        frontend_contract::domain_types::RouteMethod::Post
    }
    fn path(self) -> frontend_contract::domain_types::RegisteredRoutePath {
        frontend_contract::domain_types::RegisteredRoutePath::from(self.get())
    }
}
impl AdminFrontendPath {
    pub fn all_pages() -> impl Iterator<Item = Self> {
        [Self::Root, Self::SignIn]
            .into_iter()
            .chain(AdminPage::specs().iter().map(|spec| spec.frontend_path()))
    }
    #[must_use]
    pub fn get(self) -> &'static str {
        <&'static str>::from(self)
    }
}
impl frontend_contract::domain_types::RouteRegistrationContract for AdminFrontendPath {
    fn method(self) -> frontend_contract::domain_types::RouteMethod {
        frontend_contract::domain_types::RouteMethod::Get
    }
    fn path(self) -> frontend_contract::domain_types::RegisteredRoutePath {
        frontend_contract::domain_types::RegisteredRoutePath::from(self.get())
    }
}
impl From<super::AdminDataTable> for AdminDataTableFrontendPath {
    fn from(value: super::AdminDataTable) -> Self {
        Self(format!("{}/{}", AdminFrontendPath::Root.get(), value).into_boxed_str())
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    frontend_contract::domain_types::PageCatalog,
)]
#[page_catalog(
    spec = AdminPageSpec,
    path_ref = AdminPagePathRef,
    inventory = ADMIN_PAGE_SPECS,
)]
pub enum AdminPage {
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(AdminPageClientMode::CsrTableQuery, None),
        path = AdminFrontendPath::Users,
        route = AdminRoute::Users,
        title = AdminPageTitle::Users,
    )]
    Users,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(AdminPageClientMode::CsrTableQuery, None),
        path = AdminFrontendPath::Roles,
        route = AdminRoute::Roles,
        title = AdminPageTitle::Roles,
    )]
    Roles,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(AdminPageClientMode::CsrTableQuery, None),
        path = AdminFrontendPath::Permissions,
        route = AdminRoute::Permissions,
        title = AdminPageTitle::Permissions,
    )]
    Permissions,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(
            AdminPageClientMode::Csr,
            Some(AdminPageNavigation::Settings),
        ),
        path = AdminFrontendPath::Settings,
        route = AdminRoute::Settings,
        title = AdminPageTitle::Settings,
    )]
    Settings,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(AdminPageClientMode::Csr, None),
        path = AdminFrontendPath::Tables,
        route = AdminRoute::DataTables,
        title = AdminPageTitle::Tables,
    )]
    Tables,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(
            AdminPageClientMode::Csr,
            Some(AdminPageNavigation::Sessions),
        ),
        path = AdminFrontendPath::Sessions,
        route = AdminRoute::Sessions,
        title = AdminPageTitle::Sessions,
    )]
    Sessions,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(
            AdminPageClientMode::Ssr,
            Some(AdminPageNavigation::Metrics),
        ),
        path = AdminFrontendPath::Metrics,
        route = AdminRoute::Metrics,
        title = AdminPageTitle::Metrics,
    )]
    Metrics,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(
            AdminPageClientMode::Ssr,
            Some(AdminPageNavigation::Version),
        ),
        path = AdminFrontendPath::Version,
        route = AdminRoute::Version,
        title = AdminPageTitle::Version,
    )]
    Version,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(
            AdminPageClientMode::Csr,
            Some(AdminPageNavigation::Profile),
        ),
        path = AdminFrontendPath::Profile,
        route = AdminRoute::ChangeOwnPassword,
        title = AdminPageTitle::Profile,
    )]
    Profile,
    #[page_catalog_page(
        capability = AdminPageCapability::Swagger,
        metadata = AdminPageMetadata::new(
            AdminPageClientMode::Ssr,
            Some(AdminPageNavigation::OpenApi),
        ),
        path = AdminFrontendPath::OpenApi,
        route = AdminRoute::OpenApi,
        title = AdminPageTitle::Api,
    )]
    OpenApi,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminPageCapability {
    Always,
    Swagger,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminPageClientMode {
    Csr,
    CsrTableQuery,
    Ssr,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum AdminPageNavigation {
    OpenApi,
    Metrics,
    Profile,
    Sessions,
    Settings,
    Version,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdminPageMetadata {
    client_mode: AdminPageClientMode,
    navigation: Option<AdminPageNavigation>,
}
impl AdminPageMetadata {
    const fn new(
        client_mode: AdminPageClientMode,
        navigation: Option<AdminPageNavigation>,
    ) -> Self {
        Self {
            client_mode,
            navigation,
        }
    }
}
impl AdminPageClientMode {
    fn supports_csr(self) -> super::AdminBool {
        super::AdminBool::from(matches!(self, Self::Csr | Self::CsrTableQuery))
    }
    fn uses_table_query(self) -> super::AdminBool {
        super::AdminBool::from(matches!(self, Self::CsrTableQuery))
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
enum AdminPageTitle {
    Api,
    Metrics,
    Permissions,
    Profile,
    Roles,
    Sessions,
    Settings,
    Tables,
    Users,
    Version,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdminPageSpec {
    route: AdminRoute,
    capability: AdminPageCapability,
    metadata: AdminPageMetadata,
    page: AdminPage,
    path: AdminFrontendPath,
    title: AdminPageTitle,
}
impl AdminPageSpec {
    const fn new(
        capability: AdminPageCapability,
        metadata: AdminPageMetadata,
        page: AdminPage,
        path: AdminFrontendPath,
        route: AdminRoute,
        title: AdminPageTitle,
    ) -> Self {
        Self {
            route,
            capability,
            metadata,
            page,
            path,
            title,
        }
    }
    #[must_use]
    pub const fn capability(self) -> AdminPageCapability {
        self.capability
    }
    #[must_use]
    pub const fn client_mode(self) -> AdminPageClientMode {
        self.metadata.client_mode
    }
    #[must_use]
    pub const fn navigation(self) -> Option<AdminPageNavigation> {
        self.metadata.navigation
    }
    #[must_use]
    pub const fn frontend_path(self) -> AdminFrontendPath {
        self.path
    }
    #[must_use]
    pub const fn page(self) -> AdminPage {
        self.page
    }
    #[must_use]
    pub fn path(self) -> frontend_contract::domain_types::ContractStr {
        frontend_contract::domain_types::ContractStr::from(self.path.get())
    }
    #[must_use]
    pub fn route_name(self) -> frontend_contract::domain_types::ContractStr {
        admin_path_route_name(AdminPagePathRef::from(self.path.get()))
    }
    #[must_use]
    pub const fn route(self) -> AdminRoute {
        self.route
    }
    #[must_use]
    pub fn title(self) -> frontend_contract::domain_types::ContractStr {
        frontend_contract::domain_types::ContractStr::from(match self.title {
            AdminPageTitle::Api => constants_str::API_ALT,
            AdminPageTitle::Metrics => constants_str::METRICS_ALT,
            AdminPageTitle::Permissions => constants_str::PERMISSIONS,
            AdminPageTitle::Profile => constants_str::PROFILE,
            AdminPageTitle::Roles => constants_str::ROLES,
            AdminPageTitle::Sessions => constants_str::SESSIONS_ALT,
            AdminPageTitle::Settings => constants_str::SETTINGS,
            AdminPageTitle::Tables => constants_str::TABLES,
            AdminPageTitle::Users => constants_str::USERS,
            AdminPageTitle::Version => constants_str::VERSION_ALT,
        })
    }
}
fn admin_path_route_name(
    path: AdminPagePathRef<'static>,
) -> frontend_contract::domain_types::ContractStr {
    frontend_contract::domain_types::ContractStr::from(
        path.0
            .rsplit_once('/')
            .map_or(path.0, |(_prefix, name)| name),
    )
}
impl AdminPage {
    pub fn navigation() -> impl Iterator<Item = Self> {
        let mut pages = Self::specs()
            .iter()
            .filter_map(|spec| {
                spec.navigation()
                    .map(|navigation| (navigation, spec.page()))
            })
            .collect::<Vec<_>>();
        pages.sort_by_key(|(navigation, _page)| *navigation);
        pages.into_iter().map(|(_navigation, page)| page)
    }

    #[must_use]
    pub fn supports_csr(self) -> super::AdminBool {
        self.spec().client_mode().supports_csr()
    }
    #[must_use]
    pub fn uses_table_query(self) -> super::AdminBool {
        self.spec().client_mode().uses_table_query()
    }
    #[must_use]
    pub fn path(self) -> frontend_contract::domain_types::ContractStr {
        self.spec().path()
    }
    #[must_use]
    pub const fn route(self) -> Option<AdminRoute> {
        Some(self.spec().route())
    }
    #[must_use]
    pub fn title(self) -> frontend_contract::domain_types::ContractStr {
        self.spec().title()
    }
    #[must_use]
    pub fn authentication(self) -> frontend_contract::domain_types::AuthenticationRequirement {
        self.spec().route().contract().authentication()
    }
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
#[must_use]
pub fn admin_parameterized_route_path<Route>(parameter: &Route::Parameter) -> AdminRoutePath
where
    Route: frontend_contract::domain_types::ParameterizedRoute,
{
    admin_api_route_path(
        frontend_contract::domain_types::typed_parameterized_route_path::<Route>(parameter),
    )
}
fn admin_api_route_path(
    suffix: frontend_contract::domain_types::ParameterizedRoutePath,
) -> AdminRoutePath {
    AdminRoutePath::try_from(format!(
        "{}{}{suffix}",
        constants_str::V1,
        AdminFrontendPath::Root.get(),
        suffix = String::from(suffix),
    ))
    .unwrap_or_default()
}
