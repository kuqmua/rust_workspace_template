pub(crate) fn required_header_str_parsed<'headers, E, T>(
    headers: super::AxumHeadersRef<'headers>,
    header_name: impl axum::http::header::AsHeaderName,
    no_header_error: impl FnOnce() -> E,
    to_str_error: impl FnOnce(axum::http::header::ToStrError) -> E,
    parse: impl FnOnce(super::HeaderStrRef<'headers>) -> Result<T, E>,
) -> Result<T, E> {
    let header_value =
        super::required_header_value::required_header_value(headers, header_name, no_header_error)?;
    let header_str = header_value
        .0
        .to_str()
        .map(super::HeaderStrRef::from)
        .map_err(to_str_error)?;
    parse(header_str)
}
