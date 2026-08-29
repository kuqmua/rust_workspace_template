#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::Display,
)]
pub struct RequestId(String);

impl TryFrom<String> for RequestId {
    type Error = crate::request_id_try_from_string_error::RequestIdTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > constants_usize::VALUE_128 || !value.is_ascii() {
            Err(crate::request_id_try_from_string_error::RequestIdTryFromStringError)
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<&http::HeaderValue> for RequestId {
    type Error =
        crate::request_id_try_from_http_header_value_error::RequestIdTryFromHttpHeaderValueError;

    fn try_from(value: &http::HeaderValue) -> Result<Self, Self::Error> {
        let value_text = value.to_str().map_err(|error| {
            crate::request_id_try_from_http_header_value_error::RequestIdTryFromHttpHeaderValueError::ToStr(crate::http_header_to_str_error::HttpHeaderToStrError(error))
        })?;
        Self::try_from(value_text.to_owned())
            .map_err(crate::request_id_try_from_http_header_value_error::RequestIdTryFromHttpHeaderValueError::Invalid)
    }
}

impl TryFrom<&RequestId> for http::HeaderValue {
    type Error = http::header::InvalidHeaderValue;

    fn try_from(value: &RequestId) -> Result<Self, Self::Error> {
        Self::from_str(value.0.as_str())
    }
}
