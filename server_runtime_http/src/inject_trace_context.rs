pub fn inject_trace_context(
    context: &crate::opentelemetry_context::OpentelemetryContext,
    mut headers: crate::http_opentelemetry_header_map_mut::HttpOpentelemetryHeaderMapMut<'_>,
) {
    opentelemetry::global::get_text_map_propagator(|propagator| {
        propagator.inject_context(
            &context.0,
            &mut crate::http_header_injector::HttpHeaderInjector::from(&mut **headers),
        );
    });
}
