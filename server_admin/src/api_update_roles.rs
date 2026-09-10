#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_roles")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_update_roles(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_update_roles_request::AdminUpdateRolesRequest,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminUpdateRolesError,
> {
    crate::dispatch_filtered_update::dispatch_filtered_update(admin_auth_request, axum_admin_json)
        .await
}
