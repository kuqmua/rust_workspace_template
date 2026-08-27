use super::AxumTestHeaderValue;

pub(crate) fn non_utf8_header_value() -> AxumTestHeaderValue {
    AxumTestHeaderValue::from(
        axum::http::HeaderValue::from_bytes(&[0x80])
            .expect("86eb20cf non_utf8_header_value invariant must hold"),
    )
}
