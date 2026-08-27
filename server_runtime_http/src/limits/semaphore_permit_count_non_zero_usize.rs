#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct SemaphorePermitCountNonZeroUsize(pub(super) std::num::NonZeroUsize);
