#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestId(String);
impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestIdTryFromStringError;
impl std::fmt::Display for RequestIdTryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::REQUEST_ID_MUST_BE_NON_EMPTY_ASCII_UP_TO_128_BYTES)
    }
}
impl std::error::Error for RequestIdTryFromStringError {}
#[derive(Debug)]
pub struct HttpHeaderToStrError(http::header::ToStrError);
impl From<http::header::ToStrError> for HttpHeaderToStrError {
    fn from(value: http::header::ToStrError) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for HttpHeaderToStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for HttpHeaderToStrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug)]
pub enum RequestIdTryFromHttpHeaderValueError {
    Invalid(RequestIdTryFromStringError),
    ToStr(HttpHeaderToStrError),
}
impl std::fmt::Display for RequestIdTryFromHttpHeaderValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(error) => error.fmt(f),
            Self::ToStr(error) => write!(f, "request id is not a text header: {error}"),
        }
    }
}
impl std::error::Error for RequestIdTryFromHttpHeaderValueError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Invalid(error) => Some(error),
            Self::ToStr(error) => Some(error),
        }
    }
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
