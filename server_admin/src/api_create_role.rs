// The owner module retains lint-sensitive semantics from the original implementation.

#[proc_macro_frontend_contract::route_openapi(delegate = crate::role_mutations_create::role_mutations_create, tag = "admin_roles")]
pub(crate) async fn api_create_role(
    auth: crate::admin_auth_req::AdminAuthReq,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_create_role_req::AdminCreateRoleReq,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminCreateRoleError,
> {
}
