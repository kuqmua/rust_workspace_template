#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkAdminPositiveU64Error {
    #[error("administrator duration must be greater than zero")]
    IsZero,
    #[error("{admin_positive_u64_parsing:?}")]
    Parse {
        admin_positive_u64_parsing: crate::parse_int_error::ParseIntError,
    },
}
