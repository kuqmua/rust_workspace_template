#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Eq, PartialEq)]
pub struct HttpCookieValue(pub(super) String);

impl std::fmt::Debug for HttpCookieValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::REDACTED_ALT_3)
    }
}

impl TryFrom<String> for HttpCookieValue {
    type Error = super::HttpSecureCookieError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let valid = value.len() <= constants_usize::VALUE_8_192
            && value.bytes().all(
                |byte| matches!(byte, 0x21 | 0x23..=0x2b | 0x2d..=0x3a | 0x3c..=0x5b | 0x5d..=0x7e),
            );
        if valid {
            Ok(Self(value))
        } else {
            Err(super::HttpSecureCookieError::InvalidValue)
        }
    }
}
