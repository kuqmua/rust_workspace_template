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
    #[test]
    fn test_batch_preserves_invalid_indices_and_valid_record_order() {
        let report = validate_batch_fixture(
            vec![3i32, -1i32, 1i32, 3i32, -2i32, 2i32],
            4,
            crate::batch_duplicate_policy::BatchDuplicatePolicy::Reject,
        );
        assert_eq!(
            report.invalid_items(),
            &[
                (1usize, constants_str::TEST_NEGATIVE),
                (3usize, constants_str::TEST_DUPLICATE),
                (4usize, constants_str::TEST_NEGATIVE)
            ]
        );
        assert_eq!(
            report
                .records_by_key()
                .as_ref()
                .keys()
                .copied()
                .collect::<Vec<_>>(),
            vec![1i32, 2i32, 3i32]
        );
        assert_eq!(report.processed_item_count().get(), 6usize);
        assert!(!report.stopped_early().get());
    }

    #[test]
    fn test_invalid_limit_at_end_does_not_report_early_stop() {
        let report = validate_batch_fixture(
            vec![1i32, -1i32],
            1,
            crate::batch_duplicate_policy::BatchDuplicatePolicy::Reject,
        );
        assert_eq!(
            report.invalid_items(),
            &[(1usize, constants_str::TEST_NEGATIVE)]
        );
        assert_eq!(report.records_by_key().as_ref().get(&1i32), Some(&1i32));
        assert_eq!(report.processed_item_count().get(), 2usize);
        assert!(!report.stopped_early().get());
    }

    #[test]
    fn test_duplicate_limit_stops_before_later_record_validation() {
        let validated_items = std::cell::Cell::new(0usize);
        let report = crate::validate_batch_by_key::validate_batch_by_key(
            [1i32, 1i32, 2i32, -1i32],
            crate::batch_invalid_item_count::BatchInvalidItemCount::from(1usize),
            crate::batch_duplicate_policy::BatchDuplicatePolicy::Reject,
            |value| {
                validated_items.set(validated_items.get() + 1usize);
                Ok::<_, std::convert::Infallible>(value)
            },
            |value| *value,
            |_index, error| match error {},
            |index, _key| (index, constants_str::TEST_DUPLICATE),
        );
        assert_eq!(validated_items.get(), 2usize);
        assert_eq!(
            report.invalid_items(),
            &[(1usize, constants_str::TEST_DUPLICATE)]
        );
        assert_eq!(report.records_by_key().as_ref().len(), 1usize);
        assert_eq!(report.records_by_key().as_ref().get(&1i32), Some(&1i32));
        assert_eq!(report.processed_item_count().get(), 2usize);
        assert!(report.stopped_early().get());
    }

    #[test]
    fn test_empty_batch_with_zero_invalid_limit_is_complete() {
        let report = validate_batch_fixture(
            Vec::new(),
            0,
            crate::batch_duplicate_policy::BatchDuplicatePolicy::Reject,
        );
        assert!(report.invalid_items().is_empty());
        assert!(report.records_by_key().as_ref().is_empty());
        assert_eq!(report.processed_item_count().get(), 0usize);
        assert!(!report.stopped_early().get());
    }

    #[test]
    fn test_batch_report_ownership_transfer_preserves_records_and_invalid_allocation() {
        let report = validate_batch_fixture(
            vec![3i32, -1i32, 1i32, 3i32, -2i32, 2i32],
            4,
            crate::batch_duplicate_policy::BatchDuplicatePolicy::Reject,
        );
        let invalid_items_pointer = report.invalid_items().as_ptr();
        assert_eq!(report.invalid_item_count().get(), 3usize);
        let (records_by_key, invalid_items) = report.into_parts();
        assert_eq!(invalid_items.get_inner().as_ptr(), invalid_items_pointer);
        assert_eq!(
            invalid_items.get_inner().as_slice(),
            &[
                (1usize, constants_str::TEST_NEGATIVE),
                (3usize, constants_str::TEST_DUPLICATE),
                (4usize, constants_str::TEST_NEGATIVE),
            ],
        );
        assert_eq!(
            records_by_key.as_ref(),
            &std::collections::BTreeMap::from([(1i32, 1i32), (2i32, 2i32), (3i32, 3i32)]),
        );
    }
}
