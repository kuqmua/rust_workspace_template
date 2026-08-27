pub(in crate::domain_types::start) fn reload_after<RequestBody>(
    method: crate::domain_types::start::mutation::AdminMutationMethod,
    path: crate::domain_types::start::http::url::AdminCsrApiUrl,
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
                        &crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError::Fetch,
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
