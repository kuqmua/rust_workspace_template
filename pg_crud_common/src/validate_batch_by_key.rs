#[must_use]
pub fn validate_batch_by_key<
    SourceItems,
    SourceItem,
    Record,
    Key,
    InvalidItem,
    ValidationError,
    ValidateSourceItem,
    SelectRecordKey,
    BuildInvalidItem,
    BuildDuplicateInvalidItem,
>(
    source_items: SourceItems,
    batch_invalid_item_count: crate::batch_invalid_item_count::BatchInvalidItemCount,
    batch_duplicate_policy: crate::batch_duplicate_policy::BatchDuplicatePolicy,
    validate_source_item: ValidateSourceItem,
    select_record_key: SelectRecordKey,
    build_invalid_item: BuildInvalidItem,
    build_duplicate_invalid_item: BuildDuplicateInvalidItem,
) -> crate::batch_validation_report::BatchValidationReport<Key, Record, InvalidItem>
where
    SourceItems: IntoIterator<Item = SourceItem>,
    Key: Ord,
    ValidateSourceItem: Fn(SourceItem) -> Result<Record, ValidationError>,
    SelectRecordKey: Fn(&Record) -> Key,
    BuildInvalidItem: Fn(usize, ValidationError) -> InvalidItem,
    BuildDuplicateInvalidItem: Fn(usize, &Key) -> InvalidItem,
{
    let maximum_invalid_item_count = batch_invalid_item_count.get();
    let mut records_by_key = std::collections::BTreeMap::new();
    let mut invalid_items =
        Vec::with_capacity(maximum_invalid_item_count.min(constants_usize::VALUE_4_096));
    let mut processed_item_count = constants_usize::ZERO;
    let mut stopped_early = false;
    let _validation_flow =
        source_items
            .into_iter()
            .enumerate()
            .try_for_each(|(item_index, source_item)| {
                if invalid_items.len() >= maximum_invalid_item_count {
                    stopped_early = true;
                    return std::ops::ControlFlow::Break(());
                }
                processed_item_count = processed_item_count.saturating_add(constants_usize::ONE);
                match validate_source_item(source_item) {
                    Ok(record) => {
                        let key = select_record_key(&record);
                        match records_by_key.entry(key) {
                        std::collections::btree_map::Entry::Vacant(entry) => {
                            let _inserted_record = entry.insert(record);
                        }
                        std::collections::btree_map::Entry::Occupied(mut entry) => {
                            match batch_duplicate_policy {
                                crate::batch_duplicate_policy::BatchDuplicatePolicy::Reject => {
                                    invalid_items.push(build_duplicate_invalid_item(
                                        item_index,
                                        entry.key(),
                                    ));
                                }
                                crate::batch_duplicate_policy::BatchDuplicatePolicy::KeepFirst => {}
                                crate::batch_duplicate_policy::BatchDuplicatePolicy::KeepLast => {
                                    drop(entry.insert(record));
                                }
                            }
                        }
                    }
                    }
                    Err(error) => invalid_items.push(build_invalid_item(item_index, error)),
                }
                std::ops::ControlFlow::Continue(())
            });
    crate::batch_validation_report::BatchValidationReport::new(
        invalid_items.into(),
        crate::batch_processed_item_count::BatchProcessedItemCount::from(processed_item_count),
        records_by_key.into(),
        crate::batch_stopped_early::BatchStoppedEarly::from(stopped_early),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rejected_duplicate_reaches_limit_before_validating_next_record() {
        let validation_count = std::cell::Cell::new(0usize);
        let report = super::validate_batch_by_key(
            [(2u8, 20u8), (2u8, 30u8), (1u8, 10u8)],
            crate::batch_invalid_item_count::BatchInvalidItemCount::from(1usize),
            crate::batch_duplicate_policy::BatchDuplicatePolicy::Reject,
            |record| {
                validation_count.set(validation_count.get() + 1usize);
                Ok::<(u8, u8), u8>(record)
            },
            |record| record.0,
            |item_index, error| (item_index, error),
            |item_index, key| (item_index, *key),
        );
        assert_eq!(validation_count.get(), 2usize);
        assert_eq!(report.records_by_key().as_ref().len(), 1usize);
        assert_eq!(
            report.records_by_key().as_ref().get(&2u8),
            Some(&(2u8, 20u8))
        );
        assert_eq!(report.invalid_items(), [(1usize, 2u8)]);
        assert_eq!(
            report.processed_item_count(),
            crate::batch_processed_item_count::BatchProcessedItemCount::from(2usize),
        );
        assert_eq!(
            report.stopped_early(),
            crate::batch_stopped_early::BatchStoppedEarly::from(true),
        );
    }

    #[test]
    fn test_empty_batch_does_not_allocate_invalid_item_maximum() {
        let report = super::validate_batch_by_key(
            std::iter::empty::<u8>(),
            crate::batch_invalid_item_count::BatchInvalidItemCount::from(usize::from(
                std::num::NonZeroUsize::MAX,
            )),
            crate::batch_duplicate_policy::BatchDuplicatePolicy::Reject,
            Ok::<u8, u8>,
            |record| *record,
            |_item_index, error| error,
            |_item_index, key| *key,
        );
        assert!(report.invalid_items().is_empty());
        assert_eq!(
            report.processed_item_count(),
            crate::batch_processed_item_count::BatchProcessedItemCount::from(constants_usize::ZERO),
        );
        assert_eq!(
            report.stopped_early(),
            crate::batch_stopped_early::BatchStoppedEarly::from(false),
        );
    }

    #[test]
    fn test_batch_duplicate_policies_preserve_records_and_invalid_source_positions() {
        let cases = [
            (
                crate::batch_duplicate_policy::BatchDuplicatePolicy::Reject,
                vec![(1u8, 10u8), (2u8, 20u8), (3u8, 50u8)],
                vec![(2usize, 2u8), (3usize, 9u8), (4usize, 1u8)],
            ),
            (
                crate::batch_duplicate_policy::BatchDuplicatePolicy::KeepFirst,
                vec![(1u8, 10u8), (2u8, 20u8), (3u8, 50u8)],
                vec![(3usize, 9u8)],
            ),
            (
                crate::batch_duplicate_policy::BatchDuplicatePolicy::KeepLast,
                vec![(1u8, 40u8), (2u8, 30u8), (3u8, 50u8)],
                vec![(3usize, 9u8)],
            ),
        ];
        assert!(
            cases
                .into_iter()
                .all(|(policy, expected_records, expected_invalid)| {
                    let report = super::validate_batch_by_key(
                        vec![
                            (2u8, 20u8),
                            (1u8, 10u8),
                            (2u8, 30u8),
                            (0u8, 0u8),
                            (1u8, 40u8),
                            (3u8, 50u8),
                        ],
                        crate::batch_invalid_item_count::BatchInvalidItemCount::from(10usize),
                        policy,
                        |record| {
                            if record.0 == 0u8 {
                                Err(9u8)
                            } else {
                                Ok(record)
                            }
                        },
                        |record| record.0,
                        |item_index, error| (item_index, error),
                        |item_index, key| (item_index, *key),
                    );
                    let observed_records = report
                        .records_by_key()
                        .as_ref()
                        .values()
                        .copied()
                        .collect::<Vec<_>>();
                    observed_records == expected_records
                        && report.invalid_items() == expected_invalid
                        && report.processed_item_count()
                            == crate::batch_processed_item_count::BatchProcessedItemCount::from(
                                6usize,
                            )
                        && report.stopped_early()
                            == crate::batch_stopped_early::BatchStoppedEarly::from(false)
                })
        );
    }

    #[test]
    fn test_batch_invalid_limits_bound_validation_and_distinguish_exhaustion_from_early_stop() {
        let cases = [
            (0usize, 0usize, true),
            (1usize, 1usize, true),
            (2usize, 2usize, true),
            (3usize, 3usize, false),
            (4usize, 3usize, false),
        ];
        assert!(
            cases
                .into_iter()
                .all(|(limit, processed_count, stopped_early)| {
                    let validation_count = std::cell::Cell::new(0usize);
                    let report = super::validate_batch_by_key(
                        vec![0u8; 3usize],
                        crate::batch_invalid_item_count::BatchInvalidItemCount::from(limit),
                        crate::batch_duplicate_policy::BatchDuplicatePolicy::Reject,
                        |record| {
                            validation_count.set(validation_count.get() + 1usize);
                            Err::<u8, u8>(record)
                        },
                        |record| *record,
                        |item_index, error| (item_index, error),
                        |item_index, key| (item_index, *key),
                    );
                    let expected_invalid = (0usize..processed_count)
                        .map(|item_index| (item_index, 0u8))
                        .collect::<Vec<_>>();
                    report.records_by_key().as_ref().is_empty()
                        && report.invalid_items() == expected_invalid
                        && validation_count.get() == processed_count
                        && report.processed_item_count()
                            == crate::batch_processed_item_count::BatchProcessedItemCount::from(
                                processed_count,
                            )
                        && report.stopped_early()
                            == crate::batch_stopped_early::BatchStoppedEarly::from(stopped_early)
                })
        );
    }
}
