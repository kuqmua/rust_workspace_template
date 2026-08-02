#![allow(clippy::single_call_fn)] // route inventory and HTML composition each register focused settings operations once

pub(super) async fn update(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::AdminUpdateSettingsReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor =
        super::shared::authorize_custom(&auth, super::super::AdminPermission::SystemSettingsUpdate)
            .await?;
    if !bool::from(request.0.has_fields()) || !bool::from(request.0.is_valid()) {
        return Err(super::AdminError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::super::repository::settings::update_settings(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        request.0,
    )
    .await
    .map_err(super::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::SystemSettings,
            resource_id: super::AdminAuditResourceId::SystemSettings,
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn settings_view(
    auth: super::AdminAuthReq,
) -> Result<server_admin_contract::AdminSettingsView, super::AdminError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::SystemSettingsRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    super::super::repository::settings::read_settings(
        super::super::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
    )
    .await
    .map_err(super::shared::map_repository_error)
}
pub(super) async fn get(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    settings_view(auth).await.map(super::shared::json_response)
}
pub(super) async fn branding(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    branding_view(auth).await.map(super::shared::json_response)
}
pub(super) async fn branding_view(
    auth: super::AdminAuthReq,
) -> Result<server_admin_contract::AdminBrandingView, super::AdminError> {
    let settings = super::super::repository::settings::read_settings(
        super::super::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
    )
    .await
    .map_err(super::shared::map_repository_error)?;
    Ok(server_admin_contract::AdminBrandingView::from_settings(
        &settings,
    ))
}
