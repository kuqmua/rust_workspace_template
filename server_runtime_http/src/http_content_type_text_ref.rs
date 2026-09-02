#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub struct HttpContentTypeTextRef<'value_lt>(Option<&'value_lt str>);

impl<'value_lt> HttpContentTypeTextRef<'value_lt> {
    pub(crate) const fn get(self) -> Option<&'value_lt str> {
        self.0
    }
}
