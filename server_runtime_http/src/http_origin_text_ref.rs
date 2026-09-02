#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub(super) struct HttpOriginTextRef<'text>(&'text str);

impl<'text> HttpOriginTextRef<'text> {
    pub(crate) const fn get(self) -> &'text str {
        self.0
    }
}
