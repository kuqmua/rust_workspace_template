use super::domain_types::{StaleBeforeSystemTime, StdStaleStagingEntryLimit};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct StaleStagingCleanupCfg {
    maximum_removed: StdStaleStagingEntryLimit,
    maximum_scanned: StdStaleStagingEntryLimit,
    stale_before: StaleBeforeSystemTime,
}
impl StaleStagingCleanupCfg {
    pub(crate) const fn maximum_removed(self) -> StdStaleStagingEntryLimit {
        self.maximum_removed
    }

    pub(crate) const fn maximum_scanned(self) -> StdStaleStagingEntryLimit {
        self.maximum_scanned
    }

    #[must_use]
    pub const fn new(
        stale_before: StaleBeforeSystemTime,
        maximum_scanned: StdStaleStagingEntryLimit,
        maximum_removed: StdStaleStagingEntryLimit,
    ) -> Self {
        Self {
            maximum_removed,
            maximum_scanned,
            stale_before,
        }
    }

    pub(crate) const fn stale_before(self) -> StaleBeforeSystemTime {
        self.stale_before
    }
}
