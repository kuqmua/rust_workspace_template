#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_display::Display,
)]
#[bounded_string(max = 63usize)]
pub struct AdminFilterOperationKey(
    bounded_types::bounded_string::BoundedString<0usize, 63usize, false>,
);
impl From<frontend_contract::filter_operation::FilterOperation> for AdminFilterOperationKey {
    fn from(filter_operation: frontend_contract::filter_operation::FilterOperation) -> Self {
        let formatted = format!("{filter_operation:?}");
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
