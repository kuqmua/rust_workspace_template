use super::{AdminCrudPage, crud_resource_page};

#[frontend_contract::domain_types::route_error(AdminUsersCreatePageError)]
pub(in crate::domain_types::auth::html) async fn users_create_page(
    auth: super::super::super::AdminAuthReq,
) -> axum::response::Response {
    crud_resource_page(auth, AdminCrudPage::UserCreate).await
}
