#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct AdminNonZeroUsize(std::num::NonZeroUsize);
