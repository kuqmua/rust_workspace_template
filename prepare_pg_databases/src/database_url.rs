#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct DatabaseUrl(String);
impl TryFrom<String> for DatabaseUrl {
    type Error = crate::database_url_error::DatabaseUrlError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            Err(Self::Error::Empty)
        } else if value.len() > constants_usize::VALUE_8_192 {
            Err(Self::Error::TooLong)
        } else {
            Ok(Self(value))
        }
    }
}
