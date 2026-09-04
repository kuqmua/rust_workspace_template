#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_getters::Getters,
)]
pub struct AdminCleanupBatchSize(#[getters(copy)] i64);
impl TryFrom<i64> for AdminCleanupBatchSize {
    type Error = crate::admin_cleanup_configuration_error::AdminCleanupConfigurationError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if (constants_i64::ONE..=10_000i64).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::admin_cleanup_configuration_error::AdminCleanupConfigurationError::BatchSizeOutOfRange)
        }
    }
}
