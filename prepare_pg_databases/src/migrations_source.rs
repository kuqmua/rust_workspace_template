#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::TryFrom,
)]
#[try_from(error = crate::migrations_source_error::MigrationsSourceError, validator = |value: &str| {
    if value.len() > 4_096usize { Err(crate::migrations_source_error::MigrationsSourceError::TooLong) } else { Ok(()) }
})]
pub struct MigrationsSource(String);
