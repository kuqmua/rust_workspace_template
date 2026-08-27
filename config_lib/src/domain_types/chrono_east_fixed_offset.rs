#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub(super) struct ChronoEastFixedOffset(pub(super) chrono::FixedOffset);
