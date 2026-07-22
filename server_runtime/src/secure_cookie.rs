const COOKIE_TEXT_MAXIMUM_BYTES: usize = 8192usize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpCookieName(String);
impl TryFrom<String> for HttpCookieName {
    type Error = HttpSecureCookieError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let valid = !value.is_empty()
            && value.len() <= COOKIE_TEXT_MAXIMUM_BYTES
            && value.bytes().all(|byte| {
                byte.is_ascii_alphanumeric()
                    || matches!(byte, b'!' | b'#'..=b'+' | b'-'..=b':' | b'<'..=b'[' | b']'..=b'~')
            });
        if valid {
            Ok(Self(value))
        } else {
            Err(HttpSecureCookieError::InvalidName)
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct HttpCookieValue(String);
impl std::fmt::Debug for HttpCookieValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::REDACTED_ALT_3)
    }
}
impl TryFrom<String> for HttpCookieValue {
    type Error = HttpSecureCookieError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let valid = value.len() <= COOKIE_TEXT_MAXIMUM_BYTES
            && value.bytes().all(
                |byte| matches!(byte, 0x21 | 0x23..=0x2b | 0x2d..=0x3a | 0x3c..=0x5b | 0x5d..=0x7e),
            );
        if valid {
            Ok(Self(value))
        } else {
            Err(HttpSecureCookieError::InvalidValue)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct StdCookieMaxAgeSeconds(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpCookieAccess {
    HttpOnly,
    ScriptReadable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpCookieSecure {
    Disabled,
    Enabled,
}

#[derive(Clone, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct HttpSetCookieHeaderValue(http::HeaderValue);

impl From<HttpSetCookieHeaderValue> for http::HeaderValue {
    fn from(value: HttpSetCookieHeaderValue) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum HttpSecureCookieError {
    #[error("generated Set-Cookie header is invalid")]
    InvalidHeader,
    #[error("invalid cookie name")]
    InvalidName,
    #[error("invalid cookie value")]
    InvalidValue,
}

pub fn build_secure_strict_cookie(
    name: &HttpCookieName,
    value: &HttpCookieValue,
    maximum_age: StdCookieMaxAgeSeconds,
    access: HttpCookieAccess,
    secure: HttpCookieSecure,
) -> Result<HttpSetCookieHeaderValue, HttpSecureCookieError> {
    let http_only = match access {
        HttpCookieAccess::HttpOnly => str_constants::HTTPONLY,
        HttpCookieAccess::ScriptReadable => str_constants::EMPTY,
    };
    let secure_attribute = match secure {
        HttpCookieSecure::Disabled => str_constants::EMPTY,
        HttpCookieSecure::Enabled => str_constants::SECURE,
    };
    let text = format!(
        "{}={}; Path=/; Max-Age={}; SameSite=Strict{http_only}{secure_attribute}",
        name.0, value.0, maximum_age.0
    );
    http::HeaderValue::try_from(text)
        .map(HttpSetCookieHeaderValue)
        .map_err(|_error| HttpSecureCookieError::InvalidHeader)
}

#[cfg(test)]
mod tests {
    #[test]
    fn builder_sets_security_attributes_and_rejects_injection() {
        let name = super::HttpCookieName::try_from(String::from(str_constants::TEST_COOKIE_NAME))
            .expect("977f74f0");
        let value =
            super::HttpCookieValue::try_from(String::from(str_constants::TEST_COOKIE_VALUE))
                .expect("38fc5531");
        let header = super::build_secure_strict_cookie(
            &name,
            &value,
            60u64.into(),
            super::HttpCookieAccess::HttpOnly,
            super::HttpCookieSecure::Enabled,
        )
        .expect("0b4600b3");
        let header_value = http::HeaderValue::from(header);
        let text = header_value.to_str().expect("3176fb72");
        assert!(text.contains(str_constants::HTTPONLY));
        assert!(text.contains(str_constants::SECURE));
        assert_eq!(
            super::HttpCookieValue::try_from(String::from(str_constants::TEST_COOKIE_INJECTION)),
            Err(super::HttpSecureCookieError::InvalidValue),
        );
    }
}
