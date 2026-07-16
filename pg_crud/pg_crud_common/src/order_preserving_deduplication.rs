#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SliceOrdering {
    NonDecreasingWithDuplicates,
    StrictlyIncreasing,
    Unordered,
}

#[must_use]
pub fn classify_slice_ordering<Value>(values: &[Value]) -> SliceOrdering
where
    Value: Ord,
{
    let Some((first, rest)) = values.split_first() else {
        return SliceOrdering::StrictlyIncreasing;
    };
    let mut previous = first;
    let mut ordering = SliceOrdering::StrictlyIncreasing;
    if rest.iter().any(|current| {
        if previous > current {
            return true;
        }
        if previous == current {
            ordering = SliceOrdering::NonDecreasingWithDuplicates;
        }
        previous = current;
        false
    }) {
        SliceOrdering::Unordered
    } else {
        ordering
    }
}

#[must_use]
pub fn deduplicate_preserving_order_by_key<Value, Key, AccessKey>(
    values: Vec<Value>,
    access_key: AccessKey,
) -> Vec<Value>
where
    Key: Eq + std::hash::Hash,
    AccessKey: Fn(&Value) -> Key,
{
    let mut seen = std::collections::HashSet::with_capacity(values.len());
    values
        .into_iter()
        .filter(|value| seen.insert(access_key(value)))
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn deduplication_keeps_first_value_and_input_order() {
        let values = vec![(1u8, 10u8), (2u8, 20u8), (1u8, 30u8)];
        assert_eq!(
            super::deduplicate_preserving_order_by_key(values, |value| value.0),
            vec![(1u8, 10u8), (2u8, 20u8)]
        );
    }

    #[test]
    fn ordering_classification_distinguishes_all_shapes() {
        assert_eq!(
            super::classify_slice_ordering(&[1u8, 2u8, 3u8]),
            super::SliceOrdering::StrictlyIncreasing
        );
        assert_eq!(
            super::classify_slice_ordering(&[1u8, 1u8, 2u8]),
            super::SliceOrdering::NonDecreasingWithDuplicates
        );
        assert_eq!(
            super::classify_slice_ordering(&[2u8, 1u8]),
            super::SliceOrdering::Unordered
        );
    }
}
