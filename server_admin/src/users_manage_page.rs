#[frontend_contract_macros::route_error(AdminUsersManagePageError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn users_manage_page(
    auth: crate::admin_auth_req::AdminAuthReq,
) -> axum::response::Response {
    crate::crud_resource_page::crud_resource_page(
        auth,
        crate::admin_crud_page::AdminCrudPage::UserManage,
    )
    .await
}
