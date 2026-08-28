pub(crate) async fn settings_update(
    auth: crate::AdminAuthReq,
    request: crate::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateSettingsReq>,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let actor = crate::shared::authorize_custom::authorize_custom(
        &auth,
        crate::AdminPermission::SystemSettingsUpdate,
    )
    .await?;
    if !bool::from(request.0.has_fields()) || !bool::from(request.0.is_valid()) {
        return Err(crate::AdminError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::AdminError::from)?;
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
        .map(|value| crate::StdAdminBool::from(value.is_some()))
        .map_err(crate::AdminError::from)?
        .get()
        .then_some(())
        .ok_or(crate::AdminError::Conflict)?;
    crate::persistence::record_audit_success_in_connection(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        crate::persistence::AdminAuditSuccessRef {
            action: crate::AdminAuditAction::Update,
            login: &actor.login,
            resource: crate::AdminAuditResource::SystemSettings,
            resource_id: crate::persistence::AdminAuditResourceId::SystemSettings,
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(crate::AdminError::from)?;
    Ok(crate::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
