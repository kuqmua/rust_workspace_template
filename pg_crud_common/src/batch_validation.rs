#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum BatchDuplicatePolicy {
    KeepFirst,
    KeepLast,
    Reject,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct BatchProcessedItemCount(usize);

impl BatchProcessedItemCount {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct BatchInvalidItemCount(usize);

impl BatchInvalidItemCount {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct BatchStoppedEarly(bool);

impl BatchStoppedEarly {
    #[must_use]
    pub const fn get(self) -> bool {
        self.0
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::FromInner,
)]
pub struct BatchInvalidItems<InvalidItem>(Vec<InvalidItem>);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefOwned,
    newtype::FromInner,
)]
pub struct StdBatchRecords<Key, Record>(std::collections::BTreeMap<Key, Record>);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct BatchValidationReport<Key, Record, InvalidItem> {
    invalid_items: BatchInvalidItems<InvalidItem>,
    processed_item_count: BatchProcessedItemCount,
    records_by_key: StdBatchRecords<Key, Record>,
    stopped_early: BatchStoppedEarly,
}

impl<Key, Record, InvalidItem> BatchValidationReport<Key, Record, InvalidItem> {
    #[must_use]
    pub fn into_parts(self) -> (StdBatchRecords<Key, Record>, BatchInvalidItems<InvalidItem>) {
        (self.records_by_key, self.invalid_items)
    }

    #[must_use]
    pub fn invalid_item_count(&self) -> BatchInvalidItemCount {
        BatchInvalidItemCount::from(self.invalid_items.0.len())
    }

    #[must_use]
    pub const fn invalid_items(&self) -> &[InvalidItem] {
        self.invalid_items.0.as_slice()
    }

    #[must_use]
    pub const fn processed_item_count(&self) -> BatchProcessedItemCount {
        self.processed_item_count
    }

    #[must_use]
    pub const fn records_by_key(&self) -> &StdBatchRecords<Key, Record> {
        &self.records_by_key
    }

    #[must_use]
    pub const fn stopped_early(&self) -> BatchStoppedEarly {
        self.stopped_early
    }
}

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
    maximum_invalid_items: BatchInvalidItemCount,
    duplicate_policy: BatchDuplicatePolicy,
    validate_source_item: ValidateSourceItem,
    select_record_key: SelectRecordKey,
    build_invalid_item: BuildInvalidItem,
    build_duplicate_invalid_item: BuildDuplicateInvalidItem,
) -> BatchValidationReport<Key, Record, InvalidItem>
where
    SourceItems: IntoIterator<Item = SourceItem>,
    Key: Ord,
    ValidateSourceItem: Fn(SourceItem) -> Result<Record, ValidationError>,
    SelectRecordKey: Fn(&Record) -> Key,
    BuildInvalidItem: Fn(usize, ValidationError) -> InvalidItem,
    BuildDuplicateInvalidItem: Fn(usize, &Key) -> InvalidItem,
{
    let maximum_invalid_item_count = maximum_invalid_items.get();
    let mut records_by_key = std::collections::BTreeMap::new();
    let mut invalid_items = Vec::with_capacity(maximum_invalid_item_count);
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
                                match duplicate_policy {
                                    BatchDuplicatePolicy::Reject => invalid_items.push(
                                        build_duplicate_invalid_item(item_index, entry.key()),
                                    ),
                                    BatchDuplicatePolicy::KeepFirst => {}
                                    BatchDuplicatePolicy::KeepLast => {
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

    BatchValidationReport {
        records_by_key: records_by_key.into(),
        invalid_items: invalid_items.into(),
        processed_item_count: BatchProcessedItemCount::from(processed_item_count),
        stopped_early: BatchStoppedEarly::from(stopped_early),
    }
}

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
