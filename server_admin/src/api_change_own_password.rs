// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract_macros::route_openapi(
    delegate = crate::account_change_own_password::account_change_own_password,
    tag = "admin_auth"
)]
pub(crate) async fn api_change_own_password(
    auth: crate::admin_auth_req::AdminAuthReq,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_change_own_password_req::AdminChangeOwnPasswordReq,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminChangeOwnPasswordError,
> {
}
