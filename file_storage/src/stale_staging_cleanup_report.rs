#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::domain_types::StdStaleStagingEntryCount;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Default, Eq, PartialEq,
)]
pub struct StaleStagingCleanupReport {
    pub(super) removed: StdStaleStagingEntryCount,
    pub(super) scanned: StdStaleStagingEntryCount,
}
impl StaleStagingCleanupReport {
    pub(crate) const fn record_removed(&mut self) {
        self.removed.0 = self.removed.0.saturating_add(constants_usize::ONE);
    }

    pub(crate) const fn record_scanned(&mut self) {
        self.scanned.0 = self.scanned.0.saturating_add(constants_usize::ONE);
    }

    #[must_use]
    pub const fn removed(self) -> StdStaleStagingEntryCount {
        self.removed
    }
    #[must_use]
    pub const fn scanned(self) -> StdStaleStagingEntryCount {
        self.scanned
    }
}
