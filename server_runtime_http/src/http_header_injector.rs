#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(super) struct HttpHeaderInjector<'headers_lt>(&'headers_lt mut http::HeaderMap);

impl opentelemetry::propagation::Injector for HttpHeaderInjector<'_> {
    fn set(&mut self, key: &str, value: String) {
        let Ok(header_name) = http::HeaderName::try_from(key) else {
            return;
        };
        let Ok(header_value) = http::HeaderValue::try_from(value) else {
            return;
        };
        let _previous_value = self.0.insert(header_name, header_value);
    }
}
