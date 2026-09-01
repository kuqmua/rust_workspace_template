#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype_foundation::FromInner,
    newtype_foundation::GetInner,
)]
pub struct StdUniqueOptionSetIsEmpty(bool);
