#[track_caller]
pub(crate) fn replace_header_name<'headers_lt>(
    headers: impl Into<crate::axum_test_headers_mut_ref::AxumTestHeadersMutRef<'headers_lt>>,
    from_name: impl axum::http::header::AsHeaderName,
    to_name: impl axum::http::header::IntoHeaderName,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
) {
    let headers = headers.into();
    let value = headers.0.remove(from_name).unwrap_or_else(|| {
        let exp_id = exp_id.into();
        panic!(
            "{} missing source header while replacing, id={exp_id}",
            constants_str::catalog::ROUTE_VALIDATORS_REPLACE_HEADER_MISSING_SRC_ER_ID
        );
    });
    crate::insert_header_no_prev::insert_header_no_prev(headers.0, to_name, value);
}
