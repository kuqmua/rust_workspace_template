// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract_macros::route_openapi(delegate = crate::sessions_revoke_session::sessions_revoke_session, tag = "admin_auth")]
pub(crate) async fn api_revoke_session(
    auth: crate::admin_auth_req::AdminAuthReq,
    session: crate::admin_session_path::AdminSessionPath,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminRevokeSessionError,
> {
}
