#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct ChildDiagnosticMaximumNonZeroUsize(std::num::NonZeroUsize);

impl ChildDiagnosticMaximumNonZeroUsize {
    pub(crate) const fn get(self) -> usize {
        self.0.get()
    }
}
