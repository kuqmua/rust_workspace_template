pub(super) fn required_header_value<E>(
    headers: super::AxumHeadersRef<'_>,
    header_name: impl axum::http::header::AsHeaderName,
    no_header_error: impl FnOnce() -> E,
) -> Result<super::AxumHeaderValueRef<'_>, E> {
    headers
        .0
        .get(header_name)
        .map(super::AxumHeaderValueRef::from)
        .ok_or_else(no_header_error)
}
