#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct ListPage<Item> {
    pub(super) items: crate::list_items::ListItems<Item>,
    pub(super) total: crate::list_total::ListTotal,
}

impl<Item> ListPage<Item> {
    #[must_use]
    pub const fn items(&self) -> &[Item] {
        self.items.0.as_slice()
    }

    #[must_use]
    pub const fn total(&self) -> crate::list_total::ListTotal {
        self.total
    }
}
