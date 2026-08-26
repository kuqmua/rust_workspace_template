#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct PaginationEnd(i64);
