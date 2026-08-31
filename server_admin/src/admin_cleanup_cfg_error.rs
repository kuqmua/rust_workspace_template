#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum AdminCleanupCfgError {
    #[error("{}", constants_str::CLEANUP_BATCH_SIZE_MUST_BE_BETWEEN_1_AND_10000)]
    BatchSizeOutOfRange,
    #[error("{}", constants_str::CLEANUP_RETENTION_MUST_BE_GREATER_THAN_ZERO)]
    RetentionMustBePositive,
}
impl From<server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error>
    for AdminCleanupCfgError
{
    fn from(
        _value: server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error,
    ) -> Self {
        Self::RetentionMustBePositive
    }
}
