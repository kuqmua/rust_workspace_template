// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::route_openapi(delegate = crate::sessions_revoke_session::sessions_revoke_session, tag = "admin_auth")]
pub(crate) async fn api_revoke_session(
    auth: crate::AdminAuthReq,
    session: crate::AdminSessionPath,
) -> Result<crate::AxumAdminResponse, crate::AdminRevokeSessionError> {
}
