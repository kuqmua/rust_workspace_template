#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::BoundedStringWrapper,
    proc_macro_newtype::Display,
)]
#[bounded_string(max = 63usize)]
pub struct AdminFilterOperationKey(
    bounded_types::bounded_string::BoundedString<0usize, 63usize, false>,
);
impl From<frontend_contract::filter_operation::FilterOperation> for AdminFilterOperationKey {
    fn from(value: frontend_contract::filter_operation::FilterOperation) -> Self {
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
