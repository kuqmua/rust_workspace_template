pub(super) fn required_header_value<E>(
    axum_headers_ref: crate::axum_headers_ref::AxumHeadersRef<'_>,
    header_name: impl axum::http::header::AsHeaderName,
    no_header_error: impl FnOnce() -> E,
) -> Result<crate::axum_header_value_ref::AxumHeaderValueRef<'_>, E> {
    axum_headers_ref
        .header(header_name)
        .map(crate::axum_header_value_ref::AxumHeaderValueRef::from)
        .ok_or_else(no_header_error)
}
