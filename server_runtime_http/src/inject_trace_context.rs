pub fn inject_trace_context(
    opentelemetry_context: &crate::opentelemetry_context::OpentelemetryContext,
    mut http_opentelemetry_header_map_mut: crate::http_opentelemetry_header_map_mut::HttpOpentelemetryHeaderMapMut<'_>,
) {
    opentelemetry::global::get_text_map_propagator(|propagator| {
        propagator.inject_context(
            opentelemetry_context,
            &mut crate::http_header_injector::HttpHeaderInjector::from(
                &mut **http_opentelemetry_header_map_mut,
            ),
        );
    });
}
