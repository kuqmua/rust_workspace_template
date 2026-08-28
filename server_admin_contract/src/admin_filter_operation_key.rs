#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(max = 63usize)]
pub struct AdminFilterOperationKey(String);
impl From<frontend_contract::domain_types::FilterOperation> for AdminFilterOperationKey {
    fn from(value: frontend_contract::domain_types::FilterOperation) -> Self {
        let formatted = format!("{value:?}");
        let mut key = String::with_capacity(formatted.len().saturating_mul(2usize));
        formatted
            .chars()
            .enumerate()
            .for_each(|(index, character)| {
                if character.is_uppercase() && index > constants_usize::ZERO {
                    key.push('_');
                }
                key.extend(character.to_lowercase());
            });
        Self::try_from(key).unwrap_or_default()
    }
}
