#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkAdminPasswordHashConcurrencyError {
    #[error("administrator password hash concurrency must be greater than zero")]
    IsZero,
    #[error("{admin_positive_usize_parsing:?}")]
    Parse {
        admin_positive_usize_parsing: super::AdminPositiveUsizeParsingError,
    },
}
