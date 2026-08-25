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
