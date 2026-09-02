#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
)]
#[getters(bare)]
pub struct StaleStagingCleanupReport {
    #[getters(copy)]
    removed: crate::std_stale_staging_entry_count::StdStaleStagingEntryCount,
    #[getters(copy)]
    scanned: crate::std_stale_staging_entry_count::StdStaleStagingEntryCount,
}
impl StaleStagingCleanupReport {
    pub(crate) const fn record_removed(&mut self) {
        self.removed.increment();
    }

    pub(crate) const fn record_scanned(&mut self) {
        self.scanned.increment();
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
