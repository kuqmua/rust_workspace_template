#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct CleanupBatchSize(pub(super) std::num::NonZeroU64);

impl TryFrom<u64> for CleanupBatchSize {
    type Error = crate::cleanup_batch_size_error::CleanupBatchSizeError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        std::num::NonZeroU64::new(value)
            .map(Self::from)
            .ok_or(crate::cleanup_batch_size_error::CleanupBatchSizeError)
    }
}
