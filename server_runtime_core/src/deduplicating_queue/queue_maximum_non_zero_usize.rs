#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct QueueMaximumNonZeroUsize(pub(super) std::num::NonZeroUsize);
