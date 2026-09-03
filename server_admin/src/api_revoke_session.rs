#[proc_macro_frontend_contract::route_openapi(tag = "admin_auth")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_revoke_session(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_session_path: crate::admin_session_path::AdminSessionPath,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminRevokeSessionError,
> {
    crate::sessions_revoke_session::sessions_revoke_session(admin_auth_request, admin_session_path)
        .await
        .map_err(crate::application_auth::AdminRevokeSessionError::from)
}
