#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct PgFilterText(String);

impl TryFrom<String> for PgFilterText {
    type Error = crate::domain_types::PgFilterTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_1_048_576 {
            Err(crate::domain_types::PgFilterTextError)
        } else {
            Ok(Self(value))
        }
    }
}
