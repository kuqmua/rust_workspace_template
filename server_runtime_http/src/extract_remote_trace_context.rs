#[must_use]
pub fn extract_remote_trace_context(
    headers: super::HttpOpentelemetryHeaderMapRef<'_>,
) -> super::OpentelemetryContext {
    opentelemetry::global::get_text_map_propagator(|propagator| {
        super::OpentelemetryContext::from(
            propagator.extract(&super::HttpHeaderExtractor::from(headers.0)),
        )
    })
}
