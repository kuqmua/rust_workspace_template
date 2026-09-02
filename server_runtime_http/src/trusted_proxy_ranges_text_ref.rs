#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub struct TrustedProxyRangesTextRef<'text_lt>(&'text_lt str);

impl<'text_lt> TrustedProxyRangesTextRef<'text_lt> {
    pub(crate) const fn get(self) -> &'text_lt str {
        self.0
    }
}
