#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_users")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_delete_user(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_path: crate::axum_admin_path::AxumAdminPath<
        server_admin_core::admin_user_record_id::AdminUserRecordId,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminDeleteUserError,
> {
    crate::user_mutations_delete::user_mutations_delete(admin_auth_request, axum_admin_path)
        .await
        .map_err(crate::application_auth::AdminDeleteUserError::from)
}
