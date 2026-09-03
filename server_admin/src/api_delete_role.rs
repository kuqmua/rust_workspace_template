#[proc_macro_frontend_contract::route_openapi(tag = "admin_roles")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_delete_role(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_path: crate::axum_admin_path::AxumAdminPath<
        server_admin_core::admin_role_record_id::AdminRoleRecordId,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminDeleteRoleError,
> {
    crate::role_mutations_delete::role_mutations_delete(admin_auth_request, axum_admin_path)
        .await
        .map_err(crate::application_auth::AdminDeleteRoleError::from)
}
