#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::Display,
)]
pub struct BoundedReadMaximumBytes(pub(super) usize);
