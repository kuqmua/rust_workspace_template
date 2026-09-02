#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::GetInner,
)]
pub struct StdStaleStagingEntryLimit(usize);
impl TryFrom<usize> for StdStaleStagingEntryLimit {
    type Error = crate::stale_staging_cleanup_cfg_error::StaleStagingCleanupCfgError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == constants_usize::ZERO || value > 10_000usize {
            Err(crate::stale_staging_cleanup_cfg_error::StaleStagingCleanupCfgError::InvalidLimit)
        } else {
            Ok(Self(value))
        }
    }
}
