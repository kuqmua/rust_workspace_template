#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    Default,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
pub(super) struct AnalyzerCount(usize);
impl AnalyzerCount {
    pub(super) fn saturating_dec(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }
    pub(super) fn saturating_inc(&mut self) {
        self.0 = self.0.saturating_add(1);
    }
}
