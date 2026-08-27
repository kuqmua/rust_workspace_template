#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::Display,
    newtype::FromInner,
)]
pub(super) struct RetryAfterSecsNonZeroU64(pub(super) std::num::NonZeroU64);
