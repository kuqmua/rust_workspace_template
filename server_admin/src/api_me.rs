// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract_macros::route_openapi(delegate = crate::account_me::account_me, tag = "admin_auth")]
pub(crate) async fn api_me(
    auth: crate::admin_auth_req::AdminAuthReq,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::application_auth::AdminMeError> {
}
