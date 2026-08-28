use crate::{AdminCrudPage, crud_resource_page};

#[frontend_contract::domain_types::route_error(AdminUsersCreatePageError)]
pub(crate) async fn users_create_page(auth: crate::AdminAuthReq) -> axum::response::Response {
    crud_resource_page(auth, AdminCrudPage::UserCreate).await
}
