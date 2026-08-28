#[path = "extract_remote_trace_context.rs"]
mod extract_remote_trace_context;
#[path = "http_header_extractor.rs"]
mod http_header_extractor;
#[path = "http_header_injector.rs"]
mod http_header_injector;
#[path = "http_host_ref.rs"]
mod http_host_ref;
#[path = "http_method_ref.rs"]
mod http_method_ref;
#[path = "http_opentelemetry_header_map_mut.rs"]
mod http_opentelemetry_header_map_mut;
#[path = "http_opentelemetry_header_map_ref.rs"]
mod http_opentelemetry_header_map_ref;
#[path = "http_trace_parent.rs"]
mod http_trace_parent;
#[path = "http_trace_parent_error.rs"]
mod http_trace_parent_error;
#[path = "http_trace_state.rs"]
mod http_trace_state;
#[path = "http_trace_state_error.rs"]
mod http_trace_state_error;
#[path = "inject_trace_context.rs"]
mod inject_trace_context;
#[path = "opentelemetry_context.rs"]
mod opentelemetry_context;
#[path = "outbound_trace_context.rs"]
mod outbound_trace_context;
#[path = "reqwest_request.rs"]
mod reqwest_request;
#[path = "reqwest_request_builder.rs"]
mod reqwest_request_builder;

pub use extract_remote_trace_context::extract_remote_trace_context;
use http_header_extractor::HttpHeaderExtractor;
use http_header_injector::HttpHeaderInjector;
pub use http_host_ref::HttpHostRef;
pub use http_method_ref::HttpMethodRef;
pub use http_opentelemetry_header_map_mut::HttpOpentelemetryHeaderMapMut;
pub use http_opentelemetry_header_map_ref::HttpOpentelemetryHeaderMapRef;
pub use http_trace_parent::HttpTraceParent;
pub use http_trace_parent_error::HttpTraceParentError;
pub use http_trace_state::HttpTraceState;
pub use http_trace_state_error::HttpTraceStateError;
pub use inject_trace_context::inject_trace_context;
pub use opentelemetry_context::OpentelemetryContext;
pub use outbound_trace_context::OutboundTraceContext;
pub use reqwest_request::ReqwestRequest;
pub use reqwest_request_builder::ReqwestRequestBuilder;

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
