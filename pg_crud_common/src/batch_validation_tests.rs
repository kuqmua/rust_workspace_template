#[cfg(test)]
mod tests {
    fn validate_batch_fixture(
        vec: Vec<i32>,
        usize: usize,
        batch_duplicate_policy: crate::batch_duplicate_policy::BatchDuplicatePolicy,
    ) -> crate::batch_validation_report::BatchValidationReport<i32, i32, (usize, &'static str)>
    {
        crate::validate_batch_by_key::validate_batch_by_key(
            vec,
            crate::batch_invalid_item_count::BatchInvalidItemCount::from(usize),
            batch_duplicate_policy,
            |value| {
                if value >= constants_i32::ZERO {
                    Ok(value)
                } else {
                    Err(constants_str::TEST_NEGATIVE)
                }
            },
            |value| *value,
            |index, error| (index, error),
            |index, _key| (index, constants_str::TEST_DUPLICATE),
        )
    }

    #[test]
    fn test_rejects_duplicate_explicitly() {
        let report = validate_batch_fixture(
            vec![1i32, 1i32, 2i32],
            4,
            crate::batch_duplicate_policy::BatchDuplicatePolicy::Reject,
        );
        assert_eq!(report.records_by_key().as_ref().len(), 2usize);
        assert_eq!(
            report.invalid_items(),
            &[(constants_usize::ONE, constants_str::TEST_DUPLICATE)]
        );
        assert_eq!(report.processed_item_count().get(), 3usize);
        assert!(!report.stopped_early().get());
    }

    #[test]
    fn test_applies_keep_first_and_keep_last_policies() {
        let first = crate::validate_batch_by_key::validate_batch_by_key(
            [
                (1i32, constants_str::TEST_FIRST),
                (1i32, constants_str::TEST_LAST),
            ],
            crate::batch_invalid_item_count::BatchInvalidItemCount::from(constants_usize::ONE),
            crate::batch_duplicate_policy::BatchDuplicatePolicy::KeepFirst,
            Ok::<_, std::convert::Infallible>,
            |record| record.0,
            |_index, error| match error {},
            |_index, _key| constants_str::TEST_DUPLICATE,
        );
        let last = crate::validate_batch_by_key::validate_batch_by_key(
            [
                (1i32, constants_str::TEST_FIRST),
                (1i32, constants_str::TEST_LAST),
            ],
            crate::batch_invalid_item_count::BatchInvalidItemCount::from(constants_usize::ONE),
            crate::batch_duplicate_policy::BatchDuplicatePolicy::KeepLast,
            Ok::<_, std::convert::Infallible>,
            |record| record.0,
            |_index, error| match error {},
            |_index, _key| constants_str::TEST_DUPLICATE,
        );
        assert_eq!(
            first.records_by_key().as_ref().get(&1i32),
            Some(&(1i32, constants_str::TEST_FIRST))
        );
        assert_eq!(
            last.records_by_key().as_ref().get(&1i32),
            Some(&(1i32, constants_str::TEST_LAST))
        );
    }

    #[test]
    fn test_stops_when_invalid_item_limit_is_reached() {
        let report = validate_batch_fixture(
            vec![-1i32, -2i32, 3i32],
            1,
            crate::batch_duplicate_policy::BatchDuplicatePolicy::Reject,
        );
        assert_eq!(report.invalid_item_count().get(), constants_usize::ONE);
        assert_eq!(report.processed_item_count().get(), constants_usize::ONE);
        assert!(report.stopped_early().get());
        assert!(report.records_by_key().as_ref().is_empty());
    }

    #[test]
    fn test_zero_invalid_item_limit_processes_nothing() {
        let report = validate_batch_fixture(
            vec![1i32],
            0,
            crate::batch_duplicate_policy::BatchDuplicatePolicy::Reject,
        );
        assert_eq!(report.processed_item_count().get(), constants_usize::ZERO);
        assert!(report.stopped_early().get());
    }
}
