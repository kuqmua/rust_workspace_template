#[proc_macro_frontend_contract_route_error::route_error(AdminUserRoleReadPageError)]
#[allow(
    clippy::single_call_fn,
    reason = "typed frontend route registration requires a named endpoint function"
)]
pub(crate) async fn user_role_read_page(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_path: crate::axum_admin_path::AxumAdminPath<
        server_admin_contract::admin_user_role_id::AdminUserRoleId,
    >,
) -> axum::response::Response {
    let _admin_user_role_id = axum_admin_path.into_inner();
    crate::csr_table_page::csr_table_page(
        admin_auth_request,
        server_admin_contract::admin_data_table::AdminDataTable::UserRoles,
    )
    .await
}
