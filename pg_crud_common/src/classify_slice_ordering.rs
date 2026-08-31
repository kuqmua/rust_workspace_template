#[must_use]
pub fn classify_slice_ordering<Value>(values: &[Value]) -> crate::slice_ordering::SliceOrdering
where
    Value: Ord,
{
    let Some((first, rest)) = values.split_first() else {
        return crate::slice_ordering::SliceOrdering::StrictlyIncreasing;
    };
    let mut previous = first;
    let mut ordering = crate::slice_ordering::SliceOrdering::StrictlyIncreasing;
    if rest.iter().any(|current| {
        if previous > current {
            return true;
        }
        if previous == current {
            ordering = crate::slice_ordering::SliceOrdering::NonDecreasingWithDuplicates;
        }
        previous = current;
        false
    }) {
        crate::slice_ordering::SliceOrdering::Unordered
    } else {
        ordering
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ordering_classification_distinguishes_all_shapes() {
        assert_eq!(
            crate::classify_slice_ordering::classify_slice_ordering(&[1u8, 2u8, 3u8]),
            crate::slice_ordering::SliceOrdering::StrictlyIncreasing
        );
        assert_eq!(
            crate::classify_slice_ordering::classify_slice_ordering(&[1u8, 1u8, 2u8]),
            crate::slice_ordering::SliceOrdering::NonDecreasingWithDuplicates
        );
        assert_eq!(
            crate::classify_slice_ordering::classify_slice_ordering(&[2u8, 1u8]),
            crate::slice_ordering::SliceOrdering::Unordered
        );
    }
}
