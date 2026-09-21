#[proc_macro_frontend_contract_route_error::route_error(AdminAccessSessionReadPageError)]
#[allow(
    clippy::single_call_fn,
    reason = "typed frontend route registration requires a named endpoint function"
)]
pub(crate) async fn access_session_read_page(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_path: crate::axum_admin_path::AxumAdminPath<
        server_admin_contract::admin_access_session_id::AdminAccessSessionId,
    >,
) -> axum::response::Response {
    let _admin_access_session_id = axum_admin_path.into_inner();
    crate::csr_table_page::csr_table_page(
        admin_auth_request,
        server_admin_contract::admin_data_table::AdminDataTable::AccessSessions,
    )
    .await
}
