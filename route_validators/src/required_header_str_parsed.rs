pub(crate) fn required_header_str_parsed<'headers, E, T>(
    headers: crate::axum_headers_ref::AxumHeadersRef<'headers>,
    header_name: impl axum::http::header::AsHeaderName,
    no_header_error: impl FnOnce() -> E,
    to_str_error: impl FnOnce(axum::http::header::ToStrError) -> E,
    parse: impl FnOnce(crate::header_str_ref::HeaderStrRef<'headers>) -> Result<T, E>,
) -> Result<T, E> {
    let header_value =
        crate::required_header_value::required_header_value(headers, header_name, no_header_error)?;
    let header_str = <&axum::http::HeaderValue>::from(header_value)
        .to_str()
        .map(crate::header_str_ref::HeaderStrRef::from)
        .map_err(to_str_error)?;
    parse(header_str)
}
