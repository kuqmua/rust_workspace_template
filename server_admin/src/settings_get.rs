#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn settings_get(
    auth: crate::admin_auth_req::AdminAuthReq,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let _actor = crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.state.as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        server_admin_contract::admin_permission::AdminPermission::SystemSettingsRead.as_str(),
        server_admin_core::std_admin_bool::StdAdminBool::from(false),
    )
    .await?;
    let settings = crate::read_settings::read_settings(
        crate::sqlx_admin_repository_pool_ref::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
    )
    .await
    .map_err(crate::map_repository_error::map_repository_error)?;
    Ok(crate::json_response::json_response(settings))
}
