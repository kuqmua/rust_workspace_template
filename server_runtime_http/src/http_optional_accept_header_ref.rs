#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpOptionalAcceptHeaderRef<'value_lt>(Option<&'value_lt http::HeaderValue>);

impl<'value_lt> HttpOptionalAcceptHeaderRef<'value_lt> {
    pub(crate) const fn get(self) -> Option<&'value_lt http::HeaderValue> {
        self.0
    }
}
