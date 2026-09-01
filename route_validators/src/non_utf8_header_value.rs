pub(crate) fn non_utf8_header_value() -> crate::axum_test_header_value::AxumTestHeaderValue {
    crate::axum_test_header_value::AxumTestHeaderValue::from(
        axum::http::HeaderValue::from_bytes(&[0x80]).expect(constants_str::DIAGNOSTIC_86EB20CF),
    )
}
