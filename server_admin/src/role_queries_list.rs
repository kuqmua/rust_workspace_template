#[allow(clippy::single_call_fn)] // typed-route delegate keeps transport mapping out of query workflow
pub(in crate::domain_types::auth) async fn role_queries_list(
    auth: super::super::AdminAuthReq,
    query: super::super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    super::queries_roles_page::queries_roles_page(auth, query)
        .await
        .map(super::super::shared::json_response::json_response)
}
