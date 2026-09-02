#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub struct HttpCookieHeadersRef<'value_lt>(&'value_lt http::HeaderMap);

impl<'value_lt> HttpCookieHeadersRef<'value_lt> {
    pub(crate) const fn get(self) -> &'value_lt http::HeaderMap {
        self.0
    }
}
