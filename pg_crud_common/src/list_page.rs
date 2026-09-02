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
pub struct ListPage<Item> {
    #[getters(skip)]
    items: crate::list_items::ListItems<Item>,
    #[getters(copy)]
    total: crate::list_total::ListTotal,
}

impl<Item> ListPage<Item> {
    #[must_use]
    pub const fn items(&self) -> &[Item] {
        self.items.get_inner().as_slice()
    }
}
