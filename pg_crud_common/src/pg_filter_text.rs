#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct PgFilterText(String);

impl TryFrom<String> for PgFilterText {
    type Error = crate::pg_filter_text_error::PgFilterTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        (value.len() <= constants_usize::VALUE_1_048_576)
            .then_some(Self(value))
            .ok_or(crate::pg_filter_text_error::PgFilterTextError::TooLarge)
    }
}
