// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::route_openapi(
    delegate = crate::sessions::sessions,
    params(server_admin_contract::domain_types::AdminTableQuery),
    tag = "admin_auth"
)]
pub(crate) async fn api_sessions(
    auth: crate::AdminAuthReq,
    query: crate::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<crate::AxumAdminResponse, crate::AdminSessionsError> {
}
