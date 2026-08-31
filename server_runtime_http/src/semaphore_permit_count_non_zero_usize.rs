#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct SemaphorePermitCountNonZeroUsize(std::num::NonZeroUsize);

impl SemaphorePermitCountNonZeroUsize {
    pub(crate) const fn get(self) -> usize {
        self.0.get()
    }
}
