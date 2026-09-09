#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures run exclusively on wasm_bindgen_futures::spawn_local"
)]
pub(crate) async fn fetch_json_request<Response>(
    admin_csr_api_url: &super::admin_csr_api_url::AdminCsrApiUrl,
    transport_request: Option<&frontend_contract::transport_request::TransportRequest>,
) -> Result<Response, crate::admin_table_load_error::AdminTableLoadError>
where
    Response: serde::de::DeserializeOwned,
{
    let read_json = async || {
        let window =
            web_sys::window().ok_or(crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
        let promise = if let Some(transport_request) = transport_request {
            let options = web_sys::RequestInit::new();
            options.set_method(transport_request.route().method().as_str().as_ref());
            let body = std::str::from_utf8(transport_request.body().as_ref())
                .map_err(crate::std_str_utf8_error::StdStrUtf8Error::from)?;
            options.set_body(&wasm_bindgen::JsValue::from_str(body));
            let request =
                web_sys::Request::new_with_str_and_init(admin_csr_api_url.as_ref(), &options)
                    .map_err(
                        crate::wasm_bindgen_admin_read_error::WasmBindgenAdminReadError::from,
                    )?;
            request
                .headers()
                .set(
                    constants_str::CONTENT_TYPE,
                    constants_str::HTTP_APPLICATION_JSON,
                )
                .map_err(crate::wasm_bindgen_admin_read_error::WasmBindgenAdminReadError::from)?;
            window.fetch_with_request(&request)
        } else {
            window.fetch_with_str(admin_csr_api_url.as_ref())
        };
        let response_value = wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .map_err(crate::wasm_bindgen_admin_read_error::WasmBindgenAdminReadError::from)?;
        let response = wasm_bindgen::JsCast::dyn_into::<web_sys::Response>(response_value)
            .map_err(crate::wasm_bindgen_admin_read_error::WasmBindgenAdminReadError::from)?;
        if !response.ok() {
            return Err(crate::admin_table_load_error::AdminTableLoadError::Http(
                crate::admin_http_status::AdminHttpStatus::from(response.status()),
                admin_csr_api_url.clone(),
            ));
        }
        let text_promise = web_sys::Response::text(&response)
            .map_err(crate::wasm_bindgen_admin_read_error::WasmBindgenAdminReadError::from)?;
        let text_value = wasm_bindgen_futures::JsFuture::from(text_promise)
            .await
            .map_err(crate::wasm_bindgen_admin_read_error::WasmBindgenAdminReadError::from)?;
        let text = text_value
            .as_string()
            .ok_or(crate::admin_table_load_error::AdminTableLoadError::Response)?;
        Ok(serde_json::from_str(&text)
            .map_err(crate::std_rc_serde_json_error::StdRcSerdeJsonError::from)?)
    };
    crate::with_admin_session::with_admin_session(read_json).await
}
