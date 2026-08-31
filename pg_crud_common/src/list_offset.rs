#[derive(
    generate_accessor::Getters,
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
)]
pub struct ListOffset(i64);

impl From<crate::pagination_offset::PaginationOffset> for ListOffset {
    fn from(value: crate::pagination_offset::PaginationOffset) -> Self {
        Self(value.get())
    }
}
