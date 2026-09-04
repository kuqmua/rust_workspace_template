#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct MigrationsSource(String);
impl TryFrom<String> for MigrationsSource {
    type Error = crate::migrations_source_error::MigrationsSourceError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 4_096usize {
            Err(Self::Error::TooLong)
        } else {
            Ok(Self(value))
        }
    }
}
