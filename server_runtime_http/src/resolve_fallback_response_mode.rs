pub fn resolve_fallback_response_mode(
    http_fallback_request_path_ref: crate::http_fallback_request_path_ref::HttpFallbackRequestPathRef<'_>,
    http_fallback_api_prefix_ref: crate::http_fallback_api_prefix_ref::HttpFallbackApiPrefixRef<'_>,
    http_fallback_metrics_path_ref: crate::http_fallback_metrics_path_ref::HttpFallbackMetricsPathRef<'_>,
    http_optional_accept_header_ref: crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef<'_>,
    http_accept_header_maximum_bytes: crate::http_accept_header_maximum_bytes::HttpAcceptHeaderMaximumBytes,
) -> crate::fallback_response_mode::FallbackResponseMode {
    let request_path_text = http_fallback_request_path_ref.get();
    let api_prefix_text = http_fallback_api_prefix_ref.get();
    let metrics_path_text = http_fallback_metrics_path_ref.get();
    let normalized_api_prefix = api_prefix_text.strip_suffix('/').unwrap_or(api_prefix_text);
    let api_path = request_path_text == normalized_api_prefix
        || request_path_text
            .strip_prefix(normalized_api_prefix)
            .is_some_and(|suffix| suffix.starts_with('/'));
    if api_path || request_path_text == metrics_path_text {
        return crate::fallback_response_mode::FallbackResponseMode::MachineReadable;
    }
    let accepts_json = http_optional_accept_header_ref
        .get()
        .filter(|value| value.as_bytes().len() <= http_accept_header_maximum_bytes.get())
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
                        parameter.split_once('=').map_or_else(
                            || {
                                parameter.eq_ignore_ascii_case(
                                    constants_str::HTTP_ACCEPT_QUALITY_PARAMETER,
                                )
                            },
                            |(name, quality_value)| {
                                if !name.trim().eq_ignore_ascii_case(
                                    constants_str::HTTP_ACCEPT_QUALITY_PARAMETER,
                                ) {
                                    return false;
                                }
                                let trimmed_quality_value = quality_value.trim();
                                let (integer, fraction) =
                                    trimmed_quality_value.split_once('.').map_or(
                                        (trimmed_quality_value, None),
                                        |(integer, fraction)| (integer, Some(fraction)),
                                    );
                                let valid_fraction = fraction.is_none_or(|digits| {
                                    !digits.is_empty()
                                        && digits.len() <= constants_usize::THREE
                                        && digits.bytes().all(|byte| byte.is_ascii_digit())
                                });
                                let positive_quality = if integer == constants_str::VALUE_1 {
                                    fraction.is_none_or(|digits| {
                                        digits.bytes().all(|byte| byte == b'0')
                                    })
                                } else if integer == constants_str::VALUE_0 {
                                    fraction.is_some_and(|digits| {
                                        digits.bytes().any(|byte| byte != b'0')
                                    })
                                } else {
                                    false
                                };
                                !valid_fraction || !positive_quality
                            },
                        )
                    })
                })
        });
    if accepts_json {
        crate::fallback_response_mode::FallbackResponseMode::MachineReadable
    } else {
        crate::fallback_response_mode::FallbackResponseMode::HumanReadable
    }
}
