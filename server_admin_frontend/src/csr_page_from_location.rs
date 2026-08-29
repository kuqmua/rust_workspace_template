pub(crate) fn csr_page_from_location() -> Result<
    server_admin_contract::admin_page::AdminPage,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    let pathname = web_sys::window()
        .ok_or(crate::admin_table_load_error::AdminTableLoadError::Fetch)?
        .location()
        .pathname()
        .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
    let path =
        server_admin_contract::admin_page_path_ref::AdminPagePathRef::from(pathname.as_str());
    let page = match server_admin_contract::admin_page::AdminPage::from_path(path) {
        Some(page) => page,
        None if server_admin_contract::admin_data_table::AdminDataTable::from_frontend_path(
            path,
        )
        .is_some() =>
        {
            server_admin_contract::admin_page::AdminPage::Tables
        }
        None => return Err(crate::admin_table_load_error::AdminTableLoadError::Query),
    };
    if bool::from(page.supports_csr()) {
        Ok(page)
    } else {
        Err(crate::admin_table_load_error::AdminTableLoadError::Query)
    }
}
