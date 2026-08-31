#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpCookieHeadersRef<'value_lt>(&'value_lt http::HeaderMap);

impl<'value_lt> HttpCookieHeadersRef<'value_lt> {
    pub(crate) const fn get(self) -> &'value_lt http::HeaderMap {
        self.0
    }
}
