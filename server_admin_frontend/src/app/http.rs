#[derive(Clone, Copy, Debug, newtype::Display, newtype::FromInner)]
pub(in crate::app) struct AdminHttpStatus(u16);

#[derive(Clone, Debug, newtype::AsRefStr, newtype::BoundedString, newtype::Display)]
#[bounded_string(max = 16_384usize, chars)]
pub(in crate::app) struct AdminCsrApiUrl(String);

#[derive(Clone, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = 8_192usize, chars)]
struct AdminCsrfToken(String);

impl std::fmt::Debug for AdminCsrfToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::REDACTED_ALT_3)
    }
}

#[derive(Clone, Copy, Debug, newtype::AsRefStr, newtype::FromInner)]
pub(in crate::app) struct AdminCsrApiUrlSuffixRef<'suffix_lt>(&'suffix_lt str);

pub(in crate::app) fn admin_api_url(
    route: server_admin_contract::AdminRoute,
) -> Result<AdminCsrApiUrl, super::state::AdminTableLoadError> {
    admin_route_path_url(route.path())
}

pub(in crate::app) fn admin_route_path_url(
    path: server_admin_contract::AdminRoutePath,
) -> Result<AdminCsrApiUrl, super::state::AdminTableLoadError> {
    AdminCsrApiUrl::try_from(path.to_string())
        .map_err(|_error| super::state::AdminTableLoadError::Query)
}

pub(in crate::app) fn admin_api_url_with_suffix(
    route: server_admin_contract::AdminRoute,
    suffix: AdminCsrApiUrlSuffixRef<'_>,
) -> Result<AdminCsrApiUrl, super::state::AdminTableLoadError> {
    AdminCsrApiUrl::try_from(format!("{}{}", route.path(), suffix.as_ref()))
        .map_err(|_error| super::state::AdminTableLoadError::Query)
}

#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures run exclusively on wasm_bindgen_futures::spawn_local"
)]
pub(in crate::app) async fn fetch_json<Response>(
    url: &AdminCsrApiUrl,
) -> Result<Response, super::state::AdminTableLoadError>
where
    Response: serde::de::DeserializeOwned,
{
    let window = web_sys::window().ok_or(super::state::AdminTableLoadError::Fetch)?;
    let response_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(url.as_ref()))
        .await
        .map_err(|_error| super::state::AdminTableLoadError::Fetch)?;
    let response = wasm_bindgen::JsCast::dyn_into::<web_sys::Response>(response_value)
        .map_err(|_error| super::state::AdminTableLoadError::Response)?;
    if !response.ok() {
        return Err(super::state::AdminTableLoadError::Http(
            AdminHttpStatus::from(response.status()),
            url.clone(),
        ));
    }
    let text_promise = web_sys::Response::text(&response)
        .map_err(|_error| super::state::AdminTableLoadError::Response)?;
    let text_value = wasm_bindgen_futures::JsFuture::from(text_promise)
        .await
        .map_err(|_error| super::state::AdminTableLoadError::Response)?;
    let text = text_value
        .as_string()
        .ok_or(super::state::AdminTableLoadError::Response)?;
    serde_json::from_str(&text).map_err(|_error| super::state::AdminTableLoadError::Response)
}

fn csrf_token() -> Result<AdminCsrfToken, super::state::AdminTableLoadError> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or(super::state::AdminTableLoadError::Fetch)?;
    let document = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlDocument>(document)
        .map_err(|_error| super::state::AdminTableLoadError::Fetch)?;
    document
        .cookie()
        .map_err(|_error| super::state::AdminTableLoadError::Fetch)?
        .split(';')
        .map(str::trim)
        .find_map(|cookie| cookie.strip_prefix(str_constants::ADMIN_CSRF_TOKEN_ALT))
        .map(str::to_owned)
        .map(AdminCsrfToken::try_from)
        .transpose()
        .map_err(|_error| super::state::AdminTableLoadError::Query)?
        .ok_or(super::state::AdminTableLoadError::Fetch)
}

#[allow(
    clippy::future_not_send,
    reason = "browser mutation requests run exclusively on wasm_bindgen_futures::spawn_local"
)]
pub(in crate::app) async fn send_json<RequestBody>(
    method: super::mutation::AdminMutationMethod,
    path: &AdminCsrApiUrl,
    request_body: &RequestBody,
) -> Result<(), super::state::AdminTableLoadError>
where
    RequestBody: serde::Serialize,
{
    let body = serde_json::to_string(request_body)
        .map_err(|_error| super::state::AdminTableLoadError::Query)?;
    let options = web_sys::RequestInit::new();
    options.set_method(method.get());
    options.set_body(&wasm_bindgen::JsValue::from_str(&body));
    let request = web_sys::Request::new_with_str_and_init(path.as_ref(), &options)
        .map_err(|_error| super::state::AdminTableLoadError::Fetch)?;
    request
        .headers()
        .set(
            str_constants::CONTENT_TYPE,
            str_constants::HTTP_APPLICATION_JSON,
        )
        .map_err(|_error| super::state::AdminTableLoadError::Fetch)?;
    request
        .headers()
        .set(str_constants::X_CSRF_TOKEN, csrf_token()?.as_ref())
        .map_err(|_error| super::state::AdminTableLoadError::Fetch)?;
    let response_value = wasm_bindgen_futures::JsFuture::from(
        web_sys::window()
            .ok_or(super::state::AdminTableLoadError::Fetch)?
            .fetch_with_request(&request),
    )
    .await
    .map_err(|_error| super::state::AdminTableLoadError::Fetch)?;
    let response = wasm_bindgen::JsCast::dyn_into::<web_sys::Response>(response_value)
        .map_err(|_error| super::state::AdminTableLoadError::Response)?;
    response.ok().then_some(()).ok_or_else(|| {
        super::state::AdminTableLoadError::Http(
            AdminHttpStatus::from(response.status()),
            path.clone(),
        )
    })
}
