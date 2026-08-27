#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub(super) struct PgRateLimitWindowSecondsNonZeroI32(pub(super) std::num::NonZeroI32);
