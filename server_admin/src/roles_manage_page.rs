#[proc_macro_frontend_contract::route_error(AdminRolesManagePageError)]
#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) async fn roles_manage_page(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> axum::response::Response {
    crate::crud_resource_page::crud_resource_page(
        admin_auth_request,
        crate::admin_crud_page::AdminCrudPage::RoleManage,
    )
    .await
}
