#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkAdminPasswordHashConcurrencyError {
    #[error("administrator password hash concurrency must be greater than zero")]
    IsZero,
    #[error("{admin_positive_usize_parsing:?}")]
    Parse {
        admin_positive_usize_parsing: crate::config_parse_int_error::ConfigParseIntError,
    },
}
