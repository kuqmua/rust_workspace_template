// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract_macros::route_openapi(
    delegate = crate::sessions_revoke_all_sessions::sessions_revoke_all_sessions,
    tag = "admin_auth"
)]
pub(crate) async fn api_revoke_all_sessions(
    auth: crate::admin_auth_req::AdminAuthReq,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminRevokeAllSessionsError,
> {
}
