#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::PartialEqInner,
)]
pub struct GitCommitLinkCapacity(usize);
