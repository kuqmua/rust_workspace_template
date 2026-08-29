#[must_use]
pub fn extract_remote_trace_context(
    headers: crate::http_opentelemetry_header_map_ref::HttpOpentelemetryHeaderMapRef<'_>,
) -> crate::opentelemetry_context::OpentelemetryContext {
    opentelemetry::global::get_text_map_propagator(|propagator| {
        crate::opentelemetry_context::OpentelemetryContext::from(propagator.extract(
            &crate::http_header_extractor::HttpHeaderExtractor::from(headers.0),
        ))
    })
}
