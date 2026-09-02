#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub struct RedactedUrlTextRef<'value_lt>(&'value_lt str);

impl<'value_lt> RedactedUrlTextRef<'value_lt> {
    pub(crate) const fn get(self) -> &'value_lt str {
        self.0
    }
}
