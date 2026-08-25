#![allow(clippy::single_call_fn)] // route inventory and HTML composition each register focused settings operations once

pub(super) async fn update(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateSettingsReq>,
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
    let (
        default_admin_route,
        main_logo,
        organization_contacts,
        organization_name,
        primary_color,
        site_name,
        support_url,
        tab_title,
        clear,
    ) = request.0.into_parts();
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_SETTINGS_SQL)
        .bind(site_name.as_ref().map(AsRef::<str>::as_ref))
        .bind(tab_title.as_ref().map(AsRef::<str>::as_ref))
        .bind(main_logo.as_ref().map(AsRef::<str>::as_ref))
        .bind(primary_color.as_ref().map(AsRef::<str>::as_ref))
        .bind(default_admin_route.as_ref().map(AsRef::<str>::as_ref))
        .bind(organization_name.as_ref().map(AsRef::<str>::as_ref))
        .bind(organization_contacts.as_ref().map(AsRef::<str>::as_ref))
        .bind(support_url.as_ref().map(AsRef::<str>::as_ref))
        .bind(
            clear
                .as_ref()
                .contains(&server_admin_contract::domain_types::AdminOptionalSetting::TabTitle),
        )
        .bind(
            clear
                .as_ref()
                .contains(&server_admin_contract::domain_types::AdminOptionalSetting::MainLogo),
        )
        .bind(
            clear
                .as_ref()
                .contains(&server_admin_contract::domain_types::AdminOptionalSetting::PrimaryColor),
        )
        .bind(
            clear.as_ref().contains(
                &server_admin_contract::domain_types::AdminOptionalSetting::OrganizationName,
            ),
        )
        .bind(clear.as_ref().contains(
            &server_admin_contract::domain_types::AdminOptionalSetting::OrganizationContacts,
        ))
        .bind(
            clear
                .as_ref()
                .contains(&server_admin_contract::domain_types::AdminOptionalSetting::SupportUrl),
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| super::super::StdAdminBool::from(value.is_some()))
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
pub(super) async fn get(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::SystemSettingsRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    let settings = crate::adapters::repository::settings::read_settings(
        crate::adapters::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
    )
    .await
    .map_err(super::shared::map_repository_error)?;
    Ok(super::shared::json_response(settings))
}
pub(super) async fn branding(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    branding_view(auth).await.map(super::shared::json_response)
}
pub(super) async fn branding_view(
    auth: super::AdminAuthReq,
) -> Result<server_admin_contract::domain_types::AdminBrandingView, super::AdminError> {
    branding_view_ref(&auth).await
}
pub(super) async fn branding_view_ref(
    auth: &super::AdminAuthReq,
) -> Result<server_admin_contract::domain_types::AdminBrandingView, super::AdminError> {
    let settings = crate::adapters::repository::settings::read_settings(
        crate::adapters::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
    )
    .await
    .map_err(super::shared::map_repository_error)?;
    Ok(server_admin_contract::domain_types::AdminBrandingView::from_settings(&settings))
}
