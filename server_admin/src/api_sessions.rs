#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::sessions::sessions,
    params(server_admin_contract::domain_types::AdminTableQuery),
    tag = "admin_auth"
)]
pub(super) async fn api_sessions(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminSessionsError> {
}
