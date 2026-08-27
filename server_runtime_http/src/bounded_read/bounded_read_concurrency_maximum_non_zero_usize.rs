#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct BoundedReadConcurrencyMaximumNonZeroUsize(pub(super) std::num::NonZeroUsize);
