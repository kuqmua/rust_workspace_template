// The owner module retains lint-sensitive semantics from the original implementation.

#[proc_macro_frontend_contract::route_openapi(
    delegate = crate::sessions::sessions,
    params(server_admin_contract::admin_table_query::AdminTableQuery),
    tag = "admin_auth"
)]
pub(crate) async fn api_sessions(
    auth: crate::admin_auth_req::AdminAuthReq,
    query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_table_query::AdminTableQuery,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminSessionsError,
> {
}
