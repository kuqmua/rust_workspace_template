pub(super) fn insert_header_no_prev<'headers_lt, ValueTy>(
    headers: impl Into<crate::axum_test_headers_mut_ref::AxumTestHeadersMutRef<'headers_lt>>,
    name: impl axum::http::header::IntoHeaderName,
    value: ValueTy,
) where
    ValueTy: Into<crate::axum_test_header_value::AxumTestHeaderValue>,
{
    let headers = headers.into();
    let prev = headers.0.insert(name, value.into().0);
    assert!(prev.is_none());
}
