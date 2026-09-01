#[derive(generate_accessor::Getters, generate_constructor::New)]
#[getters(bare)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct StaleStagingCleanupCfg {
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
