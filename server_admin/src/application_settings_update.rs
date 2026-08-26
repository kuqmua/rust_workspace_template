#![allow(clippy::single_call_fn)] // route inventory and HTML composition each register focused settings operations once

pub(super) async fn settings_update(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateSettingsReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = super::shared::authorize_custom::authorize_custom(
        &auth,
        super::super::AdminPermission::SystemSettingsUpdate,
    )
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
    super::persistence::record_audit_success_in_connection(
        super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::persistence::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::SystemSettings,
            resource_id: super::persistence::AdminAuditResourceId::SystemSettings,
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
