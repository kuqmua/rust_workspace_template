#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_users")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_update_users(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_update_users_request::AdminUpdateUsersRequest,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminUpdateUsersError,
> {
    let request = axum_admin_json.into_inner();
    crate::user_mutations_update_filtered::user_mutations_update_filtered(
        admin_auth_request,
        crate::admin_user_update_slice::AdminUserUpdateSlice::from(request.updates().as_ref()),
    )
    .await
    .map_err(crate::application_auth::AdminUpdateUsersError::from)
}
