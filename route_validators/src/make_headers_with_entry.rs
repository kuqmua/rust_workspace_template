pub(crate) fn make_headers_with_entry<ValueTy>(
    name: impl axum::http::header::IntoHeaderName,
    value_ty: ValueTy,
) -> crate::axum_test_headers::AxumTestHeaders
where
    ValueTy: Into<crate::axum_test_header_value::AxumTestHeaderValue>,
{
    let mut headers = axum::http::HeaderMap::new();
    crate::insert_header_no_prev::insert_header_no_prev(&mut headers, name, value_ty);
    crate::axum_test_headers::AxumTestHeaders::from(headers)
}
