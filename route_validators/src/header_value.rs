#[cfg(test)]
pub(crate) use crate::axum_header_value_ref::AxumHeaderValueRef;
pub use crate::axum_headers_ref::AxumHeadersRef;
#[cfg(test)]
pub(crate) use crate::header_str_ref::HeaderStrRef;
#[cfg(test)]
pub(crate) use crate::required_header_str::required_header_str;
#[cfg(test)]
pub(crate) use crate::required_header_str_parsed::required_header_str_parsed;
#[cfg(test)]
mod tests {
    const TEST_HEADER_NAME: axum::http::HeaderName =
        axum::http::HeaderName::from_static(constants_str::ROUTE_VALIDATORS_TEST_HEADER_NAME);
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, PartialEq, Eq)]
    enum TestError {
        NoHeader,
        ParseBool,
        ToStr,
    }
    fn header(
        headers: &axum::http::HeaderMap,
        name: impl axum::http::header::AsHeaderName,
    ) -> Result<super::HeaderStrRef<'_>, TestError> {
        super::required_header_str(
            super::AxumHeadersRef::from(headers),
            name,
            || TestError::NoHeader,
            |_| TestError::ToStr,
        )
    }
    fn raw_header(
        headers: &axum::http::HeaderMap,
        name: impl axum::http::header::AsHeaderName,
    ) -> Result<super::AxumHeaderValueRef<'_>, TestError> {
        crate::required_header_value::required_header_value(
            super::AxumHeadersRef::from(headers),
            name,
            || TestError::NoHeader,
        )
    }
    fn bool_header(
        headers: &axum::http::HeaderMap,
        name: impl axum::http::header::AsHeaderName,
    ) -> Result<bool, TestError> {
        super::required_header_str_parsed(
            super::AxumHeadersRef::from(headers),
            name,
            || TestError::NoHeader,
            |_to_str_error| TestError::ToStr,
            |header_value| {
                header_value
                    .0
                    .parse::<bool>()
                    .map_err(|_parse_bool_error| TestError::ParseBool)
            },
        )
    }
    fn make_test_headers<ValueTy>(value: ValueTy) -> crate::test_helper::AxumTestHeaders
    where
        ValueTy: Into<crate::test_helper::AxumTestHeaderValue>,
    {
        crate::test_helper::make_headers_with_entry(TEST_HEADER_NAME, value)
    }
    fn make_test_headers_static(value: &'static str) -> crate::test_helper::AxumTestHeaders {
        make_test_headers(axum::http::HeaderValue::from_static(value))
    }
    fn assert_header_err<T>(actual: Result<T, TestError>, exp: &TestError) {
        assert!(matches!(actual, Err(v) if &v == exp));
    }
    #[test]
    fn required_header_str_returns_header_when_present_and_utf8() {
        let headers = make_test_headers_static(constants_str::ABC_ALT_3);
        let actual = header(&headers, TEST_HEADER_NAME);
        assert_eq!(actual.map(|v| v.0), Ok("abc"));
    }
    #[test]
    fn required_header_str_returns_no_header_error_when_absent() {
        let headers = axum::http::HeaderMap::new();
        assert_header_err(header(&headers, TEST_HEADER_NAME), &TestError::NoHeader);
    }
    #[test]
    fn required_header_str_returns_to_str_error_for_non_utf8_header() {
        let headers = make_test_headers(crate::test_helper::non_utf8_header_value());
        assert_header_err(header(&headers, TEST_HEADER_NAME), &TestError::ToStr);
    }
    #[test]
    fn required_header_str_accepts_str_header_name() {
        let headers = make_test_headers_static(constants_str::ABC_ALT_3);
        let actual = header(&headers, constants_str::ROUTE_VALIDATORS_TEST_HEADER_NAME);
        assert_eq!(actual.map(|v| v.0), Ok("abc"));
    }
    #[test]
    fn required_header_returns_header_value_when_present() {
        let headers = make_test_headers_static(constants_str::ABC_ALT_3);
        let actual = raw_header(&headers, TEST_HEADER_NAME);
        assert_eq!(
            actual.map(|v| v.0),
            Ok(&axum::http::HeaderValue::from_static("abc"))
        );
    }
    #[test]
    fn required_header_returns_no_header_error_when_absent() {
        let headers = axum::http::HeaderMap::new();
        assert_header_err(raw_header(&headers, TEST_HEADER_NAME), &TestError::NoHeader);
    }
    #[test]
    fn required_header_parsed_returns_parsed_value_for_valid_header() {
        let headers = make_test_headers_static(constants_str::TRUE);
        let actual = bool_header(&headers, TEST_HEADER_NAME);
        assert_eq!(actual, Ok(true));
    }
    #[test]
    fn required_header_parsed_returns_parse_error_for_invalid_header_value() {
        let headers = make_test_headers_static(constants_str::NOPE);
        assert_header_err(
            bool_header(&headers, TEST_HEADER_NAME),
            &TestError::ParseBool,
        );
    }
    #[test]
    fn required_header_mapped_applies_mapping_for_present_header() {
        let headers = make_test_headers_static(constants_str::ABC_ALT_3);
        let actual = crate::required_header_value::required_header_value(
            super::AxumHeadersRef::from(&headers),
            TEST_HEADER_NAME,
            || TestError::NoHeader,
        )
        .and_then(|v| {
            v.0.to_str()
                .map(str::len)
                .map_err(|_to_str_error| TestError::ToStr)
        });
        assert_eq!(actual, Ok(3));
    }
    #[test]
    fn required_header_str_parsed_does_not_call_parse_when_header_absent() {
        let headers = axum::http::HeaderMap::new();
        let mut parse_called = false;
        let actual = super::required_header_str_parsed(
            super::AxumHeadersRef::from(&headers),
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
    fn required_header_str_parsed_does_not_call_parse_for_non_utf8_header() {
        let headers = make_test_headers(crate::test_helper::non_utf8_header_value());
        let mut parse_called = false;
        let actual = super::required_header_str_parsed(
            super::AxumHeadersRef::from(&headers),
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
}
