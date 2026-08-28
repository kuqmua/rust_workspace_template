pub use crate::build_secure_strict_cookie::build_secure_strict_cookie;
pub use crate::http_cookie_access::HttpCookieAccess;
pub use crate::http_cookie_name::HttpCookieName;
pub use crate::http_cookie_secure::HttpCookieSecure;
pub use crate::http_cookie_value::HttpCookieValue;
pub use crate::http_secure_cookie_error::HttpSecureCookieError;
pub use crate::http_set_cookie_header_value::HttpSetCookieHeaderValue;
pub use crate::std_cookie_max_age_seconds::StdCookieMaxAgeSeconds;

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

// Root-owned module compatibility wrappers.
mod build_secure_strict_cookie {
    pub use crate::build_secure_strict_cookie::*;
}
mod http_cookie_access {
    pub use crate::http_cookie_access::*;
}
mod http_cookie_name {
    pub use crate::http_cookie_name::*;
}
mod http_cookie_secure {
    pub use crate::http_cookie_secure::*;
}
mod http_cookie_value {
    pub use crate::http_cookie_value::*;
}
mod http_secure_cookie_error {
    pub use crate::http_secure_cookie_error::*;
}
mod http_set_cookie_header_value {
    pub use crate::http_set_cookie_header_value::*;
}
mod std_cookie_max_age_seconds {
    pub use crate::std_cookie_max_age_seconds::*;
}
