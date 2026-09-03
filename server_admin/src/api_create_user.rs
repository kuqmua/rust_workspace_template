#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_users")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_create_user(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_create_user_request::AdminCreateUserRequest,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminCreateUserError,
> {
    crate::user_mutations_create::user_mutations_create(admin_auth_request, axum_admin_json)
        .await
        .map_err(crate::application_auth::AdminCreateUserError::from)
}
