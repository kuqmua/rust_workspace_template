pub fn build_secure_strict_cookie(
    http_cookie_name: &crate::http_cookie_name::HttpCookieName,
    http_cookie_value: &crate::http_cookie_value::HttpCookieValue,
    std_cookie_max_age_seconds: crate::std_cookie_max_age_seconds::StdCookieMaxAgeSeconds,
    http_cookie_access: crate::http_cookie_access::HttpCookieAccess,
    http_cookie_secure: crate::http_cookie_secure::HttpCookieSecure,
) -> Result<
    crate::http_set_cookie_header_value::HttpSetCookieHeaderValue,
    crate::http_secure_cookie_error::HttpSecureCookieError,
> {
    let maximum_age_seconds_u64 = std_cookie_max_age_seconds.get();
    let text = i64::try_from(maximum_age_seconds_u64).map_or_else(
        |_conversion_error| {
            let http_only = match http_cookie_access {
                crate::http_cookie_access::HttpCookieAccess::HttpOnly => constants_str::HTTPONLY,
                crate::http_cookie_access::HttpCookieAccess::ScriptReadable => constants_str::EMPTY,
            };
            let secure_attribute = match http_cookie_secure {
                crate::http_cookie_secure::HttpCookieSecure::Disabled => constants_str::EMPTY,
                crate::http_cookie_secure::HttpCookieSecure::Enabled => constants_str::SECURE,
            };
            format!(
                "{}={}; Path=/; Max-Age={}; SameSite=Strict{http_only}{secure_attribute}",
                http_cookie_name.as_str(),
                http_cookie_value.as_str(),
                maximum_age_seconds_u64
            )
        },
        |maximum_age_seconds| {
            cookie::Cookie::build((http_cookie_name.as_str(), http_cookie_value.as_str()))
                .path(constants_str::SLASH)
                .max_age(cookie::time::Duration::seconds(maximum_age_seconds))
                .same_site(cookie::SameSite::Strict)
                .http_only(matches!(
                    http_cookie_access,
                    crate::http_cookie_access::HttpCookieAccess::HttpOnly
                ))
                .secure(matches!(
                    http_cookie_secure,
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
