#[frontend_contract_macros::route_error(AdminUsersPageError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn users(auth: crate::admin_auth_req::AdminAuthReq) -> axum::response::Response {
    crate::csr_page::csr_page(
        auth,
        server_admin_contract::admin_page::AdminPage::Tables,
        Some(server_admin_contract::admin_data_table::AdminDataTable::Users),
    )
    .await
}
