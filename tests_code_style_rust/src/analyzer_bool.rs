#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    Default,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
pub(super) struct AnalyzerBool(bool);
impl AnalyzerBool {
    pub(super) fn set_true(&mut self) {
        self.0 = true;
    }
}
