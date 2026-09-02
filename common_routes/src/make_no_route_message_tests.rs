#[cfg(test)]
pub(super) fn make_no_route_message(
    axum_http_uri_ref: crate::axum_http_uri_ref::AxumHttpUriRef<'_>,
) -> to_err_string::error_text::ErrorText {
    crate::make_no_route_message_for_suffix_tests::make_no_route_message_for_suffix(
        crate::uri_suffix_tests::uri_suffix(axum_http_uri_ref),
    )
}
