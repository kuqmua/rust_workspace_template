#[allow(
    clippy::future_not_send,
    reason = "browser mutation futures remain on the browser thread"
)]
pub(crate) async fn send_admin_request<RequestBody>(
    admin_mutation_method: crate::admin_mutation_method::AdminMutationMethod,
    admin_csr_api_url: &crate::admin_csr_api_url::AdminCsrApiUrl,
    request_body: &RequestBody,
) -> Result<(), crate::admin_table_load_error::AdminTableLoadError>
where
    RequestBody: serde::Serialize,
{
    let csrf_token = || {
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
            .find_map(|cookie| cookie.strip_prefix(constants_str::ADMIN_CSRF_TOKEN_ALT))
            .map(str::to_owned)
            .map(crate::admin_csrf_token::AdminCsrfToken::try_from)
            .transpose()
            .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?
            .ok_or(crate::admin_table_load_error::AdminTableLoadError::MissingCsrf)
    };

    let body = serde_json::to_string(request_body)
        .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?;
    let options = web_sys::RequestInit::new();
    options.set_method(admin_mutation_method.get());
    options.set_body(&wasm_bindgen::JsValue::from_str(&body));
    let request = web_sys::Request::new_with_str_and_init(admin_csr_api_url.as_ref(), &options)
        .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
    request
        .headers()
        .set(
            constants_str::CONTENT_TYPE,
            constants_str::HTTP_APPLICATION_JSON,
        )
        .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
    if admin_csr_api_url.as_ref()
        != server_admin_contract::admin_route::AdminRoute::Refresh
            .path()
            .as_ref()
    {
        request
            .headers()
            .set(constants_str::X_CSRF_TOKEN, csrf_token()?.as_ref())
            .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
    }
    let response_value = wasm_bindgen_futures::JsFuture::from(
        web_sys::window()
            .ok_or(crate::admin_table_load_error::AdminTableLoadError::Fetch)?
            .fetch_with_request(&request),
    )
    .await
    .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
    let response = wasm_bindgen::JsCast::dyn_into::<web_sys::Response>(response_value)
        .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Response)?;
    response.ok().then_some(()).ok_or_else(|| {
        crate::admin_table_load_error::AdminTableLoadError::Http(
            crate::admin_http_status::AdminHttpStatus::from(response.status()),
            admin_csr_api_url.clone(),
        )
    })
}
