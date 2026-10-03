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
    if text.len()
        > constants_usize::VALUE_8_192
            .saturating_add(constants_usize::VALUE_8_192)
            .saturating_add(constants_usize::ONE)
    {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_cookie_resolver_missing_target_and_duplicate_headers() {
        let mut headers = http::HeaderMap::new();
        let resolves_to =
            |http_cookie_headers_ref: crate::http_cookie_headers_ref::HttpCookieHeadersRef<'_>,
             cookie_resolution: crate::cookie_resolution::CookieResolution<'_>| {
                crate::resolve_unique_cookie::resolve_unique_cookie(
                    http_cookie_headers_ref,
                    crate::http_cookie_name_ref::HttpCookieNameRef::from(
                        constants_str::TEST_COOKIE_NAME,
                    ),
                ) == cookie_resolution
            };
        assert!(resolves_to(
            crate::http_cookie_headers_ref::HttpCookieHeadersRef::from(&headers),
            crate::cookie_resolution::CookieResolution::Missing
        ));
        let unrelated = format!("{}={}", constants_str::X, constants_str::TEST_FIRST);
        assert!(http::HeaderValue::from_str(&unrelated).is_ok_and(|header| {
            let _previous = headers.insert(http::header::COOKIE, header.clone());
            assert!(resolves_to(
                crate::http_cookie_headers_ref::HttpCookieHeadersRef::from(&headers),
                crate::cookie_resolution::CookieResolution::Missing
            ));
            let _appended = headers.append(http::header::COOKIE, header);
            resolves_to(
                crate::http_cookie_headers_ref::HttpCookieHeadersRef::from(&headers),
                crate::cookie_resolution::CookieResolution::Invalid,
            )
        }));
    }

    #[test]
    fn test_cookie_pair_limit_counts_malformed_pairs_and_target_position() {
        let target = format!(
            "{}={}",
            constants_str::TEST_COOKIE_NAME,
            constants_str::TEST_FIRST
        );
        assert!([127usize, 128usize].into_iter().all(|unrelated_count| {
            [false, true].into_iter().all(|target_first| {
                let mut pairs = vec![constants_str::X.to_owned(); unrelated_count];
                if target_first {
                    pairs.insert(0usize, target.clone());
                } else {
                    pairs.push(target.clone());
                }
                http::HeaderValue::from_str(&pairs.join(&';'.to_string())).is_ok_and(|header| {
                    let mut headers = http::HeaderMap::new();
                    let _previous = headers.insert(http::header::COOKIE, header);
                    let actual = crate::resolve_unique_cookie::resolve_unique_cookie(
                        crate::http_cookie_headers_ref::HttpCookieHeadersRef::from(&headers),
                        crate::http_cookie_name_ref::HttpCookieNameRef::from(
                            constants_str::TEST_COOKIE_NAME,
                        ),
                    );
                    if unrelated_count == 127usize {
                        actual
                            == crate::cookie_resolution::CookieResolution::Resolved(
                                crate::http_cookie_value_ref::HttpCookieValueRef::from(
                                    constants_str::TEST_FIRST,
                                ),
                            )
                    } else {
                        actual == crate::cookie_resolution::CookieResolution::Invalid
                    }
                })
            })
        }));
    }

    #[test]
    fn test_cookie_resolver_rejects_non_text_header_bytes() {
        assert!(
            http::HeaderValue::from_bytes(&[0xffu8]).is_ok_and(|header| {
                let mut headers = http::HeaderMap::new();
                let _previous = headers.insert(http::header::COOKIE, header);
                crate::resolve_unique_cookie::resolve_unique_cookie(
                    crate::http_cookie_headers_ref::HttpCookieHeadersRef::from(&headers),
                    crate::http_cookie_name_ref::HttpCookieNameRef::from(
                        constants_str::TEST_COOKIE_NAME,
                    ),
                ) == crate::cookie_resolution::CookieResolution::Invalid
            })
        );
    }
}
