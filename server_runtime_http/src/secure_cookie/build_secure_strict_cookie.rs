pub fn build_secure_strict_cookie(
    name: &super::HttpCookieName,
    value: &super::HttpCookieValue,
    maximum_age: super::StdCookieMaxAgeSeconds,
    access: super::HttpCookieAccess,
    secure: super::HttpCookieSecure,
) -> Result<super::HttpSetCookieHeaderValue, super::HttpSecureCookieError> {
    let text = i64::try_from(maximum_age.0).map_or_else(
        |_conversion_error| {
            let http_only = match access {
                super::HttpCookieAccess::HttpOnly => constants_str::HTTPONLY,
                super::HttpCookieAccess::ScriptReadable => constants_str::EMPTY,
            };
            let secure_attribute = match secure {
                super::HttpCookieSecure::Disabled => constants_str::EMPTY,
                super::HttpCookieSecure::Enabled => constants_str::SECURE,
            };
            format!(
                "{}={}; Path=/; Max-Age={}; SameSite=Strict{http_only}{secure_attribute}",
                name.0, value.0, maximum_age.0
            )
        },
        |maximum_age_seconds| {
            cookie::Cookie::build((name.0.as_str(), value.0.as_str()))
                .path(constants_str::SLASH)
                .max_age(cookie::time::Duration::seconds(maximum_age_seconds))
                .same_site(cookie::SameSite::Strict)
                .http_only(matches!(access, super::HttpCookieAccess::HttpOnly))
                .secure(matches!(secure, super::HttpCookieSecure::Enabled))
                .build()
                .to_string()
        },
    );
    http::HeaderValue::try_from(text)
        .map(super::HttpSetCookieHeaderValue::from)
        .map_err(|_error| super::HttpSecureCookieError::InvalidHeader)
}
