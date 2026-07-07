#[allow(clippy::single_call_fn)] // shared helper centralizes required-header extraction and no-header error mapping
fn get_required_header_value<E>(
    headers: &axum::http::HeaderMap,
    header_name: impl axum::http::header::AsHeaderName,
    no_header_er: impl FnOnce() -> E,
) -> Result<&axum::http::HeaderValue, E> {
    headers.get(header_name).ok_or_else(no_header_er)
}
#[allow(clippy::single_call_fn)] // shared helper keeps HeaderValue->str conversion and error mapping centralized for header parsers
fn header_value_to_str<E>(
    header_value: &axum::http::HeaderValue,
    to_str_er: impl FnOnce(axum::http::header::ToStrError) -> E,
) -> Result<&str, E> {
    header_value.to_str().map_err(to_str_er)
}
#[allow(clippy::single_call_fn)] // core helper centralizes required-header transform flow reused by parsing helpers
#[cfg(test)]
pub(crate) fn get_required_header_mapped<'headers, E, T>(
    headers: &'headers axum::http::HeaderMap,
    header_name: impl axum::http::header::AsHeaderName,
    no_header_er: impl FnOnce() -> E,
    map: impl FnOnce(&'headers axum::http::HeaderValue) -> Result<T, E>,
) -> Result<T, E> {
    let header = get_required_header_value(headers, header_name, no_header_er)?;
    map(header)
}
#[allow(clippy::single_call_fn)] // helper centralizes required-header parsing and is reusable across validators
#[cfg(test)]
pub(crate) fn get_required_header<E>(
    headers: &axum::http::HeaderMap,
    header_name: impl axum::http::header::AsHeaderName,
    no_header_er: impl FnOnce() -> E,
) -> Result<&axum::http::HeaderValue, E> {
    get_required_header_value(headers, header_name, no_header_er)
}
#[allow(clippy::single_call_fn)] // helper centralizes required-header string parsing and is reusable across validators
pub(crate) fn get_required_header_str<E>(
    headers: &axum::http::HeaderMap,
    header_name: impl axum::http::header::AsHeaderName,
    no_header_er: impl FnOnce() -> E,
    to_str_er: impl FnOnce(axum::http::header::ToStrError) -> E,
) -> Result<&str, E> {
    get_required_header_str_parsed(headers, header_name, no_header_er, to_str_er, Ok)
}
#[allow(clippy::single_call_fn)] // helper centralizes required-header string parsing and is reused by route validators
pub(crate) fn get_required_header_str_parsed<'headers, E, T>(
    headers: &'headers axum::http::HeaderMap,
    header_name: impl axum::http::header::AsHeaderName,
    no_header_er: impl FnOnce() -> E,
    to_str_er: impl FnOnce(axum::http::header::ToStrError) -> E,
    parse: impl FnOnce(&'headers str) -> Result<T, E>,
) -> Result<T, E> {
    let header_value = get_required_header_value(headers, header_name, no_header_er)?;
    let header_str = header_value_to_str(header_value, to_str_er)?;
    parse(header_str)
}
#[cfg(test)]
mod tests {
    const TEST_HEADER_NAME: axum::http::HeaderName =
        axum::http::HeaderName::from_static("x-test-header");
    #[derive(Debug, PartialEq, Eq)]
    enum TestEr {
        NoHeader,
        ParseBool,
        ToStr,
    }
    fn get_header(
        headers: &axum::http::HeaderMap,
        name: impl axum::http::header::AsHeaderName,
    ) -> Result<&str, TestEr> {
        super::get_required_header_str(headers, name, || TestEr::NoHeader, |_| TestEr::ToStr)
    }
    fn get_raw_header(
        headers: &axum::http::HeaderMap,
        name: impl axum::http::header::AsHeaderName,
    ) -> Result<&axum::http::HeaderValue, TestEr> {
        super::get_required_header(headers, name, || TestEr::NoHeader)
    }
    fn get_bool_header(
        headers: &axum::http::HeaderMap,
        name: impl axum::http::header::AsHeaderName,
    ) -> Result<bool, TestEr> {
        super::get_required_header_str_parsed(
            headers,
            name,
            || TestEr::NoHeader,
            |_to_str_er| TestEr::ToStr,
            |header_value| {
                header_value
                    .parse::<bool>()
                    .map_err(|_parse_bool_er| TestEr::ParseBool)
            },
        )
    }
    fn mk_test_headers(value: axum::http::HeaderValue) -> axum::http::HeaderMap {
        crate::test_hlp::mk_headers_with_entry(TEST_HEADER_NAME, value)
    }
    #[allow(clippy::single_call_fn)] // shared literal-header fixture keeps repetitive test setup concise
    fn mk_test_headers_static(value: &'static str) -> axum::http::HeaderMap {
        mk_test_headers(axum::http::HeaderValue::from_static(value))
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps expected TestEr error checks concise across header helpers
    fn assert_header_err<T>(actual: Result<T, TestEr>, exp: &TestEr) {
        assert!(matches!(actual, Err(v) if &v == exp));
    }
    #[test]
    fn get_required_header_str_returns_header_when_present_and_utf8() {
        let headers = mk_test_headers_static("abc");
        let actual = get_header(&headers, TEST_HEADER_NAME);
        assert_eq!(actual, Ok("abc"));
    }
    #[test]
    fn get_required_header_str_returns_no_header_error_when_absent() {
        let headers = axum::http::HeaderMap::new();
        assert_header_err(get_header(&headers, TEST_HEADER_NAME), &TestEr::NoHeader);
    }
    #[test]
    fn get_required_header_str_returns_to_str_error_for_non_utf8_header() {
        let headers = mk_test_headers(crate::test_hlp::non_utf8_header_value());
        assert_header_err(get_header(&headers, TEST_HEADER_NAME), &TestEr::ToStr);
    }
    #[test]
    fn get_required_header_str_accepts_str_header_name() {
        let headers = mk_test_headers_static("abc");
        let actual = get_header(&headers, "x-test-header");
        assert_eq!(actual, Ok("abc"));
    }
    #[test]
    fn get_required_header_returns_header_value_when_present() {
        let headers = mk_test_headers_static("abc");
        let actual = get_raw_header(&headers, TEST_HEADER_NAME);
        assert_eq!(actual, Ok(&axum::http::HeaderValue::from_static("abc")));
    }
    #[test]
    fn get_required_header_returns_no_header_error_when_absent() {
        let headers = axum::http::HeaderMap::new();
        assert_header_err(
            get_raw_header(&headers, TEST_HEADER_NAME),
            &TestEr::NoHeader,
        );
    }
    #[test]
    fn get_required_header_parsed_returns_parsed_value_for_valid_header() {
        let headers = mk_test_headers_static("true");
        let actual = get_bool_header(&headers, TEST_HEADER_NAME);
        assert_eq!(actual, Ok(true));
    }
    #[test]
    fn get_required_header_parsed_returns_parse_error_for_invalid_header_value() {
        let headers = mk_test_headers_static("nope");
        assert_header_err(
            get_bool_header(&headers, TEST_HEADER_NAME),
            &TestEr::ParseBool,
        );
    }
    #[test]
    fn get_required_header_mapped_applies_mapping_for_present_header() {
        let headers = mk_test_headers_static("abc");
        let actual = super::get_required_header_mapped(
            &headers,
            TEST_HEADER_NAME,
            || TestEr::NoHeader,
            |v| v.to_str().map(str::len).map_err(|_to_str_er| TestEr::ToStr),
        );
        assert_eq!(actual, Ok(3));
    }
    #[test]
    fn get_required_header_str_parsed_does_not_call_parse_when_header_absent() {
        let headers = axum::http::HeaderMap::new();
        let mut parse_called = false;
        let actual = super::get_required_header_str_parsed(
            &headers,
            TEST_HEADER_NAME,
            || TestEr::NoHeader,
            |_to_str_er| TestEr::ToStr,
            |_header_value| {
                parse_called = true;
                Ok(true)
            },
        );
        assert_eq!(actual, Err(TestEr::NoHeader));
        assert!(!parse_called);
    }
    #[test]
    fn get_required_header_str_parsed_does_not_call_parse_for_non_utf8_header() {
        let headers = mk_test_headers(crate::test_hlp::non_utf8_header_value());
        let mut parse_called = false;
        let actual = super::get_required_header_str_parsed(
            &headers,
            TEST_HEADER_NAME,
            || TestEr::NoHeader,
            |_to_str_er| TestEr::ToStr,
            |_header_value| {
                parse_called = true;
                Ok(true)
            },
        );
        assert_eq!(actual, Err(TestEr::ToStr));
        assert!(!parse_called);
    }
}
