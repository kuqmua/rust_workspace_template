#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

const READ_SETTINGS: &str = "SELECT site_name, tab_title, main_logo, primary_color, default_admin_route, organization_name, organization_contacts, support_url FROM admin_system_settings WHERE id = 1";
const UPDATE_SETTINGS: &str = "UPDATE admin_system_settings SET site_name = COALESCE($1, site_name), tab_title = COALESCE($2, tab_title), main_logo = COALESCE($3, main_logo), primary_color = COALESCE($4, primary_color), default_admin_route = COALESCE($5, default_admin_route), organization_name = COALESCE($6, organization_name), organization_contacts = COALESCE($7, organization_contacts), support_url = COALESCE($8, support_url) WHERE id = 1 RETURNING true";

pub(crate) async fn read_settings(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
) -> Result<server_admin_contract::AdminSettingsView, super::AdminRepositoryError> {
    let row = sqlx::query_as::<
        _,
        (
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
        ),
    >(READ_SETTINGS)
    .fetch_one(pool.0)
    .await
    .map_err(crate::SqlxAdminError::from)?;
    Ok(server_admin_contract::AdminSettingsView::new(
        server_admin_contract::AdminDefaultRoute::try_from(row.4)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        row.2
            .map(server_admin_contract::AdminMainLogo::try_from)
            .transpose()
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        row.6
            .map(server_admin_contract::AdminOrganizationContacts::try_from)
            .transpose()
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        row.5
            .map(server_admin_contract::AdminOrganizationName::try_from)
            .transpose()
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        row.3
            .map(server_admin_contract::AdminPrimaryColor::try_from)
            .transpose()
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        server_admin_contract::AdminSiteName::try_from(row.0)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        row.7
            .map(server_admin_contract::AdminSupportUrl::try_from)
            .transpose()
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        row.1
            .map(server_admin_contract::AdminTabTitle::try_from)
            .transpose()
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
    ))
}

pub(crate) async fn update_settings(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    request: server_admin_contract::AdminUpdateSettingsReq,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    let (
        default_admin_route,
        main_logo,
        organization_contacts,
        organization_name,
        primary_color,
        site_name,
        support_url,
        tab_title,
    ) = request.into_parts();
    sqlx::query_scalar::<_, bool>(UPDATE_SETTINGS)
        .bind(site_name.as_ref().map(AsRef::<str>::as_ref))
        .bind(tab_title.as_ref().map(AsRef::<str>::as_ref))
        .bind(main_logo.as_ref().map(AsRef::<str>::as_ref))
        .bind(primary_color.as_ref().map(AsRef::<str>::as_ref))
        .bind(default_admin_route.as_ref().map(AsRef::<str>::as_ref))
        .bind(organization_name.as_ref().map(AsRef::<str>::as_ref))
        .bind(organization_contacts.as_ref().map(AsRef::<str>::as_ref))
        .bind(support_url.as_ref().map(AsRef::<str>::as_ref))
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
}
