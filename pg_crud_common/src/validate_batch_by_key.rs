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
    maximum_invalid_items: crate::batch_invalid_item_count::BatchInvalidItemCount,
    duplicate_policy: crate::batch_duplicate_policy::BatchDuplicatePolicy,
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
