#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct ChildProcessSetMaximumNonZeroUsize(std::num::NonZeroUsize);

impl ChildProcessSetMaximumNonZeroUsize {
    pub(crate) const fn get(self) -> usize {
        self.0.get()
    }
}
