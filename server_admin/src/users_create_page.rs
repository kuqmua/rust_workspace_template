use crate::{AdminCrudPage, crud_resource_page};

#[frontend_contract::route_error(AdminUsersCreatePageError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn users_create_page(auth: crate::AdminAuthReq) -> axum::response::Response {
    crud_resource_page(auth, AdminCrudPage::UserCreate).await
}
