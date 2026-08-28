use crate::domain_types::StaleStagingCleanupCfgError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, newtype::GetInner,
)]
pub struct StdStaleStagingEntryLimit(usize);
impl TryFrom<usize> for StdStaleStagingEntryLimit {
    type Error = StaleStagingCleanupCfgError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == constants_usize::ZERO || value > 10_000usize {
            Err(StaleStagingCleanupCfgError)
        } else {
            Ok(Self(value))
        }
    }
}
