// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract_macros::route_openapi(delegate = crate::role_mutations_update::role_mutations_update, tag = "admin_roles")]
pub(crate) async fn api_update_role(
    auth: crate::admin_auth_req::AdminAuthReq,
    path: crate::axum_admin_path::AxumAdminPath<server_admin_core::admin_role_id::AdminRoleId>,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_update_role_req::AdminUpdateRoleReq,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminUpdateRoleError,
> {
}
