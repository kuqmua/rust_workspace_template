#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures run exclusively on wasm_bindgen_futures::spawn_local"
)]
pub(in crate::app) async fn fetch_json<Response>(
    url: &super::url::AdminCsrApiUrl,
) -> Result<Response, crate::app::state::AdminTableLoadError>
where
    Response: serde::de::DeserializeOwned,
{
    let window = web_sys::window().ok_or(crate::app::state::AdminTableLoadError::Fetch)?;
    let response_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(url.as_ref()))
        .await
        .map_err(|_error| crate::app::state::AdminTableLoadError::Fetch)?;
    let response = wasm_bindgen::JsCast::dyn_into::<web_sys::Response>(response_value)
        .map_err(|_error| crate::app::state::AdminTableLoadError::Response)?;
    if !response.ok() {
        return Err(crate::app::state::AdminTableLoadError::Http(
            super::url::AdminHttpStatus::from(response.status()),
            url.clone(),
        ));
    }
    let text_promise = web_sys::Response::text(&response)
        .map_err(|_error| crate::app::state::AdminTableLoadError::Response)?;
    let text_value = wasm_bindgen_futures::JsFuture::from(text_promise)
        .await
        .map_err(|_error| crate::app::state::AdminTableLoadError::Response)?;
    let text = text_value
        .as_string()
        .ok_or(crate::app::state::AdminTableLoadError::Response)?;
    serde_json::from_str(&text).map_err(|_error| crate::app::state::AdminTableLoadError::Response)
}
