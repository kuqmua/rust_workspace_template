#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    generate_constructor::New,
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
