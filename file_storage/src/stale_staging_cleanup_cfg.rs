#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct StaleStagingCleanupCfg {
    maximum_removed: crate::std_stale_staging_entry_limit::StdStaleStagingEntryLimit,
    maximum_scanned: crate::std_stale_staging_entry_limit::StdStaleStagingEntryLimit,
    stale_before: crate::stale_before_system_time::StaleBeforeSystemTime,
}
impl StaleStagingCleanupCfg {
    pub(crate) const fn maximum_removed(
        self,
    ) -> crate::std_stale_staging_entry_limit::StdStaleStagingEntryLimit {
        self.maximum_removed
    }

    pub(crate) const fn maximum_scanned(
        self,
    ) -> crate::std_stale_staging_entry_limit::StdStaleStagingEntryLimit {
        self.maximum_scanned
    }

    #[must_use]
    pub const fn new(
        stale_before: crate::stale_before_system_time::StaleBeforeSystemTime,
        maximum_scanned: crate::std_stale_staging_entry_limit::StdStaleStagingEntryLimit,
        maximum_removed: crate::std_stale_staging_entry_limit::StdStaleStagingEntryLimit,
    ) -> Self {
        Self {
            maximum_removed,
            maximum_scanned,
            stale_before,
        }
    }

    pub(crate) const fn stale_before(
        self,
    ) -> crate::stale_before_system_time::StaleBeforeSystemTime {
        self.stale_before
    }
}
