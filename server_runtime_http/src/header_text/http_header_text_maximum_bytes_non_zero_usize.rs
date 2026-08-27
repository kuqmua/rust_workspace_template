#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub(super) struct HttpHeaderTextMaximumBytesNonZeroUsize(pub(super) std::num::NonZeroUsize);
