#[proc_macro_frontend_contract::route_openapi(tag = "admin_users")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_set_user_roles(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_path: crate::axum_admin_path::AxumAdminPath<
        server_admin_core::admin_user_record_id::AdminUserRecordId,
    >,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_set_user_roles_request::AdminSetUserRolesRequest,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminSetUserRolesError,
> {
    crate::mutations_set_roles::mutations_set_roles(
        admin_auth_request,
        axum_admin_path,
        axum_admin_json,
    )
    .await
    .map_err(crate::application_auth::AdminSetUserRolesError::from)
}
