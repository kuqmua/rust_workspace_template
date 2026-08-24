#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::AsRefStr, newtype::BoundedString,
)]
#[bounded_string(max = 8_192usize, chars)]
struct AdminCsrfToken(String);

impl std::fmt::Debug for AdminCsrfToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::REDACTED_ALT_3)
    }
}

fn csrf_token() -> Result<AdminCsrfToken, crate::app::state::AdminTableLoadError> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or(crate::app::state::AdminTableLoadError::Fetch)?;
    let document = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlDocument>(document)
        .map_err(|_error| crate::app::state::AdminTableLoadError::Fetch)?;
    document
        .cookie()
        .map_err(|_error| crate::app::state::AdminTableLoadError::Fetch)?
        .split(';')
        .map(str::trim)
        .find_map(|cookie| cookie.strip_prefix(str_constants::ADMIN_CSRF_TOKEN_ALT))
        .map(str::to_owned)
        .map(AdminCsrfToken::try_from)
        .transpose()
        .map_err(|_error| crate::app::state::AdminTableLoadError::Query)?
        .ok_or(crate::app::state::AdminTableLoadError::Fetch)
}

#[allow(
    clippy::future_not_send,
    reason = "browser mutation requests run exclusively on wasm_bindgen_futures::spawn_local"
)]
pub(in crate::app) async fn send_json<RequestBody>(
    method: crate::app::mutation::AdminMutationMethod,
    path: &super::url::AdminCsrApiUrl,
    request_body: &RequestBody,
) -> Result<(), crate::app::state::AdminTableLoadError>
where
    RequestBody: serde::Serialize,
{
    let body = serde_json::to_string(request_body)
        .map_err(|_error| crate::app::state::AdminTableLoadError::Query)?;
    let options = web_sys::RequestInit::new();
    options.set_method(method.get());
    options.set_body(&wasm_bindgen::JsValue::from_str(&body));
    let request = web_sys::Request::new_with_str_and_init(path.as_ref(), &options)
        .map_err(|_error| crate::app::state::AdminTableLoadError::Fetch)?;
    request
        .headers()
        .set(
            str_constants::CONTENT_TYPE,
            str_constants::HTTP_APPLICATION_JSON,
        )
        .map_err(|_error| crate::app::state::AdminTableLoadError::Fetch)?;
    request
        .headers()
        .set(str_constants::X_CSRF_TOKEN, csrf_token()?.as_ref())
        .map_err(|_error| crate::app::state::AdminTableLoadError::Fetch)?;
    let response_value = wasm_bindgen_futures::JsFuture::from(
        web_sys::window()
            .ok_or(crate::app::state::AdminTableLoadError::Fetch)?
            .fetch_with_request(&request),
    )
    .await
    .map_err(|_error| crate::app::state::AdminTableLoadError::Fetch)?;
    let response = wasm_bindgen::JsCast::dyn_into::<web_sys::Response>(response_value)
        .map_err(|_error| crate::app::state::AdminTableLoadError::Response)?;
    response.ok().then_some(()).ok_or_else(|| {
        crate::app::state::AdminTableLoadError::Http(
            super::url::AdminHttpStatus::from(response.status()),
            path.clone(),
        )
    })
}
