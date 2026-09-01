#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, newtype_foundation::FromInner, newtype_foundation::GetInner,
)]
#[accessor(pub(super))]
pub struct PartIndex(usize);
