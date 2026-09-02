pub(crate) fn reload_after<RequestBody>(
    admin_mutation_method: crate::admin_mutation_method::AdminMutationMethod,
    admin_csr_api_url: crate::admin_csr_api_url::AdminCsrApiUrl,
    request_body: RequestBody,
) where
    RequestBody: serde::Serialize + 'static,
{
    wasm_bindgen_futures::spawn_local(async move {
        match crate::send_json::send_json(admin_mutation_method, &admin_csr_api_url, &request_body)
            .await
        {
            Ok(()) => match web_sys::window() {
                Some(window) if window.location().reload().is_ok() => {}
                Some(_) | None => {
                    crate::show_mutation_error::show_mutation_error(
                        &crate::admin_table_load_error::AdminTableLoadError::Fetch,
                    );
                }
            },
            Err(error) => {
                crate::show_mutation_error::show_mutation_error(&error);
            }
        }
    });
}
