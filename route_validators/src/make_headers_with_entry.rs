use super::{AxumTestHeaderValue, AxumTestHeaders, insert_header_no_prev};

pub(crate) fn make_headers_with_entry<ValueTy>(
    name: impl axum::http::header::IntoHeaderName,
    value: ValueTy,
) -> AxumTestHeaders
where
    ValueTy: Into<AxumTestHeaderValue>,
{
    let mut headers = axum::http::HeaderMap::new();
    insert_header_no_prev(&mut headers, name, value);
    AxumTestHeaders::from(headers)
}
