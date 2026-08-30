#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Default, Eq, PartialEq,
)]
pub struct StaleStagingCleanupReport {
    removed: crate::std_stale_staging_entry_count::StdStaleStagingEntryCount,
    scanned: crate::std_stale_staging_entry_count::StdStaleStagingEntryCount,
}
impl StaleStagingCleanupReport {
    pub(crate) const fn record_removed(&mut self) {
        self.removed.increment();
    }

    pub(crate) const fn record_scanned(&mut self) {
        self.scanned.increment();
    }

    #[must_use]
    pub const fn removed(self) -> crate::std_stale_staging_entry_count::StdStaleStagingEntryCount {
        self.removed
    }
    #[must_use]
    pub const fn scanned(self) -> crate::std_stale_staging_entry_count::StdStaleStagingEntryCount {
        self.scanned
    }
}

impl
    From<(
        crate::std_stale_staging_entry_count::StdStaleStagingEntryCount,
        crate::std_stale_staging_entry_count::StdStaleStagingEntryCount,
    )> for StaleStagingCleanupReport
{
    fn from(
        value: (
            crate::std_stale_staging_entry_count::StdStaleStagingEntryCount,
            crate::std_stale_staging_entry_count::StdStaleStagingEntryCount,
        ),
    ) -> Self {
        Self {
            removed: value.0,
            scanned: value.1,
        }
    }
}
