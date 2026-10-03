#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct HttpHeaderInjector<'headers_lt>(&'headers_lt mut http::HeaderMap);

impl opentelemetry::propagation::Injector for HttpHeaderInjector<'_> {
    fn set(&mut self, str: &str, string: String) {
        let Ok(header_name) = http::HeaderName::try_from(str) else {
            return;
        };
        let Ok(header_value) = http::HeaderValue::try_from(string) else {
            return;
        };
        let _previous_value = self.0.insert(header_name, header_value);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trace_header_injection_replaces_valid_values_and_preserves_state_on_invalid_input() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            constants_str::TRACEPARENT,
            http::HeaderValue::from_static(constants_str::ABC),
        );
        let _previous_request_id = headers.insert(
            constants_str::X_REQUEST_ID,
            http::HeaderValue::from_static(constants_str::ABC),
        );
        let mut invalid_injector =
            crate::http_header_injector::HttpHeaderInjector::from(&mut headers);
        opentelemetry::propagation::Injector::set(
            &mut invalid_injector,
            constants_str::EMPTY,
            constants_str::X.to_owned(),
        );
        opentelemetry::propagation::Injector::set(
            &mut invalid_injector,
            constants_str::TRACEPARENT,
            String::from(char::from(0u8)),
        );

        assert_eq!(headers.len(), 2usize);
        assert_eq!(
            headers.get(constants_str::TRACEPARENT),
            Some(&http::HeaderValue::from_static(constants_str::ABC))
        );
        let mut valid_injector =
            crate::http_header_injector::HttpHeaderInjector::from(&mut headers);
        opentelemetry::propagation::Injector::set(
            &mut valid_injector,
            &constants_str::TRACEPARENT.to_ascii_uppercase(),
            constants_str::X.to_owned(),
        );
        opentelemetry::propagation::Injector::set(
            &mut valid_injector,
            constants_str::TRACESTATE,
            constants_str::EMPTY.to_owned(),
        );

        assert_eq!(headers.len(), 3usize);
        assert_eq!(
            headers.get(constants_str::TRACEPARENT),
            Some(&http::HeaderValue::from_static(constants_str::X))
        );
        assert_eq!(
            headers.get(constants_str::X_REQUEST_ID),
            Some(&http::HeaderValue::from_static(constants_str::ABC))
        );
        assert_eq!(
            headers.get(constants_str::TRACESTATE),
            Some(&http::HeaderValue::from_static(constants_str::EMPTY))
        );
    }
}
