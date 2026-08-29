pub(crate) fn csrf_token() -> Result<
    crate::admin_csrf_token::AdminCsrfToken,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or(crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
    let document = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlDocument>(document)
        .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
    document
        .cookie()
        .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?
        .split(';')
        .map(str::trim)
        .find_map(|cookie| cookie.strip_prefix(constants_str::catalog::ADMIN_CSRF_TOKEN_ALT))
        .map(str::to_owned)
        .map(crate::admin_csrf_token::AdminCsrfToken::try_from)
        .transpose()
        .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?
        .ok_or(crate::admin_table_load_error::AdminTableLoadError::Fetch)
}
