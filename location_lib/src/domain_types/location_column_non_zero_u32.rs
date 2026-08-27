#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    serde::Serialize,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::Display,
    newtype::FromInner,
)]
pub(super) struct LocationColumnNonZeroU32(pub(super) std::num::NonZeroU32);
