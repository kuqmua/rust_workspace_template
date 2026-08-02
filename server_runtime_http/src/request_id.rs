#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::Display)]
pub struct RequestId(String);

impl TryFrom<String> for RequestId {
    type Error = RequestIdTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 128usize || !value.is_ascii() {
            Err(RequestIdTryFromStringError)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error(
    "{}",
    str_constants::REQUEST_ID_MUST_BE_NON_EMPTY_ASCII_UP_TO_128_BYTES
)]
pub struct RequestIdTryFromStringError;
#[derive(optml::Optml, Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
pub struct HttpHeaderToStrError(http::header::ToStrError);
#[derive(optml::Optml, Debug, thiserror::Error)]
pub enum RequestIdTryFromHttpHeaderValueError {
    #[error(transparent)]
    Invalid(RequestIdTryFromStringError),
    #[error("request id is not a text header: {0}")]
    ToStr(HttpHeaderToStrError),
}
impl TryFrom<&http::HeaderValue> for RequestId {
    type Error = RequestIdTryFromHttpHeaderValueError;
    fn try_from(value: &http::HeaderValue) -> Result<Self, Self::Error> {
        let value_text = value.to_str().map_err(|error| {
            RequestIdTryFromHttpHeaderValueError::ToStr(HttpHeaderToStrError(error))
        })?;
        Self::try_from(value_text.to_owned()).map_err(RequestIdTryFromHttpHeaderValueError::Invalid)
    }
}
impl TryFrom<&RequestId> for http::HeaderValue {
    type Error = http::header::InvalidHeaderValue;
    fn try_from(value: &RequestId) -> Result<Self, Self::Error> {
        Self::from_str(value.0.as_str())
    }
}
