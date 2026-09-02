#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::Display,
)]
pub struct RequestId(String);

impl TryFrom<String> for RequestId {
    type Error = crate::request_id_try_from_string_error::RequestIdTryFromStringError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.is_empty() || string.len() > constants_usize::VALUE_128 || !string.is_ascii() {
            Err(crate::request_id_try_from_string_error::RequestIdTryFromStringError::Invalid)
        } else {
            Ok(Self(string))
        }
    }
}

impl TryFrom<&http::HeaderValue> for RequestId {
    type Error =
        crate::request_id_try_from_http_header_value_error::RequestIdTryFromHttpHeaderValueError;

    fn try_from(header_value: &http::HeaderValue) -> Result<Self, Self::Error> {
        let value_text = header_value.to_str().map_err(|error| {
            crate::request_id_try_from_http_header_value_error::RequestIdTryFromHttpHeaderValueError::ToStr(crate::http_header_to_str_error::HttpHeaderToStrError::from(error))
        })?;
        Self::try_from(value_text.to_owned())
            .map_err(crate::request_id_try_from_http_header_value_error::RequestIdTryFromHttpHeaderValueError::Invalid)
    }
}

impl TryFrom<&RequestId> for http::HeaderValue {
    type Error = http::header::InvalidHeaderValue;

    fn try_from(request_id: &RequestId) -> Result<Self, Self::Error> {
        Self::from_str(request_id.0.as_str())
    }
}
