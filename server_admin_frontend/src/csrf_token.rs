pub(in crate::domain_types::start) fn csrf_token() -> Result<
    crate::domain_types::start::http::mutation::AdminCsrfToken,
    crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError,
> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or(
            crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError::Fetch,
        )?;
    let document =
        wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlDocument>(document).map_err(|_error| {
            crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError::Fetch
        })?;
    document
        .cookie()
        .map_err(|_error| {
            crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError::Fetch
        })?
        .split(';')
        .map(str::trim)
        .find_map(|cookie| cookie.strip_prefix(constants_str::ADMIN_CSRF_TOKEN_ALT))
        .map(str::to_owned)
        .map(crate::domain_types::start::http::mutation::AdminCsrfToken::try_from)
        .transpose()
        .map_err(|_error| {
            crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError::Query
        })?
        .ok_or(
            crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError::Fetch,
        )
}
