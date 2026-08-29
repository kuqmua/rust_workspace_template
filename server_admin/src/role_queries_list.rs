#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn role_queries_list(
    auth: crate::admin_auth_req::AdminAuthReq,
    query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_table_query::AdminTableQuery,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    crate::queries_roles_page::queries_roles_page(auth, query)
        .await
        .map(crate::json_response::json_response)
}
