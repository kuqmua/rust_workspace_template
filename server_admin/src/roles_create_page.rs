#[proc_macro_frontend_contract::route_error(AdminRolesCreatePageError)]
#[allow(
    clippy::single_call_fn,
    reason = "roles create page remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn roles_create_page(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> axum::response::Response {
    crate::crud_resource_page::crud_resource_page(
        admin_auth_request,
        crate::admin_crud_page::AdminCrudPage::RoleCreate,
    )
    .await
}
