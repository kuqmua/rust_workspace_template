#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct PgFilterText(String);

impl TryFrom<String> for PgFilterText {
    type Error = crate::pg_filter_text_error::PgFilterTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_1_048_576 {
            Err(crate::pg_filter_text_error::PgFilterTextError)
        } else {
            Ok(Self(value))
        }
    }
}
