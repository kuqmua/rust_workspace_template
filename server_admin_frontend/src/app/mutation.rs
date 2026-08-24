#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(in crate::app) enum AdminMutationMethod {
    Delete,
    Patch,
    Post,
}

impl AdminMutationMethod {
    pub(in crate::app) const fn get(self) -> &'static str {
        match self {
            Self::Delete => str_constants::DELETE,
            Self::Patch => str_constants::PATCH,
            Self::Post => str_constants::POST,
        }
    }
}

pub(in crate::app) fn reload_after<RequestBody>(
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
    let Some(root) = document.get_element_by_id(str_constants::ADMIN_CSR_ROOT_ID) else {
        return;
    };
    let Ok(alert) = document.create_element("p") else {
        return;
    };
    if alert
        .set_attribute("role", str_constants::HTML_ALERT_ROLE)
        .is_err()
    {
        return;
    }
    if alert
        .set_attribute(
            str_constants::HTML_DATA_NAME,
            str_constants::ADMIN_ALERT_DATA_NAME,
        )
        .is_err()
    {
        return;
    }
    alert.set_text_content(Some(&error.to_string()));
    alert.set_class_name(str_constants::ADMIN_FIELD_ERROR_CLASS);
    if root.append_child(&alert).is_err() {
        root.set_text_content(Some(&error.to_string()));
        root.set_class_name(str_constants::ADMIN_FIELD_ERROR_CLASS);
    }
}
