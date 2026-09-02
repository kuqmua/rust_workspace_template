#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct PgFilterText(String);

impl TryFrom<String> for PgFilterText {
    type Error = crate::pg_filter_text_error::PgFilterTextError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        (string.len() <= constants_usize::VALUE_1_048_576)
            .then_some(Self(string))
            .ok_or(crate::pg_filter_text_error::PgFilterTextError::TooLarge)
    }
}
