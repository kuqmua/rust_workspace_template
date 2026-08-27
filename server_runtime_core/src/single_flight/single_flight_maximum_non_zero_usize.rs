#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct SingleFlightMaximumNonZeroUsize(pub(super) std::num::NonZeroUsize);
