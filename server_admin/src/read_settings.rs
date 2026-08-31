pub(crate) async fn read_settings(
    pool: crate::sqlx_admin_repository_pool_ref::SqlxAdminRepositoryPoolRef<'_>,
) -> Result<
    server_admin_contract::admin_settings_view::AdminSettingsView,
    crate::admin_repository_error::AdminRepositoryError,
> {
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
    .fetch_one(*pool)
    .await
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
    Ok(server_admin_contract::admin_settings_view::AdminSettingsView::new(
        server_admin_contract::admin_default_route::AdminDefaultRoute::try_from(row.4).map_err(
            |_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
        )?,
        Some(
            server_admin_contract::admin_main_logo::AdminMainLogo::try_from(row.2).map_err(
                |_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
            )?,
        ),
        Some(
            server_admin_contract::admin_organization_contacts::AdminOrganizationContacts::try_from(row.6)
                .map_err(|_error| {
                    crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                })?,
        ),
        Some(
            server_admin_contract::admin_organization_name::AdminOrganizationName::try_from(row.5).map_err(
                |_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
            )?,
        ),
        Some(
            server_admin_contract::admin_primary_color::AdminPrimaryColor::try_from(row.3).map_err(
                |_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
            )?,
        ),
        server_admin_contract::admin_site_name::AdminSiteName::try_from(row.0).map_err(|_error| {
            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
        })?,
        Some(
            server_admin_contract::admin_support_url::AdminSupportUrl::try_from(row.7).map_err(
                |_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
            )?,
        ),
        Some(
            server_admin_contract::admin_tab_title::AdminTabTitle::try_from(row.1).map_err(
                |_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
            )?,
        ),
    ))
}
