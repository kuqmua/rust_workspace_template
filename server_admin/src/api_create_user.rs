#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::users::mutations_create::mutations_create, tag = "admin_users")]
pub(super) async fn api_create_user(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminCreateUserReq>,
) -> Result<super::AxumAdminResponse, super::AdminCreateUserError> {
}
