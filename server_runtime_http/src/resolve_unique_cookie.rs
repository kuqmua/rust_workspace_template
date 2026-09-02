#[must_use]
pub fn resolve_unique_cookie<'value_lt>(
    http_cookie_headers_ref: crate::http_cookie_headers_ref::HttpCookieHeadersRef<'value_lt>,
    http_cookie_name_ref: crate::http_cookie_name_ref::HttpCookieNameRef<'_>,
) -> crate::cookie_resolution::CookieResolution<'value_lt> {
    let mut header_values = http_cookie_headers_ref
        .get()
        .get_all(http::header::COOKIE)
        .iter();
    let cookie_name = http_cookie_name_ref.get();
    let Some(header) = header_values.next() else {
        return crate::cookie_resolution::CookieResolution::Missing;
    };
    if header_values.next().is_some() {
        return crate::cookie_resolution::CookieResolution::Invalid;
    }
    let Ok(text) = header.to_str() else {
        return crate::cookie_resolution::CookieResolution::Invalid;
    };
    if text.len() > constants_usize::VALUE_4_096 {
        return crate::cookie_resolution::CookieResolution::Invalid;
    }
    match text.split(';').try_fold(
        (constants_usize::ZERO, None),
        |(pair_count, found), pair| {
            if pair_count == constants_usize::VALUE_128 {
                return std::ops::ControlFlow::Break(());
            }
            let Some((pair_name, value)) = pair.trim().split_once('=') else {
                return std::ops::ControlFlow::Continue((
                    pair_count.saturating_add(constants_usize::ONE),
                    found,
                ));
            };
            if pair_name != cookie_name {
                return std::ops::ControlFlow::Continue((
                    pair_count.saturating_add(constants_usize::ONE),
                    found,
                ));
            }
            if found.is_some() {
                std::ops::ControlFlow::Break(())
            } else {
                std::ops::ControlFlow::Continue((
                    pair_count.saturating_add(constants_usize::ONE),
                    Some(value),
                ))
            }
        },
    ) {
        std::ops::ControlFlow::Break(()) => crate::cookie_resolution::CookieResolution::Invalid,
        std::ops::ControlFlow::Continue((_pair_count, Some(value))) => {
            crate::cookie_resolution::CookieResolution::Resolved(
                crate::http_cookie_value_ref::HttpCookieValueRef::from(value),
            )
        }
        std::ops::ControlFlow::Continue((_pair_count, None)) => {
            crate::cookie_resolution::CookieResolution::Missing
        }
    }
}
