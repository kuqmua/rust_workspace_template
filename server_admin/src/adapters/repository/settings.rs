#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

pub(crate) async fn read_settings(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
) -> Result<server_admin_contract::domain_types::AdminSettingsView, super::AdminRepositoryError> {
    let row = sqlx::query_as::<
        _,
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    >(constants_str::SERVER_ADMIN_READ_SETTINGS_SQL)
    .fetch_one(pool.0)
    .await
    .map_err(crate::domain_types::SqlxAdminError::from)?;
    Ok(server_admin_contract::domain_types::AdminSettingsView::new(
        server_admin_contract::domain_types::AdminDefaultRoute::try_from(row.4)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        Some(
            server_admin_contract::domain_types::AdminMainLogo::try_from(row.2)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        ),
        Some(
            server_admin_contract::domain_types::AdminOrganizationContacts::try_from(row.6)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        ),
        Some(
            server_admin_contract::domain_types::AdminOrganizationName::try_from(row.5)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        ),
        Some(
            server_admin_contract::domain_types::AdminPrimaryColor::try_from(row.3)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        ),
        server_admin_contract::domain_types::AdminSiteName::try_from(row.0)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        Some(
            server_admin_contract::domain_types::AdminSupportUrl::try_from(row.7)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        ),
        Some(
            server_admin_contract::domain_types::AdminTabTitle::try_from(row.1)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        ),
    ))
}

pub(crate) async fn update_settings(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    request: server_admin_contract::domain_types::AdminUpdateSettingsReq,
) -> Result<crate::domain_types::StdAdminBool, crate::domain_types::SqlxAdminError> {
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
    ) = request.into_parts();
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
        .fetch_optional(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| crate::domain_types::StdAdminBool::from(value.is_some()))
}
