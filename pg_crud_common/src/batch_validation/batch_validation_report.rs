#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct BatchValidationReport<Key, Record, InvalidItem> {
    pub(super) invalid_items: super::BatchInvalidItems<InvalidItem>,
    pub(super) processed_item_count: super::BatchProcessedItemCount,
    pub(super) records_by_key: super::BatchRecordsBTreeMap<Key, Record>,
    pub(super) stopped_early: super::BatchStoppedEarly,
}

impl<Key, Record, InvalidItem> BatchValidationReport<Key, Record, InvalidItem> {
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        super::BatchRecordsBTreeMap<Key, Record>,
        super::BatchInvalidItems<InvalidItem>,
    ) {
        (self.records_by_key, self.invalid_items)
    }

    #[must_use]
    pub fn invalid_item_count(&self) -> super::BatchInvalidItemCount {
        super::BatchInvalidItemCount::from(self.invalid_items.0.len())
    }

    #[must_use]
    pub const fn invalid_items(&self) -> &[InvalidItem] {
        self.invalid_items.0.as_slice()
    }

    #[must_use]
    pub const fn processed_item_count(&self) -> super::BatchProcessedItemCount {
        self.processed_item_count
    }

    #[must_use]
    pub const fn records_by_key(&self) -> &super::BatchRecordsBTreeMap<Key, Record> {
        &self.records_by_key
    }

    #[must_use]
    pub const fn stopped_early(&self) -> super::BatchStoppedEarly {
        self.stopped_early
    }
}
