use super::domain_types::MigrationsSourceError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::TryFrom,
)]
#[try_from(validator = |value: &str| {
    if value.len() > 4_096usize { Err(MigrationsSourceError::TooLong) } else { Ok(()) }
})]
pub struct MigrationsSource(String);
