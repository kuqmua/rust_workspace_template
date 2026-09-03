#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_auth")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_change_own_password(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_change_own_password_request::AdminChangeOwnPasswordRequest,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminChangeOwnPasswordError,
> {
    crate::account_change_own_password::account_change_own_password(
        admin_auth_request,
        axum_admin_json,
    )
    .await
    .map_err(crate::application_auth::AdminChangeOwnPasswordError::from)
}
