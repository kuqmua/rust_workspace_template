#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct BatchValidationReport<Key, Record, InvalidItem> {
    pub(super) invalid_items: crate::batch_invalid_items::BatchInvalidItems<InvalidItem>,
    pub(super) processed_item_count: crate::batch_processed_item_count::BatchProcessedItemCount,
    pub(super) records_by_key: crate::batch_records_b_tree_map::BatchRecordsBTreeMap<Key, Record>,
    pub(super) stopped_early: crate::batch_stopped_early::BatchStoppedEarly,
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
        crate::batch_invalid_item_count::BatchInvalidItemCount::from(self.invalid_items.0.len())
    }

    #[must_use]
    pub const fn invalid_items(&self) -> &[InvalidItem] {
        self.invalid_items.0.as_slice()
    }

    #[must_use]
    pub const fn processed_item_count(
        &self,
    ) -> crate::batch_processed_item_count::BatchProcessedItemCount {
        self.processed_item_count
    }

    #[must_use]
    pub const fn records_by_key(
        &self,
    ) -> &crate::batch_records_b_tree_map::BatchRecordsBTreeMap<Key, Record> {
        &self.records_by_key
    }

    #[must_use]
    pub const fn stopped_early(&self) -> crate::batch_stopped_early::BatchStoppedEarly {
        self.stopped_early
    }
}
