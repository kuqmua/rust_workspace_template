#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub struct ListRows<Item> {
    items: crate::list_items::ListItems<Item>,
    window_total: Option<crate::list_total::ListTotal>,
}

impl<Item> ListRows<Item> {
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        crate::list_items::ListItems<Item>,
        Option<crate::list_total::ListTotal>,
    ) {
        (self.items, self.window_total)
    }

    #[must_use]
    pub const fn new(
        list_items: crate::list_items::ListItems<Item>,
        option: Option<crate::list_total::ListTotal>,
    ) -> Self {
        Self {
            items: list_items,
            window_total: option,
        }
    }
}
