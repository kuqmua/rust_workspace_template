#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpHeaderMapRef<'lt>(&'lt http::HeaderMap);

impl<'lt> HttpHeaderMapRef<'lt> {
    pub(crate) const fn get(self) -> &'lt http::HeaderMap {
        self.0
    }
}
