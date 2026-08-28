#[path = "batch_duplicate_policy.rs"]
mod batch_duplicate_policy;
#[path = "batch_invalid_item_count.rs"]
mod batch_invalid_item_count;
#[path = "batch_invalid_items.rs"]
mod batch_invalid_items;
#[path = "batch_processed_item_count.rs"]
mod batch_processed_item_count;
#[path = "batch_records_b_tree_map.rs"]
mod batch_records_b_tree_map;
#[path = "batch_stopped_early.rs"]
mod batch_stopped_early;
#[path = "batch_validation_report.rs"]
mod batch_validation_report;
#[path = "validate_batch_by_key.rs"]
mod validate_batch_by_key;

pub use batch_duplicate_policy::BatchDuplicatePolicy;
pub use batch_invalid_item_count::BatchInvalidItemCount;
pub use batch_invalid_items::BatchInvalidItems;
pub use batch_processed_item_count::BatchProcessedItemCount;
pub use batch_records_b_tree_map::BatchRecordsBTreeMap;
pub use batch_stopped_early::BatchStoppedEarly;
pub use batch_validation_report::BatchValidationReport;
pub use validate_batch_by_key::validate_batch_by_key;

#[cfg(test)]
mod tests {
    fn validate(
        values: Vec<i32>,
        maximum_invalid_items: usize,
        duplicate_policy: super::BatchDuplicatePolicy,
    ) -> super::BatchValidationReport<i32, i32, (usize, &'static str)> {
        super::validate_batch_by_key(
            values,
            super::BatchInvalidItemCount::from(maximum_invalid_items),
            duplicate_policy,
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
    fn rejects_duplicate_explicitly() {
        let report = validate(
            vec![1i32, 1i32, 2i32],
            4,
            super::BatchDuplicatePolicy::Reject,
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
    fn applies_keep_first_and_keep_last_policies() {
        let first = super::validate_batch_by_key(
            [
                (1i32, constants_str::TEST_FIRST),
                (1i32, constants_str::TEST_LAST),
            ],
            super::BatchInvalidItemCount::from(constants_usize::ONE),
            super::BatchDuplicatePolicy::KeepFirst,
            Ok::<_, std::convert::Infallible>,
            |record| record.0,
            |_index, error| match error {},
            |_index, _key| constants_str::TEST_DUPLICATE,
        );
        let last = super::validate_batch_by_key(
            [
                (1i32, constants_str::TEST_FIRST),
                (1i32, constants_str::TEST_LAST),
            ],
            super::BatchInvalidItemCount::from(constants_usize::ONE),
            super::BatchDuplicatePolicy::KeepLast,
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
    fn stops_when_invalid_item_limit_is_reached() {
        let report = validate(
            vec![-1i32, -2i32, 3i32],
            1,
            super::BatchDuplicatePolicy::Reject,
        );
        assert_eq!(report.invalid_item_count().get(), constants_usize::ONE);
        assert_eq!(report.processed_item_count().get(), constants_usize::ONE);
        assert!(report.stopped_early().get());
        assert!(report.records_by_key().as_ref().is_empty());
    }

    #[test]
    fn zero_invalid_item_limit_processes_nothing() {
        let report = validate(vec![1i32], 0, super::BatchDuplicatePolicy::Reject);
        assert_eq!(report.processed_item_count().get(), constants_usize::ZERO);
        assert!(report.stopped_early().get());
    }
}
