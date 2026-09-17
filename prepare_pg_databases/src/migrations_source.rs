#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct MigrationsSource(
    bounded_types::bounded_string::BoundedString<0usize, 4_096usize, false>,
);
impl TryFrom<String> for MigrationsSource {
    type Error = crate::migrations_source_error::MigrationsSourceError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 4_096usize {
            Err(Self::Error::TooLong)
        } else {
            bounded_types::bounded_string::BoundedString::try_from(value)
                .map(Self)
                .map_err(|source| match source {
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                        ..
                    }
                    | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                        ..
                    } => Self::Error::TooLong,
                })
        }
    }
}
