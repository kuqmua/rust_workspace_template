pub use super::fallback_response_mode::FallbackResponseMode;
pub use super::http_accept_header_maximum_bytes::HttpAcceptHeaderMaximumBytes;
pub use super::http_fallback_api_prefix_ref::HttpFallbackApiPrefixRef;
pub use super::http_fallback_metrics_path_ref::HttpFallbackMetricsPathRef;
pub use super::http_fallback_request_path_ref::HttpFallbackRequestPathRef;
pub use super::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef;
pub use super::resolve_fallback_response_mode::resolve_fallback_response_mode;
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

// Root-owned module compatibility wrappers.
mod fallback_response_mode {
    pub use super::super::fallback_response_mode::*;
}
mod http_accept_header_maximum_bytes {
    pub use super::super::http_accept_header_maximum_bytes::*;
}
mod http_fallback_api_prefix_ref {
    pub use super::super::http_fallback_api_prefix_ref::*;
}
mod http_fallback_metrics_path_ref {
    pub use super::super::http_fallback_metrics_path_ref::*;
}
mod http_fallback_request_path_ref {
    pub use super::super::http_fallback_request_path_ref::*;
}
mod http_optional_accept_header_ref {
    pub use super::super::http_optional_accept_header_ref::*;
}
mod resolve_fallback_response_mode {
    pub use super::super::resolve_fallback_response_mode::*;
}
