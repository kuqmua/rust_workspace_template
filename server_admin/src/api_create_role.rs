#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_roles")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_create_role(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_create_role_request::AdminCreateRoleRequest,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminCreateRoleError,
> {
    crate::role_mutations_create::role_mutations_create(admin_auth_request, axum_admin_json)
        .await
        .map_err(crate::application_auth::AdminCreateRoleError::from)
}
