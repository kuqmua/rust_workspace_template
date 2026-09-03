#[allow(
    clippy::single_call_fn,
    reason = "user queries list remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn user_queries_list(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_table_query::AdminTableQuery,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    crate::queries_users_page::queries_users_page(admin_auth_request, axum_admin_query)
        .await
        .map(crate::json_response::json_response)
}
