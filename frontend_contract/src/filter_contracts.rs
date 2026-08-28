use super::FilterOperation;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct FilterContracts(&'static [FilterOperation]);
