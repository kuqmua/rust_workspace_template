#[cfg(test)]
pub(super) fn make_no_route_message(
    uri: crate::axum_http_uri_ref::AxumHttpUriRef<'_>,
) -> to_err_string::error_text::ErrorText {
    crate::make_no_route_message_for_suffix::make_no_route_message_for_suffix(
        crate::uri_suffix::uri_suffix(uri),
    )
}
