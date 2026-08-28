use crate::{AdminCrudPage, crud_resource_page};

#[frontend_contract::domain_types::route_error(AdminRolesManagePageError)]
pub(crate) async fn roles_manage_page(auth: crate::AdminAuthReq) -> axum::response::Response {
    crud_resource_page(auth, AdminCrudPage::RoleManage).await
}
