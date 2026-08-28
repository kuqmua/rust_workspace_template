// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::domain_types::route_openapi(delegate = crate::role_mutations_delete::role_mutations_delete, tag = "admin_roles")]
pub(crate) async fn api_delete_role(
    auth: crate::AdminAuthReq,
    path: crate::AxumAdminPath<crate::AdminRoleId>,
) -> Result<crate::AxumAdminResponse, crate::AdminDeleteRoleError> {
}
