#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::users::mutations_update::mutations_update, tag = "admin_users")]
pub(super) async fn api_update_user(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateUserReq>,
) -> Result<super::AxumAdminResponse, super::AdminUpdateUserError> {
}
