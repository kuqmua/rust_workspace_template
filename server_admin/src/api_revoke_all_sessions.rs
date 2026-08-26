#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::sessions_revoke_all_sessions::revoke_all_sessions,
    tag = "admin_auth"
)]
pub(super) async fn revoke_all_sessions(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminRevokeAllSessionsError> {
}
