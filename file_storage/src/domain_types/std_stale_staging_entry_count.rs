#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::GetInner,
    newtype::IntoInnerFrom,
    newtype::Display,
)]
pub struct StdStaleStagingEntryCount(pub(super) usize);
