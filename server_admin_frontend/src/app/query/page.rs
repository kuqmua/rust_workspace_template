pub(in crate::app) fn csr_page_from_location()
-> Result<server_admin_contract::AdminPage, crate::app::state::AdminTableLoadError> {
    let pathname = web_sys::window()
        .ok_or(crate::app::state::AdminTableLoadError::Fetch)?
        .location()
        .pathname()
        .map_err(|_error| crate::app::state::AdminTableLoadError::Fetch)?;
    let path = server_admin_contract::AdminPagePathRef::from(pathname.as_str());
    let page = match server_admin_contract::AdminPage::from_path(path) {
        Some(page) => page,
        None if server_admin_contract::AdminDataTable::from_frontend_path(path).is_some() => {
            server_admin_contract::AdminPage::Tables
        }
        None => return Err(crate::app::state::AdminTableLoadError::Query),
    };
    if bool::from(page.supports_csr()) {
        Ok(page)
    } else {
        Err(crate::app::state::AdminTableLoadError::Query)
    }
}
