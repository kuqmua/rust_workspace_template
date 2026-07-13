#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestId(String);
impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl TryFrom<String> for RequestId {
    type Error = RequestIdTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 128usize || !value.is_ascii() {
            Err(RequestIdTryFromStringEr)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestIdTryFromStringEr;
impl std::fmt::Display for RequestIdTryFromStringEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request id must be non-empty ASCII up to 128 bytes")
    }
}
impl std::error::Error for RequestIdTryFromStringEr {}
#[derive(Debug)]
pub struct HttpHeaderToStrEr(http::header::ToStrError);
impl std::fmt::Display for HttpHeaderToStrEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for HttpHeaderToStrEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug)]
pub enum RequestIdTryFromHttpHeaderValueEr {
    Invalid(RequestIdTryFromStringEr),
    ToStr(HttpHeaderToStrEr),
}
impl std::fmt::Display for RequestIdTryFromHttpHeaderValueEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(error) => error.fmt(f),
            Self::ToStr(error) => write!(f, "request id is not a text header: {error}"),
        }
    }
}
impl std::error::Error for RequestIdTryFromHttpHeaderValueEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Invalid(error) => Some(error),
            Self::ToStr(error) => Some(error),
        }
    }
}
impl TryFrom<&http::HeaderValue> for RequestId {
    type Error = RequestIdTryFromHttpHeaderValueEr;
    fn try_from(value: &http::HeaderValue) -> Result<Self, Self::Error> {
        let value_text = value
            .to_str()
            .map_err(|error| RequestIdTryFromHttpHeaderValueEr::ToStr(HttpHeaderToStrEr(error)))?;
        Self::try_from(value_text.to_owned()).map_err(RequestIdTryFromHttpHeaderValueEr::Invalid)
    }
}
impl TryFrom<&RequestId> for http::HeaderValue {
    type Error = http::header::InvalidHeaderValue;
    fn try_from(value: &RequestId) -> Result<Self, Self::Error> {
        Self::from_str(value.0.as_str())
    }
}
