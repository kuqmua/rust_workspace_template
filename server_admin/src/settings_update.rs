pub(crate) async fn settings_update(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_update_settings_request::AdminUpdateSettingsRequest,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &admin_auth_request,
        server_admin_contract::admin_permission::AdminPermission::SystemSettingsUpdate,
    )
    .await?;
    if !bool::from(axum_admin_json.get_inner().has_fields())
        || !bool::from(axum_admin_json.get_inner().is_valid())
    {
        return Err(crate::admin_error::AdminError::Validation);
    }
    let mut tx = admin_auth_request
        .get_state()
        .as_ref()
        .get_pool()
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
    ) = axum_admin_json.into_inner().into_parts();
    sqlx::query_scalar::<_, bool>(
        constants_str::SERVER_ADMIN_UPDATE_SETTINGS_SQL,
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
    crate::finalize_audited_transaction::finalize_audited_transaction(
        crate::sqlx_admin_transaction::SqlxAdminTransaction::from(tx),
        crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
            crate::admin_audit_action::AdminAuditAction::Update,
            actor.get_login(),
            crate::admin_audit_resource::AdminAuditResource::SystemSettings,
            crate::admin_audit_resource_id::AdminAuditResourceId::SystemSettings,
            *actor.get_id(),
        ),
    )
    .await?;
    Ok(crate::axum_admin_response::AxumAdminResponse::from(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
