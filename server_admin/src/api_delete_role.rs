// The owner module retains lint-sensitive semantics from the original implementation.

#[proc_macro_frontend_contract::route_openapi(delegate = crate::role_mutations_delete::role_mutations_delete, tag = "admin_roles")]
pub(crate) async fn api_delete_role(
    auth: crate::admin_auth_req::AdminAuthReq,
    path: crate::axum_admin_path::AxumAdminPath<
        server_admin_core::admin_role_record_id::AdminRoleRecordId,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminDeleteRoleError,
> {
}
