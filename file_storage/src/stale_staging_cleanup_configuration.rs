#[derive(proc_macro_getters::Getters, proc_macro_new::New)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub struct StaleStagingCleanupConfiguration {
    #[getters(copy)]
    #[constructor(order = 2)]
    maximum_removed: crate::std_stale_staging_entry_limit::StdStaleStagingEntryLimit,
    #[getters(copy)]
    #[constructor(order = 1)]
    maximum_scanned: crate::std_stale_staging_entry_limit::StdStaleStagingEntryLimit,
    #[getters(copy)]
    #[constructor(order = 0)]
    stale_before: crate::stale_before_system_time::StaleBeforeSystemTime,
}
