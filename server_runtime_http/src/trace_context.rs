pub use super::extract_remote_trace_context::extract_remote_trace_context;
use super::http_header_extractor::HttpHeaderExtractor;
use super::http_header_injector::HttpHeaderInjector;
pub use super::http_host_ref::HttpHostRef;
pub use super::http_method_ref::HttpMethodRef;
pub use super::http_opentelemetry_header_map_mut::HttpOpentelemetryHeaderMapMut;
pub use super::http_opentelemetry_header_map_ref::HttpOpentelemetryHeaderMapRef;
pub use super::http_trace_parent::HttpTraceParent;
pub use super::http_trace_parent_error::HttpTraceParentError;
pub use super::http_trace_state::HttpTraceState;
pub use super::http_trace_state_error::HttpTraceStateError;
pub use super::inject_trace_context::inject_trace_context;
pub use super::opentelemetry_context::OpentelemetryContext;
pub use super::outbound_trace_context::OutboundTraceContext;
pub use super::reqwest_request::ReqwestRequest;
pub use super::reqwest_request_builder::ReqwestRequestBuilder;
#[cfg(test)]
mod tests {
    #[test]
    #[cfg_attr(
        miri,
        ignore = "native TLS initialization calls OpenSSL functions that Miri does not support"
    )]
    fn validates_and_applies_w3c_trace_context() {
        let trace_parent =
            super::HttpTraceParent::try_from(constants_str::TRACEPARENT_TEST_VALUE.to_owned())
                .expect("6b490bf8 validates_and_applies_w3c_trace_context invariant must hold");
        let trace_state =
            super::HttpTraceState::try_from(constants_str::TRACESTATE_TEST_VALUE.to_owned())
                .expect("b82fb9ef validates_and_applies_w3c_trace_context invariant must hold");
        let request_id = crate::domain_types::RequestId::try_from(
            constants_str::REQUEST_ID_TEST_VALUE.to_owned(),
        )
        .expect("50c01ea8 validates_and_applies_w3c_trace_context invariant must hold");
        let client = crate::domain_types::ReqwestClient::try_new(
            crate::domain_types::ReqwestClientPolicy::new(
                crate::domain_types::ReqwestConnectTimeoutDuration::try_from(
                    std::time::Duration::from_secs(1u64),
                )
                .expect("ce032a9f validates_and_applies_w3c_trace_context invariant must hold"),
                crate::domain_types::ReqwestRequestTimeoutDuration::try_from(
                    std::time::Duration::from_secs(2u64),
                )
                .expect("a1dabed3 validates_and_applies_w3c_trace_context invariant must hold"),
            ),
        )
        .expect("8ded9d63 validates_and_applies_w3c_trace_context invariant must hold");
        let request_builder: reqwest::RequestBuilder =
            super::OutboundTraceContext::new(trace_parent, Some(trace_state), Some(request_id))
                .apply(
                    reqwest::Client::from(client)
                        .get(constants_str::HTTPS_EXAMPLE_COM)
                        .into(),
                )
                .into();
        let request = request_builder
            .build()
            .expect("1574578f validates_and_applies_w3c_trace_context invariant must hold");
        assert_eq!(
            request.headers()[constants_str::TRACESTATE],
            constants_str::TRACESTATE_TEST_VALUE
        );
        assert_eq!(
            request.headers()[constants_str::X_REQUEST_ID],
            constants_str::REQUEST_ID_TEST_VALUE
        );
    }

    #[test]
    fn rejects_zero_identifiers() {
        assert_eq!(
            super::HttpTraceParent::try_from(
                constants_str::TRACEPARENT_ZERO_TRACE_ID_TEST_VALUE.to_owned(),
            ),
            Err(super::HttpTraceParentError::ZeroTraceId)
        );
    }

    #[test]
    fn extracts_valid_w3c_parent_context() {
        opentelemetry::global::set_text_map_propagator(
            opentelemetry_sdk::propagation::TraceContextPropagator::new(),
        );
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            http::HeaderName::from_static(constants_str::TRACEPARENT),
            http::HeaderValue::from_static(constants_str::TRACEPARENT_TEST_VALUE),
        );
        let context = super::extract_remote_trace_context(
            super::HttpOpentelemetryHeaderMapRef::from(&headers),
        );
        let span = opentelemetry::trace::TraceContextExt::span(&context.0);
        assert!(span.span_context().is_remote());
        let expected_trace_id = constants_str::TRACEPARENT_TEST_VALUE
            .get(3usize..35usize)
            .expect("65aa5eca extracts_valid_w3c_parent_context invariant must hold");
        assert_eq!(
            span.span_context().trace_id().to_string(),
            expected_trace_id
        );
    }

    #[test]
    fn injects_w3c_context() {
        opentelemetry::global::set_text_map_propagator(
            opentelemetry_sdk::propagation::TraceContextPropagator::new(),
        );
        let headers = http::HeaderMap::from_iter([
            (
                http::HeaderName::from_static(constants_str::TRACEPARENT),
                http::HeaderValue::from_static(constants_str::TRACEPARENT_TEST_VALUE),
            ),
            (
                http::HeaderName::from_static(constants_str::TRACESTATE),
                http::HeaderValue::from_static(constants_str::TRACESTATE_TEST_VALUE),
            ),
        ]);
        let context = super::extract_remote_trace_context(
            super::HttpOpentelemetryHeaderMapRef::from(&headers),
        );
        let mut injected_headers = http::HeaderMap::new();
        super::inject_trace_context(
            &context,
            super::HttpOpentelemetryHeaderMapMut::from(&mut injected_headers),
        );
        assert_eq!(
            injected_headers.get(constants_str::TRACEPARENT),
            Some(&http::HeaderValue::from_static(
                constants_str::TRACEPARENT_TEST_VALUE
            ))
        );
        assert_eq!(
            injected_headers.get(constants_str::TRACESTATE),
            Some(&http::HeaderValue::from_static(
                constants_str::TRACESTATE_TEST_VALUE
            ))
        );
    }
}

// Root-owned module compatibility wrappers.
mod extract_remote_trace_context {
    pub use super::super::extract_remote_trace_context::*;
}
mod http_header_extractor {
    pub use super::super::http_header_extractor::*;
}
mod http_header_injector {
    pub use super::super::http_header_injector::*;
}
mod http_host_ref {
    pub use super::super::http_host_ref::*;
}
mod http_method_ref {
    pub use super::super::http_method_ref::*;
}
mod http_opentelemetry_header_map_mut {
    pub use super::super::http_opentelemetry_header_map_mut::*;
}
mod http_opentelemetry_header_map_ref {
    pub use super::super::http_opentelemetry_header_map_ref::*;
}
mod http_trace_parent {
    pub use super::super::http_trace_parent::*;
}
mod http_trace_parent_error {
    pub use super::super::http_trace_parent_error::*;
}
mod http_trace_state {
    pub use super::super::http_trace_state::*;
}
mod http_trace_state_error {
    pub use super::super::http_trace_state_error::*;
}
mod inject_trace_context {
    pub use super::super::inject_trace_context::*;
}
mod opentelemetry_context {
    pub use super::super::opentelemetry_context::*;
}
mod outbound_trace_context {
    pub use super::super::outbound_trace_context::*;
}
mod reqwest_request {
    pub use super::super::reqwest_request::*;
}
mod reqwest_request_builder {
    pub use super::super::reqwest_request_builder::*;
}
