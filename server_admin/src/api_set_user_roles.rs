// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract_macros::route_openapi(delegate = crate::mutations_set_roles::mutations_set_roles, tag = "admin_users")]
pub(crate) async fn api_set_user_roles(
    auth: crate::admin_auth_req::AdminAuthReq,
    path: crate::axum_admin_path::AxumAdminPath<server_admin_core::admin_user_id::AdminUserId>,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_set_user_roles_req::AdminSetUserRolesReq,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminSetUserRolesError,
> {
}
