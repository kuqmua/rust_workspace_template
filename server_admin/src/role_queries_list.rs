pub(crate) async fn role_queries_list(
    auth: crate::AdminAuthReq,
    query: crate::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    crate::queries_roles_page::queries_roles_page(auth, query)
        .await
        .map(crate::shared::json_response::json_response)
}
