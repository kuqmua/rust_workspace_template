#[derive(Clone, Copy, Debug)]
pub(in crate::app) enum AdminMutationMethod {
    Delete,
    Patch,
    Post,
    Put,
}

impl AdminMutationMethod {
    pub(in crate::app) const fn get(self) -> &'static str {
        match self {
            Self::Delete => str_constants::DELETE,
            Self::Patch => str_constants::PATCH,
            Self::Post => str_constants::POST,
            Self::Put => str_constants::HTTP_METHOD_PUT_LABEL,
        }
    }
}

#[derive(Clone, Copy, Debug, newtype::AsRefStr, newtype::FromInner)]
pub(in crate::app) struct MutationConfirmationMessageRef<'message_lt>(&'message_lt str);

#[derive(Clone, Copy, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub(in crate::app) struct MutationConfirmed(bool);

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
    root.set_text_content(Some(&error.to_string()));
    root.set_class_name(str_constants::ADMIN_FIELD_ERROR_CLASS);
}

pub(in crate::app) fn mutation_confirmed(
    message: MutationConfirmationMessageRef<'_>,
) -> MutationConfirmed {
    if let Some(Ok(confirmed)) =
        web_sys::window().map(|window| window.confirm_with_message(message.as_ref()))
    {
        MutationConfirmed::from(confirmed)
    } else {
        show_mutation_error(&super::state::AdminTableLoadError::Fetch);
        MutationConfirmed::from(false)
    }
}
