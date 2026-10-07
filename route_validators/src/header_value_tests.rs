#[cfg(test)]
mod tests {
    const TEST_HEADER_NAME: axum::http::HeaderName =
        axum::http::HeaderName::from_static(constants_str::ROUTE_VALIDATORS_TEST_HEADER_NAME);
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, PartialEq, Eq)]
    enum TestError {
        NoHeader,
        ParseBool,
        ToStr,
    }
    fn header(
        header_map: &axum::http::HeaderMap,
        name: impl axum::http::header::AsHeaderName,
    ) -> Result<crate::header_str_ref::HeaderStrRef<'_>, TestError> {
        crate::required_header_str::required_header_str(
            crate::axum_headers_ref::AxumHeadersRef::from(header_map),
            name,
            || TestError::NoHeader,
            |_| TestError::ToStr,
        )
    }
    fn raw_header(
        header_map: &axum::http::HeaderMap,
        name: impl axum::http::header::AsHeaderName,
    ) -> Result<crate::axum_header_value_ref::AxumHeaderValueRef<'_>, TestError> {
        crate::required_header_value::required_header_value(
            crate::axum_headers_ref::AxumHeadersRef::from(header_map),
            name,
            || TestError::NoHeader,
        )
    }
    fn bool_header(
        header_map: &axum::http::HeaderMap,
        name: impl axum::http::header::AsHeaderName,
    ) -> Result<bool, TestError> {
        crate::required_header_str_parsed::required_header_str_parsed(
            crate::axum_headers_ref::AxumHeadersRef::from(header_map),
            name,
            || TestError::NoHeader,
            |_to_str_error| TestError::ToStr,
            |header_value| {
                header_value
                    .parse::<bool>()
                    .map_err(|_parse_bool_error| TestError::ParseBool)
            },
        )
    }
    fn make_test_headers<ValueTy>(value_ty: ValueTy) -> crate::axum_test_headers::AxumTestHeaders
    where
        ValueTy: Into<crate::axum_test_header_value::AxumTestHeaderValue>,
    {
        crate::make_headers_with_entry::make_headers_with_entry(TEST_HEADER_NAME, value_ty)
    }
    fn make_test_headers_static(str: &'static str) -> crate::axum_test_headers::AxumTestHeaders {
        make_test_headers(axum::http::HeaderValue::from_static(str))
    }
    fn assert_header_err<T>(result: Result<T, TestError>, test_error: &TestError) {
        assert!(matches!(result, Err(v) if &v == test_error));
    }
    #[test]
    fn test_required_header_str_returns_header_when_present_and_utf8() {
        let headers = make_test_headers_static(constants_str::ABC_ALT_3);
        let actual = header(&headers, TEST_HEADER_NAME);
        assert_eq!(actual.map(<&str>::from), Ok(constants_str::ABC_ALT_3));
    }
    #[test]
    fn test_required_header_str_returns_no_header_error_when_absent() {
        let headers = axum::http::HeaderMap::new();
        assert_header_err(header(&headers, TEST_HEADER_NAME), &TestError::NoHeader);
    }
    #[test]
    fn test_required_header_str_returns_to_str_error_for_non_utf8_header() {
        let headers = make_test_headers(crate::non_utf8_header_value::non_utf8_header_value());
        assert_header_err(header(&headers, TEST_HEADER_NAME), &TestError::ToStr);
    }
    #[test]
    fn test_required_header_str_accepts_str_header_name() {
        let headers = make_test_headers_static(constants_str::ABC_ALT_3);
        let actual = header(&headers, constants_str::ROUTE_VALIDATORS_TEST_HEADER_NAME);
        assert_eq!(actual.map(<&str>::from), Ok(constants_str::ABC_ALT_3));
    }
    #[test]
    fn test_required_header_returns_header_value_when_present() {
        let headers = make_test_headers_static(constants_str::ABC_ALT_3);
        let actual = raw_header(&headers, TEST_HEADER_NAME);
        assert_eq!(
            actual.map(<&axum::http::HeaderValue>::from),
            Ok(&axum::http::HeaderValue::from_static(
                constants_str::ABC_ALT_3
            ))
        );
    }
    #[test]
    fn test_required_header_returns_no_header_error_when_absent() {
        let headers = axum::http::HeaderMap::new();
        assert_header_err(raw_header(&headers, TEST_HEADER_NAME), &TestError::NoHeader);
    }
    #[test]
    fn test_required_header_parsed_returns_parsed_value_for_valid_header() {
        let headers = make_test_headers_static(constants_str::TRUE);
        let actual = bool_header(&headers, TEST_HEADER_NAME);
        assert_eq!(actual, Ok(true));
    }
    #[test]
    fn test_required_header_parsed_returns_parse_error_for_invalid_header_value() {
        let headers = make_test_headers_static(constants_str::NOPE);
        assert_header_err(
            bool_header(&headers, TEST_HEADER_NAME),
            &TestError::ParseBool,
        );
    }
    #[test]
    fn test_required_header_mapped_applies_mapping_for_present_header() {
        let headers = make_test_headers_static(constants_str::ABC_ALT_3);
        let actual = crate::required_header_value::required_header_value(
            crate::axum_headers_ref::AxumHeadersRef::from(&headers),
            TEST_HEADER_NAME,
            || TestError::NoHeader,
        )
        .and_then(|v| {
            v.to_str()
                .map(str::len)
                .map_err(|_to_str_error| TestError::ToStr)
        });
        assert_eq!(actual, Ok(3));
    }
    #[test]
    fn test_required_header_str_parsed_does_not_call_parse_when_header_absent() {
        let headers = axum::http::HeaderMap::new();
        let mut parse_called = false;
        let actual = crate::required_header_str_parsed::required_header_str_parsed(
            crate::axum_headers_ref::AxumHeadersRef::from(&headers),
            TEST_HEADER_NAME,
            || TestError::NoHeader,
            |_to_str_error| TestError::ToStr,
            |_header_value| {
                parse_called = true;
                Ok(true)
            },
        );
        assert_eq!(actual, Err(TestError::NoHeader));
        assert!(!parse_called);
    }
    #[test]
    fn test_required_header_str_parsed_does_not_call_parse_for_non_utf8_header() {
        let headers = make_test_headers(crate::non_utf8_header_value::non_utf8_header_value());
        let mut parse_called = false;
        let actual = crate::required_header_str_parsed::required_header_str_parsed(
            crate::axum_headers_ref::AxumHeadersRef::from(&headers),
            TEST_HEADER_NAME,
            || TestError::NoHeader,
            |_to_str_error| TestError::ToStr,
            |_header_value| {
                parse_called = true;
                Ok(true)
            },
        );
        assert_eq!(actual, Err(TestError::ToStr));
        assert!(!parse_called);
    }
    #[test]
    fn test_present_header_preserves_borrow_and_skips_error_callbacks() {
        let headers = make_test_headers_static(constants_str::ABC_ALT_3);
        let missing_calls = std::cell::Cell::new(constants_usize::ZERO);
        let conversion_calls = std::cell::Cell::new(constants_usize::ZERO);
        let parse_calls = std::cell::Cell::new(constants_usize::ZERO);
        let raw = crate::required_header_value::required_header_value(
            crate::axum_headers_ref::AxumHeadersRef::from(&headers),
            TEST_HEADER_NAME,
            || {
                missing_calls.set(missing_calls.get().saturating_add(constants_usize::ONE));
                TestError::NoHeader
            },
        );
        assert!(raw.is_ok_and(|axum_header_value_ref| {
            headers.get(TEST_HEADER_NAME).is_some_and(|header_value| {
                std::ptr::eq(
                    <&axum::http::HeaderValue>::from(axum_header_value_ref),
                    header_value,
                )
            })
        }));
        let parsed = crate::required_header_str_parsed::required_header_str_parsed(
            crate::axum_headers_ref::AxumHeadersRef::from(&headers),
            TEST_HEADER_NAME,
            || {
                missing_calls.set(missing_calls.get().saturating_add(constants_usize::ONE));
                TestError::NoHeader
            },
            |_to_str_error| {
                conversion_calls.set(conversion_calls.get().saturating_add(constants_usize::ONE));
                TestError::ToStr
            },
            |header_str_ref| {
                parse_calls.set(parse_calls.get().saturating_add(constants_usize::ONE));
                Ok(header_str_ref)
            },
        );
        assert!(parsed.is_ok_and(|header_str_ref| {
            headers.get(TEST_HEADER_NAME).is_some_and(|header_value| {
                header_value.to_str().is_ok_and(|text| {
                    std::ptr::eq(header_str_ref.as_ref(), text)
                        && header_str_ref.as_ref() == constants_str::ABC_ALT_3
                })
            })
        }));
        assert_eq!(missing_calls.get(), constants_usize::ZERO);
        assert_eq!(conversion_calls.get(), constants_usize::ZERO);
        assert_eq!(parse_calls.get(), constants_usize::ONE);
    }

    #[test]
    fn test_header_failures_call_only_the_selected_error_callback() {
        [
            (
                crate::axum_test_headers::AxumTestHeaders::from(axum::http::HeaderMap::new()),
                TestError::NoHeader,
            ),
            (
                make_test_headers(crate::non_utf8_header_value::non_utf8_header_value()),
                TestError::ToStr,
            ),
        ]
        .into_iter()
        .fold((), |(), (headers, expected)| {
            let missing_calls = std::cell::Cell::new(constants_usize::ZERO);
            let conversion_calls = std::cell::Cell::new(constants_usize::ZERO);
            let parse_calls = std::cell::Cell::new(constants_usize::ZERO);
            let actual = crate::required_header_str_parsed::required_header_str_parsed(
                crate::axum_headers_ref::AxumHeadersRef::from(&headers),
                TEST_HEADER_NAME,
                || {
                    missing_calls.set(missing_calls.get().saturating_add(constants_usize::ONE));
                    TestError::NoHeader
                },
                |_to_str_error| {
                    conversion_calls
                        .set(conversion_calls.get().saturating_add(constants_usize::ONE));
                    TestError::ToStr
                },
                |_header_str_ref| {
                    parse_calls.set(parse_calls.get().saturating_add(constants_usize::ONE));
                    Ok(true)
                },
            );
            assert_eq!(
                missing_calls.get(),
                usize::from(expected == TestError::NoHeader)
            );
            assert_eq!(
                conversion_calls.get(),
                usize::from(expected == TestError::ToStr)
            );
            assert_eq!(parse_calls.get(), constants_usize::ZERO);
            assert_eq!(actual, Err(expected));
        });
    }
}
