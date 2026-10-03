#[must_use]
pub fn deduplicate_preserving_order_by_key<Value, Key, AccessKey>(
    mut order_preserving_values: crate::order_preserving_values::OrderPreservingValues<Value>,
    access_key: AccessKey,
) -> crate::order_preserving_values::OrderPreservingValues<Value>
where
    Key: Eq + std::hash::Hash,
    AccessKey: Fn(&Value) -> Key,
{
    let mut seen =
        std::collections::HashSet::with_capacity(order_preserving_values.get_inner().len());
    order_preserving_values
        .get_inner_mut()
        .retain(|value| seen.insert(access_key(value)));
    order_preserving_values
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
            [(1u8, 10u8), (2u8, 20u8)]
        );
    }

    #[test]
    fn test_deduplication_handles_empty_unique_and_repeated_keys_with_one_access_per_value() {
        let cases = [
            (Vec::new(), Vec::new()),
            (
                vec![(3u8, 30u8), (1u8, 10u8), (2u8, 20u8)],
                vec![(3u8, 30u8), (1u8, 10u8), (2u8, 20u8)],
            ),
            (
                vec![(1u8, 10u8), (1u8, 20u8), (1u8, 30u8)],
                vec![(1u8, 10u8)],
            ),
            (
                vec![
                    (2u8, 20u8),
                    (1u8, 10u8),
                    (2u8, 30u8),
                    (3u8, 40u8),
                    (1u8, 50u8),
                ],
                vec![(2u8, 20u8), (1u8, 10u8), (3u8, 40u8)],
            ),
        ];
        assert!(cases.into_iter().all(|(values, expected)| {
            let input_count = values.len();
            let access_count = std::cell::Cell::new(0usize);
            let observed = Vec::from(super::deduplicate_preserving_order_by_key(
                values.into(),
                |value| {
                    access_count.set(access_count.get() + 1usize);
                    value.0
                },
            ));
            observed == expected && access_count.get() == input_count
        }));
    }
}
