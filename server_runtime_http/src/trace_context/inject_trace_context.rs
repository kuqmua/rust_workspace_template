pub fn inject_trace_context(
    context: &super::OpentelemetryContext,
    mut headers: super::HttpOpentelemetryHeaderMapMut<'_>,
) {
    opentelemetry::global::get_text_map_propagator(|propagator| {
        propagator.inject_context(
            &context.0,
            &mut super::HttpHeaderInjector::from(&mut **headers),
        );
    });
}
