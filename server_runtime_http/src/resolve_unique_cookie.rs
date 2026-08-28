#[must_use]
pub fn resolve_unique_cookie<'value_lt>(
    headers: super::HttpCookieHeadersRef<'value_lt>,
    name: super::HttpCookieNameRef<'_>,
) -> super::CookieResolution<'value_lt> {
    let mut header_values = headers.0.get_all(http::header::COOKIE).iter();
    let Some(header) = header_values.next() else {
        return super::CookieResolution::Missing;
    };
    if header_values.next().is_some() {
        return super::CookieResolution::Invalid;
    }
    let Ok(text) = header.to_str() else {
        return super::CookieResolution::Invalid;
    };
    if text.len() > constants_usize::VALUE_4_096 {
        return super::CookieResolution::Invalid;
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
            if pair_name != name.0 {
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
        std::ops::ControlFlow::Break(()) => super::CookieResolution::Invalid,
        std::ops::ControlFlow::Continue((_pair_count, Some(value))) => {
            super::CookieResolution::Resolved(super::HttpCookieValueRef::from(value))
        }
        std::ops::ControlFlow::Continue((_pair_count, None)) => super::CookieResolution::Missing,
    }
}
