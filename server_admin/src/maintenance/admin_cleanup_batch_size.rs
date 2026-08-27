use super::AdminCleanupCfgError;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupBatchSize(i64);
impl TryFrom<i64> for AdminCleanupBatchSize {
    type Error = AdminCleanupCfgError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if (constants_i64::ONE..=10_000i64).contains(&value) {
            Ok(Self(value))
        } else {
            Err(AdminCleanupCfgError::BatchSizeOutOfRange)
        }
    }
}
impl AdminCleanupBatchSize {
    pub(crate) const fn get(self) -> i64 {
        self.0
    }
}
