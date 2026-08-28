pub(crate) fn csr_page_from_location() -> Result<
    server_admin_contract::domain_types::AdminPage,
    crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError,
> {
    let pathname = web_sys::window()
        .ok_or(
            crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError::Fetch,
        )?
        .location()
        .pathname()
        .map_err(|_error| {
            crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError::Fetch
        })?;
    let path = server_admin_contract::domain_types::AdminPagePathRef::from(pathname.as_str());
    let page = match server_admin_contract::domain_types::AdminPage::from_path(path) {
        Some(page) => page,
        None if server_admin_contract::domain_types::AdminDataTable::from_frontend_path(path)
            .is_some() =>
        {
            server_admin_contract::domain_types::AdminPage::Tables
        }
        None => return Err(
            crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError::Query,
        ),
    };
    if bool::from(page.supports_csr()) {
        Ok(page)
    } else {
        Err(crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError::Query)
    }
}
