#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct HttpContentLength(String);

impl TryFrom<String> for HttpContentLength {
    type Error = crate::http_content_length_error::HttpContentLengthError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.len() > constants_usize::TWENTY {
            return Err(Self::Error::TooLong);
        }
        if string.is_empty() {
            return Err(Self::Error::Empty);
        }
        if !string.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(Self::Error::InvalidSymbol);
        }
        let _parsed = string
            .parse::<u64>()
            .map_err(|_error| Self::Error::OutOfRange)?;
        Ok(Self(string))
    }
}

impl TryFrom<HttpContentLength> for u64 {
    type Error = crate::http_content_length_error::HttpContentLengthError;

    fn try_from(http_content_length: HttpContentLength) -> Result<Self, Self::Error> {
        http_content_length
            .0
            .parse::<Self>()
            .map_err(|_error| Self::Error::OutOfRange)
    }
}
