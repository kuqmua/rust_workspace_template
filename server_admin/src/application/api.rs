#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::authn::sign_in, tag = "admin_auth")]
pub(super) async fn sign_in(
    auth: super::AdminAuthReq,
    peer: super::AdminPeerAddr,
    request_json: super::AdminSignInJson,
) -> Result<super::AxumAdminResponse, super::AdminSignInError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::account::me, tag = "admin_auth")]
pub(super) async fn me(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminMeError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::account::change_own_password,
    tag = "admin_auth"
)]
pub(super) async fn change_own_password(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminChangeOwnPasswordReq>,
) -> Result<super::AxumAdminResponse, super::AdminChangeOwnPasswordError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::authn::refresh, tag = "admin_auth")]
pub(super) async fn refresh(
    auth: super::AdminAuthReq,
    peer: super::AdminPeerAddr,
) -> Result<super::AxumAdminResponse, super::AdminRefreshError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::authn::sign_out, tag = "admin_auth")]
pub(super) async fn sign_out(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminSignOutError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::sessions::sessions,
    params(server_admin_contract::domain_types::AdminTableQuery),
    tag = "admin_auth"
)]
pub(super) async fn sessions(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminSessionsError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::sessions::revoke_session, tag = "admin_auth")]
pub(super) async fn revoke_session(
    auth: super::AdminAuthReq,
    session: super::AdminSessionPath,
) -> Result<super::AxumAdminResponse, super::AdminRevokeSessionError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::sessions::revoke_all_sessions,
    tag = "admin_auth"
)]
pub(super) async fn revoke_all_sessions(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminRevokeAllSessionsError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::users::create, tag = "admin_users")]
pub(super) async fn create_user(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminCreateUserReq>,
) -> Result<super::AxumAdminResponse, super::AdminCreateUserError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::users::update, tag = "admin_users")]
pub(super) async fn update_user(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateUserReq>,
) -> Result<super::AxumAdminResponse, super::AdminUpdateUserError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::users::set_password,
    tag = "admin_users"
)]
pub(super) async fn set_user_password(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminSetUserPasswordReq>,
) -> Result<super::AxumAdminResponse, super::AdminSetUserPasswordError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::users::set_ban, tag = "admin_users")]
pub(super) async fn set_user_ban(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminSetUserBanReq>,
) -> Result<super::AxumAdminResponse, super::AdminSetUserBanError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::users::delete, tag = "admin_users")]
pub(super) async fn delete_user(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
) -> Result<super::AxumAdminResponse, super::AdminDeleteUserError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::roles::create, tag = "admin_roles")]
pub(super) async fn create_role(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminCreateRoleReq>,
) -> Result<super::AxumAdminResponse, super::AdminCreateRoleError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::roles::update, tag = "admin_roles")]
pub(super) async fn update_role(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminRoleId>,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateRoleReq>,
) -> Result<super::AxumAdminResponse, super::AdminUpdateRoleError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::roles::delete, tag = "admin_roles")]
pub(super) async fn delete_role(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminRoleId>,
) -> Result<super::AxumAdminResponse, super::AdminDeleteRoleError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::roles::set_permissions,
    tag = "admin_roles"
)]
pub(super) async fn set_role_permissions(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminRoleId>,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminSetRolePermissionsReq>,
) -> Result<super::AxumAdminResponse, super::AdminSetRolePermissionsError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::users::set_roles, tag = "admin_users")]
pub(super) async fn set_user_roles(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminSetUserRolesReq>,
) -> Result<super::AxumAdminResponse, super::AdminSetUserRolesError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::audit::query_log,
    params(super::AdminAuditQuery),
    tag = "admin_audit"
)]
pub(super) async fn audit_log(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<super::AdminAuditQuery>,
) -> Result<super::AxumAdminResponse, super::AdminAuditLogError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::audit::export_log,
    params(super::AdminAuditQuery),
    tag = "admin_audit"
)]
pub(super) async fn export_audit_log(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<super::AdminAuditQuery>,
) -> Result<super::AxumAdminResponse, super::AdminAuditExportError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::settings::branding, tag = "admin_settings")]
pub(super) async fn branding(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminBrandingError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::data_tables::list, tag = "admin_tables")]
pub(super) async fn data_tables(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminDataTablesError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::data_tables::get,
    params(server_admin_contract::domain_types::AdminDataTableQuery),
    tag = "admin_tables"
)]
pub(super) async fn data_table(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<server_admin_contract::domain_types::AdminDataTable>,
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminDataTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminDataTableError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::settings::update,
    tag = "admin_settings"
)]
pub(super) async fn update_settings(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateSettingsReq>,
) -> Result<super::AxumAdminResponse, super::AdminUpdateSettingsError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::users::list,
    params(server_admin_contract::domain_types::AdminTableQuery),
    tag = "admin_users"
)]
pub(super) async fn list_users(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminListUsersError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::roles::list,
    params(server_admin_contract::domain_types::AdminTableQuery),
    tag = "admin_roles"
)]
pub(super) async fn list_roles(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminListRolesError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::roles::list_permissions,
    params(server_admin_contract::domain_types::AdminTableQuery),
    tag = "admin_roles"
)]
pub(super) async fn list_permissions(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminListPermissionsError> {
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::settings::get, tag = "admin_settings")]
pub(super) async fn settings(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminSettingsError> {
}
