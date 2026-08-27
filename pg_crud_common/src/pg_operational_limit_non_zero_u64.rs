#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::FromInner,
)]
pub(super) struct PgOperationalLimitNonZeroU64(pub(super) std::num::NonZeroU64);
