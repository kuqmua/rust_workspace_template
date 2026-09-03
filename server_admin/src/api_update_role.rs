#[proc_macro_frontend_contract::route_openapi(tag = "admin_roles")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_update_role(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_path: crate::axum_admin_path::AxumAdminPath<
        server_admin_core::admin_role_record_id::AdminRoleRecordId,
    >,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_update_role_request::AdminUpdateRoleRequest,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminUpdateRoleError,
> {
    crate::role_mutations_update::role_mutations_update(
        admin_auth_request,
        axum_admin_path,
        axum_admin_json,
    )
    .await
    .map_err(crate::application_auth::AdminUpdateRoleError::from)
}
