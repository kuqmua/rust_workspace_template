#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::Display,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct ObservedErrorCode(&'static str);
