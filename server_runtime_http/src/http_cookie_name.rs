#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpCookieName(String);

impl HttpCookieName {
    pub(crate) const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for HttpCookieName {
    type Error = crate::http_secure_cookie_error::HttpSecureCookieError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let valid = !value.is_empty()
            && value.len() <= constants_usize::VALUE_8_192
            && value.bytes().all(|byte| {
                byte.is_ascii_alphanumeric()
                    || matches!(
                        byte,
                        b'!' | b'#'
                            | b'$'
                            | b'%'
                            | b'&'
                            | b'\''
                            | b'*'
                            | b'+'
                            | b'-'
                            | b'.'
                            | b'^'
                            | b'_'
                            | b'`'
                            | b'|'
                            | b'~'
                    )
            });
        if valid {
            Ok(Self(value))
        } else {
            Err(crate::http_secure_cookie_error::HttpSecureCookieError::InvalidName)
        }
    }
}
