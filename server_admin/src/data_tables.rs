#[proc_macro_frontend_contract::route_error(AdminDataTablesPageError)]
#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) async fn data_tables(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_path: crate::axum_admin_path::AxumAdminPath<
        server_admin_contract::admin_data_table::AdminDataTable,
    >,
) -> axum::response::Response {
    crate::csr_page::csr_page(
        admin_auth_request,
        server_admin_contract::admin_page::AdminPage::Tables,
        Some(axum_admin_path.into_inner()),
    )
    .await
}
