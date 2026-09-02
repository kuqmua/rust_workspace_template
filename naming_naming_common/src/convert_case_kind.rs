#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::FromInner,
)]
pub(super) struct ConvertCaseKind(convert_case::Case<'static>);
impl ConvertCaseKind {
    pub(super) const fn get(self) -> convert_case::Case<'static> {
        self.0
    }
}
