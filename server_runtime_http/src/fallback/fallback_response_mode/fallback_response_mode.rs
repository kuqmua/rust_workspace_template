#[must_use]
pub fn fallback_response_mode(
    request_path: super::super::HttpFallbackRequestPathRef<'_>,
    api_prefix: super::super::HttpFallbackApiPrefixRef<'_>,
    metrics_path: super::super::HttpFallbackMetricsPathRef<'_>,
    accept: super::super::HttpOptionalAcceptHeaderRef<'_>,
    maximum_accept_bytes: super::super::HttpAcceptHeaderMaximumBytes,
) -> super::FallbackResponseMode {
    let normalized_api_prefix = api_prefix.0.strip_suffix('/').unwrap_or(api_prefix.0);
    let api_path = request_path.0 == normalized_api_prefix
        || request_path
            .0
            .strip_prefix(normalized_api_prefix)
            .is_some_and(|suffix| suffix.starts_with('/'));
    if api_path || request_path.0 == metrics_path.0 {
        return super::FallbackResponseMode::MachineReadable;
    }
    let accepts_json = accept
        .0
        .filter(|value| value.as_bytes().len() <= maximum_accept_bytes.0)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| {
            value
                .split(',')
                .take(constants_usize::VALUE_128.saturating_add(constants_usize::ONE))
                .enumerate()
                .any(|(index, range)| {
                    if index >= constants_usize::VALUE_128 {
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
        super::FallbackResponseMode::MachineReadable
    } else {
        super::FallbackResponseMode::HumanReadable
    }
}
