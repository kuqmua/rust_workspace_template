#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub struct HttpOptionalAcceptHeaderRef<'value_lt>(Option<&'value_lt http::HeaderValue>);

impl<'value_lt> HttpOptionalAcceptHeaderRef<'value_lt> {
    pub(crate) const fn get(self) -> Option<&'value_lt http::HeaderValue> {
        self.0
    }
}
