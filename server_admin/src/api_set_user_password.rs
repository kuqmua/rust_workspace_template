// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::users::mutations_set_password::mutations_set_password,
    tag = "admin_users"
)]
pub(super) async fn api_set_user_password(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminSetUserPasswordReq>,
) -> Result<super::AxumAdminResponse, super::AdminSetUserPasswordError> {
}
