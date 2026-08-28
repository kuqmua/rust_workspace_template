use crate::{AdminCrudPage, crud_resource_page};

#[frontend_contract::route_error(AdminRolesManagePageError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn roles_manage_page(auth: crate::AdminAuthReq) -> axum::response::Response {
    crud_resource_page(auth, AdminCrudPage::RoleManage).await
}
