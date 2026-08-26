#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::sessions_revoke_session::sessions_revoke_session, tag = "admin_auth")]
pub(super) async fn api_revoke_session(
    auth: super::AdminAuthReq,
    session: super::AdminSessionPath,
) -> Result<super::AxumAdminResponse, super::AdminRevokeSessionError> {
}
