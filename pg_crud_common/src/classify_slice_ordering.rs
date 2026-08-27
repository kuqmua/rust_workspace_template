#[must_use]
pub fn classify_slice_ordering<Value>(values: &[Value]) -> crate::domain_types::SliceOrdering
where
    Value: Ord,
{
    let Some((first, rest)) = values.split_first() else {
        return crate::domain_types::SliceOrdering::StrictlyIncreasing;
    };
    let mut previous = first;
    let mut ordering = crate::domain_types::SliceOrdering::StrictlyIncreasing;
    if rest.iter().any(|current| {
        if previous > current {
            return true;
        }
        if previous == current {
            ordering = crate::domain_types::SliceOrdering::NonDecreasingWithDuplicates;
        }
        previous = current;
        false
    }) {
        crate::domain_types::SliceOrdering::Unordered
    } else {
        ordering
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ordering_classification_distinguishes_all_shapes() {
        assert_eq!(
            super::classify_slice_ordering(&[1u8, 2u8, 3u8]),
            crate::domain_types::SliceOrdering::StrictlyIncreasing
        );
        assert_eq!(
            super::classify_slice_ordering(&[1u8, 1u8, 2u8]),
            crate::domain_types::SliceOrdering::NonDecreasingWithDuplicates
        );
        assert_eq!(
            super::classify_slice_ordering(&[2u8, 1u8]),
            crate::domain_types::SliceOrdering::Unordered
        );
    }
}
