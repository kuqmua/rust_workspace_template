#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::Display,
)]
pub struct RequestId(String);

impl TryFrom<String> for RequestId {
    type Error = super::RequestIdTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > constants_usize::VALUE_128 || !value.is_ascii() {
            Err(super::RequestIdTryFromStringError)
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<&http::HeaderValue> for RequestId {
    type Error = super::RequestIdTryFromHttpHeaderValueError;

    fn try_from(value: &http::HeaderValue) -> Result<Self, Self::Error> {
        let value_text = value.to_str().map_err(|error| {
            super::RequestIdTryFromHttpHeaderValueError::ToStr(super::HttpHeaderToStrError(error))
        })?;
        Self::try_from(value_text.to_owned())
            .map_err(super::RequestIdTryFromHttpHeaderValueError::Invalid)
    }
}

impl TryFrom<&RequestId> for http::HeaderValue {
    type Error = http::header::InvalidHeaderValue;

    fn try_from(value: &RequestId) -> Result<Self, Self::Error> {
        Self::from_str(value.0.as_str())
    }
}
