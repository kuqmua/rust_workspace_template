// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::route_openapi(
    delegate = crate::queries_list_permissions::queries_list_permissions,
    params(server_admin_contract::domain_types::AdminTableQuery),
    tag = "admin_roles"
)]
pub(crate) async fn api_list_permissions(
    auth: crate::AdminAuthReq,
    query: crate::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<crate::AxumAdminResponse, crate::AdminListPermissionsError> {
}
