#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct ChildDiagnosticMaximumNonZeroUsize(std::num::NonZeroUsize);

impl ChildDiagnosticMaximumNonZeroUsize {
    pub(crate) const fn get(self) -> usize {
        self.0.get()
    }
}
