#[proc_macro_frontend_contract_route_error::route_error(AdminRoleUpdatePageError)]
#[allow(
    clippy::single_call_fn,
    reason = "typed frontend route registration requires a named endpoint function"
)]
pub(crate) async fn role_update_page(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_path: crate::axum_admin_path::AxumAdminPath<
        server_admin_contract::admin_role_id::AdminRoleId,
    >,
) -> axum::response::Response {
    crate::crud_resource_page::crud_resource_page(
        admin_auth_request,
        crate::admin_crud_page::AdminCrudPage::RoleUpdate(Some(axum_admin_path.into_inner())),
    )
    .await
}
