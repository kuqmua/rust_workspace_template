#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_try_from::TryFrom,
)]
#[try_from(error = crate::database_url_error::DatabaseUrlError, validator = |value: &str| {
    if value.trim().is_empty() {
        Err(crate::database_url_error::DatabaseUrlError::Empty)
    } else if value.len() > constants_usize::VALUE_8_192 {
        Err(crate::database_url_error::DatabaseUrlError::TooLong)
    } else { Ok(()) }
})]
pub struct DatabaseUrl(String);
