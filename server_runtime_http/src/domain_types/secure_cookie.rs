#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpCookieName(String);
impl TryFrom<String> for HttpCookieName {
    type Error = HttpSecureCookieError;
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
            Err(HttpSecureCookieError::InvalidName)
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Eq, PartialEq)]
pub struct HttpCookieValue(String);
impl std::fmt::Debug for HttpCookieValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::REDACTED_ALT_3)
    }
}
impl TryFrom<String> for HttpCookieValue {
    type Error = HttpSecureCookieError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let valid = value.len() <= constants_usize::VALUE_8_192
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

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct StdCookieMaxAgeSeconds(u64);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpCookieAccess {
    HttpOnly,
    ScriptReadable,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpCookieSecure {
    Disabled,
    Enabled,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct HttpSetCookieHeaderValue(http::HeaderValue);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
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
    let text = i64::try_from(maximum_age.0).map_or_else(
        |_conversion_error| {
            let http_only = match access {
                HttpCookieAccess::HttpOnly => constants_str::HTTPONLY,
                HttpCookieAccess::ScriptReadable => constants_str::EMPTY,
            };
            let secure_attribute = match secure {
                HttpCookieSecure::Disabled => constants_str::EMPTY,
                HttpCookieSecure::Enabled => constants_str::SECURE,
            };
            format!(
                "{}={}; Path=/; Max-Age={}; SameSite=Strict{http_only}{secure_attribute}",
                name.0, value.0, maximum_age.0
            )
        },
        |maximum_age_seconds| {
            cookie::Cookie::build((name.0.as_str(), value.0.as_str()))
                .path("/")
                .max_age(cookie::time::Duration::seconds(maximum_age_seconds))
                .same_site(cookie::SameSite::Strict)
                .http_only(matches!(access, HttpCookieAccess::HttpOnly))
                .secure(matches!(secure, HttpCookieSecure::Enabled))
                .build()
                .to_string()
        },
    );
    http::HeaderValue::try_from(text)
        .map(HttpSetCookieHeaderValue)
        .map_err(|_error| HttpSecureCookieError::InvalidHeader)
}

#[cfg(test)]
mod tests {
    #[test]
    fn builder_sets_security_attributes_and_rejects_injection() {
        let name = super::HttpCookieName::try_from(String::from(constants_str::TEST_COOKIE_NAME))
            .expect("977f74f0 builder_sets_security_attributes_and_rejects_injection invariant must hold");
        let value = super::HttpCookieValue::try_from(String::from(
            constants_str::TEST_COOKIE_VALUE,
        ))
        .expect(
            "38fc5531 builder_sets_security_attributes_and_rejects_injection invariant must hold",
        );
        let header = super::build_secure_strict_cookie(
            &name,
            &value,
            60u64.into(),
            super::HttpCookieAccess::HttpOnly,
            super::HttpCookieSecure::Enabled,
        )
        .expect(
            "0b4600b3 builder_sets_security_attributes_and_rejects_injection invariant must hold",
        );
        let header_value = http::HeaderValue::from(header);
        let text = header_value.to_str().expect(
            "3176fb72 builder_sets_security_attributes_and_rejects_injection invariant must hold",
        );
        assert!(text.contains(constants_str::HTTPONLY));
        assert!(text.contains(constants_str::SECURE));
        assert_eq!(
            super::HttpCookieValue::try_from(String::from(constants_str::TEST_COOKIE_INJECTION)),
            Err(super::HttpSecureCookieError::InvalidValue),
        );
        assert_eq!(
            super::HttpCookieName::try_from(String::from("session/path")),
            Err(super::HttpSecureCookieError::InvalidName),
        );
        assert_eq!(
            super::HttpCookieName::try_from(String::from("session=shadow")),
            Err(super::HttpSecureCookieError::InvalidName),
        );
    }

    #[test]
    fn builder_preserves_unsigned_maximum_age_range() {
        let name = super::HttpCookieName::try_from(String::from(constants_str::TEST_COOKIE_NAME))
            .expect("3dde3ff2 builder_preserves_unsigned_maximum_age_range invariant must hold");
        let value =
            super::HttpCookieValue::try_from(String::from(constants_str::TEST_COOKIE_VALUE))
                .expect(
                    "7b47e5b5 builder_preserves_unsigned_maximum_age_range invariant must hold",
                );
        let header = super::build_secure_strict_cookie(
            &name,
            &value,
            u64::MAX.into(),
            super::HttpCookieAccess::ScriptReadable,
            super::HttpCookieSecure::Disabled,
        )
        .expect("0a722d46 builder_preserves_unsigned_maximum_age_range invariant must hold");
        assert!(
            http::HeaderValue::from(header)
                .to_str()
                .expect("b1dde58f builder_preserves_unsigned_maximum_age_range invariant must hold")
                .contains(u64::MAX.to_string().as_str())
        );
    }
}
