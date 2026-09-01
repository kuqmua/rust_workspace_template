#[track_caller]
pub(crate) fn replace_header_name<'headers_lt>(
    headers: impl Into<crate::axum_test_headers_mut_ref::AxumTestHeadersMutRef<'headers_lt>>,
    from_name: impl axum::http::header::AsHeaderName,
    to_name: impl axum::http::header::IntoHeaderName,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
) {
    let mut headers = headers.into();
    let value = headers.remove(from_name).unwrap_or_else(|| {
        let exp_id = exp_id.into();
        std::panic::panic_any(
            constants_str::PANIC_REPLACE_HEADER_MISSING_SOURCE
                .replacen(
                    constants_str::PANIC_POSITIONAL_PLACEHOLDER,
                    constants_str::ROUTE_VALIDATORS_REPLACE_HEADER_MISSING_SRC_ER_ID,
                    1usize,
                )
                .replacen(
                    constants_str::PANIC_PLACEHOLDER_D8C45567,
                    exp_id.to_string().as_str(),
                    1usize,
                ),
        );
    });
    crate::insert_header_no_prev::insert_header_no_prev(&mut **headers, to_name, value);
}
