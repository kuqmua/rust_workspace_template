#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct SingleFlightMaximumNonZeroUsize(std::num::NonZeroUsize);
