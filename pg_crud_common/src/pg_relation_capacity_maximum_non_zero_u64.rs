#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub(super) struct PgRelationCapacityMaximumNonZeroU64(pub(super) std::num::NonZeroU64);
