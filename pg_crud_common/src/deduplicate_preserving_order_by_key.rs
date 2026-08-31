#[must_use]
pub fn deduplicate_preserving_order_by_key<Value, Key, AccessKey>(
    mut values: crate::order_preserving_values::OrderPreservingValues<Value>,
    access_key: AccessKey,
) -> crate::order_preserving_values::OrderPreservingValues<Value>
where
    Key: Eq + std::hash::Hash,
    AccessKey: Fn(&Value) -> Key,
{
    let mut seen = std::collections::HashSet::with_capacity(values.get_inner().len());
    values
        .get_inner_mut()
        .retain(|value| seen.insert(access_key(value)));
    values
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_deduplication_keeps_first_value_and_input_order() {
        let values = vec![(1u8, 10u8), (2u8, 20u8), (1u8, 30u8)];
        assert_eq!(
            Vec::from(
                crate::deduplicate_preserving_order_by_key::deduplicate_preserving_order_by_key(
                    values.into(),
                    |value| value.0
                )
            ),
            vec![(1u8, 10u8), (2u8, 20u8)]
        );
    }
}
