pub(crate) async fn user_queries_list(
    auth: crate::AdminAuthReq,
    query: crate::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    crate::queries_users_page::queries_users_page(auth, query)
        .await
        .map(crate::shared::json_response::json_response)
}
