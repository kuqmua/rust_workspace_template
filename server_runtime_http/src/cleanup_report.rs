#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct CleanupReport {
    pub(super) batches: super::CleanupBatchCount,
    pub(super) rows: super::CleanupRows,
    pub(super) completion: super::CleanupCompletion,
}

impl CleanupReport {
    #[must_use]
    pub const fn batches(self) -> super::CleanupBatchCount {
        self.batches
    }

    #[must_use]
    pub const fn completion(self) -> super::CleanupCompletion {
        self.completion
    }

    #[must_use]
    pub const fn rows(self) -> super::CleanupRows {
        self.rows
    }
}
