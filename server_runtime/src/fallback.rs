#![allow(
    clippy::single_call_fn,
    reason = "media-range classification stays isolated from fallback policy resolution"
)]
const MAXIMUM_ACCEPT_MEDIA_RANGE_COUNT: usize = 128usize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FallbackResponseMode {
    HumanReadable,
    MachineReadable,
}

#[derive(Clone, Copy, Debug)]
pub struct HttpFallbackRequestPathRef<'value_lt>(&'value_lt str);
impl<'value_lt> From<&'value_lt str> for HttpFallbackRequestPathRef<'value_lt> {
    fn from(value: &'value_lt str) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct HttpFallbackApiPrefixRef<'value_lt>(&'value_lt str);
impl<'value_lt> From<&'value_lt str> for HttpFallbackApiPrefixRef<'value_lt> {
    fn from(value: &'value_lt str) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct HttpFallbackMetricsPathRef<'value_lt>(&'value_lt str);
impl<'value_lt> From<&'value_lt str> for HttpFallbackMetricsPathRef<'value_lt> {
    fn from(value: &'value_lt str) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct HttpOptionalAcceptHeaderRef<'value_lt>(Option<&'value_lt http::HeaderValue>);
impl<'value_lt> From<Option<&'value_lt http::HeaderValue>>
    for HttpOptionalAcceptHeaderRef<'value_lt>
{
    fn from(value: Option<&'value_lt http::HeaderValue>) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct HttpAcceptHeaderMaximumBytes(usize);
impl From<usize> for HttpAcceptHeaderMaximumBytes {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy)]
struct HttpMediaRangeRef<'value_lt>(&'value_lt str);
#[derive(Clone, Copy)]
struct AcceptsApplicationJson(bool);

#[must_use]
pub fn fallback_response_mode(
    request_path: HttpFallbackRequestPathRef<'_>,
    api_prefix: HttpFallbackApiPrefixRef<'_>,
    metrics_path: HttpFallbackMetricsPathRef<'_>,
    accept: HttpOptionalAcceptHeaderRef<'_>,
    maximum_accept_bytes: HttpAcceptHeaderMaximumBytes,
) -> FallbackResponseMode {
    let normalized_api_prefix = api_prefix.0.strip_suffix('/').unwrap_or(api_prefix.0);
    let api_path = request_path.0 == normalized_api_prefix
        || request_path
            .0
            .strip_prefix(normalized_api_prefix)
            .is_some_and(|suffix| suffix.starts_with('/'));
    if api_path || request_path.0 == metrics_path.0 {
        return FallbackResponseMode::MachineReadable;
    }
    let accepts_json = accept
        .0
        .filter(|value| value.as_bytes().len() <= maximum_accept_bytes.0)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| {
            value
                .split(',')
                .take(MAXIMUM_ACCEPT_MEDIA_RANGE_COUNT.saturating_add(1usize))
                .enumerate()
                .any(|(index, range)| {
                    index < MAXIMUM_ACCEPT_MEDIA_RANGE_COUNT
                        && media_range_accepts_json(HttpMediaRangeRef(range)).0
                })
        });
    if accepts_json {
        FallbackResponseMode::MachineReadable
    } else {
        FallbackResponseMode::HumanReadable
    }
}

fn media_range_accepts_json(range: HttpMediaRangeRef<'_>) -> AcceptsApplicationJson {
    let mut segments = range.0.split(';').map(str::trim);
    AcceptsApplicationJson(
        segments.next().is_some_and(|media_type| {
            media_type.eq_ignore_ascii_case(str_constants::APPLICATION_JSON)
        }) && !segments.any(|parameter| {
            parameter.split_once('=').is_some_and(|(name, value)| {
                name.trim()
                    .eq_ignore_ascii_case(str_constants::HTTP_ACCEPT_QUALITY_PARAMETER)
                    && value.trim().strip_prefix('0').is_some_and(|suffix| {
                        suffix.is_empty()
                            || suffix.strip_prefix('.').is_some_and(|digits| {
                                !digits.is_empty() && digits.bytes().all(|byte| byte == b'0')
                            })
                    })
            })
        }),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn api_path_is_machine_readable_without_accept_header() {
        assert_eq!(
            super::fallback_response_mode(
                super::HttpFallbackRequestPathRef::from(str_constants::TEST_API_USERS_PATH),
                super::HttpFallbackApiPrefixRef::from(str_constants::TEST_API_PREFIX),
                super::HttpFallbackMetricsPathRef::from(str_constants::METRICS),
                super::HttpOptionalAcceptHeaderRef::from(None),
                super::HttpAcceptHeaderMaximumBytes::from(1024usize),
            ),
            super::FallbackResponseMode::MachineReadable
        );
    }
    #[test]
    fn zero_quality_json_is_not_accepted() {
        let accept =
            http::HeaderValue::from_static(str_constants::TEST_ACCEPT_HTML_JSON_ZERO_QUALITY);
        assert_eq!(
            super::fallback_response_mode(
                super::HttpFallbackRequestPathRef::from(str_constants::TEST_SIGNIN_PATH),
                super::HttpFallbackApiPrefixRef::from(str_constants::TEST_API_PREFIX),
                super::HttpFallbackMetricsPathRef::from(str_constants::METRICS),
                super::HttpOptionalAcceptHeaderRef::from(Some(&accept)),
                super::HttpAcceptHeaderMaximumBytes::from(1024usize),
            ),
            super::FallbackResponseMode::HumanReadable
        );
    }
}
