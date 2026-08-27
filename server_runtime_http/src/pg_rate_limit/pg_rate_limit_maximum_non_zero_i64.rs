#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub(super) struct PgRateLimitMaximumNonZeroI64(pub(super) std::num::NonZeroI64);
