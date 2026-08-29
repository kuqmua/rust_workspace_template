#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct CleanupReport {
    pub(super) batches: crate::cleanup_batch_count::CleanupBatchCount,
    pub(super) rows: crate::cleanup_rows::CleanupRows,
    pub(super) completion: crate::cleanup_completion::CleanupCompletion,
}

impl CleanupReport {
    #[must_use]
    pub const fn batches(self) -> crate::cleanup_batch_count::CleanupBatchCount {
        self.batches
    }

    #[must_use]
    pub const fn completion(self) -> crate::cleanup_completion::CleanupCompletion {
        self.completion
    }

    #[must_use]
    pub const fn rows(self) -> crate::cleanup_rows::CleanupRows {
        self.rows
    }
}
