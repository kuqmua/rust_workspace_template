#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct PgFilterText(
    bounded_types::bounded_string::BoundedString<0usize, 1_048_576usize, false>,
);

impl TryFrom<String> for PgFilterText {
    type Error = crate::pg_filter_text_error::PgFilterTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(Self::Error::TooLarge)
    }
}
