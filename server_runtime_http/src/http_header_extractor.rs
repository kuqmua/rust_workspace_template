#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct HttpHeaderExtractor<'headers_lt>(&'headers_lt http::HeaderMap);

impl opentelemetry::propagation::Extractor for HttpHeaderExtractor<'_> {
    fn get(&self, str: &str) -> Option<&str> {
        let value = self.0.get(str)?;
        value.to_str().ok()
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(http::HeaderName::as_str).collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trace_header_extraction_handles_missing_binary_and_repeated_values() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            constants_str::TRACEPARENT,
            http::HeaderValue::from_static(constants_str::ABC),
        );
        let appended = headers.append(
            constants_str::TRACEPARENT,
            http::HeaderValue::from_static(constants_str::X),
        );
        assert!(appended);
        let binary_result = http::HeaderValue::from_bytes(&[u8::MAX]);
        assert!(binary_result.is_ok());
        let Ok(binary) = binary_result else {
            return;
        };
        let _previous_tracestate = headers.insert(constants_str::TRACESTATE, binary);
        let extractor = crate::http_header_extractor::HttpHeaderExtractor::from(&headers);
        assert_eq!(
            opentelemetry::propagation::Extractor::get(
                &extractor,
                &constants_str::TRACEPARENT.to_ascii_uppercase()
            ),
            Some(constants_str::ABC)
        );
        assert_eq!(
            opentelemetry::propagation::Extractor::get(&extractor, constants_str::TRACESTATE),
            None
        );
        assert_eq!(
            opentelemetry::propagation::Extractor::get(&extractor, constants_str::X_REQUEST_ID),
            None
        );
        let keys = opentelemetry::propagation::Extractor::keys(&extractor);
        assert_eq!(keys.len(), 2usize);
        assert!(keys.contains(&constants_str::TRACEPARENT));
        assert!(keys.contains(&constants_str::TRACESTATE));
    }
}
