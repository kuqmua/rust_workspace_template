#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub struct AxumHeadersRef<'headers_lt>(&'headers_lt axum::http::HeaderMap);
#[cfg(test)]
impl<'headers_lt> From<&'headers_lt crate::test_hlp::AxumTestHeaders>
    for AxumHeadersRef<'headers_lt>
{
    fn from(value: &'headers_lt crate::test_hlp::AxumTestHeaders) -> Self {
        Self(value.as_ref())
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub(crate) struct AxumHeaderValueRef<'header_value_lt>(&'header_value_lt axum::http::HeaderValue);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::AsRefInner,
    newtype::DerefTarget,
    newtype::FromInner,
)]
pub(crate) struct HeaderStrRef<'header_str_lt>(&'header_str_lt str);
#[cfg(test)]
impl<'header_str_lt> HeaderStrRef<'header_str_lt> {
    #[allow(clippy::single_call_fn)] // typed accessor keeps test assertions from exposing the tuple field
    pub(crate) const fn get(self) -> &'header_str_lt str {
        self.0
    }
}
#[allow(clippy::single_call_fn)] // shared helper centralizes required-header extraction and no-header error mapping
fn get_required_header_value<E>(
    headers: AxumHeadersRef<'_>,
    header_name: impl axum::http::header::AsHeaderName,
    no_header_error: impl FnOnce() -> E,
) -> Result<AxumHeaderValueRef<'_>, E> {
    headers
        .0
        .get(header_name)
        .map(AxumHeaderValueRef::from)
        .ok_or_else(no_header_error)
}
#[allow(clippy::single_call_fn)] // shared helper keeps HeaderValue->str conversion and error mapping centralized for header parsers
fn header_value_to_str<E>(
    header_value: AxumHeaderValueRef<'_>,
    to_str_error: impl FnOnce(axum::http::header::ToStrError) -> E,
) -> Result<HeaderStrRef<'_>, E> {
    header_value
        .0
        .to_str()
        .map(HeaderStrRef)
        .map_err(to_str_error)
}
#[allow(clippy::single_call_fn)] // core helper centralizes required-header transform flow reused by parsing helpers
#[cfg(test)]
pub(crate) fn get_required_header_mapped<'headers, E, T>(
    headers: AxumHeadersRef<'headers>,
    header_name: impl axum::http::header::AsHeaderName,
    no_header_error: impl FnOnce() -> E,
    map: impl FnOnce(AxumHeaderValueRef<'headers>) -> Result<T, E>,
) -> Result<T, E> {
    let header = get_required_header_value(headers, header_name, no_header_error)?;
    map(header)
}
#[allow(clippy::single_call_fn)] // helper centralizes required-header parsing and is reusable across validators
#[cfg(test)]
pub(crate) fn get_required_header<E>(
    headers: AxumHeadersRef<'_>,
    header_name: impl axum::http::header::AsHeaderName,
    no_header_error: impl FnOnce() -> E,
) -> Result<AxumHeaderValueRef<'_>, E> {
    get_required_header_value(headers, header_name, no_header_error)
}
#[allow(clippy::single_call_fn)] // helper centralizes required-header string parsing and is reusable across validators
pub(crate) fn get_required_header_str<E>(
    headers: AxumHeadersRef<'_>,
    header_name: impl axum::http::header::AsHeaderName,
    no_header_error: impl FnOnce() -> E,
    to_str_error: impl FnOnce(axum::http::header::ToStrError) -> E,
) -> Result<HeaderStrRef<'_>, E> {
    get_required_header_str_parsed(headers, header_name, no_header_error, to_str_error, Ok)
}
#[allow(clippy::single_call_fn)] // helper centralizes required-header string parsing and is reused by route validators
pub(crate) fn get_required_header_str_parsed<'headers, E, T>(
    headers: AxumHeadersRef<'headers>,
    header_name: impl axum::http::header::AsHeaderName,
    no_header_error: impl FnOnce() -> E,
    to_str_error: impl FnOnce(axum::http::header::ToStrError) -> E,
    parse: impl FnOnce(HeaderStrRef<'headers>) -> Result<T, E>,
) -> Result<T, E> {
    let header_value = get_required_header_value(headers, header_name, no_header_error)?;
    let header_str = header_value_to_str(header_value, to_str_error)?;
    parse(header_str)
}
#[cfg(test)]
mod tests {
    const TEST_HEADER_NAME: axum::http::HeaderName =
        axum::http::HeaderName::from_static(str_constants::ROUTE_VALIDATORS_TEST_HEADER_NAME);
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, PartialEq, Eq)]
    enum TestError {
        NoHeader,
        ParseBool,
        ToStr,
    }
    fn get_header(
        headers: &axum::http::HeaderMap,
        name: impl axum::http::header::AsHeaderName,
    ) -> Result<super::HeaderStrRef<'_>, TestError> {
        super::get_required_header_str(
            super::AxumHeadersRef::from(headers),
            name,
            || TestError::NoHeader,
            |_| TestError::ToStr,
        )
    }
    fn get_raw_header(
        headers: &axum::http::HeaderMap,
        name: impl axum::http::header::AsHeaderName,
    ) -> Result<super::AxumHeaderValueRef<'_>, TestError> {
        super::get_required_header(super::AxumHeadersRef::from(headers), name, || {
            TestError::NoHeader
        })
    }
    fn get_bool_header(
        headers: &axum::http::HeaderMap,
        name: impl axum::http::header::AsHeaderName,
    ) -> Result<bool, TestError> {
        super::get_required_header_str_parsed(
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
    fn mk_test_headers<ValueTy>(value: ValueTy) -> crate::test_hlp::AxumTestHeaders
    where
        ValueTy: Into<crate::test_hlp::AxumTestHeaderValue>,
    {
        crate::test_hlp::mk_headers_with_entry(TEST_HEADER_NAME, value)
    }
    fn mk_test_headers_static(value: &'static str) -> crate::test_hlp::AxumTestHeaders {
        mk_test_headers(axum::http::HeaderValue::from_static(value))
    }
    fn assert_header_err<T>(actual: Result<T, TestError>, exp: &TestError) {
        assert!(matches!(actual, Err(v) if &v == exp));
    }
    #[test]
    fn get_required_header_str_returns_header_when_present_and_utf8() {
        let headers = mk_test_headers_static(str_constants::ABC_ALT_3);
        let actual = get_header(&headers, TEST_HEADER_NAME);
        assert_eq!(actual.map(|v| v.0), Ok("abc"));
    }
    #[test]
    fn get_required_header_str_returns_no_header_error_when_absent() {
        let headers = axum::http::HeaderMap::new();
        assert_header_err(get_header(&headers, TEST_HEADER_NAME), &TestError::NoHeader);
    }
    #[test]
    fn get_required_header_str_returns_to_str_error_for_non_utf8_header() {
        let headers = mk_test_headers(crate::test_hlp::non_utf8_header_value());
        assert_header_err(get_header(&headers, TEST_HEADER_NAME), &TestError::ToStr);
    }
    #[test]
    fn get_required_header_str_accepts_str_header_name() {
        let headers = mk_test_headers_static(str_constants::ABC_ALT_3);
        let actual = get_header(&headers, str_constants::ROUTE_VALIDATORS_TEST_HEADER_NAME);
        assert_eq!(actual.map(|v| v.0), Ok("abc"));
    }
    #[test]
    fn get_required_header_returns_header_value_when_present() {
        let headers = mk_test_headers_static(str_constants::ABC_ALT_3);
        let actual = get_raw_header(&headers, TEST_HEADER_NAME);
        assert_eq!(
            actual.map(|v| v.0),
            Ok(&axum::http::HeaderValue::from_static("abc"))
        );
    }
    #[test]
    fn get_required_header_returns_no_header_error_when_absent() {
        let headers = axum::http::HeaderMap::new();
        assert_header_err(
            get_raw_header(&headers, TEST_HEADER_NAME),
            &TestError::NoHeader,
        );
    }
    #[test]
    fn get_required_header_parsed_returns_parsed_value_for_valid_header() {
        let headers = mk_test_headers_static(str_constants::TRUE);
        let actual = get_bool_header(&headers, TEST_HEADER_NAME);
        assert_eq!(actual, Ok(true));
    }
    #[test]
    fn get_required_header_parsed_returns_parse_error_for_invalid_header_value() {
        let headers = mk_test_headers_static(str_constants::NOPE);
        assert_header_err(
            get_bool_header(&headers, TEST_HEADER_NAME),
            &TestError::ParseBool,
        );
    }
    #[test]
    fn get_required_header_mapped_applies_mapping_for_present_header() {
        let headers = mk_test_headers_static(str_constants::ABC_ALT_3);
        let actual = super::get_required_header_mapped(
            super::AxumHeadersRef::from(&headers),
            TEST_HEADER_NAME,
            || TestError::NoHeader,
            |v| {
                v.0.to_str()
                    .map(str::len)
                    .map_err(|_to_str_error| TestError::ToStr)
            },
        );
        assert_eq!(actual, Ok(3));
    }
    #[test]
    fn get_required_header_str_parsed_does_not_call_parse_when_header_absent() {
        let headers = axum::http::HeaderMap::new();
        let mut parse_called = false;
        let actual = super::get_required_header_str_parsed(
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
    fn get_required_header_str_parsed_does_not_call_parse_for_non_utf8_header() {
        let headers = mk_test_headers(crate::test_hlp::non_utf8_header_value());
        let mut parse_called = false;
        let actual = super::get_required_header_str_parsed(
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
