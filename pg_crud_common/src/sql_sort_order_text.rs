#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(crate) struct SqlSortOrderText(&'static str);
