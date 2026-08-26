#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::roles::mutations_delete::mutations_delete, tag = "admin_roles")]
pub(super) async fn api_delete_role(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminRoleId>,
) -> Result<super::AxumAdminResponse, super::AdminDeleteRoleError> {
}
