pub(crate) fn reload_after<RequestBody>(
    method: crate::admin_mutation_method::AdminMutationMethod,
    path: crate::admin_csr_api_url::AdminCsrApiUrl,
    request_body: RequestBody,
) where
    RequestBody: serde::Serialize + 'static,
{
    wasm_bindgen_futures::spawn_local(async move {
        match crate::domain_types::start::http::mutation::send_json(method, &path, &request_body)
            .await
        {
            Ok(()) => match web_sys::window() {
                Some(window) if window.location().reload().is_ok() => {}
                Some(_) | None => {
                    crate::domain_types::start::mutation::show_mutation_error::show_mutation_error(
                        &crate::admin_table_load_error::AdminTableLoadError::Fetch,
                    );
                }
            },
            Err(error) => {
                crate::domain_types::start::mutation::show_mutation_error::show_mutation_error(
                    &error,
                );
            }
        }
    });
}
