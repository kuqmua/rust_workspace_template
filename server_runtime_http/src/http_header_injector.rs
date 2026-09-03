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
