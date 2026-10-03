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

#[cfg(test)]
mod tests {
    #[test]
    fn test_filter_operation_keys_match_wire_names_without_collisions() {
        let operations = [
            frontend_contract::filter_operation::FilterOperation::AdjacentWithRange,
            frontend_contract::filter_operation::FilterOperation::Before,
            frontend_contract::filter_operation::FilterOperation::Between,
            frontend_contract::filter_operation::FilterOperation::CurrentDate,
            frontend_contract::filter_operation::FilterOperation::CurrentTime,
            frontend_contract::filter_operation::FilterOperation::CurrentTimestamp,
            frontend_contract::filter_operation::FilterOperation::Eq,
            frontend_contract::filter_operation::FilterOperation::EqToEncodedStringRepresentation,
            frontend_contract::filter_operation::FilterOperation::ExcludedUpperBound,
            frontend_contract::filter_operation::FilterOperation::FindRangesThatFullyContainTheGivenRange,
            frontend_contract::filter_operation::FilterOperation::FindRangesWithinGivenRange,
            frontend_contract::filter_operation::FilterOperation::GreaterThan,
            frontend_contract::filter_operation::FilterOperation::GreaterThanCurrentDate,
            frontend_contract::filter_operation::FilterOperation::GreaterThanCurrentTime,
            frontend_contract::filter_operation::FilterOperation::GreaterThanCurrentTimestamp,
            frontend_contract::filter_operation::FilterOperation::GreaterThanExcludedUpperBound,
            frontend_contract::filter_operation::FilterOperation::GreaterThanIncludedLowerBound,
            frontend_contract::filter_operation::FilterOperation::In,
            frontend_contract::filter_operation::FilterOperation::IncludedLowerBound,
            frontend_contract::filter_operation::FilterOperation::OverlapWithRange,
            frontend_contract::filter_operation::FilterOperation::RangeLen,
            frontend_contract::filter_operation::FilterOperation::Regex,
            frontend_contract::filter_operation::FilterOperation::StrictlyToLeftOfRange,
            frontend_contract::filter_operation::FilterOperation::StrictlyToRightOfRange,
        ];
        let mut keys = std::collections::BTreeSet::new();
        assert!(operations.into_iter().all(|operation| {
            let key = super::AdminFilterOperationKey::from(operation);
            !key.as_ref().is_empty()
                && keys.insert(key.to_string())
                && serde_json::to_value(operation)
                    .is_ok_and(|wire| wire.as_str() == Some(key.as_ref()))
        }));
        assert_eq!(keys.len(), operations.len());
    }
}
