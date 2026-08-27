#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct DuplicateIdx(usize);
