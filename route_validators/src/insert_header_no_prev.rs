use super::{AxumTestHeaderValue, AxumTestHeadersMutRef};

pub(super) fn insert_header_no_prev<'headers_lt, ValueTy>(
    headers: impl Into<AxumTestHeadersMutRef<'headers_lt>>,
    name: impl axum::http::header::IntoHeaderName,
    value: ValueTy,
) where
    ValueTy: Into<AxumTestHeaderValue>,
{
    let headers = headers.into();
    let prev = headers.0.insert(name, value.into().0);
    assert!(prev.is_none());
}
