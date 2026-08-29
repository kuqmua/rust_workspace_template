#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct ListRows<Item> {
    pub(super) items: crate::list_items::ListItems<Item>,
    pub(super) window_total: Option<crate::list_total::ListTotal>,
}

impl<Item> ListRows<Item> {
    #[must_use]
    pub const fn new(
        items: crate::list_items::ListItems<Item>,
        window_total: Option<crate::list_total::ListTotal>,
    ) -> Self {
        Self {
            items,
            window_total,
        }
    }
}
