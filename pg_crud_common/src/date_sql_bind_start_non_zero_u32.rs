#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct DateSqlBindStartNonZeroU32(pub(crate) std::num::NonZeroU32);
