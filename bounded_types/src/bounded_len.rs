#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::Display,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct BoundedLen(usize);
