#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(in crate::domain_types::app) enum AdminMutationMethod {
    Delete,
    Patch,
    Post,
}

impl AdminMutationMethod {
    pub(in crate::domain_types::app) const fn get(self) -> &'static str {
        match self {
            Self::Delete => constants_str::DELETE,
            Self::Patch => constants_str::PATCH,
            Self::Post => constants_str::POST,
        }
    }
}

pub(in crate::domain_types::app) fn reload_after<RequestBody>(
    method: AdminMutationMethod,
    path: super::http::url::AdminCsrApiUrl,
    request_body: RequestBody,
) where
    RequestBody: serde::Serialize + 'static,
{
    wasm_bindgen_futures::spawn_local(async move {
        match super::http::mutation::send_json(method, &path, &request_body).await {
            Ok(()) => match web_sys::window() {
                Some(window) if window.location().reload().is_ok() => {}
                Some(_) | None => {
                    show_mutation_error(&super::state::AdminTableLoadError::Fetch);
                }
            },
            Err(error) => show_mutation_error(&error),
        }
    });
}

fn show_mutation_error(error: &super::state::AdminTableLoadError) {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let Some(root) = document.get_element_by_id(constants_str::ADMIN_CSR_ROOT_ID) else {
        return;
    };
    let Ok(alert) = document.create_element(constants_str::VALUE_148DE9C5) else {
        return;
    };
    if alert
        .set_attribute(constants_str::ROLE, constants_str::HTML_ALERT_ROLE)
        .is_err()
    {
        return;
    }
    if alert
        .set_attribute(
            constants_str::HTML_DATA_NAME,
            constants_str::ADMIN_ALERT_DATA_NAME,
        )
        .is_err()
    {
        return;
    }
    alert.set_text_content(Some(&error.to_string()));
    alert.set_class_name(constants_str::ADMIN_FIELD_ERROR_CLASS);
    if root.append_child(&alert).is_err() {
        root.set_text_content(Some(&error.to_string()));
        root.set_class_name(constants_str::ADMIN_FIELD_ERROR_CLASS);
    }
}
