// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::users::user_mutations_delete::user_mutations_delete, tag = "admin_users")]
pub(super) async fn api_delete_user(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
) -> Result<super::AxumAdminResponse, super::AdminDeleteUserError> {
}
