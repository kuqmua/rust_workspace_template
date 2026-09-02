#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
)]
pub struct CleanupBatchSize(std::num::NonZeroU64);

impl CleanupBatchSize {
    pub(crate) const fn get(self) -> u64 {
        self.0.get()
    }
}

impl TryFrom<u64> for CleanupBatchSize {
    type Error = crate::cleanup_batch_size_error::CleanupBatchSizeError;

    fn try_from(u64: u64) -> Result<Self, Self::Error> {
        std::num::NonZeroU64::new(u64)
            .map(Self::from)
            .ok_or(crate::cleanup_batch_size_error::CleanupBatchSizeError::Zero)
    }
}
