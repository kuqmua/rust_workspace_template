// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract_macros::route_openapi(
    delegate = crate::mutations_set_permissions::mutations_set_permissions,
    tag = "admin_roles"
)]
pub(crate) async fn api_set_role_permissions(
    auth: crate::admin_auth_req::AdminAuthReq,
    path: crate::axum_admin_path::AxumAdminPath<
        server_admin_core::admin_role_record_id::AdminRoleRecordId,
    >,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_set_role_permissions_req::AdminSetRolePermissionsReq,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminSetRolePermissionsError,
> {
}
