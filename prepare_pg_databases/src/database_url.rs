use crate::domain_types::DatabaseUrlError;

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
    if value.trim().is_empty() {
        Err(DatabaseUrlError::Empty)
    } else if value.len() > constants_usize::VALUE_8_192 {
        Err(DatabaseUrlError::TooLong)
    } else { Ok(()) }
})]
pub struct DatabaseUrl(String);
