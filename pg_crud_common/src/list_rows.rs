#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct ListRows<Item> {
    pub(super) items: super::ListItems<Item>,
    pub(super) window_total: Option<super::ListTotal>,
}

impl<Item> ListRows<Item> {
    #[must_use]
    pub const fn new(
        items: super::ListItems<Item>,
        window_total: Option<super::ListTotal>,
    ) -> Self {
        Self {
            items,
            window_total,
        }
    }
}
