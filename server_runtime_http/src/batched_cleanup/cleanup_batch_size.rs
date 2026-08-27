#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct CleanupBatchSize(pub(super) super::CleanupBatchSizeNonZeroU64);

impl TryFrom<u64> for CleanupBatchSize {
    type Error = super::CleanupBatchSizeError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        std::num::NonZeroU64::new(value)
            .map(Self::from)
            .ok_or(super::CleanupBatchSizeError)
    }
}

impl From<std::num::NonZeroU64> for CleanupBatchSize {
    fn from(value: std::num::NonZeroU64) -> Self {
        Self(super::CleanupBatchSizeNonZeroU64::from(value))
    }
}
