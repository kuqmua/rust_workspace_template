#[allow(
    clippy::future_not_send,
    reason = "browser mutation requests run exclusively on wasm_bindgen_futures::spawn_local"
)]
pub(crate) async fn send_json<RequestBody>(
    admin_mutation_method: crate::admin_mutation_method::AdminMutationMethod,
    admin_csr_api_url: &crate::admin_csr_api_url::AdminCsrApiUrl,
    request_body: &RequestBody,
) -> Result<(), crate::admin_table_load_error::AdminTableLoadError>
where
    RequestBody: serde::Serialize,
{
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
    request
        .headers()
        .set(
            constants_str::X_CSRF_TOKEN,
            crate::domain_types::start::http::mutation::csrf_token::csrf_token()?.as_ref(),
        )
        .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
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
