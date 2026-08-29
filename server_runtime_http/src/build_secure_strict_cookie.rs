pub fn build_secure_strict_cookie(
    name: &crate::http_cookie_name::HttpCookieName,
    value: &crate::http_cookie_value::HttpCookieValue,
    maximum_age: crate::std_cookie_max_age_seconds::StdCookieMaxAgeSeconds,
    access: crate::http_cookie_access::HttpCookieAccess,
    secure: crate::http_cookie_secure::HttpCookieSecure,
) -> Result<
    crate::http_set_cookie_header_value::HttpSetCookieHeaderValue,
    crate::http_secure_cookie_error::HttpSecureCookieError,
> {
    let text = i64::try_from(maximum_age.0).map_or_else(
        |_conversion_error| {
            let http_only = match access {
                crate::http_cookie_access::HttpCookieAccess::HttpOnly => {
                    constants_str::catalog::HTTPONLY
                }
                crate::http_cookie_access::HttpCookieAccess::ScriptReadable => {
                    constants_str::test_fixtures::EMPTY
                }
            };
            let secure_attribute = match secure {
                crate::http_cookie_secure::HttpCookieSecure::Disabled => {
                    constants_str::test_fixtures::EMPTY
                }
                crate::http_cookie_secure::HttpCookieSecure::Enabled => {
                    constants_str::catalog::SECURE
                }
            };
            format!(
                "{}={}; Path=/; Max-Age={}; SameSite=Strict{http_only}{secure_attribute}",
                name.0, value.0, maximum_age.0
            )
        },
        |maximum_age_seconds| {
            cookie::Cookie::build((name.0.as_str(), value.0.as_str()))
                .path(constants_str::catalog::SLASH)
                .max_age(cookie::time::Duration::seconds(maximum_age_seconds))
                .same_site(cookie::SameSite::Strict)
                .http_only(matches!(
                    access,
                    crate::http_cookie_access::HttpCookieAccess::HttpOnly
                ))
                .secure(matches!(
                    secure,
                    crate::http_cookie_secure::HttpCookieSecure::Enabled
                ))
                .build()
                .to_string()
        },
    );
    http::HeaderValue::try_from(text)
        .map(crate::http_set_cookie_header_value::HttpSetCookieHeaderValue::from)
        .map_err(|_error| crate::http_secure_cookie_error::HttpSecureCookieError::InvalidHeader)
}
