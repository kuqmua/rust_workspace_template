#![allow(clippy::single_call_fn)] // route inventory registers this settings operation once

pub(super) async fn settings_get(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let _actor = super::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::SystemSettingsRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    let settings = crate::adapters::repository::read_settings::read_settings(
        crate::adapters::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
    )
    .await
    .map_err(super::shared::map_repository_error::map_repository_error)?;
    Ok(super::shared::json_response::json_response(settings))
}
