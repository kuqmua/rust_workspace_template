#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::TryFrom,
)]
#[try_from(error = crate::database_url_error::DatabaseUrlError, validator = |value: &str| {
    if value.trim().is_empty() {
        Err(crate::database_url_error::DatabaseUrlError::Empty)
    } else if value.len() > constants_usize::VALUE_8_192 {
        Err(crate::database_url_error::DatabaseUrlError::TooLong)
    } else { Ok(()) }
})]
pub struct DatabaseUrl(String);
