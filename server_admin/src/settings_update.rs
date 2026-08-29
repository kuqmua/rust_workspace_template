pub(crate) async fn settings_update(
    auth: crate::admin_auth_req::AdminAuthReq,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_update_settings_req::AdminUpdateSettingsReq,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &auth,
        server_admin_contract::admin_permission::AdminPermission::SystemSettingsUpdate,
    )
    .await?;
    if !bool::from(request.0.has_fields()) || !bool::from(request.0.is_valid()) {
        return Err(crate::admin_error::AdminError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
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
    sqlx::query_scalar::<_, bool>(
        constants_str::integration_fixtures::SERVER_ADMIN_UPDATE_SETTINGS_SQL,
    )
    .bind(site_name.as_ref().map(AsRef::<str>::as_ref))
    .bind(tab_title.as_ref().map(AsRef::<str>::as_ref))
    .bind(main_logo.as_ref().map(AsRef::<str>::as_ref))
    .bind(primary_color.as_ref().map(AsRef::<str>::as_ref))
    .bind(default_admin_route.as_ref().map(AsRef::<str>::as_ref))
    .bind(organization_name.as_ref().map(AsRef::<str>::as_ref))
    .bind(organization_contacts.as_ref().map(AsRef::<str>::as_ref))
    .bind(support_url.as_ref().map(AsRef::<str>::as_ref))
    .bind(
        clear.as_ref().contains(
            &server_admin_contract::admin_optional_setting::AdminOptionalSetting::TabTitle,
        ),
    )
    .bind(
        clear.as_ref().contains(
            &server_admin_contract::admin_optional_setting::AdminOptionalSetting::MainLogo,
        ),
    )
    .bind(clear.as_ref().contains(
        &server_admin_contract::admin_optional_setting::AdminOptionalSetting::PrimaryColor,
    ))
    .bind(clear.as_ref().contains(
        &server_admin_contract::admin_optional_setting::AdminOptionalSetting::OrganizationName,
    ))
    .bind(clear.as_ref().contains(
        &server_admin_contract::admin_optional_setting::AdminOptionalSetting::OrganizationContacts,
    ))
    .bind(
        clear.as_ref().contains(
            &server_admin_contract::admin_optional_setting::AdminOptionalSetting::SupportUrl,
        ),
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .map(|value| server_admin_core::std_admin_bool::StdAdminBool::from(value.is_some()))
    .map_err(crate::admin_error::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(crate::admin_error::AdminError::Conflict)?;
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        crate::admin_audit_success_ref::AdminAuditSuccessRef {
            action: crate::admin_audit_action::AdminAuditAction::Update,
            login: &actor.login,
            resource: crate::admin_audit_resource::AdminAuditResource::SystemSettings,
            resource_id: crate::admin_audit_resource_id::AdminAuditResourceId::SystemSettings,
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    Ok(crate::axum_admin_response::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
