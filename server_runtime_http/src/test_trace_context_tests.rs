#[cfg(test)]
mod tests {
    #[test]
    fn test_trace_state_ascii_character_policy_preserves_printable_text() {
        assert!((0u8..=127u8).all(|byte| {
            let text = char::from(byte).to_string();
            let result = crate::http_trace_state::HttpTraceState::try_from(text.clone());
            if (32u8..=126u8).contains(&byte) {
                result.is_ok_and(|http_trace_state| http_trace_state.as_ref() == text)
            } else {
                result == Err(crate::http_trace_state_error::HttpTraceStateError::Invalid)
            }
        }));
        assert_eq!(
            crate::http_trace_state::HttpTraceState::try_from('\u{e9}'.to_string()),
            Err(crate::http_trace_state_error::HttpTraceStateError::Invalid)
        );
    }

    #[test]
    fn test_trace_state_exact_size_limits() {
        assert!(
            [0usize, 1usize, 511usize, 512usize, 513usize]
                .into_iter()
                .all(|length| {
                    let text = constants_str::X.repeat(length);
                    let result = crate::http_trace_state::HttpTraceState::try_from(text.clone());
                    if (1usize..=512usize).contains(&length) {
                        result.is_ok_and(|http_trace_state| http_trace_state.as_ref() == text)
                    } else {
                        result == Err(crate::http_trace_state_error::HttpTraceStateError::Invalid)
                    }
                })
        );
    }

    #[test]
    fn test_trace_parent_rejects_invalid_symbols_at_every_position() {
        assert!((0usize..55usize).all(|position| {
            ['A', 'x'].into_iter().all(|symbol| {
                let mut text = constants_str::TRACEPARENT_TEST_VALUE.to_owned();
                text.replace_range(
                    position..position.saturating_add(1usize),
                    &symbol.to_string(),
                );
                crate::http_trace_parent::HttpTraceParent::try_from(text)
                    == Err(crate::http_trace_parent_error::HttpTraceParentError::Format)
            })
        }));
        assert!([54usize, 56usize].into_iter().all(|length| {
            let text = constants_str::TRACEPARENT_TEST_VALUE
                .chars()
                .chain(std::iter::once('0'))
                .take(length)
                .collect::<String>();
            crate::http_trace_parent::HttpTraceParent::try_from(text)
                == Err(crate::http_trace_parent_error::HttpTraceParentError::Format)
        }));
    }

    #[test]
    fn test_trace_parent_zero_parent_and_zero_trace_error_precedence() {
        let mut zero_parent = constants_str::TRACEPARENT_TEST_VALUE.to_owned();
        zero_parent.replace_range(36usize..52usize, &'0'.to_string().repeat(16usize));
        assert_eq!(
            crate::http_trace_parent::HttpTraceParent::try_from(zero_parent.clone()),
            Err(crate::http_trace_parent_error::HttpTraceParentError::ZeroParentId)
        );
        zero_parent.replace_range(3usize..35usize, &'0'.to_string().repeat(32usize));
        assert_eq!(
            crate::http_trace_parent::HttpTraceParent::try_from(zero_parent),
            Err(crate::http_trace_parent_error::HttpTraceParentError::ZeroTraceId)
        );
    }

    #[test]
    fn test_trace_parent_preserves_every_valid_flag_byte() {
        assert!(
            constants_str::TRACEPARENT_TEST_VALUE
                .get(..53usize)
                .is_some_and(|prefix| {
                    (0u16..256u16).all(|flags| {
                        let text = format!("{prefix}{flags:02x}");
                        crate::http_trace_parent::HttpTraceParent::try_from(text.clone())
                            .is_ok_and(|http_trace_parent| http_trace_parent.as_ref() == text)
                    })
                })
        );
    }

    #[test]
    fn test_rejects_zero_identifiers() {
        assert_eq!(
            crate::http_trace_parent::HttpTraceParent::try_from(
                constants_str::TRACEPARENT_ZERO_TRACE_ID_TEST_VALUE.to_owned(),
            ),
            Err(crate::http_trace_parent_error::HttpTraceParentError::ZeroTraceId)
        );
    }

    #[test]
    fn test_extracts_valid_w3c_parent_context() {
        opentelemetry::global::set_text_map_propagator(
            opentelemetry_sdk::propagation::TraceContextPropagator::new(),
        );
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            http::HeaderName::from_static(constants_str::TRACEPARENT),
            http::HeaderValue::from_static(constants_str::TRACEPARENT_TEST_VALUE),
        );
        let context = crate::extract_remote_trace_context::extract_remote_trace_context(
            crate::http_opentelemetry_header_map_ref::HttpOpentelemetryHeaderMapRef::from(&headers),
        );
        let span = opentelemetry::trace::TraceContextExt::span(&*context);
        assert!(span.span_context().is_remote());
        let expected_trace_id = constants_str::TRACEPARENT_TEST_VALUE
            .get(3usize..35usize)
            .expect(constants_str::DIAGNOSTIC_65AA5ECA);
        assert_eq!(
            span.span_context().trace_id().to_string(),
            expected_trace_id
        );
    }

    #[test]
    fn test_injects_w3c_context() {
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
        let context = crate::extract_remote_trace_context::extract_remote_trace_context(
            crate::http_opentelemetry_header_map_ref::HttpOpentelemetryHeaderMapRef::from(&headers),
        );
        let mut injected_headers = http::HeaderMap::new();
        crate::inject_trace_context::inject_trace_context(
            &context,
            crate::http_opentelemetry_header_map_mut::HttpOpentelemetryHeaderMapMut::from(
                &mut injected_headers,
            ),
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
