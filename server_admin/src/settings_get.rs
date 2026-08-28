#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn settings_get(
    auth: crate::AdminAuthReq,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let _actor = crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        crate::AdminPermission::SystemSettingsRead.as_str(),
        crate::StdAdminBool::from(false),
    )
    .await?;
    let settings = crate::repository::read_settings::read_settings(
        crate::repository::SqlxAdminRepositoryPoolRef::from(auth.state.as_ref().pool.as_ref()),
    )
    .await
    .map_err(crate::shared::map_repository_error::map_repository_error)?;
    Ok(crate::shared::json_response::json_response(settings))
}
