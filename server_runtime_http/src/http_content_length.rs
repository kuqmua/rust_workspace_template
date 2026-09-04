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

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::TWENTY {
            return Err(Self::Error::TooLong);
        }
        if value.is_empty() {
            return Err(Self::Error::Empty);
        }
        if !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(Self::Error::InvalidSymbol);
        }
        let _parsed = value
            .parse::<u64>()
            .map_err(|_error| Self::Error::OutOfRange)?;
        Ok(Self(value))
    }
}

impl TryFrom<HttpContentLength> for u64 {
    type Error = crate::http_content_length_error::HttpContentLengthError;

    fn try_from(value: HttpContentLength) -> Result<Self, Self::Error> {
        value
            .0
            .parse::<Self>()
            .map_err(|_error| Self::Error::OutOfRange)
    }
}
