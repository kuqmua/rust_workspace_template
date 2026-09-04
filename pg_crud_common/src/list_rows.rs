#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_new::New,
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
}
