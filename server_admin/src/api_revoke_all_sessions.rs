// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::domain_types::route_openapi(
    delegate = crate::sessions_revoke_all_sessions::sessions_revoke_all_sessions,
    tag = "admin_auth"
)]
pub(crate) async fn api_revoke_all_sessions(
    auth: crate::AdminAuthReq,
) -> Result<crate::AxumAdminResponse, crate::AdminRevokeAllSessionsError> {
}
