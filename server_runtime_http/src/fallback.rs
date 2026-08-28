#[path = "fallback_response_mode.rs"]
mod fallback_response_mode;
#[path = "http_accept_header_maximum_bytes.rs"]
mod http_accept_header_maximum_bytes;
#[path = "http_fallback_api_prefix_ref.rs"]
mod http_fallback_api_prefix_ref;
#[path = "http_fallback_metrics_path_ref.rs"]
mod http_fallback_metrics_path_ref;
#[path = "http_fallback_request_path_ref.rs"]
mod http_fallback_request_path_ref;
#[path = "http_optional_accept_header_ref.rs"]
mod http_optional_accept_header_ref;
#[path = "resolve_fallback_response_mode.rs"]
mod resolve_fallback_response_mode;

pub use fallback_response_mode::FallbackResponseMode;
pub use http_accept_header_maximum_bytes::HttpAcceptHeaderMaximumBytes;
pub use http_fallback_api_prefix_ref::HttpFallbackApiPrefixRef;
pub use http_fallback_metrics_path_ref::HttpFallbackMetricsPathRef;
pub use http_fallback_request_path_ref::HttpFallbackRequestPathRef;
pub use http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef;
pub use resolve_fallback_response_mode::resolve_fallback_response_mode;

#[cfg(test)]
mod tests {
    #[test]
    fn api_path_is_machine_readable_without_accept_header() {
        assert_eq!(
            super::resolve_fallback_response_mode(
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
            super::resolve_fallback_response_mode(
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
