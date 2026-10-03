#[test]
fn test_cookie_lookup_resolves_each_admin_cookie_kind() {
    assert!(
        [
            crate::admin_cookie_kind::AdminCookieKind::Access,
            crate::admin_cookie_kind::AdminCookieKind::Csrf,
            crate::admin_cookie_kind::AdminCookieKind::Refresh,
        ]
        .into_iter()
        .all(|kind| {
            let text = format!("{}={}", kind.name().get(), constants_str::TEST_FIRST);
            http::HeaderValue::from_str(text.as_str()).is_ok_and(|value| {
                let mut headers = http::HeaderMap::new();
                let _previous = headers.insert(http::header::COOKIE, value);
                crate::find_admin_cookie::find_admin_cookie(
                    crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(&headers),
                    kind,
                )
                .is_some_and(|resolved| resolved.get() == constants_str::TEST_FIRST)
            })
        })
    );
}

#[test]
fn test_cookie_lookup_rejects_missing_and_duplicate_admin_cookies() {
    assert!(
        [
            crate::admin_cookie_kind::AdminCookieKind::Access,
            crate::admin_cookie_kind::AdminCookieKind::Csrf,
            crate::admin_cookie_kind::AdminCookieKind::Refresh,
        ]
        .into_iter()
        .all(|kind| {
            let mut headers = http::HeaderMap::new();
            if crate::find_admin_cookie::find_admin_cookie(
                crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(&headers),
                kind,
            )
            .is_some()
            {
                return false;
            }
            let text = format!(
                "{}={};{}={}",
                kind.name().get(),
                constants_str::TEST_FIRST,
                kind.name().get(),
                constants_str::TEST_LAST
            );
            http::HeaderValue::from_str(text.as_str()).is_ok_and(|value| {
                let _previous = headers.insert(http::header::COOKIE, value);
                crate::find_admin_cookie::find_admin_cookie(
                    crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(&headers),
                    kind,
                )
                .is_none()
            })
        })
    );
}

#[test]
fn test_cookie_lookup_rejects_multiple_cookie_headers() {
    assert!(
        [
            crate::admin_cookie_kind::AdminCookieKind::Access,
            crate::admin_cookie_kind::AdminCookieKind::Csrf,
            crate::admin_cookie_kind::AdminCookieKind::Refresh,
        ]
        .into_iter()
        .all(|kind| {
            let text = format!("{}={}", kind.name().get(), constants_str::TEST_FIRST);
            http::HeaderValue::from_str(text.as_str()).is_ok_and(|value| {
                let mut headers = http::HeaderMap::new();
                let _previous = headers.insert(http::header::COOKIE, value.clone());
                let _appended = headers.append(http::header::COOKIE, value);
                crate::find_admin_cookie::find_admin_cookie(
                    crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(&headers),
                    kind,
                )
                .is_none()
            })
        })
    );
}
