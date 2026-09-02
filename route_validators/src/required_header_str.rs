pub(crate) fn required_header_str<E>(
    axum_headers_ref: crate::axum_headers_ref::AxumHeadersRef<'_>,
    header_name: impl axum::http::header::AsHeaderName,
    no_header_error: impl FnOnce() -> E,
    to_str_error: impl FnOnce(axum::http::header::ToStrError) -> E,
) -> Result<crate::header_str_ref::HeaderStrRef<'_>, E> {
    crate::required_header_str_parsed::required_header_str_parsed(
        axum_headers_ref,
        header_name,
        no_header_error,
        to_str_error,
        Ok,
    )
}
