#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_new::New,
)]
pub struct BatchValidationReport<Key, Record, InvalidItem> {
    #[getters(skip)]
    invalid_items: crate::batch_invalid_items::BatchInvalidItems<InvalidItem>,
    #[getters(copy)]
    processed_item_count: crate::batch_processed_item_count::BatchProcessedItemCount,
    records_by_key: crate::batch_records_b_tree_map::BatchRecordsBTreeMap<Key, Record>,
    #[getters(copy)]
    stopped_early: crate::batch_stopped_early::BatchStoppedEarly,
}

impl<Key, Record, InvalidItem> BatchValidationReport<Key, Record, InvalidItem> {
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        crate::batch_records_b_tree_map::BatchRecordsBTreeMap<Key, Record>,
        crate::batch_invalid_items::BatchInvalidItems<InvalidItem>,
    ) {
        (self.records_by_key, self.invalid_items)
    }

    #[must_use]
    pub fn invalid_item_count(&self) -> crate::batch_invalid_item_count::BatchInvalidItemCount {
        crate::batch_invalid_item_count::BatchInvalidItemCount::from(
            self.invalid_items.get_inner().len(),
        )
    }

    #[must_use]
    pub const fn invalid_items(&self) -> &[InvalidItem] {
        self.invalid_items.get_inner().as_slice()
    }
}
