#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype_foundation::FromInner,
    newtype_foundation::GetInner,
)]
#[accessor(pub(super))]
#[borrow]
pub(crate) struct SnakeIdentifierifierLen(usize);
