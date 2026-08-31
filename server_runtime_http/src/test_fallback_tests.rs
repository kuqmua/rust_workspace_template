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
    fn test_zero_quality_json_is_not_accepted() {
        let accept =
            http::HeaderValue::from_static(constants_str::TEST_ACCEPT_HTML_JSON_ZERO_QUALITY);
        assert_eq!(
            crate::resolve_fallback_response_mode::resolve_fallback_response_mode(
                crate::http_fallback_request_path_ref::HttpFallbackRequestPathRef::from(
                    constants_str::TEST_SIGNIN_PATH
                ),
                crate::http_fallback_api_prefix_ref::HttpFallbackApiPrefixRef::from(
                    constants_str::TEST_SERVICE_PREFIX
                ),
                crate::http_fallback_metrics_path_ref::HttpFallbackMetricsPathRef::from(
                    constants_str::METRICS
                ),
                crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(Some(
                    &accept
                )),
                crate::http_accept_header_maximum_bytes::HttpAcceptHeaderMaximumBytes::from(
                    1024usize
                ),
            ),
            crate::fallback_response_mode::FallbackResponseMode::HumanReadable
        );
    }
}

// Root-owned module compatibility wrappers.
