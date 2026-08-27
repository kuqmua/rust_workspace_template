#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub(super) struct CursorMaximumLengthNonZeroUsize(pub(super) std::num::NonZeroUsize);
