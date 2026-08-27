#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct ListPage<Item> {
    pub(super) items: super::ListItems<Item>,
    pub(super) total: super::ListTotal,
}

impl<Item> ListPage<Item> {
    #[must_use]
    pub const fn items(&self) -> &[Item] {
        self.items.0.as_slice()
    }

    #[must_use]
    pub const fn total(&self) -> super::ListTotal {
        self.total
    }
}
