// The owner module retains lint-sensitive semantics from the original implementation.

#[proc_macro_frontend_contract::route_openapi(
    delegate = crate::account_change_own_password::account_change_own_password,
    tag = "admin_auth"
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
}
