#[cfg(test)]
mod tests {
    #[test]
    fn test_api_path_is_machine_readable_without_accept_header() {
        assert_eq!(
            crate::resolve_fallback_response_mode::resolve_fallback_response_mode(
                crate::http_fallback_request_path_ref::HttpFallbackRequestPathRef::from(
                    constants_str::TEST_SERVICE_USERS_PATH
                ),
                crate::http_fallback_api_prefix_ref::HttpFallbackApiPrefixRef::from(
                    constants_str::TEST_SERVICE_PREFIX
                ),
                crate::http_fallback_metrics_path_ref::HttpFallbackMetricsPathRef::from(
                    constants_str::METRICS
                ),
                crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(None),
                crate::http_accept_header_maximum_bytes::HttpAcceptHeaderMaximumBytes::from(
                    1024usize
                ),
            ),
            crate::fallback_response_mode::FallbackResponseMode::MachineReadable
        );
    }
    #[test]
    fn test_invalid_or_zero_quality_json_is_not_accepted() {
        let resolve_mode = |http_optional_accept_header_ref: crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef<'_>| {
            crate::resolve_fallback_response_mode::resolve_fallback_response_mode(
                    crate::http_fallback_request_path_ref::HttpFallbackRequestPathRef::from(
                        constants_str::TEST_SIGNIN_PATH,
                    ),
                    crate::http_fallback_api_prefix_ref::HttpFallbackApiPrefixRef::from(
                        constants_str::TEST_SERVICE_PREFIX,
                    ),
                    crate::http_fallback_metrics_path_ref::HttpFallbackMetricsPathRef::from(
                        constants_str::METRICS,
                    ),
                    http_optional_accept_header_ref,
                    crate::http_accept_header_maximum_bytes::HttpAcceptHeaderMaximumBytes::from(
                        1024usize,
                    ),
                )
        };
        let invalid_accepts = [
            constants_str::TEST_ACCEPT_JSON_OUT_OF_RANGE_QUALITY,
            constants_str::TEST_ACCEPT_JSON_NONZERO_ONE_FRACTION_QUALITY,
            constants_str::TEST_ACCEPT_JSON_EXCESS_PRECISION_QUALITY,
        ];
        assert!(invalid_accepts.into_iter().all(|value| {
            let accept = http::HeaderValue::from_static(value);
            resolve_mode(
                crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(Some(
                    &accept,
                )),
            ) == crate::fallback_response_mode::FallbackResponseMode::HumanReadable
        }));
        let accept =
            http::HeaderValue::from_static(constants_str::TEST_ACCEPT_HTML_JSON_ZERO_QUALITY);
        assert_eq!(
            resolve_mode(
                crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(Some(
                    &accept,
                )),
            ),
            crate::fallback_response_mode::FallbackResponseMode::HumanReadable
        );
        let decimal_accept = http::HeaderValue::from_static(
            constants_str::TEST_ACCEPT_HTML_JSON_ZERO_DECIMAL_QUALITY,
        );
        assert_eq!(
            resolve_mode(
                crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(Some(
                    &decimal_accept,
                )),
            ),
            crate::fallback_response_mode::FallbackResponseMode::HumanReadable
        );
        assert!(
            [
                constants_str::APPLICATION_JSON,
                constants_str::TEST_ACCEPT_JSON_MINIMUM_POSITIVE_QUALITY,
                constants_str::TEST_ACCEPT_JSON_ONE_DECIMAL_QUALITY,
            ]
            .into_iter()
            .all(|value| {
                let positive_accept = http::HeaderValue::from_static(value);
                resolve_mode(
                    crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(
                        Some(&positive_accept),
                    ),
                ) == crate::fallback_response_mode::FallbackResponseMode::MachineReadable
            })
        );
    }
}
