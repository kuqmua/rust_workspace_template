// The owner module retains lint-sensitive semantics from the original implementation.

#[proc_macro_frontend_contract::route_openapi(delegate = crate::sessions_revoke_session::sessions_revoke_session, tag = "admin_auth")]
pub(crate) async fn api_revoke_session(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_session_path: crate::admin_session_path::AdminSessionPath,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminRevokeSessionError,
> {
}
