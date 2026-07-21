#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SliceOrdering {
    NonDecreasingWithDuplicates,
    StrictlyIncreasing,
    Unordered,
}
#[derive(Clone, Debug, Eq, PartialEq, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct OrderPreservingValues<Value>(Vec<Value>);

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
    values: OrderPreservingValues<Value>,
    access_key: AccessKey,
) -> OrderPreservingValues<Value>
where
    Key: Eq + std::hash::Hash,
    AccessKey: Fn(&Value) -> Key,
{
    let mut seen = std::collections::HashSet::with_capacity(values.0.len());
    values
        .0
        .into_iter()
        .filter(|value| seen.insert(access_key(value)))
        .collect::<Vec<_>>()
        .into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn deduplication_keeps_first_value_and_input_order() {
        let values = vec![(1u8, 10u8), (2u8, 20u8), (1u8, 30u8)];
        assert_eq!(
            Vec::from(super::deduplicate_preserving_order_by_key(
                values.into(),
                |value| value.0
            )),
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
