#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::TryFrom,
)]
#[try_from(error = crate::migrations_source_error::MigrationsSourceError, validator = |value: &str| {
    if value.len() > 4_096usize { Err(crate::migrations_source_error::MigrationsSourceError::TooLong) } else { Ok(()) }
})]
pub struct MigrationsSource(String);
