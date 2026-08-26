const MAXIMUM_ACCEPT_MEDIA_RANGE_COUNT: usize = 128usize;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum FallbackResponseMode {
    HumanReadable,
    MachineReadable,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpFallbackRequestPathRef<'value_lt>(&'value_lt str);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpFallbackApiPrefixRef<'value_lt>(&'value_lt str);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpFallbackMetricsPathRef<'value_lt>(&'value_lt str);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpOptionalAcceptHeaderRef<'value_lt>(Option<&'value_lt http::HeaderValue>);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpAcceptHeaderMaximumBytes(usize);

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
                .take(MAXIMUM_ACCEPT_MEDIA_RANGE_COUNT.saturating_add(constants_usize::ONE))
                .enumerate()
                .any(|(index, range)| {
                    if index >= MAXIMUM_ACCEPT_MEDIA_RANGE_COUNT {
                        return false;
                    }
                    let mut segments = range.split(';').map(str::trim);
                    segments.next().is_some_and(|media_type| {
                        media_type.eq_ignore_ascii_case(constants_str::APPLICATION_JSON)
                    }) && !segments.any(|parameter| {
                        parameter
                            .split_once('=')
                            .is_some_and(|(name, quality_value)| {
                                name.trim().eq_ignore_ascii_case(
                                    constants_str::HTTP_ACCEPT_QUALITY_PARAMETER,
                                ) && quality_value
                                    .trim()
                                    .strip_prefix('0')
                                    .is_some_and(|suffix| {
                                        suffix.is_empty()
                                            || suffix.strip_prefix('.').is_some_and(|digits| {
                                                !digits.is_empty()
                                                    && digits.bytes().all(|byte| byte == b'0')
                                            })
                                    })
                            })
                    })
                })
        });
    if accepts_json {
        FallbackResponseMode::MachineReadable
    } else {
        FallbackResponseMode::HumanReadable
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn api_path_is_machine_readable_without_accept_header() {
        assert_eq!(
            super::fallback_response_mode(
                super::HttpFallbackRequestPathRef::from(constants_str::TEST_SERVICE_USERS_PATH),
                super::HttpFallbackApiPrefixRef::from(constants_str::TEST_SERVICE_PREFIX),
                super::HttpFallbackMetricsPathRef::from(constants_str::METRICS),
                super::HttpOptionalAcceptHeaderRef::from(None),
                super::HttpAcceptHeaderMaximumBytes::from(1024usize),
            ),
            super::FallbackResponseMode::MachineReadable
        );
    }
    #[test]
    fn zero_quality_json_is_not_accepted() {
        let accept =
            http::HeaderValue::from_static(constants_str::TEST_ACCEPT_HTML_JSON_ZERO_QUALITY);
        assert_eq!(
            super::fallback_response_mode(
                super::HttpFallbackRequestPathRef::from(constants_str::TEST_SIGNIN_PATH),
                super::HttpFallbackApiPrefixRef::from(constants_str::TEST_SERVICE_PREFIX),
                super::HttpFallbackMetricsPathRef::from(constants_str::METRICS),
                super::HttpOptionalAcceptHeaderRef::from(Some(&accept)),
                super::HttpAcceptHeaderMaximumBytes::from(1024usize),
            ),
            super::FallbackResponseMode::HumanReadable
        );
    }
}
