#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub(super) struct ResourceBudgetMaximumNonZeroUsize(pub(super) std::num::NonZeroUsize);
