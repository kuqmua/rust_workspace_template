#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_iterator::IntoIterator,
)]
pub(super) struct TableTestNames<'value_lt>(
    bounded_types::bounded_vec::BoundedVec<
        &'value_lt str,
        { constants_usize::FOUR },
        { constants_usize::FOUR },
    >,
);
impl<'value_lt> TryFrom<Vec<&'value_lt str>> for TableTestNames<'value_lt> {
    type Error = bounded_types::bounded_value_error::BoundedValueError;

    fn try_from(value: Vec<&'value_lt str>) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from(value).map(Self::from)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_table_test_names_requires_exact_count() {
        assert!(
            super::TableTestNames::try_from(vec![
                constants_str::VALUE_8E427AD7,
                constants_str::EB24448C,
                constants_str::VALUE_9AC6D79A,
            ])
            .is_err(),
            "352fa672"
        );
        assert!(
            super::TableTestNames::try_from(vec![
                constants_str::VALUE_8E427AD7,
                constants_str::EB24448C,
                constants_str::VALUE_9AC6D79A,
                constants_str::VALUE_5A52AF33,
            ])
            .is_ok(),
            "bc783ae9"
        );
        assert!(
            super::TableTestNames::try_from(vec![
                constants_str::VALUE_8E427AD7,
                constants_str::EB24448C,
                constants_str::VALUE_9AC6D79A,
                constants_str::VALUE_5A52AF33,
                constants_str::VALUE_8E427AD7,
            ])
            .is_err(),
            "55ee5a1d"
        );
    }
}
