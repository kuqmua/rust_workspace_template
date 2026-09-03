#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_users")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_update_user(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_path: crate::axum_admin_path::AxumAdminPath<
        server_admin_core::admin_user_record_id::AdminUserRecordId,
    >,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_update_user_request::AdminUpdateUserRequest,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminUpdateUserError,
> {
    crate::user_mutations_update::user_mutations_update(
        admin_auth_request,
        axum_admin_path,
        axum_admin_json,
    )
    .await
    .map_err(crate::application_auth::AdminUpdateUserError::from)
}
